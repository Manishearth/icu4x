// generated by diplomat-tool
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";

// Base enumerator definition
export class BidiDirection {
    #value = undefined;

    static #values = new Map([
        ["Ltr", 0],
        ["Rtl", 1],
        ["Mixed", 2]
    ]);

    constructor(value) {
        if (arguments.length > 1 && arguments[0] === diplomatRuntime.internalConstructor) {
            // We pass in two internalConstructor arguments to create *new*
            // instances of this type, otherwise the enums are treated as singletons.
            if (arguments[1] === diplomatRuntime.internalConstructor ) {
                this.#value = arguments[2];
                return;
            }
            return BidiDirection.#objectValues[arguments[1]];
        }

        if (value instanceof BidiDirection) {
            return value;
        }

        let intVal = BidiDirection.#values.get(value);

        // Nullish check, checks for null or undefined
        if (intVal == null) {
            return BidiDirection.#objectValues[intVal];
        }

        throw TypeError(value + " is not a BidiDirection and does not correspond to any of its enumerator values.");
    }

    get value() {
        return [...BidiDirection.#values.keys()][this.#value];
    }

    get ffiValue() {
        return this.#value;
    }
    static #objectValues = [
        new BidiDirection(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 0),
        new BidiDirection(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 1),
        new BidiDirection(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 2),
    ];

    static Ltr = BidiDirection.#objectValues[0];
    static Rtl = BidiDirection.#objectValues[1];
    static Mixed = BidiDirection.#objectValues[2];
}