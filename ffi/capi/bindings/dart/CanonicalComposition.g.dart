// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

part of 'lib.g.dart';

/// The raw canonical composition operation.
///
/// Callers should generally use ICU4XComposingNormalizer unless they specifically need raw composition operations
///
/// See the [Rust documentation for `CanonicalComposition`](https://docs.rs/icu/latest/icu/normalizer/properties/struct.CanonicalComposition.html) for more information.
final class CanonicalComposition implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  final core.List<Object> _edge_self;

  // Internal constructor from FFI.
  // isOwned is whether this is owned (has finalizer) or not
  // This also takes in a list of lifetime edges (including for &self borrows)
  // corresponding to data this may borrow from. These should be flat arrays containing
  // references to objects, and this object will hold on to them to keep them alive and
  // maintain borrow validity.
  CanonicalComposition._(this._underlying, bool isOwned, this._edge_self) {
    if (isOwned) {
      _finalizer.attach(this, _underlying.cast());
    }
  }

  static final _finalizer = ffi.NativeFinalizer(ffi.Native.addressOf(_ICU4XCanonicalComposition_destroy));

  /// Construct a new ICU4XCanonicalComposition instance for NFC
  ///
  /// See the [Rust documentation for `new`](https://docs.rs/icu/latest/icu/normalizer/properties/struct.CanonicalComposition.html#method.new) for more information.
  ///
  /// Throws [Error] on failure.
  factory CanonicalComposition(DataProvider provider) {
    final result = _ICU4XCanonicalComposition_create(provider._underlying);
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._underlying == result.union.err);
    }
    return CanonicalComposition._(result.union.ok, true, []);
  }

  /// Performs canonical composition (including Hangul) on a pair of characters
  /// or returns NUL if these characters don’t compose. Composition exclusions are taken into account.
  ///
  /// See the [Rust documentation for `compose`](https://docs.rs/icu/latest/icu/normalizer/properties/struct.CanonicalComposition.html#method.compose) for more information.
  Rune compose(Rune starter, Rune second) {
    final result = _ICU4XCanonicalComposition_compose(_underlying, starter, second);
    return result;
  }
}

@meta.ResourceIdentifier()
@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Void>)>(isLeaf: true, symbol: 'ICU4XCanonicalComposition_destroy')
// ignore: non_constant_identifier_names
external void _ICU4XCanonicalComposition_destroy(ffi.Pointer<ffi.Void> self);

@meta.ResourceIdentifier()
@ffi.Native<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XCanonicalComposition_create')
// ignore: non_constant_identifier_names
external _ResultOpaqueInt32 _ICU4XCanonicalComposition_create(ffi.Pointer<ffi.Opaque> provider);

@meta.ResourceIdentifier()
@ffi.Native<ffi.Uint32 Function(ffi.Pointer<ffi.Opaque>, ffi.Uint32, ffi.Uint32)>(isLeaf: true, symbol: 'ICU4XCanonicalComposition_compose')
// ignore: non_constant_identifier_names
external Rune _ICU4XCanonicalComposition_compose(ffi.Pointer<ffi.Opaque> self, Rune starter, Rune second);
