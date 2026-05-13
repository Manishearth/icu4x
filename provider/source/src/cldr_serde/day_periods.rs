// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Debug, Deserialize)]
pub struct Resource {
    pub supplemental: Supplemental,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Supplemental {
    pub day_period_rule_set: DayPeriodRules,
}

#[derive(Debug, Deserialize)]
pub struct DayPeriodRules(pub BTreeMap<String, BTreeMap<String, DayPeriodRule>>);

#[derive(Debug, Deserialize)]
pub struct DayPeriodRule {
    #[serde(rename = "_from")]
    pub from: Option<String>,
    #[serde(rename = "_before")]
    pub before: Option<String>,
    #[serde(rename = "_at")]
    pub at: Option<String>,
}
