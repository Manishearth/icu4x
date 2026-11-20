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
use alloc::vec::Vec;

use icu_collections::{
    codepointinvlist::{CodePointInversionList, CodePointInversionListULE},
    codepointinvliststringlist::CodePointInversionListAndStringListULE,
};
use icu_provider::prelude::*;
use vecs::Index32;
use zerovec::ule::{EncodeAsVarULE, UleError, VarULE};
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
    pub filter: CodePointInversionList<'a>,
    /// The list of transform rule groups this transliterator uses.
    pub id_group_list: VarZeroVec<'a, VarZeroSlice<SimpleIdULE>>,
    /// The list of conversion rule groups this transliterator uses.
    pub rule_group_list: VarZeroVec<'a, RuleGroupULE, Index32>,
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
            pub filter: CodePointInversionList<'a>,
            #[serde(borrow)]
            pub id_group_list: VarZeroVec<'a, VarZeroSlice<SimpleIdULE>>,
            #[serde(borrow)]
            pub rule_group_list: VarZeroVec<'a, RuleGroupULE, Index32>,
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
    #[zerovec::varule(CodePointInversionListULE)]
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub filter: CodePointInversionList<'a>,
    /// The ID of the transliterator.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub id: Cow<'a, str>,
}

/// A conversion rule. The source patterns as well as the replacer use inlined private use characters
/// that refer to elements of the [`VarTable`] for special matchers (variables, `UnicodeSets`, ...).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(serde::Serialize))]
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

/// A zero-copy representation of a group of conversion [`Rule`]s stored sequentially without an offset table.
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct RuleGroupULE([u8]);

impl core::fmt::Debug for RuleGroupULE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl RuleGroupULE {
    /// Returns `true` if this rule group contains no rules.
    pub const fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns an iterator over the [`Rule`]s in this group.
    pub fn iter(&self) -> RuleGroupIter<'_> {
        RuleGroupIter { bytes: &self.0 }
    }
}

/// An iterator over the [`Rule`]s in a [`RuleGroupULE`].
#[derive(Debug, Clone)]
pub struct RuleGroupIter<'a> {
    // Safety invariant: `bytes` begins on a record boundary and encodes zero or more valid rule
    // records as checked by `RuleGroupULE::validate_bytes`.
    bytes: &'a [u8],
}

impl<'a> Iterator for RuleGroupIter<'a> {
    type Item = Rule<'a>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.bytes.is_empty() {
            return None;
        }
        let mut pos = 0;
        let (ante_len, key_len, post_len, replacer_len) =
            decode_record_header(self.bytes, &mut pos)?;
        let post_offset = ante_len.checked_add(key_len)?;
        let replacer_offset = post_offset.checked_add(post_len)?;
        let record_text_len = replacer_offset.checked_add(replacer_len)?;
        let record_end = pos.checked_add(record_text_len)?;
        let record_text = self.bytes.get(pos..record_end)?;
        let rest = self.bytes.get(record_end..)?;

        // SAFETY: `RuleGroupULE::validate_bytes` guarantees `record_text` is valid UTF-8 and
        // `ante_len`, `post_offset`, and `replacer_offset` lie on UTF-8 char boundaries.
        let (ante, key, post, replacer) = unsafe {
            let text = core::str::from_utf8_unchecked(record_text);
            (
                text.get_unchecked(..ante_len),
                text.get_unchecked(ante_len..post_offset),
                text.get_unchecked(post_offset..replacer_offset),
                text.get_unchecked(replacer_offset..),
            )
        };
        // Maintain the safety invariant by advancing past this record.
        self.bytes = rest;

        Some(Rule {
            ante: Cow::Borrowed(ante),
            key: Cow::Borrowed(key),
            post: Cow::Borrowed(post),
            replacer: Cow::Borrowed(replacer),
        })
    }
}

impl core::iter::FusedIterator for RuleGroupIter<'_> {}

#[inline]
fn read_u8(bytes: &[u8], pos: &mut usize) -> Option<u8> {
    let b = *bytes.get(*pos)?;
    *pos = pos.checked_add(1)?;
    Some(b)
}

#[inline]
fn read_u32_le_as_usize(bytes: &[u8], pos: &mut usize) -> Option<usize> {
    let end = pos.checked_add(4)?;
    let slice = bytes.get(*pos..end)?;
    let arr: [u8; 4] = slice.try_into().ok()?;
    *pos = end;
    usize::try_from(u32::from_le_bytes(arr)).ok()
}

#[inline]
fn decode_record_header(bytes: &[u8], pos: &mut usize) -> Option<(usize, usize, usize, usize)> {
    let hdr = read_u8(bytes, pos)?;
    if hdr < 0xF0 {
        return Some((0, usize::from(hdr >> 4), 0, usize::from(hdr & 0x0F)));
    }
    if hdr < 0xF4 {
        let has_ante = (hdr & 0x01) != 0;
        let has_post = (hdr & 0x02) != 0;
        let ante_len = if has_ante {
            let len = usize::from(read_u8(bytes, pos)?);
            if len == 0 {
                return None;
            }
            len
        } else {
            0
        };
        let key_len = usize::from(read_u8(bytes, pos)?);
        let post_len = if has_post {
            let len = usize::from(read_u8(bytes, pos)?);
            if len == 0 {
                return None;
            }
            len
        } else {
            0
        };
        let replacer_len = usize::from(read_u8(bytes, pos)?);
        if !has_ante && !has_post && key_len < 15 && replacer_len < 16 {
            return None;
        }
        return Some((ante_len, key_len, post_len, replacer_len));
    }
    if hdr < 0xF8 {
        let has_ante = (hdr & 0x01) != 0;
        let has_post = (hdr & 0x02) != 0;
        let ante_len = if has_ante {
            let len = read_u32_le_as_usize(bytes, pos)?;
            if len == 0 {
                return None;
            }
            len
        } else {
            0
        };
        let key_len = read_u32_le_as_usize(bytes, pos)?;
        let post_len = if has_post {
            let len = read_u32_le_as_usize(bytes, pos)?;
            if len == 0 {
                return None;
            }
            len
        } else {
            0
        };
        let replacer_len = read_u32_le_as_usize(bytes, pos)?;
        if ante_len <= 255 && key_len <= 255 && post_len <= 255 && replacer_len <= 255 {
            return None;
        }
        return Some((ante_len, key_len, post_len, replacer_len));
    }
    None
}

// Safety (based on the `VarULE` safety checklist):
// 1. `RuleGroupULE` is `#[repr(transparent)]` over `[u8]`, so it has no uninitialized or padding bytes.
// 2. Its alignment is 1 byte, matching `[u8]`.
// 3. `validate_bytes` checks every record header, canonicality, length bounds, UTF-8 validity, and field char boundaries.
// 4. `validate_bytes` consumes `bytes` sequentially from 0 and rejects trailing or truncated data.
// 5. `from_bytes_unchecked` returns `&Self` referencing the same validated byte slice.
// 6. `as_bytes` returns `&self.0`, matching the default `VarULE` impl; all other methods use defaults.
// 7. Canonical encoding is enforced in `validate_bytes`, so byte equality matches semantic equality.
// 8. `RuleGroupULE` is `#[repr(transparent)]` over `[u8]`, so fat pointer metadata is the byte length.
unsafe impl VarULE for RuleGroupULE {
    fn validate_bytes(bytes: &[u8]) -> Result<(), UleError> {
        let mut pos = 0;
        while pos < bytes.len() {
            let (ante_len, key_len, post_len, replacer_len) =
                decode_record_header(bytes, &mut pos).ok_or_else(UleError::parse::<Self>)?;
            let post_offset = ante_len
                .checked_add(key_len)
                .ok_or_else(UleError::parse::<Self>)?;
            let replacer_offset = post_offset
                .checked_add(post_len)
                .ok_or_else(UleError::parse::<Self>)?;
            let record_text_len = replacer_offset
                .checked_add(replacer_len)
                .ok_or_else(UleError::parse::<Self>)?;
            let record_end = pos
                .checked_add(record_text_len)
                .ok_or_else(UleError::parse::<Self>)?;
            let record_text = bytes
                .get(pos..record_end)
                .ok_or_else(UleError::parse::<Self>)?;
            let text = core::str::from_utf8(record_text).map_err(|_| UleError::parse::<Self>())?;
            if !text.is_char_boundary(ante_len)
                || !text.is_char_boundary(post_offset)
                || !text.is_char_boundary(replacer_offset)
            {
                return Err(UleError::parse::<Self>());
            }
            pos = record_end;
        }
        Ok(())
    }

    unsafe fn from_bytes_unchecked(bytes: &[u8]) -> &Self {
        // SAFETY: `RuleGroupULE` is `#[repr(transparent)]` over `[u8]`, and the caller guarantees
        // `bytes` passed `RuleGroupULE::validate_bytes`.
        unsafe { &*(bytes as *const [u8] as *const Self) }
    }

    #[inline]
    fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl Rule<'_> {
    fn encoded_len(&self) -> usize {
        let ante_len = self.ante.len();
        let key_len = self.key.len();
        let post_len = self.post.len();
        let replacer_len = self.replacer.len();
        let text_len = ante_len + key_len + post_len + replacer_len;
        let hdr_and_lens_len =
            if ante_len == 0 && post_len == 0 && key_len < 15 && replacer_len < 16 {
                1
            } else if ante_len <= 255 && key_len <= 255 && post_len <= 255 && replacer_len <= 255 {
                1 + usize::from(ante_len != 0) + 1 + usize::from(post_len != 0) + 1
            } else {
                assert!(
                    u32::try_from(ante_len).is_ok()
                        && u32::try_from(key_len).is_ok()
                        && u32::try_from(post_len).is_ok()
                        && u32::try_from(replacer_len).is_ok(),
                    "Rule field exceeds u32::MAX bytes"
                );
                1 + 4 * usize::from(ante_len != 0) + 4 + 4 * usize::from(post_len != 0) + 4
            };
        hdr_and_lens_len + text_len
    }

    fn encode_write(&self, dst: &mut &mut [u8]) {
        #[inline]
        fn write_slice(dst: &mut &mut [u8], src: &[u8]) {
            if let Some((head, tail)) = core::mem::take(dst).split_at_mut_checked(src.len()) {
                head.copy_from_slice(src);
                *dst = tail;
            } else {
                debug_assert!(false, "encode_var_ule_write buffer too small");
            }
        }

        let ante_len = self.ante.len();
        let key_len = self.key.len();
        let post_len = self.post.len();
        let replacer_len = self.replacer.len();

        if ante_len == 0 && post_len == 0 && key_len < 15 && replacer_len < 16 {
            let hdr = ((key_len as u8) << 4) | (replacer_len as u8);
            write_slice(dst, &[hdr]);
        } else if ante_len <= 255 && key_len <= 255 && post_len <= 255 && replacer_len <= 255 {
            let has_ante = ante_len != 0;
            let has_post = post_len != 0;
            let hdr = 0xF0 | u8::from(has_ante) | (u8::from(has_post) << 1);
            write_slice(dst, &[hdr]);
            if has_ante {
                write_slice(dst, &[ante_len as u8]);
            }
            write_slice(dst, &[key_len as u8]);
            if has_post {
                write_slice(dst, &[post_len as u8]);
            }
            write_slice(dst, &[replacer_len as u8]);
        } else {
            let (Ok(ante_u32), Ok(key_u32), Ok(post_u32), Ok(replacer_u32)) = (
                u32::try_from(ante_len),
                u32::try_from(key_len),
                u32::try_from(post_len),
                u32::try_from(replacer_len),
            ) else {
                unreachable!("Rule field exceeds u32::MAX bytes");
            };
            let has_ante = ante_len != 0;
            let has_post = post_len != 0;
            let hdr = 0xF4 | u8::from(has_ante) | (u8::from(has_post) << 1);
            write_slice(dst, &[hdr]);
            if has_ante {
                write_slice(dst, &ante_u32.to_le_bytes());
            }
            write_slice(dst, &key_u32.to_le_bytes());
            if has_post {
                write_slice(dst, &post_u32.to_le_bytes());
            }
            write_slice(dst, &replacer_u32.to_le_bytes());
        }

        if ante_len != 0 {
            write_slice(dst, self.ante.as_bytes());
        }
        write_slice(dst, self.key.as_bytes());
        if post_len != 0 {
            write_slice(dst, self.post.as_bytes());
        }
        write_slice(dst, self.replacer.as_bytes());
    }
}

// SAFETY: `encode_var_ule_len` returns the exact byte count written by `encode_var_ule_write`,
// and `encode_var_ule_write` produces canonical records accepted by `RuleGroupULE::validate_bytes`.
unsafe impl EncodeAsVarULE<RuleGroupULE> for [Rule<'_>] {
    fn encode_var_ule_as_slices<R>(&self, _: impl FnOnce(&[&[u8]]) -> R) -> R {
        // Unnecessary because `encode_var_ule_len` and `encode_var_ule_write` are implemented.
        unreachable!()
    }

    fn encode_var_ule_len(&self) -> usize {
        self.iter().map(Rule::encoded_len).sum()
    }

    fn encode_var_ule_write(&self, mut dst: &mut [u8]) {
        debug_assert_eq!(dst.len(), self.encode_var_ule_len());
        for rule in self {
            rule.encode_write(&mut dst);
        }
        debug_assert!(dst.is_empty());
    }
}

// SAFETY: Delegates both `encode_var_ule_len` and `encode_var_ule_write` to `[Rule<'_>]`.
unsafe impl EncodeAsVarULE<RuleGroupULE> for Vec<Rule<'_>> {
    fn encode_var_ule_as_slices<R>(&self, _: impl FnOnce(&[&[u8]]) -> R) -> R {
        // Unnecessary because `encode_var_ule_len` and `encode_var_ule_write` are implemented.
        unreachable!()
    }

    fn encode_var_ule_len(&self) -> usize {
        self.as_slice().encode_var_ule_len()
    }

    fn encode_var_ule_write(&self, dst: &mut [u8]) {
        self.as_slice().encode_var_ule_write(dst)
    }
}

// SAFETY: Delegates both `encode_var_ule_len` and `encode_var_ule_write` to `[Rule<'_>]`.
unsafe impl EncodeAsVarULE<RuleGroupULE> for &[Rule<'_>] {
    fn encode_var_ule_as_slices<R>(&self, _: impl FnOnce(&[&[u8]]) -> R) -> R {
        // Unnecessary because `encode_var_ule_len` and `encode_var_ule_write` are implemented.
        unreachable!()
    }

    fn encode_var_ule_len(&self) -> usize {
        (*self).encode_var_ule_len()
    }

    fn encode_var_ule_write(&self, dst: &mut [u8]) {
        (*self).encode_var_ule_write(dst)
    }
}

#[cfg(feature = "datagen")]
impl serde::Serialize for RuleGroupULE {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if serializer.is_human_readable() {
            serializer.collect_seq(self.iter())
        } else {
            serializer.serialize_bytes(&self.0)
        }
    }
}

#[cfg(feature = "serde")]
impl<'de, 'data> serde::Deserialize<'de> for &'data RuleGroupULE
where
    'de: 'data,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            Err(serde::de::Error::custom(
                "&RuleGroupULE cannot be deserialized from human-readable formats",
            ))
        } else {
            let bytes = <&[u8]>::deserialize(deserializer)?;
            RuleGroupULE::parse_bytes(bytes).map_err(serde::de::Error::custom)
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Box<RuleGroupULE> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        if deserializer.is_human_readable() {
            let rules = Vec::<Rule<'de>>::deserialize(deserializer)?;
            Ok(ule::encode_varule_to_box(&rules))
        } else {
            struct BytesVisitor;
            impl serde::de::Visitor<'_> for BytesVisitor {
                type Value = Box<RuleGroupULE>;
                fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.write_str("RuleGroupULE bytes")
                }
                fn visit_bytes<E: Error>(self, v: &[u8]) -> Result<Self::Value, E> {
                    RuleGroupULE::parse_bytes(v)
                        .map(VarULE::to_boxed)
                        .map_err(E::custom)
                }
            }
            deserializer.deserialize_bytes(BytesVisitor)
        }
    }
}

/// The special matchers and replacers used by this transliterator.
#[derive(Debug, Clone, zerofrom::ZeroFrom, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_experimental::transliterate::provider))]
pub struct VarTable<'a> {
    /// Variable definitions.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub compounds: VarZeroVec<'a, str>,
    /// Zero or one quantifiers.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub quantifiers_opt: VarZeroVec<'a, str>,
    /// Zero or more quantifiers.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub quantifiers_kleene: VarZeroVec<'a, str>,
    /// One or more quantifiers.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub quantifiers_kleene_plus: VarZeroVec<'a, str>,
    /// Segments.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub segments: VarZeroVec<'a, SegmentULE>,
    /// `UnicodeSets`. These are represented as a [`CodePointInversionListAndStringList`](icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList)
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub unicode_sets: VarZeroVec<'a, CodePointInversionListAndStringListULE, Index32>,
    /// Function calls.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub function_calls: VarZeroVec<'a, FunctionCallULE>,
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
    use core::hint::black_box;
    use std::alloc::{GlobalAlloc, Layout, System};
    use std::cell::Cell;

    struct MeasuringAllocator;

    impl MeasuringAllocator {
        thread_local! {
            static ACTIVE: Cell<bool> = const { Cell::new(false) };
            static TOTAL_ALLOCATED: Cell<usize> = const { Cell::new(0) };
            static TOTAL_DEALLOCATED: Cell<usize> = const { Cell::new(0) };
        }

        fn start_measure() {
            Self::TOTAL_ALLOCATED.with(|c| c.set(0));
            Self::TOTAL_DEALLOCATED.with(|c| c.set(0));
            Self::ACTIVE.with(|c| c.set(true));
        }

        fn end_measure() -> (usize, usize) {
            Self::ACTIVE.with(|c| c.set(false));
            (
                Self::TOTAL_ALLOCATED.with(|c| c.take()),
                Self::TOTAL_DEALLOCATED.with(|c| c.take()),
            )
        }
    }

    unsafe impl GlobalAlloc for MeasuringAllocator {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            if Self::ACTIVE.with(|f| f.get()) {
                Self::TOTAL_ALLOCATED.with(|c| c.set(c.get() + layout.size()));
            }
            // SAFETY: Forwards directly to the system allocator with the same layout.
            unsafe { System.alloc(layout) }
        }

        unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
            if Self::ACTIVE.with(|f| f.get()) {
                Self::TOTAL_DEALLOCATED.with(|c| c.set(c.get() + layout.size()));
            }
            // SAFETY: `ptr` and `layout` came from a matching `System.alloc` call.
            unsafe { System.dealloc(ptr, layout) }
        }
    }

    #[global_allocator]
    static ALLOCATOR: MeasuringAllocator = MeasuringAllocator;

    fn rule<'a>(ante: &'a str, key: &'a str, post: &'a str, replacer: &'a str) -> Rule<'a> {
        Rule {
            ante: Cow::Borrowed(ante),
            key: Cow::Borrowed(key),
            post: Cow::Borrowed(post),
            replacer: Cow::Borrowed(replacer),
        }
    }

    #[test]
    fn test_empty_rule_group() {
        let empty = RuleGroupULE::parse_bytes(b"").unwrap();
        assert!(empty.is_empty());
        assert_eq!(empty.iter().count(), 0);
        assert_eq!(format!("{empty:?}"), "[]");

        let boxed = ule::encode_varule_to_box::<[Rule<'_>], RuleGroupULE>(&[]);
        assert!(boxed.is_empty());
        assert_eq!(boxed.as_bytes(), b"");
        assert_eq!(&*boxed, empty);
    }

    #[test]
    fn test_short_context_free_encoding() {
        let max_key = "a".repeat(14);
        let max_rep = "b".repeat(15);
        let rules = vec![
            rule("", "", "", ""),
            rule("", "x", "", "yy"),
            rule("", &max_key, "", &max_rep),
        ];
        let boxed = ule::encode_varule_to_box(&rules);
        assert_eq!(
            boxed.as_bytes(),
            [
                &[0x00][..],
                &[0x12, b'x', b'y', b'y'][..],
                &[0xEF][..],
                max_key.as_bytes(),
                max_rep.as_bytes(),
            ]
            .concat()
            .as_slice()
        );
        let parsed = RuleGroupULE::parse_bytes(boxed.as_bytes()).unwrap();
        assert!(!parsed.is_empty());
        let mut iter = parsed.iter();
        assert_eq!(iter.next().as_ref(), Some(&rules[0]));
        assert_eq!(iter.next().as_ref(), Some(&rules[1]));
        assert_eq!(iter.next().as_ref(), Some(&rules[2]));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_medium_u8_encoding_and_contexts() {
        let key_15 = "k".repeat(15);
        let rep_16 = "r".repeat(16);
        let s_255 = "z".repeat(255);
        let rules = vec![
            // key.len() == 15 forces 0xF0
            rule("", &key_15, "", "a"),
            // replacer.len() == 16 forces 0xF0
            rule("", "a", "", &rep_16),
            // ante only -> 0xF1
            rule("a", "b", "", "c"),
            // post only -> 0xF2
            rule("", "b", "p", "c"),
            // ante and post at 255 bytes -> 0xF3
            rule(&s_255, &s_255, &s_255, &s_255),
        ];
        let boxed = ule::encode_varule_to_box(&rules);
        let bytes = boxed.as_bytes();
        assert_eq!(bytes[0], 0xF0);
        assert_eq!(bytes[1], 15);
        assert_eq!(bytes[2], 1);

        let parsed = RuleGroupULE::parse_bytes(bytes).unwrap();
        let decoded: Vec<Rule<'_>> = parsed.iter().collect();
        assert_eq!(decoded, rules);
    }

    #[test]
    fn test_long_u32_encoding_and_mixed_group() {
        let s_256 = "ä".repeat(128); // 256 UTF-8 bytes
        let s_300 = "x".repeat(300);
        let rules = vec![
            rule("", "a", "", "b"),
            rule("", &s_256, "", "c"),
            rule(&s_256, "k", "", "c"),
            rule("", "k", &s_256, "c"),
            rule(&s_256, &s_300, &s_256, &s_300),
            rule("ante", "key", "post", "rep"),
        ];
        let boxed = ule::encode_varule_to_box(&rules);
        let bytes = boxed.as_bytes();
        // Record 0: 0x11, 'a', 'b' -> 3 bytes; record 1 starts at 3 with 0xF4
        assert_eq!(bytes[0], 0x11);
        assert_eq!(bytes[3], 0xF4);

        let parsed = RuleGroupULE::parse_bytes(bytes).unwrap();
        let decoded: Vec<Rule<'_>> = parsed.iter().collect();
        assert_eq!(decoded, rules);

        let vzv: VarZeroVec<'_, RuleGroupULE, Index32> =
            VarZeroVec::from(&vec![Vec::new(), rules.clone()]);
        assert_eq!(vzv.len(), 2);
        assert!(vzv.get(0).unwrap().is_empty());
        assert_eq!(vzv.get(1).unwrap().iter().collect::<Vec<_>>(), rules);
    }

    #[test]
    fn test_invalid_bytes() {
        let invalid_cases: &[(&str, &[u8])] = &[
            ("truncated fast-path body (missing all)", &[0x11]),
            ("truncated fast-path body (missing 1)", &[0x11, b'a']),
            ("truncated 0xF0 header (0 len bytes)", &[0xF0]),
            ("truncated 0xF0 header (1 len byte)", &[0xF0, 15]),
            ("truncated 0xF1 header (2 len bytes)", &[0xF1, 1, 0]),
            ("truncated 0xF3 header (3 len bytes)", &[0xF3, 1, 0, 1]),
            ("truncated 0xF0 body", &[0xF0, 15, 1, b'a']),
            ("truncated 0xF4 header", &[0xF4, 0, 1, 0]),
            ("truncated 0xF5 header", &[0xF5, 1, 0, 0, 0, 0, 0]),
            ("truncated 0xF4 body", &[0xF4, 0, 1, 0, 0, 0, 0, 0, 0, b'a']),
            ("invalid UTF-8 in fast path", &[0x11, 0xFF, b'a']),
            (
                "invalid UTF-8 in u8 path",
                &[0xF1, 1, 1, 1, 0xFF, b'a', b'b'],
            ),
            (
                "split 2-byte UTF-8 across key/replacer",
                &[0x11, 0xC3, 0xA4],
            ),
            (
                "split 2-byte UTF-8 across ante/key",
                &[0xF1, 1, 1, 0, 0xC3, 0xA4],
            ),
            (
                "split 2-byte UTF-8 across key/post",
                &[0xF2, 1, 1, 0, 0xC3, 0xA4],
            ),
            (
                "split 4-byte UTF-8 across post/replacer",
                &[0xF2, 0, 2, 2, 0xF3, 0xB0, 0x80, 0x80],
            ),
            ("reserved header 0xF8", &[0xF8]),
            ("reserved header 0xFF", &[0xFF]),
            ("non-canonical 0xF0 for empty fast-path rule", &[0xF0, 0, 0]),
            (
                "non-canonical 0xF0 at fast-path boundary",
                &[
                    0xF0, 14, 15, b'a', b'a', b'a', b'a', b'a', b'a', b'a', b'a', b'a', b'a', b'a',
                    b'a', b'a', b'a', b'b', b'b', b'b', b'b', b'b', b'b', b'b', b'b', b'b', b'b',
                    b'b', b'b', b'b', b'b', b'b',
                ],
            ),
            (
                "non-canonical 0xF1 with ante_len=0",
                &[0xF1, 0, 1, 1, b'a', b'b'],
            ),
            (
                "non-canonical 0xF2 with post_len=0",
                &[0xF2, 1, 0, 1, b'a', b'b'],
            ),
            (
                "non-canonical 0xF3 with ante_len=0",
                &[0xF3, 0, 1, 1, 1, b'a', b'b', b'c'],
            ),
            (
                "non-canonical 0xF3 with post_len=0",
                &[0xF3, 1, 1, 0, 1, b'a', b'b', b'c'],
            ),
            (
                "non-canonical 0xF4 with all lengths <= 255",
                &[
                    0xF4, 15, 0, 0, 0, 0, 0, 0, 0, b'a', b'a', b'a', b'a', b'a', b'a', b'a', b'a',
                    b'a', b'a', b'a', b'a', b'a', b'a', b'a',
                ],
            ),
            (
                "non-canonical 0xF5 with ante_len=0 and key=256",
                &[0xF5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
            ),
            (
                "non-canonical 0xF6 with post_len=0 and replacer=256",
                &[0xF6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            ),
            (
                "overflow u32 lengths",
                &[
                    0xF7, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                    0xFF, 0xFF, 0xFF, 0xFF,
                ],
            ),
            (
                "second record invalid after valid first record",
                &[0x11, b'a', b'b', 0xF8],
            ),
        ];

        for (label, bytes) in invalid_cases {
            assert!(
                RuleGroupULE::parse_bytes(bytes).is_err(),
                "expected error for case: {label}"
            );
        }
    }

    #[cfg(feature = "datagen")]
    #[test]
    fn test_serde_json_and_postcard_roundtrip() {
        let long_str = "x".repeat(300);
        let groups = vec![
            vec![
                rule("", "a", "", "b"),
                rule("ante", "key_longer_than_15", "post", "replacer"),
                rule("", &long_str, "", &long_str),
            ],
            Vec::new(),
        ];

        let rbt = RuleBasedTransliterator {
            visibility: true,
            variable_table: VarTable {
                compounds: VarZeroVec::from(&["\u{F0001}"]),
                quantifiers_opt: VarZeroVec::from(&["a"]),
                quantifiers_kleene: VarZeroVec::new(),
                quantifiers_kleene_plus: VarZeroVec::new(),
                segments: VarZeroVec::from(&[Segment {
                    idx: 0,
                    content: Cow::Borrowed("s"),
                }]),
                unicode_sets: VarZeroVec::new(),
                function_calls: VarZeroVec::from(&[FunctionCall {
                    translit: SimpleId {
                        filter: CodePointInversionList::all(),
                        id: Cow::Borrowed("any-lower"),
                    },
                    arg: Cow::Borrowed("x"),
                }]),
                max_left_placeholder_count: 1,
                max_right_placeholder_count: 2,
            },
            filter: CodePointInversionList::all(),
            id_group_list: VarZeroVec::from(&[
                VarZeroVec::from(&[SimpleId {
                    filter: CodePointInversionList::all(),
                    id: Cow::Borrowed("any-nfc"),
                }]),
                VarZeroVec::new(),
            ]),
            rule_group_list: VarZeroVec::from(&groups),
        };

        // JSON schema and round-trip
        let json = serde_json::to_string(&rbt).unwrap();
        assert!(json.contains(
            r#""rule_group_list":[[{"ante":"","key":"a","post":"","replacer":"b"},{"ante":"ante","key":"key_longer_than_15","post":"post","replacer":"replacer"}"#
        ));
        let from_json: RuleBasedTransliterator<'_> = serde_json::from_str(&json).unwrap();
        assert_eq!(from_json, rbt);

        // Box<RuleGroupULE> JSON (including non-borrowing reader) and postcard round-trip
        let boxed_group = ule::encode_varule_to_box(&groups[0]);
        let group_json = serde_json::to_string(&boxed_group).unwrap();
        let group_from_json: Box<RuleGroupULE> = serde_json::from_str(&group_json).unwrap();
        assert_eq!(group_from_json, boxed_group);
        let group_from_json_reader: Box<RuleGroupULE> =
            serde_json::from_reader(group_json.as_bytes()).unwrap();
        assert_eq!(group_from_json_reader, boxed_group);

        let group_postcard = postcard::to_allocvec(&boxed_group).unwrap();
        let group_from_postcard: Box<RuleGroupULE> = postcard::from_bytes(&group_postcard).unwrap();
        assert_eq!(group_from_postcard, boxed_group);

        MeasuringAllocator::start_measure();
        let borrowed_group: &RuleGroupULE =
            black_box(postcard::from_bytes(black_box(&group_postcard)).unwrap());
        let (group_allocated, group_deallocated) = MeasuringAllocator::end_measure();
        assert_eq!(group_allocated, 0);
        assert_eq!(group_deallocated, 0);
        assert_eq!(borrowed_group, &*boxed_group);

        // Postcard round-trip for full RuleBasedTransliterator with 0-byte allocation check
        let postcard_bytes = postcard::to_allocvec(&rbt).unwrap();
        MeasuringAllocator::start_measure();
        let from_postcard: RuleBasedTransliterator<'_> =
            black_box(postcard::from_bytes(black_box(&postcard_bytes)).unwrap());
        let (allocated, deallocated) = MeasuringAllocator::end_measure();

        assert_eq!(
            allocated, 0,
            "expected 0 allocated bytes during postcard deserialization"
        );
        assert_eq!(
            deallocated, 0,
            "expected 0 deallocated bytes during postcard deserialization"
        );
        assert!(!from_postcard.rule_group_list.is_owned());
        assert!(!from_postcard.variable_table.compounds.is_owned());
        assert_eq!(from_postcard, rbt);
    }
}
