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

struct RootOnlyBaked;

const _: () = {
    impl RootOnlyBaked {
        #[allow(dead_code)]
        pub(crate) const MUST_USE_MAKE_PROVIDER_MACRO: () = ();
    }
    icu_provider::marker::impl_data_provider_never_marker!(RootOnlyBaked);
    #[allow(unused_imports)]
    mod baked_data {
        include!("root_only/hello_world_v1.rs.data");
    }
    baked_data::impl_hello_world_v1!(RootOnlyBaked, DRY, ITER);
};

struct RootOnlyProvider;

impl DataProvider<HelloWorldV1> for RootOnlyProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<HelloWorldV1>, DataError> {
        let msg = match (
            req.id.locale.is_unknown(),
            req.id.marker_attributes.as_str(),
        ) {
            (true, "") => "root",
            (true, "reverse") => "toor",
            _ => return Err(DataErrorKind::IdentifierNotFound.with_req(HelloWorldV1::INFO, req)),
        };
        Ok(DataResponse {
            metadata: DataResponseMetadata::default().with_checksum(1234),
            payload: DataPayload::from_owned(HelloWorld {
                message: msg.into(),
            }),
        })
    }
}

impl IterableDataProvider<HelloWorldV1> for RootOnlyProvider {
    fn iter_ids(&self) -> Result<BTreeSet<DataIdentifierCow<'_>>, DataError> {
        Ok(["", "reverse"]
            .into_iter()
            .map(|a| {
                DataIdentifierCow::from_owned(
                    DataMarkerAttributes::from_str_or_panic(a).to_owned(),
                    Default::default(),
                )
            })
            .collect())
    }
}

make_exportable_provider!(RootOnlyProvider, [HelloWorldV1,]);

#[test]
fn verify_fixture_and_root_only_dry_load() {
    let fixture =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/root_only/hello_world_v1.rs.data");
    let tmp_dir = std::env::temp_dir().join(format!("icu4x_root_only_{}", std::process::id()));
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
        &RootOnlyProvider,
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

    for (loc, attr, prefix, expected) in [
        (data_locale!("und"), "", false, Ok(None)),
        (data_locale!("fr"), "", false, Ok(Some(data_locale!("und")))),
        (data_locale!("und"), "reverse", false, Ok(None)),
        (
            data_locale!("fr"),
            "reverse",
            false,
            Ok(Some(data_locale!("und"))),
        ),
        (
            data_locale!("fr"),
            "rev",
            true,
            Ok(Some(data_locale!("und"))),
        ),
        (
            data_locale!("fr"),
            "nonexistent",
            false,
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
        let dry = DryDataProvider::<HelloWorldV1>::dry_load(&RootOnlyBaked, req);
        let full = DataProvider::<HelloWorldV1>::load(&RootOnlyBaked, req).map(|r| r.metadata);
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
