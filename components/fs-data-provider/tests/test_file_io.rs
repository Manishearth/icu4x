use icu_data_provider::prelude::*;
use icu_data_provider::structs;
use icu_fs_data_provider::FsDataProvider;
use std::borrow::Cow;

#[test]
fn test_read_json() {
    let provider = FsDataProvider::try_new("../../resources/testdata/data/json")
        .expect("Loading file from testdata directory");

    let response = provider
        .load(&DataRequest {
            data_key: icu_data_key!(plurals: cardinal@1),
            data_entry: DataEntry {
                variant: None,
                // TODO: Migrate to LanguageIdentifier macro
                langid: "sr".parse().expect("Valid language tag"),
            },
        })
        .expect("The key should be present in the testdata");
    let plurals_data: &structs::plurals::PluralRuleStringsV1 = response
        .borrow_payload()
        .expect("The JSON should match the struct definition");
    assert_eq!(
        plurals_data,
        &structs::plurals::PluralRuleStringsV1 {
            zero: None,
            one: Some(Cow::Borrowed("v = 0 and i % 10 = 1 and i % 100 != 11 or f % 10 = 1 and f % 100 != 11")),
            two: None,
            few: Some(Cow::Borrowed("v = 0 and i % 10 = 2..4 and i % 100 != 12..14 or f % 10 = 2..4 and f % 100 != 12..14")),
            many: None,
        }
    );
}
