// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 1.82.4.

// ignore_for_file: invalid_use_of_internal_member, unused_import

import '../../frb_generated.dart';
import 'package:collection/collection.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;
part 'rust_opaque_twin_rust_async.freezed.dart';

Future<HideData> createOpaqueTwinRustAsync({dynamic hint}) =>
    RustLib.instance.api.createOpaqueTwinRustAsync(hint: hint);

Future<HideData?> createOptionOpaqueTwinRustAsync(
        {HideData? opaque, dynamic hint}) =>
    RustLib.instance.api
        .createOptionOpaqueTwinRustAsync(opaque: opaque, hint: hint);

Future<EnumOpaqueTwinRustAsyncArray5> createArrayOpaqueEnumTwinRustAsync(
        {dynamic hint}) =>
    RustLib.instance.api.createArrayOpaqueEnumTwinRustAsync(hint: hint);

Future<String> runEnumOpaqueTwinRustAsync(
        {required EnumOpaqueTwinRustAsync opaque, dynamic hint}) =>
    RustLib.instance.api.runEnumOpaqueTwinRustAsync(opaque: opaque, hint: hint);

Future<String> runOpaqueTwinRustAsync(
        {required HideData opaque, dynamic hint}) =>
    RustLib.instance.api.runOpaqueTwinRustAsync(opaque: opaque, hint: hint);

Future<String> runOpaqueWithDelayTwinRustAsync(
        {required HideData opaque, dynamic hint}) =>
    RustLib.instance.api
        .runOpaqueWithDelayTwinRustAsync(opaque: opaque, hint: hint);

Future<HideDataArray2> opaqueArrayTwinRustAsync({dynamic hint}) =>
    RustLib.instance.api.opaqueArrayTwinRustAsync(hint: hint);

Future<String> runNonCloneTwinRustAsync(
        {required NonCloneData clone, dynamic hint}) =>
    RustLib.instance.api.runNonCloneTwinRustAsync(clone: clone, hint: hint);

Future<NonSendHideData> createSyncOpaqueTwinRustAsync({dynamic hint}) =>
    RustLib.instance.api.createSyncOpaqueTwinRustAsync(hint: hint);

Future<void> opaqueArrayRunTwinRustAsync(
        {required HideDataArray2 data, dynamic hint}) =>
    RustLib.instance.api.opaqueArrayRunTwinRustAsync(data: data, hint: hint);

Future<List<HideData>> opaqueVecTwinRustAsync({dynamic hint}) =>
    RustLib.instance.api.opaqueVecTwinRustAsync(hint: hint);

Future<void> opaqueVecRunTwinRustAsync(
        {required List<HideData> data, dynamic hint}) =>
    RustLib.instance.api.opaqueVecRunTwinRustAsync(data: data, hint: hint);

Future<OpaqueNestedTwinRustAsync> createNestedOpaqueTwinRustAsync(
        {dynamic hint}) =>
    RustLib.instance.api.createNestedOpaqueTwinRustAsync(hint: hint);

Future<void> runNestedOpaqueTwinRustAsync(
        {required OpaqueNestedTwinRustAsync opaque, dynamic hint}) =>
    RustLib.instance.api
        .runNestedOpaqueTwinRustAsync(opaque: opaque, hint: hint);

Future<String> unwrapRustOpaqueTwinRustAsync(
        {required HideData opaque, dynamic hint}) =>
    RustLib.instance.api
        .unwrapRustOpaqueTwinRustAsync(opaque: opaque, hint: hint);

/// Function to check the code generator.
/// FrbOpaqueReturn must be only return type.
/// FrbOpaqueReturn must not be used as an argument.
Future<FrbOpaqueReturn> frbGeneratorTestTwinRustAsync({dynamic hint}) =>
    RustLib.instance.api.frbGeneratorTestTwinRustAsync(hint: hint);

@sealed
class MutexHideData extends RustOpaque {
  MutexHideData.fromWire(dynamic wire) : this._fromRaw(wire[0], wire[1]);

  MutexHideData._fromRaw(int ptr, int size) : super.unsafe(ptr, size);

  @override
  OpaqueDropFnType get dropFn => RustLib.instance.api.dropOpaqueMutexHideData;

  @override
  OpaqueShareFnType get shareFn =>
      RustLib.instance.api.shareOpaqueMutexHideData;

  @override
  OpaqueTypeFinalizer get staticFinalizer =>
      RustLib.instance.api.mutexHideDataFinalizer;
}

@sealed
class RwLockHideData extends RustOpaque {
  RwLockHideData.fromWire(dynamic wire) : this._fromRaw(wire[0], wire[1]);

  RwLockHideData._fromRaw(int ptr, int size) : super.unsafe(ptr, size);

  @override
  OpaqueDropFnType get dropFn => RustLib.instance.api.dropOpaqueRwLockHideData;

  @override
  OpaqueShareFnType get shareFn =>
      RustLib.instance.api.shareOpaqueRwLockHideData;

  @override
  OpaqueTypeFinalizer get staticFinalizer =>
      RustLib.instance.api.rwLockHideDataFinalizer;
}

@sealed
class BoxDartDebugTwinRustAsync extends RustOpaque {
  BoxDartDebugTwinRustAsync.fromWire(dynamic wire)
      : this._fromRaw(wire[0], wire[1]);

  BoxDartDebugTwinRustAsync._fromRaw(int ptr, int size)
      : super.unsafe(ptr, size);

  @override
  OpaqueDropFnType get dropFn =>
      RustLib.instance.api.dropOpaqueBoxDartDebugTwinRustAsync;

  @override
  OpaqueShareFnType get shareFn =>
      RustLib.instance.api.shareOpaqueBoxDartDebugTwinRustAsync;

  @override
  OpaqueTypeFinalizer get staticFinalizer =>
      RustLib.instance.api.boxDartDebugTwinRustAsyncFinalizer;
}

@sealed
class FrbOpaqueReturn extends RustOpaque {
  FrbOpaqueReturn.fromWire(dynamic wire) : this._fromRaw(wire[0], wire[1]);

  FrbOpaqueReturn._fromRaw(int ptr, int size) : super.unsafe(ptr, size);

  @override
  OpaqueDropFnType get dropFn => RustLib.instance.api.dropOpaqueFrbOpaqueReturn;

  @override
  OpaqueShareFnType get shareFn =>
      RustLib.instance.api.shareOpaqueFrbOpaqueReturn;

  @override
  OpaqueTypeFinalizer get staticFinalizer =>
      RustLib.instance.api.frbOpaqueReturnFinalizer;
}

@sealed
class HideData extends RustOpaque {
  HideData.fromWire(dynamic wire) : this._fromRaw(wire[0], wire[1]);

  HideData._fromRaw(int ptr, int size) : super.unsafe(ptr, size);

  @override
  OpaqueDropFnType get dropFn => RustLib.instance.api.dropOpaqueHideData;

  @override
  OpaqueShareFnType get shareFn => RustLib.instance.api.shareOpaqueHideData;

  @override
  OpaqueTypeFinalizer get staticFinalizer =>
      RustLib.instance.api.hideDataFinalizer;
}

class HideDataArray2 extends NonGrowableListView<HideData> {
  static const arraySize = 2;
  HideDataArray2(List<HideData> inner)
      : assert(inner.length == arraySize),
        super(inner);
  HideDataArray2.unchecked(List<HideData> inner) : super(inner);
  HideDataArray2.init(HideData fill)
      : super(List<HideData>.filled(arraySize, fill));
}

@sealed
class I32 extends RustOpaque {
  I32.fromWire(dynamic wire) : this._fromRaw(wire[0], wire[1]);

  I32._fromRaw(int ptr, int size) : super.unsafe(ptr, size);

  @override
  OpaqueDropFnType get dropFn => RustLib.instance.api.dropOpaqueI32;

  @override
  OpaqueShareFnType get shareFn => RustLib.instance.api.shareOpaqueI32;

  @override
  OpaqueTypeFinalizer get staticFinalizer => RustLib.instance.api.i32Finalizer;
}

@sealed
class NonCloneData extends RustOpaque {
  NonCloneData.fromWire(dynamic wire) : this._fromRaw(wire[0], wire[1]);

  NonCloneData._fromRaw(int ptr, int size) : super.unsafe(ptr, size);

  @override
  OpaqueDropFnType get dropFn => RustLib.instance.api.dropOpaqueNonCloneData;

  @override
  OpaqueShareFnType get shareFn => RustLib.instance.api.shareOpaqueNonCloneData;

  @override
  OpaqueTypeFinalizer get staticFinalizer =>
      RustLib.instance.api.nonCloneDataFinalizer;
}

@sealed
class NonSendHideData extends RustOpaque {
  NonSendHideData.fromWire(dynamic wire) : this._fromRaw(wire[0], wire[1]);

  NonSendHideData._fromRaw(int ptr, int size) : super.unsafe(ptr, size);

  @override
  OpaqueDropFnType get dropFn => RustLib.instance.api.dropOpaqueNonSendHideData;

  @override
  OpaqueShareFnType get shareFn =>
      RustLib.instance.api.shareOpaqueNonSendHideData;

  @override
  OpaqueTypeFinalizer get staticFinalizer =>
      RustLib.instance.api.nonSendHideDataFinalizer;
}

@freezed
sealed class EnumOpaqueTwinRustAsync with _$EnumOpaqueTwinRustAsync {
  const factory EnumOpaqueTwinRustAsync.struct(
    HideData field0,
  ) = EnumOpaqueTwinRustAsync_Struct;
  const factory EnumOpaqueTwinRustAsync.primitive(
    I32 field0,
  ) = EnumOpaqueTwinRustAsync_Primitive;
  const factory EnumOpaqueTwinRustAsync.traitObj(
    BoxDartDebugTwinRustAsync field0,
  ) = EnumOpaqueTwinRustAsync_TraitObj;
  const factory EnumOpaqueTwinRustAsync.mutex(
    MutexHideData field0,
  ) = EnumOpaqueTwinRustAsync_Mutex;
  const factory EnumOpaqueTwinRustAsync.rwLock(
    RwLockHideData field0,
  ) = EnumOpaqueTwinRustAsync_RwLock;
}

class EnumOpaqueTwinRustAsyncArray5
    extends NonGrowableListView<EnumOpaqueTwinRustAsync> {
  static const arraySize = 5;
  EnumOpaqueTwinRustAsyncArray5(List<EnumOpaqueTwinRustAsync> inner)
      : assert(inner.length == arraySize),
        super(inner);
  EnumOpaqueTwinRustAsyncArray5.unchecked(List<EnumOpaqueTwinRustAsync> inner)
      : super(inner);
  EnumOpaqueTwinRustAsyncArray5.init(EnumOpaqueTwinRustAsync fill)
      : super(List<EnumOpaqueTwinRustAsync>.filled(arraySize, fill));
}

/// [`HideData`] has private fields.
class OpaqueNestedTwinRustAsync {
  final HideData first;
  final HideData second;

  const OpaqueNestedTwinRustAsync({
    required this.first,
    required this.second,
  });

  @override
  int get hashCode => first.hashCode ^ second.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is OpaqueNestedTwinRustAsync &&
          runtimeType == other.runtimeType &&
          first == other.first &&
          second == other.second;
}
