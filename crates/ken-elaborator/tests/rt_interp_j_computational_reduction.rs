//! Computational J is checked by the kernel and evaluated by the interpreter.
//! Spec 15 §4 and 42 §3.6; Architect ruling evt_2rjaht2mjnsje.

use ken_elaborator::{ElabEnv, ElabError};
use ken_interp::eval::{eval_checked, EvalStore, EvalVal};
use ken_kernel::{Decl, KernelError};

// Promise class: durable invariant. MEASURED: each checked source has an
// accepted constructor theorem, a rejected wrong-constructor theorem, and
// eval_checked yields that constructor on the SAME source. CLAIMED: the
// interpreter's computational J agrees with kernel conversion. THE GAP:
// dependent graph motives need independent runtime probes (AC-2).
fn check_j_case(source: &str, result: &str, right: &str, wrong: &str, expected: usize) {
    let mut env = ElabEnv::new().expect("checked prelude");
    env.elaborate_file(source)
        .expect("source with J is admitted");
    env.elaborate_decl(&format!(
        "theorem result_is_right : Equal Nat {result} ({right}) = Proved"
    ))
    .expect("kernel derives the right constructor");
    let wrong_error = env
        .elaborate_decl(&format!(
            "theorem result_is_wrong : Equal Nat {result} ({wrong}) = Proved"
        ))
        .expect_err("the wrong constructor must be kernel-rejected");
    assert!(
        matches!(
            &wrong_error,
            ElabError::KernelRejected {
                error: KernelError::TypeMismatch { .. },
                ..
            }
        ),
        "the kernel, not a parser error, must reject the wrong constructor: {wrong_error:?}"
    );

    let id = env
        .elaborate_decl(&format!("const observed_j : Nat = {result}"))
        .expect("same source checks for interpreter observation");
    let Some(Decl::Transparent { body, ty, .. }) = env.env.lookup(id) else {
        panic!("checked J observation must have a transparent body");
    };
    let value = eval_checked(body, ty, &env.env, &mut EvalStore::new())
        .expect("kernel-classified interpreter erasure");
    let mut count = 0;
    let mut current = &value;
    loop {
        match current {
            EvalVal::Ctor { id, args, .. } if *id == env.prelude_env.zero_id => {
                assert!(args.is_empty(), "Zero is nullary");
                break;
            }
            EvalVal::Ctor { id, args, .. } if *id == env.prelude_env.suc_id => {
                count += 1;
                current = args.first().expect("Suc carries predecessor");
            }
            other => panic!("expected a checked Nat constructor, got {other:?}"),
        }
    }
    assert_eq!(count, expected, "interpreter must agree with kernel");
}

#[test]
fn checked_constant_motive_j_at_true_yields_one() {
    check_j_case(
        "fn tr (x : Bool) (h : Equal Bool x True) : Nat = \
         J (λp _. Nat) (Suc Zero) h\n\
         const c : Nat = tr True Proved",
        "c",
        "Suc Zero",
        "Suc (Suc Zero)",
        1,
    );
}

#[test]
fn checked_same_endpoint_j_at_true_yields_two() {
    check_j_case(
        "fn trId (x : Bool) (h : Equal Bool x x) : Nat = \
         J (λp _. Nat) (Suc (Suc Zero)) h\n\
         const c2 : Nat = trId True Proved",
        "c2",
        "Suc (Suc Zero)",
        "Suc Zero",
        2,
    );
}

#[test]
fn checked_computed_endpoint_and_value_base_j_yields_one() {
    // Steward evt_42tsz9t8r26dp authorized this exact third AC-1 row.
    check_j_case(
        "fn same_nat (n : Nat) : Nat = \
           match n { Zero ↦ Zero; Suc k ↦ Suc k }\n\
         fn tv (n : Nat) (h : Equal Nat (same_nat n) n) : Nat = \
           J (λp _. Nat) n h\n\
         const v : Nat = tv (Suc Zero) Proved",
        "v",
        "Suc Zero",
        "Suc (Suc Zero)",
        1,
    );
}
