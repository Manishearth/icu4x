// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

extern crate alloc;

use icu_locale_core::data_locale;
use icu_locale_fallback::LocaleFallbacker;
use icu_provider::export::*;
use icu_provider::hello_world::{HelloWorld, HelloWorldV1};
use icu_provider::prelude::*;
use icu_provider_baked::export::{BakedExporter, Options};
use icu_provider_export::prelude::*;
use std::collections::BTreeSet;
use std::path::Path;

struct RetainBaseBaked;

const _: () = {
    impl RetainBaseBaked {
        #[allow(dead_code)]
        pub(crate) const MUST_USE_MAKE_PROVIDER_MACRO: () = ();
    }
    icu_provider::marker::impl_data_provider_never_marker!(RetainBaseBaked);
    #[allow(unused_imports)]
    mod baked_data {
        include!("retain_base/hello_world_v1.rs.data");
    }
    baked_data::impl_hello_world_v1!(RetainBaseBaked, DRY, ITER);
};

struct TestingProvider(&'static [(&'static str, &'static str, &'static str)]);

impl DataProvider<HelloWorldV1> for TestingProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<HelloWorldV1>, DataError> {
        let locale_str = req.id.locale.to_string();
        let (_, _, msg) = self
            .0
            .iter()
            .find(|(l, a, _)| *l == locale_str && *a == req.id.marker_attributes.as_str())
            .ok_or_else(|| DataErrorKind::IdentifierNotFound.with_req(HelloWorldV1::INFO, req))?;
        Ok(DataResponse {
            metadata: DataResponseMetadata::default().with_checksum(1234),
            payload: DataPayload::from_owned(HelloWorld {
                message: (*msg).into(),
            }),
        })
    }
}

impl IterableDataProvider<HelloWorldV1> for TestingProvider {
    fn iter_ids(&self) -> Result<BTreeSet<DataIdentifierCow<'_>>, DataError> {
        Ok(self
            .0
            .iter()
            .map(|(l, a, _)| {
                DataIdentifierCow::from_owned(
                    DataMarkerAttributes::from_str_or_panic(a).to_owned(),
                    l.parse().unwrap(),
                )
            })
            .collect())
    }
}

make_exportable_provider!(TestingProvider, [HelloWorldV1,]);

const SAMPLE: &[(&str, &str, &str)] = &[
    ("und", "", "root"),
    ("en", "", "root"), // same as `und`, retained in TRIE by `RetainBaseLanguages`
    ("en-001", "", "root"), // deduplicated to `en`
    ("en-GB", "", "root"), // deduplicated to `en`
    ("es", "", "hola"), // distinct base
    ("es-AR", "", "holaAR"), // distinct regional override
    ("es-MX", "", "hola"), // deduplicated to `es`
    ("sr-Latn", "", "zdravo"),
    ("sr-Latn-ME", "", "zdravo"),
    ("de", "", "hallo"),
    ("de", "reverse", "ollah"),
    ("de-AT", "reverse", "ollah"),
];

#[test]
fn verify_fixture_and_dry_load() {
    let fixture =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/retain_base/hello_world_v1.rs.data");
    let tmp_dir = std::env::temp_dir().join(format!("icu4x_retain_base_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&tmp_dir);

    let mut options = Options::default();
    options.pretty = true;
    options.overwrite = true;
    ExportDriver::new(
        [DataLocaleFamily::FULL],
        DeduplicationStrategy::RetainBaseLanguages.into(),
        LocaleFallbacker::new().static_to_owned(),
    )
    .export(
        &TestingProvider(SAMPLE),
        BakedExporter::new(tmp_dir.clone(), options).unwrap(),
    )
    .unwrap();

    let generated = std::fs::read_to_string(tmp_dir.join("hello_world_v1.rs.data")).unwrap();
    let _ = std::fs::remove_dir_all(&tmp_dir);

    if std::env::var_os("ICU4X_WRITE_BAKED_TESTDATA").is_some() {
        std::fs::create_dir_all(fixture.parent().unwrap()).unwrap();
        std::fs::write(&fixture, &generated).unwrap();
        return;
    }
    assert_eq!(generated, std::fs::read_to_string(&fixture).unwrap());

    let (_, dry_section) = generated.split_once("($ provider : ty , DRY) =>").unwrap();
    assert!(dry_section.contains("DataStore::contains"));
    assert!(!dry_section.contains("DataStore::get") && !dry_section.contains("DataProvider::"));

    for (loc, attr, prefix, expected) in [
        (data_locale!("es"), "", false, Ok(None)),
        (data_locale!("en"), "", false, Ok(None)),
        (data_locale!("es-AR"), "", false, Ok(None)),
        (data_locale!("und"), "", false, Ok(None)),
        (
            data_locale!("en-GB"),
            "",
            false,
            Ok(Some(data_locale!("en"))),
        ),
        (
            data_locale!("es-MX"),
            "",
            false,
            Ok(Some(data_locale!("es"))),
        ),
        (
            data_locale!("sr-ME"),
            "",
            false,
            Ok(Some(data_locale!("sr-Latn"))),
        ),
        (data_locale!("fr"), "", false, Ok(Some(data_locale!("und")))),
        (data_locale!("de"), "reverse", false, Ok(None)),
        (
            data_locale!("de-AT"),
            "reverse",
            false,
            Ok(Some(data_locale!("de"))),
        ),
        (
            data_locale!("de"),
            "rev",
            false,
            Err(DataErrorKind::IdentifierNotFound),
        ),
        (data_locale!("de"), "rev", true, Ok(None)),
        (
            data_locale!("de-AT"),
            "rev",
            true,
            Ok(Some(data_locale!("de"))),
        ),
        (
            data_locale!("fr"),
            "reverse",
            false,
            Err(DataErrorKind::IdentifierNotFound),
        ),
        (
            data_locale!("de"),
            "nonexistent",
            true,
            Err(DataErrorKind::IdentifierNotFound),
        ),
    ] {
        let mut metadata = DataRequestMetadata::default();
        metadata.attributes_prefix_match = prefix;
        let req = DataRequest {
            id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                DataMarkerAttributes::from_str_or_panic(attr),
                &loc,
            ),
            metadata,
        };
        let dry = DryDataProvider::<HelloWorldV1>::dry_load(&RetainBaseBaked, req);
        let full = DataProvider::<HelloWorldV1>::load(&RetainBaseBaked, req).map(|r| r.metadata);
        assert_eq!(dry, full, "diverged for {req:?}");
        assert_eq!(
            dry.map(|m| {
                assert_eq!(m.checksum, Some(1234));
                m.locale
            })
            .map_err(|e| e.kind),
            expected,
            "unexpected result for {req:?}"
        );
    }
}
