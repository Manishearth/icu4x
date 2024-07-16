import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
import { TimeZoneInvalidIdError_js_to_rust, TimeZoneInvalidIdError_rust_to_js } from "./TimeZoneInvalidIdError.mjs"
import { TimeZoneInvalidOffsetError_js_to_rust, TimeZoneInvalidOffsetError_rust_to_js } from "./TimeZoneInvalidOffsetError.mjs"

const CustomTimeZone_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XCustomTimeZone_destroy(underlying);
});

export class CustomTimeZone {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      CustomTimeZone_box_destroy_registry.register(this, underlying);
    }
  }

  static create_from_string(arg_s) {
    const buf_arg_s = diplomatRuntime.DiplomatBuf.str8(wasm, arg_s);
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XCustomTimeZone_create_from_string(diplomat_receive_buffer, buf_arg_s.ptr, buf_arg_s.size);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = new CustomTimeZone(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), true, []);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = TimeZoneInvalidOffsetError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
    buf_arg_s.free();
    return diplomat_out;
  }

  static create_empty() {
    return new CustomTimeZone(wasm.ICU4XCustomTimeZone_create_empty(), true, []);
  }

  static create_utc() {
    return new CustomTimeZone(wasm.ICU4XCustomTimeZone_create_utc(), true, []);
  }

  static create_gmt() {
    return new CustomTimeZone(wasm.ICU4XCustomTimeZone_create_gmt(), true, []);
  }

  static create_bst() {
    return new CustomTimeZone(wasm.ICU4XCustomTimeZone_create_bst(), true, []);
  }

  try_set_gmt_offset_seconds(arg_offset_seconds) {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XCustomTimeZone_try_set_gmt_offset_seconds(diplomat_receive_buffer, this.underlying, arg_offset_seconds);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = {};
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = TimeZoneInvalidOffsetError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
  }

  set_gmt_offset_eighths_of_hour(arg_offset_eighths_of_hour) {
    wasm.ICU4XCustomTimeZone_set_gmt_offset_eighths_of_hour(this.underlying, arg_offset_eighths_of_hour);
  }

  clear_gmt_offset() {
    wasm.ICU4XCustomTimeZone_clear_gmt_offset(this.underlying);
  }

  gmt_offset_seconds() {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XCustomTimeZone_gmt_offset_seconds(diplomat_receive_buffer, this.underlying);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (!is_ok) {
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return;
      }
      const value = (new Int32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0];
      wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
      return value;
    })();
  }

  is_gmt_offset_positive() {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(2, 1);
      wasm.ICU4XCustomTimeZone_is_gmt_offset_positive(diplomat_receive_buffer, this.underlying);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 1);
      if (!is_ok) {
        wasm.diplomat_free(diplomat_receive_buffer, 2, 1);
        return;
      }
      const value = (new Uint8Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0] == 1;
      wasm.diplomat_free(diplomat_receive_buffer, 2, 1);
      return value;
    })();
  }

  is_gmt_offset_zero() {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(2, 1);
      wasm.ICU4XCustomTimeZone_is_gmt_offset_zero(diplomat_receive_buffer, this.underlying);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 1);
      if (!is_ok) {
        wasm.diplomat_free(diplomat_receive_buffer, 2, 1);
        return;
      }
      const value = (new Uint8Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0] == 1;
      wasm.diplomat_free(diplomat_receive_buffer, 2, 1);
      return value;
    })();
  }

  gmt_offset_has_minutes() {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(2, 1);
      wasm.ICU4XCustomTimeZone_gmt_offset_has_minutes(diplomat_receive_buffer, this.underlying);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 1);
      if (!is_ok) {
        wasm.diplomat_free(diplomat_receive_buffer, 2, 1);
        return;
      }
      const value = (new Uint8Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0] == 1;
      wasm.diplomat_free(diplomat_receive_buffer, 2, 1);
      return value;
    })();
  }

  gmt_offset_has_seconds() {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(2, 1);
      wasm.ICU4XCustomTimeZone_gmt_offset_has_seconds(diplomat_receive_buffer, this.underlying);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 1);
      if (!is_ok) {
        wasm.diplomat_free(diplomat_receive_buffer, 2, 1);
        return;
      }
      const value = (new Uint8Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0] == 1;
      wasm.diplomat_free(diplomat_receive_buffer, 2, 1);
      return value;
    })();
  }

  try_set_time_zone_id(arg_id) {
    const buf_arg_id = diplomatRuntime.DiplomatBuf.str8(wasm, arg_id);
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XCustomTimeZone_try_set_time_zone_id(diplomat_receive_buffer, this.underlying, buf_arg_id.ptr, buf_arg_id.size);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = {};
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = TimeZoneInvalidIdError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
    buf_arg_id.free();
    return diplomat_out;
  }

  try_set_iana_time_zone_id(arg_mapper, arg_id) {
    const buf_arg_id = diplomatRuntime.DiplomatBuf.str8(wasm, arg_id);
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XCustomTimeZone_try_set_iana_time_zone_id(diplomat_receive_buffer, this.underlying, arg_mapper.underlying, buf_arg_id.ptr, buf_arg_id.size);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = {};
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = TimeZoneInvalidIdError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
    buf_arg_id.free();
    return diplomat_out;
  }

  clear_time_zone_id() {
    wasm.ICU4XCustomTimeZone_clear_time_zone_id(this.underlying);
  }

  time_zone_id() {
    return diplomatRuntime.withDiplomatWrite(wasm, (write) => {
      return (() => {
        const is_ok = wasm.ICU4XCustomTimeZone_time_zone_id(this.underlying, write) == 1;
        if (!is_ok) return;
      })();
    });
  }

  try_set_metazone_id(arg_id) {
    const buf_arg_id = diplomatRuntime.DiplomatBuf.str8(wasm, arg_id);
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XCustomTimeZone_try_set_metazone_id(diplomat_receive_buffer, this.underlying, buf_arg_id.ptr, buf_arg_id.size);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = {};
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = TimeZoneInvalidIdError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
    buf_arg_id.free();
    return diplomat_out;
  }

  clear_metazone_id() {
    wasm.ICU4XCustomTimeZone_clear_metazone_id(this.underlying);
  }

  metazone_id() {
    return diplomatRuntime.withDiplomatWrite(wasm, (write) => {
      return (() => {
        const is_ok = wasm.ICU4XCustomTimeZone_metazone_id(this.underlying, write) == 1;
        if (!is_ok) return;
      })();
    });
  }

  try_set_zone_variant(arg_id) {
    const buf_arg_id = diplomatRuntime.DiplomatBuf.str8(wasm, arg_id);
    const diplomat_out = (() => {
      const is_ok = wasm.ICU4XCustomTimeZone_try_set_zone_variant(this.underlying, buf_arg_id.ptr, buf_arg_id.size) == 1;
      if (!is_ok) return;
    })();
    buf_arg_id.free();
    return diplomat_out;
  }

  clear_zone_variant() {
    wasm.ICU4XCustomTimeZone_clear_zone_variant(this.underlying);
  }

  zone_variant() {
    return diplomatRuntime.withDiplomatWrite(wasm, (write) => {
      return (() => {
        const is_ok = wasm.ICU4XCustomTimeZone_zone_variant(this.underlying, write) == 1;
        if (!is_ok) return;
      })();
    });
  }

  set_standard_time() {
    wasm.ICU4XCustomTimeZone_set_standard_time(this.underlying);
  }

  set_daylight_time() {
    wasm.ICU4XCustomTimeZone_set_daylight_time(this.underlying);
  }

  is_standard_time() {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(2, 1);
      wasm.ICU4XCustomTimeZone_is_standard_time(diplomat_receive_buffer, this.underlying);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 1);
      if (!is_ok) {
        wasm.diplomat_free(diplomat_receive_buffer, 2, 1);
        return;
      }
      const value = (new Uint8Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0] == 1;
      wasm.diplomat_free(diplomat_receive_buffer, 2, 1);
      return value;
    })();
  }

  is_daylight_time() {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(2, 1);
      wasm.ICU4XCustomTimeZone_is_daylight_time(diplomat_receive_buffer, this.underlying);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 1);
      if (!is_ok) {
        wasm.diplomat_free(diplomat_receive_buffer, 2, 1);
        return;
      }
      const value = (new Uint8Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0] == 1;
      wasm.diplomat_free(diplomat_receive_buffer, 2, 1);
      return value;
    })();
  }

  maybe_calculate_metazone(arg_metazone_calculator, arg_local_datetime) {
    wasm.ICU4XCustomTimeZone_maybe_calculate_metazone(this.underlying, arg_metazone_calculator.underlying, arg_local_datetime.underlying);
  }
}