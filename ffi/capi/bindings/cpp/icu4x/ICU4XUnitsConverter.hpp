#ifndef ICU4X_ICU4XUnitsConverter_HPP
#define ICU4X_ICU4XUnitsConverter_HPP

#include "ICU4XUnitsConverter.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "diplomat_runtime.hpp"


namespace icu4x {
namespace capi {
    extern "C" {

    double icu4x_ICU4XUnitsConverter_convert_mv1(const icu4x::capi::ICU4XUnitsConverter* self, double value);

    void icu4x_ICU4XUnitsConverter_destroy_mv1(ICU4XUnitsConverter* self);

    } // extern "C"
} // namespace capi
} // namespace

inline double icu4x::ICU4XUnitsConverter::convert(double value) const {
    auto result = icu4x::capi::icu4x_ICU4XUnitsConverter_convert_mv1(this->AsFFI(),
        value);
    return result;
}

inline const icu4x::capi::ICU4XUnitsConverter* icu4x::ICU4XUnitsConverter::AsFFI() const {
    return reinterpret_cast<const icu4x::capi::ICU4XUnitsConverter*>(this);
}

inline icu4x::capi::ICU4XUnitsConverter* icu4x::ICU4XUnitsConverter::AsFFI() {
    return reinterpret_cast<icu4x::capi::ICU4XUnitsConverter*>(this);
}

inline const icu4x::ICU4XUnitsConverter* icu4x::ICU4XUnitsConverter::FromFFI(const icu4x::capi::ICU4XUnitsConverter* ptr) {
    return reinterpret_cast<const icu4x::ICU4XUnitsConverter*>(ptr);
}

inline icu4x::ICU4XUnitsConverter* icu4x::ICU4XUnitsConverter::FromFFI(icu4x::capi::ICU4XUnitsConverter* ptr) {
    return reinterpret_cast<icu4x::ICU4XUnitsConverter*>(ptr);
}

inline void icu4x::ICU4XUnitsConverter::operator delete(void* ptr) {
    icu4x::capi::icu4x_ICU4XUnitsConverter_destroy_mv1(reinterpret_cast<icu4x::capi::ICU4XUnitsConverter*>(ptr));
}


#endif // ICU4X_ICU4XUnitsConverter_HPP
