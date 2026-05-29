// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_locale_core::{langid, locale};
use icu_plurals::{provider::PluralsCardinalV1, PluralCategory, PluralRules};
use icu_provider::prelude::*;

#[test]
fn test_plural_rules() {
    assert_eq!(
        PluralRules::try_new(locale!("en").into(), Default::default())
            .unwrap()
            .category_for(5_usize),
        PluralCategory::Other
    );
}

#[test]
fn test_static_load_works() {
    DataProvider::<PluralsCardinalV1>::load(
        &icu_plurals::provider::Baked,
        DataRequest {
            id: DataIdentifierBorrowed::for_locale(&langid!("en").into()),
            ..Default::default()
        },
    )
    .expect("Failed to load payload");
}

#[test]
fn test_plural_category_all() {
    let categories: Vec<PluralCategory> = PluralCategory::all().collect();

    assert_eq!(categories.len(), 6);

    assert_eq!(categories[0], PluralCategory::Few);
    assert_eq!(categories[1], PluralCategory::Many);
    assert_eq!(categories[2], PluralCategory::One);
    assert_eq!(categories[3], PluralCategory::Other);
    assert_eq!(categories[4], PluralCategory::Two);
    assert_eq!(categories[5], PluralCategory::Zero);
}

#[cfg(feature = "serde")]
#[test]
fn test_plural_elements_serde() {
    use icu_plurals::PluralElements;

    // 1. Single "other" value
    let single = PluralElements::new("hello".to_string());

    #[cfg(feature = "datagen")]
    {
        let single_json = serde_json::to_string(&single).unwrap();
        // In human-readable format, it should serialize as a flat string.
        assert_eq!(single_json, "\"hello\"");

        // Deserialize back
        let single_deser: PluralElements<String> = serde_json::from_str(&single_json).unwrap();
        assert_eq!(single, single_deser);
    }

    // Deserialize from map format (it should also work)
    let map_json = "{\"other\":\"hello\"}";
    let map_deser: PluralElements<String> = serde_json::from_str(map_json).unwrap();
    assert_eq!(single, map_deser);

    // 2. Multiple values
    let multi =
        PluralElements::new("other_val".to_string()).with_one_value(Some("one_val".to_string()));

    #[cfg(feature = "datagen")]
    {
        let multi_json = serde_json::to_string(&multi).unwrap();
        // Should serialize as map
        assert_eq!(multi_json, "{\"one\":\"one_val\",\"other\":\"other_val\"}");

        // Deserialize back
        let multi_deser: PluralElements<String> = serde_json::from_str(&multi_json).unwrap();
        assert_eq!(multi, multi_deser);
    }
}
