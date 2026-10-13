//! A behavioural stack guard for the recursive elaborator cycle.
//!
//! Promise class: durable, bounded performance invariant. MEASURED: a fixed
//! dependent-match depth elaborates on the stated child-worker stack. CLAIMED:
//! growth in check/infer/check_dependent_branch_body's cycle is caught before
//! the default 2 MiB worker overflows on unrelated suites. THE GAP: this
//! witnesses one depth and source shape, not every possible elaboration path.

use std::io::Read;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use ken_elaborator::ElabEnv;

// Measured 2026-10-13 with rustc 1.97.1, LLVM 22.1.6, Linux x86_64,
// Cargo test profile (Ken crates unoptimized) at depth 24. A fresh child with
// RUST_MIN_STACK removed passed at 2,176 KiB; at 2,112 KiB it overflowed
// AFTER full-prelude registration and BEFORE the nested-match completion.
// This is the first passing 64-KiB grid point, not an exact-byte minimum.
// Reproduce with the scratch AC-0 child-process sweep: on the repaired tip,
// run scripts/ken-cargo test -p ken-elaborator --test
// elab_recursive_frame_budget -- nested_dependent_match_fits_stated_stack.
const GUARD_STACK_BYTES: usize = 2_176 * 1024;
const DEPENDENT_MATCH_DEPTH: usize = 24;
const CHILD_TIMEOUT: Duration = Duration::from_secs(600);
const REGRESSION_MESSAGE: &str = "the elaborator's recursive cycle \
    (check / infer / check_dependent_branch_body) grew: outline the new block \
    with #[inline(never)], or re-measure and re-record GUARD_STACK with the evidence";

fn goal(index: &str) -> String {
    format!(
        "Equal Nat (frame_budget_carrier (frame_budget_id {index}) x) \
         (frame_budget_carrier (frame_budget_id {index}) x)"
    )
}

// Each branch binds a strictly smaller Nat scrutinee. The annotated local
// proof puts each inner match in checking mode at a goal that mentions that
// scrutinee. The checked scrutinee itself is an application, entering infer
// once per level before check_dependent_branch_body checks its methods.
fn nested_match(index: &str, depth: usize) -> String {
    if depth == 0 {
        return "Refl".to_owned();
    }
    let successor = format!("k{depth}");
    format!(
        "let p{depth} : {} = match (frame_budget_id {index}) \
         {{ Zero |-> Refl; Suc {successor} |-> {} }} in Refl",
        goal(index),
        nested_match(&successor, depth - 1),
    )
}

#[test]
fn nested_dependent_match_fits_stated_stack() {
    let mut child = Command::new(std::env::current_exe().expect("test binary path"))
        .args([
            "--exact",
            "frame_budget_child",
            "--ignored",
            "--nocapture",
            "--test-threads=1",
        ])
        .env("FRAME_BUDGET_CHILD", "1")
        .env_remove("RUST_MIN_STACK")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn an isolated, stated-stack test process");
    let started = Instant::now();
    let timed_out = loop {
        if child.try_wait().expect("poll stated-stack child").is_some() {
            break false;
        }
        if started.elapsed() >= CHILD_TIMEOUT {
            child.kill().expect("kill stalled child");
            break true;
        }
        std::thread::sleep(Duration::from_millis(50));
    };
    let mut stdout = String::new();
    let mut stderr = String::new();
    child
        .stdout
        .take()
        .expect("child stdout")
        .read_to_string(&mut stdout)
        .expect("read child stdout");
    child
        .stderr
        .take()
        .expect("child stderr")
        .read_to_string(&mut stderr)
        .expect("read child stderr");
    let status = child.wait().expect("child exit");
    assert!(
        !timed_out
            && status.success()
            && stdout.contains("test frame_budget_child ... ok")
            && stderr.contains("FRAME_BUDGET PRELUDE_OK")
            && stderr.contains("FRAME_BUDGET CYCLE_OK"),
        "{REGRESSION_MESSAGE}; status={status:?}; timeout={timed_out}; \
         stdout={stdout:?}; stderr={stderr:?}"
    );
}

#[test]
#[ignore = "run by nested_dependent_match_fits_stated_stack in an isolated process"]
fn frame_budget_child() {
    assert_eq!(
        std::env::var("FRAME_BUDGET_CHILD").as_deref(),
        Ok("1"),
        "only the parent invokes this child"
    );
    std::thread::Builder::new()
        .name("elaborator-recursive-frame-budget".into())
        .stack_size(GUARD_STACK_BYTES)
        .spawn(|| {
            let mut env = ElabEnv::new().expect("full prelude registration");
            eprintln!("FRAME_BUDGET PRELUDE_OK");
            env.elaborate_decl("fn frame_budget_id (n : Nat) : Nat = n")
                .expect("checked identity application in each scrutinee");
            env.elaborate_decl(
                "fn frame_budget_carrier (n : Nat) (x : Nat) : Nat = \
                 match n { Zero |-> x; Suc k |-> Suc (frame_budget_carrier k x) }",
            )
            .expect("checked carrier whose result depends on the matched Nat");
            let declaration = format!(
                "theorem frame_budget_depth_{} (n : Nat) (x : Nat) : {} = \
                 match (frame_budget_id n) {{ Zero |-> Refl; Suc k{} |-> {} }}",
                DEPENDENT_MATCH_DEPTH,
                goal("n"),
                DEPENDENT_MATCH_DEPTH,
                nested_match(
                    &format!("k{}", DEPENDENT_MATCH_DEPTH),
                    DEPENDENT_MATCH_DEPTH - 1,
                )
            );
            env.elaborate_decl(&declaration)
                .expect("fixed-depth dependent proof must elaborate");
            eprintln!("FRAME_BUDGET CYCLE_OK");
        })
        .expect("spawn stated-stack worker")
        .join()
        .expect("stated-stack worker must complete");
}
