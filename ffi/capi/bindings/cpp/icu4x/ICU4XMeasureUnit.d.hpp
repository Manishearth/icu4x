#ifndef ICU4X_ICU4XMeasureUnit_D_HPP
#define ICU4X_ICU4XMeasureUnit_D_HPP

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
namespace capi { struct ICU4XMeasureUnit; }
class ICU4XMeasureUnit;
class DataError;
} // namespace icu4x



namespace icu4x {
namespace capi {
    struct ICU4XMeasureUnit;
} // namespace capi
} // namespace

namespace icu4x {
class ICU4XMeasureUnit {
public:

  /**
   * Parse a {@link ICU4XMeasureUnit} from a string.
   */
  inline static icu4x::diplomat::result<std::unique_ptr<icu4x::ICU4XMeasureUnit>, icu4x::DataError> deserialize(std::string_view s);

    inline const icu4x::capi::ICU4XMeasureUnit* AsFFI() const;
    inline icu4x::capi::ICU4XMeasureUnit* AsFFI();
    inline static const icu4x::ICU4XMeasureUnit* FromFFI(const icu4x::capi::ICU4XMeasureUnit* ptr);
    inline static icu4x::ICU4XMeasureUnit* FromFFI(icu4x::capi::ICU4XMeasureUnit* ptr);
    inline static void operator delete(void* ptr);
private:
    ICU4XMeasureUnit() = delete;
    ICU4XMeasureUnit(const icu4x::ICU4XMeasureUnit&) = delete;
    ICU4XMeasureUnit(icu4x::ICU4XMeasureUnit&&) noexcept = delete;
    ICU4XMeasureUnit operator=(const icu4x::ICU4XMeasureUnit&) = delete;
    ICU4XMeasureUnit operator=(icu4x::ICU4XMeasureUnit&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // ICU4X_ICU4XMeasureUnit_D_HPP
