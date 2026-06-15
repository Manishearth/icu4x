// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::SourceDataProvider;
use icu::collections::codepointinvlist::{CodePointInversionList, CodePointInversionListBuilder};
use icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList;
use icu::properties::props::{BidiPairedBracketType, Script};
use icu::properties::script::ScriptWithExt;
use icu::properties::PropertyParser;
use icu::properties::CodePointMapData;
use icu_provider::prelude::*;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use zerovec::VarZeroVec;

/// Type of property name alias.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(super) enum NameType {
    Short,
    Long,
    Numeric,
    Alias,
}

impl SourceDataProvider {
    /// Helper to parse UCD files line-by-line, providing an iterator over the fields of each line.
    ///
    /// It reads the file, strips comments (lines starting with `#` or anything after `#`),
    /// skips empty lines, and passes a mutable reference to an iterator over the trimmed,
    /// semicolon-separated fields to the callback.
    ///
    /// **Important:** Eagerly strips all comments, so it **cannot** be used for parsing
    /// `@missing` rules which are semantically significant comments starting with `# @missing`.
    fn parse_ucd_lines<F>(&self, file: &str, mut action: F) -> Result<(), DataError>
    where
        F: for<'a> FnMut(&mut dyn Iterator<Item = &'a str>) -> Result<(), DataError>,
    {
        for line in self.unicode()?.read_to_string(file)?.lines() {
            let line = line.split('#').next().unwrap().trim();
            if line.is_empty() {
                continue;
            }
            let mut fields = line.split(';').map(str::trim);
            action(&mut fields)?;
        }
        Ok(())
    }

    /// Validates that the given property name matches its expected short name.
    ///
    /// This helper reads `PropertyValueAliases.txt` to verify the association.
    pub(super) fn validate_property_name(
        &self,
        name: &str,
        short_name: &str,
    ) -> Result<(), DataError> {
        let mut sn = None;
        self.parse_ucd_lines("ucd/PropertyAliases.txt", |fields| {
            if sn.is_some() {
                return Ok(());
            }
            let s_n = fields.next().unwrap();
            let n = fields.next().unwrap();
            if n == name {
                sn = Some(s_n.to_string());
            }
            Ok(())
        })?;

        if let Some(sn) = sn
            && sn != short_name
        {
            return Err(DataError::custom("Property name mismatch")
                .with_display_context(name)
                .with_debug_context(&(sn, short_name)));
        }

        Ok(())
    }

    /// Retrieves the set of code points associated with a binary property.
    ///
    /// This helper reads the appropriate UCD file (e.g., `DerivedCoreProperties.txt`,
    /// `PropList.txt`) and builds a `CodePointInversionList` representing the set.
    pub(super) fn get_binary_prop(
        &self,
        name: &str,
        short_name: &str,
    ) -> Result<CodePointInversionList<'static>, DataError> {
        let mut builder = CodePointInversionListBuilder::new();

        self.validate_property_name(name, short_name)?;

        let file = match name {
            "Alphabetic"
            | "Case_Ignorable"
            | "Cased"
            | "Changes_When_Casefolded"
            | "Changes_When_Casemapped"
            | "Changes_When_Lowercased"
            | "Changes_When_Titlecased"
            | "Changes_When_Uppercased"
            | "Default_Ignorable_Code_Point"
            | "Grapheme_Base"
            | "Grapheme_Extend"
            | "Grapheme_Link"
            | "ID_Continue"
            | "ID_Start"
            | "Lowercase"
            | "Math"
            | "Uppercase"
            | "XID_Continue"
            | "XID_Start" => "ucd/DerivedCoreProperties.txt",
            "Changes_When_NFKC_Casefolded" | "Full_Composition_Exclusion" => {
                "ucd/DerivedNormalizationProps.txt"
            }
            "Emoji_Component"
            | "Emoji_Modifier_Base"
            | "Emoji_Modifier"
            | "Emoji_Presentation"
            | "Emoji"
            | "Extended_Pictographic" => "ucd/emoji/emoji-data.txt",
            "Bidi_Mirrored" => "ucd/extracted/DerivedBinaryProperties.txt",
            _ => "ucd/PropList.txt",
        };

        self.parse_ucd_lines(file, |fields| {
            let range = fields.next().unwrap();
            if fields.next() != Some(name) {
                return Ok(());
            }

            builder.add_range32(parse_range(range));
            Ok(())
        })?;

        Ok(builder.build())
    }

    /// Retrieves the set of code points and strings associated with an emoji property.
    ///
    /// This helper reads `emoji-sequences.txt` and builds a `CodePointInversionListAndStringList`.
    pub(super) fn get_unicodeset_property(
        &self,
        name: &str,
        short_name: &str,
    ) -> Result<CodePointInversionListAndStringList<'static>, DataError> {
        self.validate_property_name(name, short_name)?;

        let mut inv_list = CodePointInversionListBuilder::new();
        let mut strings = BTreeSet::new();

        self.parse_ucd_lines("emoji/emoji-sequences.txt", |fields| {
            let seq = fields.next().unwrap();
            if fields.next() != Some(short_name) {
                return Ok(());
            }
            if seq.contains(' ') {
                strings.insert(
                    seq.split(' ')
                        .map(|cp| char::from_u32(u32::from_str_radix(cp, 16).unwrap()).unwrap())
                        .collect::<String>(),
                );
            } else {
                inv_list.add_range32(parse_range(seq));
            }
            Ok(())
        })?;

        let inv_list = inv_list.build();

        Ok(CodePointInversionListAndStringList::try_from(
            inv_list,
            VarZeroVec::from(&strings.into_iter().collect::<Vec<_>>()),
        )
        .expect("invariants upheld"))
    }

    /// Parses `BidiMirroring.txt` to create a map of code points to their mirrored characters.
    pub(super) fn parse_bidi_mirroring(&self) -> Result<HashMap<u32, char>, DataError> {
        let mut bidi_mirroring = HashMap::new();
        self.parse_ucd_lines("ucd/BidiMirroring.txt", |fields| {
            let cp = u32::from_str_radix(fields.next().unwrap(), 16).unwrap();
            let value = u32::from_str_radix(fields.next().unwrap(), 16).unwrap();
            bidi_mirroring.insert(cp, char::from_u32(value).unwrap());
            Ok(())
        })?;
        Ok(bidi_mirroring)
    }

    /// Parses `BidiBrackets.txt` to create a map of code points to their bidi paired bracket types.
    ///
    /// This helper also cross-validates the results with the provided `bidi_mirroring` map.
    pub(super) fn parse_bidi_brackets(
        &self,
        bidi_mirroring: &HashMap<u32, char>,
    ) -> Result<HashMap<u32, BidiPairedBracketType>, DataError> {
        let mut paired_brackets = HashMap::new();
        self.parse_ucd_lines("ucd/BidiBrackets.txt", |fields| {
            let cp = u32::from_str_radix(fields.next().unwrap(), 16).unwrap();
            let mirror = u32::from_str_radix(fields.next().unwrap(), 16).unwrap();

            if bidi_mirroring[&cp] as u32 != mirror {
                log::warn!(
                    "BidiMirroring.txt and BidiBrackets.txt disagree for U+{cp:X}: {:?} vs U+{mirror:X}", 
                    bidi_mirroring[&cp]
                );
            }

            let typ = match fields.next().unwrap() {
                "o" => BidiPairedBracketType::Open,
                "c" => BidiPairedBracketType::Close,
                "n" => BidiPairedBracketType::None,
                _ => unreachable!(),
            };
            paired_brackets.insert(cp, typ);
            Ok(())
        })?;
        Ok(paired_brackets)
    }

    /// Parses `ScriptExtensions.txt` to create script sets and a lookup map for code points.
    ///
    /// Returns a tuple containing:
    /// 1. A list of unique script sets (`Vec<Vec<Script>>`), where each set is sorted.
    /// 2. A map from code points to `ScriptWithExt` (`HashMap<u32, ScriptWithExt>`), which
    ///    associates the code point with its primary script and an index into the script sets list.
    #[allow(clippy::type_complexity)]
    pub(super) fn parse_script_extensions(
        &self,
        script_parser: &PropertyParser<Script>,
        script: &CodePointMapData<Script>,
    ) -> Result<(Vec<Vec<Script>>, HashMap<u32, ScriptWithExt>), DataError> {
        let mut script_sets = vec![];
        let mut script_sets_lookup = BTreeMap::new();
        let mut char_with_extensions = HashMap::new();

        self.parse_ucd_lines("ucd/ScriptExtensions.txt", |fields| {
            let cp_range = fields.next().unwrap();
            let values = fields.next().unwrap();
            let mut value = values
                .split_ascii_whitespace()
                .filter_map(|s| script_parser.as_borrowed().get_strict(s))
                .collect::<Vec<_>>();
            // Sort in discriminant order
            value.sort();

            for cp in parse_range(cp_range) {
                let mut value = value.clone();

                let script = script.as_borrowed().get32(cp);
                if !matches!(script, Script::Inherited | Script::Common) {
                    value.insert(0, script);
                }

                if !script_sets_lookup.contains_key(&value) {
                    script_sets_lookup.insert(value.clone(), script_sets.len());
                    script_sets.push(value.clone());
                }

                char_with_extensions.insert(
                    cp,
                    ScriptWithExt::new(script, script_sets_lookup[&value] as u16),
                );
            }
            Ok(())
        })?;

        Ok((script_sets, char_with_extensions))
    }
    /// Parses `PropertyValueAliases.txt` to retrieve the names and aliases for an enumerated property.
    ///
    /// Returns a map from names/aliases to their canonical short names and name types,
    /// along with an optional default value if declared in the file.
    ///
    /// Note: This function does NOT use the `parse_ucd_lines` helper because it needs to
    /// parse `@missing` metadata which is encoded in UCD comments.
    #[allow(clippy::type_complexity)] // just a tuple
    pub(super) fn enumerated_prop_names<'a>(
        &'a self,
        name: &str,
        short_name: &str,
    ) -> Result<(BTreeMap<&'a str, (&'a str, NameType)>, Option<&'a str>), DataError> {
        let mut names = BTreeMap::new();
        let mut default = None;

        for line in self
            .unicode()?
            .read_to_string("ucd/PropertyValueAliases.txt")?
            .lines()
        {
            if let Some(line) = line.strip_prefix("# @missing: 0000..10FFFF; ") {
                let mut parts = line.split(';').map(str::trim);
                if parts.next().unwrap() != name {
                    continue;
                }
                default = Some(parts.next().unwrap());
            };
            let line = line.split('#').next().unwrap().trim();
            if line.is_empty() {
                continue;
            }
            let mut parts = line.split(';').map(str::trim);
            if parts.next().unwrap() != short_name {
                continue;
            }
            let numeric_name = (short_name.as_bytes()
                == icu::properties::props::CanonicalCombiningClass::SHORT_NAME)
                .then(|| parts.next().unwrap());
            let short = parts.next().unwrap();
            let long = parts.next().unwrap();
            names.insert(short, (short, NameType::Short));
            names.insert(long, (short, NameType::Long));
            for alias in parts {
                names.insert(alias, (short, NameType::Alias));
            }
            if let Some(numeric_name) = numeric_name {
                names.insert(numeric_name, (short, NameType::Numeric));
            }
        }

        for name in names.keys() {
            if name.contains('-') || name.bytes().any(|b| b.is_ascii_whitespace()) {
                return Err(
                    DataError::custom("Property name contains '-' or whitespace")
                        .with_display_context(name),
                );
            }
        }

        Ok((names, default))
    }
}
