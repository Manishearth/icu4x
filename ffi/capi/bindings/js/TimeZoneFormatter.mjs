// generated by diplomat-tool
import { CustomTimeZone } from "./CustomTimeZone.mjs"
import { DataProvider } from "./DataProvider.mjs"
import { Error } from "./Error.mjs"
import { IsoTimeZoneOptions } from "./IsoTimeZoneOptions.mjs"
import { Locale } from "./Locale.mjs"
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";


/** An ICU4X TimeZoneFormatter object capable of formatting an [`CustomTimeZone`] type (and others) as a string
*
*See the [Rust documentation for `TimeZoneFormatter`](https://docs.rs/icu/latest/icu/datetime/time_zone/struct.TimeZoneFormatter.html) for more information.
*/
const TimeZoneFormatter_box_destroy_registry = new FinalizationRegistry((ptr) => {
    wasm.icu4x_TimeZoneFormatter_destroy_mv1(ptr);
});

export class TimeZoneFormatter {
    // Internal ptr reference:
    #ptr = null;

    // Lifetimes are only to keep dependencies alive.
    // Since JS won't garbage collect until there are no incoming edges.
    #selfEdge = [];
    
    constructor(symbol, ptr, selfEdge) {
        if (symbol !== diplomatRuntime.internalConstructor) {
            console.error("TimeZoneFormatter is an Opaque type. You cannot call its constructor.");
            return;
        }
        
        this.#ptr = ptr;
        this.#selfEdge = selfEdge;
        
        // Are we being borrowed? If not, we can register.
        if (this.#selfEdge.length === 0) {
            TimeZoneFormatter_box_destroy_registry.register(this, this.#ptr);
        }
    }

    get ffiValue() {
        return this.#ptr;
    }

    static createWithLocalizedGmtFallback(provider, locale) {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 5, 4, true);
        
        const result = wasm.icu4x_TimeZoneFormatter_create_with_localized_gmt_fallback_mv1(diplomatReceive.buffer, provider.ffiValue, locale.ffiValue);
    
        try {
            if (!diplomatReceive.resultFlag) {
                const cause = new Error(diplomatRuntime.internalConstructor, diplomatRuntime.enumDiscriminant(wasm, diplomatReceive.buffer));
                throw new globalThis.Error('Error: ' + cause.value, { cause });
            }
            return new TimeZoneFormatter(diplomatRuntime.internalConstructor, diplomatRuntime.ptrRead(wasm, diplomatReceive.buffer), []);
        }
        
        finally {
            diplomatReceive.free();
        }
    }

    static createWithIso8601Fallback(provider, locale, options) {
        let functionCleanupArena = new diplomatRuntime.CleanupArena();
        
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 5, 4, true);
        
        const result = wasm.icu4x_TimeZoneFormatter_create_with_iso_8601_fallback_mv1(diplomatReceive.buffer, provider.ffiValue, locale.ffiValue, ...options._intoFFI(functionCleanupArena, {}));
    
        try {
            if (!diplomatReceive.resultFlag) {
                const cause = new Error(diplomatRuntime.internalConstructor, diplomatRuntime.enumDiscriminant(wasm, diplomatReceive.buffer));
                throw new globalThis.Error('Error: ' + cause.value, { cause });
            }
            return new TimeZoneFormatter(diplomatRuntime.internalConstructor, diplomatRuntime.ptrRead(wasm, diplomatReceive.buffer), []);
        }
        
        finally {
            functionCleanupArena.free();
        
            diplomatReceive.free();
        }
    }

    loadGenericNonLocationLong(provider) {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 5, 4, true);
        
        const result = wasm.icu4x_TimeZoneFormatter_load_generic_non_location_long_mv1(diplomatReceive.buffer, this.ffiValue, provider.ffiValue);
    
        try {
            if (!diplomatReceive.resultFlag) {
                const cause = new Error(diplomatRuntime.internalConstructor, diplomatRuntime.enumDiscriminant(wasm, diplomatReceive.buffer));
                throw new globalThis.Error('Error: ' + cause.value, { cause });
            }
    
        }
        
        finally {
            diplomatReceive.free();
        }
    }

    loadGenericNonLocationShort(provider) {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 5, 4, true);
        
        const result = wasm.icu4x_TimeZoneFormatter_load_generic_non_location_short_mv1(diplomatReceive.buffer, this.ffiValue, provider.ffiValue);
    
        try {
            if (!diplomatReceive.resultFlag) {
                const cause = new Error(diplomatRuntime.internalConstructor, diplomatRuntime.enumDiscriminant(wasm, diplomatReceive.buffer));
                throw new globalThis.Error('Error: ' + cause.value, { cause });
            }
    
        }
        
        finally {
            diplomatReceive.free();
        }
    }

    loadSpecificNonLocationLong(provider) {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 5, 4, true);
        
        const result = wasm.icu4x_TimeZoneFormatter_load_specific_non_location_long_mv1(diplomatReceive.buffer, this.ffiValue, provider.ffiValue);
    
        try {
            if (!diplomatReceive.resultFlag) {
                const cause = new Error(diplomatRuntime.internalConstructor, diplomatRuntime.enumDiscriminant(wasm, diplomatReceive.buffer));
                throw new globalThis.Error('Error: ' + cause.value, { cause });
            }
    
        }
        
        finally {
            diplomatReceive.free();
        }
    }

    loadSpecificNonLocationShort(provider) {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 5, 4, true);
        
        const result = wasm.icu4x_TimeZoneFormatter_load_specific_non_location_short_mv1(diplomatReceive.buffer, this.ffiValue, provider.ffiValue);
    
        try {
            if (!diplomatReceive.resultFlag) {
                const cause = new Error(diplomatRuntime.internalConstructor, diplomatRuntime.enumDiscriminant(wasm, diplomatReceive.buffer));
                throw new globalThis.Error('Error: ' + cause.value, { cause });
            }
    
        }
        
        finally {
            diplomatReceive.free();
        }
    }

    loadGenericLocationFormat(provider) {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 5, 4, true);
        
        const result = wasm.icu4x_TimeZoneFormatter_load_generic_location_format_mv1(diplomatReceive.buffer, this.ffiValue, provider.ffiValue);
    
        try {
            if (!diplomatReceive.resultFlag) {
                const cause = new Error(diplomatRuntime.internalConstructor, diplomatRuntime.enumDiscriminant(wasm, diplomatReceive.buffer));
                throw new globalThis.Error('Error: ' + cause.value, { cause });
            }
    
        }
        
        finally {
            diplomatReceive.free();
        }
    }

    includeLocalizedGmtFormat() {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 5, 4, true);
        
        const result = wasm.icu4x_TimeZoneFormatter_include_localized_gmt_format_mv1(diplomatReceive.buffer, this.ffiValue);
    
        try {
            if (!diplomatReceive.resultFlag) {
                const cause = new Error(diplomatRuntime.internalConstructor, diplomatRuntime.enumDiscriminant(wasm, diplomatReceive.buffer));
                throw new globalThis.Error('Error: ' + cause.value, { cause });
            }
    
        }
        
        finally {
            diplomatReceive.free();
        }
    }

    loadIso8601Format(options) {
        let functionCleanupArena = new diplomatRuntime.CleanupArena();
        
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 5, 4, true);
        
        const result = wasm.icu4x_TimeZoneFormatter_load_iso_8601_format_mv1(diplomatReceive.buffer, this.ffiValue, ...options._intoFFI(functionCleanupArena, {}));
    
        try {
            if (!diplomatReceive.resultFlag) {
                const cause = new Error(diplomatRuntime.internalConstructor, diplomatRuntime.enumDiscriminant(wasm, diplomatReceive.buffer));
                throw new globalThis.Error('Error: ' + cause.value, { cause });
            }
    
        }
        
        finally {
            functionCleanupArena.free();
        
            diplomatReceive.free();
        }
    }

    formatCustomTimeZone(value) {
        const write = new diplomatRuntime.DiplomatWriteBuf(wasm);
        wasm.icu4x_TimeZoneFormatter_format_custom_time_zone_mv1(this.ffiValue, value.ffiValue, write.buffer);
    
        try {
            return write.readString8();
        }
        
        finally {
            write.free();
        }
    }

    formatCustomTimeZoneNoFallback(value) {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 5, 4, true);
        
        const write = new diplomatRuntime.DiplomatWriteBuf(wasm);
        
        const result = wasm.icu4x_TimeZoneFormatter_format_custom_time_zone_no_fallback_mv1(diplomatReceive.buffer, this.ffiValue, value.ffiValue, write.buffer);
    
        try {
            if (!diplomatReceive.resultFlag) {
                const cause = new Error(diplomatRuntime.internalConstructor, diplomatRuntime.enumDiscriminant(wasm, diplomatReceive.buffer));
                throw new globalThis.Error('Error: ' + cause.value, { cause });
            }
            return write.readString8();
        }
        
        finally {
            diplomatReceive.free();
        
            write.free();
        }
    }
}