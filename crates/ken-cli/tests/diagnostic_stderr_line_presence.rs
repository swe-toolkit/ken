//! Spec 70 §1.1/§1.3: durable invariants for whole coded stderr lines and
//! absent-versus-present subcommand parsing. Fixture bytes are not a promise
//! about diagnostic message wording.

use std::fs;
use std::path::Path;
use std::process::{Command, Output};

fn run(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_ken"))
        .args(args)
        .output()
        .expect("run ken")
}

fn write_source(dir: &Path, name: &str, source: &str) -> String {
    let path = dir.join(name);
    fs::write(&path, source).expect("write Ken fixture");
    path.to_str().expect("temporary path is UTF-8").to_owned()
}

fn host_exit_source(bytes: &str, exit: &str) -> String {
    format!(
        "program capabilities FS APartial\n\
         proc act : IO Unit visits [Console] =\n\
           Vis ConsoleOp console_resp Unit (Write Stderr (bytes_encode {bytes}))\n\
             (\\_. Ret ConsoleOp console_resp Unit MkUnit)\n\
         proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)\n\
           : HostIO APartial ExitCode visits [Console] =\n\
           host_program_then APartial act {exit}\n"
    )
}

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
    let dir = tempfile::tempdir().unwrap();
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

#[test]
fn post_execution_x_diagnostic_starts_on_its_own_line() {
    let dir = tempfile::tempdir().unwrap();
    // A checked continuation becomes stuck after the Console write, so run_io
    // returns X0003 rather than an admitted program outcome.
    let source = write_source(
        dir.path(),
        "stuck-tree.ken",
        r#"program capabilities FS APartial
proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode visits [Console] =
  bind (Coproduct (FSOp APartial) AmbientOp) (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp) Unit ExitCode
    (host_console APartial Unit (Vis ConsoleOp console_resp Unit (Write Stderr (bytes_encode "partial")) (\_. Ret ConsoleOp console_resp Unit MkUnit)))
    (\_ . match (intToInt8 (sub_int 0 128)) { Some m |-> (match (eq_int (int8_to_int (neg_int8 m)) 0) { True |-> host_exit APartial ExitCode.Success ; False |-> host_exit APartial ExitCode.Success }) ; None |-> host_exit APartial ExitCode.Success })
"#,
    );
    let output = run(&["run", &source]);
    assert_whole_line_after_program_bytes(&output, b"partial", "X0003");
}

#[test]
fn finished_program_line_adds_no_blank_line_and_success_keeps_exact_stderr() {
    let dir = tempfile::tempdir().unwrap();
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
