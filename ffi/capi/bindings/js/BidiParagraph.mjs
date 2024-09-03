// generated by diplomat-tool
import { BidiDirection } from "./BidiDirection.mjs"
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";


/** Bidi information for a single processed paragraph
*/
const BidiParagraph_box_destroy_registry = new FinalizationRegistry((ptr) => {
    wasm.icu4x_BidiParagraph_destroy_mv1(ptr);
});

export class BidiParagraph {
    // Internal ptr reference:
    #ptr = null;

    // Lifetimes are only to keep dependencies alive.
    // Since JS won't garbage collect until there are no incoming edges.
    #selfEdge = [];
    #infoEdge = [];
    
    constructor(symbol, ptr, selfEdge, infoEdge) {
        if (symbol !== diplomatRuntime.internalConstructor) {
            console.error("BidiParagraph is an Opaque type. You cannot call its constructor.");
            return;
        }
        
        
        this.#infoEdge = infoEdge;
        
        this.#ptr = ptr;
        this.#selfEdge = selfEdge;
        
        // Are we being borrowed? If not, we can register.
        if (this.#selfEdge.length === 0) {
            BidiParagraph_box_destroy_registry.register(this, this.#ptr);
        }
    }

    get ffiValue() {
        return this.#ptr;
    }

    setParagraphInText(n) {
        const result = wasm.icu4x_BidiParagraph_set_paragraph_in_text_mv1(this.ffiValue, n);
    
        try {
            return result;
        }
        
        finally {}
    }

    get direction() {
        const result = wasm.icu4x_BidiParagraph_direction_mv1(this.ffiValue);
    
        try {
            return new BidiDirection(diplomatRuntime.internalConstructor, result);
        }
        
        finally {}
    }

    get size() {
        const result = wasm.icu4x_BidiParagraph_size_mv1(this.ffiValue);
    
        try {
            return result;
        }
        
        finally {}
    }

    get rangeStart() {
        const result = wasm.icu4x_BidiParagraph_range_start_mv1(this.ffiValue);
    
        try {
            return result;
        }
        
        finally {}
    }

    get rangeEnd() {
        const result = wasm.icu4x_BidiParagraph_range_end_mv1(this.ffiValue);
    
        try {
            return result;
        }
        
        finally {}
    }

    reorderLine(rangeStart, rangeEnd) {
        const write = new diplomatRuntime.DiplomatWriteBuf(wasm);
        
        const result = wasm.icu4x_BidiParagraph_reorder_line_mv1(this.ffiValue, rangeStart, rangeEnd, write.buffer);
    
        try {
            return result === 0 ? null : write.readString8();
        }
        
        finally {
            write.free();
        }
    }

    levelAt(pos) {
        const result = wasm.icu4x_BidiParagraph_level_at_mv1(this.ffiValue, pos);
    
        try {
            return result;
        }
        
        finally {}
    }
}