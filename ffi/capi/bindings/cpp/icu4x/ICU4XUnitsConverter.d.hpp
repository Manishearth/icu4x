#ifndef ICU4X_ICU4XUnitsConverter_D_HPP
#define ICU4X_ICU4XUnitsConverter_D_HPP

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
    struct ICU4XUnitsConverter;
} // namespace capi
} // namespace

namespace icu4x {
class ICU4XUnitsConverter {
public:

  /**
   * Convert a value.
   */
  inline double convert(double value) const;

    inline const icu4x::capi::ICU4XUnitsConverter* AsFFI() const;
    inline icu4x::capi::ICU4XUnitsConverter* AsFFI();
    inline static const icu4x::ICU4XUnitsConverter* FromFFI(const icu4x::capi::ICU4XUnitsConverter* ptr);
    inline static icu4x::ICU4XUnitsConverter* FromFFI(icu4x::capi::ICU4XUnitsConverter* ptr);
    inline static void operator delete(void* ptr);
private:
    ICU4XUnitsConverter() = delete;
    ICU4XUnitsConverter(const icu4x::ICU4XUnitsConverter&) = delete;
    ICU4XUnitsConverter(icu4x::ICU4XUnitsConverter&&) noexcept = delete;
    ICU4XUnitsConverter operator=(const icu4x::ICU4XUnitsConverter&) = delete;
    ICU4XUnitsConverter operator=(icu4x::ICU4XUnitsConverter&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // ICU4X_ICU4XUnitsConverter_D_HPP
