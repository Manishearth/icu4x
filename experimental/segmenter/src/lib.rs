// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// TODO: Document all exported types in this module
// #![warn(missing_docs)]

//! A segmenter implementation for the following rules.
//!
//! - Line breaker that is compatible with [Unicode Standard Annex #14][UAX14] and CSS properties.
//! - Grapheme cluster breaker, word breaker, and sentence breaker that are compatible with
//!   [Unicode Standard Annex #29][UAX29].
//!
//! [UAX14]: https://www.unicode.org/reports/tr14/
//! [UAX29]: https://www.unicode.org/reports/tr29/
//!
//! # Examples
//!
//! ## Line Break
//!
//! Segment a string with default options:
//!
//!```rust
//! use icu_segmenter::LineBreakSegmenter;
//!
//! let provider = icu_provider::inv::InvariantDataProvider;
//! let segmenter = LineBreakSegmenter::try_new(&provider)
//!     .expect("Data exists");
//!
//! let breakpoints: Vec<usize> = segmenter.segment_str("Hello World").collect();
//! assert_eq!(&breakpoints, &[6, 11]);
//! ```
//!
//! Segment a string with CSS option overrides:
//!
//! ```rust
//! use icu_segmenter::{LineBreakSegmenter, LineBreakOptions, LineBreakRule, WordBreakRule};
//!
//! let mut options = LineBreakOptions::default();
//! options.line_break_rule = LineBreakRule::Strict;
//! options.word_break_rule = WordBreakRule::BreakAll;
//! options.ja_zh = false;
//! let provider = icu_provider::inv::InvariantDataProvider;
//! let segmenter = LineBreakSegmenter::try_new_with_options(&provider, options)
//!     .expect("Data exists");
//!
//! let breakpoints: Vec<usize> = segmenter.segment_str("Hello World").collect();
//! assert_eq!(&breakpoints, &[1, 2, 3, 4, 6, 7, 8, 9, 10, 11]);
//! ```
//!
//! Segment a Latin1 byte string:
//!
//! ```rust
//! use icu_segmenter::LineBreakSegmenter;
//!
//! let provider = icu_provider::inv::InvariantDataProvider;
//! let segmenter = LineBreakSegmenter::try_new(&provider)
//!     .expect("Data exists");
//!
//! let breakpoints: Vec<usize> = segmenter.segment_latin1(b"Hello World").collect();
//! assert_eq!(&breakpoints, &[6, 11]);
//! ```
//!
//! ## Grapheme Cluster Break
//!
//! Segment a string:
//!
//!```rust
//! use icu_segmenter::GraphemeClusterBreakIterator;
//!
//! let breakpoints: Vec<usize> = GraphemeClusterBreakIterator::new("Hello 🗺").collect();
//! assert_eq!(&breakpoints, &[0, 1, 2, 3, 4, 5, 6, 10]); // World Map (U+1F5FA) is encoded in four bytes in UTF-8.
//! ```
//!
//! Segment a Latin1 byte string:
//!
//! ```rust
//! use icu_segmenter::GraphemeClusterBreakIteratorLatin1;
//!
//! let breakpoints: Vec<usize> = GraphemeClusterBreakIteratorLatin1::new(b"Hello World").collect();
//! assert_eq!(&breakpoints, &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
//! ```
//!
//! ## Word Break
//!
//! Segment a string:
//!
//!```rust
//! use icu_segmenter::WordBreakIterator;
//!
//! let breakpoints: Vec<usize> = WordBreakIterator::new("Hello World").collect();
//! assert_eq!(&breakpoints, &[0, 5, 6, 11]);
//! ```
//!
//! Segment a Latin1 byte string:
//!
//! ```rust
//! use icu_segmenter::WordBreakIteratorLatin1;
//!
//! let breakpoints: Vec<usize> = WordBreakIteratorLatin1::new(b"Hello World").collect();
//! assert_eq!(&breakpoints, &[0, 5, 6, 11]);
//! ```
//!
//! ## Sentence Break
//!
//! Segment a string:
//!
//!```rust
//! use icu_segmenter::SentenceBreakIterator;
//!
//! let breakpoints: Vec<usize> = SentenceBreakIterator::new("Hello World").collect();
//! assert_eq!(&breakpoints, &[0, 11]);
//! ```
//!
//! Segment a Latin1 byte string:
//!
//! ```rust
//! use icu_segmenter::SentenceBreakIteratorLatin1;
//!
//! let breakpoints: Vec<usize> = SentenceBreakIteratorLatin1::new(b"Hello World").collect();
//! assert_eq!(&breakpoints, &[0, 11]);
//! ```
//!
//! # Generating property table
//!
//! Copy the following files to `tools` directory. Then run `./generate_properties.py` in `tools` directory (requires Python 3.8+). Machine generated files are moved to `src` directory.
//! - <https://www.unicode.org/Public/UCD/latest/ucd/LineBreak.txt>
//! - <https://www.unicode.org/Public/UCD/latest/ucd/EastAsianWidth.txt>
//! - <https://www.unicode.org/Public/UCD/latest/ucd/emoji/emoji-data.txt>

#![cfg_attr(not(any(test, feature = "std")), no_std)]

extern crate alloc;

mod indices;
mod language;
mod lb_define;
mod line_breaker;
mod properties_defines;
mod properties_other;
mod property_table;
mod rule_segmenter;
mod rule_table;

mod grapheme;
mod sentence;
mod word;

pub mod provider;

#[cfg(feature = "lstm")]
#[macro_use]
extern crate lazy_static;

// Use the LSTM when the feature is enabled.
#[cfg(feature = "lstm")]
mod lstm;

// No-op functions when LSTM is disabled.
#[cfg(not(feature = "lstm"))]
mod lstm {
    use alloc::vec::Vec;
    pub fn get_line_break_utf16(_: &[u16]) -> Option<Vec<usize>> {
        None
    }
    pub fn get_line_break_utf8(_: &str) -> Option<Vec<usize>> {
        None
    }
}

pub use crate::grapheme::{
    GraphemeClusterBreakIterator, GraphemeClusterBreakIteratorLatin1,
    GraphemeClusterBreakIteratorUtf16,
};
pub use crate::line_breaker::*;
pub use crate::sentence::{
    SentenceBreakIterator, SentenceBreakIteratorLatin1, SentenceBreakIteratorUtf16,
};
pub use crate::word::{WordBreakIterator, WordBreakIteratorLatin1, WordBreakIteratorUtf16};
