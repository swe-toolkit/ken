//! Batch CLI diagnostics: carrier-selected codes, status, and uncoded outcomes.
//! Contract-derived tests here are normative compatibility vectors for the
//! ledger identities and durable invariants for line shape and program outcomes.

use std::fs;
use std::path::Path;
use std::process::{Command, Output};

const RESOURCE_PROFILE: &str = r#"{"runtime":{"invocation_epochs":0},"call_events":{"event_generations":0,"live_pending_slots":0},"invocation":{"nodes":0,"words":0,"data_bytes":0,"native_int_limbs":0},"persistent":{"nodes":0,"words":0,"data_bytes":0,"native_int_limbs":0}}"#;

fn run(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_ken"))
        .args(args)
        .output()
        .expect("run the ken CLI")
}

fn coded_lines(output: &Output) -> Vec<&str> {
    std::str::from_utf8(&output.stderr)
        .expect("CLI stderr is UTF-8 for these fixtures")
        .lines()
        .filter(|line| line.starts_with("error["))
        .collect()
}

fn assert_codes(output: &Output, expected: &[&str]) {
    assert_eq!(output.status.code(), Some(1), "stderr: {:?}", output.stderr);
    let lines = coded_lines(output);
    assert_eq!(lines.len(), expected.len(), "stderr: {:?}", output.stderr);
    for code in expected {
        let prefix = format!("error[{code}]: ");
        assert!(
            lines.iter().any(|line| line.starts_with(&prefix)),
            "missing {prefix:?} in {lines:?}"
        );
    }
    assert!(
        lines.iter().all(|line| line.matches("error[").count() == 1),
        "each diagnostic has one code slot: {lines:?}"
    );
}

fn write_source(dir: &Path, name: &str, source: &str) -> String {
    let path = dir.join(name);
    fs::write(&path, source).expect("write Ken fixture");
    path.to_str().expect("temporary path is UTF-8").to_owned()
}

fn main_source(exit: &str) -> String {
    format!(
        "program capabilities FS APartial\n\
         proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)\n\
           : HostIO APartial ExitCode visits [Console] = host_exit APartial {exit}\n"
    )
}

#[test]
fn cli_argument_and_file_failures_have_distinct_c_carriers() {
    let dir = tempfile::tempdir().expect("private temp directory");
    let source = write_source(dir.path(), "app.ken", &main_source("Success"));
    let missing = dir.path().join("missing.ken");
    let missing = missing.to_str().unwrap();

    assert_codes(&run(&["not-a-subcommand"]), &["C0003"]);
    let embedded_newline = run(&["unknown\ncommand"]);
    assert_codes(&embedded_newline, &["C0003"]);
    assert!(
        coded_lines(&embedded_newline)[0].contains("unknown\\ncommand"),
        "newline must remain inside the one diagnostic line"
    );
    assert_codes(&run(&["run"]), &["C0001"]);
    assert_codes(&run(&["run", &source, "--unexpected"]), &["C0002"]);
    assert_codes(&run(&["run", missing]), &["C0020"]);
    assert_codes(&run(&["check"]), &["C0018"]);
    assert_codes(&run(&["check", missing]), &["C0019"]);
    assert_codes(&run(&["native-build"]), &["C0004"]);
    assert_codes(&run(&["native-build", &source]), &["C0005"]);
    assert_codes(&run(&["native-build", &source, missing]), &["C0006"]);
    assert_codes(
        &run(&["native-build", &source, missing, missing, "extra"]),
        &["C0007"],
    );
    assert_codes(
        &run(&["native-build", &source, missing, missing]),
        &["C0008"],
    );
    let malformed_profile = write_source(dir.path(), "bad-profile.json", "not json\n");
    assert_codes(
        &run(&["native-build", &source, missing, &malformed_profile]),
        &["C0009"],
    );
    let valid_profile = write_source(dir.path(), "profile.json", RESOURCE_PROFILE);
    assert_codes(
        &run(&["native-build", missing, missing, &valid_profile]),
        &["C0010"],
    );
    assert_codes(&run(&["fmt", "--unexpected"]), &["C0012"]);
    assert_codes(&run(&["fmt"]), &["C0013"]);
}

#[cfg(unix)]
#[test]
fn invalid_utf8_fmt_path_is_a_coded_cli_failure() {
    use std::ffi::OsString;
    use std::os::unix::ffi::OsStringExt;
    let output = Command::new(env!("CARGO_BIN_EXE_ken"))
        .arg("fmt")
        .arg(OsString::from_vec(vec![b'/', 0xff]))
        .output()
        .expect("run ken fmt with a non-UTF-8 path");
    assert_codes(&output, &["C0011"]);
}

#[test]
fn fmt_reports_each_failed_path_and_delegates_formatter_errors_to_e() {
    let dir = tempfile::tempdir().expect("private temp directory");
    let noncanonical = write_source(dir.path(), "unformatted.ken", "const x:Nat=Zero\n");
    let unreadable = dir.path().join("absent.ken");
    let unreadable = unreadable.to_str().unwrap();
    let output = run(&["fmt", "--check", &noncanonical, unreadable]);
    assert_codes(&output, &["C0016", "C0014"]);
    assert_codes(&run(&["fmt", "wrong.extension"]), &["C0014"]);
    let unsupported = write_source(dir.path(), "wrong.extension", "const x : Nat = Zero\n");
    assert_codes(&run(&["fmt", &unsupported]), &["C0015"]);

    let malformed = write_source(dir.path(), "malformed.ken", "fn\n");
    assert_codes(&run(&["fmt", &malformed]), &["E0001"]);
}

#[test]
fn wrapped_carriers_keep_e_and_x_and_admitted_program_statuses_are_uncoded() {
    let dir = tempfile::tempdir().expect("private temp directory");
    let malformed = write_source(dir.path(), "malformed.ken", "fn\n");
    assert_codes(&run(&["check", &malformed]), &["E0001"]);
    let output_dir = dir.path().join("native-out");
    let output_dir = output_dir.to_str().unwrap();
    let profile = write_source(dir.path(), "native-profile.json", RESOURCE_PROFILE);
    assert_codes(
        &run(&["native-build", &malformed, output_dir, &profile]),
        &["E0001"],
    );
    let missing_main = write_source(
        dir.path(),
        "missing-main.ken",
        "program capabilities FS APartial\nconst helper : Nat = Zero\n",
    );
    assert_codes(&run(&["run", &missing_main]), &["X0005"]);
    let non_failure = run(&["check", &missing_main]);
    assert_eq!(non_failure.status.code(), Some(0));
    assert!(coded_lines(&non_failure).is_empty());

    for (exit, status) in [("Success", 0), ("(Failure 1)", 1), ("(Failure 2)", 2)] {
        let path = write_source(
            dir.path(),
            &format!("outcome-{status}.ken"),
            &main_source(exit),
        );
        let output = run(&["run", &path]);
        assert_eq!(
            output.status.code(),
            Some(status),
            "stderr: {:?}",
            output.stderr
        );
        assert!(
            coded_lines(&output).is_empty(),
            "stderr: {:?}",
            output.stderr
        );
    }
}
