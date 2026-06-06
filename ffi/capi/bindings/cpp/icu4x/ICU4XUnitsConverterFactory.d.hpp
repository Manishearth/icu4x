#ifndef ICU4X_ICU4XUnitsConverterFactory_D_HPP
#define ICU4X_ICU4XUnitsConverterFactory_D_HPP

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
namespace capi { struct DataProvider; }
class DataProvider;
namespace capi { struct ICU4XMeasureUnit; }
class ICU4XMeasureUnit;
namespace capi { struct ICU4XUnitsConverter; }
class ICU4XUnitsConverter;
namespace capi { struct ICU4XUnitsConverterFactory; }
class ICU4XUnitsConverterFactory;
class DataError;
} // namespace icu4x



namespace icu4x {
namespace capi {
    struct ICU4XUnitsConverterFactory;
} // namespace capi
} // namespace

namespace icu4x {
class ICU4XUnitsConverterFactory {
public:

  /**
   * Create a new {@link ICU4XUnitsConverterFactory} with compiled data.
   *
   * See the [Rust documentation for `new`](https://docs.rs/icu/2.2.0/icu/units/converter_factory/struct.ConverterFactory.html#method.new) for more information.
   */
  inline static std::unique_ptr<icu4x::ICU4XUnitsConverterFactory> create();

  /**
   * Create a new {@link ICU4XUnitsConverterFactory} with a data provider.
   *
   * See the [Rust documentation for `try_new_with_buffer_provider`](https://docs.rs/icu/2.2.0/icu/units/converter_factory/struct.ConverterFactory.html#method.try_new_with_buffer_provider) for more information.
   */
  inline static icu4x::diplomat::result<std::unique_ptr<icu4x::ICU4XUnitsConverterFactory>, icu4x::DataError> create_with_provider(const icu4x::DataProvider& provider);

  /**
   * Create a new {@link ICU4XUnitsConverter} for the given input and output units.
   */
  inline std::unique_ptr<icu4x::ICU4XUnitsConverter> converter(const icu4x::ICU4XMeasureUnit& input_unit, const icu4x::ICU4XMeasureUnit& output_unit) const;

    inline const icu4x::capi::ICU4XUnitsConverterFactory* AsFFI() const;
    inline icu4x::capi::ICU4XUnitsConverterFactory* AsFFI();
    inline static const icu4x::ICU4XUnitsConverterFactory* FromFFI(const icu4x::capi::ICU4XUnitsConverterFactory* ptr);
    inline static icu4x::ICU4XUnitsConverterFactory* FromFFI(icu4x::capi::ICU4XUnitsConverterFactory* ptr);
    inline static void operator delete(void* ptr);
private:
    ICU4XUnitsConverterFactory() = delete;
    ICU4XUnitsConverterFactory(const icu4x::ICU4XUnitsConverterFactory&) = delete;
    ICU4XUnitsConverterFactory(icu4x::ICU4XUnitsConverterFactory&&) noexcept = delete;
    ICU4XUnitsConverterFactory operator=(const icu4x::ICU4XUnitsConverterFactory&) = delete;
    ICU4XUnitsConverterFactory operator=(icu4x::ICU4XUnitsConverterFactory&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // ICU4X_ICU4XUnitsConverterFactory_D_HPP
