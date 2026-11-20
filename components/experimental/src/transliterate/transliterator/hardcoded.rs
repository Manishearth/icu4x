// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module defines implementations for code-based transliterators that are part of
//! transform rules.

use super::replaceable::Replaceable;
use alloc::string::String;

/// A transliterator that replaces every character with its `case`-case hexadecimal representation,
/// 0-padded to `min_length`, and surrounded by `prefix` and `suffix`.
#[derive(Debug)]
pub(super) struct HexTransliterator {
    prefix: &'static str,
    suffix: &'static str,
    min_length: u8,
    case: Case,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Case {
    Upper,
    Lower,
}

impl HexTransliterator {
    pub(super) fn new(
        prefix: &'static str,
        suffix: &'static str,
        min_length: u8,
        case: Case,
    ) -> Self {
        Self {
            prefix,
            suffix,
            min_length,
            case,
        }
    }

    pub(super) fn transliterate(&self, mut rep: Replaceable) {
        let input = rep.as_str_modifiable();
        if input.is_empty() {
            return;
        }
        let per_char_min = self.prefix.len() + self.min_length as usize + self.suffix.len();
        let mut out = String::with_capacity(input.len().saturating_mul(per_char_min));
        let base_hex = if self.case == Case::Lower { b'a' } else { b'A' };
        for c in input.chars() {
            let c_u32 = c as u32;
            // Rounding-up division by 4 (`0` has `length = 0`, matching padded-zero output).
            let length = (u32::BITS - c_u32.leading_zeros()).div_ceil(4) as usize;
            let padding = (self.min_length as usize).saturating_sub(length);

            out.push_str(self.prefix);
            for _ in 0..padding {
                out.push('0');
            }
            for shift in (0..length).rev() {
                let nibble = ((c_u32 >> (shift * 4)) & 0xF) as u8;
                let digit = if nibble < 10 {
                    b'0' + nibble
                } else {
                    base_hex + (nibble - 10)
                };
                out.push(digit as char);
            }
            out.push_str(self.suffix);
        }
        rep.replace_modifiable_with_str(&out);
    }
}
