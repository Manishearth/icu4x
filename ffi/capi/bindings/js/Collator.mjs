// generated by diplomat-tool
import { CollatorOptions } from "./CollatorOptions.mjs"
import { CollatorResolvedOptions } from "./CollatorResolvedOptions.mjs"
import { DataError } from "./DataError.mjs"
import { DataProvider } from "./DataProvider.mjs"
import { Locale } from "./Locale.mjs"
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";


/** See the [Rust documentation for `Collator`](https://docs.rs/icu/latest/icu/collator/struct.Collator.html) for more information.
*/
const Collator_box_destroy_registry = new FinalizationRegistry((ptr) => {
    wasm.icu4x_Collator_destroy_mv1(ptr);
});

export class Collator {
    // Internal ptr reference:
    #ptr = null;

    // Lifetimes are only to keep dependencies alive.
    // Since JS won't garbage collect until there are no incoming edges.
    #selfEdge = [];
    
    constructor(symbol, ptr, selfEdge) {
        if (symbol !== diplomatRuntime.internalConstructor) {
            console.error("Collator is an Opaque type. You cannot call its constructor.");
            return;
        }
        
        this.#ptr = ptr;
        this.#selfEdge = selfEdge;
        
        // Are we being borrowed? If not, we can register.
        if (this.#selfEdge.length === 0) {
            Collator_box_destroy_registry.register(this, this.#ptr);
        }
    }

    get ffiValue() {
        return this.#ptr;
    }

    static create(provider, locale, options) {
        let functionCleanupArena = new diplomatRuntime.CleanupArena();
        
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 5, 4, true);
        
        const result = wasm.icu4x_Collator_create_v1_mv1(diplomatReceive.buffer, provider.ffiValue, locale.ffiValue, ...options._intoFFI(functionCleanupArena, {}));
    
        try {
            if (!diplomatReceive.resultFlag) {
                const cause = new DataError(diplomatRuntime.internalConstructor, diplomatRuntime.enumDiscriminant(wasm, diplomatReceive.buffer));
                throw new globalThis.Error('DataError: ' + cause.value, { cause });
            }
            return new Collator(diplomatRuntime.internalConstructor, diplomatRuntime.ptrRead(wasm, diplomatReceive.buffer), []);
        }
        
        finally {
            functionCleanupArena.free();
        
            diplomatReceive.free();
        }
    }

    compare(left, right) {
        let functionCleanupArena = new diplomatRuntime.CleanupArena();
        
        const leftSlice = functionCleanupArena.alloc(diplomatRuntime.DiplomatBuf.str16(wasm, left));
        
        const rightSlice = functionCleanupArena.alloc(diplomatRuntime.DiplomatBuf.str16(wasm, right));
        
        const result = wasm.icu4x_Collator_compare_utf16_mv1(this.ffiValue, ...leftSlice.splat(), ...rightSlice.splat());
    
        try {
            return result;
        }
        
        finally {
            functionCleanupArena.free();
        }
    }

    get resolvedOptions() {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 28, 4, false);
        
        const result = wasm.icu4x_Collator_resolved_options_v1_mv1(diplomatReceive.buffer, this.ffiValue);
    
        try {
            return new CollatorResolvedOptions(diplomatRuntime.internalConstructor, diplomatReceive.buffer);
        }
        
        finally {
            diplomatReceive.free();
        }
    }
}