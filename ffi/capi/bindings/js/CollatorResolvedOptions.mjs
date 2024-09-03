// generated by diplomat-tool
import { CollatorAlternateHandling } from "./CollatorAlternateHandling.mjs"
import { CollatorBackwardSecondLevel } from "./CollatorBackwardSecondLevel.mjs"
import { CollatorCaseFirst } from "./CollatorCaseFirst.mjs"
import { CollatorCaseLevel } from "./CollatorCaseLevel.mjs"
import { CollatorMaxVariable } from "./CollatorMaxVariable.mjs"
import { CollatorNumeric } from "./CollatorNumeric.mjs"
import { CollatorStrength } from "./CollatorStrength.mjs"
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";


/** See the [Rust documentation for `ResolvedCollatorOptions`](https://docs.rs/icu/latest/icu/collator/struct.ResolvedCollatorOptions.html) for more information.
*/
export class CollatorResolvedOptions {

    #strength;
    get strength()  {
        return this.#strength;
    }
    

    #alternateHandling;
    get alternateHandling()  {
        return this.#alternateHandling;
    }
    

    #caseFirst;
    get caseFirst()  {
        return this.#caseFirst;
    }
    

    #maxVariable;
    get maxVariable()  {
        return this.#maxVariable;
    }
    

    #caseLevel;
    get caseLevel()  {
        return this.#caseLevel;
    }
    

    #numeric;
    get numeric()  {
        return this.#numeric;
    }
    

    #backwardSecondLevel;
    get backwardSecondLevel()  {
        return this.#backwardSecondLevel;
    }
    
    constructor() {
        if (arguments.length > 0 && arguments[0] === diplomatRuntime.internalConstructor) {
            this.#fromFFI(...Array.prototype.slice.call(arguments, 1));
        } else {
            console.error("CollatorResolvedOptions is an out struct and can only be created internally.");
        }
    }

    // Return this struct in FFI function friendly format.
    // Returns an array that can be expanded with spread syntax (...)
    
    _intoFFI(
        functionCleanupArena,
        appendArrayMap
    ) {
        return [this.#strength.ffiValue, this.#alternateHandling.ffiValue, this.#caseFirst.ffiValue, this.#maxVariable.ffiValue, this.#caseLevel.ffiValue, this.#numeric.ffiValue, this.#backwardSecondLevel.ffiValue]
    }

    _writeToArrayBuffer(
        arrayBuffer,
        offset,
        functionCleanupArena,
        appendArrayMap
    ) {
        diplomatRuntime.writeToArrayBuffer(arrayBuffer, offset + 0, this.#strength.ffiValue, Int32Array);
        diplomatRuntime.writeToArrayBuffer(arrayBuffer, offset + 4, this.#alternateHandling.ffiValue, Int32Array);
        diplomatRuntime.writeToArrayBuffer(arrayBuffer, offset + 8, this.#caseFirst.ffiValue, Int32Array);
        diplomatRuntime.writeToArrayBuffer(arrayBuffer, offset + 12, this.#maxVariable.ffiValue, Int32Array);
        diplomatRuntime.writeToArrayBuffer(arrayBuffer, offset + 16, this.#caseLevel.ffiValue, Int32Array);
        diplomatRuntime.writeToArrayBuffer(arrayBuffer, offset + 20, this.#numeric.ffiValue, Int32Array);
        diplomatRuntime.writeToArrayBuffer(arrayBuffer, offset + 24, this.#backwardSecondLevel.ffiValue, Int32Array);
    }

    // This struct contains borrowed fields, so this takes in a list of
    // "edges" corresponding to where each lifetime's data may have been borrowed from
    // and passes it down to individual fields containing the borrow.
    // This method does not attempt to handle any dependencies between lifetimes, the caller
    // should handle this when constructing edge arrays.
    #fromFFI(ptr) {
        const strengthDeref = diplomatRuntime.enumDiscriminant(wasm, ptr);
        this.#strength = new CollatorStrength(diplomatRuntime.internalConstructor, strengthDeref);
        const alternateHandlingDeref = diplomatRuntime.enumDiscriminant(wasm, ptr + 4);
        this.#alternateHandling = new CollatorAlternateHandling(diplomatRuntime.internalConstructor, alternateHandlingDeref);
        const caseFirstDeref = diplomatRuntime.enumDiscriminant(wasm, ptr + 8);
        this.#caseFirst = new CollatorCaseFirst(diplomatRuntime.internalConstructor, caseFirstDeref);
        const maxVariableDeref = diplomatRuntime.enumDiscriminant(wasm, ptr + 12);
        this.#maxVariable = new CollatorMaxVariable(diplomatRuntime.internalConstructor, maxVariableDeref);
        const caseLevelDeref = diplomatRuntime.enumDiscriminant(wasm, ptr + 16);
        this.#caseLevel = new CollatorCaseLevel(diplomatRuntime.internalConstructor, caseLevelDeref);
        const numericDeref = diplomatRuntime.enumDiscriminant(wasm, ptr + 20);
        this.#numeric = new CollatorNumeric(diplomatRuntime.internalConstructor, numericDeref);
        const backwardSecondLevelDeref = diplomatRuntime.enumDiscriminant(wasm, ptr + 24);
        this.#backwardSecondLevel = new CollatorBackwardSecondLevel(diplomatRuntime.internalConstructor, backwardSecondLevelDeref);
    }
}