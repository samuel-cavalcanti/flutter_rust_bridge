// ignore_for_file: unused_import, unused_element, duplicate_ignore

import 'api/simple.dart';
import 'dart:async';
import 'dart:convert';
import 'frb_generated.io.dart' if (dart.library.html) 'frb_generated.web.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

/// Main entrypoint of the Rust API
class RustLib extends BaseEntrypoint<RustLibApi, RustLibApiImpl, RustLibWire> {
  @internal
  static final instance = RustLib._();

  RustLib._();

  /// Initialize flutter_rust_bridge
  static Future<void> init({
    RustLibApi? api,
    BaseHandler? handler,
  }) async {
    await instance.initImpl(api: api, handler: handler);
  }

  /// Dispose flutter_rust_bridge
  ///
  /// The call to this function is optional, since flutter_rust_bridge (and everything else)
  /// is automatically disposed when the app stops.
  static void dispose() => instance.disposeImpl();

  @override
  ApiImplConstructor<RustLibApiImpl, RustLibWire> get apiImplConstructor =>
      RustLibApiImpl.new;

  @override
  WireConstructor<RustLibWire> get wireConstructor =>
      RustLibWire.fromExternalLibrary;

  @override
  ExternalLibraryLoaderConfig get defaultExternalLibraryLoaderConfig =>
      const ExternalLibraryLoaderConfig(
        stem: 'rust_lib',
        ioDirectory: 'rust/target/release/',
        webPrefix: 'pkg/',
      );
}

abstract class RustLibApi extends BaseApi {
  String greet({required String name, dynamic hint});
}

class RustLibApiImpl extends RustLibApiImplPlatform implements RustLibApi {
  RustLibApiImpl({
    super.handler,
    required super.wire,
    required super.generalizedFrbRustBinding,
    required super.dropPortManager,
  });

  @override
  String greet({required String name, dynamic hint}) {
    var arg0 = api2wire_String(name);
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_greet(arg0),
      parseSuccessData: _wire2api_String,
      parseErrorData: null,
      constMeta: kGreetConstMeta,
      argValues: [name],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kGreetConstMeta => const TaskConstMeta(
        debugName: "greet",
        argNames: ["name"],
      );

  String _wire2api_String(dynamic raw) {
    return raw as String;
  }

  Uint8List _wire2api_list_prim_u_8(dynamic raw) {
    return raw as Uint8List;
  }

  int _wire2api_u_8(dynamic raw) {
    return raw as int;
  }
}

// Section: api2wire_funcs

int api2wire_u_8(int raw) {
  return raw;
}
