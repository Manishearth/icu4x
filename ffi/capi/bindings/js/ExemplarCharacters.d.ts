// generated by diplomat-tool
import type { DataError } from "./DataError"
import type { DataProvider } from "./DataProvider"
import type { Locale } from "./Locale"
import type { pointer, codepoint } from "./diplomat-runtime.d.ts";


/** An ICU4X Unicode Set Property object, capable of querying whether a code point is contained in a set based on a Unicode property.
*
*See the [Rust documentation for `locale`](https://docs.rs/icu/latest/icu/locale/index.html) for more information.
*
*See the [Rust documentation for `ExemplarCharacters`](https://docs.rs/icu/latest/icu/locale/exemplar_chars/struct.ExemplarCharacters.html) for more information.
*
*See the [Rust documentation for `ExemplarCharactersBorrowed`](https://docs.rs/icu/latest/icu/locale/exemplar_chars/struct.ExemplarCharactersBorrowed.html) for more information.
*/
export class ExemplarCharacters {
    

    get ffiValue(): pointer;

    contains(s: string): boolean;

    containsChar(cp: codepoint): boolean;

    static tryNewMain(provider: DataProvider, locale: Locale): ExemplarCharacters;

    static tryNewAuxiliary(provider: DataProvider, locale: Locale): ExemplarCharacters;

    static tryNewPunctuation(provider: DataProvider, locale: Locale): ExemplarCharacters;

    static tryNewNumbers(provider: DataProvider, locale: Locale): ExemplarCharacters;

    static tryNewIndex(provider: DataProvider, locale: Locale): ExemplarCharacters;
}