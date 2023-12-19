// ignore_for_file: avoid_print

import 'package:args/command_runner.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/consts.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/release.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/test.dart';
import 'package:flutter_rust_bridge_internal/src/utils/makefile_dart_infra.dart';

List<Command<void>> createCommands() {
  return [
    SimpleCommand('post-release-mimic-quickstart', postReleaseMimicQuickstart),
  ];
}

Future<void> postReleaseMimicQuickstart() async {
  await _quickstartStepInstall(config.installMode);
  await _quickstartStepCreate();
  await _quickstartStepRun();
  await _quickstartStepModifyAndGenerate();
  await _quickstartStepRun(); // Run again after modification
}

const _kPackageName = 'my_app';

enum InstallMode {
  cargoInstall,
  cargoBinstall,
  scoop,
  homebrew,
}

Future<void> _quickstartStepInstall(InstallMode mode) async {
  switch (mode) {
    case InstallMode.cargoInstall:
      await exec('cargo install flutter_rust_bridge_codegen');
    case InstallMode.cargoBinstall:
      await exec('cargo binstall flutter_rust_bridge_codegen');
    case InstallMode.scoop:
      await exec(
          'scoop bucket add frb https://github.com/Desdaemon/scoop-repo');
      await exec('scoop install flutter_rust_bridge_codegen');
    case InstallMode.homebrew:
      await exec('brew install desdaemon/repo/flutter_rust_bridge_codegen');
  }
}

Future<void> _quickstartStepCreate() async {
  await exec('flutter_rust_bridge_codegen create $_kPackageName');
}

Future<void> _quickstartStepRun() async {
  await flutterIntegrationTestRaw(
    flutterTestArgs: TODO,
    relativePwd: _kPackageName,
  );
}

Future<void> _quickstartStepModifyAndGenerate() async {
  const kExtraRustSrc = TODO;
  const kExtraDartTest = TODO;

  simpleReplaceFile(
    '${exec.pwd}$_kPackageName/rust/src/api/simple.rs',
    RegExp(r'$', multiLine: true),
    kExtraRustSrc,
  );
  simpleReplaceFile(
    '${exec.pwd}$_kPackageName/integration_test/simple_test.dart',
    'testWidgets(',
    '$kExtraDartTest\ntestWidgets(',
  );
}
