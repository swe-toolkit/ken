//! Spec 36 §2.4: a performed let RHS runs before its continuation.
//! Promise class: durable invariant. MEASURED: the checked CLI reference
//! driver writes each action's byte once in source order. CLAIMED: a local
//! let cannot silently drop a performed HostIO or IO action. THE GAP: the
//! interpreter observation is one concrete signature and authority; the
//! elaborator tests also inspect the emitted bind and inference routes.

use std::process::{Command, Output};

const PREFIX: &str = "program capabilities FS APartial\n\
proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)\n\
  : HostIO APartial ExitCode visits [Console] =\n";

fn run_source(name: &str, source: &str) -> Output {
    let root = std::env::temp_dir().join(format!("ken-let-hostio-{}", std::process::id()));
    std::fs::create_dir_all(&root).expect("create fixture root");
    let path = root.join(format!("{name}.ken"));
    std::fs::write(&path, source).expect("write fixture");
    Command::new(env!("CARGO_BIN_EXE_ken"))
        .arg("run")
        .arg(path)
        .output()
        .expect("run checked program")
}

fn run(name: &str, body: &str) -> Output {
    run_source(name, &format!("{PREFIX}{body}\n"))
}

fn expect_stdout(name: &str, body: &str, expected: &[u8]) {
    let output = run(name, body);
    assert_eq!(
        output.status.code(),
        Some(0),
        "{name}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        output.stdout, expected,
        "{name}: performed actions must run once in order"
    );
}

#[test]
fn performed_let_single_grouped_and_io_proc_run_in_order() {
    let single = r#"  let x = host_console APartial (Result IOError Unit)
    (write ConsoleStream.Stdout (bytes_encode "a"))
  in host_bind APartial (Result IOError Unit) ExitCode
    (host_console APartial (Result IOError Unit)
      (write ConsoleStream.Stdout (bytes_encode "b")))
    (\_. host_pure APartial ExitCode ExitCode.Success)"#;
    expect_stdout("single", single, b"ab");

    let grouped = r#"  let x = host_console APartial (Result IOError Unit)
    (write ConsoleStream.Stdout (bytes_encode "a"));
        y = host_console APartial (Result IOError Unit)
    (write ConsoleStream.Stdout (bytes_encode "b"))
  in host_bind APartial (Result IOError Unit) ExitCode
    (host_console APartial (Result IOError Unit)
      (write ConsoleStream.Stdout (bytes_encode "c")))
    (\_. host_pure APartial ExitCode ExitCode.Success)"#;
    expect_stdout("grouped", grouped, b"abc");

    // A proc's IO body reaches the same check_let_once path as main's HostIO.
    let io_proc = r#"proc action : IO (Result IOError Unit) visits [Console] =
  let x = write ConsoleStream.Stdout (bytes_encode "a") in
    write ConsoleStream.Stdout (bytes_encode "b")
proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode visits [Console] =
  host_bind APartial (Result IOError Unit) ExitCode
    (host_console APartial (Result IOError Unit) action)
    (\_. host_pure APartial ExitCode ExitCode.Success)"#;
    let output = run_source(
        "io-proc",
        &format!("program capabilities FS APartial\n{io_proc}\n"),
    );
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(output.stdout, b"ab");
}

#[test]
fn annotated_let_binds_the_performed_result_not_the_action() {
    let body = r#"  let r : Result IOError Unit = host_console APartial (Result IOError Unit)
    (write ConsoleStream.Stdout (bytes_encode "a"))
  in host_bind APartial (Result IOError Unit) ExitCode
    (host_console APartial (Result IOError Unit)
      (write ConsoleStream.Stdout (bytes_encode "b")))
    (\_. host_pure APartial ExitCode ExitCode.Success)"#;
    expect_stdout("annotated-result", body, b"ab");

    let mismatch = body.replace(
        "let r : Result IOError Unit",
        "let p : HostIO APartial (Result IOError Unit)",
    );
    let output = run("annotated-action-type-mismatch", &mismatch);
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("annotated result type differs from the action's result type"),
        "{stderr}"
    );
}

#[test]
fn nested_action_result_is_bound_and_then_run_in_the_continuation() {
    let source = r#"program capabilities FS APartial
proc make_outer : HostIO APartial (HostIO APartial Unit) visits [Console] =
  host_bind APartial (Result IOError Unit) (HostIO APartial Unit)
    (host_console APartial (Result IOError Unit)
      (write ConsoleStream.Stdout (bytes_encode "a")))
    (\_. host_pure APartial (HostIO APartial Unit)
      (host_pure APartial Unit MkUnit))
proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode visits [Console] =
  let t : HostIO APartial Unit = make_outer in
    host_bind APartial Unit ExitCode t
      (\_. host_program APartial (print_line "b"))
"#;
    let output = run_source("nested-action-result", source);
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        output.stdout, b"ab\n",
        "outer action executes once before its returned action is consumed"
    );
}

#[test]
fn performed_let_result_is_available_to_match_in_the_continuation() {
    let body = r#"  let r = host_console APartial (Result IOError Unit)
    (write ConsoleStream.Stdout (bytes_encode "a"))
  in match r {
    Ok _ |-> host_program APartial (print_line "b");
    Err _ |-> host_program APartial (print_line "e")
  }"#;
    expect_stdout("matched-result", body, b"ab\n");
}

#[test]
fn valid_cross_signature_let_fails_closed_until_inclusion_is_built() {
    let body = r#"  let x = write ConsoleStream.Stdout (bytes_encode "a") in
    host_program APartial (print_line "b")"#;
    let output = run("inclusion-limitation", body);
    assert_eq!(
        output.status.code(),
        Some(1),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output.stdout.is_empty());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains(
            "unsupported: 36 §2.4 inclusion into the enclosing effect signature is not built"
        ),
        "{stderr}"
    );
}
