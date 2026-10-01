// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::FormattedDateTime;
use crate::options::Alignment;
use crate::parts;
use crate::provider::fields;
use crate::provider::pattern::PatternItem;
use crate::provider::pattern::runtime;
use crate::provider::range_patterns::RangePatternInfoBorrowed;
use crate::provider::semantic_skeletons::GluePattern;
use core::fmt;
use writeable::adapters::WithPart;
use writeable::{PartsWrite, Writeable};
use zerovec::ule::AsULE;

/// The formatting result of a date/time range.
#[derive(Debug)]
pub struct FormattedDateRange<'l>(pub(crate) FormattedDateRangeInner<'l>);

#[derive(Debug)]
#[allow(clippy::large_enum_variant)] // Short-lived formatting result, avoid heap allocation
pub(crate) enum FormattedDateRangeInner<'l> {
    /// The range resolved to a single date/time (start and end are equal).
    Single(FormattedDateTime<'l>),
    /// The range is formatted using a greatest difference pattern.
    GreatestDifference(FormattedGreatestDifference<'l>),
    /// The range has a single date but a range of times.
    TimeRangeMixed(FormattedTimeRangeMixed<'l>),
    /// The range is formatted using the fallback range pattern.
    Fallback(FormattedRangeFallback<'l>),
}

impl Writeable for FormattedDateRange<'_> {
    fn write_to_parts<S: PartsWrite + ?Sized>(&self, sink: &mut S) -> Result<(), fmt::Error> {
        match &self.0 {
            FormattedDateRangeInner::Single(x) => x.write_to_parts(sink),
            FormattedDateRangeInner::GreatestDifference(x) => x.write_to_parts(sink),
            FormattedDateRangeInner::TimeRangeMixed(x) => x.write_to_parts(sink),
            FormattedDateRangeInner::Fallback(x) => x.write_to_parts(sink),
        }
    }

    fn writeable_length_hint(&self) -> writeable::LengthHint {
        match &self.0 {
            FormattedDateRangeInner::Single(x) => x.writeable_length_hint(),
            FormattedDateRangeInner::GreatestDifference(x) => x.writeable_length_hint(),
            FormattedDateRangeInner::TimeRangeMixed(x) => x.writeable_length_hint(),
            FormattedDateRangeInner::Fallback(x) => x.writeable_length_hint(),
        }
    }
}

writeable::impl_display_with_writeable!(FormattedDateRange<'_>);

/// The formatting result of a date/time range where the start and end dates
/// differ by a specific field (e.g., day, month, year) and are formatted
/// using a greatest-difference pattern.
#[derive(Debug)]
pub(crate) struct FormattedGreatestDifference<'l> {
    pub(crate) start: FormattedDateTime<'l>,
    pub(crate) end: FormattedDateTime<'l>,
    pub(crate) pattern_info: RangePatternInfoBorrowed<'l>,
    pub(crate) glue: &'l GluePattern<'l>,
    pub(crate) alignment: Option<Alignment>,
}

impl Writeable for FormattedGreatestDifference<'_> {
    fn write_to_parts<S: PartsWrite + ?Sized>(&self, sink: &mut S) -> Result<(), fmt::Error> {
        match &self.pattern_info {
            RangePatternInfoBorrowed::FullRange(pattern) => {
                let (start_pattern, end_pattern) = pattern.split_on_repeated_field();
                let start_side = WithPart {
                    part: parts::RANGE_START,
                    writeable: FormattedSingleSide {
                        datetime: &self.start,
                        pattern: start_pattern,
                        alignment: self.alignment,
                    },
                };
                let end_side = WithPart {
                    part: parts::RANGE_END,
                    writeable: FormattedSingleSide {
                        datetime: &self.end,
                        pattern: end_pattern,
                        alignment: self.alignment,
                    },
                };
                start_side.write_to_parts(sink)?;
                end_side.write_to_parts(sink)
            }
            RangePatternInfoBorrowed::Symmetric(pattern) => {
                let start_side = WithPart {
                    part: parts::RANGE_START,
                    writeable: FormattedSingleSide {
                        datetime: &self.start,
                        pattern: *pattern,
                        alignment: self.alignment,
                    },
                };
                let end_side = WithPart {
                    part: parts::RANGE_END,
                    writeable: FormattedSingleSide {
                        datetime: &self.end,
                        pattern: *pattern,
                        alignment: self.alignment,
                    },
                };
                write_glue_pattern(sink, self.glue, &start_side, &end_side)
            }
        }
    }
}

#[derive(Debug)]
struct FormattedSingleSide<'a, 'l> {
    datetime: &'a FormattedDateTime<'l>,
    pattern: runtime::PatternBorrowed<'l>,
    alignment: Option<Alignment>,
}

impl Writeable for FormattedSingleSide<'_, '_> {
    fn write_to_parts<S: PartsWrite + ?Sized>(&self, sink: &mut S) -> Result<(), fmt::Error> {
        let metadata = self.pattern.metadata;
        let decimal_formatter = self.datetime.names.decimal_formatter;

        for item in self.pattern.items.iter() {
            match item {
                PatternItem::Literal(ch) => sink.write_char(ch)?,
                PatternItem::Field(field) => {
                    let mut field = field;
                    let alignment = self.alignment.unwrap_or_default();
                    if matches!(alignment, Alignment::Column)
                        && field.length == fields::FieldLength::One
                        && matches!(
                            field.symbol,
                            fields::FieldSymbol::Month(_)
                                | fields::FieldSymbol::Day(_)
                                | fields::FieldSymbol::Week(_)
                                | fields::FieldSymbol::Hour(_)
                        )
                    {
                        field.length = fields::FieldLength::Two;
                    }

                    let field = self.datetime.names.resolve_field_with_loaded_names(field);

                    let r = crate::format::datetime::try_write_field(
                        field,
                        metadata,
                        &self.datetime.input,
                        &self.datetime.names,
                        decimal_formatter,
                        sink,
                    );

                    match r {
                        Ok(Ok(())) => {}
                        Err(fmt::Error) => return Err(fmt::Error),
                        Ok(Err(e)) => {
                            debug_assert!(false, "unexpected error in FormattedSingleSide: {e:?}");
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub(crate) struct FormattedTimeRangeMixed<'l> {
    pub(crate) date: FormattedDateTime<'l>,
    pub(crate) time_range: FormattedGreatestDifference<'l>,
    pub(crate) glue: &'l GluePattern<'l>,
}

impl Writeable for FormattedTimeRangeMixed<'_> {
    fn write_to_parts<S: PartsWrite + ?Sized>(&self, sink: &mut S) -> Result<(), fmt::Error> {
        write_glue_pattern(sink, self.glue, &self.time_range, &self.date)
    }
}

#[derive(Debug)]
pub(crate) struct FormattedRangeFallback<'l> {
    pub(crate) start: FormattedDateTime<'l>,
    pub(crate) end: FormattedDateTime<'l>,
    pub(crate) glue: &'l GluePattern<'l>,
}

impl Writeable for FormattedRangeFallback<'_> {
    fn write_to_parts<S: PartsWrite + ?Sized>(&self, sink: &mut S) -> Result<(), fmt::Error> {
        write_glue_pattern(
            sink,
            self.glue,
            &WithPart {
                part: parts::RANGE_START,
                writeable: &self.start,
            },
            &WithPart {
                part: parts::RANGE_END,
                writeable: &self.end,
            },
        )
    }
}

pub(crate) fn write_glue_pattern<W, S, E>(
    sink: &mut W,
    glue: &GluePattern<'_>,
    start: &S,
    end: &E,
) -> fmt::Result
where
    W: PartsWrite + ?Sized,
    S: Writeable + ?Sized,
    E: Writeable + ?Sized,
{
    for generic_item_ule in glue.pattern.items.as_ule_slice().iter() {
        match generic_item_ule.as_pattern_item_ule() {
            Ok(pattern_item_ule) => {
                let pattern_item = <PatternItem as AsULE>::from_unaligned(*pattern_item_ule);
                if let PatternItem::Literal(ch) = pattern_item {
                    ch.write_to_parts(sink)?;
                } else {
                    debug_assert!(false, "Expected only literals in glue pattern");
                }
            }
            Err(0) => {
                start.write_to_parts(sink)?;
            }
            Err(1) => {
                end.write_to_parts(sink)?;
            }
            Err(_) => {
                debug_assert!(false, "Unexpected placeholder index in glue pattern");
            }
        }
    }
    Ok(())
}

#[cfg(all(test, feature = "compiled_data"))]
mod tests {
    use super::*;
    use crate::fieldsets::enums::CompositeDateTimeFieldSet;
    use crate::input::{Date, DateTime, Time};
    use crate::pattern::{DateTimePattern, FixedCalendarDateTimeNames};
    use crate::unchecked::DateTimeInputUnchecked;
    use icu_calendar::Gregorian;
    use icu_locale_core::locale;
    use writeable::Writeable;

    #[test]
    fn test_runtime_fallback_to_loaded_names() {
        let mut names: FixedCalendarDateTimeNames<Gregorian, CompositeDateTimeFieldSet> =
            FixedCalendarDateTimeNames::try_new(locale!("en").into()).unwrap();
        // Load abbreviated text month (`MMM`), weekday (`EEE`), era (`G`), and day period (`a`).
        let loaded_pattern: DateTimePattern = "EEE, MMM d, y G h:mm a".parse().unwrap();
        names.include_for_pattern(&loaded_pattern).unwrap();

        let dt = DateTime {
            date: Date::try_new_gregorian(2024, 11, 22).unwrap(),
            time: Time::try_new(14, 30, 0, 0).unwrap(),
        };
        let mut input = DateTimeInputUnchecked::default();
        input.set_date_fields_unchecked(dt.date);
        input.set_time_fields(dt.time);

        let formatted_dt = FormattedDateTime {
            pattern: crate::raw::neo::DateTimeZonePatternDataBorrowed {
                date: None,
                time: None,
                zone: None,
                glue: None,
            },
            input,
            names: names.inner.as_borrowed(),
        };

        // Even if the range pattern requests wide standalone/format symbols (`cccc`, `LLLL`, `GGGG`, `aaaa`),
        // `FormattedSingleSide` falls back to the loaded symbols (`EEE`, `MMM`, `G`, `a`).
        let mismatched_range_pattern: DateTimePattern =
            "cccc, LLLL d, y GGGG h:mm aaaa".parse().unwrap();
        let side = FormattedSingleSide {
            datetime: &formatted_dt,
            pattern: mismatched_range_pattern.as_borrowed().0.as_borrowed(),
            alignment: None,
        };
        assert_eq!(side.write_to_string(), "Fri, Nov 22, 2024 AD 2:30 PM");

        // Verify numeric month context fallback (`L` -> loaded `M` while preserving 2-digit `LL` length).
        let mut num_names: FixedCalendarDateTimeNames<Gregorian, CompositeDateTimeFieldSet> =
            FixedCalendarDateTimeNames::try_new(locale!("en").into()).unwrap();
        let loaded_num_pattern: DateTimePattern = "y-M-d".parse().unwrap();
        num_names.include_for_pattern(&loaded_num_pattern).unwrap();
        let formatted_num_dt = FormattedDateTime {
            pattern: crate::raw::neo::DateTimeZonePatternDataBorrowed {
                date: None,
                time: None,
                zone: None,
                glue: None,
            },
            input,
            names: num_names.inner.as_borrowed(),
        };
        let mismatched_num_range_pattern: DateTimePattern = "y-LL-dd".parse().unwrap();
        let num_side = FormattedSingleSide {
            datetime: &formatted_num_dt,
            pattern: mismatched_num_range_pattern.as_borrowed().0.as_borrowed(),
            alignment: None,
        };
        assert_eq!(num_side.write_to_string(), "2024-11-22");
    }
}
