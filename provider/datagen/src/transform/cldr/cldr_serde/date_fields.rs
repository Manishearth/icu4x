// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON dateFields.json files.
//!
//! Sample file:
//! <https://github.com/unicode-org/cldr-json/blob/main/cldr-json/cldr-dates-full/main/en/dateFields.json>

use std::collections::HashMap;

use serde::{
    de::{Error, IgnoredAny, Visitor},
    Deserialize,
};

#[derive(Debug, Deserialize, Default)]
pub(in crate::provider) struct PluralRulesPattern {
    #[serde(rename = "relativeTimePattern-count-zero")]
    pub(in crate::provider) zero: Option<String>,
    #[serde(rename = "relativeTimePattern-count-one")]
    pub(in crate::provider) one: Option<String>,
    #[serde(rename = "relativeTimePattern-count-two")]
    pub(in crate::provider) two: Option<String>,
    #[serde(rename = "relativeTimePattern-count-few")]
    pub(in crate::provider) few: Option<String>,
    #[serde(rename = "relativeTimePattern-count-many")]
    pub(in crate::provider) many: Option<String>,
    #[serde(rename = "relativeTimePattern-count-other")]
    pub(in crate::provider) other: String,
}

#[derive(Debug)]
pub(in crate::provider) struct Relative {
    pub(in crate::provider) count: i8,
    pub(in crate::provider) pattern: String,
}

#[derive(Debug)]
pub(in crate::provider) struct Field {
    pub(in crate::provider) _display_name: String,
    pub(in crate::provider) _relative_period: Option<String>,
    pub(in crate::provider) relatives: Vec<Relative>,
    pub(in crate::provider) past: PluralRulesPattern,
    pub(in crate::provider) future: PluralRulesPattern,
}

pub(in crate::provider) struct FieldVisitor;

impl<'de> Visitor<'de> for FieldVisitor {
    type Value = Field;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "field data")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut display_name = String::new();
        let mut relative_period = None;
        let mut relatives = Vec::new();
        let mut past = None;
        let mut future = None;
        if let Some(size_hint) = map.size_hint() {
            relatives.reserve(size_hint);
        }
        while let Some(key) = map.next_key::<String>()? {
            // Keys must be either "displayName",  "relativePeriod", "relativeTime-type-past", "relativeTime-Type-future", "relative-type-{type}"
            if key == "displayName" {
                display_name = map.next_value::<String>()?;
            } else if key == "relativePeriod" {
                relative_period = Some(map.next_value::<String>()?);
            } else if key == "relativeTime-type-past" {
                if past.is_some() {
                    return Err(A::Error::duplicate_field(
                        r#"encountered duplicate key "relativeTime-type-past""#,
                    ));
                }
                let pattern: PluralRulesPattern = map.next_value()?;
                past = Some(pattern);
            } else if key == "relativeTime-type-future" {
                if future.is_some() {
                    return Err(A::Error::duplicate_field(
                        r#"encountered duplicate key "relativeTime-type-future""#,
                    ));
                }
                let pattern: PluralRulesPattern = map.next_value()?;
                future = Some(pattern);
            } else if let Some(count) = key.strip_prefix("relative-type-") {
                let count = count
                    .parse::<i8>()
                    .map_err(|_| A::Error::unknown_field(&key, &["not able to parse as u32"]))?;
                relatives.push(Relative {
                    count,
                    pattern: map.next_value::<String>()?,
                });
            } else {
                // Ignore other keys
                let _ = map.next_value::<IgnoredAny>()?;
            }
        }
        Ok(Field {
            _display_name: display_name,
            _relative_period: relative_period,
            relatives,
            past: past.unwrap_or_default(),
            future: future.unwrap_or_default(),
        })
    }
}

impl<'de> Deserialize<'de> for Field {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(FieldVisitor)
    }
}

#[derive(Debug, Deserialize)]
pub(in crate::provider) struct Fields(pub(in crate::provider) HashMap<String, Field>);

#[derive(Debug, Deserialize)]
pub(in crate::provider) struct Dates {
    pub(in crate::provider) fields: Fields,
}

#[derive(Debug, Deserialize)]
pub(in crate::provider) struct RelativeTimeDates {
    pub(in crate::provider) dates: Dates,
}

pub(in crate::provider) type Resource = super::LocaleResource<RelativeTimeDates>;
