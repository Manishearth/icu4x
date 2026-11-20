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
#[cfg(feature = "serde")]
use alloc::boxed::Box;

use icu_collections::{
    codepointinvlist::{CodePointInversionList, CodePointInversionListBuilder},
    codepointinvliststringlist::CodePointInversionListAndStringList,
};
use icu_provider::prelude::*;
use vecs::Index32;
use zerovec::{
    ule::{EncodeAsVarULE, UleError, VarULE},
    *,
};

const MAX_SENTINEL: u32 = char::MAX as u32 + 1;

fn decode_varint(iter: &mut core::iter::Copied<core::slice::Iter<'_, u8>>) -> Option<u32> {
    let mut val: u32 = 0;
    let mut shift: u32 = 0;
    for b in iter {
        if shift >= 28 {
            return None;
        }
        val |= u32::from(b & 0x7F) << shift;
        if b & 0x80 == 0 {
            if shift > 0 && b == 0 {
                return None;
            }
            return Some(val);
        }
        shift += 7;
    }
    None
}

const fn varint_len(mut v: u32) -> usize {
    let mut len = 1;
    while v >= 0x80 {
        v >>= 7;
        len += 1;
    }
    len
}

fn write_varint(mut v: u32, mut dst: &mut [u8]) -> &mut [u8] {
    while let Some((first, rest)) = core::mem::take(&mut dst).split_first_mut() {
        dst = rest;
        if v < 0x80 {
            *first = v as u8;
            break;
        }
        *first = ((v & 0x7F) as u8) | 0x80;
        v >>= 7;
    }
    dst
}

fn is_all_inversion_list(list: &CodePointInversionList<'_>) -> bool {
    list.size() == MAX_SENTINEL as usize
}

/// A compact delta-LEB128 encoded set of Unicode code points.
///
/// Encoding:
/// - `b""` (0 bytes): empty code point set `{}`.
/// - `b"\xFF"` (1 byte): universal code point set `0..=0x10FFFF` ([`CodePointInversionList::all()`]).
/// - Otherwise: an even number (`>= 2`) of minimal unsigned LEB128 varints encoding
///   strictly-increasing inversion list endpoints `[p_0, p_1, ..., p_{2k-1}]`
///   (`0 <= p_0 < p_1 < ... < p_{2k-1} <= 0x110000`, and not `[0, 0x110000]`),
///   where `d_0 = p_0` and `d_i = p_i - p_{i-1} - 1` for `i >= 1`.
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct CodePointSetULE([u8]);

impl CodePointSetULE {
    /// Returns the canonical byte representation of the universal code point set (`0..=0x10FFFF`).
    pub const fn all_bytes() -> &'static [u8] {
        &[0xFF]
    }

    /// Returns a [`VarZeroCow`] representing the universal code point set (`0..=0x10FFFF`).
    pub const fn new_all_cow() -> VarZeroCow<'static, Self> {
        // SAFETY: `all_bytes()` (`b"\xFF"`) is the canonical representation of the universal set.
        unsafe { VarZeroCow::from_bytes_unchecked(Self::all_bytes()) }
    }

    /// Converts a [`CodePointInversionList`] into a [`VarZeroCow<'static, CodePointSetULE>`].
    pub fn from_inversion_list(list: &CodePointInversionList<'_>) -> VarZeroCow<'static, Self> {
        if list.is_empty() {
            // SAFETY: `b""` is the canonical representation of the empty set.
            return unsafe { VarZeroCow::from_bytes_unchecked(&[]) };
        }
        if is_all_inversion_list(list) {
            return Self::new_all_cow();
        }
        VarZeroCow::from_encodeable(list)
    }

    /// Parses a byte slice into a `&CodePointSetULE`.
    pub fn parse_bytes(bytes: &[u8]) -> Result<&Self, UleError> {
        <Self as VarULE>::parse_bytes(bytes)
    }

    /// Returns `true` if `self` contains all Unicode code points (`0..=0x10FFFF`).
    pub fn is_all(&self) -> bool {
        self.0 == [0xFF]
    }

    /// Returns `true` if `self` is the empty set.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns `true` if `self` contains the code point `c`.
    pub fn contains(&self, c: char) -> bool {
        if self.0.is_empty() {
            return false;
        }
        if self.0 == [0xFF] {
            return true;
        }
        let target = c as u32;
        let mut iter = self.0.iter().copied();
        let Some(p0) = decode_varint(&mut iter) else {
            return false;
        };
        if target < p0 {
            return false;
        }
        let mut cur = p0;
        let mut in_set = true;
        while let Some(delta) = decode_varint(&mut iter) {
            cur = cur.saturating_add(delta).saturating_add(1);
            if target < cur {
                return in_set;
            }
            in_set = !in_set;
        }
        false
    }

    /// Decodes `self` into an owned [`CodePointInversionList`].
    pub fn to_code_point_inversion_list(&self) -> CodePointInversionList<'static> {
        if self.is_all() {
            return CodePointInversionList::all();
        }
        let mut builder = CodePointInversionListBuilder::new();
        let mut iter = self.0.iter().copied();
        let Some(first) = decode_varint(&mut iter) else {
            return builder.build();
        };
        let mut cur = first;
        while iter.len() > 0 {
            let start = cur;
            let Some(delta) = decode_varint(&mut iter) else {
                break;
            };
            let end = start.saturating_add(delta).saturating_add(1);
            if let Some(end_incl) = end.checked_sub(1) {
                builder.add_range32(start..=end_incl);
            }
            let Some(next_delta) = decode_varint(&mut iter) else {
                break;
            };
            cur = end.saturating_add(next_delta).saturating_add(1);
        }
        builder.build()
    }
}

// Safety:
// 1. `CodePointSetULE` is `#[repr(transparent)]` over `[u8]`, so it has no padding/uninitialized bytes
//    and alignment 1.
// 2. `validate_bytes` checks that the full slice is canonical empty, canonical universal `[0xFF]`,
//    or an even (>= 2) sequence of minimal LEB128 deltas yielding strictly-increasing endpoints <= 0x110000
//    (and excluding non-canonical `[0, 0x110000]`).
// 3. `from_bytes_unchecked` preserves exact pointer and slice length via `#[repr(transparent)]`.
// 4. Canonical encoding guarantees semantic equality equals byte equality.
unsafe impl VarULE for CodePointSetULE {
    fn validate_bytes(bytes: &[u8]) -> Result<(), UleError> {
        if bytes.is_empty() || bytes == [0xFF] {
            return Ok(());
        }
        let mut iter = bytes.iter().copied();
        let Some(first) = decode_varint(&mut iter) else {
            return Err(UleError::parse::<Self>());
        };
        if first >= MAX_SENTINEL {
            return Err(UleError::parse::<Self>());
        }
        let mut prev = first;
        let mut count: usize = 1;
        while iter.len() > 0 {
            let Some(delta) = decode_varint(&mut iter) else {
                return Err(UleError::parse::<Self>());
            };
            let Some(next) = prev.checked_add(delta).and_then(|v| v.checked_add(1)) else {
                return Err(UleError::parse::<Self>());
            };
            if next > MAX_SENTINEL {
                return Err(UleError::parse::<Self>());
            }
            prev = next;
            count += 1;
        }
        if count < 2 || !count.is_multiple_of(2) {
            return Err(UleError::parse::<Self>());
        }
        if count == 2 && first == 0 && prev == MAX_SENTINEL {
            return Err(UleError::parse::<Self>());
        }
        Ok(())
    }

    unsafe fn from_bytes_unchecked(bytes: &[u8]) -> &Self {
        // SAFETY: `CodePointSetULE` is `#[repr(transparent)]` over `[u8]`.
        unsafe { &*(bytes as *const [u8] as *const Self) }
    }
}

impl core::fmt::Debug for CodePointSetULE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.to_code_point_inversion_list().fmt(f)
    }
}

// Safety: `encode_var_ule_len` and `encode_var_ule_write` produce the exact canonical
// `CodePointSetULE` byte representation matching `Self::iter_ranges()`.
unsafe impl EncodeAsVarULE<CodePointSetULE> for CodePointInversionList<'_> {
    fn encode_var_ule_as_slices<R>(&self, _: impl FnOnce(&[&[u8]]) -> R) -> R {
        unreachable!()
    }

    fn encode_var_ule_len(&self) -> usize {
        if self.is_empty() {
            return 0;
        }
        if is_all_inversion_list(self) {
            return 1;
        }
        let mut len = 0;
        let mut prev: Option<u32> = None;
        for range in self.iter_ranges() {
            let start = *range.start();
            let end = *range.end() + 1;
            let d_start = match prev {
                None => start,
                Some(p) => start - p - 1,
            };
            len += varint_len(d_start);
            let d_end = end - start - 1;
            len += varint_len(d_end);
            prev = Some(end);
        }
        len
    }

    fn encode_var_ule_write(&self, dst: &mut [u8]) {
        if self.is_empty() {
            return;
        }
        if is_all_inversion_list(self) {
            if let Some(first) = dst.first_mut() {
                *first = 0xFF;
            }
            return;
        }
        let mut rest = dst;
        let mut prev: Option<u32> = None;
        for range in self.iter_ranges() {
            let start = *range.start();
            let end = *range.end() + 1;
            let d_start = match prev {
                None => start,
                Some(p) => start - p - 1,
            };
            rest = write_varint(d_start, rest);
            let d_end = end - start - 1;
            rest = write_varint(d_end, rest);
            prev = Some(end);
        }
        debug_assert!(rest.is_empty());
    }
}

#[cfg(feature = "datagen")]
impl serde::Serialize for CodePointSetULE {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if serializer.is_human_readable() {
            self.to_code_point_inversion_list().serialize(serializer)
        } else {
            serializer.serialize_bytes(&self.0)
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Box<CodePointSetULE> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            let list = CodePointInversionList::deserialize(deserializer)?;
            Ok(ule::encode_varule_to_box(&list))
        } else {
            let bytes = <&[u8]>::deserialize(deserializer)?;
            CodePointSetULE::parse_bytes(bytes)
                .map(VarULE::to_boxed)
                .map_err(serde::de::Error::custom)
        }
    }
}

/// A `UnicodeSet` used in a transliterator rule, consisting of a compact code point set
/// and a sorted list of unique multi-code-point (or empty) strings.
#[derive(Debug, Clone, PartialEq, Eq)]
#[make_varule(UnicodeSetULE)]
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
pub struct UnicodeSet<'a> {
    /// The code points contained in this set.
    #[zerovec::varule(CodePointSetULE)]
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub cp_inv_list: VarZeroCow<'a, CodePointSetULE>,
    /// Sorted list of unique strings in this set.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub str_list: VarZeroVec<'a, str>,
}

impl<'a> UnicodeSet<'a> {
    /// Constructs a [`UnicodeSet`] from a [`CodePointInversionListAndStringList`].
    pub fn from_cpilsl(set: &'a CodePointInversionListAndStringList<'_>) -> Self {
        Self {
            cp_inv_list: CodePointSetULE::from_inversion_list(set.code_points()),
            str_list: set.strings().as_varzerovec(),
        }
    }
}

impl UnicodeSetULE {
    /// Returns `true` if `self` contains the code point `c`.
    pub fn contains(&self, c: char) -> bool {
        self.cp_inv_list().contains(c)
    }

    /// Returns `true` if `self` contains the string `s`.
    pub fn contains_str(&self, s: &str) -> bool {
        let mut chars = s.chars();
        if let Some(first_char) = chars.next()
            && chars.next().is_none()
        {
            return self.contains(first_char);
        }
        self.str_list().binary_search(s).is_ok()
    }

    /// Access the contained strings.
    pub fn strings(&self) -> &VarZeroSlice<str> {
        self.str_list()
    }
}

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
    pub filter: VarZeroCow<'a, CodePointSetULE>,
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
            pub filter: VarZeroCow<'a, CodePointSetULE>,
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
    pub filter: VarZeroCow<'a, CodePointSetULE>,
    /// The ID of the transliterator.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub id: Cow<'a, str>,
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
    /// `UnicodeSets`. These are represented as a [`UnicodeSet`].
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

    #[test]
    fn test_empty_and_all() {
        let empty_list = CodePointInversionListBuilder::new().build();
        let empty_cow = CodePointSetULE::from_inversion_list(&empty_list);
        assert_eq!(empty_cow.as_bytes(), b"");
        assert!(empty_cow.is_empty());
        assert!(!empty_cow.is_all());
        assert!(!empty_cow.contains('\0'));
        assert!(!empty_cow.contains('a'));
        assert!(!empty_cow.contains(char::MAX));
        assert_eq!(empty_cow.to_code_point_inversion_list(), empty_list);

        let all_list = CodePointInversionList::all();
        let all_cow = CodePointSetULE::from_inversion_list(&all_list);
        assert_eq!(all_cow.as_bytes(), b"\xFF");
        assert!(all_cow.is_all());
        assert!(!all_cow.is_empty());
        assert!(all_cow.contains('\0'));
        assert!(all_cow.contains('a'));
        assert!(all_cow.contains(char::MAX));
        assert_eq!(all_cow.to_code_point_inversion_list(), all_list);

        let const_all = CodePointSetULE::new_all_cow();
        assert_eq!(const_all.as_bytes(), b"\xFF");
        assert!(const_all.is_all());
    }

    #[test]
    fn test_single_chars_and_boundaries() {
        for (ch, expected_bytes) in [
            ('a', &[0x61u8, 0x00][..]),
            ('\0', &[0x00u8, 0x00][..]),
            ('\u{10FFFF}', &[0xFFu8, 0xFF, 0x43, 0x00][..]),
        ] {
            let mut builder = CodePointInversionListBuilder::new();
            builder.add_char(ch);
            let list = builder.build();
            let cow = CodePointSetULE::from_inversion_list(&list);
            assert_eq!(cow.as_bytes(), expected_bytes);
            assert!(!cow.is_empty());
            assert!(!cow.is_all());
            assert!(cow.contains(ch));
            if let Some(prev) = u32::from(ch).checked_sub(1).and_then(char::from_u32) {
                assert!(!cow.contains(prev));
            }
            if let Some(next) = u32::from(ch).checked_add(1).and_then(char::from_u32) {
                assert!(!cow.contains(next));
            }
            assert_eq!(cow.to_code_point_inversion_list(), list);
        }

        let mut builder = CodePointInversionListBuilder::new();
        builder.add_range('0'..='9');
        builder.add_range('a'..='z');
        let list = builder.build();
        let cow = CodePointSetULE::from_inversion_list(&list);
        assert_eq!(cow.to_code_point_inversion_list(), list);
        for c in '\0'..='\u{FF}' {
            assert_eq!(cow.contains(c), list.contains(c), "mismatch at {c:?}");
        }
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn test_mn_property_set() {
        let mn = icu_properties::unicodeset_parse::parse_unstable(
            "[:Mn:]",
            &icu_properties::provider::Baked,
        )
        .unwrap()
        .0
        .code_points()
        .clone();
        let cow = CodePointSetULE::from_inversion_list(&mn);
        // Should be compact (~791 B vs 2,208 B)
        assert!(
            cow.as_bytes().len() < 1000,
            "unexpected len: {}",
            cow.as_bytes().len()
        );
        assert_eq!(cow.to_code_point_inversion_list(), mn);
        for c in '\0'..=char::MAX {
            assert_eq!(cow.contains(c), mn.contains(c), "mismatch at {c:?}");
        }
    }

    #[test]
    fn test_parse_bytes_rejections() {
        let invalid_cases: &[(&str, &[u8])] = &[
            ("non-minimal trailing zero 1", b"\x80\x00"),
            ("non-minimal trailing zero 2", b"\x01\x80\x00"),
            ("truncated varint 1", b"\x80"),
            ("truncated varint 2", b"\x00\x80"),
            ("shift >= 28", b"\x80\x80\x80\x80\x01"),
            ("too large 28-bit varint", b"\xFF\xFF\xFF\x7F"),
            ("odd count 1", b"\x00"),
            ("odd count 3", b"\x01\x02\x03"),
            ("end > 0x110000", b"\x00\x80\x80\x44"),
            ("start == 0x110000", b"\x80\x80\x44\x00"),
            ("start > 0x110000", b"\x81\x80\x44\x00"),
            ("non-canonical all", b"\x00\xFF\xFF\x43"),
        ];
        for (name, bytes) in invalid_cases {
            assert!(
                CodePointSetULE::parse_bytes(bytes).is_err(),
                "expected parse failure for {name}: {bytes:02X?}"
            );
        }

        assert!(CodePointSetULE::parse_bytes(b"").is_ok());
        assert!(CodePointSetULE::parse_bytes(b"\xFF").is_ok());
        assert!(CodePointSetULE::parse_bytes(b"\x61\x00").is_ok());
    }

    #[cfg(all(feature = "datagen", feature = "serde"))]
    #[test]
    fn test_serde_roundtrip() {
        let mut builder = CodePointInversionListBuilder::new();
        builder.add_range('0'..='9');
        builder.add_range('a'..='z');
        let sample_list = builder.build();

        for list in [
            CodePointInversionListBuilder::new().build(),
            CodePointInversionList::all(),
            sample_list.clone(),
        ] {
            let cow = CodePointSetULE::from_inversion_list(&list);

            // JSON matches CodePointInversionList human-readable schema
            let json = serde_json::to_string(&cow).unwrap();
            let inv_json = serde_json::to_string(&list).unwrap();
            assert_eq!(json, inv_json);
            let json_de: VarZeroCow<'_, CodePointSetULE> = serde_json::from_str(&json).unwrap();
            assert_eq!(json_de, cow);

            // Postcard borrows zero-copy from input slice
            let bytes = postcard::to_allocvec(&cow).unwrap();
            let de: VarZeroCow<'_, CodePointSetULE> = postcard::from_bytes(&bytes).unwrap();
            assert!(!de.is_owned(), "expected zero-copy borrow for {list:?}");
            assert_eq!(de.as_bytes(), cow.as_bytes());
        }

        // Full RuleBasedTransliterator postcard zero-copy borrow check
        let rbt = RuleBasedTransliterator {
            visibility: true,
            variable_table: VarTable {
                compounds: Default::default(),
                quantifiers_opt: Default::default(),
                quantifiers_kleene: Default::default(),
                quantifiers_kleene_plus: Default::default(),
                segments: Default::default(),
                unicode_sets: VarZeroVec::from(&[UnicodeSet {
                    cp_inv_list: CodePointSetULE::from_inversion_list(&sample_list),
                    str_list: VarZeroVec::from(&["ab", "cd"]),
                }]),
                function_calls: VarZeroVec::from(&[FunctionCall {
                    translit: SimpleId {
                        filter: CodePointSetULE::from_inversion_list(&sample_list),
                        id: Cow::Borrowed("Any-Hex"),
                    },
                    arg: Cow::Borrowed("x"),
                }]),
                max_left_placeholder_count: 0,
                max_right_placeholder_count: 0,
            },
            filter: CodePointSetULE::from_inversion_list(&sample_list),
            id_group_list: VarZeroVec::from(&[&[SimpleId {
                filter: CodePointSetULE::new_all_cow(),
                id: Cow::Borrowed("Lower"),
            }][..]]),
            rule_group_list: VarZeroVec::from(&[&[Rule {
                ante: Cow::Borrowed(""),
                key: Cow::Borrowed("a"),
                post: Cow::Borrowed(""),
                replacer: Cow::Borrowed("b"),
            }][..]]),
        };

        let rbt_bytes = postcard::to_allocvec(&rbt).unwrap();
        let rbt_de: RuleBasedTransliterator<'_> = postcard::from_bytes(&rbt_bytes).unwrap();
        assert!(!rbt_de.filter.is_owned());
        assert_eq!(rbt_de, rbt);

        let rbt_json = serde_json::to_string(&rbt).unwrap();
        let rbt_json_de: RuleBasedTransliterator<'_> = serde_json::from_str(&rbt_json).unwrap();
        assert_eq!(rbt_json_de, rbt);
    }
}
