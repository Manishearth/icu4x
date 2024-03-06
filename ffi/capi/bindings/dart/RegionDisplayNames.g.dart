// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

part of 'lib.g.dart';

/// See the [Rust documentation for `RegionDisplayNames`](https://docs.rs/icu/latest/icu/displaynames/struct.RegionDisplayNames.html) for more information.
final class RegionDisplayNames implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  final core.List<Object> _edge_self;

  // Internal constructor from FFI.
  // isOwned is whether this is owned (has finalizer) or not
  // This also takes in a list of lifetime edges (including for &self borrows)
  // corresponding to data this may borrow from. These should be flat arrays containing
  // references to objects, and this object will hold on to them to keep them alive and
  // maintain borrow validity.
  RegionDisplayNames._(this._underlying, bool isOwned, this._edge_self) {
    if (isOwned) {
      _finalizer.attach(this, _underlying.cast());
    }
  }

  static final _finalizer = ffi.NativeFinalizer(ffi.Native.addressOf(_ICU4XRegionDisplayNames_destroy));

  /// Creates a new `RegionDisplayNames` from locale data and an options bag.
  ///
  /// See the [Rust documentation for `try_new`](https://docs.rs/icu/latest/icu/displaynames/struct.RegionDisplayNames.html#method.try_new) for more information.
  ///
  /// Throws [Error] on failure.
  factory RegionDisplayNames(DataProvider provider, Locale locale) {
    final result = _ICU4XRegionDisplayNames_create(provider._underlying, locale._underlying);
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._underlying == result.union.err);
    }
    return RegionDisplayNames._(result.union.ok, true, []);
  }

  /// Returns the locale specific display name of a region.
  /// Note that the function returns an empty string in case the display name for a given
  /// region code is not found.
  ///
  /// See the [Rust documentation for `of`](https://docs.rs/icu/latest/icu/displaynames/struct.RegionDisplayNames.html#method.of) for more information.
  ///
  /// Throws [Error] on failure.
  String of(String region) {
    final temp = ffi2.Arena();
    final regionView = region.utf8View;
    final writeable = _Writeable();
    final result = _ICU4XRegionDisplayNames_of(_underlying, regionView.pointer(temp), regionView.length, writeable._underlying);
    temp.releaseAll();
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._underlying == result.union.err);
    }
    return writeable.finalize();
  }
}

@meta.ResourceIdentifier()
@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Void>)>(isLeaf: true, symbol: 'ICU4XRegionDisplayNames_destroy')
// ignore: non_constant_identifier_names
external void _ICU4XRegionDisplayNames_destroy(ffi.Pointer<ffi.Void> self);

@meta.ResourceIdentifier()
@ffi.Native<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XRegionDisplayNames_create')
// ignore: non_constant_identifier_names
external _ResultOpaqueInt32 _ICU4XRegionDisplayNames_create(ffi.Pointer<ffi.Opaque> provider, ffi.Pointer<ffi.Opaque> locale);

@meta.ResourceIdentifier()
@ffi.Native<_ResultVoidInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Uint8>, ffi.Size, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XRegionDisplayNames_of')
// ignore: non_constant_identifier_names
external _ResultVoidInt32 _ICU4XRegionDisplayNames_of(ffi.Pointer<ffi.Opaque> self, ffi.Pointer<ffi.Uint8> regionData, int regionLength, ffi.Pointer<ffi.Opaque> writeable);
