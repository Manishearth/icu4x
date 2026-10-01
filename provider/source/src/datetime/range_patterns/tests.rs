// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Tests verifying range pattern consistency and coverage across locales in CLDR.
//!
//! # Overlap Pattern Coverage Invariant
//! Overlap skeletons such as `ej` (`ET`, weekday + time) unify date and time fields into an atomic
//! pattern (e.g. `ccc h:mm a`) without separate datetime glue (`{1}, {0}`). In `icu_datetime`, when
//! an overlap pattern is selected in `DateTimeZonePatternSelectionData::try_new_with_skeleton`,
//! it returns early with `date: DatePatternSelectionData::none()` and `glue: None`.
//!
//! Consequently, when a range formatter (`FixedCalendarDateRangeFormatter` or `DateRangeFormatter`)
//! formats an interval using an overlap skeleton, Case 3 (mixed range where only time differs)
//! cannot decompose into `<date><glue><time_range>`. Instead, it relies on fallback range formatting
//! (`format_fallback`), which combines the single datetime overlap formatting with fallback range
//! glue.
//!
//! To ensure range formatting operates cleanly and without unexpected pattern load failures across
//! all locales, we rely on the critical assumption that any locale in CLDR supporting `ej` in standard
//! date/time patterns ALSO supports `ej` in its interval range pattern data. The tests below enforce
//! this invariant across all 12 supported calendars and all locales in CLDR.

use super::*;
use crate::SourceDataProvider;
use icu::datetime::provider::semantic_skeletons::*;

fn check_ej_coverage<M1, M2>(provider: &SourceDataProvider, calendar: DatagenCalendar)
where
    SourceDataProvider: DataProvider<M1> + DataProvider<M2>,
    M1: DataMarker,
    M2: DataMarker,
{
    let dates = provider.cldr().unwrap().dates(Some(calendar));
    let locales = dates.list_locales().unwrap();

    for locale in locales {
        let req = DataRequest {
            id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                DataMarkerAttributes::from_str_or_panic("ej"),
                &locale,
            ),
            metadata: Default::default(),
        };

        let has_date_pattern = DataProvider::<M1>::load(provider, req).is_ok();
        let has_range_pattern = DataProvider::<M2>::load(provider, req).is_ok();

        if has_date_pattern {
            assert!(
                has_range_pattern,
                "Locale {locale} ({calendar:?}) has an `ej` date pattern but is missing an `ej` date range pattern"
            );
        }
    }
}

macro_rules! check_all_calendars {
    ($provider:expr) => {
        check_ej_coverage::<DatetimePatternsDateBuddhistV1, DatetimePatternsRangeDateBuddhistV1>(
            $provider,
            DatagenCalendar::Buddhist,
        );
        check_ej_coverage::<DatetimePatternsDateChineseV1, DatetimePatternsRangeDateChineseV1>(
            $provider,
            DatagenCalendar::Chinese,
        );
        check_ej_coverage::<DatetimePatternsDateCopticV1, DatetimePatternsRangeDateCopticV1>(
            $provider,
            DatagenCalendar::Coptic,
        );
        check_ej_coverage::<DatetimePatternsDateDangiV1, DatetimePatternsRangeDateDangiV1>(
            $provider,
            DatagenCalendar::Dangi,
        );
        check_ej_coverage::<DatetimePatternsDateEthiopianV1, DatetimePatternsRangeDateEthiopianV1>(
            $provider,
            DatagenCalendar::Ethiopic,
        );
        check_ej_coverage::<DatetimePatternsDateGregorianV1, DatetimePatternsRangeDateGregorianV1>(
            $provider,
            DatagenCalendar::Gregorian,
        );
        check_ej_coverage::<DatetimePatternsDateHebrewV1, DatetimePatternsRangeDateHebrewV1>(
            $provider,
            DatagenCalendar::Hebrew,
        );
        check_ej_coverage::<DatetimePatternsDateIndianV1, DatetimePatternsRangeDateIndianV1>(
            $provider,
            DatagenCalendar::Indian,
        );
        check_ej_coverage::<DatetimePatternsDateHijriV1, DatetimePatternsRangeDateHijriV1>(
            $provider,
            DatagenCalendar::Hijri,
        );
        check_ej_coverage::<DatetimePatternsDateJapaneseV1, DatetimePatternsRangeDateJapaneseV1>(
            $provider,
            DatagenCalendar::Japanese,
        );
        check_ej_coverage::<DatetimePatternsDatePersianV1, DatetimePatternsRangeDatePersianV1>(
            $provider,
            DatagenCalendar::Persian,
        );
        check_ej_coverage::<DatetimePatternsDateRocV1, DatetimePatternsRangeDateRocV1>(
            $provider,
            DatagenCalendar::Roc,
        );
    };
}

#[test]
fn test_ej_overlap_coverage_testing() {
    let provider = SourceDataProvider::new_testing();
    check_all_calendars!(&provider);
}

/// Runs over the complete downloaded CLDR database across all locales and calendars.
/// Marked as `#[ignore]` because enumerating and checking ~12,000 data request combinations
/// across all 12 calendars and hundreds of locales takes ~46 seconds.
#[test]
#[ignore]
#[cfg(feature = "networking")]
fn test_ej_overlap_coverage_all_locales() {
    let provider = SourceDataProvider::new();
    check_all_calendars!(&provider);
}

fn get_range_element<'a>(
    range: &'a PackedRangePatterns<'a>,
    length: icu::datetime::options::Length,
    variant: usize, // 0 = Standard, 1 = Variant0, 2 = Variant1
) -> Option<&'a PatternsByGreatestDifferenceULE> {
    use icu::datetime::options::Length;
    let lms = range.header & 0x3;
    let pattern_index = if variant == 0 {
        match (length, lms) {
            (Length::Long, _) => 0,
            (Length::Medium, 0 | 2) => 0,
            (Length::Medium, _) => 1,
            (Length::Short, 0) => 0,
            (Length::Short, 1 | 2) => 1,
            (Length::Short, _) => 2,
            _ => unreachable!(),
        }
    } else {
        let s_offset = match lms {
            0 => 0,
            1 | 2 => 1,
            _ => 2,
        };
        let q = range.header & 0x4;
        let cell = match (length, variant) {
            (Length::Long, 1) => 0,
            (Length::Medium, 1) => 1,
            (Length::Short, 1) => 2,
            (Length::Long, 2) => 3,
            (Length::Medium, 2) => 4,
            (Length::Short, 2) => 5,
            _ => unreachable!(),
        };
        if q == 0 {
            let chunk = (range.header >> (3 * (cell + 1))) & 0x7;
            if chunk == 0 {
                return get_range_element(range, length, 0);
            }
            chunk - 1
        } else {
            s_offset + 1 + cell
        }
    };
    range.elements.get(pattern_index as usize)
}

fn check_names_consistency<MDate, MRange>(
    provider: &SourceDataProvider,
    cal_name: &str,
    locales: &[DataLocale],
    attrs_list: &[&DataMarkerAttributes],
) where
    SourceDataProvider: DataProvider<MDate> + DataProvider<MRange>,
    MDate:
        DataMarker<DataStruct = icu::datetime::provider::packed_pattern::PackedPatterns<'static>>,
    MRange: DataMarker<DataStruct = PackedRangePatterns<'static>>,
{
    use icu::datetime::options::Length;
    use icu::datetime::pattern::{DateTimePattern, FixedCalendarDateTimeNames};

    let mut conflicts = Vec::new();
    for locale in locales {
        for &attrs in attrs_list {
            let req = DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(attrs, locale),
                metadata: Default::default(),
            };
            let Ok(single_resp) = DataProvider::<MDate>::load(provider, req) else {
                continue;
            };
            let Ok(range_resp) = DataProvider::<MRange>::load(provider, req) else {
                continue;
            };
            let single_builder = single_resp.payload.get().to_builder();
            let range = range_resp.payload.get();

            let single_v1_lms = single_builder
                .variant1
                .as_ref()
                .unwrap_or(&single_builder.standard);

            for length in [Length::Long, Length::Medium, Length::Short] {
                // Single formatter loads Variant1 (with Era/FullYear)
                let single_v1_plural = match length {
                    Length::Long => &single_v1_lms.long,
                    Length::Medium => &single_v1_lms.medium,
                    Length::Short => &single_v1_lms.short,
                    _ => unreachable!(),
                };
                let single_v1 = single_v1_plural.clone().try_into_other().unwrap();
                let dt_single = DateTimePattern::from(single_v1.clone());

                for variant in [0usize, 1, 2] {
                    let Some(pgd_ule) = get_range_element(range, length, variant) else {
                        continue;
                    };
                    let pgd = PatternsByGreatestDifference::zero_from(pgd_ule);
                    for pat in pgd.patterns.iter() {
                        let rt_pat = Pattern::zero_from(pat);
                        let dt_pat = DateTimePattern::from(rt_pat.clone());
                        let mut names_test =
                            FixedCalendarDateTimeNames::<()>::new_without_number_formatting(
                                Default::default(),
                            );
                        names_test
                            .load_for_pattern(&DebugProvider, &dt_single)
                            .unwrap();
                        if let Err(e) = names_test
                            .load_for_pattern(&crate::debug_provider::EmptyProvider, &dt_pat)
                        {
                            conflicts.push(format!(
                                "CONFLICT {cal_name}/{locale}/{attrs:?}/{length:?}/v{variant}: single={single_v1:?} range={rt_pat:?} err={e:?}"
                            ));
                        }
                    }
                }
            }
        }
    }
    assert!(
        conflicts.is_empty(),
        "Found {} range pattern name conflicts in {cal_name}:\n{}",
        conflicts.len(),
        conflicts.join("\n")
    );
}

fn check_all_names_consistency(provider: &SourceDataProvider, locales: &[DataLocale]) {
    let date_attrs: Vec<&DataMarkerAttributes> = DateFieldSet::ALL_DATA_MARKER_ATTRIBUTES
        .iter()
        .chain(CalendarPeriodFieldSet::ALL_DATA_MARKER_ATTRIBUTES.iter())
        .chain(DateAndTimeFieldSet::ALL_DATA_MARKER_ATTRIBUTES.iter())
        .copied()
        .collect();

    check_names_consistency::<DatetimePatternsDateGregorianV1, DatetimePatternsRangeDateGregorianV1>(
        provider,
        "gregorian",
        locales,
        &date_attrs,
    );
    check_names_consistency::<DatetimePatternsDateBuddhistV1, DatetimePatternsRangeDateBuddhistV1>(
        provider,
        "buddhist",
        locales,
        &date_attrs,
    );
    check_names_consistency::<DatetimePatternsDateHebrewV1, DatetimePatternsRangeDateHebrewV1>(
        provider,
        "hebrew",
        locales,
        &date_attrs,
    );
    check_names_consistency::<DatetimePatternsTimeV1, DatetimePatternsRangeTimeV1>(
        provider,
        "time",
        locales,
        TimeFieldSet::ALL_DATA_MARKER_ATTRIBUTES,
    );
}

/// Verifies across the test locales that every range pattern can be formatted using the
/// `FixedCalendarDateTimeNames` loaded by the corresponding single pattern (with no missing or
/// conflicting field widths/symbols).
#[test]
fn test_range_pattern_names_consistency_testing() {
    let provider = SourceDataProvider::new_testing();
    let locales: Vec<DataLocale> = provider
        .cldr()
        .unwrap()
        .dates(Some(DatagenCalendar::Gregorian))
        .list_locales()
        .unwrap()
        .collect();
    check_all_names_consistency(&provider, &locales);
}

/// Verifies across all modern locales in CLDR that every range pattern can be formatted using the
/// `FixedCalendarDateTimeNames` loaded by the corresponding single pattern.
#[test]
#[ignore]
#[cfg(feature = "networking")]
fn test_range_pattern_names_consistency_all_locales() {
    use crate::CoverageLevel;

    let provider = SourceDataProvider::new();
    let locales: Vec<DataLocale> = provider
        .locales_for_coverage_levels([CoverageLevel::Modern])
        .unwrap()
        .into_iter()
        .collect();
    check_all_names_consistency(&provider, &locales);
}
