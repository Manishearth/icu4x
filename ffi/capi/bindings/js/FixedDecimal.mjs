// generated by diplomat-tool
import { FixedDecimalLimitError } from "./FixedDecimalLimitError.mjs"
import { FixedDecimalParseError } from "./FixedDecimalParseError.mjs"
import { FixedDecimalRoundingIncrement } from "./FixedDecimalRoundingIncrement.mjs"
import { FixedDecimalRoundingMode } from "./FixedDecimalRoundingMode.mjs"
import { FixedDecimalSign } from "./FixedDecimalSign.mjs"
import { FixedDecimalSignDisplay } from "./FixedDecimalSignDisplay.mjs"
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";


/** See the [Rust documentation for `FixedDecimal`](https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html) for more information.
*/
const FixedDecimal_box_destroy_registry = new FinalizationRegistry((ptr) => {
    wasm.icu4x_FixedDecimal_destroy_mv1(ptr);
});

export class FixedDecimal {
    // Internal ptr reference:
    #ptr = null;

    // Lifetimes are only to keep dependencies alive.
    // Since JS won't garbage collect until there are no incoming edges.
    #selfEdge = [];
    
    constructor(symbol, ptr, selfEdge) {
        if (symbol !== diplomatRuntime.internalConstructor) {
            console.error("FixedDecimal is an Opaque type. You cannot call its constructor.");
            return;
        }
        
        this.#ptr = ptr;
        this.#selfEdge = selfEdge;
        
        // Are we being borrowed? If not, we can register.
        if (this.#selfEdge.length === 0) {
            FixedDecimal_box_destroy_registry.register(this, this.#ptr);
        }
    }

    get ffiValue() {
        return this.#ptr;
    }

    static fromNumber(v) {
        const result = wasm.icu4x_FixedDecimal_from_int32_mv1(v);
    
        try {
            return new FixedDecimal(diplomatRuntime.internalConstructor, result, []);
        }
        
        finally {}
    }

    static fromBigInt(v) {
        const result = wasm.icu4x_FixedDecimal_from_int64_mv1(v);
    
        try {
            return new FixedDecimal(diplomatRuntime.internalConstructor, result, []);
        }
        
        finally {}
    }

    static fromNumberWithLowerMagnitude(f, magnitude) {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 5, 4, true);
        
        const result = wasm.icu4x_FixedDecimal_from_double_with_lower_magnitude_mv1(diplomatReceive.buffer, f, magnitude);
    
        try {
            if (!diplomatReceive.resultFlag) {
                const cause = new FixedDecimalLimitError(diplomatRuntime.internalConstructor);
                throw new globalThis.Error('FixedDecimalLimitError', { cause });
            }
            return new FixedDecimal(diplomatRuntime.internalConstructor, diplomatRuntime.ptrRead(wasm, diplomatReceive.buffer), []);
        }
        
        finally {
            diplomatReceive.free();
        }
    }

    static fromNumberWithSignificantDigits(f, digits) {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 5, 4, true);
        
        const result = wasm.icu4x_FixedDecimal_from_double_with_significant_digits_mv1(diplomatReceive.buffer, f, digits);
    
        try {
            if (!diplomatReceive.resultFlag) {
                const cause = new FixedDecimalLimitError(diplomatRuntime.internalConstructor);
                throw new globalThis.Error('FixedDecimalLimitError', { cause });
            }
            return new FixedDecimal(diplomatRuntime.internalConstructor, diplomatRuntime.ptrRead(wasm, diplomatReceive.buffer), []);
        }
        
        finally {
            diplomatReceive.free();
        }
    }

    static fromNumberWithRoundTripPrecision(f) {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 5, 4, true);
        
        const result = wasm.icu4x_FixedDecimal_from_double_with_round_trip_precision_mv1(diplomatReceive.buffer, f);
    
        try {
            if (!diplomatReceive.resultFlag) {
                const cause = new FixedDecimalLimitError(diplomatRuntime.internalConstructor);
                throw new globalThis.Error('FixedDecimalLimitError', { cause });
            }
            return new FixedDecimal(diplomatRuntime.internalConstructor, diplomatRuntime.ptrRead(wasm, diplomatReceive.buffer), []);
        }
        
        finally {
            diplomatReceive.free();
        }
    }

    static fromString(v) {
        let functionCleanupArena = new diplomatRuntime.CleanupArena();
        
        const vSlice = functionCleanupArena.alloc(diplomatRuntime.DiplomatBuf.str8(wasm, v));
        
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 5, 4, true);
        
        const result = wasm.icu4x_FixedDecimal_from_string_mv1(diplomatReceive.buffer, ...vSlice.splat());
    
        try {
            if (!diplomatReceive.resultFlag) {
                const cause = new FixedDecimalParseError(diplomatRuntime.internalConstructor, diplomatRuntime.enumDiscriminant(wasm, diplomatReceive.buffer));
                throw new globalThis.Error('FixedDecimalParseError: ' + cause.value, { cause });
            }
            return new FixedDecimal(diplomatRuntime.internalConstructor, diplomatRuntime.ptrRead(wasm, diplomatReceive.buffer), []);
        }
        
        finally {
            functionCleanupArena.free();
        
            diplomatReceive.free();
        }
    }

    digitAt(magnitude) {
        const result = wasm.icu4x_FixedDecimal_digit_at_mv1(this.ffiValue, magnitude);
    
        try {
            return result;
        }
        
        finally {}
    }

    get magnitudeStart() {
        const result = wasm.icu4x_FixedDecimal_magnitude_start_mv1(this.ffiValue);
    
        try {
            return result;
        }
        
        finally {}
    }

    get magnitudeEnd() {
        const result = wasm.icu4x_FixedDecimal_magnitude_end_mv1(this.ffiValue);
    
        try {
            return result;
        }
        
        finally {}
    }

    get nonzeroMagnitudeStart() {
        const result = wasm.icu4x_FixedDecimal_nonzero_magnitude_start_mv1(this.ffiValue);
    
        try {
            return result;
        }
        
        finally {}
    }

    get nonzeroMagnitudeEnd() {
        const result = wasm.icu4x_FixedDecimal_nonzero_magnitude_end_mv1(this.ffiValue);
    
        try {
            return result;
        }
        
        finally {}
    }

    get isZero() {
        const result = wasm.icu4x_FixedDecimal_is_zero_mv1(this.ffiValue);
    
        try {
            return result;
        }
        
        finally {}
    }

    multiplyPow10(power) {wasm.icu4x_FixedDecimal_multiply_pow10_mv1(this.ffiValue, power);
    
        try {}
        
        finally {}
    }

    get sign() {
        const result = wasm.icu4x_FixedDecimal_sign_mv1(this.ffiValue);
    
        try {
            return new FixedDecimalSign(diplomatRuntime.internalConstructor, result);
        }
        
        finally {}
    }

    set sign(sign) {wasm.icu4x_FixedDecimal_set_sign_mv1(this.ffiValue, sign.ffiValue);
    
        try {}
        
        finally {}
    }

    applySignDisplay(signDisplay) {wasm.icu4x_FixedDecimal_apply_sign_display_mv1(this.ffiValue, signDisplay.ffiValue);
    
        try {}
        
        finally {}
    }

    trimStart() {wasm.icu4x_FixedDecimal_trim_start_mv1(this.ffiValue);
    
        try {}
        
        finally {}
    }

    trimEnd() {wasm.icu4x_FixedDecimal_trim_end_mv1(this.ffiValue);
    
        try {}
        
        finally {}
    }

    padStart(position) {wasm.icu4x_FixedDecimal_pad_start_mv1(this.ffiValue, position);
    
        try {}
        
        finally {}
    }

    padEnd(position) {wasm.icu4x_FixedDecimal_pad_end_mv1(this.ffiValue, position);
    
        try {}
        
        finally {}
    }

    setMaxPosition(position) {wasm.icu4x_FixedDecimal_set_max_position_mv1(this.ffiValue, position);
    
        try {}
        
        finally {}
    }

    round(position) {wasm.icu4x_FixedDecimal_round_mv1(this.ffiValue, position);
    
        try {}
        
        finally {}
    }

    ceil(position) {wasm.icu4x_FixedDecimal_ceil_mv1(this.ffiValue, position);
    
        try {}
        
        finally {}
    }

    expand(position) {wasm.icu4x_FixedDecimal_expand_mv1(this.ffiValue, position);
    
        try {}
        
        finally {}
    }

    floor(position) {wasm.icu4x_FixedDecimal_floor_mv1(this.ffiValue, position);
    
        try {}
        
        finally {}
    }

    trunc(position) {wasm.icu4x_FixedDecimal_trunc_mv1(this.ffiValue, position);
    
        try {}
        
        finally {}
    }

    roundWithMode(position, mode) {wasm.icu4x_FixedDecimal_round_with_mode_mv1(this.ffiValue, position, mode.ffiValue);
    
        try {}
        
        finally {}
    }

    roundWithModeAndIncrement(position, mode, increment) {wasm.icu4x_FixedDecimal_round_with_mode_and_increment_mv1(this.ffiValue, position, mode.ffiValue, increment.ffiValue);
    
        try {}
        
        finally {}
    }

    concatenateEnd(other) {
        const result = wasm.icu4x_FixedDecimal_concatenate_end_mv1(this.ffiValue, other.ffiValue);
    
        try {
            return result === 1;
        }
        
        finally {}
    }

    toString() {
        const write = new diplomatRuntime.DiplomatWriteBuf(wasm);
        wasm.icu4x_FixedDecimal_to_string_mv1(this.ffiValue, write.buffer);
    
        try {
            return write.readString8();
        }
        
        finally {
            write.free();
        }
    }
}