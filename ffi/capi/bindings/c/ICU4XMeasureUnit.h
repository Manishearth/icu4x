#ifndef ICU4XMeasureUnit_H
#define ICU4XMeasureUnit_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataError.d.h"

#include "ICU4XMeasureUnit.d.h"






typedef struct icu4x_ICU4XMeasureUnit_deserialize_mv1_result {union {ICU4XMeasureUnit* ok; DataError err;}; bool is_ok;} icu4x_ICU4XMeasureUnit_deserialize_mv1_result;
icu4x_ICU4XMeasureUnit_deserialize_mv1_result icu4x_ICU4XMeasureUnit_deserialize_mv1(DiplomatStringView s);

void icu4x_ICU4XMeasureUnit_destroy_mv1(ICU4XMeasureUnit* self);





#endif // ICU4XMeasureUnit_H
