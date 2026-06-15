// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::SourceDataProvider;
use icu_provider::prelude::*;

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
}
