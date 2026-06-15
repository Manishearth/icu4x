// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::SourceDataProvider;
use icu::properties::props::EmojiSet;
use icu::properties::provider::*;
use icu_provider::prelude::*;
use std::collections::HashSet;

macro_rules! expand {
    ($(($prop:ty, $marker:ident)),+) => {
        $(
            impl DataProvider<$marker> for SourceDataProvider {
                fn load(
                    &self,
                    req: DataRequest,
                ) -> Result<DataResponse<$marker>, DataError> {
                    self.check_req::<$marker>(req)?;
                    let data = self.get_unicodeset_property(
                        core::str::from_utf8(<$prop as EmojiSet>::NAME).unwrap(),
                        core::str::from_utf8(<$prop as EmojiSet>::SHORT_NAME).unwrap(),
                    )?;

                    Ok(DataResponse {
                        metadata: Default::default(),
                        payload: DataPayload::from_owned(PropertyUnicodeSet::CPInversionListStrList(data)),
                    })
                }
            }

            impl crate::IterableDataProviderCached<$marker> for SourceDataProvider {
                fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
                    Ok(HashSet::from_iter([Default::default()]))
                }
            }
        )+
    };
}

expand!((
    icu::properties::props::BasicEmoji,
    PropertyBinaryBasicEmojiV1
));

#[test]
fn test_basic() {
    use icu::properties::{EmojiSetData, props::BasicEmoji};

    let provider = SourceDataProvider::new_testing();

    let basic_emoji = EmojiSetData::try_new_unstable::<BasicEmoji>(&provider).unwrap();
    let basic_emoji = basic_emoji
        .as_code_point_inversion_list_string_list()
        .unwrap();

    assert!(!basic_emoji.contains32(0x0020));
    assert!(!basic_emoji.contains('\n'));
    assert!(basic_emoji.contains('🦃')); // U+1F983 TURKEY
    assert!(basic_emoji.contains_str("\u{1F983}"));
    assert!(basic_emoji.contains_str("\u{1F6E4}\u{FE0F}")); // railway track
    assert!(!basic_emoji.contains_str("\u{0033}\u{FE0F}\u{20E3}")); // Emoji_Keycap_Sequence, keycap 3
}
