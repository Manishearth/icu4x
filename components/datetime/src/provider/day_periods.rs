// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data structs and markers for day periods rules.

use icu_provider::prelude::*;

/// Day period rules representing the active period for each hour of the day.
///
/// Lookup assumes that at hour 0, the active period is the *last* present period in the
/// sequence of present periods (sorted by enum value). This is because night periods
/// often spill over past midnight (e.g., from 21:00 to 06:00), meaning hour 0 is
/// typically covered by the last chronological period, which is usually a night period.
/// Transitions move to the next present period in the sequence, wrapping around.
#[derive(Debug, PartialEq, Clone, Copy, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_datetime::provider::day_periods))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct DayPeriodRules {
    /// A bitmask of present day periods. Bit `i` is set if the period with
    /// enum value `i` is present in the locale's rules.
    pub presence: u8,
    /// A 24-bit map (packed into 3 bytes) where bit `h` is set if a transition
    /// to the next present period occurs at hour `h`.
    pub transitions: [u8; 3],
}

icu_provider::data_struct!(
    DayPeriodRules,
    #[cfg(feature = "datagen")]
);

icu_provider::data_marker!(
    /// `DayPeriodRulesV1` marker
    DayPeriodRulesV1,
    DayPeriodRules,
);

/// Day periods.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_datetime::provider::day_periods))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub enum DayPeriod {
    /// Morning 1
    Morning1 = 0,
    /// Morning 2
    Morning2 = 1,
    /// Afternoon 1
    Afternoon1 = 2,
    /// Afternoon 2
    Afternoon2 = 3,
    /// Evening 1
    Evening1 = 4,
    /// Evening 2
    Evening2 = 5,
    /// Night 1
    Night1 = 6,
    /// Night 2
    Night2 = 7,
}

impl DayPeriod {
    /// Parses a CLDR day period name into a `DayPeriod`.
    pub fn from_cldr_name(name: &str) -> Option<Self> {
        match name {
            "morning1" => Some(DayPeriod::Morning1),
            "morning2" => Some(DayPeriod::Morning2),
            "afternoon1" => Some(DayPeriod::Afternoon1),
            "afternoon2" => Some(DayPeriod::Afternoon2),
            "evening1" => Some(DayPeriod::Evening1),
            "evening2" => Some(DayPeriod::Evening2),
            "night1" => Some(DayPeriod::Night1),
            "night2" => Some(DayPeriod::Night2),
            _ => None,
        }
    }

    /// Converts a u8 index to a `DayPeriod` enum.
    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(DayPeriod::Morning1),
            1 => Some(DayPeriod::Morning2),
            2 => Some(DayPeriod::Afternoon1),
            3 => Some(DayPeriod::Afternoon2),
            4 => Some(DayPeriod::Evening1),
            5 => Some(DayPeriod::Evening2),
            6 => Some(DayPeriod::Night1),
            7 => Some(DayPeriod::Night2),
            _ => None,
        }
    }
}

impl DayPeriodRules {
    /// Looks up the day period for a given hour (0-24).
    pub fn lookup(&self, hour: u8) -> DayPeriod {
        // Make the lookup cyclic and handle hour 24 (as hour 0 of next day).
        let hour = hour % 24;

        // GIGO: If presence is 0 (bad data), we return Morning1 as a safe fallback.
        if self.presence == 0 {
            debug_assert!(false, "presence must not be zero");
            return DayPeriod::Morning1;
        }

        let count = self.presence.count_ones() as usize;

        // Combine transitions bytes into a u32 for easier bit manipulation.
        let transitions_u32 = u32::from_le_bytes([
            self.transitions[0],
            self.transitions[1],
            self.transitions[2],
            0,
        ]);

        // Create a mask for bits 0..=hour.
        // For hour 0, mask is 0x1 (bit 0).
        // For hour 23, mask is 0x00FF_FFFF (bits 0..23).
        // Since hour < 24, (hour + 1) <= 24, so 1u32 << (hour + 1) fits in u32 without overflow.
        let mask = (1u32 << (hour + 1)) - 1;
        // Count how many transitions occurred up to `hour`.
        let transitions = (transitions_u32 & mask).count_ones() as usize;

        // Assume first period (at hour 0) is the last present period in the sorted sequence.
        // Adding `transitions` moves us forward in the sequence.
        // We subtract 1 and modulo `count` to start from the last period (index count - 1)
        // and wrap around correctly.
        let which_period = (transitions + count - 1) % count;

        // Find the which_period-th set bit in presence (0-indexed).
        let mut period_idx = 0u8;
        let mut found_count = 0;
        for i in 0..8 {
            if (self.presence & (1 << i)) != 0 {
                if found_count == which_period {
                    period_idx = i;
                    break;
                }
                found_count += 1;
            }
        }

        if let Some(period) = DayPeriod::from_u8(period_idx) {
            period
        } else {
            // This should be unreachable by construction as which_period < count.
            debug_assert!(false, "Unreachable day period index: {}", period_idx);
            DayPeriod::Morning1 // Fallback
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lookup() {
        // Create a dummy DayPeriodRules
        // presence: Morning1 (0), Afternoon1 (2), Night1 (6)
        // presence = (1<<0) | (1<<2) | (1<<6) = 1 | 4 | 64 = 69
        // transitions: transitions at 6 (Morning1), 12 (Afternoon1), 18 (Night1)
        // 6 is bit 6 of byte 0.
        // 12 is bit 4 of byte 1.
        // 18 is bit 2 of byte 2.

        let mut transitions = [0u8; 3];
        transitions[0] |= 1 << 6;
        transitions[1] |= 1 << 4;
        transitions[2] |= 1 << 2;

        let rules = DayPeriodRules {
            presence: 69,
            transitions,
        };

        // Assume first period is Night1 (6) because it's the last night period.
        // At hour 0, no transition, so it should be Night1.
        assert_eq!(rules.lookup(0), DayPeriod::Night1);
        assert_eq!(rules.lookup(5), DayPeriod::Night1);

        // At hour 6, transition to Morning1 (0).
        assert_eq!(rules.lookup(6), DayPeriod::Morning1);
        assert_eq!(rules.lookup(11), DayPeriod::Morning1);

        // At hour 12, transition to Afternoon1 (2).
        assert_eq!(rules.lookup(12), DayPeriod::Afternoon1);
        assert_eq!(rules.lookup(17), DayPeriod::Afternoon1);

        // At hour 18, transition to Night1 (6).
        assert_eq!(rules.lookup(18), DayPeriod::Night1);
        assert_eq!(rules.lookup(23), DayPeriod::Night1);

        // Hour 24 should be same as hour 0.
        assert_eq!(rules.lookup(24), DayPeriod::Night1);
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn test_compiled_data() {
        use icu_provider::prelude::*;
        let provider = crate::provider::Baked;
        let rules: DataPayload<DayPeriodRulesV1> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&icu_locale::langid!("en").into()),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(rules.get().lookup(0), DayPeriod::Morning1);
        assert_eq!(rules.get().lookup(11), DayPeriod::Morning1);
        assert_eq!(rules.get().lookup(12), DayPeriod::Afternoon1);
        assert_eq!(rules.get().lookup(17), DayPeriod::Afternoon1);
        assert_eq!(rules.get().lookup(18), DayPeriod::Evening1);
        assert_eq!(rules.get().lookup(20), DayPeriod::Evening1);
        assert_eq!(rules.get().lookup(21), DayPeriod::Night1);
        assert_eq!(rules.get().lookup(23), DayPeriod::Night1);

        // Test 'zh' (Chinese) rules
        let rules_zh: DataPayload<DayPeriodRulesV1> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&icu_locale::langid!("zh").into()),
                ..Default::default()
            })
            .unwrap()
            .payload;
        assert_eq!(rules_zh.get().lookup(0), DayPeriod::Night1);
        assert_eq!(rules_zh.get().lookup(4), DayPeriod::Night1);
        assert_eq!(rules_zh.get().lookup(5), DayPeriod::Morning1);
        assert_eq!(rules_zh.get().lookup(7), DayPeriod::Morning1);
        assert_eq!(rules_zh.get().lookup(8), DayPeriod::Morning2);
        assert_eq!(rules_zh.get().lookup(11), DayPeriod::Morning2);
        assert_eq!(rules_zh.get().lookup(12), DayPeriod::Afternoon1);
        assert_eq!(rules_zh.get().lookup(13), DayPeriod::Afternoon2);
        assert_eq!(rules_zh.get().lookup(18), DayPeriod::Afternoon2);
        assert_eq!(rules_zh.get().lookup(19), DayPeriod::Evening1);
        assert_eq!(rules_zh.get().lookup(23), DayPeriod::Evening1);

        // Test 'de' (German) rules
        let rules_de: DataPayload<DayPeriodRulesV1> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&icu_locale::langid!("de").into()),
                ..Default::default()
            })
            .unwrap()
            .payload;
        assert_eq!(rules_de.get().lookup(0), DayPeriod::Night1);
        assert_eq!(rules_de.get().lookup(4), DayPeriod::Night1);
        assert_eq!(rules_de.get().lookup(5), DayPeriod::Morning1);
        assert_eq!(rules_de.get().lookup(9), DayPeriod::Morning1);
        assert_eq!(rules_de.get().lookup(10), DayPeriod::Morning2);
        assert_eq!(rules_de.get().lookup(11), DayPeriod::Morning2);
        assert_eq!(rules_de.get().lookup(12), DayPeriod::Afternoon1);
        assert_eq!(rules_de.get().lookup(13), DayPeriod::Afternoon2);
        assert_eq!(rules_de.get().lookup(17), DayPeriod::Afternoon2);
        assert_eq!(rules_de.get().lookup(18), DayPeriod::Evening1);
        assert_eq!(rules_de.get().lookup(23), DayPeriod::Evening1);
    }
}
