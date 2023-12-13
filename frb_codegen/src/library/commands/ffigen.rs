use crate::command_run;
use crate::commands::command_runner::call_shell;
use crate::utils::dart_repository::dart_repo::DartRepository;
use crate::utils::path_utils::path_to_string;
use anyhow::bail;
use itertools::Itertools;
use log::{debug, warn};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Output;
use std::str::FromStr;

pub(crate) struct FfigenArgs<'a> {
    pub c_file_content: &'a str,
    pub dart_class_name: &'a str,
    pub llvm_path: &'a [PathBuf],
    pub llvm_compiler_opts: &'a str,
    pub dart_root: &'a Path,
}

pub(crate) fn ffigen(args: FfigenArgs) -> anyhow::Result<String> {
    let temp_c_file = tempfile::Builder::new().suffix(".h").tempfile()?;
    let temp_dart_file = tempfile::NamedTempFile::new()?;

    fs::write(temp_c_file.path(), args.c_file_content)?;
    ffigen_to_file(FfigenToFileArgs {
        c_path: temp_c_file.path(),
        dart_path: temp_dart_file.path(),
        dart_class_name: args.dart_class_name,
        llvm_path: args.llvm_path,
        llvm_compiler_opts: args.llvm_compiler_opts,
        dart_root: args.dart_root,
    })?;
    let output_text = fs::read_to_string(temp_dart_file.path())?;

    // do not drop too early
    drop(temp_c_file);
    drop(temp_dart_file);

    Ok(output_text)
}

struct FfigenToFileArgs<'a> {
    c_path: &'a Path,
    dart_path: &'a Path,
    dart_class_name: &'a str,
    llvm_path: &'a [PathBuf],
    llvm_compiler_opts: &'a str,
    dart_root: &'a Path,
}

fn ffigen_to_file(args: FfigenToFileArgs) -> anyhow::Result<()> {
    debug!(
        "execute ffigen c_path={c_path:?} dart_path={dart_path:?} llvm_path={llvm_path:?}",
        c_path = args.c_path,
        dart_path = args.c_path,
        llvm_path = args.llvm_path,
    );

    let config = parse_config(&args);

    ffigen_raw(&config, args.dart_root)
}

pub(crate) fn ffigen_raw(config: &FfigenCommandConfig, dart_root: &Path) -> anyhow::Result<()> {
    let config = serde_json::to_string(config)?;

    let mut config_file = tempfile::NamedTempFile::new()?;
    config_file.write_all(config.as_bytes())?;
    debug!("ffigen_raw config={config:?} config_file={config_file:?}");

    let repo = DartRepository::from_str(&path_to_string(dart_root)?).unwrap();
    let res = command_run!(
        call_shell[Some(dart_root)],
        *repo.toolchain.as_run_command(),
        *repo.command_extra_args(),
        "run",
        "ffigen",
        "--config",
        config_file.path()
    )?;

    if let Some(warning) = handle_output(
        res.status.success(),
        &String::from_utf8_lossy(&res.stdout).into_owned(),
        &String::from_utf8_lossy(&res.stderr).into_owned(),
    )? {
        warn!("{}", warning);
    }

    Ok(())
}

fn handle_output(
    status_success: bool,
    stdout: &str,
    stderr: &str,
) -> anyhow::Result<Option<String>> {
    let hint_link = "Please refer to https://fzyzcjy.github.io/flutter_rust_bridge/manual/ffigen-troubleshooting for details";

    if !status_success {
        let pat = "Couldn't find dynamic library in default locations.";
        if stderr.contains(pat) || stdout.contains(pat) {
            bail!("ffigen could not find LLVM. {}", hint_link);
        }

        bail!("ffigen failed. {}", hint_link);
    }

    // This is usually not a problem
    let nullability_message = "pointer is missing a nullability type specifier (_Nonnull, _Nullable, or _Null_unspecified) [Nullability Issue]";

    let stdout_lines = stdout.split("\n").collect_vec();
    if stdout_lines
        .iter()
        .any(|line| line.contains("[SEVERE]") && !line.contains(nullability_message))
    {
        // If ffigen can't find a header file it will generate broken
        // bindings but still exit successfully. We can detect these broken
        // bindings by looking for a "[SEVERE]" log message.
        //
        // It may emit SEVERE log messages for non-fatal errors though, so
        // we don't want to error out completely.
        return Ok(Some(format!(
            "The `ffigen` command emitted a SEVERE error. Maybe there is a problem? {}",
            hint_link
        )));
    }

    Ok(None)
}

fn parse_config(args: &FfigenToFileArgs) -> FfigenCommandConfig {
    let llvm_compiler_opts_list = if args.llvm_compiler_opts.is_empty() {
        vec![]
    } else {
        vec![args.llvm_compiler_opts.to_owned()]
    };

    FfigenCommandConfig{
        output: args.dart_path.to_owned(),
        name: args.dart_class_name.to_owned(),
        description: "generated by flutter_rust_bridge".to_owned(),
        headers: FfigenCommandConfigHeaders{
          entry_points: vec![args.c_path.to_owned()],
          include_directives: vec![args.c_path.to_owned()],
        },
        comments: Some(false),
        preamble: "// ignore_for_file: camel_case_types, non_constant_identifier_names, avoid_positional_boolean_parameters, annotate_overrides, constant_identifier_names".to_owned(),
        llvm_path: args.llvm_path.to_owned(),
        compiler_opts: llvm_compiler_opts_list,
    }
}

#[derive(Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct FfigenCommandConfig {
    pub output: PathBuf,
    pub name: String,
    pub description: String,
    pub headers: FfigenCommandConfigHeaders,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<bool>,
    pub preamble: String,
    pub llvm_path: Vec<PathBuf>,
    pub compiler_opts: Vec<String>,
}

#[derive(Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct FfigenCommandConfigHeaders {
    pub entry_points: Vec<PathBuf>,
    pub include_directives: Vec<PathBuf>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_handle_output_when_normal() {
        let result = handle_output(true, "", "");
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    pub fn test_handle_output_when_has_severe_should_warn() {
        let result = handle_output(true, "One line\n[SEVERE] Something\nAnother line", "");
        assert!(result
            .unwrap()
            .unwrap()
            .contains("The `ffigen` command emitted a SEVERE error."));
    }

    #[test]
    pub fn test_handle_output_when_has_severe_but_known_no_problem() {
        let result = handle_output(true, "One line
[SEVERE] :     /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdlib.h:364:13: warning: pointer is missing a nullability type specifier (_Nonnull, _Nullable, or _Null_unspecified) [Nullability Issue]
Another line", "");
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    pub fn test_handle_output_when_has_severe_stdbool_not_found() {
        // Commonly seen problem, e.g. https://github.com/fzyzcjy/flutter_rust_bridge/issues/1083
        let result = handle_output(
            true,
            r#"One line
[SEVERE] : Header C:\Users\Someone\AppData\Local\Temp\.tmpvqq91Q.h: Total errors/warnings: 1.
[SEVERE] :     C:\Users\Someone\AppData\Local\Temp\.tmpvqq91Q.h:1:10: fatal error: 'stdbool.h' file not found [Lexical or Preprocessor Issue]
Another line"#,
            "",
        );
        assert!(result
            .unwrap()
            .unwrap()
            .contains("The `ffigen` command emitted a SEVERE error."));
    }

    #[test]
    pub fn test_handle_output_when_cannot_find_llvm_should_fail() {
        let result = handle_output(
            false,
            "One line\nCouldn't find dynamic library in default locations.\nAnother line",
            "",
        );
        assert!(result
            .err()
            .unwrap()
            .to_string()
            .contains("ffigen could not find LLVM"));
    }
}
