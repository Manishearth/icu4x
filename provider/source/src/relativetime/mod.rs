// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::IterableDataProviderCached;
use crate::SourceDataProvider;
use crate::cldr_serde;
use icu::experimental::relativetime::provider::*;
use icu::plurals::PluralElements;
use icu::plurals::provider::PluralElementsPackedCow;
use icu_pattern::SinglePlaceholderPattern;
use icu_provider::prelude::*;
use std::collections::{HashMap, HashSet};
use std::sync::OnceLock;

pub(crate) static MARKER_FILTERS: OnceLock<HashMap<DataMarkerInfo, &'static str>> = OnceLock::new();

fn marker_filters() -> &'static HashMap<DataMarkerInfo, &'static str> {
    MARKER_FILTERS.get_or_init(|| {
        [
            (DatetimeRelativeSecondLongV1::INFO, "second"),
            (DatetimeRelativeSecondShortV1::INFO, "second-short"),
            (DatetimeRelativeSecondNarrowV1::INFO, "second-narrow"),
            (DatetimeRelativeMinuteLongV1::INFO, "minute"),
            (DatetimeRelativeMinuteShortV1::INFO, "minute-short"),
            (DatetimeRelativeMinuteNarrowV1::INFO, "minute-narrow"),
            (DatetimeRelativeHourLongV1::INFO, "hour"),
            (DatetimeRelativeHourShortV1::INFO, "hour-short"),
            (DatetimeRelativeHourNarrowV1::INFO, "hour-narrow"),
            (DatetimeRelativeDayLongV1::INFO, "day"),
            (DatetimeRelativeDayShortV1::INFO, "day-short"),
            (DatetimeRelativeDayNarrowV1::INFO, "day-narrow"),
            (DatetimeRelativeWeekLongV1::INFO, "week"),
            (DatetimeRelativeWeekShortV1::INFO, "week-short"),
            (DatetimeRelativeWeekNarrowV1::INFO, "week-narrow"),
            (DatetimeRelativeMonthLongV1::INFO, "month"),
            (DatetimeRelativeMonthShortV1::INFO, "month-short"),
            (DatetimeRelativeMonthNarrowV1::INFO, "month-narrow"),
            (DatetimeRelativeQuarterLongV1::INFO, "quarter"),
            (DatetimeRelativeQuarterShortV1::INFO, "quarter-short"),
            (DatetimeRelativeQuarterNarrowV1::INFO, "quarter-narrow"),
            (DatetimeRelativeYearLongV1::INFO, "year"),
            (DatetimeRelativeYearShortV1::INFO, "year-short"),
            (DatetimeRelativeYearNarrowV1::INFO, "year-narrow"),
        ]
        .into_iter()
        .collect()
    })
}
macro_rules! make_data_provider {
    ($($marker: ident),+ $(,)?) => {
        $(
            impl DataProvider<$marker> for SourceDataProvider {
                fn load(&self, req: DataRequest) -> Result<DataResponse<$marker>, DataError> {
                    self.check_req::<$marker>(req)?;
                    let resource: &cldr_serde::date_fields::Resource = self
                        .cldr()?
                        .dates(None)
                        .read_and_parse(req.id.locale, "dateFields.json")?;
                    let fields = &resource.main.value.dates.fields;

                    let field = marker_filters()
                        .get(&$marker::INFO)
                        .ok_or(DataErrorKind::MarkerNotFound.into_error())?;

                    let data = fields.0.get(*field).ok_or(DataError::custom(
                        "Field not found in relative time format data.",
                    ))?;

                    Ok(DataResponse {
                        metadata: Default::default(),
                        payload: DataPayload::from_owned(RelativeTimePatternData {
                            relatives: data.relatives.iter().map(|r| (&r.count, r.pattern.as_ref())).collect(),
                            past: (&data.past).into(),
                            future: (&data.future).into(),
                        }),
                    })
                }
            }

            impl IterableDataProviderCached<$marker> for SourceDataProvider {
                fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
                    Ok(self
                        .cldr()?
                        .dates(None)
                        .list_locales()?
                        .map(DataIdentifierCow::from_locale)
                        .collect())
                }
            }
        )+
    };
}

impl From<&cldr_serde::date_fields::PluralRulesPattern>
    for PluralElementsPackedCow<'_, SinglePlaceholderPattern>
{
    fn from(field: &cldr_serde::date_fields::PluralRulesPattern) -> Self {
        PluralElements::new(&*field.other)
            .with_zero_value(field.zero.as_deref())
            .with_one_value(field.one.as_deref())
            .with_two_value(field.two.as_deref())
            .with_few_value(field.few.as_deref())
            .with_many_value(field.many.as_deref())
            .with_explicit_one_value(field.explicit_one.as_deref())
            .with_explicit_zero_value(field.explicit_zero.as_deref())
            .into()
    }
}

make_data_provider!(
    DatetimeRelativeSecondLongV1,
    DatetimeRelativeSecondShortV1,
    DatetimeRelativeSecondNarrowV1,
    DatetimeRelativeMinuteLongV1,
    DatetimeRelativeMinuteShortV1,
    DatetimeRelativeMinuteNarrowV1,
    DatetimeRelativeHourLongV1,
    DatetimeRelativeHourShortV1,
    DatetimeRelativeHourNarrowV1,
    DatetimeRelativeDayLongV1,
    DatetimeRelativeDayShortV1,
    DatetimeRelativeDayNarrowV1,
    DatetimeRelativeWeekLongV1,
    DatetimeRelativeWeekShortV1,
    DatetimeRelativeWeekNarrowV1,
    DatetimeRelativeMonthLongV1,
    DatetimeRelativeMonthShortV1,
    DatetimeRelativeMonthNarrowV1,
    DatetimeRelativeQuarterLongV1,
    DatetimeRelativeQuarterShortV1,
    DatetimeRelativeQuarterNarrowV1,
    DatetimeRelativeYearLongV1,
    DatetimeRelativeYearShortV1,
    DatetimeRelativeYearNarrowV1,
);

const WEEKDAY_ATTRIBUTES: &[(&DataMarkerAttributes, &str)] = &[
    (DataMarkerAttributes::from_str_or_panic("sunL"), "sun"),
    (DataMarkerAttributes::from_str_or_panic("sunS"), "sun-short"),
    (
        DataMarkerAttributes::from_str_or_panic("sunN"),
        "sun-narrow",
    ),
    (DataMarkerAttributes::from_str_or_panic("monL"), "mon"),
    (DataMarkerAttributes::from_str_or_panic("monS"), "mon-short"),
    (
        DataMarkerAttributes::from_str_or_panic("monN"),
        "mon-narrow",
    ),
    (DataMarkerAttributes::from_str_or_panic("tueL"), "tue"),
    (DataMarkerAttributes::from_str_or_panic("tueS"), "tue-short"),
    (
        DataMarkerAttributes::from_str_or_panic("tueN"),
        "tue-narrow",
    ),
    (DataMarkerAttributes::from_str_or_panic("wedL"), "wed"),
    (DataMarkerAttributes::from_str_or_panic("wedS"), "wed-short"),
    (
        DataMarkerAttributes::from_str_or_panic("wedN"),
        "wed-narrow",
    ),
    (DataMarkerAttributes::from_str_or_panic("thuL"), "thu"),
    (DataMarkerAttributes::from_str_or_panic("thuS"), "thu-short"),
    (
        DataMarkerAttributes::from_str_or_panic("thuN"),
        "thu-narrow",
    ),
    (DataMarkerAttributes::from_str_or_panic("friL"), "fri"),
    (DataMarkerAttributes::from_str_or_panic("friS"), "fri-short"),
    (
        DataMarkerAttributes::from_str_or_panic("friN"),
        "fri-narrow",
    ),
    (DataMarkerAttributes::from_str_or_panic("satL"), "sat"),
    (DataMarkerAttributes::from_str_or_panic("satS"), "sat-short"),
    (
        DataMarkerAttributes::from_str_or_panic("satN"),
        "sat-narrow",
    ),
];

impl DataProvider<DatetimeRelativeWeekdayV1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<DatetimeRelativeWeekdayV1>, DataError> {
        self.check_req::<DatetimeRelativeWeekdayV1>(req)?;
        let resource: &cldr_serde::date_fields::Resource = self
            .cldr()?
            .dates(None)
            .read_and_parse(req.id.locale, "dateFields.json")?;
        let fields = &resource.main.value.dates.fields;

        let &(_, field) = WEEKDAY_ATTRIBUTES
            .iter()
            .find(|(attr, _)| *attr == req.id.marker_attributes)
            .ok_or_else(|| {
                DataError::custom("Unknown marker attribute for DatetimeRelativeWeekdayV1")
                    .with_req(DatetimeRelativeWeekdayV1::INFO, req)
            })?;

        let data = fields.0.get(field).ok_or(DataError::custom(
            "Field not found in relative time format data.",
        ))?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(RelativeTimePatternData {
                relatives: data
                    .relatives
                    .iter()
                    .map(|r| (&r.count, r.pattern.as_ref()))
                    .collect(),
                past: (&data.past).into(),
                future: (&data.future).into(),
            }),
        })
    }
}

impl IterableDataProviderCached<DatetimeRelativeWeekdayV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(self
            .cldr()?
            .dates(None)
            .list_locales()?
            .flat_map(|locale| {
                WEEKDAY_ATTRIBUTES.iter().map(move |&(attr, _)| {
                    DataIdentifierCow::from_borrowed_and_owned(attr, locale.clone())
                })
            })
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use icu::locale::{data_locale, locale};
    use icu::plurals::PluralRules;
    use writeable::assert_writeable_eq;

    #[test]
    fn test_basic() {
        let provider = SourceDataProvider::new_testing();
        let data: DataPayload<DatetimeRelativeQuarterShortV1> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&data_locale!("en")),
                ..Default::default()
            })
            .unwrap()
            .payload;
        let rules =
            PluralRules::try_new_cardinal_unstable(&provider, locale!("en").into()).unwrap();
        assert_eq!(data.get().relatives.get(&0).unwrap(), "this qtr.");
        assert_writeable_eq!(
            data.get().past.get(1.into(), &rules).interpolate([1]),
            "1 qtr. ago"
        );
        assert_writeable_eq!(
            data.get().past.get(2.into(), &rules).interpolate([2]),
            "2 qtrs. ago"
        );
        assert_writeable_eq!(
            data.get().future.get(1.into(), &rules).interpolate([1]),
            "in 1 qtr."
        );
    }

    #[test]
    fn test_singular_sub_pattern() {
        let provider = SourceDataProvider::new_testing();
        let data: DataPayload<DatetimeRelativeYearLongV1> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&data_locale!("ar")),
                ..Default::default()
            })
            .unwrap()
            .payload;
        let rules =
            PluralRules::try_new_cardinal_unstable(&provider, locale!("ar").into()).unwrap();
        assert_eq!(data.get().relatives.get(&-1).unwrap(), "السنة الماضية");

        // past.one, future.two are without a placeholder.
        assert_writeable_eq!(
            data.get().past.get(1.into(), &rules).interpolate([1]),
            "قبل سنة واحدة"
        );
        assert_writeable_eq!(
            data.get().future.get(2.into(), &rules).interpolate([2]),
            "خلال سنتين"
        );

        assert_writeable_eq!(
            data.get().past.get(15.into(), &rules).interpolate([15]),
            "قبل 15 سنة"
        );
        assert_writeable_eq!(
            data.get().future.get(100.into(), &rules).interpolate([100]),
            "خلال 100 سنة"
        );
    }

    #[test]
    fn test_weekday() {
        let provider = SourceDataProvider::new_testing();
        let data: DataPayload<DatetimeRelativeWeekdayV1> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    DataMarkerAttributes::from_str_or_panic("monL"),
                    &data_locale!("en"),
                ),
                ..Default::default()
            })
            .unwrap()
            .payload;
        let rules =
            PluralRules::try_new_cardinal_unstable(&provider, locale!("en").into()).unwrap();
        assert_eq!(data.get().relatives.get(&-1).unwrap(), "last Monday");
        assert_eq!(data.get().relatives.get(&0).unwrap(), "this Monday");
        assert_eq!(data.get().relatives.get(&1).unwrap(), "next Monday");
        assert_writeable_eq!(
            data.get().past.get(2.into(), &rules).interpolate([2]),
            "2 Mondays ago"
        );
        assert_writeable_eq!(
            data.get().future.get(2.into(), &rules).interpolate([2]),
            "in 2 Mondays"
        );
    }
}
