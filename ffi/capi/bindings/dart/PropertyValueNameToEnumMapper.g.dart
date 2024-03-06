// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

part of 'lib.g.dart';

/// A type capable of looking up a property value from a string name.
///
/// See the [Rust documentation for `PropertyValueNameToEnumMapper`](https://docs.rs/icu/latest/icu/properties/names/struct.PropertyValueNameToEnumMapper.html) for more information.
///
/// See the [Rust documentation for `PropertyValueNameToEnumMapperBorrowed`](https://docs.rs/icu/latest/icu/properties/names/struct.PropertyValueNameToEnumMapperBorrowed.html) for more information.
final class PropertyValueNameToEnumMapper implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  final core.List<Object> _edge_self;

  // Internal constructor from FFI.
  // isOwned is whether this is owned (has finalizer) or not
  // This also takes in a list of lifetime edges (including for &self borrows)
  // corresponding to data this may borrow from. These should be flat arrays containing
  // references to objects, and this object will hold on to them to keep them alive and
  // maintain borrow validity.
  PropertyValueNameToEnumMapper._(this._underlying, bool isOwned, this._edge_self) {
    if (isOwned) {
      _finalizer.attach(this, _underlying.cast());
    }
  }

  static final _finalizer = ffi.NativeFinalizer(ffi.Native.addressOf(_ICU4XPropertyValueNameToEnumMapper_destroy));

  /// Get the property value matching the given name, using strict matching
  ///
  /// Returns -1 if the name is unknown for this property
  ///
  /// See the [Rust documentation for `get_strict`](https://docs.rs/icu/latest/icu/properties/names/struct.PropertyValueNameToEnumMapperBorrowed.html#method.get_strict) for more information.
  int getStrict(String name) {
    final temp = ffi2.Arena();
    final nameView = name.utf8View;
    final result = _ICU4XPropertyValueNameToEnumMapper_get_strict(_underlying, nameView.pointer(temp), nameView.length);
    temp.releaseAll();
    return result;
  }

  /// Get the property value matching the given name, using loose matching
  ///
  /// Returns -1 if the name is unknown for this property
  ///
  /// See the [Rust documentation for `get_loose`](https://docs.rs/icu/latest/icu/properties/names/struct.PropertyValueNameToEnumMapperBorrowed.html#method.get_loose) for more information.
  int getLoose(String name) {
    final temp = ffi2.Arena();
    final nameView = name.utf8View;
    final result = _ICU4XPropertyValueNameToEnumMapper_get_loose(_underlying, nameView.pointer(temp), nameView.length);
    temp.releaseAll();
    return result;
  }

  /// See the [Rust documentation for `get_name_to_enum_mapper`](https://docs.rs/icu/latest/icu/properties/struct.GeneralCategory.html#method.get_name_to_enum_mapper) for more information.
  ///
  /// Throws [Error] on failure.
  factory PropertyValueNameToEnumMapper.loadGeneralCategory(DataProvider provider) {
    final result = _ICU4XPropertyValueNameToEnumMapper_load_general_category(provider._underlying);
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._underlying == result.union.err);
    }
    return PropertyValueNameToEnumMapper._(result.union.ok, true, []);
  }

  /// See the [Rust documentation for `name_to_enum_mapper`](https://docs.rs/icu/latest/icu/properties/struct.BidiClass.html#method.name_to_enum_mapper) for more information.
  ///
  /// Throws [Error] on failure.
  factory PropertyValueNameToEnumMapper.loadBidiClass(DataProvider provider) {
    final result = _ICU4XPropertyValueNameToEnumMapper_load_bidi_class(provider._underlying);
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._underlying == result.union.err);
    }
    return PropertyValueNameToEnumMapper._(result.union.ok, true, []);
  }

  /// See the [Rust documentation for `name_to_enum_mapper`](https://docs.rs/icu/latest/icu/properties/struct.EastAsianWidth.html#method.name_to_enum_mapper) for more information.
  ///
  /// Throws [Error] on failure.
  factory PropertyValueNameToEnumMapper.loadEastAsianWidth(DataProvider provider) {
    final result = _ICU4XPropertyValueNameToEnumMapper_load_east_asian_width(provider._underlying);
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._underlying == result.union.err);
    }
    return PropertyValueNameToEnumMapper._(result.union.ok, true, []);
  }

  /// See the [Rust documentation for `name_to_enum_mapper`](https://docs.rs/icu/latest/icu/properties/struct.IndicSyllabicCategory.html#method.name_to_enum_mapper) for more information.
  ///
  /// Throws [Error] on failure.
  factory PropertyValueNameToEnumMapper.loadIndicSyllabicCategory(DataProvider provider) {
    final result = _ICU4XPropertyValueNameToEnumMapper_load_indic_syllabic_category(provider._underlying);
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._underlying == result.union.err);
    }
    return PropertyValueNameToEnumMapper._(result.union.ok, true, []);
  }

  /// See the [Rust documentation for `name_to_enum_mapper`](https://docs.rs/icu/latest/icu/properties/struct.LineBreak.html#method.name_to_enum_mapper) for more information.
  ///
  /// Throws [Error] on failure.
  factory PropertyValueNameToEnumMapper.loadLineBreak(DataProvider provider) {
    final result = _ICU4XPropertyValueNameToEnumMapper_load_line_break(provider._underlying);
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._underlying == result.union.err);
    }
    return PropertyValueNameToEnumMapper._(result.union.ok, true, []);
  }

  /// See the [Rust documentation for `get_name_to_enum_mapper`](https://docs.rs/icu/latest/icu/properties/struct.GraphemeClusterBreak.html#method.get_name_to_enum_mapper) for more information.
  ///
  /// Throws [Error] on failure.
  factory PropertyValueNameToEnumMapper.loadGraphemeClusterBreak(DataProvider provider) {
    final result = _ICU4XPropertyValueNameToEnumMapper_load_grapheme_cluster_break(provider._underlying);
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._underlying == result.union.err);
    }
    return PropertyValueNameToEnumMapper._(result.union.ok, true, []);
  }

  /// See the [Rust documentation for `name_to_enum_mapper`](https://docs.rs/icu/latest/icu/properties/struct.WordBreak.html#method.name_to_enum_mapper) for more information.
  ///
  /// Throws [Error] on failure.
  factory PropertyValueNameToEnumMapper.loadWordBreak(DataProvider provider) {
    final result = _ICU4XPropertyValueNameToEnumMapper_load_word_break(provider._underlying);
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._underlying == result.union.err);
    }
    return PropertyValueNameToEnumMapper._(result.union.ok, true, []);
  }

  /// See the [Rust documentation for `name_to_enum_mapper`](https://docs.rs/icu/latest/icu/properties/struct.SentenceBreak.html#method.name_to_enum_mapper) for more information.
  ///
  /// Throws [Error] on failure.
  factory PropertyValueNameToEnumMapper.loadSentenceBreak(DataProvider provider) {
    final result = _ICU4XPropertyValueNameToEnumMapper_load_sentence_break(provider._underlying);
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._underlying == result.union.err);
    }
    return PropertyValueNameToEnumMapper._(result.union.ok, true, []);
  }

  /// See the [Rust documentation for `name_to_enum_mapper`](https://docs.rs/icu/latest/icu/properties/struct.Script.html#method.name_to_enum_mapper) for more information.
  ///
  /// Throws [Error] on failure.
  factory PropertyValueNameToEnumMapper.loadScript(DataProvider provider) {
    final result = _ICU4XPropertyValueNameToEnumMapper_load_script(provider._underlying);
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._underlying == result.union.err);
    }
    return PropertyValueNameToEnumMapper._(result.union.ok, true, []);
  }
}

@meta.ResourceIdentifier()
@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Void>)>(isLeaf: true, symbol: 'ICU4XPropertyValueNameToEnumMapper_destroy')
// ignore: non_constant_identifier_names
external void _ICU4XPropertyValueNameToEnumMapper_destroy(ffi.Pointer<ffi.Void> self);

@meta.ResourceIdentifier()
@ffi.Native<ffi.Int16 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Uint8>, ffi.Size)>(isLeaf: true, symbol: 'ICU4XPropertyValueNameToEnumMapper_get_strict')
// ignore: non_constant_identifier_names
external int _ICU4XPropertyValueNameToEnumMapper_get_strict(ffi.Pointer<ffi.Opaque> self, ffi.Pointer<ffi.Uint8> nameData, int nameLength);

@meta.ResourceIdentifier()
@ffi.Native<ffi.Int16 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Uint8>, ffi.Size)>(isLeaf: true, symbol: 'ICU4XPropertyValueNameToEnumMapper_get_loose')
// ignore: non_constant_identifier_names
external int _ICU4XPropertyValueNameToEnumMapper_get_loose(ffi.Pointer<ffi.Opaque> self, ffi.Pointer<ffi.Uint8> nameData, int nameLength);

@meta.ResourceIdentifier()
@ffi.Native<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XPropertyValueNameToEnumMapper_load_general_category')
// ignore: non_constant_identifier_names
external _ResultOpaqueInt32 _ICU4XPropertyValueNameToEnumMapper_load_general_category(ffi.Pointer<ffi.Opaque> provider);

@meta.ResourceIdentifier()
@ffi.Native<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XPropertyValueNameToEnumMapper_load_bidi_class')
// ignore: non_constant_identifier_names
external _ResultOpaqueInt32 _ICU4XPropertyValueNameToEnumMapper_load_bidi_class(ffi.Pointer<ffi.Opaque> provider);

@meta.ResourceIdentifier()
@ffi.Native<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XPropertyValueNameToEnumMapper_load_east_asian_width')
// ignore: non_constant_identifier_names
external _ResultOpaqueInt32 _ICU4XPropertyValueNameToEnumMapper_load_east_asian_width(ffi.Pointer<ffi.Opaque> provider);

@meta.ResourceIdentifier()
@ffi.Native<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XPropertyValueNameToEnumMapper_load_indic_syllabic_category')
// ignore: non_constant_identifier_names
external _ResultOpaqueInt32 _ICU4XPropertyValueNameToEnumMapper_load_indic_syllabic_category(ffi.Pointer<ffi.Opaque> provider);

@meta.ResourceIdentifier()
@ffi.Native<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XPropertyValueNameToEnumMapper_load_line_break')
// ignore: non_constant_identifier_names
external _ResultOpaqueInt32 _ICU4XPropertyValueNameToEnumMapper_load_line_break(ffi.Pointer<ffi.Opaque> provider);

@meta.ResourceIdentifier()
@ffi.Native<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XPropertyValueNameToEnumMapper_load_grapheme_cluster_break')
// ignore: non_constant_identifier_names
external _ResultOpaqueInt32 _ICU4XPropertyValueNameToEnumMapper_load_grapheme_cluster_break(ffi.Pointer<ffi.Opaque> provider);

@meta.ResourceIdentifier()
@ffi.Native<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XPropertyValueNameToEnumMapper_load_word_break')
// ignore: non_constant_identifier_names
external _ResultOpaqueInt32 _ICU4XPropertyValueNameToEnumMapper_load_word_break(ffi.Pointer<ffi.Opaque> provider);

@meta.ResourceIdentifier()
@ffi.Native<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XPropertyValueNameToEnumMapper_load_sentence_break')
// ignore: non_constant_identifier_names
external _ResultOpaqueInt32 _ICU4XPropertyValueNameToEnumMapper_load_sentence_break(ffi.Pointer<ffi.Opaque> provider);

@meta.ResourceIdentifier()
@ffi.Native<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XPropertyValueNameToEnumMapper_load_script')
// ignore: non_constant_identifier_names
external _ResultOpaqueInt32 _ICU4XPropertyValueNameToEnumMapper_load_script(ffi.Pointer<ffi.Opaque> provider);
