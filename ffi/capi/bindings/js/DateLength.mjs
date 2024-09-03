// generated by diplomat-tool
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";

// Base enumerator definition
/** See the [Rust documentation for `Date`](https://docs.rs/icu/latest/icu/datetime/options/length/enum.Date.html) for more information.
*/
export class DateLength {
    #value = undefined;

    static #values = new Map([
        ["Full", 0],
        ["Long", 1],
        ["Medium", 2],
        ["Short", 3]
    ]);

    constructor(value) {
        if (arguments.length > 1 && arguments[0] === diplomatRuntime.internalConstructor) {
            // We pass in two internalConstructor arguments to create *new*
            // instances of this type, otherwise the enums are treated as singletons.
            if (arguments[1] === diplomatRuntime.internalConstructor ) {
                this.#value = arguments[2];
                return;
            }
            return DateLength.#objectValues[arguments[1]];
        }

        if (value instanceof DateLength) {
            return value;
        }

        let intVal = DateLength.#values.get(value);

        // Nullish check, checks for null or undefined
        if (intVal == null) {
            return DateLength.#objectValues[intVal];
        }

        throw TypeError(value + " is not a DateLength and does not correspond to any of its enumerator values.");
    }

    get value() {
        return [...DateLength.#values.keys()][this.#value];
    }

    get ffiValue() {
        return this.#value;
    }
    static #objectValues = [
        new DateLength(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 0),
        new DateLength(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 1),
        new DateLength(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 2),
        new DateLength(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 3),
    ];

    static Full = DateLength.#objectValues[0];
    static Long = DateLength.#objectValues[1];
    static Medium = DateLength.#objectValues[2];
    static Short = DateLength.#objectValues[3];
}