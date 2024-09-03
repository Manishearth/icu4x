// generated by diplomat-tool
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";

// Base enumerator definition
export class IsoWeekday {
    #value = undefined;

    static #values = new Map([
        ["Monday", 1],
        ["Tuesday", 2],
        ["Wednesday", 3],
        ["Thursday", 4],
        ["Friday", 5],
        ["Saturday", 6],
        ["Sunday", 7]
    ]);

    constructor(value) {
        if (arguments.length > 1 && arguments[0] === diplomatRuntime.internalConstructor) {
            // We pass in two internalConstructor arguments to create *new*
            // instances of this type, otherwise the enums are treated as singletons.
            if (arguments[1] === diplomatRuntime.internalConstructor ) {
                this.#value = arguments[2];
                return;
            }
            return IsoWeekday.#objectValues[arguments[1]];
        }

        if (value instanceof IsoWeekday) {
            return value;
        }

        let intVal = IsoWeekday.#values.get(value);

        // Nullish check, checks for null or undefined
        if (intVal == null) {
            return IsoWeekday.#objectValues[intVal];
        }

        throw TypeError(value + " is not a IsoWeekday and does not correspond to any of its enumerator values.");
    }

    get value() {
        for (let entry of IsoWeekday.#values) {
            if (entry[1] == this.#value) {
                return entry[0];
            }
        }
    }

    get ffiValue() {
        return this.#value;
    }
    static #objectValues = {
        [1]: new IsoWeekday(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 1),
        [2]: new IsoWeekday(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 2),
        [3]: new IsoWeekday(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 3),
        [4]: new IsoWeekday(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 4),
        [5]: new IsoWeekday(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 5),
        [6]: new IsoWeekday(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 6),
        [7]: new IsoWeekday(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 7),
    };

    static Monday = IsoWeekday.#objectValues[1];
    static Tuesday = IsoWeekday.#objectValues[2];
    static Wednesday = IsoWeekday.#objectValues[3];
    static Thursday = IsoWeekday.#objectValues[4];
    static Friday = IsoWeekday.#objectValues[5];
    static Saturday = IsoWeekday.#objectValues[6];
    static Sunday = IsoWeekday.#objectValues[7];
}