// generated by diplomat-tool
import { DisplayNamesFallback } from "./DisplayNamesFallback.mjs"
import { DisplayNamesStyle } from "./DisplayNamesStyle.mjs"
import { LanguageDisplay } from "./LanguageDisplay.mjs"
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";


/** See the [Rust documentation for `DisplayNamesOptions`](https://docs.rs/icu/latest/icu/displaynames/options/struct.DisplayNamesOptions.html) for more information.
*/
export class DisplayNamesOptions {

    #style;
    get style()  {
        return this.#style;
    }
    set style(value) {
        this.#style = value;
    }

    #fallback;
    get fallback()  {
        return this.#fallback;
    }
    set fallback(value) {
        this.#fallback = value;
    }

    #languageDisplay;
    get languageDisplay()  {
        return this.#languageDisplay;
    }
    set languageDisplay(value) {
        this.#languageDisplay = value;
    }
    constructor() {
        if (arguments.length > 0 && arguments[0] === diplomatRuntime.internalConstructor) {
            this.#fromFFI(...Array.prototype.slice.call(arguments, 1));
        } else {
            
            this.#style = arguments[0];
            this.#fallback = arguments[1];
            this.#languageDisplay = arguments[2];
        }
    }

    // Return this struct in FFI function friendly format.
    // Returns an array that can be expanded with spread syntax (...)
    
    _intoFFI(
        functionCleanupArena,
        appendArrayMap
    ) {
        return [this.#style.ffiValue, this.#fallback.ffiValue, this.#languageDisplay.ffiValue]
    }

    // This struct contains borrowed fields, so this takes in a list of
    // "edges" corresponding to where each lifetime's data may have been borrowed from
    // and passes it down to individual fields containing the borrow.
    // This method does not attempt to handle any dependencies between lifetimes, the caller
    // should handle this when constructing edge arrays.
    #fromFFI(ptr) {
        const styleDeref = diplomatRuntime.enumDiscriminant(wasm, ptr);
        this.#style = DisplayNamesStyle[Array.from(DisplayNamesStyle.values.keys())[styleDeref]];
        const fallbackDeref = diplomatRuntime.enumDiscriminant(wasm, ptr + 4);
        this.#fallback = DisplayNamesFallback[Array.from(DisplayNamesFallback.values.keys())[fallbackDeref]];
        const languageDisplayDeref = diplomatRuntime.enumDiscriminant(wasm, ptr + 8);
        this.#languageDisplay = LanguageDisplay[Array.from(LanguageDisplay.values.keys())[languageDisplayDeref]];
    }
}