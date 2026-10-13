//! Behavioural stack guards for two recursive elaborator cycles.
//!
//! Promise class: durable, bounded performance invariant. MEASURED: each
//! fixed-depth workload completes on its stated stack after the full prelude.
//! CLAIMED: a frame increase on the match or inference cycle reddens its own
//! row before unrelated CI shards overflow. THE GAP: these two paths are
//! representatives, not a universal bound for every source or AST shape.
//!
//! The threshold alone does not attribute an overflow. The calibration below
//! also records the low-stack unwind, and each row was separately tested
//! against a compile-preserving production inline mutation.

use std::io::Read;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use ken_elaborator::{Decl, ElabEnv, Expr, Span};

// Measured on 2026-10-13 with rustc 1.97.1 / LLVM 22.1.6, Linux x86_64,
// via scripts/ken-cargo test -p ken-elaborator --test
// elab_recursive_frame_budget -- --test-threads=1. The sizes are the first
// passing 64-KiB grid points, not exact-byte minima. Each child removes
// RUST_MIN_STACK and states its own Builder::stack_size, so an ambient machine
// setting cannot silently change the detector.
//
// Row M, 48 dependent Nat matches, one application per scrutinee: 3,072 KiB
// passes; 3,008 KiB overflows after PRELUDE_OK. At the latter size, the
// stripped-binary lldb unwind has 47 each of check_match_dependent_mode,
// check_dependent_branch_body and its closure, plus 96 check frames; the
// terminal operation is kernel subst_levels (three frames), not a second
// recursive syntax-walker population. Inlining check_match_dependent_mode
// grows its caller by ~24.5 KiB per held level and reddens this row at 3,072.
const MATCH_DEPTH: usize = 48;
const MATCH_STACK_BYTES: usize = 3_072 * 1024;
// Row I, 128 nested ascriptions constructed from a parsed shallow function's
// public Expr AST: 1,920 KiB passes; 1,856 KiB overflows after PRELUDE_OK.
// The stripped-binary unwind at 1,856 KiB has 125 each of infer,
// infer_ascription and check_inferred_without_group_transport, plus 126
// check frames. Inlining infer_application on these NON-application infer
// frames reddens this row at 1,920. Constructing the nested ascriptions as
// source instead would make the parser's eight frames per level bind first;
// public AST construction isolates the elaborator cycle under test.
const INFER_DEPTH: usize = 128;
const INFER_STACK_BYTES: usize = 1_920 * 1024;
const CHILD_TIMEOUT: Duration = Duration::from_secs(600);
const REGRESSION_MESSAGE: &str = "the elaborator's recursive cycle \
    (check / infer / check_dependent_branch_body) grew: outline the new block \
    with #[inline(never)], or re-measure and re-record GUARD_STACK with the evidence";

fn scrutinee(index: &str) -> String {
    format!("(frame_budget_id {index})")
}

fn goal(index: &str) -> String {
    let operand = scrutinee(index);
    format!("Equal Nat {operand} {operand}")
}

fn nested_match(index: &str, depth: usize) -> String {
    if depth == 0 {
        return "Refl".to_owned();
    }
    let successor = format!("k{depth}");
    format!(
        "let p{depth} : {} = match {} \
         {{ Zero |-> Proved; Suc {successor} |-> {} }} in Refl",
        goal(index),
        scrutinee(index),
        nested_match(&successor, depth - 1),
    )
}

fn run_guard(row: &str, bytes: usize, depth: usize) {
    let mut child = Command::new(std::env::current_exe().expect("test binary path"))
        .args([
            "--exact",
            "frame_budget_child",
            "--ignored",
            "--nocapture",
            "--test-threads=1",
        ])
        .env("FRAME_BUDGET_CHILD", "1")
        .env("FRAME_BUDGET_ROW", row)
        .env("FRAME_BUDGET_DEPTH", depth.to_string())
        .env("FRAME_BUDGET_STACK_BYTES", bytes.to_string())
        .env_remove("RUST_MIN_STACK")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn isolated stated-stack child");
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
            && stderr.contains(&format!("FRAME_BUDGET PRELUDE_OK row={row} depth={depth}"))
            && stderr.contains(&format!("FRAME_BUDGET CYCLE_OK row={row} depth={depth}")),
        "{REGRESSION_MESSAGE}; row={row}; depth={depth}; stack={bytes}; \
         status={status:?}; timeout={timed_out}; stdout={stdout:?}; stderr={stderr:?}"
    );
}

#[test]
fn nested_dependent_match_cycle_fits_stated_stack() {
    run_guard("M", MATCH_STACK_BYTES, MATCH_DEPTH);
}

#[test]
fn ascription_infer_cycle_fits_stated_stack() {
    run_guard("I", INFER_STACK_BYTES, INFER_DEPTH);
}

#[test]
#[ignore = "invoked in an isolated process by each guard row"]
fn frame_budget_child() {
    assert_eq!(std::env::var("FRAME_BUDGET_CHILD").as_deref(), Ok("1"));
    let row = std::env::var("FRAME_BUDGET_ROW").expect("row selector");
    let depth = std::env::var("FRAME_BUDGET_DEPTH")
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let bytes = std::env::var("FRAME_BUDGET_STACK_BYTES")
        .unwrap()
        .parse::<usize>()
        .unwrap();
    assert!(
        matches!(
            (row.as_str(), depth),
            ("M", MATCH_DEPTH) | ("I", INFER_DEPTH)
        ),
        "only the two fixed-depth guard rows invoke this child"
    );
    std::thread::Builder::new()
        .name(format!("frame-budget-{row}-depth-{depth}"))
        .stack_size(bytes)
        .spawn(move || {
            let mut env = ElabEnv::new().expect("full prelude");
            eprintln!("FRAME_BUDGET PRELUDE_OK row={row} depth={depth}");
            if row == "M" {
                env.elaborate_decl("fn frame_budget_id (n : Nat) : Nat = n")
                    .expect("checked identity applied to each scrutinee");
                let declaration = format!(
                    "theorem frame_budget_depth_{depth} (n : Nat) (x : Nat) : {} = \
                     match {} {{ Zero |-> Proved; Suc k{depth} |-> {} }}",
                    goal("n"),
                    scrutinee("n"),
                    nested_match(&format!("k{depth}"), depth - 1),
                );
                env.elaborate_decl(&declaration)
                    .expect("fixed-depth dependent theorem must elaborate");
            } else {
                // Parse a shallow, real fn declaration and grow only its body.
                // This exercises the public parser/AST plus production module
                // resolution, reassociation, rewriting and elaboration, but not
                // the parser's unrelated deep-source recursion as a guard proxy.
                let declaration = format!("fn frame_budget_ascription_{depth} (x : Nat) : Nat = x");
                let mut decls = ken_elaborator::parser::parse_decls(&declaration)
                    .expect("shallow function source parses");
                let [Decl::ViewDecl {
                    body,
                    ret_ty: Some(ty),
                    ..
                }] = decls.as_mut_slice()
                else {
                    panic!("expected one pure function declaration")
                };
                let mut nested = body.clone();
                for _ in 0..depth {
                    nested = Expr::EAsc(Box::new(nested), Box::new(ty.clone()), Span::zero());
                }
                *body = nested;
                ken_elaborator::modules::expand_and_elaborate(&mut env, &decls)
                    .expect("fixed-depth ascription AST must elaborate");
            }
            eprintln!("FRAME_BUDGET CYCLE_OK row={row} depth={depth}");
        })
        .expect("spawn stated-stack worker")
        .join()
        .expect("stated-stack worker must complete");
}
