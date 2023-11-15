// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_locid::langid;
use icu_provider::datagen::IterableDataProvider;
use icu_provider::hello_world::{HelloWorldProvider, HelloWorldV1, HelloWorldV1Marker};
use icu_provider::prelude::*;
use icu_provider_fs::FsDataProvider;
use std::path::PathBuf;

fn paths() -> [PathBuf; 3] {
    [
        PathBuf::from("./tests/data/json"),
        PathBuf::from("./tests/data/bincode"),
        PathBuf::from("./tests/data/postcard"),
    ]
}

#[test]
fn test_provider() {
    for path in paths() {
        let provider = FsDataProvider::try_new(&path).unwrap();
        for locale in HelloWorldProvider.supported_locales().unwrap() {
            let req = DataRequest {
                locale: &locale,
                metadata: Default::default(),
            };

            let expected = HelloWorldProvider
                .load(req)
                .unwrap_or_else(|e| panic!("{e}: {req} ({path:?})"))
                .take_payload()
                .unwrap();

            let actual: DataPayload<HelloWorldV1Marker> = provider
                .as_deserializing()
                .load(req)
                .unwrap_or_else(|e| panic!("{e}: {req} ({path:?})"))
                .take_payload()
                .unwrap();
            assert_eq!(actual.get(), expected.get());

            let actual: DataPayload<HelloWorldV1Marker> = (&provider as &dyn BufferProvider)
                .as_deserializing()
                .load(req)
                .unwrap_or_else(|e| panic!("{e}: {req} ({path:?})"))
                .take_payload()
                .unwrap();
            assert_eq!(actual.get(), expected.get());
        }
    }
}

#[test]
fn test_errors() {
    for path in paths() {
        let provider = FsDataProvider::try_new(&path).unwrap();

        let err: Result<DataResponse<HelloWorldV1Marker>, DataError> =
            provider.as_deserializing().load(DataRequest {
                locale: &langid!("zh-DE").into(),
                metadata: Default::default(),
            });

        assert!(
            matches!(
                err,
                Err(DataError {
                    kind: DataErrorKind::MissingLocale,
                    ..
                })
            ),
            "{err:?}"
        );

        struct WrongV1Marker;
        impl DataMarker for WrongV1Marker {
            type Yokeable = HelloWorldV1<'static>;
        }
        impl KeyedDataMarker for WrongV1Marker {
            const KEY: DataKey = data_key!("nope@1");
        }

        let err: Result<DataResponse<WrongV1Marker>, DataError> =
            provider.as_deserializing().load(Default::default());

        assert!(
            matches!(
                err,
                Err(DataError {
                    kind: DataErrorKind::MissingDataKey,
                    ..
                })
            ),
            "{err:?}"
        );
    }
}
