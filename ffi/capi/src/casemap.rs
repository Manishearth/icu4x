// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_casemap::options::TitlecaseOptions;

struct Write16Adapter<'a>(&'a mut diplomat_runtime::DiplomatWrite16);

impl write16::Write16 for Write16Adapter<'_> {
    #[inline(always)]
    fn write_slice(&mut self, s: &[u16]) -> core::fmt::Result {
        self.0.write_slice(s);
        Ok(())
    }

    #[inline(always)]
    fn write_char(&mut self, c: char) -> core::fmt::Result {
        self.0.write_char_as_utf16(c);
        Ok(())
    }

    #[inline(always)]
    fn size_hint(&mut self, upcoming: usize) -> core::fmt::Result {
        self.0.reserve(upcoming);
        Ok(())
    }
}

#[diplomat::bridge]
#[diplomat::abi_rename = "icu4x_{0}_mv1"]
pub mod ffi {
    use alloc::boxed::Box;

    #[cfg(any(feature = "compiled_data", feature = "buffer_provider"))]
    use crate::unstable::errors::ffi::DataError;
    use crate::unstable::locale_core::ffi::Locale;
    #[cfg(feature = "buffer_provider")]
    use crate::unstable::provider::ffi::DataProvider;
    use diplomat_runtime::DiplomatOption;
    use icu_locale_core::locale;

    use writeable::Writeable;

    #[diplomat::enum_convert(icu_casemap::options::LeadingAdjustment, needs_wildcard)]
    #[diplomat::rust_link(icu::casemap::options::LeadingAdjustment, Enum)]
    #[non_exhaustive]
    pub enum LeadingAdjustment {
        #[diplomat::attr(auto, default)]
        Auto,
        None,
        ToCased,
    }

    #[diplomat::enum_convert(icu_casemap::options::TrailingCase, needs_wildcard)]
    #[diplomat::rust_link(icu::casemap::options::TrailingCase, Enum)]
    #[non_exhaustive]
    pub enum TrailingCase {
        #[diplomat::attr(auto, default)]
        Lower,
        Unchanged,
    }

    #[diplomat::rust_link(icu::casemap::options::TitlecaseOptions, Struct)]
    #[diplomat::attr(supports = non_exhaustive_structs, rename = "TitlecaseOptions")]
    pub struct TitlecaseOptionsV1 {
        pub leading_adjustment: DiplomatOption<LeadingAdjustment>,
        pub trailing_case: DiplomatOption<TrailingCase>,
    }

    impl TitlecaseOptionsV1 {
        #[diplomat::rust_link(icu::casemap::options::TitlecaseOptions::default, FnInStruct)]
        #[diplomat::attr(auto, constructor)]
        #[diplomat::attr(any(cpp, js), rename = "default_options")]
        pub fn default() -> TitlecaseOptionsV1 {
            Self {
                leading_adjustment: None.into(),
                trailing_case: None.into(),
            }
        }
    }

    #[diplomat::opaque]
    #[diplomat::rust_link(icu::casemap::CaseMapper, Struct)]
    #[diplomat::rust_link(icu::casemap::CaseMapperBorrowed, Struct, hidden)]
    pub struct CaseMapper(pub icu_casemap::CaseMapper);

    impl CaseMapper {
        /// Construct a new `CaseMapper` instance using compiled data.
        #[diplomat::rust_link(icu::casemap::CaseMapper::new, FnInStruct)]
        #[diplomat::rust_link(icu::casemap::CaseMapperBorrowed::new, FnInStruct, hidden)]
        #[diplomat::attr(auto, constructor)]
        #[cfg(feature = "compiled_data")]
        pub fn create() -> Box<CaseMapper> {
            Box::new(CaseMapper(icu_casemap::CaseMapper::new().static_to_owned()))
        }

        /// Construct a new `CaseMapper` instance using a particular data source.
        #[diplomat::rust_link(icu::casemap::CaseMapper::new, FnInStruct)]
        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "with_provider")]
        #[cfg(feature = "buffer_provider")]
        pub fn create_with_provider(provider: &DataProvider) -> Result<Box<CaseMapper>, DataError> {
            Ok(Box::new(CaseMapper(
                icu_casemap::CaseMapper::try_new_with_buffer_provider(provider.get()?)?,
            )))
        }
        /// Returns the full lowercase mapping of the given string
        #[diplomat::rust_link(icu::casemap::CaseMapperBorrowed::lowercase, FnInStruct)]
        #[diplomat::rust_link(
            icu::casemap::CaseMapperBorrowed::lowercase_to_string,
            FnInStruct,
            hidden
        )]
        #[diplomat::attr(not(supports = utf8_strings), disable)]
        #[diplomat::attr(*, rename = "lowercase")]
        pub fn lowercase(&self, s: &str, locale: &Locale, write: &mut DiplomatWrite) {
            let _infallible = self
                .0
                .as_borrowed()
                .lowercase(s, &locale.0.id)
                .write_to(write);
        }

        /// Returns the full lowercase mapping of the given UTF-16 string
        ///
        /// Unpaired surrogates are passed through unchanged.
        #[diplomat::rust_link(icu::casemap::CaseMapperBorrowed::lowercase_utf16_to, FnInStruct)]
        #[diplomat::attr(not(supports = utf8_strings), rename = "lowercase")]
        #[diplomat::attr(supports = utf8_strings, rename = "lowercase16")]
        #[diplomat::cfg(supports = utf16_writeables)]
        pub fn lowercase_utf16(
            &self,
            s: &DiplomatStr16,
            locale: &Locale,
            write: &mut DiplomatWrite16,
        ) {
            let _infallible = self.0.as_borrowed().lowercase_utf16_to(
                s,
                &locale.0.id,
                &mut super::Write16Adapter(write),
            );
        }

        /// Returns the full uppercase mapping of the given string
        #[diplomat::rust_link(icu::casemap::CaseMapperBorrowed::uppercase, FnInStruct)]
        #[diplomat::rust_link(
            icu::casemap::CaseMapperBorrowed::uppercase_to_string,
            FnInStruct,
            hidden
        )]
        #[diplomat::attr(not(supports = utf8_strings), disable)]
        #[diplomat::attr(*, rename = "uppercase")]
        pub fn uppercase(&self, s: &str, locale: &Locale, write: &mut DiplomatWrite) {
            let _infallible = self
                .0
                .as_borrowed()
                .uppercase(s, &locale.0.id)
                .write_to(write);
        }

        /// Returns the full uppercase mapping of the given UTF-16 string
        ///
        /// Unpaired surrogates are passed through unchanged.
        #[diplomat::rust_link(icu::casemap::CaseMapperBorrowed::uppercase_utf16_to, FnInStruct)]
        #[diplomat::attr(not(supports = utf8_strings), rename = "uppercase")]
        #[diplomat::attr(supports = utf8_strings, rename = "uppercase16")]
        #[diplomat::cfg(supports = utf16_writeables)]
        pub fn uppercase_utf16(
            &self,
            s: &DiplomatStr16,
            locale: &Locale,
            write: &mut DiplomatWrite16,
        ) {
            let _infallible = self.0.as_borrowed().uppercase_utf16_to(
                s,
                &locale.0.id,
                &mut super::Write16Adapter(write),
            );
        }

        /// Returns the full lowercase mapping of the given string, using compiled data (avoids having to allocate a `CaseMapper` object)
        #[diplomat::rust_link(icu::casemap::CaseMapperBorrowed::lowercase, FnInStruct)]
        #[diplomat::rust_link(
            icu::casemap::CaseMapperBorrowed::lowercase_to_string,
            FnInStruct,
            hidden
        )]
        #[cfg(feature = "compiled_data")]
        #[diplomat::attr(any(demo_gen, not(supports = utf8_strings)), disable)] // available through Self::create()
        #[diplomat::attr(*, rename = "lowercase_with_compiled_data")]
        pub fn lowercase_with_compiled_data(s: &str, locale: &Locale, write: &mut DiplomatWrite) {
            let _infallible = icu_casemap::CaseMapper::new()
                .lowercase(s, &locale.0.id)
                .write_to(write);
        }

        /// Returns the full lowercase mapping of the given UTF-16 string, using compiled data (avoids having to allocate a `CaseMapper` object)
        ///
        /// Unpaired surrogates are passed through unchanged.
        #[diplomat::rust_link(icu::casemap::CaseMapperBorrowed::lowercase_utf16_to, FnInStruct)]
        #[cfg(feature = "compiled_data")]
        #[diplomat::attr(demo_gen, disable)] // available through Self::create()
        #[diplomat::attr(not(supports = utf8_strings), rename = "lowercase_with_compiled_data")]
        #[diplomat::attr(supports = utf8_strings, rename = "lowercase_with_compiled_data16")]
        #[diplomat::cfg(supports = utf16_writeables)]
        pub fn lowercase_with_compiled_data_utf16(
            s: &DiplomatStr16,
            locale: &Locale,
            write: &mut DiplomatWrite16,
        ) {
            let _infallible = icu_casemap::CaseMapper::new().lowercase_utf16_to(
                s,
                &locale.0.id,
                &mut super::Write16Adapter(write),
            );
        }

        /// Returns the full uppercase mapping of the given string, using compiled data (avoids having to allocate a `CaseMapper` object)
        #[diplomat::rust_link(icu::casemap::CaseMapperBorrowed::uppercase, FnInStruct)]
        #[diplomat::rust_link(
            icu::casemap::CaseMapperBorrowed::uppercase_to_string,
            FnInStruct,
            hidden
        )]
        #[cfg(feature = "compiled_data")]
        #[diplomat::attr(any(demo_gen, not(supports = utf8_strings)), disable)] // available through Self::create()
        #[diplomat::attr(*, rename = "uppercase_with_compiled_data")]
        pub fn uppercase_with_compiled_data(s: &str, locale: &Locale, write: &mut DiplomatWrite) {
            let _infallible = icu_casemap::CaseMapper::new()
                .uppercase(s, &locale.0.id)
                .write_to(write);
        }

        /// Returns the full uppercase mapping of the given UTF-16 string, using compiled data (avoids having to allocate a `CaseMapper` object)
        ///
        /// Unpaired surrogates are passed through unchanged.
        #[diplomat::rust_link(icu::casemap::CaseMapperBorrowed::uppercase_utf16_to, FnInStruct)]
        #[cfg(feature = "compiled_data")]
        #[diplomat::attr(demo_gen, disable)] // available through Self::create()
        #[diplomat::attr(not(supports = utf8_strings), rename = "uppercase_with_compiled_data")]
        #[diplomat::attr(supports = utf8_strings, rename = "uppercase_with_compiled_data16")]
        #[diplomat::cfg(supports = utf16_writeables)]
        pub fn uppercase_with_compiled_data_utf16(
            s: &DiplomatStr16,
            locale: &Locale,
            write: &mut DiplomatWrite16,
        ) {
            let _infallible = icu_casemap::CaseMapper::new().uppercase_utf16_to(
                s,
                &locale.0.id,
                &mut super::Write16Adapter(write),
            );
        }

        /// Returns the full titlecase mapping of the given string, performing head adjustment without
        /// loading additional data.
        /// (if head adjustment is enabled in the options)
        ///
        /// The `v1` refers to the version of the options struct, which may change as we add more options
        #[diplomat::rust_link(
            icu::casemap::CaseMapperBorrowed::titlecase_segment_with_only_case_data,
            FnInStruct
        )]
        #[diplomat::rust_link(
            icu::casemap::CaseMapperBorrowed::titlecase_segment_with_only_case_data_to_string,
            FnInStruct,
            hidden
        )]
        #[diplomat::attr(supports = non_exhaustive_structs, rename = "titlecase_segment_with_only_case_data")]
        pub fn titlecase_segment_with_only_case_data_v1(
            &self,
            s: &str,
            locale: &Locale,
            options: TitlecaseOptionsV1,
            write: &mut DiplomatWrite,
        ) {
            let _infallible = self
                .0
                .as_borrowed()
                .titlecase_segment_with_only_case_data(s, &locale.0.id, options.into())
                .write_to(write);
        }

        /// Returns the full titlecase mapping of the given string, performing head adjustment without
        /// loading additional data, using compiled data (avoids having to allocate a `CaseMapper` object).
        ///
        /// (if head adjustment is enabled in the options)
        ///
        /// The `v1` refers to the version of the options struct, which may change as we add more options
        #[diplomat::rust_link(
            icu::casemap::CaseMapperBorrowed::titlecase_segment_with_only_case_data_to_string,
            FnInStruct
        )]
        #[diplomat::rust_link(
            icu::casemap::CaseMapperBorrowed::titlecase_segment_with_only_case_data_to_string,
            FnInStruct,
            hidden
        )]
        #[cfg(feature = "compiled_data")]
        #[diplomat::attr(supports = non_exhaustive_structs, rename = "titlecase_segment_with_only_case_compiled_data")]
        #[diplomat::attr(demo_gen, disable)] // available through Self::create()
        pub fn titlecase_segment_with_only_case_compiled_data_v1(
            s: &str,
            locale: &Locale,
            options: TitlecaseOptionsV1,
            write: &mut DiplomatWrite,
        ) {
            let _infallible = icu_casemap::CaseMapper::new()
                .titlecase_segment_with_only_case_data(s, &locale.0.id, options.into())
                .write_to(write);
        }

        /// Case-folds the characters in the given string
        #[diplomat::rust_link(icu::casemap::CaseMapperBorrowed::fold, FnInStruct)]
        #[diplomat::rust_link(icu::casemap::CaseMapperBorrowed::fold_string, FnInStruct, hidden)]
        pub fn fold(&self, s: &str, write: &mut DiplomatWrite) {
            let _infallible = self.0.as_borrowed().fold(s).write_to(write);
        }
        /// Case-folds the characters in the given string
        /// using Turkic (T) mappings for dotted/dotless I.
        #[diplomat::rust_link(icu::casemap::CaseMapperBorrowed::fold_turkic, FnInStruct)]
        #[diplomat::rust_link(
            icu::casemap::CaseMapperBorrowed::fold_turkic_string,
            FnInStruct,
            hidden
        )]
        pub fn fold_turkic(&self, s: &str, write: &mut DiplomatWrite) {
            let _infallible = self.0.as_borrowed().fold_turkic(s).write_to(write);
        }

        /// Adds all simple case mappings and the full case folding for `c` to `builder`.
        /// Also adds special case closure mappings.
        ///
        /// In other words, this adds all characters that this casemaps to, as
        /// well as all characters that may casemap to this one.
        ///
        /// Note that since `CodePointSetBuilder` does not contain strings, this will
        /// ignore string mappings.
        ///
        /// Identical to the similarly named method on `CaseMapCloser`, use that if you
        /// plan on using string case closure mappings too.
        #[cfg(feature = "properties")]
        #[diplomat::rust_link(icu::casemap::CaseMapperBorrowed::add_case_closure_to, FnInStruct)]
        #[diplomat::rust_link(icu::casemap::ClosureSink, Trait, hidden)]
        #[diplomat::rust_link(icu::casemap::ClosureSink::add_char, FnInTrait, hidden)]
        #[diplomat::rust_link(icu::casemap::ClosureSink::add_string, FnInTrait, hidden)]
        pub fn add_case_closure_to(
            &self,
            c: DiplomatChar,
            builder: &mut crate::unstable::collections_sets::ffi::CodePointSetBuilder,
        ) {
            if let Some(ch) = char::from_u32(c) {
                self.0.as_borrowed().add_case_closure_to(ch, &mut builder.0)
            }
        }

        /// Returns the simple lowercase mapping of the given character.
        ///
        /// This function only implements simple and common mappings.
        /// Full mappings, which can map one char to a string, are not included.
        /// For full mappings, use `CaseMapperBorrowed::lowercase`.
        #[diplomat::rust_link(icu::casemap::CaseMapperBorrowed::simple_lowercase, FnInStruct)]
        pub fn simple_lowercase(&self, ch: DiplomatChar) -> DiplomatChar {
            char::from_u32(ch)
                .map(|ch| self.0.as_borrowed().simple_lowercase(ch) as DiplomatChar)
                .unwrap_or(ch)
        }
        /// Returns the simple lowercase mapping of the given character, using compiled data (avoids having to allocate a `CaseMapper` object)
        #[diplomat::rust_link(icu::casemap::CaseMapperBorrowed::simple_lowercase, FnInStruct)]
        #[cfg(feature = "compiled_data")]
        #[diplomat::attr(demo_gen, disable)] // available through Self::create()
        pub fn simple_lowercase_with_compiled_data(ch: DiplomatChar) -> DiplomatChar {
            char::from_u32(ch)
                .map(|ch| icu_casemap::CaseMapper::new().simple_lowercase(ch) as DiplomatChar)
                .unwrap_or(ch)
        }

        /// Returns the simple uppercase mapping of the given character.
        ///
        /// This function only implements simple and common mappings.
        /// Full mappings, which can map one char to a string, are not included.
        /// For full mappings, use `CaseMapperBorrowed::uppercase`.
        #[diplomat::rust_link(icu::casemap::CaseMapperBorrowed::simple_uppercase, FnInStruct)]
        pub fn simple_uppercase(&self, ch: DiplomatChar) -> DiplomatChar {
            char::from_u32(ch)
                .map(|ch| self.0.as_borrowed().simple_uppercase(ch) as DiplomatChar)
                .unwrap_or(ch)
        }

        /// Returns the simple uppercase mapping of the given character, using compiled data (avoids having to allocate a `CaseMapper` object)
        #[diplomat::rust_link(icu::casemap::CaseMapperBorrowed::simple_uppercase, FnInStruct)]
        #[cfg(feature = "compiled_data")]
        #[diplomat::attr(demo_gen, disable)] // available through Self::create()
        pub fn simple_uppercase_with_compiled_data(ch: DiplomatChar) -> DiplomatChar {
            char::from_u32(ch)
                .map(|ch| icu_casemap::CaseMapper::new().simple_uppercase(ch) as DiplomatChar)
                .unwrap_or(ch)
        }

        /// Returns the simple titlecase mapping of the given character.
        ///
        /// This function only implements simple and common mappings.
        /// Full mappings, which can map one char to a string, are not included.
        /// For full mappings, use `CaseMapperBorrowed::titlecase_segment`.
        #[diplomat::rust_link(icu::casemap::CaseMapperBorrowed::simple_titlecase, FnInStruct)]
        pub fn simple_titlecase(&self, ch: DiplomatChar) -> DiplomatChar {
            char::from_u32(ch)
                .map(|ch| self.0.as_borrowed().simple_titlecase(ch) as DiplomatChar)
                .unwrap_or(ch)
        }

        /// Returns the simple titlecase mapping of the given character, using compiled data (avoids having to allocate a `CaseMapper` object)
        #[diplomat::rust_link(icu::casemap::CaseMapperBorrowed::simple_titlecase, FnInStruct)]
        #[cfg(feature = "compiled_data")]
        #[diplomat::attr(demo_gen, disable)] // available through Self::create()
        pub fn simple_titlecase_with_compiled_data(ch: DiplomatChar) -> DiplomatChar {
            char::from_u32(ch)
                .map(|ch| icu_casemap::CaseMapper::new().simple_titlecase(ch) as DiplomatChar)
                .unwrap_or(ch)
        }

        /// Returns the simple casefolding of the given character.
        ///
        /// This function only implements simple folding.
        /// For full folding, use `CaseMapperBorrowed::fold`.
        #[diplomat::rust_link(icu::casemap::CaseMapperBorrowed::simple_fold, FnInStruct)]
        pub fn simple_fold(&self, ch: DiplomatChar) -> DiplomatChar {
            char::from_u32(ch)
                .map(|ch| self.0.as_borrowed().simple_fold(ch) as DiplomatChar)
                .unwrap_or(ch)
        }

        /// Returns the simple casefolding of the given character, using compiled data (avoids having to allocate a `CaseMapper` object)
        #[diplomat::rust_link(icu::casemap::CaseMapperBorrowed::simple_fold, FnInStruct)]
        #[cfg(feature = "compiled_data")]
        #[diplomat::attr(demo_gen, disable)] // available through Self::create()
        pub fn simple_fold_with_compiled_data(ch: DiplomatChar) -> DiplomatChar {
            char::from_u32(ch)
                .map(|ch| icu_casemap::CaseMapper::new().simple_fold(ch) as DiplomatChar)
                .unwrap_or(ch)
        }

        /// Returns the simple casefolding of the given character in the Turkic locale.
        ///
        /// This function only implements simple folding.
        /// For full folding, use `CaseMapperBorrowed::fold_turkic`.
        #[diplomat::rust_link(icu::casemap::CaseMapperBorrowed::simple_fold_turkic, FnInStruct)]
        pub fn simple_fold_turkic(&self, ch: DiplomatChar) -> DiplomatChar {
            char::from_u32(ch)
                .map(|ch| self.0.as_borrowed().simple_fold_turkic(ch) as DiplomatChar)
                .unwrap_or(ch)
        }

        /// Returns the simple Turkic casefolding of the given character, using compiled data (avoids having to allocate a `CaseMapper` object)
        #[diplomat::rust_link(icu::casemap::CaseMapperBorrowed::simple_fold_turkic, FnInStruct)]
        #[cfg(feature = "compiled_data")]
        #[diplomat::attr(demo_gen, disable)] // available through Self::create()
        pub fn simple_fold_turkic_with_compiled_data(ch: DiplomatChar) -> DiplomatChar {
            char::from_u32(ch)
                .map(|ch| icu_casemap::CaseMapper::new().simple_fold_turkic(ch) as DiplomatChar)
                .unwrap_or(ch)
        }
    }

    #[diplomat::opaque]
    #[diplomat::rust_link(icu::casemap::CaseMapCloser, Struct)]
    #[diplomat::rust_link(icu::casemap::CaseMapCloserBorrowed, Struct, hidden)]
    pub struct CaseMapCloser(pub icu_casemap::CaseMapCloser<icu_casemap::CaseMapper>);

    impl CaseMapCloser {
        /// Construct a new `CaseMapCloser` instance using compiled data.
        #[diplomat::rust_link(icu::casemap::CaseMapCloser::new, FnInStruct)]
        #[diplomat::rust_link(icu::casemap::CaseMapCloserBorrowed::new, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::casemap::CaseMapCloser::new_with_mapper, FnInStruct, hidden)]
        #[diplomat::attr(supports = "fallible_constructors", constructor)]
        #[cfg(feature = "compiled_data")]
        pub fn create() -> Result<Box<CaseMapCloser>, DataError> {
            Ok(Box::new(CaseMapCloser(
                icu_casemap::CaseMapCloser::new().static_to_owned(),
            )))
        }
        /// Construct a new `CaseMapCloser` instance using a particular data source.
        #[diplomat::rust_link(icu::casemap::CaseMapCloser::new, FnInStruct)]
        #[diplomat::rust_link(icu::casemap::CaseMapCloser::new_with_mapper, FnInStruct, hidden)]
        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "with_provider")]
        #[cfg(feature = "buffer_provider")]
        pub fn create_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CaseMapCloser>, DataError> {
            Ok(Box::new(CaseMapCloser(
                icu_casemap::CaseMapCloser::try_new_with_buffer_provider(provider.get()?)?,
            )))
        }
        /// Adds all simple case mappings and the full case folding for `c` to `builder`.
        /// Also adds special case closure mappings.
        #[cfg(feature = "properties")]
        #[diplomat::rust_link(icu::casemap::CaseMapCloserBorrowed::add_case_closure_to, FnInStruct)]
        pub fn add_case_closure_to(
            &self,
            c: DiplomatChar,
            builder: &mut crate::unstable::collections_sets::ffi::CodePointSetBuilder,
        ) {
            if let Some(ch) = char::from_u32(c) {
                self.0.as_borrowed().add_case_closure_to(ch, &mut builder.0)
            }
        }

        /// Finds all characters and strings which may casemap to `s` as their full case folding string
        /// and adds them to the set.
        ///
        /// Returns true if the string was found
        #[cfg(feature = "properties")]
        #[diplomat::rust_link(
            icu::casemap::CaseMapCloserBorrowed::add_string_case_closure_to,
            FnInStruct
        )]
        pub fn add_string_case_closure_to(
            &self,
            s: &DiplomatStr,
            builder: &mut crate::unstable::collections_sets::ffi::CodePointSetBuilder,
        ) -> bool {
            let s = str::from_utf8(s).unwrap_or("");
            self.0
                .as_borrowed()
                .add_string_case_closure_to(s, &mut builder.0)
        }
    }

    #[diplomat::opaque]
    #[diplomat::rust_link(icu::casemap::TitlecaseMapper, Struct)]
    #[diplomat::rust_link(icu::casemap::TitlecaseMapperBorrowed, Struct, hidden)]
    pub struct TitlecaseMapper(pub icu_casemap::TitlecaseMapper<icu_casemap::CaseMapper>);

    impl TitlecaseMapper {
        /// Construct a new `TitlecaseMapper` instance using compiled data.
        #[diplomat::rust_link(icu::casemap::TitlecaseMapper::new, FnInStruct)]
        #[diplomat::rust_link(icu::casemap::TitlecaseMapperBorrowed::new, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::casemap::TitlecaseMapper::new_with_mapper, FnInStruct, hidden)]
        #[diplomat::attr(supports = "fallible_constructors", constructor)]
        #[cfg(feature = "compiled_data")]
        pub fn create() -> Result<Box<TitlecaseMapper>, DataError> {
            Ok(Box::new(TitlecaseMapper(
                icu_casemap::TitlecaseMapper::new().static_to_owned(),
            )))
        }
        /// Construct a new `TitlecaseMapper` instance using a particular data source.
        #[diplomat::rust_link(icu::casemap::TitlecaseMapper::new, FnInStruct)]
        #[diplomat::rust_link(icu::casemap::TitlecaseMapper::new_with_mapper, FnInStruct, hidden)]
        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "with_provider")]
        #[cfg(feature = "buffer_provider")]
        pub fn create_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<TitlecaseMapper>, DataError> {
            Ok(Box::new(TitlecaseMapper(
                icu_casemap::TitlecaseMapper::try_new_with_buffer_provider(provider.get()?)?,
            )))
        }
        /// Returns the full titlecase mapping of the given string
        ///
        /// The `v1` refers to the version of the options struct, which may change as we add more options
        #[diplomat::rust_link(icu::casemap::TitlecaseMapperBorrowed::titlecase_segment, FnInStruct)]
        #[diplomat::rust_link(
            icu::casemap::TitlecaseMapperBorrowed::titlecase_segment_to_string,
            FnInStruct,
            hidden
        )]
        #[diplomat::attr(supports = non_exhaustive_structs, rename = "titlecase_segment")]
        pub fn titlecase_segment_v1(
            &self,
            s: &str,
            locale: &Locale,
            options: TitlecaseOptionsV1,
            write: &mut DiplomatWrite,
        ) {
            let _infallible = self
                .0
                .as_borrowed()
                .titlecase_segment(s, &locale.0.id, options.into())
                .write_to(write);
        }
        /// Returns the full titlecase mapping of the given string, using compiled data (avoids having to allocate a `TitlecaseMapper` object)
        ///
        /// The `v1` refers to the version of the options struct, which may change as we add more options
        #[diplomat::rust_link(icu::casemap::TitlecaseMapperBorrowed::titlecase_segment, FnInStruct)]
        #[diplomat::rust_link(
            icu::casemap::TitlecaseMapperBorrowed::titlecase_segment_to_string,
            FnInStruct,
            hidden
        )]
        #[diplomat::attr(supports = non_exhaustive_structs, rename = "titlecase_segment_with_compiled_data")]
        #[cfg(feature = "compiled_data")]
        #[diplomat::attr(demo_gen, disable)] // available through Self::create()
        pub fn titlecase_segment_with_compiled_data_v1(
            s: &str,
            locale: &Locale,
            options: TitlecaseOptionsV1,
            write: &mut DiplomatWrite,
        ) {
            let _infallible = icu_casemap::TitlecaseMapper::new()
                .titlecase_segment(s, &locale.0.id, options.into())
                .write_to(write);
        }
    }

    /// A type that provides access to preconstructred Locales, to avoid
    /// needing to parse user locales when the code knows what locale it wants.
    ///
    /// In most cases, you should be taking a locale from the user, but in some
    /// limited cases you may be parsing from a limited set of locales and not
    /// wish to pull in full-fledged parsing code.
    ///
    /// This type is for locales that have special meaning to `CaseMapper`,
    /// since it only cares about a small set of locales and locale parsing takes
    /// up a relatively large amount of binary size in the context of casemapping.
    #[diplomat::attr(not(cpp), disable)]
    #[diplomat::opaque]
    pub struct CaseMapLocales;

    impl CaseMapLocales {
        /// Returns a borrowed "az" [`Locale`], without allocating.
        pub fn az() -> &'static Locale {
            static AZ: Locale = Locale(locale!("az"));
            &AZ
        }
        /// Returns a borrowed "el" [`Locale`], without allocating.
        pub fn el() -> &'static Locale {
            static EL: Locale = Locale(locale!("el"));
            &EL
        }
        /// Returns a borrowed "hy" [`Locale`], without allocating.
        pub fn hy() -> &'static Locale {
            static HY: Locale = Locale(locale!("hy"));
            &HY
        }
        /// Returns a borrowed "lt" [`Locale`], without allocating.
        pub fn lt() -> &'static Locale {
            static LT: Locale = Locale(locale!("lt"));
            &LT
        }
        /// Returns a borrowed "nl" [`Locale`], without allocating.
        pub fn nl() -> &'static Locale {
            static NL: Locale = Locale(locale!("nl"));
            &NL
        }
        /// Returns a borrowed "tr" [`Locale`], without allocating.
        pub fn tr() -> &'static Locale {
            static TR: Locale = Locale(locale!("tr"));
            &TR
        }
    }
}

impl From<ffi::TitlecaseOptionsV1> for TitlecaseOptions {
    fn from(other: ffi::TitlecaseOptionsV1) -> Self {
        let mut ret = Self::default();

        ret.leading_adjustment = other.leading_adjustment.into_converted_option();

        ret.trailing_case = other.trailing_case.into_converted_option();

        ret
    }
}

#[cfg(all(test, feature = "compiled_data"))]
mod tests {
    use super::ffi::{CaseMapLocales, CaseMapper};
    use crate::unstable::locale_core::ffi::Locale;
    use diplomat_runtime::rust_interop::RustWriteVecGeneric;

    #[test]
    fn test_casemap_utf16() {
        let cm = CaseMapper::create();
        let root = Locale::from_string(b"und").unwrap();
        let tr = CaseMapLocales::tr();
        let el = CaseMapLocales::el();

        // Turkish dotted I (U+0130 -> 'i' in tr, "i\u{307}" in root) and 'I' (-> U+0131 'ı' in tr)
        let input_tr = [0x0130u16, 0x0049];
        let mut out = RustWriteVecGeneric::<u16>::with_capacity(0);
        cm.lowercase_utf16(&input_tr, tr, unsafe { out.borrow_mut() });
        assert_eq!(out.as_slice(), &[0x0069u16, 0x0131]);

        let mut out_root = RustWriteVecGeneric::<u16>::with_capacity(0);
        CaseMapper::lowercase_with_compiled_data_utf16(&input_tr, &root, unsafe {
            out_root.borrow_mut()
        });
        assert_eq!(out_root.as_slice(), &[0x0069u16, 0x0307, 0x0069]);

        // Greek final sigma "ΙΙΙΣ" -> "ιιις" and Greek uppercase accent removal "ά" -> "Α"
        let input_el: Vec<u16> = "ΙΙΙΣ".encode_utf16().collect();
        let expected_el: Vec<u16> = "ιιις".encode_utf16().collect();
        let mut out_el = RustWriteVecGeneric::<u16>::with_capacity(0);
        cm.lowercase_utf16(&input_el, el, unsafe { out_el.borrow_mut() });
        assert_eq!(out_el.as_slice(), expected_el.as_slice());

        // Supplementary cased letter (Deseret U+10400 -> U+10428) + unpaired surrogates (0xD800, 0xDC00)
        // Unpaired surrogates must pass through unchanged.
        let supp_and_unpaired = [0xD801u16, 0xDC00, 0xD800, 0x00DF, 0xDC00];
        let mut out_lower = RustWriteVecGeneric::<u16>::with_capacity(0);
        cm.lowercase_utf16(&supp_and_unpaired, &root, unsafe { out_lower.borrow_mut() });
        assert_eq!(
            out_lower.as_slice(),
            &[0xD801u16, 0xDC28, 0xD800, 0x00DF, 0xDC00]
        );

        let mut out_upper = RustWriteVecGeneric::<u16>::with_capacity(0);
        CaseMapper::uppercase_with_compiled_data_utf16(out_lower.as_slice(), &root, unsafe {
            out_upper.borrow_mut()
        });
        // U+00DF 'ß' uppercases to "SS" (0x0053, 0x0053), Deseret U+10428 -> U+10400, unpaired surrogates preserved
        assert_eq!(
            out_upper.as_slice(),
            &[0xD801u16, 0xDC00, 0xD800, 0x0053, 0x0053, 0xDC00]
        );
    }
}
