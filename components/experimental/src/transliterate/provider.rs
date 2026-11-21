// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// Provider structs must be stable
#![allow(clippy::exhaustive_structs, clippy::exhaustive_enums)]

//! 🚧 \[Unstable\] Data provider struct definitions for this ICU4X component.
//!
//! <div class="stab unstable">
//! 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
//! including in SemVer minor releases. While the serde representation of data structs is guaranteed
//! to be stable, their Rust representation might not be. Use with caution.
//! </div>
//!
//! Read more about data providers: [`icu_provider`]

use core::ops::RangeInclusive;

use alloc::borrow::Cow;
use alloc::vec::Vec;

use icu_collections::{
    codepointinvlist::CodePointInversionList,
    codepointinvliststringlist::CodePointInversionListAndStringList,
};
use icu_provider::prelude::*;
use vecs::Index32;
use zerovec::*;

// TODO(#3776): Improve the documentation of this datastruct.

icu_provider::data_marker!(
    /// `TransliteratorRulesV1`
    TransliteratorRulesV1,
    RuleBasedTransliterator<'static>,
    #[cfg(feature = "datagen")]
    expose_baked_consts = true,
    #[cfg(feature = "datagen")]
    attributes_domain = "transliterator",
);

/// The data struct representing [UTS #35 transform rules](https://unicode.org/reports/tr35/tr35-general.html#Transforms).
#[derive(Debug, Clone, PartialEq, Eq, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_experimental::transliterate::provider))]
pub struct RuleBasedTransliterator<'a> {
    /// Whether this transliterator is accessible directly through the constructor.
    /// Hidden transliterators are intended as dependencies for visible transliterators,
    /// see, e.g., [Devanagari-Latin](https://github.com/unicode-org/cldr/blob/main/common/transforms/Devanagari-Latin.xml)
    pub visibility: bool,
    /// The [`VarTable`] containing any special matchers (variables, `UnicodeSets`, ...) used by this transliterator.
    pub variable_table: VarTable<'a>,
    /// The filter for this transliterator. If there is none, the set of all code points is used.
    pub filter: CodePointSet<'a>,
    /// The list of transform rule groups this transliterator uses.
    pub id_group_list: VarZeroVec<'a, VarZeroSlice<SimpleIdULE>>,
    /// The list of conversion rule groups this transliterator uses.
    pub rule_group_list: VarZeroVec<'a, VarZeroSlice<RuleULE, Index32>, Index32>,
}

icu_provider::data_struct!(RuleBasedTransliterator<'_>, #[cfg(feature = "datagen")]);

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for RuleBasedTransliterator<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        #[derive(serde::Deserialize)]
        pub struct Raw<'a> {
            pub visibility: bool,
            #[serde(borrow)]
            pub variable_table: VarTable<'a>,
            #[serde(borrow)]
            pub filter: CodePointSet<'a>,
            #[serde(borrow)]
            pub id_group_list: VarZeroVec<'a, VarZeroSlice<SimpleIdULE>>,
            #[serde(borrow)]
            pub rule_group_list: VarZeroVec<'a, VarZeroSlice<RuleULE, Index32>, Index32>,
        }

        let Raw {
            visibility,
            variable_table,
            filter,
            id_group_list,
            rule_group_list,
        } = Raw::deserialize(deserializer)?;
        if id_group_list.len() != rule_group_list.len() {
            return Err(D::Error::custom(
                "invalid data: id_group_list and rule_group_list have different lengths",
            ));
        }
        Ok(Self {
            visibility,
            variable_table,
            filter,
            id_group_list,
            rule_group_list,
        })
    }
}

impl RuleBasedTransliterator<'_> {
    /// Returns an iterator of dependencies on other transliterators.
    ///
    /// Note that this may contain duplicate entries.
    pub fn deps(&self) -> impl Iterator<Item = Cow<'_, str>> {
        use zerofrom::ZeroFrom;
        self.id_group_list
            .iter()
            .flat_map(|id_group| id_group.iter().map(|s| SimpleId::zero_from(s).id))
            .chain(
                self.variable_table
                    .function_calls
                    .iter()
                    .map(|s| FunctionCall::zero_from(s).translit.id),
            )
    }
}

/// The ID of a transliterator plus an optional filter.
#[derive(Debug, Clone)]
#[make_varule(SimpleIdULE)]
#[zerovec::skip_derive(Ord)]
#[zerovec::derive(Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    zerovec::derive(Deserialize)
)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize),
    zerovec::derive(Serialize)
)]
pub struct SimpleId<'a> {
    /// The filter for the transliterator. If there is none, the set of all code points is used.
    #[zerovec::varule(CodePointSetULE)]
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub filter: CodePointSet<'a>,
    /// The ID of the transliterator.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub id: Cow<'a, str>,
}

/// A set of Unicode code points encoded as delta-varint inclusive `(gap, len)` ranges (`[0xFF]` = all).
#[make_varule(CodePointSetULE)]
#[zerovec::skip_derive(Ord)]
#[zerovec::derive(Debug)]
#[derive(Debug, Clone, PartialEq, Eq, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    zerovec::derive(Deserialize)
)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_experimental::transliterate::provider),
    zerovec::derive(Serialize)
)]
pub struct CodePointSet<'a>(
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[doc(hidden)]
    /// Delta-varint encoded inclusive `(gap, len)` range bytes (`[0xFF]` = all code points).
    pub ZeroVec<'a, u8>,
);

impl CodePointSet<'static> {
    pub(crate) const fn all() -> Self {
        // [0xFF] cannot collide with any valid LEB128 sequence (continuation bit set on a 1-byte slice).
        Self(ZeroSlice::from_ule_slice(&[0xFF]).as_zerovec())
    }
}

impl CodePointSet<'_> {
    pub(crate) fn from_inversion_list(list: &CodePointInversionList<'_>) -> CodePointSet<'static> {
        if list == &CodePointInversionList::all() {
            return CodePointSet::all();
        }
        let mut out = Vec::new();
        let mut prev = 0u32;
        for r in list.iter_ranges() {
            let (start, end) = (*r.start(), *r.end());
            push_varint(&mut out, start - prev);
            push_varint(&mut out, end - start);
            prev = end + 1;
        }
        CodePointSet(ZeroVec::alloc_from_slice(&out))
    }

    pub(crate) fn contains(&self, c: char) -> bool {
        cp_set_contains(self.0.as_ule_slice(), c)
    }
}

impl CodePointSetULE {
    pub(crate) fn contains(&self, c: char) -> bool {
        cp_set_contains(self.0.as_ule_slice(), c)
    }
}

/// A `UnicodeSet` represented as a [`CodePointSet`] plus sorted multi-char strings.
#[make_varule(UnicodeSetULE)]
#[zerovec::skip_derive(Ord)]
#[zerovec::derive(Debug)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    zerovec::derive(Deserialize)
)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize),
    zerovec::derive(Serialize)
)]
pub struct UnicodeSet<'a> {
    /// The code points in this set.
    #[zerovec::varule(CodePointSetULE)]
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub code_points: CodePointSet<'a>,
    /// Multi-code-point strings (and possibly the empty string) in sorted order.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub strings: VarZeroVec<'a, str>,
}

impl UnicodeSet<'_> {
    pub(crate) fn from_inversion_list_and_strings(
        set: &CodePointInversionListAndStringList<'_>,
    ) -> UnicodeSet<'static> {
        let strings: Vec<_> = set.strings().iter().collect();
        UnicodeSet {
            code_points: CodePointSet::from_inversion_list(set.code_points()),
            strings: VarZeroVec::from(&strings),
        }
    }
}

impl UnicodeSetULE {
    pub(crate) fn contains(&self, c: char) -> bool {
        self.code_points().contains(c)
    }

    pub(crate) fn contains_empty(&self) -> bool {
        self.strings().get(0) == Some("")
    }
}

fn push_varint(out: &mut Vec<u8>, mut n: u32) {
    while n >= 0x80 {
        out.push((n & 0x7F) as u8 | 0x80);
        n >>= 7;
    }
    out.push(n as u8);
}

fn read_varint(p: &mut &[u8]) -> Option<u32> {
    let mut n = 0u32;
    let mut shift = 0u32;
    loop {
        let (&b, rest) = p.split_first()?;
        *p = rest;
        n |= ((b & 0x7F) as u32).checked_shl(shift)?;
        if b < 0x80 {
            return Some(n);
        }
        shift += 7;
    }
}

fn cp_set_contains(mut p: &[u8], c: char) -> bool {
    if p == [0xFF] {
        return true;
    }
    let target = c as u32;
    let mut prev = 0u32;
    while let (Some(gap), Some(len)) = (read_varint(&mut p), read_varint(&mut p)) {
        let Some(start) = prev.checked_add(gap) else {
            return false;
        };
        if target < start {
            return false;
        }
        let Some(end) = start.checked_add(len) else {
            return false;
        };
        if target <= end {
            return true;
        }
        prev = end.saturating_add(1);
    }
    false
}

/// A conversion rule. The source patterns as well as the replacer use inlined private use characters
/// that refer to elements of the [`VarTable`] for special matchers (variables, `UnicodeSets`, ...).
#[derive(Debug, Clone)]
#[make_varule(RuleULE)]
#[zerovec::skip_derive(Ord)]
#[zerovec::derive(Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    zerovec::derive(Deserialize)
)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize),
    zerovec::derive(Serialize)
)]
pub struct Rule<'a> {
    /// The pattern for the ante context. This is not replaced.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub ante: Cow<'a, str>,
    /// The pattern for the key. This is what gets replaced.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub key: Cow<'a, str>,
    /// The pattern for the post context. This is not replaced.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub post: Cow<'a, str>,
    /// The replacer. The key gets replaced with this.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub replacer: Cow<'a, str>,
}

/// The special matchers and replacers used by this transliterator.
#[derive(Debug, Clone, zerofrom::ZeroFrom, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_experimental::transliterate::provider))]
pub struct VarTable<'a> {
    /// Variable definitions.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub compounds: VarZeroVec<'a, str, Index32>,
    /// Zero or one quantifiers.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub quantifiers_opt: VarZeroVec<'a, str, Index32>,
    /// Zero or more quantifiers.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub quantifiers_kleene: VarZeroVec<'a, str, Index32>,
    /// One or more quantifiers.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub quantifiers_kleene_plus: VarZeroVec<'a, str, Index32>,
    /// Segments.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub segments: VarZeroVec<'a, SegmentULE, Index32>,
    /// `UnicodeSets`.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub unicode_sets: VarZeroVec<'a, UnicodeSetULE, Index32>,
    /// Function calls.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub function_calls: VarZeroVec<'a, FunctionCallULE, Index32>,
    /// The maximum number of _left_ placeholders (`rest @@@ |`) in any rule.
    pub max_left_placeholder_count: u16,
    /// The maximum number of _right_ placeholders (`| @@@ rest`) in any rule.
    pub max_right_placeholder_count: u16,
}

impl VarTable<'_> {
    /// The lowest `char` used for encoding specials.
    pub const BASE: char = '\u{F0000}';
    /// The highest `char` used for encoding dynamic (i.e., growing, non-reserved) specials.
    pub const MAX_DYNAMIC: char = '\u{FFFF0}';
    /// The `char` that encodes a pure cursor, `|` without `@`.
    pub const RESERVED_PURE_CURSOR: char = '\u{FFFFB}';
    /// The `char` that encodes a start anchor, `^`.
    pub const RESERVED_ANCHOR_START: char = '\u{FFFFC}';
    /// The `char` that encodes an end anchor, `$`.
    pub const RESERVED_ANCHOR_END: char = '\u{FFFFD}';

    /// The range used for encoded specials.
    pub const ENCODE_RANGE: RangeInclusive<char> = Self::BASE..=Self::RESERVED_ANCHOR_END;
    /// The number of `char`s available for encoding dynamic (i.e., growing, non-reserved) specials.
    pub const NUM_DYNAMIC: usize = Self::MAX_DYNAMIC as usize - Self::BASE as usize + 1;
}

/// Segments store matched parts of the input dynamically and can be referred to by back references
/// in the replacer.
#[derive(Debug, Clone)]
#[make_varule(SegmentULE)]
#[zerovec::skip_derive(Ord)]
#[zerovec::derive(Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    zerovec::derive(Deserialize)
)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize),
    zerovec::derive(Serialize)
)]
pub struct Segment<'a> {
    /// The 0-based index of this segment.
    pub idx: u16,
    #[cfg_attr(feature = "serde", serde(borrow))]
    /// The content of the segment.
    pub content: Cow<'a, str>,
}

/// An inline recursive call to a transliterator with an arbitrary argument.
#[derive(Debug, Clone)]
#[make_varule(FunctionCallULE)]
#[zerovec::skip_derive(Ord)]
#[zerovec::derive(Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    zerovec::derive(Deserialize)
)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize),
    zerovec::derive(Serialize)
)]
pub struct FunctionCall<'a> {
    /// The transliterator that will be called.
    #[zerovec::varule(SimpleIdULE)]
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub translit: SimpleId<'a>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    /// The argument to be transliterated given to the transliterator.
    pub arg: Cow<'a, str>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use icu_collections::codepointinvlist::CodePointInversionListBuilder;

    #[test]
    fn roundtrip_cp_sets() {
        assert_eq!(
            CodePointSet::from_inversion_list(&CodePointInversionList::all()),
            CodePointSet::all()
        );
        assert!(CodePointSet::all().contains('\0'));
        assert!(CodePointSet::all().contains('\u{10FFFF}'));

        let empty = CodePointInversionListBuilder::new().build();
        let empty_set = CodePointSet::from_inversion_list(&empty);
        assert!(empty_set.0.is_empty());
        assert!(!empty_set.contains('a'));

        let mut b = CodePointInversionListBuilder::new();
        b.add_char('\u{FF}'); // [0xFF, 0x01, 0x00] must not collide with all() == [0xFF]
        b.add_range('a'..='z');
        b.add_char('\u{FFFF}');
        b.add_char('\u{10FFFF}');
        let set = CodePointSet::from_inversion_list(&b.build());
        assert_ne!(set, CodePointSet::all());
        assert!(set.contains('\u{FF}'));
        assert!(!set.contains('\u{FE}'));
        assert!(!set.contains('\u{100}'));
        assert!(set.contains('a'));
        assert!(set.contains('z'));
        assert!(set.contains('\u{FFFF}'));
        assert!(set.contains('\u{10FFFF}'));
    }
}
