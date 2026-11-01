//! Increment 2: the checked HostIO composition must drive both host actions.
//! The CLI route remains Legacy until Increment 3; the Strict import/control
//! for the same composition is exercised by the elaborator's named target.

use std::path::PathBuf;
use std::process::Command;

/// Promise class: durable invariant (33 §3.2.1; 36 §2.2–2.4).
/// MEASURED: the real `ken run` host writes two distinct lines in order.
/// CLAIMED: host_bind sequences rather than merely building/dropping a tree.
/// THE GAP: this observes one concrete value/authority; the independent
/// Strict entry pin establishes the imported name and checked identity.
#[test]
fn host_bind_drives_two_print_lines_before_host_pure_exit() {
    let source = r#"program capabilities FS APartial
proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode visits [Console] =
  host_bind APartial Unit ExitCode
    (host_console APartial Unit (print_line "one"))
    (\u. host_bind APartial Unit ExitCode
      (host_console APartial Unit (print_line "two"))
      (\v. host_pure APartial ExitCode ExitCode.Success))
"#;
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("isolated-inc2-host-bind");
    std::fs::create_dir_all(&dir).unwrap();
    let program = dir.join("program.ken");
    std::fs::write(&program, source).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_ken"))
        .arg("run")
        .arg(&program)
        .output()
        .unwrap();
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        output.stdout, b"one\ntwo\n",
        "neither action can be silently dropped"
    );
}
