// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::IterableDataProviderCached;
use crate::SourceDataProvider;
use icu::datetime::provider::day_periods::*;
use icu_provider::prelude::*;
use std::collections::HashSet;

fn parse_hour(s: &str) -> u8 {
    s.split(':').next().unwrap().parse().unwrap()
}

fn period_idx(name: &str) -> Option<u8> {
    match name {
        "morning1" => Some(0),
        "morning2" => Some(1),
        "afternoon1" => Some(2),
        "afternoon2" => Some(3),
        "evening1" => Some(4),
        "evening2" => Some(5),
        "night1" => Some(6),
        "night2" => Some(7),
        _ => None,
    }
}

impl DataProvider<DayPeriodRulesDesign1V1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<DayPeriodRulesDesign1V1>, DataError> {
        self.check_req::<DayPeriodRulesDesign1V1>(req)?;
        
        let day_periods: &cldr_serde::day_periods::Resource = self
            .cldr()?
            .core()
            .read_and_parse("supplemental/dayPeriods.json")?;
            
        let langid = req.id.locale.langid();
        let rules = day_periods
            .supplemental
            .day_period_rules
            .0
            .get(&langid)
            .ok_or(DataErrorKind::IdentifierNotFound.into_error())?;

        let mut entries = Vec::new();
        for (period, rule) in rules {
            if let Some(idx) = period_idx(period) {
                if let (Some(from), Some(before)) = (&rule.from, &rule.before) {
                    let start = parse_hour(from);
                    let end = parse_hour(before);
                    let val = ((idx as u16) << 10) | ((start as u16) << 5) | (end as u16);
                    entries.push(DayPeriodRule1(val));
                }
            }
        }
        // Sort by period index to be consistent
        entries.sort_by_key(|e| e.0 >> 10);

        let data = DayPeriodRulesV1Design1 {
            entries: zerovec::ZeroVec::from_vec_take_owned(entries),
        };

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(data),
        })
    }
}

impl IterableDataProviderCached<DayPeriodRulesDesign1V1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        let day_periods: &cldr_serde::day_periods::Resource = self
            .cldr()?
            .core()
            .read_and_parse("supplemental/dayPeriods.json")?;
        Ok(day_periods
            .supplemental
            .day_period_rules
            .0
            .keys()
            .map(|l| DataIdentifierCow::from_locale(DataLocale::from(l.clone())))
            .collect())
    }
}

impl DataProvider<DayPeriodRulesDesign2V1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<DayPeriodRulesDesign2V1>, DataError> {
        self.check_req::<DayPeriodRulesDesign2V1>(req)?;
        
        let day_periods: &cldr_serde::day_periods::Resource = self
            .cldr()?
            .core()
            .read_and_parse("supplemental/dayPeriods.json")?;
            
        let langid = req.id.locale.langid();
        let rules = day_periods
            .supplemental
            .day_period_rules
            .0
            .get(&langid)
            .ok_or(DataErrorKind::IdentifierNotFound.into_error())?;

        let mut entries = Vec::new();
        let mut presence = 0u8;
        for (period, rule) in rules {
            if let Some(idx) = period_idx(period) {
                if let (Some(from), Some(before)) = (&rule.from, &rule.before) {
                    let start = parse_hour(from);
                    let end = parse_hour(before);
                    let delta = if end >= start { end - start } else { end + 24 - start };
                    let val = ((start as u8) << 3) | ((delta - 1) as u8); // assuming delta >= 1
                    entries.push((idx, DayPeriodRule2(val)));
                    presence |= 1 << idx;
                }
            }
        }
        // Sort by period index to match presence bitmask
        entries.sort_by_key(|e| e.0);
        let entries = entries.into_iter().map(|e| e.1).collect::<Vec<_>>();

        let data = DayPeriodRulesV1Design2 {
            presence,
            entries: zerovec::ZeroVec::from_vec_take_owned(entries),
        };

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(data),
        })
    }
}

impl IterableDataProviderCached<DayPeriodRulesDesign2V1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        let day_periods: &cldr_serde::day_periods::Resource = self
            .cldr()?
            .core()
            .read_and_parse("supplemental/dayPeriods.json")?;
        Ok(day_periods
            .supplemental
            .day_period_rules
            .0
            .keys()
            .map(|l| DataIdentifierCow::from_locale(DataLocale::from(l.clone())))
            .collect())
    }
}

impl DataProvider<DayPeriodRulesDesign3V1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<DayPeriodRulesDesign3V1>, DataError> {
        self.check_req::<DayPeriodRulesDesign3V1>(req)?;
        
        let day_periods: &cldr_serde::day_periods::Resource = self
            .cldr()?
            .core()
            .read_and_parse("supplemental/dayPeriods.json")?;
            
        let langid = req.id.locale.langid();
        let rules = day_periods
            .supplemental
            .day_period_rules
            .0
            .get(&langid)
            .ok_or(DataErrorKind::IdentifierNotFound.into_error())?;

        let mut entries = Vec::new();
        let mut presence = 0u8;
        for (period, rule) in rules {
            if let Some(idx) = period_idx(period) {
                if let (Some(from), Some(before)) = (&rule.from, &rule.before) {
                    let start = parse_hour(from);
                    let end = parse_hour(before);
                    entries.push((idx, start, end));
                    presence |= 1 << idx;
                }
            }
        }
        // Sort by start time to construct diffs
        entries.sort_by_key(|e| e.1);

        let mut diffs = Vec::new();
        let mut start_time = 0;
        if let Some(first) = entries.first() {
            start_time = first.1;
            let mut last_end = first.2;
            for entry in entries.iter().skip(1) {
                let diff = if entry.1 >= last_end { entry.1 - last_end } else { entry.1 + 24 - last_end };
                diffs.push(diff);
                last_end = entry.2;
            }
        }

        let data = DayPeriodRulesV1Design3 {
            presence,
            start_time,
            diffs: zerovec::ZeroVec::from_vec_take_owned(diffs),
        };

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(data),
        })
    }
}

impl IterableDataProviderCached<DayPeriodRulesDesign3V1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        let day_periods: &cldr_serde::day_periods::Resource = self
            .cldr()?
            .core()
            .read_and_parse("supplemental/dayPeriods.json")?;
        Ok(day_periods
            .supplemental
            .day_period_rules
            .0
            .keys()
            .map(|l| DataIdentifierCow::from_locale(DataLocale::from(l.clone())))
            .collect())
    }
}
