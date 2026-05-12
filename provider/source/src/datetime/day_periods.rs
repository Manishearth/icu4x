// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::IterableDataProviderCached;
use crate::SourceDataProvider;
use icu::datetime::provider::day_periods::*;
use icu_provider::prelude::*;
use std::collections::HashSet;

impl DataProvider<DayPeriodRulesV1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<DayPeriodRulesV1>, DataError> {
        self.check_req::<DayPeriodRulesV1>(req)?;

        let day_periods: &cldr_serde::day_periods::Resource = self
            .cldr()?
            .core()
            .read_and_parse("supplemental/dayPeriods.json")?;

        let langid = icu::locale::LanguageIdentifier::from((
            req.id.locale.language,
            req.id.locale.script,
            req.id.locale.region,
        ));

        let rules = day_periods
            .supplemental
            .day_period_rule_set
            .0
            .get(&langid.to_string())
            .ok_or_else(|| {
                DataErrorKind::IdentifierNotFound
                    .with_req(<DayPeriodRulesV1 as DataMarker>::INFO, req)
            })?;

        let data = compute_day_periods(rules, &req.id.locale.to_string()).ok_or_else(|| {
            DataErrorKind::IdentifierNotFound.with_req(<DayPeriodRulesV1 as DataMarker>::INFO, req)
        })?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(data),
        })
    }
}

/// Computes `DayPeriodRules` from CLDR supplemental day period rules.
///
/// Returns `None` if the rules are empty or do not contain any flexible day periods.
pub(crate) fn compute_day_periods(
    rules: &std::collections::BTreeMap<String, cldr_serde::day_periods::DayPeriodRule>,
    locale_str: &str,
) -> Option<DayPeriodRules> {
    if rules.is_empty() {
        return None;
    }

    let mut transitions_u32 = 0u32;

    // Entries stores parsed CLDR rules as (period_idx, start_hour, end_hour) tuples.
    let mut entries = Vec::new();

    for (period, rule) in rules {
        if rule.at.is_some() {
            assert!(
                period == "noon" || period == "midnight",
                "Found 'at' rule for non-noon/midnight period: {} in locale {}",
                period,
                locale_str
            );
        }
        if let Some(period_enum) = DayPeriod::from_cldr_name(period) {
            let idx = period_enum as u8;
            if let (Some(from), Some(before)) = (&rule.from, &rule.before) {
                let start = parse_hour(from);
                let end = parse_hour(before);
                entries.push((idx, start, end));
            }
        }
    }

    // Sort by start time so we can iterate chronologically to determine active periods.
    entries.sort_by_key(|e| e.1);

    if entries.is_empty() {
        return None;
    }

    // Compute presence bitmask from entries.
    let mut presence = 0u8;
    for entry in &entries {
        presence |= 1 << entry.0;
    }

    // Find the initial active period at hour 0.
    // By definition, this is the last present period in the sorted sequence of present periods,
    // which corresponds to the maximum period index present in entries.
    let mut current_period = entries.iter().map(|e| e.0).max().unwrap();

    // Determine the active period index (0-7) at each hour (0-23).
    let hour_periods = compute_hour_periods(&entries);

    // Compute transitions by detecting transitions between expected and actual periods.
    for h in 0..24 {
        let expected_period = current_period;
        let actual_period = hour_periods[h as usize];

        if actual_period != expected_period {
            // A transition occurred at hour h.
            transitions_u32 |= 1 << h;

            // Advance to the next present period in the sequence.
            current_period = next_present_period(presence, current_period);

            // If it still doesn't match, resync to the actual period.
            if actual_period != current_period {
                current_period = actual_period;
            }
        }
    }

    let bytes = transitions_u32.to_le_bytes();
    let transitions = [bytes[0], bytes[1], bytes[2]];

    Some(DayPeriodRules {
        presence,
        transitions,
    })
}

impl IterableDataProviderCached<DayPeriodRulesV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        let day_periods: &cldr_serde::day_periods::Resource = self
            .cldr()?
            .core()
            .read_and_parse("supplemental/dayPeriods.json")?;
        Ok(day_periods
            .supplemental
            .day_period_rule_set
            .0
            .iter()
            .filter_map(|(l, rules)| {
                let langid: icu::locale::LanguageIdentifier = l.parse().unwrap();
                let has_flexible = rules.keys().any(|k| DayPeriod::from_cldr_name(k).is_some());
                if has_flexible {
                    Some(DataIdentifierCow::from_locale(DataLocale::from(langid)))
                } else {
                    None
                }
            })
            .collect())
    }
}

/// Parses a "HH:MM" time string and returns the hour as a u8.
/// Logs a warning if the minute value is non-zero, as precision will be lost.
fn parse_hour(s: &str) -> u8 {
    let mut parts = s.split(':');
    let hour = parts.next().unwrap().parse().unwrap();
    if let Some(min_str) = parts.next() {
        let min: u32 = min_str.parse().unwrap();
        if min != 0 {
            log::warn!(
                "Non-zero minute found in day period time: {}, precision will be lost",
                s
            );
        }
    }
    hour
}

/// Finds the next present period index after `current` (wrapping around) in the presence bitmask.
/// Assumes presence is non-zero.
fn next_present_period(presence: u8, current: u8) -> u8 {
    let mut next = current + 1;
    loop {
        next %= 8;
        if (presence & (1 << next)) != 0 {
            return next;
        }
        next += 1;
    }
}

/// Computes the active period index (0-7) for each of the 24 hours.
fn compute_hour_periods(entries: &[(u8, u8, u8)]) -> [u8; 24] {
    let mut hour_periods = [0u8; 24];
    for entry in entries {
        let start = entry.1;
        let end = entry.2;
        let mut h = start;
        loop {
            hour_periods[h as usize] = entry.0;
            h = (h + 1) % 24;
            if h == end || (h == 0 && end == 24) {
                break;
            }
        }
    }
    hour_periods
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cldr_serde::day_periods::DayPeriodRule;
    use std::collections::BTreeMap;

    #[test]
    fn test_compute_day_periods() {
        let mut rules = BTreeMap::new();

        // Simulate rules where night extends into morning:
        // morning1: 06:00 - 12:00
        // afternoon1: 12:00 - 18:00
        // evening1: 18:00 - 21:00
        // night1: 21:00 - 06:00

        rules.insert(
            "morning1".to_string(),
            DayPeriodRule {
                from: Some("06:00".to_string()),
                before: Some("12:00".to_string()),
                at: None,
            },
        );
        rules.insert(
            "afternoon1".to_string(),
            DayPeriodRule {
                from: Some("12:00".to_string()),
                before: Some("18:00".to_string()),
                at: None,
            },
        );
        rules.insert(
            "evening1".to_string(),
            DayPeriodRule {
                from: Some("18:00".to_string()),
                before: Some("21:00".to_string()),
                at: None,
            },
        );
        rules.insert(
            "night1".to_string(),
            DayPeriodRule {
                from: Some("21:00".to_string()),
                before: Some("06:00".to_string()),
                at: None,
            },
        );

        let rules = compute_day_periods(&rules, "test").unwrap();

        // Test lookup for various hours
        assert_eq!(rules.lookup(0), DayPeriod::Night1);
        assert_eq!(rules.lookup(5), DayPeriod::Night1);
        assert_eq!(rules.lookup(6), DayPeriod::Morning1);
        assert_eq!(rules.lookup(11), DayPeriod::Morning1);
        assert_eq!(rules.lookup(12), DayPeriod::Afternoon1);
        assert_eq!(rules.lookup(17), DayPeriod::Afternoon1);
        assert_eq!(rules.lookup(18), DayPeriod::Evening1);
        assert_eq!(rules.lookup(20), DayPeriod::Evening1);
        assert_eq!(rules.lookup(21), DayPeriod::Night1);
        assert_eq!(rules.lookup(23), DayPeriod::Night1);
        assert_eq!(rules.lookup(24), DayPeriod::Night1); // Should wrap to 0
    }
}
