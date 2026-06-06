// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
#[diplomat::abi_rename = "icu4x_{0}_mv1"]
pub mod ffi {
    use alloc::boxed::Box;
    use crate::unstable::errors::ffi::DataError;

    #[cfg(feature = "buffer_provider")]
    use crate::unstable::provider::ffi::DataProvider;

    #[diplomat::opaque_mut]
    #[diplomat::attr(demo_gen, disable)]
    pub struct ICU4XMeasureUnit(pub icu_experimental::measure::measureunit::MeasureUnit);

    #[diplomat::opaque_mut]
    #[diplomat::attr(demo_gen, disable)]
    pub struct ICU4XUnitsConverter(pub icu_experimental::units::converter::UnitsConverter<f64>);

    #[diplomat::opaque_mut]
    #[diplomat::attr(demo_gen, disable)]
    pub struct ICU4XUnitsConverterFactory(pub icu_experimental::units::converter_factory::ConverterFactory);

    impl ICU4XUnitsConverterFactory {
        /// Create a new [`ICU4XUnitsConverterFactory`] with compiled data.
        #[diplomat::rust_link(icu::units::converter_factory::ConverterFactory::new, FnInStruct)]
        #[cfg(feature = "compiled_data")]
        #[diplomat::attr(auto, constructor)]
        pub fn create() -> Box<ICU4XUnitsConverterFactory> {
            Box::new(ICU4XUnitsConverterFactory(
                icu_experimental::units::converter_factory::ConverterFactory::new()
            ))
        }

        /// Create a new [`ICU4XUnitsConverterFactory`] with a data provider.
        #[diplomat::rust_link(icu::units::converter_factory::ConverterFactory::try_new_with_buffer_provider, FnInStruct)]
        #[cfg(feature = "buffer_provider")]
        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "with_provider")]
        pub fn create_with_provider(provider: &DataProvider) -> Result<Box<ICU4XUnitsConverterFactory>, DataError> {
            Ok(Box::new(ICU4XUnitsConverterFactory(
                icu_experimental::units::converter_factory::ConverterFactory::try_new_with_buffer_provider(provider.get()?)?
            )))
        }

        /// Create a new [`ICU4XUnitsConverter`] for the given input and output units.
        pub fn converter(
            &self,
            input_unit: &ICU4XMeasureUnit,
            output_unit: &ICU4XMeasureUnit,
        ) -> Option<Box<ICU4XUnitsConverter>> {
            self.0.converter::<f64>(&input_unit.0, &output_unit.0)
                .map(|c| Box::new(ICU4XUnitsConverter(c)))
        }
    }

    impl ICU4XMeasureUnit {
        /// Parse a [`ICU4XMeasureUnit`] from a string.
        pub fn deserialize(
            s: &DiplomatStr,
        ) -> Result<Box<ICU4XMeasureUnit>, DataError> {
            let unit_str = core::str::from_utf8(s).map_err(|_| DataError::Deserialize)?;
            let unit = icu_experimental::measure::measureunit::MeasureUnit::try_from_str(unit_str)
                .map_err(|_| DataError::Deserialize)?;
            Ok(Box::new(ICU4XMeasureUnit(unit)))
        }
    }

    impl ICU4XUnitsConverter {
        /// Convert a value.
        pub fn convert(&self, value: f64) -> f64 {
            self.0.convert(&value)
        }
    }
}
