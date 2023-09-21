// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{DataError, DataErrorKind};
use core::cmp::Ordering;
use core::default::Default;
use core::fmt;
use core::fmt::Debug;
use core::hash::Hash;
use core::str::FromStr;
use icu_locid::extensions::unicode as unicode_ext;
use icu_locid::subtags::{Language, Region, Script, Variants};
use icu_locid::{LanguageIdentifier, Locale, SubtagOrderingResult};
use writeable::{LengthHint, Writeable};

#[cfg(feature = "experimental")]
use alloc::string::String;
#[cfg(feature = "experimental")]
use core::ops::Deref;
#[cfg(feature = "experimental")]
use tinystr::TinyAsciiStr;

#[cfg(doc)]
use icu_locid::subtags::Variant;

const AUXILIARY_KEY_SEPARATOR: u8 = b'+';

/// The request type passed into all data provider implementations.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
#[allow(clippy::exhaustive_structs)] // this type is stable
pub struct DataRequest<'a> {
    /// The locale for which to load data.
    ///
    /// If locale fallback is enabled, the resulting data may be from a different locale
    /// than the one requested here.
    pub locale: &'a DataLocale,
    /// Metadata that may affect the behavior of the data provider.
    pub metadata: DataRequestMetadata,
}

impl fmt::Display for DataRequest<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.locale, f)
    }
}

/// Metadata for data requests. This is currently empty, but it may be extended with options
/// for tuning locale fallback, buffer layout, and so forth.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub struct DataRequestMetadata {
    /// Silent requests do not log errors. This can be used for exploratory querying, such as fallbacks.
    pub silent: bool,
}

/// A locale type optimized for use in fallbacking and the ICU4X data pipeline.
///
/// [`DataLocale`] contains less functionality than [`Locale`] but more than
/// [`LanguageIdentifier`] for better size and performance while still meeting
/// the needs of the ICU4X data pipeline.
///
/// # Examples
///
/// Convert a [`Locale`] to a [`DataLocale`] and back:
///
/// ```
/// use icu_locid::locale;
/// use icu_provider::DataLocale;
///
/// let locale = locale!("en-u-ca-buddhist");
/// let data_locale = DataLocale::from(locale);
/// let locale = data_locale.into_locale();
///
/// assert_eq!(locale, locale!("en-u-ca-buddhist"));
/// ```
///
/// You can alternatively create a [`DataLocale`] from a borrowed [`Locale`], which is more
/// efficient than cloning the [`Locale`], but less efficient than converting an owned
/// [`Locale`]:
///
/// ```
/// use icu_locid::locale;
/// use icu_provider::DataLocale;
///
/// let locale1 = locale!("en-u-ca-buddhist");
/// let data_locale = DataLocale::from(&locale1);
/// let locale2 = data_locale.into_locale();
///
/// assert_eq!(locale1, locale2);
/// ```
///
/// If you are sure that you have no Unicode keywords, start with [`LanguageIdentifier`]:
///
/// ```
/// use icu_locid::langid;
/// use icu_provider::DataLocale;
///
/// let langid = langid!("es-CA-valencia");
/// let data_locale = DataLocale::from(langid);
/// let langid = data_locale.get_langid();
///
/// assert_eq!(langid, langid!("es-CA-valencia"));
/// ```
///
/// [`DataLocale`] only supports `-u` keywords, to reflect the current state of CLDR data
/// lookup and fallback. This may change in the future.
///
/// ```
/// use icu_locid::{locale, Locale};
/// use icu_provider::DataLocale;
///
/// let locale = "hi-t-en-h0-hybrid-u-attr-ca-buddhist"
///     .parse::<Locale>()
///     .unwrap();
/// let data_locale = DataLocale::from(locale);
///
/// assert_eq!(data_locale.into_locale(), locale!("hi-u-ca-buddhist"));
/// ```
#[derive(PartialEq, Clone, Default, Eq, Hash)]
pub struct DataLocale {
    langid: LanguageIdentifier,
    keywords: unicode_ext::Keywords,
    #[cfg(feature = "experimental")]
    aux: Option<AuxiliaryKeys>,
}

impl<'a> Default for &'a DataLocale {
    fn default() -> Self {
        static DEFAULT: DataLocale = DataLocale {
            langid: LanguageIdentifier::UND,
            keywords: unicode_ext::Keywords::new(),
            #[cfg(feature = "experimental")]
            aux: None,
        };
        &DEFAULT
    }
}

impl fmt::Debug for DataLocale {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DataLocale{{{self}}}")
    }
}

impl Writeable for DataLocale {
    fn write_to<W: core::fmt::Write + ?Sized>(&self, sink: &mut W) -> core::fmt::Result {
        self.langid.write_to(sink)?;
        if !self.keywords.is_empty() {
            sink.write_str("-u-")?;
            self.keywords.write_to(sink)?;
        }
        #[cfg(feature = "experimental")]
        if let Some(aux) = self.aux.as_ref() {
            sink.write_char(AuxiliaryKeys::separator() as char)?;
            aux.write_to(sink)?;
        }
        Ok(())
    }

    fn writeable_length_hint(&self) -> LengthHint {
        let mut length_hint = self.langid.writeable_length_hint();
        if !self.keywords.is_empty() {
            length_hint += self.keywords.writeable_length_hint() + 3;
        }
        #[cfg(feature = "experimental")]
        if let Some(aux) = self.aux.as_ref() {
            length_hint += aux.writeable_length_hint() + 1;
        }
        length_hint
    }

    fn write_to_string(&self) -> alloc::borrow::Cow<str> {
        #[cfg_attr(not(feature = "experimental"), allow(unused_mut))]
        let mut is_only_langid = self.keywords.is_empty();
        #[cfg(feature = "experimental")]
        {
            is_only_langid = is_only_langid && self.aux.is_none();
        }
        if is_only_langid {
            return self.langid.write_to_string();
        }
        let mut string =
            alloc::string::String::with_capacity(self.writeable_length_hint().capacity());
        let _ = self.write_to(&mut string);
        alloc::borrow::Cow::Owned(string)
    }
}

writeable::impl_display_with_writeable!(DataLocale);

impl From<LanguageIdentifier> for DataLocale {
    fn from(langid: LanguageIdentifier) -> Self {
        Self {
            langid,
            keywords: unicode_ext::Keywords::new(),
            #[cfg(feature = "experimental")]
            aux: None,
        }
    }
}

impl From<Locale> for DataLocale {
    fn from(locale: Locale) -> Self {
        Self {
            langid: locale.id,
            keywords: locale.extensions.unicode.keywords,
            #[cfg(feature = "experimental")]
            aux: None,
        }
    }
}

impl From<&LanguageIdentifier> for DataLocale {
    fn from(langid: &LanguageIdentifier) -> Self {
        Self {
            langid: langid.clone(),
            keywords: unicode_ext::Keywords::new(),
            #[cfg(feature = "experimental")]
            aux: None,
        }
    }
}

impl From<&Locale> for DataLocale {
    fn from(locale: &Locale) -> Self {
        Self {
            langid: locale.id.clone(),
            keywords: locale.extensions.unicode.keywords.clone(),
            #[cfg(feature = "experimental")]
            aux: None,
        }
    }
}

impl FromStr for DataLocale {
    type Err = DataError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut aux_iter = s.splitn(2, AUXILIARY_KEY_SEPARATOR as char);
        let Some(locale_str) = aux_iter.next() else {
            return Err(DataErrorKind::KeyLocaleSyntax
                .into_error()
                .with_display_context(s));
        };
        let locale = Locale::from_str(locale_str).map_err(|e| {
            DataErrorKind::KeyLocaleSyntax
                .into_error()
                .with_display_context(s)
                .with_display_context(&e)
        })?;
        #[cfg_attr(not(feature = "experimental"), allow(unused_mut))]
        let mut data_locale = DataLocale::from(locale);
        #[cfg(feature = "experimental")]
        if let Some(aux_str) = aux_iter.next() {
            let aux = AuxiliaryKeys::from_str(aux_str)?;
            data_locale.set_aux(aux);
        }
        if aux_iter.next().is_some() {
            return Err(DataErrorKind::KeyLocaleSyntax
                .into_error()
                .with_display_context(s));
        }
        Ok(data_locale)
    }
}

impl DataLocale {
    /// Compare this [`DataLocale`] with BCP-47 bytes.
    ///
    /// The return value is equivalent to what would happen if you first converted this
    /// [`DataLocale`] to a BCP-47 string and then performed a byte comparison.
    ///
    /// This function is case-sensitive and results in a *total order*, so it is appropriate for
    /// binary search. The only argument producing [`Ordering::Equal`] is `self.to_string()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locid::Locale;
    /// use icu_provider::DataLocale;
    /// use std::cmp::Ordering;
    ///
    /// let bcp47_strings: &[&str] = &[
    ///     "ca",
    ///     "ca+EUR",
    ///     "ca-ES",
    ///     "ca-ES+GBP",
    ///     "ca-ES+GBP+short",
    ///     "ca-ES+USD",
    ///     "ca-ES-u-ca-buddhist",
    ///     "ca-ES-valencia",
    ///     "cat",
    ///     "pl-Latn-PL",
    ///     "und",
    ///     "und+MXN",
    ///     "und-fonipa",
    ///     "und-u-ca-hebrew",
    ///     "und-u-ca-japanese",
    ///     "zh",
    /// ];
    ///
    /// for ab in bcp47_strings.windows(2) {
    ///     let a = ab[0];
    ///     let b = ab[1];
    ///     assert_eq!(a.cmp(b), Ordering::Less, "strings: {} < {}", a, b);
    ///     let a_loc: DataLocale = a.parse().unwrap();
    ///     assert_eq!(
    ///         a_loc.strict_cmp(a.as_bytes()),
    ///         Ordering::Equal,
    ///         "strict_cmp: {} == {}",
    ///         a_loc,
    ///         a
    ///     );
    ///     assert_eq!(
    ///         a_loc.strict_cmp(b.as_bytes()),
    ///         Ordering::Less,
    ///         "strict_cmp: {} < {}",
    ///         a_loc,
    ///         b
    ///     );
    ///     let b_loc: DataLocale = b.parse().unwrap();
    ///     assert_eq!(
    ///         b_loc.strict_cmp(b.as_bytes()),
    ///         Ordering::Equal,
    ///         "strict_cmp: {} == {}",
    ///         b_loc,
    ///         b
    ///     );
    ///     assert_eq!(
    ///         b_loc.strict_cmp(a.as_bytes()),
    ///         Ordering::Greater,
    ///         "strict_cmp: {} > {}",
    ///         b_loc,
    ///         a
    ///     );
    /// }
    /// ```
    ///
    /// Comparison against invalid strings:
    ///
    /// ```
    /// use icu_provider::DataLocale;
    ///
    /// let invalid_strings: &[&str] = &[
    ///     // Less than "ca-ES"
    ///     "CA",
    ///     "ar+GBP+FOO",
    ///     // Greater than "ca-ES+GBP"
    ///     "ca_ES",
    ///     "ca-ES+GBP+FOO",
    /// ];
    ///
    /// let data_locale = "ca-ES+GBP".parse::<DataLocale>().unwrap();
    ///
    /// for s in invalid_strings.iter() {
    ///     let expected_ordering = "ca-ES+GBP".cmp(s);
    ///     let actual_ordering = data_locale.strict_cmp(s.as_bytes());
    ///     assert_eq!(expected_ordering, actual_ordering, "{}", s);
    /// }
    /// ```
    pub fn strict_cmp(&self, other: &[u8]) -> Ordering {
        let mut aux_iter = other.splitn(2, |b| *b == AUXILIARY_KEY_SEPARATOR);
        let Some(locale_str) = aux_iter.next() else {
            debug_assert!(other.is_empty());
            return Ordering::Greater;
        };
        let aux_str = aux_iter.next();
        let subtags = locale_str.split(|b| *b == b'-');
        let mut subtag_result = self.langid.strict_cmp_iter(subtags);
        if self.has_unicode_ext() {
            let mut subtags = match subtag_result {
                SubtagOrderingResult::Subtags(s) => s,
                SubtagOrderingResult::Ordering(o) => return o,
            };
            match subtags.next() {
                Some(b"u") => (),
                Some(s) => return s.cmp(b"u").reverse(),
                None => return Ordering::Greater,
            }
            subtag_result = self.keywords.strict_cmp_iter(subtags);
        }
        let has_more_subtags = match subtag_result {
            SubtagOrderingResult::Subtags(mut s) => s.next().is_some(),
            SubtagOrderingResult::Ordering(o) => return o,
        };
        // If we get here, `self` has equal or fewer subtags than the `other`.
        // There are 2^3 = 8 cases to handle for auxiliary keys, expanded below.
        match (has_more_subtags, self.get_aux(), aux_str) {
            (false, None, None) => {
                // foo == foo
                Ordering::Equal
            }
            (false, Some(self_aux), Some(other_aux)) => {
                // foo+BAR1 ?= foo+BAR2
                let aux_ordering = self_aux.as_bytes().cmp(other_aux);
                if aux_ordering != Ordering::Equal {
                    return aux_ordering;
                }
                Ordering::Equal
            }
            (false, Some(_), None) => {
                // foo+BAR > foo
                Ordering::Greater
            }
            (_, _, _) => {
                // foo < foo-bar
                // foo < foo-bar+BAR
                // foo < foo+BAR
                // foo+BAR < foo-bar
                // foo+BAR < foo-bar+BAR
                Ordering::Less
            }
        }
    }
}

impl DataLocale {
    /// Returns whether this [`DataLocale`] has all empty fields (no components).
    ///
    /// See also:
    ///
    /// - [`DataLocale::is_und()`]
    /// - [`DataLocale::is_langid_und()`]
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_provider::DataLocale;
    ///
    /// assert!("und".parse::<DataLocale>().unwrap().is_empty());
    /// assert!(!"und-u-ca-buddhist"
    ///     .parse::<DataLocale>()
    ///     .unwrap()
    ///     .is_empty());
    /// assert!(!"und+auxiliary".parse::<DataLocale>().unwrap().is_empty());
    /// assert!(!"ca-ES".parse::<DataLocale>().unwrap().is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self == <&DataLocale>::default()
    }

    /// Returns whether this [`DataLocale`] is `und` in the locale and extensions portion.
    ///
    /// This ignores auxiliary keys.
    ///
    /// See also:
    ///
    /// - [`DataLocale::is_empty()`]
    /// - [`DataLocale::is_langid_und()`]
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_provider::DataLocale;
    ///
    /// assert!("und".parse::<DataLocale>().unwrap().is_und());
    /// assert!(!"und-u-ca-buddhist".parse::<DataLocale>().unwrap().is_und());
    /// assert!("und+auxiliary".parse::<DataLocale>().unwrap().is_und());
    /// assert!(!"ca-ES".parse::<DataLocale>().unwrap().is_und());
    /// ```
    pub fn is_und(&self) -> bool {
        self.langid == LanguageIdentifier::UND && self.keywords.is_empty()
    }

    /// Returns whether the [`LanguageIdentifier`] associated with this request is `und`.
    ///
    /// This ignores extension keywords and auxiliary keys.
    ///
    /// See also:
    ///
    /// - [`DataLocale::is_empty()`]
    /// - [`DataLocale::is_und()`]
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_provider::DataLocale;
    ///
    /// assert!("und".parse::<DataLocale>().unwrap().is_langid_und());
    /// assert!("und-u-ca-buddhist"
    ///     .parse::<DataLocale>()
    ///     .unwrap()
    ///     .is_langid_und());
    /// assert!("und+auxiliary"
    ///     .parse::<DataLocale>()
    ///     .unwrap()
    ///     .is_langid_und());
    /// assert!(!"ca-ES".parse::<DataLocale>().unwrap().is_langid_und());
    /// ```
    pub fn is_langid_und(&self) -> bool {
        self.langid == LanguageIdentifier::UND
    }

    /// Gets the [`LanguageIdentifier`] for this [`DataLocale`].
    ///
    /// This may allocate memory if there are variant subtags. If you need only the language,
    /// script, and/or region subtag, use the specific getters for those subtags:
    ///
    /// - [`DataLocale::language()`]
    /// - [`DataLocale::script()`]
    /// - [`DataLocale::region()`]
    ///
    /// If you have ownership over the `DataLocale`, use [`DataLocale::into_locale()`]
    /// and then access the `id` field.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locid::langid;
    /// use icu_provider::prelude::*;
    ///
    /// const FOO_BAR: DataKey = icu_provider::data_key!("foo/bar@1");
    ///
    /// let req_no_langid = DataRequest {
    ///     locale: &Default::default(),
    ///     metadata: Default::default(),
    /// };
    ///
    /// let req_with_langid = DataRequest {
    ///     locale: &langid!("ar-EG").into(),
    ///     metadata: Default::default(),
    /// };
    ///
    /// assert_eq!(req_no_langid.locale.get_langid(), langid!("und"));
    /// assert_eq!(req_with_langid.locale.get_langid(), langid!("ar-EG"));
    /// ```
    pub fn get_langid(&self) -> LanguageIdentifier {
        self.langid.clone()
    }

    /// Overrides the entire [`LanguageIdentifier`] portion of this [`DataLocale`].
    #[inline]
    pub fn set_langid(&mut self, lid: LanguageIdentifier) {
        self.langid = lid;
    }

    /// Converts this [`DataLocale`] into a [`Locale`].
    ///
    /// See also [`DataLocale::get_langid()`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locid::{
    ///     langid, locale,
    ///     subtags::{language, region},
    ///     Locale,
    /// };
    /// use icu_provider::prelude::*;
    ///
    /// let locale: DataLocale = locale!("it-IT-u-ca-coptic").into();
    ///
    /// assert_eq!(locale.get_langid(), langid!("it-IT"));
    /// assert_eq!(locale.language(), language!("it"));
    /// assert_eq!(locale.script(), None);
    /// assert_eq!(locale.region(), Some(region!("IT")));
    ///
    /// let locale = locale.into_locale();
    /// assert_eq!(locale, locale!("it-IT-u-ca-coptic"));
    /// ```
    pub fn into_locale(self) -> Locale {
        let mut loc = Locale {
            id: self.langid,
            ..Default::default()
        };
        loc.extensions.unicode.keywords = self.keywords;
        loc
    }

    /// Returns the [`Language`] for this [`DataLocale`].
    #[inline]
    pub fn language(&self) -> Language {
        self.langid.language
    }

    /// Returns the [`Language`] for this [`DataLocale`].
    #[inline]
    pub fn set_language(&mut self, language: Language) {
        self.langid.language = language;
    }

    /// Returns the [`Script`] for this [`DataLocale`].
    #[inline]
    pub fn script(&self) -> Option<Script> {
        self.langid.script
    }

    /// Sets the [`Script`] for this [`DataLocale`].
    #[inline]
    pub fn set_script(&mut self, script: Option<Script>) {
        self.langid.script = script;
    }

    /// Returns the [`Region`] for this [`DataLocale`].
    #[inline]
    pub fn region(&self) -> Option<Region> {
        self.langid.region
    }

    /// Sets the [`Region`] for this [`DataLocale`].
    #[inline]
    pub fn set_region(&mut self, region: Option<Region>) {
        self.langid.region = region;
    }

    /// Returns whether there are any [`Variant`] subtags in this [`DataLocale`].
    #[inline]
    pub fn has_variants(&self) -> bool {
        !self.langid.variants.is_empty()
    }

    /// Sets all [`Variants`] on this [`DataLocale`], overwriting any that were there previously.
    #[inline]
    pub fn set_variants(&mut self, variants: Variants) {
        self.langid.variants = variants;
    }

    /// Removes all [`Variant`] subtags in this [`DataLocale`].
    #[inline]
    pub fn clear_variants(&mut self) -> Variants {
        self.langid.variants.clear()
    }

    /// Gets the value of the specified Unicode extension keyword for this [`DataLocale`].
    #[inline]
    pub fn get_unicode_ext(&self, key: &unicode_ext::Key) -> Option<unicode_ext::Value> {
        self.keywords.get(key).cloned()
    }

    /// Returns whether there are any Unicode extension keywords in this [`DataLocale`].
    #[inline]
    pub fn has_unicode_ext(&self) -> bool {
        !self.keywords.is_empty()
    }

    /// Returns whether a specific Unicode extension keyword is present in this [`DataLocale`].
    #[inline]
    pub fn contains_unicode_ext(&self, key: &unicode_ext::Key) -> bool {
        self.keywords.contains_key(key)
    }

    /// Returns whether this [`DataLocale`] contains a Unicode extension keyword
    /// with the specified key and value.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locid::{
    ///     extensions::unicode::{key, value},
    ///     Locale,
    /// };
    /// use icu_provider::prelude::*;
    ///
    /// let locale: Locale = "it-IT-u-ca-coptic".parse().expect("Valid BCP-47");
    /// let locale: DataLocale = locale.into();
    ///
    /// assert_eq!(locale.get_unicode_ext(&key!("hc")), None);
    /// assert_eq!(locale.get_unicode_ext(&key!("ca")), Some(value!("coptic")));
    /// assert!(locale.matches_unicode_ext(&key!("ca"), &value!("coptic"),));
    /// ```
    #[inline]
    pub fn matches_unicode_ext(&self, key: &unicode_ext::Key, value: &unicode_ext::Value) -> bool {
        self.keywords.get(key) == Some(value)
    }

    /// Sets the value for a specific Unicode extension keyword on this [`DataLocale`].
    #[inline]
    pub fn set_unicode_ext(
        &mut self,
        key: unicode_ext::Key,
        value: unicode_ext::Value,
    ) -> Option<unicode_ext::Value> {
        self.keywords.set(key, value)
    }

    /// Removes a specific Unicode extension keyword from this [`DataLocale`], returning
    /// the value if it was present.
    #[inline]
    pub fn remove_unicode_ext(&mut self, key: &unicode_ext::Key) -> Option<unicode_ext::Value> {
        self.keywords.remove(key)
    }

    /// Retains a subset of keywords as specified by the predicate function.
    #[inline]
    pub fn retain_unicode_ext<F>(&mut self, predicate: F)
    where
        F: FnMut(&unicode_ext::Key) -> bool,
    {
        self.keywords.retain_by_key(predicate)
    }

    /// Gets the auxiliary key for this [`DataLocale`].
    ///
    /// For more information and examples, see [`AuxiliaryKeys`].
    #[cfg(feature = "experimental")]
    pub fn get_aux(&self) -> Option<&AuxiliaryKeys> {
        self.aux.as_ref()
    }

    #[cfg(not(feature = "experimental"))]
    pub(crate) fn get_aux(&self) -> Option<&str> {
        None
    }

    /// Returns whether this [`DataLocale`] has an auxiliary key.
    ///
    /// For more information and examples, see [`AuxiliaryKeys`].
    #[cfg(feature = "experimental")]
    pub fn has_aux(&self) -> bool {
        self.aux.is_some()
    }

    /// Sets an auxiliary key on this [`DataLocale`].
    ///
    /// Returns the previous auxiliary key if present.
    ///
    /// For more information and examples, see [`AuxiliaryKeys`].
    #[cfg(feature = "experimental")]
    pub fn set_aux(&mut self, value: AuxiliaryKeys) -> Option<AuxiliaryKeys> {
        self.aux.replace(value)
    }

    /// Remove an auxiliary key, if present. Returns the removed auxiliary key.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locid::locale;
    /// use icu_provider::prelude::*;
    /// use writeable::assert_writeable_eq;
    ///
    /// let mut data_locale: DataLocale = locale!("ar-EG").into();
    /// let aux = "GBP"
    ///     .parse::<AuxiliaryKeys>()
    ///     .expect("contains valid characters");
    /// data_locale.set_aux(aux);
    /// assert_writeable_eq!(data_locale, "ar-EG+GBP");
    ///
    /// let maybe_aux = data_locale.remove_aux();
    /// assert_writeable_eq!(data_locale, "ar-EG");
    /// assert_writeable_eq!(maybe_aux.unwrap(), "GBP");
    /// ```
    #[cfg(feature = "experimental")]
    pub fn remove_aux(&mut self) -> Option<AuxiliaryKeys> {
        self.aux.take()
    }
}

/// The "auxiliary key" is an annotation on [`DataLocale`] that can contain an arbitrary
/// information that does not fit into the [`LanguageIdentifier`] or [`Keywords`].
///
/// A [`DataLocale`] can have multiple auxiliary keys, represented by this struct. The auxiliary
/// keys are separated from the BCP-47 locale and from each other with the character returned by
/// [`AuxiliaryKeys::separator()`].
///
/// An auxiliary key currently allows alphanumerics and `-`.
///
/// <div class="stab unstable">
/// 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. It can be enabled with the "experimental" Cargo feature
/// of the `icu_provider` crate. Use with caution.
/// <a href="https://github.com/unicode-org/icu4x/issues/3632">#3632</a>
/// </div>
///
/// # Examples
///
/// ```
/// use icu_locid::locale;
/// use icu_provider::prelude::*;
/// use writeable::assert_writeable_eq;
///
/// let mut data_locale: DataLocale = locale!("ar-EG").into();
/// assert_writeable_eq!(data_locale, "ar-EG");
/// assert!(!data_locale.has_aux());
/// assert_eq!(data_locale.get_aux(), None);
///
/// let aux = "GBP"
///     .parse::<AuxiliaryKeys>()
///     .expect("contains valid characters");
///
/// data_locale.set_aux(aux);
/// assert_writeable_eq!(data_locale, "ar-EG+GBP");
/// assert!(data_locale.has_aux());
/// assert_eq!(data_locale.get_aux(), Some(&"GBP".parse().unwrap()));
/// ```
///
/// Multiple auxiliary keys are allowed:
///
/// ```
/// use icu_locid::locale;
/// use icu_provider::prelude::*;
/// use writeable::assert_writeable_eq;
///
/// let data_locale = "ar-EG+GBP+long".parse::<DataLocale>().unwrap();
/// assert_writeable_eq!(data_locale, "ar-EG+GBP+long");
/// assert_eq!(data_locale.get_aux().unwrap().iter().count(), 2);
/// ```
///
/// Not all strings are valid auxiliary keys:
///
/// ```
/// use icu_provider::prelude::*;
///
/// assert!("abcdefg".parse::<AuxiliaryKeys>().is_ok());
/// assert!("ABC123".parse::<AuxiliaryKeys>().is_ok());
/// assert!("abc-xyz".parse::<AuxiliaryKeys>().is_ok());
///
/// assert!("".parse::<AuxiliaryKeys>().is_err());
/// assert!("!@#$%".parse::<AuxiliaryKeys>().is_err());
/// assert!("abc_xyz".parse::<AuxiliaryKeys>().is_err());
/// ```
///
/// [`Keywords`]: unicode_ext::Keywords
#[derive(Debug, PartialEq, Clone, Eq, Hash)]
#[cfg(feature = "experimental")]
pub struct AuxiliaryKeys {
    // DISCUSS: SmallStr? TinyStrAuto?
    // DISCUSS: Make this a dynamically sized type so references can be taken?
    value: AuxiliaryKeysInner,
}

#[cfg(feature = "experimental")]
#[derive(Clone)]
enum AuxiliaryKeysInner {
    Boxed(alloc::boxed::Box<str>),
    Stack(TinyAsciiStr<23>),
    // NOTE: In the future, a `Static` variant could be added to allow `data_locale!("...")`
    // Static(&'static str),
}

#[cfg(feature = "experimental")]
impl Deref for AuxiliaryKeysInner {
    type Target = str;
    #[inline]
    fn deref(&self) -> &Self::Target {
        match self {
            Self::Boxed(s) => s.deref(),
            Self::Stack(s) => s.as_str(),
        }
    }
}

#[cfg(feature = "experimental")]
impl PartialEq for AuxiliaryKeysInner {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.deref() == other.deref()
    }
}

#[cfg(feature = "experimental")]
impl Eq for AuxiliaryKeysInner {}

#[cfg(feature = "experimental")]
impl Debug for AuxiliaryKeysInner {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.deref().fmt(f)
    }
}

#[cfg(feature = "experimental")]
impl Hash for AuxiliaryKeysInner {
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.deref().hash(state)
    }
}

#[cfg(feature = "experimental")]
writeable::impl_display_with_writeable!(AuxiliaryKeys);

#[cfg(feature = "experimental")]
impl Writeable for AuxiliaryKeys {
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        self.value.write_to(sink)
    }
    fn writeable_length_hint(&self) -> LengthHint {
        self.value.writeable_length_hint()
    }
    fn write_to_string(&self) -> alloc::borrow::Cow<str> {
        self.value.write_to_string()
    }
}

#[cfg(feature = "experimental")]
impl FromStr for AuxiliaryKeys {
    type Err = DataError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from_str(s)
    }
}

#[cfg(feature = "experimental")]
impl AuxiliaryKeys {
    /// Returns this [`AuxiliaryKeys`] as a single byte slice.
    ///
    /// NOTE: Do not make this public because we might not always store these in a single string.
    /// External clients who need this can use `<Self as Writeable>::write_to_string`.
    #[inline]
    pub(crate) fn as_bytes(&self) -> &[u8] {
        self.value.as_bytes()
    }

    /// Creates an [`AuxiliaryKeys`] from an iterator of individual keys.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_provider::prelude::*;
    ///
    /// // Single auxiliary key:
    /// let a = AuxiliaryKeys::try_from_iter(["abc"]).unwrap();
    /// let b = "abc".parse::<AuxiliaryKeys>().unwrap();
    /// assert_eq!(a, b);
    ///
    /// // Multiple auxiliary keys:
    /// let a = AuxiliaryKeys::try_from_iter(["abc", "defg"]).unwrap();
    /// let b = "abc+defg".parse::<AuxiliaryKeys>().unwrap();
    /// assert_eq!(a, b);
    /// ```
    ///
    /// Don't include the auxiliary key separator or other invalid chars in the iterator strings:
    ///
    /// ```
    /// use icu_provider::prelude::*;
    ///
    /// assert!(AuxiliaryKeys::try_from_iter(["abc+defg"]).is_err());
    /// assert!(AuxiliaryKeys::try_from_iter(["AB$C"]).is_err());
    /// ```
    pub fn try_from_iter<'a>(iter: impl IntoIterator<Item = &'a str>) -> Result<Self, DataError> {
        // TODO: Avoid the allocation when possible
        let mut builder = String::new();
        for item in iter {
            if !item.is_empty()
                && item
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-'))
            {
                if !builder.is_empty() {
                    builder.push(AuxiliaryKeys::separator() as char);
                }
                builder.push_str(item)
            } else {
                return Err(DataErrorKind::KeyLocaleSyntax
                    .into_error()
                    .with_display_context(item));
            }
        }
        if builder.len() <= 23 {
            #[allow(clippy::unwrap_used)] // we just checked that the string is ascii
            Ok(Self {
                value: AuxiliaryKeysInner::Stack(builder.parse().unwrap()),
            })
        } else {
            Ok(Self {
                value: AuxiliaryKeysInner::Boxed(builder.into()),
            })
        }
    }

    pub(crate) fn try_from_str(s: &str) -> Result<Self, DataError> {
        if !s.is_empty()
            && s.bytes()
                .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'+'))
        {
            if s.len() <= 23 {
                #[allow(clippy::unwrap_used)] // we just checked that the string is ascii
                Ok(Self {
                    value: AuxiliaryKeysInner::Stack(s.parse().unwrap()),
                })
            } else {
                Ok(Self {
                    value: AuxiliaryKeysInner::Boxed(s.into()),
                })
            }
        } else {
            Err(DataErrorKind::KeyLocaleSyntax
                .into_error()
                .with_display_context(s))
        }
    }

    /// Iterates over the components of the auxiliary key.
    ///
    /// # Example
    ///
    /// ```
    /// use icu_provider::AuxiliaryKeys;
    ///
    /// let aux: AuxiliaryKeys = "abc+defg".parse().unwrap();
    /// assert_eq!(aux.iter().collect::<Vec<_>>(), vec!["abc", "defg"]);
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = &str> + '_ {
        self.value.split(Self::separator() as char)
    }

    /// Returns the separator byte used for auxiliary keys in data locales.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_provider::AuxiliaryKeys;
    ///
    /// assert_eq!(AuxiliaryKeys::separator(), b'+');
    /// ```
    #[inline]
    pub const fn separator() -> u8 {
        AUXILIARY_KEY_SEPARATOR
    }
}

#[test]
fn test_data_locale_to_string() {
    use icu_locid::locale;

    struct TestCase {
        pub locale: Locale,
        pub aux: Option<&'static str>,
        pub expected: &'static str,
    }

    for cas in [
        TestCase {
            locale: Locale::UND,
            aux: None,
            expected: "und",
        },
        TestCase {
            locale: locale!("und-u-cu-gbp"),
            aux: None,
            expected: "und-u-cu-gbp",
        },
        TestCase {
            locale: locale!("en-ZA-u-cu-gbp"),
            aux: None,
            expected: "en-ZA-u-cu-gbp",
        },
        #[cfg(feature = "experimental")]
        TestCase {
            locale: locale!("en-ZA-u-nu-arab"),
            aux: Some("GBP"),
            expected: "en-ZA-u-nu-arab+GBP",
        },
    ] {
        let mut data_locale = DataLocale::from(cas.locale);
        #[cfg(feature = "experimental")]
        if let Some(aux) = cas.aux {
            data_locale.set_aux(aux.parse().unwrap());
        }
        writeable::assert_writeable_eq!(data_locale, cas.expected);
    }
}

#[test]
fn test_data_locale_from_string() {
    #[derive(Debug)]
    struct TestCase {
        pub input: &'static str,
        pub success: bool,
    }

    for cas in [
        TestCase {
            input: "und",
            success: true,
        },
        TestCase {
            input: "und-u-cu-gbp",
            success: true,
        },
        TestCase {
            input: "en-ZA-u-cu-gbp",
            success: true,
        },
        TestCase {
            input: "en...",
            success: false,
        },
        #[cfg(feature = "experimental")]
        TestCase {
            input: "en-ZA-u-nu-arab+GBP",
            success: true,
        },
        #[cfg(not(feature = "experimental"))]
        TestCase {
            input: "en-ZA-u-nu-arab+GBP",
            success: false,
        },
    ] {
        let data_locale = match (DataLocale::from_str(cas.input), cas.success) {
            (Ok(l), true) => l,
            (Err(_), false) => {
                continue;
            }
            (Ok(_), false) => {
                panic!("DataLocale parsed but it was supposed to fail: {cas:?}");
            }
            (Err(_), true) => {
                panic!("DataLocale was supposed to parse but it failed: {cas:?}");
            }
        };
        writeable::assert_writeable_eq!(data_locale, cas.input);
    }
}
