//! Interpreter and typed-direction client for spec 38 §1.11.1–§1.11.2.

#![cfg(all(target_os = "linux", target_env = "gnu"))]

use std::path::PathBuf;
use std::process::Command;

use ken_elaborator::{ElabEnv, ElabError};

const CLIENT: &str = include_str!("fixtures/lang_process_child_pipe_write_home.ken");

fn catalog_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages")
}

/// Promise class: durable invariant. MEASURED: the actual `ken run`
/// interpreter executes a child with pipe-backed stdin/stdout, and its Ken
/// continuation produces Success only after a positive write, successful
/// write-end close, and an Exited 0 wait in that order. CLAIMED: explicit
/// pipe imports make stdin writing and early EOF expressible and executable.
/// THE GAP: this exercises one short transfer and the interpreter, not native
/// ResourceStreamWrite availability or arbitrary partial-write completion.
#[test]
fn interpreter_writes_child_stdin_closes_early_then_observes_exit() {
    let fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/lang_process_child_pipe_write_home.ken");
    let output = Command::new("timeout")
        .arg("120s")
        .arg(env!("CARGO_BIN_EXE_ken"))
        .arg("run")
        .arg(fixture)
        .output()
        .expect("bounded Ken interpreter CLI");
    assert!(
        output.status.success(),
        "interpreter exit {:?}: {}",
        output.status.code(),
        String::from_utf8_lossy(&output.stderr)
    );
}

/// Promise class: durable invariant. MEASURED: only the pipe direction is
/// changed on the same checked client, and the checked argument type rejects
/// a PipeRead where write_pipe requires PipeWrite. CLAIMED: the operation
/// cannot write into a read end, regardless of its runtime identity.
/// THE GAP: this is a type refusal, not a dynamic wrong-handle probe; the
/// accepted client is run separately above.
#[test]
fn write_pipe_rejects_a_read_end_at_type_checking() {
    let old = "write_pipe AFull writable buffer span";
    let replacement = "write_pipe AFull readable buffer span";
    assert_eq!(CLIENT.matches(old).count(), 1, "one write call to perturb");
    let wrong = CLIENT.replacen(old, replacement, 1);
    let mut env = ElabEnv::new().expect("prelude");
    match env.elaborate_isolated_file_strict_v1(&[catalog_root()], &wrong) {
        Err(ElabError::TypeMismatch { span, reason }) => {
            assert!(
                span.start >= wrong.find("write_pipe AFull readable").unwrap()
                    && span.end <= wrong.len(),
                "typed refusal must occur at the changed write call: {span:?}, {reason}"
            );
        }
        Err(error) => panic!("wrong-direction call refused for the wrong reason: {error:?}"),
        Ok(_) => panic!("PipeRead was accepted by write_pipe"),
    }
}
