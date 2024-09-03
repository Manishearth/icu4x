// generated by diplomat-tool
import { DataError } from "./DataError.mjs"
import { DataProvider } from "./DataProvider.mjs"
import { SentenceBreakIteratorUtf16 } from "./SentenceBreakIteratorUtf16.mjs"
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";


/** An ICU4X sentence-break segmenter, capable of finding sentence breakpoints in strings.
*
*See the [Rust documentation for `SentenceSegmenter`](https://docs.rs/icu/latest/icu/segmenter/struct.SentenceSegmenter.html) for more information.
*/
const SentenceSegmenter_box_destroy_registry = new FinalizationRegistry((ptr) => {
    wasm.icu4x_SentenceSegmenter_destroy_mv1(ptr);
});

export class SentenceSegmenter {
    // Internal ptr reference:
    #ptr = null;

    // Lifetimes are only to keep dependencies alive.
    // Since JS won't garbage collect until there are no incoming edges.
    #selfEdge = [];
    
    constructor(symbol, ptr, selfEdge) {
        if (symbol !== diplomatRuntime.internalConstructor) {
            console.error("SentenceSegmenter is an Opaque type. You cannot call its constructor.");
            return;
        }
        
        this.#ptr = ptr;
        this.#selfEdge = selfEdge;
        
        // Are we being borrowed? If not, we can register.
        if (this.#selfEdge.length === 0) {
            SentenceSegmenter_box_destroy_registry.register(this, this.#ptr);
        }
    }

    get ffiValue() {
        return this.#ptr;
    }

    static create(provider) {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 5, 4, true);
        
        const result = wasm.icu4x_SentenceSegmenter_create_mv1(diplomatReceive.buffer, provider.ffiValue);
    
        try {
            if (!diplomatReceive.resultFlag) {
                const cause = new DataError(diplomatRuntime.internalConstructor, diplomatRuntime.enumDiscriminant(wasm, diplomatReceive.buffer));
                throw new globalThis.Error('DataError: ' + cause.value, { cause });
            }
            return new SentenceSegmenter(diplomatRuntime.internalConstructor, diplomatRuntime.ptrRead(wasm, diplomatReceive.buffer), []);
        }
        
        finally {
            diplomatReceive.free();
        }
    }

    segment(input) {
        let functionGarbageCollectorGrip = new diplomatRuntime.GarbageCollectorGrip();
        const inputSlice = functionGarbageCollectorGrip.alloc(diplomatRuntime.DiplomatBuf.str16(wasm, input));
        
        // This lifetime edge depends on lifetimes 'a
        let aEdges = [this, inputSlice];
        
        const result = wasm.icu4x_SentenceSegmenter_segment_utf16_mv1(this.ffiValue, ...inputSlice.splat());
    
        try {
            return new SentenceBreakIteratorUtf16(diplomatRuntime.internalConstructor, result, [], aEdges);
        }
        
        finally {
            functionGarbageCollectorGrip.releaseToGarbageCollector();
        }
    }
}