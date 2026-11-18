// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! ECMA-402 `Intl.ListFormat` glue types, locale negotiation, and `resolvedOptions()`.
//!
//! <div class="stab unstable">
//! 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
//! including in SemVer minor releases. Do not use this module unless you are prepared for things to occasionally break.
//!
//! Graduation tracking issue: [issue #5868](https://github.com/unicode-org/icu4x/issues/5868).
//! </div>
//!
//! ✨ *Enabled with the `unstable` Cargo feature.*
//!
//! This module provides [`ECMAListFormatter`], an ECMA-402 shim around [`ListFormatter`]
//! implementing the locale negotiation, options resolution, `supportedLocalesOf`, and
//! `resolvedOptions()` semantics of [`Intl.ListFormat`][spec].
//!
//! [`ECMAListFormatter`] implements [`Deref<Target = ListFormatter>`](core::ops::Deref) so callers
//! can invoke [`ListFormatter::format`] and [`ListFormatter::format_to_string`] directly without
//! duplicating formatting methods.
//!
//! # Data Generation Requirement (`RetainBaseLanguages`)
//!
//! Accurate `resolvedOptions().locale` and `supportedLocalesOf()` results require generating data
//! with `DeduplicationStrategy::RetainBaseLanguages` (or `DeduplicationStrategy::None`). Under
//! `DeduplicationStrategy::Maximal`, base languages whose list patterns match `und` (for example,
//! English `"en"` wide/short unit lists `"{0}, {1}"`) are stripped from the data trie and fall back
//! to `und`, causing locale negotiation to treat them as unsupported.
//!
//! Regional locales such as `"es-AR"` or `"zh-Hant-TW"` may still be deduplicated into `"es"` or
//! `"zh-Hant"`, which is compliant with ECMA-402.
//!
//! [spec]: https://tc39.es/ecma402/#listformat-objects

#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use core::cell::Cell;
use core::ops::Deref;
use icu_locale_core::{LanguageIdentifier, Locale};
use icu_provider::prelude::*;

use crate::options::{ListFormatterOptions, ListLength};
#[cfg(any(feature = "alloc", test))]
use crate::provider::ListFormatterPatterns;
use crate::provider::{ListAndV1, ListOrV1, ListUnitV1};
use crate::{ListFormatter, ListFormatterPreferences};

/// The ECMA-402 `type` option for `Intl.ListFormat`.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. Do not use this type unless you are prepared for things to occasionally break.
///
/// Graduation tracking issue: [issue #5868](https://github.com/unicode-org/icu4x/issues/5868).
/// </div>
///
/// ✨ *Enabled with the `unstable` Cargo feature.*
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
#[non_exhaustive]
pub enum ListType {
    /// `"conjunction"` (default): "and"-based lists (e.g., `"A, B, and C"`), backed by [`ListAndV1`].
    #[default]
    Conjunction,
    /// `"disjunction"`: "or"-based lists (e.g., `"A, B, or C"`), backed by [`ListOrV1`].
    Disjunction,
    /// `"unit"`: unit lists (e.g., `"5 pounds, 12 ounces"`), backed by [`ListUnitV1`].
    Unit,
}

impl ListType {
    /// Returns the ECMA-402 string representation of this option
    /// (`"conjunction"`, `"disjunction"`, or `"unit"`).
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Conjunction => "conjunction",
            Self::Disjunction => "disjunction",
            Self::Unit => "unit",
        }
    }
}

/// The ECMA-402 `style` option for `Intl.ListFormat`.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. Do not use this type unless you are prepared for things to occasionally break.
///
/// Graduation tracking issue: [issue #5868](https://github.com/unicode-org/icu4x/issues/5868).
/// </div>
///
/// ✨ *Enabled with the `unstable` Cargo feature.*
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
#[non_exhaustive]
pub enum ListStyle {
    /// `"long"` (default): typical full list formatting, corresponding to [`ListLength::Wide`].
    #[default]
    Long,
    /// `"short"`: abbreviated list formatting, corresponding to [`ListLength::Short`].
    Short,
    /// `"narrow"`: compact list formatting, corresponding to [`ListLength::Narrow`].
    Narrow,
}

impl ListStyle {
    /// Converts this ECMA-402 [`ListStyle`] into the corresponding ICU4X [`ListLength`].
    pub const fn to_list_length(self) -> ListLength {
        match self {
            Self::Long => ListLength::Wide,
            Self::Short => ListLength::Short,
            Self::Narrow => ListLength::Narrow,
        }
    }

    /// Returns the ECMA-402 string representation of this option (`"long"`, `"short"`, or `"narrow"`).
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Long => "long",
            Self::Short => "short",
            Self::Narrow => "narrow",
        }
    }
}

impl From<ListStyle> for ListLength {
    fn from(style: ListStyle) -> Self {
        style.to_list_length()
    }
}

/// Options bag for constructing an [`ECMAListFormatter`], corresponding to the `options`
/// parameter of ECMA-402 `new Intl.ListFormat(locales, options)`.
///
/// Note: ECMA-402's `localeMatcher` option (`"lookup"` vs `"best fit"`) is not currently exposed
/// here; callers implementing JS bindings should still read and validate `options.localeMatcher`
/// on the JS options object per ECMA-402 `ResolveOptions`.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. Do not use this type unless you are prepared for things to occasionally break.
///
/// Graduation tracking issue: [issue #5868](https://github.com/unicode-org/icu4x/issues/5868).
/// </div>
///
/// ✨ *Enabled with the `unstable` Cargo feature.*
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
#[non_exhaustive]
pub struct ECMAListFormatterOptions {
    // TODO(#5868): Consider adding a shared `LocaleMatcher` option once designed across components.
    /// The list formatting type (`type` in ECMA-402).
    ///
    /// Defaults to [`ListType::Conjunction`].
    pub list_type: Option<ListType>,
    /// The list formatting width style (`style` in ECMA-402).
    ///
    /// Defaults to [`ListStyle::Long`].
    pub style: Option<ListStyle>,
}

impl ECMAListFormatterOptions {
    /// Constructs a new [`ECMAListFormatterOptions`] with all fields unset (`None`).
    pub const fn default() -> Self {
        Self {
            list_type: None,
            style: None,
        }
    }

    /// Sets the [`ListType`] option (`type`).
    pub const fn with_type(mut self, list_type: ListType) -> Self {
        self.list_type = Some(list_type);
        self
    }

    /// Sets the [`ListStyle`] option (`style`).
    pub const fn with_style(mut self, style: ListStyle) -> Self {
        self.style = Some(style);
        self
    }
}

/// The resolved locale and options returned by [`ECMAListFormatter::resolved_options()`],
/// corresponding to `Intl.ListFormat.prototype.resolvedOptions()`.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. Do not use this type unless you are prepared for things to occasionally break.
///
/// Graduation tracking issue: [issue #5868](https://github.com/unicode-org/icu4x/issues/5868).
/// </div>
///
/// ✨ *Enabled with the `unstable` Cargo feature.*
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct ResolvedListFormatterOptions {
    /// The resolved BCP 47 locale (`"locale"` in ECMA-402).
    ///
    /// Per ECMA-402 `%Intl.ListFormat%.[[RelevantExtensionKeys]]` (`« »`), Unicode locale
    /// extensions (`-u-...`) are never retained on the resolved locale.
    pub locale: Locale,
    /// The resolved list formatting type (`"type"` in ECMA-402).
    pub list_type: ListType,
    /// The resolved list formatting style (`"style"` in ECMA-402).
    pub style: ListStyle,
}

/// Wraps a [`DataProvider`] to record the resolved [`DataLocale`] from a successful load.
struct LocaleObservingProvider<'a, P: ?Sized> {
    inner: &'a P,
    silent: bool,
    resolved_locale: Cell<Option<DataLocale>>,
}

impl<M: DataMarker, P: DataProvider<M> + ?Sized> DataProvider<M>
    for LocaleObservingProvider<'_, P>
{
    fn load(&self, mut req: DataRequest) -> Result<DataResponse<M>, DataError> {
        if self.silent {
            req.metadata.silent = true;
        }
        let response = self.inner.load(req)?;
        // In `icu_provider_baked`, an exact trie hit leaves `response.metadata.locale = None`
        // and only populates `Some(fallback_locale)` when fallback steps ran.
        let resolved = response.metadata.locale.unwrap_or(*req.id.locale);
        self.resolved_locale.set(Some(resolved));
        Ok(response)
    }
}

/// An ECMA-402 `Intl.ListFormat` wrapper around [`ListFormatter`] that performs ECMA-402
/// locale negotiation and stores [`ResolvedListFormatterOptions`].
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. Do not use this type unless you are prepared for things to occasionally break.
///
/// Graduation tracking issue: [issue #5868](https://github.com/unicode-org/icu4x/issues/5868).
/// </div>
///
/// ✨ *Enabled with the `unstable` Cargo feature.*
///
/// # Data Generation Requirement (`RetainBaseLanguages`)
///
/// Accurate [`resolved_options()`](Self::resolved_options) locale reporting requires generating
/// provider data with `DeduplicationStrategy::RetainBaseLanguages` (or `DeduplicationStrategy::None`)
/// so supported base languages whose list patterns match `und` do not fall back to `und`.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "compiled_data")]
/// # {
/// use icu_list::ecma::{
///     ECMAListFormatter, ECMAListFormatterOptions, ListStyle, ListType,
/// };
/// use icu_locale_core::locale;
/// use writeable::assert_writeable_eq;
///
/// let requested = [locale!("zxx"), locale!("es-AR-u-nu-latn")];
/// let default_locale = locale!("en");
/// let options = ECMAListFormatterOptions::default()
///     .with_type(ListType::Conjunction)
///     .with_style(ListStyle::Long);
///
/// let formatter =
///     ECMAListFormatter::try_new(&requested, &default_locale, options)
///         .unwrap();
///
/// assert_eq!(formatter.resolved_options().locale, locale!("es"));
/// assert_eq!(formatter.resolved_options().list_type, ListType::Conjunction);
/// assert_eq!(formatter.resolved_options().style, ListStyle::Long);
/// assert_writeable_eq!(
///     formatter.format(["España", "Italia"].iter()),
///     "España e Italia"
/// );
/// # }
/// ```
#[derive(Debug)]
pub struct ECMAListFormatter {
    formatter: ListFormatter,
    resolved_options: ResolvedListFormatterOptions,
}

impl Deref for ECMAListFormatter {
    type Target = ListFormatter;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.formatter
    }
}

impl AsRef<ListFormatter> for ECMAListFormatter {
    #[inline]
    fn as_ref(&self) -> &ListFormatter {
        &self.formatter
    }
}

impl ECMAListFormatter {
    icu_provider::gen_buffer_data_constructors!(
        (
            requested_locales: &[Locale],
            default_locale: &Locale,
            options: ECMAListFormatterOptions
        ) -> error: DataError,
        /// Creates a new [`ECMAListFormatter`] from compiled data, negotiating `requested_locales`
        /// and `default_locale` according to the ECMA-402 `Intl.ListFormat` specification.
        ///
        /// Note: Accurate `resolved_options().locale` behavior requires provider data generated with
        /// `DeduplicationStrategy::RetainBaseLanguages` so base languages whose payloads match `und`
        /// do not fall back to `und`.
    );

    /// Creates a new [`ECMAListFormatter`] using a custom [`DataProvider`], negotiating
    /// `requested_locales` and `default_locale` according to the ECMA-402 `Intl.ListFormat`
    /// specification.
    ///
    /// Callers are expected to pass canonicalized [`Locale`]s (per ECMA-402
    /// `CanonicalizeLocaleList` and `DefaultLocale()`).
    ///
    /// Note: Accurate `resolved_options().locale` behavior requires provider data generated with
    /// `DeduplicationStrategy::RetainBaseLanguages` so base languages whose payloads match `und`
    /// do not fall back to `und`.
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    ///
    /// <div class="stab unstable">⚠️ The bounds on <tt>provider</tt> may change over time, including in SemVer minor releases.</div>
    pub fn try_new_unstable<P>(
        provider: &P,
        requested_locales: &[Locale],
        default_locale: &Locale,
        options: ECMAListFormatterOptions,
    ) -> Result<Self, DataError>
    where
        P: DataProvider<ListAndV1> + DataProvider<ListOrV1> + DataProvider<ListUnitV1> + ?Sized,
    {
        // ECMA-402 `Intl.ListFormat ( [ locales [ , options ] ] )`
        // https://tc39.es/ecma402/#sec-Intl.ListFormat
        //
        // Step 3: Let optionsResolution be ? ResolveOptions(%Intl.ListFormat%,
        //         %Intl.ListFormat%.[[LocaleData]], locales, options).
        //   - ResolveOptions step 4 (`localeMatcher`):
        //     TODO(#5868): We may expose a shared `LocaleMatcher` option once designed across
        //     components. Currently we resolve locales via ICU4X's `LocaleFallbacker` (likely
        //     subtags + CLDR parent locales, e.g. `zh-TW` -> `zh-Hant` and `es-AR` -> `es`) rather
        //     than syntactic `-` prefix truncation against a pre-enumerated `[[AvailableLocales]]`
        //     set. This matches shipping browser engines and satisfies `LookupMatchingLocaleByBestFit`.
        //   - %Intl.ListFormat%.[[RelevantExtensionKeys]] is « » (empty), and
        //     %Intl.ListFormat%.[[ResolutionOptionDescriptors]] is « » (empty).

        // Step 7-8: Let type be ? GetOption(options, "type", ~string~,
        //           « "conjunction", "disjunction", "unit" », "conjunction").
        let list_type = options.list_type.unwrap_or_default();

        // Step 9-10: Let style be ? GetOption(options, "style", ~string~,
        //            « "long", "short", "narrow" », "long").
        let style = options.style.unwrap_or_default();

        // `ResolveLocale ( availableLocales, requestedLocales, options, relevantExtensionKeys, localeData )`
        // https://tc39.es/ecma402/#sec-resolvelocale
        //
        // Steps 1-3: Match `requestedLocales` in priority order (`LookupMatchingLocaleByPrefix` /
        //            `LookupMatchingLocaleByBestFit`).
        //   - `LookupMatchingLocaleByPrefix` step 1.b removes any `-u-` Unicode locale extension
        //     sequence before matching, and step 1.d strips trailing `-t-` / `-x-` / other
        //     singleton extensions because `[[AvailableLocales]]` contains only CLDR locales
        //     without extensions. Converting `&candidate.id` (`&LanguageIdentifier`) strips all
        //     extensions (including `-u-sd-` and `-u-rg-`) upfront without allocation.
        //   - Because `%Intl.ListFormat%.[[RelevantExtensionKeys]]` is empty (`« »`),
        //     `ResolveLocale` steps 10-12 never re-attach any `-u-` keywords to `[[Locale]]`.
        for candidate in requested_locales {
            // `"und"` and `"und-*"` tags are never elements of `%Intl.ListFormat%.[[AvailableLocales]]`.
            if candidate.id.language.is_unknown() {
                continue;
            }

            match Self::load_candidate(provider, &candidate.id, list_type, style, true) {
                Ok((formatter, resolved_data_locale))
                    if !resolved_data_locale.is_unknown()
                        && !resolved_data_locale.language.is_unknown() =>
                {
                    // Step 6 & 11-14: Set `[[Locale]]` to `resolvedLocale.[[Locale]]` and load
                    // `[[Templates]]` for `[[Type]]` and `[[Style]]`.
                    return Ok(Self {
                        formatter,
                        resolved_options: ResolvedListFormatterOptions {
                            locale: resolved_data_locale.into_locale(),
                            list_type,
                            style,
                        },
                    });
                }
                // Candidate fell back to root (`und`) or was absent from the provider:
                // treat as `undefined` match and try the next requested locale.
                Ok(_) => continue,
                Err(e) if e.kind == DataErrorKind::IdentifierNotFound => continue,
                Err(e) => return Err(e),
            }
        }

        // `ResolveLocale` step 4:
        // If match is undefined, set match to the Record { [[locale]]: DefaultLocale(), [[extension]]: ~empty~ }.
        let (formatter, resolved_data_locale) =
            Self::load_candidate(provider, &default_locale.id, list_type, style, false)?;

        Ok(Self {
            formatter,
            resolved_options: ResolvedListFormatterOptions {
                locale: resolved_data_locale.into_locale(),
                list_type,
                style,
            },
        })
    }

    /// Loads the [`ListFormatter`] for a single extension-stripped [`LanguageIdentifier`] while
    /// observing the resolved [`DataLocale`] from the underlying [`DataProvider`].
    fn load_candidate<P>(
        provider: &P,
        langid: &LanguageIdentifier,
        list_type: ListType,
        style: ListStyle,
        silent: bool,
    ) -> Result<(ListFormatter, DataLocale), DataError>
    where
        P: DataProvider<ListAndV1> + DataProvider<ListOrV1> + DataProvider<ListUnitV1> + ?Sized,
    {
        let observer = LocaleObservingProvider {
            inner: provider,
            silent,
            resolved_locale: Cell::new(None),
        };
        let prefs = ListFormatterPreferences::from(langid);
        let list_options = ListFormatterOptions::default().with_length(style.to_list_length());
        let formatter = match list_type {
            ListType::Conjunction => {
                ListFormatter::try_new_and_unstable(&observer, prefs, list_options)?
            }
            ListType::Disjunction => {
                ListFormatter::try_new_or_unstable(&observer, prefs, list_options)?
            }
            ListType::Unit => ListFormatter::try_new_unit_unstable(&observer, prefs, list_options)?,
        };
        // Non-spec glue: `observer.resolved_locale` is always populated on an `Ok` load;
        // `unwrap_or_else` avoids `unwrap`/`expect` in `no_std` library code.
        let resolved = observer
            .resolved_locale
            .get()
            .unwrap_or_else(|| DataLocale::from(langid));
        Ok((formatter, resolved))
    }

    /// Filters `requested_locales` to the subset supported by `provider` without falling back to
    /// `und`, corresponding to ECMA-402 `Intl.ListFormat.supportedLocalesOf(locales, options)`.
    ///
    /// Callers are expected to pass canonicalized [`Locale`]s (per ECMA-402
    /// `CanonicalizeLocaleList`). Matching input [`Locale`]s are returned in their original
    /// relative order with any Unicode (`-u-`), transformed (`-t-`), or private-use (`-x-`)
    /// extensions and regional subtags preserved per ECMA-402 `FilterLocales`.
    ///
    /// Note: This method requires a [`DryDataProvider<ListAndV1>`] backed by data exported with
    /// `DeduplicationStrategy::RetainBaseLanguages` (or `DeduplicationStrategy::None`).
    ///
    /// ✨ *Enabled with the `alloc` and `unstable` Cargo features.*
    ///
    /// <div class="stab unstable">⚠️ The bounds on <tt>provider</tt> may change over time, including in SemVer minor releases.</div>
    #[cfg(feature = "alloc")]
    pub fn supported_locales_of_unstable(
        provider: &(impl DryDataProvider<ListAndV1> + ?Sized),
        requested_locales: &[Locale],
    ) -> Result<Vec<Locale>, DataError> {
        // ECMA-402 `Intl.ListFormat.supportedLocalesOf ( locales [ , options ] )`
        // https://tc39.es/ecma402/#sec-Intl.ListFormat.supportedLocalesOf
        // and `FilterLocales ( availableLocales, requestedLocales, options )`
        // https://tc39.es/ecma402/#sec-filterlocales
        //
        // Step 1-2 (`CoerceOptionsToObject` + `GetOption(options, "localeMatcher", ...)`):
        // Handled by the caller on the JS options object.
        // TODO(#5868): Consider accepting a shared `LocaleMatcher` option once designed across
        // components. Currently probes `ListAndV1` (`WIDE`) via `DryDataProvider::dry_load` using
        // ICU4X `LocaleFallbacker` rules.

        // Step 3: Let subset be a new empty List.
        let mut subset = Vec::new();

        // Step 4: For each element locale of requestedLocales, do
        for locale in requested_locales {
            // `"und"` and `"und-*"` tags are never elements of `%Intl.ListFormat%.[[AvailableLocales]]`.
            if locale.id.language.is_unknown() {
                continue;
            }

            // `LookupMatchingLocaleByPrefix` step 1.b: strip Unicode locale extensions (and other
            // non-LanguageIdentifier extensions) when probing availability.
            let data_locale = DataLocale::from(&locale.id);
            let mut req_metadata = DataRequestMetadata::default();
            req_metadata.silent = true;
            let req = DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    ListFormatterPatterns::WIDE,
                    &data_locale,
                ),
                metadata: req_metadata,
            };

            match provider.dry_load(req) {
                Ok(resp_metadata) => {
                    let resolved = resp_metadata.locale.unwrap_or(data_locale);
                    // Step 4.c: If match is not undefined, append `locale` (with its original
                    // subtags and extensions intact) to `subset`.
                    if !resolved.is_unknown() && !resolved.language.is_unknown() {
                        subset.push(locale.clone());
                    }
                }
                Err(e) if e.kind == DataErrorKind::IdentifierNotFound => continue,
                Err(e) => return Err(e),
            }
        }

        // Step 5: Return CreateArrayFromList(subset).
        Ok(subset)
    }

    /// Returns the [`ResolvedListFormatterOptions`] computed during construction, corresponding
    /// to ECMA-402 `Intl.ListFormat.prototype.resolvedOptions()`.
    ///
    /// Note: Accurate `locale` resolution requires provider data generated with
    /// `DeduplicationStrategy::RetainBaseLanguages` so base languages whose payloads match `und`
    /// do not fall back to `und`.
    #[inline]
    pub const fn resolved_options(&self) -> &ResolvedListFormatterOptions {
        // ECMA-402 `Intl.ListFormat.prototype.resolvedOptions ( )`
        // https://tc39.es/ecma402/#sec-Intl.ListFormat.prototype.resolvedoptions
        // Table: [[Locale]] -> "locale", [[Type]] -> "type", [[Style]] -> "style".
        &self.resolved_options
    }

    /// Consumes this [`ECMAListFormatter`] and returns the inner [`ListFormatter`].
    #[inline]
    pub fn into_inner(self) -> ListFormatter {
        self.formatter
    }

    /// Consumes this [`ECMAListFormatter`] and returns the inner [`ListFormatter`] along with its
    /// [`ResolvedListFormatterOptions`].
    #[inline]
    pub fn into_parts(self) -> (ListFormatter, ResolvedListFormatterOptions) {
        (self.formatter, self.resolved_options)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "compiled_data")]
    use icu_locale_core::locale;
    #[cfg(feature = "compiled_data")]
    use writeable::assert_writeable_eq;

    #[cfg(feature = "compiled_data")]
    #[test]
    fn test_locale_negotiation_and_extensions() {
        let default_en = locale!("en");

        // Exact base language hits
        let lf = ECMAListFormatter::try_new(
            &[locale!("en")],
            &default_en,
            ECMAListFormatterOptions::default(),
        )
        .unwrap();
        assert_eq!(lf.resolved_options().locale, locale!("en"));

        let lf = ECMAListFormatter::try_new(
            &[locale!("es")],
            &default_en,
            ECMAListFormatterOptions::default(),
        )
        .unwrap();
        assert_eq!(lf.resolved_options().locale, locale!("es"));

        // Regional deduplication (`es-AR` -> `es`)
        let lf = ECMAListFormatter::try_new(
            &[locale!("es-AR")],
            &default_en,
            ECMAListFormatterOptions::default(),
        )
        .unwrap();
        assert_eq!(lf.resolved_options().locale, locale!("es"));

        // Likely subtags / CLDR parent resolution (`zh-TW` and `zh-Hant-TW` -> `zh-Hant`,
        // rather than syntactic `-` prefix truncation to `zh`)
        for tag in [locale!("zh-TW"), locale!("zh-Hant-TW")] {
            let lf = ECMAListFormatter::try_new(
                &[tag],
                &default_en,
                ECMAListFormatterOptions::default(),
            )
            .unwrap();
            assert_eq!(lf.resolved_options().locale, locale!("zh-Hant"));
        }

        // `%Intl.ListFormat%.[[RelevantExtensionKeys]]` is empty (`« »`), so `-u-` extensions
        // (including `-u-nu-*`, `-u-co-*`, `-u-sd-*`, `-u-rg-*`) as well as `-t-` and `-x-`
        // extensions are stripped and never retained in `resolvedOptions().locale`.
        for raw in [
            "es-AR-u-nu-latn",
            "es-u-nu-thai",
            "es-u-co-phonebk",
            "es-ES-u-sd-esct",
            "es-u-rg-mxzzzz",
            "es-AR-t-en-x-private",
        ] {
            let requested = [raw.parse::<Locale>().unwrap()];
            let lf = ECMAListFormatter::try_new(
                &requested,
                &default_en,
                ECMAListFormatterOptions::default(),
            )
            .unwrap();
            assert_eq!(lf.resolved_options().locale, locale!("es"), "for {raw}");
        }
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn test_multi_locale_fallback_and_default_locale() {
        let default_de = locale!("de");

        // Empty `requested_locales` uses `default_locale`
        let lf = ECMAListFormatter::try_new(&[], &default_de, ECMAListFormatterOptions::default())
            .unwrap();
        assert_eq!(lf.resolved_options().locale, locale!("de"));
        assert_writeable_eq!(lf.format(["A", "B"].iter()), "A und B");

        // Multi-locale priority list skips unsupported `"zxx"` and resolves `"es-AR"` -> `"es"`
        let lf = ECMAListFormatter::try_new(
            &[locale!("zxx"), locale!("es-AR")],
            &default_de,
            ECMAListFormatterOptions::default(),
        )
        .unwrap();
        assert_eq!(lf.resolved_options().locale, locale!("es"));
        assert_writeable_eq!(lf.format(["España", "Ibiza"].iter()), "España e Ibiza");

        // `"und"` and `"und-*"` tags are not in `[[AvailableLocales]]` and must be skipped
        let lf = ECMAListFormatter::try_new(
            &[
                locale!("und"),
                locale!("und-US"),
                locale!("und-Latn"),
                locale!("fr-CA"),
            ],
            &default_de,
            ECMAListFormatterOptions::default(),
        )
        .unwrap();
        assert_eq!(lf.resolved_options().locale, locale!("fr"));
        assert_writeable_eq!(lf.format(["A", "B"].iter()), "A et B");

        // All unsupported candidates fall back to `default_locale`
        let lf = ECMAListFormatter::try_new(
            &[locale!("zxx"), locale!("tlh")],
            &locale!("es-AR"),
            ECMAListFormatterOptions::default(),
        )
        .unwrap();
        assert_eq!(lf.resolved_options().locale, locale!("es"));
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn test_all_types_styles_and_deref_formatting() {
        let es = [locale!("es")];
        let default_en = locale!("en");
        let items = ["uno", "dos", "tres"];

        let cases = [
            (ListType::Conjunction, ListStyle::Long, "uno, dos y tres"),
            (ListType::Conjunction, ListStyle::Short, "uno, dos y tres"),
            (ListType::Conjunction, ListStyle::Narrow, "uno, dos y tres"),
            (ListType::Disjunction, ListStyle::Long, "uno, dos o tres"),
            (ListType::Disjunction, ListStyle::Short, "uno, dos o tres"),
            (ListType::Disjunction, ListStyle::Narrow, "uno, dos o tres"),
            (ListType::Unit, ListStyle::Long, "uno, dos y tres"),
            (ListType::Unit, ListStyle::Short, "uno, dos, tres"),
            (ListType::Unit, ListStyle::Narrow, "uno dos tres"),
        ];

        for (list_type, style, expected) in cases {
            let options = ECMAListFormatterOptions::default()
                .with_type(list_type)
                .with_style(style);
            let lf = ECMAListFormatter::try_new(&es, &default_en, options).unwrap();

            assert_eq!(lf.resolved_options().locale, locale!("es"));
            assert_eq!(lf.resolved_options().list_type, list_type);
            assert_eq!(lf.resolved_options().style, style);

            // Formatting via `Deref<Target = ListFormatter>` and `AsRef<ListFormatter>`
            assert_writeable_eq!(lf.format(items.iter()), expected);
            assert_writeable_eq!(lf.as_ref().format(items.iter()), expected);
            #[cfg(feature = "alloc")]
            assert_eq!(lf.format_to_string(items.iter()), expected);

            let (inner, resolved) = lf.into_parts();
            assert_eq!(resolved.locale, locale!("es"));
            assert_writeable_eq!(inner.format(items.iter()), expected);
        }

        // Also check English conjunction widths (`"and"`, `"&"`, `", "`) and `into_inner()`.
        // Note: Upstream `icu_list_data` currently uses `DeduplicationStrategy::Maximal`, which
        // strips `en/N` (`ListStyle::Narrow`, `"{0}, {1}"`) because it matches `und/N`, whereas
        // `en/W` (`Long`) and `en/S` (`Short`) differ from `und` and resolve to `"en"`.
        let en = [locale!("en")];
        let en_items = ["one", "two", "three"];
        for (style, expected_locale, expected) in [
            (ListStyle::Long, locale!("en"), "one, two, and three"),
            (ListStyle::Short, locale!("en"), "one, two, & three"),
            (ListStyle::Narrow, locale!("und"), "one, two, three"),
        ] {
            let options = ECMAListFormatterOptions::default()
                .with_type(ListType::Conjunction)
                .with_style(style);
            let lf = ECMAListFormatter::try_new(&en, &default_en, options).unwrap();
            assert_eq!(lf.resolved_options().locale, expected_locale);
            let inner = lf.into_inner();
            assert_writeable_eq!(inner.format(en_items.iter()), expected);
        }

        assert_eq!(ListType::Conjunction.as_str(), "conjunction");
        assert_eq!(ListType::Disjunction.as_str(), "disjunction");
        assert_eq!(ListType::Unit.as_str(), "unit");
        assert_eq!(ListStyle::Long.as_str(), "long");
        assert_eq!(ListStyle::Short.as_str(), "short");
        assert_eq!(ListStyle::Narrow.as_str(), "narrow");
    }

    /// Mock provider simulating `DeduplicationStrategy::Maximal` vs
    /// `DeduplicationStrategy::RetainBaseLanguages` for a base language (`"en"` on unit lists)
    /// whose payload is identical to `und` (`"{0}, {1}"`), plus regional fallback (`"es-AR"` -> `"es"`).
    struct MockRetainBaseProvider {
        retain_base_languages: bool,
    }

    impl MockRetainBaseProvider {
        fn unit_pattern(sep: &'static [u8]) -> ListFormatterPatterns<'static> {
            use crate::provider::{ConditionalListJoinerPattern, ListJoinerPattern};
            use zerovec::VarZeroCow;

            // SAFETY: ASCII separator bytes are valid `str` VarULE bytes.
            let cow: VarZeroCow<'static, str> = unsafe { VarZeroCow::from_bytes_unchecked(sep) };
            let len = u8::try_from(sep.len()).unwrap();
            ListFormatterPatterns {
                start: ListJoinerPattern::from_parts(cow.clone(), len),
                middle: cow.clone(),
                end: ConditionalListJoinerPattern {
                    default: ListJoinerPattern::from_parts(cow, len),
                    special_case: None,
                },
                pair: None,
            }
        }

        /// Simulates `icu_provider_baked` lookup + fallback:
        /// - Exact trie hits return `metadata.locale = None`.
        /// - Fallback hits return `metadata.locale = Some(fallback_locale)`.
        fn resolve_locale(&self, req_locale: DataLocale) -> DataLocale {
            let lang = req_locale.language.as_str();
            if lang == "es" {
                // Simulate regional deduplication: `"es"` is in the trie, `"es-AR"` falls back to `"es"`.
                icu_locale_core::data_locale!("es")
            } else if lang == "en" && self.retain_base_languages {
                // Under `RetainBaseLanguages`, base language `"en"` is kept in the trie even though
                // its payload matches `"und"` (`"{0}, {1}"`); `"en-US"` falls back to `"en"`.
                icu_locale_core::data_locale!("en")
            } else {
                // Under `Maximal` (or for unsupported locales like `"zxx"`), falls back to `"und"`.
                DataLocale::default()
            }
        }

        fn build_response<M>(&self, req: DataRequest) -> DataResponse<M>
        where
            M: DataMarker<DataStruct = ListFormatterPatterns<'static>>,
        {
            let resolved = self.resolve_locale(*req.id.locale);
            let mut metadata = DataResponseMetadata::default();
            // Match `icu_provider_baked` contract: `None` on exact hit, `Some` only on fallback!
            if resolved != *req.id.locale {
                metadata.locale = Some(resolved);
            }
            let sep: &'static [u8] = if resolved.language.as_str() == "es" {
                b" y "
            } else {
                b", " // Both `"en"` unit lists and `"und"` root patterns use `", "`!
            };
            DataResponse {
                metadata,
                payload: DataPayload::from_owned(Self::unit_pattern(sep)),
            }
        }
    }

    impl DataProvider<ListAndV1> for MockRetainBaseProvider {
        fn load(&self, req: DataRequest) -> Result<DataResponse<ListAndV1>, DataError> {
            Ok(self.build_response(req))
        }
    }

    impl DataProvider<ListOrV1> for MockRetainBaseProvider {
        fn load(&self, req: DataRequest) -> Result<DataResponse<ListOrV1>, DataError> {
            Ok(self.build_response(req))
        }
    }

    impl DataProvider<ListUnitV1> for MockRetainBaseProvider {
        fn load(&self, req: DataRequest) -> Result<DataResponse<ListUnitV1>, DataError> {
            Ok(self.build_response(req))
        }
    }

    impl DryDataProvider<ListAndV1> for MockRetainBaseProvider {
        fn dry_load(&self, req: DataRequest) -> Result<DataResponseMetadata, DataError> {
            let resolved = self.resolve_locale(*req.id.locale);
            let mut metadata = DataResponseMetadata::default();
            if resolved != *req.id.locale {
                metadata.locale = Some(resolved);
            }
            Ok(metadata)
        }
    }

    #[test]
    fn test_retain_base_languages_and_supported_locales_of() {
        let en = icu_locale_core::locale!("en");
        let en_us = icu_locale_core::locale!("en-US");
        let es = icu_locale_core::locale!("es");
        let es_ar = icu_locale_core::locale!("es-AR");
        let es_ext = icu_locale_core::locale!("es-u-nu-latn");
        let zxx = icu_locale_core::locale!("zxx");
        let und = icu_locale_core::locale!("und");
        let und_us = icu_locale_core::locale!("und-US");

        let unit_opts = ECMAListFormatterOptions::default()
            .with_type(ListType::Unit)
            .with_style(ListStyle::Long);

        // 1. With `RetainBaseLanguages`: `"en"` (exact trie hit -> `metadata.locale = None`) and
        //    `"en-US"` (fallback -> `metadata.locale = Some("en")`) both resolve to `"en"`, even
        //    though `"en"` shares the `", "` payload with `"und"`.
        let retain_provider = MockRetainBaseProvider {
            retain_base_languages: true,
        };
        let lf_en = ECMAListFormatter::try_new_unstable(
            &retain_provider,
            core::slice::from_ref(&en),
            &es,
            unit_opts,
        )
        .unwrap();
        assert_eq!(lf_en.resolved_options().locale, en);

        let lf_en_us = ECMAListFormatter::try_new_unstable(
            &retain_provider,
            core::slice::from_ref(&en_us),
            &es,
            unit_opts,
        )
        .unwrap();
        assert_eq!(lf_en_us.resolved_options().locale, en);

        // 2. Without `RetainBaseLanguages` (`Maximal`): `"en"` falls back to `"und"`, so
        //    negotiation treats `"en"` as unsupported and falls back to `default_locale` (`"es"`).
        let maximal_provider = MockRetainBaseProvider {
            retain_base_languages: false,
        };
        let lf_maximal = ECMAListFormatter::try_new_unstable(
            &maximal_provider,
            core::slice::from_ref(&en),
            &es,
            unit_opts,
        )
        .unwrap();
        assert_eq!(lf_maximal.resolved_options().locale, es);

        // 3. `supported_locales_of_unstable`:
        //    - Preserves input extensions (`"es-u-nu-latn"`) and regional subtags (`"es-AR"`, `"en-US"`)
        //      per ECMA-402 `FilterLocales` step 4.c / `LookupSupportedLocales` step 2.c.
        //    - Rejects `"zxx"`, `"und"`, and `"und-US"`.
        //    - Includes `"en"` / `"en-US"` under `RetainBaseLanguages`, omits them under `Maximal`.
        #[cfg(feature = "alloc")]
        {
            let query = [
                zxx.clone(),
                und.clone(),
                und_us.clone(),
                en_us.clone(),
                es_ext.clone(),
                es_ar.clone(),
            ];

            let supported_retain =
                ECMAListFormatter::supported_locales_of_unstable(&retain_provider, &query).unwrap();
            assert_eq!(
                supported_retain,
                alloc::vec![en_us.clone(), es_ext.clone(), es_ar.clone()]
            );

            let supported_maximal =
                ECMAListFormatter::supported_locales_of_unstable(&maximal_provider, &query)
                    .unwrap();
            assert_eq!(supported_maximal, alloc::vec![es_ext, es_ar]);
        }
    }

    #[test]
    fn test_non_not_found_error_propagates() {
        struct ErrorProvider;

        impl DataProvider<ListAndV1> for ErrorProvider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<ListAndV1>, DataError> {
                Err(DataErrorKind::MarkerNotFound.with_req(ListAndV1::INFO, req))
            }
        }
        impl DataProvider<ListOrV1> for ErrorProvider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<ListOrV1>, DataError> {
                Err(DataErrorKind::MarkerNotFound.with_req(ListOrV1::INFO, req))
            }
        }
        impl DataProvider<ListUnitV1> for ErrorProvider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<ListUnitV1>, DataError> {
                Err(DataErrorKind::MarkerNotFound.with_req(ListUnitV1::INFO, req))
            }
        }
        impl DryDataProvider<ListAndV1> for ErrorProvider {
            fn dry_load(&self, req: DataRequest) -> Result<DataResponseMetadata, DataError> {
                Err(DataErrorKind::MarkerNotFound.with_req(ListAndV1::INFO, req))
            }
        }

        let requested = [icu_locale_core::locale!("es")];
        let default_en = icu_locale_core::locale!("en");
        let err = ECMAListFormatter::try_new_unstable(
            &ErrorProvider,
            &requested,
            &default_en,
            ECMAListFormatterOptions::default(),
        )
        .unwrap_err();
        assert_eq!(err.kind, DataErrorKind::MarkerNotFound);

        #[cfg(feature = "alloc")]
        {
            let dry_err =
                ECMAListFormatter::supported_locales_of_unstable(&ErrorProvider, &requested)
                    .unwrap_err();
            assert_eq!(dry_err.kind, DataErrorKind::MarkerNotFound);
        }
    }
}
