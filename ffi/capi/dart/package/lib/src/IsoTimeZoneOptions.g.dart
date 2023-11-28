// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

part of 'lib.g.dart';

final class _IsoTimeZoneOptionsFfi extends ffi.Struct {
  @ffi.Int32()
  external int format;
  @ffi.Int32()
  external int minutes;
  @ffi.Int32()
  external int seconds;
}

final class IsoTimeZoneOptions {
  final _IsoTimeZoneOptionsFfi _underlying;

  IsoTimeZoneOptions._(this._underlying);

  factory IsoTimeZoneOptions() {
    final pointer = ffi2.calloc<_IsoTimeZoneOptionsFfi>();
    final result = IsoTimeZoneOptions._(pointer.ref);
    _callocFree.attach(result, pointer.cast());
    return result;
  }

  IsoTimeZoneFormat get format => IsoTimeZoneFormat.values[_underlying.format];
  set format(IsoTimeZoneFormat format) {
    _underlying.format = format.index;
  }

  IsoTimeZoneMinuteDisplay get minutes => IsoTimeZoneMinuteDisplay.values[_underlying.minutes];
  set minutes(IsoTimeZoneMinuteDisplay minutes) {
    _underlying.minutes = minutes.index;
  }

  IsoTimeZoneSecondDisplay get seconds => IsoTimeZoneSecondDisplay.values[_underlying.seconds];
  set seconds(IsoTimeZoneSecondDisplay seconds) {
    _underlying.seconds = seconds.index;
  }

  @override
  bool operator ==(Object other) =>
      other is IsoTimeZoneOptions &&
      other._underlying.format == _underlying.format &&
      other._underlying.minutes == _underlying.minutes &&
      other._underlying.seconds == _underlying.seconds;

  @override
  int get hashCode => Object.hashAll([
        _underlying.format,
        _underlying.minutes,
        _underlying.seconds,
      ]);
}