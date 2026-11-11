//! Spec 70 §1.1/§1.3: durable invariants for whole coded stderr lines and
//! absent-versus-present subcommand parsing. Fixture bytes are not a promise
//! about diagnostic message wording.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn run(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_ken"))
        .args(args)
        .output()
        .expect("run ken")
}

fn rooted_tempdir() -> tempfile::TempDir {
    let parent = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target/tmp");
    fs::create_dir_all(&parent).expect("fixture parent under catalog root");
    tempfile::tempdir_in(parent).expect("rooted Ken source")
}

fn write_source(dir: &Path, name: &str, source: &str) -> String {
    let path = dir.join(name);
    fs::write(&path, source).expect("write Ken fixture");
    path.to_str().expect("temporary path is UTF-8").to_owned()
}

fn host_exit_source(bytes: &str, exit: &str) -> String {
    format!(
        "program capabilities FS APartial\n\
         import Capability.Console.Text (write)\n\
         import Capability.Process.Exit (host_program_then)\n\
         proc act : Unit visits [Console] =\n\
           let _ = write ConsoleStream.Stderr (bytes_encode {bytes}) in MkUnit\n\
         proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)\n\
           : HostIO APartial ExitCode visits [Console] =\n\
           host_program_then APartial act {exit}\n"
    )
}

const STUCK_TREE: &str = r#"program capabilities FS APartial
import Capability.Console.Text (write)
import Capability.Process.Exit (host_bind, host_console, host_pure)
import Data.Numeric.FixedWidth (intToInt8)
proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode visits [Console] =
  host_bind APartial (Result IOError Unit) ExitCode
    (host_console APartial (Result IOError Unit)
      (write ConsoleStream.Stderr (bytes_encode "partial")))
    (\_ . match (intToInt8 (sub_int 0 128)) { Some m |-> (match (eq_int (int8_to_int (neg_int8 m)) 0) { True |-> host_pure APartial ExitCode ExitCode.Success ; False |-> host_pure APartial ExitCode ExitCode.Success }) ; None |-> host_pure APartial ExitCode ExitCode.Success })
"#;

fn assert_whole_line_after_program_bytes(output: &Output, program_bytes: &[u8], code: &str) {
    assert_eq!(output.status.code(), Some(1), "stderr: {:?}", output.stderr);
    let stderr = std::str::from_utf8(&output.stderr).expect("diagnostic stderr is UTF-8");
    let program_bytes = std::str::from_utf8(program_bytes).expect("fixture bytes are UTF-8");
    let expected = format!(
        "{}error[{code}]: ",
        if program_bytes.ends_with('\n') {
            program_bytes.to_owned()
        } else {
            format!("{program_bytes}\n")
        }
    );
    assert!(stderr.starts_with(&expected), "stderr: {stderr:?}");
    assert_eq!(stderr.lines().next(), Some("partial"), "stderr: {stderr:?}");
}

#[test]
fn post_execution_host_diagnostic_starts_on_its_own_line() {
    let dir = rooted_tempdir();
    let source = write_source(
        dir.path(),
        "malformed-exit.ken",
        &host_exit_source(
            "\"partial\"",
            "(ExitCode.Failure (add_uint8 (255 : UInt8) (1 : UInt8)))",
        ),
    );
    let output = run(&["run", &source]);
    assert_whole_line_after_program_bytes(&output, b"partial", "H0003");
}

/// Promise class: durable invariant. MEASURED: the actual stuck-tree source
/// imports the checked safe intToInt8 through FixedWidth in Strict, and
/// removing only that import refuses it. CLAIMED: the package home does not
/// alter the program's checked continuation. THE GAP: X0003 after the write
/// is separately observed by the unchanged CLI runtime assertion below.
#[test]
fn strict_stuck_tree_requires_its_fixed_width_import() {
    let catalog = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../catalog/packages")
        .canonicalize()
        .expect("catalog root");
    let imported = STUCK_TREE;
    let mut selected = ken_elaborator::ElabEnv::new().expect("prelude");
    selected
        .elaborate_isolated_file_strict_v1(&[catalog.clone()], imported)
        .expect("the checked narrowing import preserves the stuck continuation");
    let without_conversion =
        STUCK_TREE.replacen("import Data.Numeric.FixedWidth (intToInt8)\n", "", 1);
    assert_ne!(
        without_conversion, STUCK_TREE,
        "negative must remove FixedWidth import"
    );
    assert!(
        !without_conversion.contains("import Data.Numeric.FixedWidth (intToInt8)"),
        "negative must retain no narrowing import"
    );
    let mut negative = ken_elaborator::ElabEnv::new().expect("prelude");
    match negative.elaborate_isolated_file_strict_v1(&[catalog], &without_conversion) {
        Err(ken_elaborator::ElabError::UnboundName { name, .. }) => assert_eq!(name, "intToInt8"),
        Err(error) => panic!("wrong no-import refusal: {error:?}"),
        Ok(_) => panic!("intToInt8 was ambient without the FixedWidth import"),
    }
}

#[test]
fn post_execution_x_diagnostic_starts_on_its_own_line() {
    let dir = rooted_tempdir();
    // A checked continuation becomes stuck after the Console write, so run_io
    // returns X0003 rather than an admitted program outcome.
    let source = write_source(dir.path(), "stuck-tree.ken", STUCK_TREE);
    let output = run(&["run", &source]);
    assert_whole_line_after_program_bytes(&output, b"partial", "X0003");
}

#[test]
fn finished_program_line_adds_no_blank_line_and_success_keeps_exact_stderr() {
    let dir = rooted_tempdir();
    let malformed = write_source(
        dir.path(),
        "finished-line.ken",
        &host_exit_source(
            "\"partial\\n\"",
            "(ExitCode.Failure (add_uint8 (255 : UInt8) (1 : UInt8)))",
        ),
    );
    let output = run(&["run", &malformed]);
    assert_whole_line_after_program_bytes(&output, b"partial\n", "H0003");

    let success = write_source(
        dir.path(),
        "success.ken",
        &host_exit_source("\"partial\"", "ExitCode.Success"),
    );
    let output = run(&["run", &success]);
    assert_eq!(output.status.code(), Some(0), "stderr: {:?}", output.stderr);
    assert_eq!(output.stderr, b"partial");
}

#[test]
fn absent_subcommand_is_help_but_empty_subcommand_is_coded_failure() {
    let absent = run(&[]);
    assert_eq!(absent.status.code(), Some(0));
    assert!(absent.stderr.is_empty());
    assert!(std::str::from_utf8(&absent.stdout)
        .expect("help is UTF-8")
        .lines()
        .any(|line| line.starts_with("Usage: ken ")));

    let empty = run(&[""]);
    assert_eq!(empty.status.code(), Some(1));
    assert!(
        empty.stderr.starts_with(b"error[C0003]: "),
        "{:?}",
        empty.stderr
    );
    assert!(empty.stderr.ends_with(b"\n"));
}

#[cfg(unix)]
#[test]
fn non_utf8_subcommand_is_coded_failure() {
    use std::ffi::OsString;
    use std::os::unix::ffi::OsStringExt;

    let output = Command::new(env!("CARGO_BIN_EXE_ken"))
        .arg(OsString::from_vec(vec![0xff]))
        .output()
        .expect("run ken with a non-UTF-8 subcommand");
    assert_eq!(output.status.code(), Some(1));
    assert!(
        output.stderr.starts_with(b"error[C0003]: "),
        "{:?}",
        output.stderr
    );
    assert!(output.stderr.ends_with(b"\n"));
}
