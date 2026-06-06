#ifndef ICU4X_ICU4XMeasureUnit_HPP
#define ICU4X_ICU4XMeasureUnit_HPP

#include "ICU4XMeasureUnit.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "DataError.hpp"
#include "diplomat_runtime.hpp"


namespace icu4x {
namespace capi {
    extern "C" {

    typedef struct icu4x_ICU4XMeasureUnit_deserialize_mv1_result {union {icu4x::capi::ICU4XMeasureUnit* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_ICU4XMeasureUnit_deserialize_mv1_result;
    icu4x_ICU4XMeasureUnit_deserialize_mv1_result icu4x_ICU4XMeasureUnit_deserialize_mv1(icu4x::diplomat::capi::DiplomatStringView s);

    void icu4x_ICU4XMeasureUnit_destroy_mv1(ICU4XMeasureUnit* self);

    } // extern "C"
} // namespace capi
} // namespace

inline icu4x::diplomat::result<std::unique_ptr<icu4x::ICU4XMeasureUnit>, icu4x::DataError> icu4x::ICU4XMeasureUnit::deserialize(std::string_view s) {
    auto result = icu4x::capi::icu4x_ICU4XMeasureUnit_deserialize_mv1({s.data(), s.size()});
    return result.is_ok ? icu4x::diplomat::result<std::unique_ptr<icu4x::ICU4XMeasureUnit>, icu4x::DataError>(icu4x::diplomat::Ok<std::unique_ptr<icu4x::ICU4XMeasureUnit>>(std::unique_ptr<icu4x::ICU4XMeasureUnit>(icu4x::ICU4XMeasureUnit::FromFFI(result.ok)))) : icu4x::diplomat::result<std::unique_ptr<icu4x::ICU4XMeasureUnit>, icu4x::DataError>(icu4x::diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline const icu4x::capi::ICU4XMeasureUnit* icu4x::ICU4XMeasureUnit::AsFFI() const {
    return reinterpret_cast<const icu4x::capi::ICU4XMeasureUnit*>(this);
}

inline icu4x::capi::ICU4XMeasureUnit* icu4x::ICU4XMeasureUnit::AsFFI() {
    return reinterpret_cast<icu4x::capi::ICU4XMeasureUnit*>(this);
}

inline const icu4x::ICU4XMeasureUnit* icu4x::ICU4XMeasureUnit::FromFFI(const icu4x::capi::ICU4XMeasureUnit* ptr) {
    return reinterpret_cast<const icu4x::ICU4XMeasureUnit*>(ptr);
}

inline icu4x::ICU4XMeasureUnit* icu4x::ICU4XMeasureUnit::FromFFI(icu4x::capi::ICU4XMeasureUnit* ptr) {
    return reinterpret_cast<icu4x::ICU4XMeasureUnit*>(ptr);
}

inline void icu4x::ICU4XMeasureUnit::operator delete(void* ptr) {
    icu4x::capi::icu4x_ICU4XMeasureUnit_destroy_mv1(reinterpret_cast<icu4x::capi::ICU4XMeasureUnit*>(ptr));
}


#endif // ICU4X_ICU4XMeasureUnit_HPP
