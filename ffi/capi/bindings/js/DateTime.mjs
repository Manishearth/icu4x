// generated by diplomat-tool
import { Calendar } from "./Calendar.mjs"
import { CalendarError } from "./CalendarError.mjs"
import { CalendarParseError } from "./CalendarParseError.mjs"
import { Date } from "./Date.mjs"
import { IsoDateTime } from "./IsoDateTime.mjs"
import { IsoWeekday } from "./IsoWeekday.mjs"
import { Time } from "./Time.mjs"
import { WeekCalculator } from "./WeekCalculator.mjs"
import { WeekOf } from "./WeekOf.mjs"
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";


/** An ICU4X DateTime object capable of containing a date and time for any calendar.
*
*See the [Rust documentation for `DateTime`](https://docs.rs/icu/latest/icu/calendar/struct.DateTime.html) for more information.
*/
const DateTime_box_destroy_registry = new FinalizationRegistry((ptr) => {
    wasm.icu4x_DateTime_destroy_mv1(ptr);
});

export class DateTime {
    // Internal ptr reference:
    #ptr = null;

    // Lifetimes are only to keep dependencies alive.
    // Since JS won't garbage collect until there are no incoming edges.
    #selfEdge = [];
    
    constructor(symbol, ptr, selfEdge) {
        if (symbol !== diplomatRuntime.internalConstructor) {
            console.error("DateTime is an Opaque type. You cannot call its constructor.");
            return;
        }
        
        this.#ptr = ptr;
        this.#selfEdge = selfEdge;
        
        // Are we being borrowed? If not, we can register.
        if (this.#selfEdge.length === 0) {
            DateTime_box_destroy_registry.register(this, this.#ptr);
        }
    }

    get ffiValue() {
        return this.#ptr;
    }

    static fromIsoInCalendar(year, month, day, hour, minute, second, nanosecond, calendar) {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 5, 4, true);
        
        const result = wasm.icu4x_DateTime_from_iso_in_calendar_mv1(diplomatReceive.buffer, year, month, day, hour, minute, second, nanosecond, calendar.ffiValue);
    
        try {
            if (!diplomatReceive.resultFlag) {
                const cause = new CalendarError(diplomatRuntime.internalConstructor, diplomatRuntime.enumDiscriminant(wasm, diplomatReceive.buffer));
                throw new globalThis.Error('CalendarError: ' + cause.value, { cause });
            }
            return new DateTime(diplomatRuntime.internalConstructor, diplomatRuntime.ptrRead(wasm, diplomatReceive.buffer), []);
        }
        
        finally {
            diplomatReceive.free();
        }
    }

    static fromCodesInCalendar(eraCode, year, monthCode, day, hour, minute, second, nanosecond, calendar) {
        let functionCleanupArena = new diplomatRuntime.CleanupArena();
        
        const eraCodeSlice = functionCleanupArena.alloc(diplomatRuntime.DiplomatBuf.str8(wasm, eraCode));
        
        const monthCodeSlice = functionCleanupArena.alloc(diplomatRuntime.DiplomatBuf.str8(wasm, monthCode));
        
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 5, 4, true);
        
        const result = wasm.icu4x_DateTime_from_codes_in_calendar_mv1(diplomatReceive.buffer, ...eraCodeSlice.splat(), year, ...monthCodeSlice.splat(), day, hour, minute, second, nanosecond, calendar.ffiValue);
    
        try {
            if (!diplomatReceive.resultFlag) {
                const cause = new CalendarError(diplomatRuntime.internalConstructor, diplomatRuntime.enumDiscriminant(wasm, diplomatReceive.buffer));
                throw new globalThis.Error('CalendarError: ' + cause.value, { cause });
            }
            return new DateTime(diplomatRuntime.internalConstructor, diplomatRuntime.ptrRead(wasm, diplomatReceive.buffer), []);
        }
        
        finally {
            functionCleanupArena.free();
        
            diplomatReceive.free();
        }
    }

    static fromDateAndTime(date, time) {
        const result = wasm.icu4x_DateTime_from_date_and_time_mv1(date.ffiValue, time.ffiValue);
    
        try {
            return new DateTime(diplomatRuntime.internalConstructor, result, []);
        }
        
        finally {}
    }

    static fromString(v) {
        let functionCleanupArena = new diplomatRuntime.CleanupArena();
        
        const vSlice = functionCleanupArena.alloc(diplomatRuntime.DiplomatBuf.str8(wasm, v));
        
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 5, 4, true);
        
        const result = wasm.icu4x_DateTime_from_string_mv1(diplomatReceive.buffer, ...vSlice.splat());
    
        try {
            if (!diplomatReceive.resultFlag) {
                const cause = new CalendarParseError(diplomatRuntime.internalConstructor, diplomatRuntime.enumDiscriminant(wasm, diplomatReceive.buffer));
                throw new globalThis.Error('CalendarParseError: ' + cause.value, { cause });
            }
            return new DateTime(diplomatRuntime.internalConstructor, diplomatRuntime.ptrRead(wasm, diplomatReceive.buffer), []);
        }
        
        finally {
            functionCleanupArena.free();
        
            diplomatReceive.free();
        }
    }

    get date() {
        const result = wasm.icu4x_DateTime_date_mv1(this.ffiValue);
    
        try {
            return new Date(diplomatRuntime.internalConstructor, result, []);
        }
        
        finally {}
    }

    get time() {
        const result = wasm.icu4x_DateTime_time_mv1(this.ffiValue);
    
        try {
            return new Time(diplomatRuntime.internalConstructor, result, []);
        }
        
        finally {}
    }

    toIso() {
        const result = wasm.icu4x_DateTime_to_iso_mv1(this.ffiValue);
    
        try {
            return new IsoDateTime(diplomatRuntime.internalConstructor, result, []);
        }
        
        finally {}
    }

    toCalendar(calendar) {
        const result = wasm.icu4x_DateTime_to_calendar_mv1(this.ffiValue, calendar.ffiValue);
    
        try {
            return new DateTime(diplomatRuntime.internalConstructor, result, []);
        }
        
        finally {}
    }

    get hour() {
        const result = wasm.icu4x_DateTime_hour_mv1(this.ffiValue);
    
        try {
            return result;
        }
        
        finally {}
    }

    get minute() {
        const result = wasm.icu4x_DateTime_minute_mv1(this.ffiValue);
    
        try {
            return result;
        }
        
        finally {}
    }

    get second() {
        const result = wasm.icu4x_DateTime_second_mv1(this.ffiValue);
    
        try {
            return result;
        }
        
        finally {}
    }

    get nanosecond() {
        const result = wasm.icu4x_DateTime_nanosecond_mv1(this.ffiValue);
    
        try {
            return result;
        }
        
        finally {}
    }

    get dayOfYear() {
        const result = wasm.icu4x_DateTime_day_of_year_mv1(this.ffiValue);
    
        try {
            return result;
        }
        
        finally {}
    }

    get dayOfMonth() {
        const result = wasm.icu4x_DateTime_day_of_month_mv1(this.ffiValue);
    
        try {
            return result;
        }
        
        finally {}
    }

    get dayOfWeek() {
        const result = wasm.icu4x_DateTime_day_of_week_mv1(this.ffiValue);
    
        try {
            return new IsoWeekday(diplomatRuntime.internalConstructor, result);
        }
        
        finally {}
    }

    weekOfMonth(firstWeekday) {
        const result = wasm.icu4x_DateTime_week_of_month_mv1(this.ffiValue, firstWeekday.ffiValue);
    
        try {
            return result;
        }
        
        finally {}
    }

    weekOfYear(calculator) {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 8, 4, false);
        
        const result = wasm.icu4x_DateTime_week_of_year_mv1(diplomatReceive.buffer, this.ffiValue, calculator.ffiValue);
    
        try {
            return new WeekOf(diplomatRuntime.internalConstructor, diplomatReceive.buffer);
        }
        
        finally {
            diplomatReceive.free();
        }
    }

    get ordinalMonth() {
        const result = wasm.icu4x_DateTime_ordinal_month_mv1(this.ffiValue);
    
        try {
            return result;
        }
        
        finally {}
    }

    get monthCode() {
        const write = new diplomatRuntime.DiplomatWriteBuf(wasm);
        wasm.icu4x_DateTime_month_code_mv1(this.ffiValue, write.buffer);
    
        try {
            return write.readString8();
        }
        
        finally {
            write.free();
        }
    }

    get yearInEra() {
        const result = wasm.icu4x_DateTime_year_in_era_mv1(this.ffiValue);
    
        try {
            return result;
        }
        
        finally {}
    }

    get era() {
        const write = new diplomatRuntime.DiplomatWriteBuf(wasm);
        wasm.icu4x_DateTime_era_mv1(this.ffiValue, write.buffer);
    
        try {
            return write.readString8();
        }
        
        finally {
            write.free();
        }
    }

    get monthsInYear() {
        const result = wasm.icu4x_DateTime_months_in_year_mv1(this.ffiValue);
    
        try {
            return result;
        }
        
        finally {}
    }

    get daysInMonth() {
        const result = wasm.icu4x_DateTime_days_in_month_mv1(this.ffiValue);
    
        try {
            return result;
        }
        
        finally {}
    }

    get daysInYear() {
        const result = wasm.icu4x_DateTime_days_in_year_mv1(this.ffiValue);
    
        try {
            return result;
        }
        
        finally {}
    }

    get calendar() {
        const result = wasm.icu4x_DateTime_calendar_mv1(this.ffiValue);
    
        try {
            return new Calendar(diplomatRuntime.internalConstructor, result, []);
        }
        
        finally {}
    }
}