#ifndef ICU4X_ICU4XUnitsConverterFactory_HPP
#define ICU4X_ICU4XUnitsConverterFactory_HPP

#include "ICU4XUnitsConverterFactory.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "DataError.hpp"
#include "DataProvider.hpp"
#include "ICU4XMeasureUnit.hpp"
#include "ICU4XUnitsConverter.hpp"
#include "diplomat_runtime.hpp"


namespace icu4x {
namespace capi {
    extern "C" {

    icu4x::capi::ICU4XUnitsConverterFactory* icu4x_ICU4XUnitsConverterFactory_create_mv1(void);

    typedef struct icu4x_ICU4XUnitsConverterFactory_create_with_provider_mv1_result {union {icu4x::capi::ICU4XUnitsConverterFactory* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_ICU4XUnitsConverterFactory_create_with_provider_mv1_result;
    icu4x_ICU4XUnitsConverterFactory_create_with_provider_mv1_result icu4x_ICU4XUnitsConverterFactory_create_with_provider_mv1(const icu4x::capi::DataProvider* provider);

    icu4x::capi::ICU4XUnitsConverter* icu4x_ICU4XUnitsConverterFactory_converter_mv1(const icu4x::capi::ICU4XUnitsConverterFactory* self, const icu4x::capi::ICU4XMeasureUnit* input_unit, const icu4x::capi::ICU4XMeasureUnit* output_unit);

    void icu4x_ICU4XUnitsConverterFactory_destroy_mv1(ICU4XUnitsConverterFactory* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<icu4x::ICU4XUnitsConverterFactory> icu4x::ICU4XUnitsConverterFactory::create() {
    auto result = icu4x::capi::icu4x_ICU4XUnitsConverterFactory_create_mv1();
    return std::unique_ptr<icu4x::ICU4XUnitsConverterFactory>(icu4x::ICU4XUnitsConverterFactory::FromFFI(result));
}

inline icu4x::diplomat::result<std::unique_ptr<icu4x::ICU4XUnitsConverterFactory>, icu4x::DataError> icu4x::ICU4XUnitsConverterFactory::create_with_provider(const icu4x::DataProvider& provider) {
    auto result = icu4x::capi::icu4x_ICU4XUnitsConverterFactory_create_with_provider_mv1(provider.AsFFI());
    return result.is_ok ? icu4x::diplomat::result<std::unique_ptr<icu4x::ICU4XUnitsConverterFactory>, icu4x::DataError>(icu4x::diplomat::Ok<std::unique_ptr<icu4x::ICU4XUnitsConverterFactory>>(std::unique_ptr<icu4x::ICU4XUnitsConverterFactory>(icu4x::ICU4XUnitsConverterFactory::FromFFI(result.ok)))) : icu4x::diplomat::result<std::unique_ptr<icu4x::ICU4XUnitsConverterFactory>, icu4x::DataError>(icu4x::diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline std::unique_ptr<icu4x::ICU4XUnitsConverter> icu4x::ICU4XUnitsConverterFactory::converter(const icu4x::ICU4XMeasureUnit& input_unit, const icu4x::ICU4XMeasureUnit& output_unit) const {
    auto result = icu4x::capi::icu4x_ICU4XUnitsConverterFactory_converter_mv1(this->AsFFI(),
        input_unit.AsFFI(),
        output_unit.AsFFI());
    return std::unique_ptr<icu4x::ICU4XUnitsConverter>(icu4x::ICU4XUnitsConverter::FromFFI(result));
}

inline const icu4x::capi::ICU4XUnitsConverterFactory* icu4x::ICU4XUnitsConverterFactory::AsFFI() const {
    return reinterpret_cast<const icu4x::capi::ICU4XUnitsConverterFactory*>(this);
}

inline icu4x::capi::ICU4XUnitsConverterFactory* icu4x::ICU4XUnitsConverterFactory::AsFFI() {
    return reinterpret_cast<icu4x::capi::ICU4XUnitsConverterFactory*>(this);
}

inline const icu4x::ICU4XUnitsConverterFactory* icu4x::ICU4XUnitsConverterFactory::FromFFI(const icu4x::capi::ICU4XUnitsConverterFactory* ptr) {
    return reinterpret_cast<const icu4x::ICU4XUnitsConverterFactory*>(ptr);
}

inline icu4x::ICU4XUnitsConverterFactory* icu4x::ICU4XUnitsConverterFactory::FromFFI(icu4x::capi::ICU4XUnitsConverterFactory* ptr) {
    return reinterpret_cast<icu4x::ICU4XUnitsConverterFactory*>(ptr);
}

inline void icu4x::ICU4XUnitsConverterFactory::operator delete(void* ptr) {
    icu4x::capi::icu4x_ICU4XUnitsConverterFactory_destroy_mv1(reinterpret_cast<icu4x::capi::ICU4XUnitsConverterFactory*>(ptr));
}


#endif // ICU4X_ICU4XUnitsConverterFactory_HPP
