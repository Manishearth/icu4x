#ifndef Calendar_HPP
#define Calendar_HPP

#include "Calendar.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "AnyCalendarKind.hpp"
#include "DataError.hpp"
#include "DataProvider.hpp"
#include "Locale.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    typedef struct ICU4XCalendar_create_for_locale_result {union {diplomat::capi::Calendar* ok; diplomat::capi::DataError err;}; bool is_ok;} ICU4XCalendar_create_for_locale_result;
    ICU4XCalendar_create_for_locale_result ICU4XCalendar_create_for_locale(const diplomat::capi::DataProvider* provider, const diplomat::capi::Locale* locale);
    
    typedef struct ICU4XCalendar_create_for_kind_result {union {diplomat::capi::Calendar* ok; diplomat::capi::DataError err;}; bool is_ok;} ICU4XCalendar_create_for_kind_result;
    ICU4XCalendar_create_for_kind_result ICU4XCalendar_create_for_kind(const diplomat::capi::DataProvider* provider, diplomat::capi::AnyCalendarKind kind);
    
    diplomat::capi::AnyCalendarKind ICU4XCalendar_kind(const diplomat::capi::Calendar* self);
    
    
    void ICU4XCalendar_destroy(Calendar* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<Calendar>, DataError> Calendar::create_for_locale(const DataProvider& provider, const Locale& locale) {
  auto result = diplomat::capi::ICU4XCalendar_create_for_locale(provider.AsFFI(),
    locale.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<Calendar>, DataError>(diplomat::Ok<std::unique_ptr<Calendar>>(std::unique_ptr<Calendar>(Calendar::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<Calendar>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<Calendar>, DataError> Calendar::create_for_kind(const DataProvider& provider, AnyCalendarKind kind) {
  auto result = diplomat::capi::ICU4XCalendar_create_for_kind(provider.AsFFI(),
    kind.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<Calendar>, DataError>(diplomat::Ok<std::unique_ptr<Calendar>>(std::unique_ptr<Calendar>(Calendar::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<Calendar>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline AnyCalendarKind Calendar::kind() const {
  auto result = diplomat::capi::ICU4XCalendar_kind(this->AsFFI());
  return AnyCalendarKind::FromFFI(result);
}

inline const diplomat::capi::Calendar* Calendar::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::Calendar*>(this);
}

inline diplomat::capi::Calendar* Calendar::AsFFI() {
  return reinterpret_cast<diplomat::capi::Calendar*>(this);
}

inline const Calendar* Calendar::FromFFI(const diplomat::capi::Calendar* ptr) {
  return reinterpret_cast<const Calendar*>(ptr);
}

inline Calendar* Calendar::FromFFI(diplomat::capi::Calendar* ptr) {
  return reinterpret_cast<Calendar*>(ptr);
}

inline void Calendar::operator delete(void* ptr) {
  diplomat::capi::ICU4XCalendar_destroy(reinterpret_cast<diplomat::capi::Calendar*>(ptr));
}


#endif // Calendar_HPP