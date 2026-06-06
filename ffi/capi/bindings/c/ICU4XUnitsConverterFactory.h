#ifndef ICU4XUnitsConverterFactory_H
#define ICU4XUnitsConverterFactory_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataError.d.h"
#include "DataProvider.d.h"
#include "ICU4XMeasureUnit.d.h"
#include "ICU4XUnitsConverter.d.h"

#include "ICU4XUnitsConverterFactory.d.h"






ICU4XUnitsConverterFactory* icu4x_ICU4XUnitsConverterFactory_create_mv1(void);

typedef struct icu4x_ICU4XUnitsConverterFactory_create_with_provider_mv1_result {union {ICU4XUnitsConverterFactory* ok; DataError err;}; bool is_ok;} icu4x_ICU4XUnitsConverterFactory_create_with_provider_mv1_result;
icu4x_ICU4XUnitsConverterFactory_create_with_provider_mv1_result icu4x_ICU4XUnitsConverterFactory_create_with_provider_mv1(const DataProvider* provider);

ICU4XUnitsConverter* icu4x_ICU4XUnitsConverterFactory_converter_mv1(const ICU4XUnitsConverterFactory* self, const ICU4XMeasureUnit* input_unit, const ICU4XMeasureUnit* output_unit);

void icu4x_ICU4XUnitsConverterFactory_destroy_mv1(ICU4XUnitsConverterFactory* self);





#endif // ICU4XUnitsConverterFactory_H
