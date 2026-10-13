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

/// Check the kernel and trust on the checked source before starting the
/// interpreter clock. The worker owns its elaboration environment because
/// evaluator values and globals need not be Send. A timeout deliberately does
/// not join the potentially exponential worker; the test process can exit.
fn check_statically_regular_j_cost(name: &'static str, source: &'static str) {
    use std::collections::BTreeSet;
    use std::sync::mpsc;
    use std::time::Duration;

    let (ready_tx, ready_rx) = mpsc::sync_channel(1);
    let (result_tx, result_rx) = mpsc::sync_channel(1);
    std::thread::spawn(move || {
        let nat = |n: usize| (0..n).fold(String::from("Zero"), |acc, _| format!("Suc ({acc})"));
        let n16 = nat(16);
        let n15 = nat(15);
        let n20 = nat(20);
        let mut env = ElabEnv::new().expect("checked prelude");
        let trust_before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
        env.elaborate_file(source).expect("checked J recursion");
        env.elaborate_decl(&format!(
            "theorem cost_right : Equal Nat ({name} ({n16})) ({n16}) = Proved"
        ))
        .expect("the kernel must prove the constructor at n=16");
        let wrong = env
            .elaborate_decl(&format!(
                "theorem cost_wrong : Equal Nat ({name} ({n16})) ({n15}) = Proved"
            ))
            .expect_err("the kernel must reject the n−1 twin");
        assert!(
            matches!(
                &wrong,
                ElabError::KernelRejected {
                    error: KernelError::TypeMismatch { .. },
                    ..
                }
            ),
            "{name}: the wrong twin must be kernel-rejected, not parser-rejected: {wrong:?}"
        );
        let id = env
            .elaborate_decl(&format!("const cost_observed : Nat = {name} ({n20})"))
            .expect("same checked source must supply the runtime observation");
        assert_eq!(
            env.env.trusted_base().into_iter().collect::<BTreeSet<_>>(),
            trust_before,
            "{name}: no new trust assumptions"
        );
        let Some(Decl::Transparent { body, ty, .. }) = env.env.lookup(id) else {
            panic!("{name}: checked runtime observation must be transparent");
        };
        ready_tx.send(()).expect("clock start delivered");
        let value = eval_checked(body, ty, &env.env, &mut EvalStore::new())
            .expect("kernel-classified erasure must evaluate");
        let mut current = &value;
        let mut count = 0;
        loop {
            match current {
                EvalVal::Ctor { id, args, .. } if *id == env.prelude_env.zero_id => {
                    assert!(args.is_empty(), "Zero is nullary");
                    break;
                }
                EvalVal::Ctor { id, args, .. } if *id == env.prelude_env.suc_id => {
                    let [previous] = args.as_slice() else {
                        panic!("{name}: Suc must have one predecessor");
                    };
                    count += 1;
                    current = previous;
                }
                other => panic!("{name}: expected a Nat constructor, got {other:?}"),
            }
        }
        let _ = result_tx.send(count);
    });
    ready_rx
        .recv_timeout(Duration::from_secs(120))
        .unwrap_or_else(|error| panic!("{name}: checked setup failed before the clock: {error}"));
    let count = result_rx
        .recv_timeout(Duration::from_secs(5))
        .unwrap_or_else(|error| {
            panic!("{name}: eval_checked must finish within 5s in debug: {error}")
        });
    assert_eq!(count, 20, "{name}: interpreter must agree with kernel");
}

// Promise class: durable invariant (42 §3.2, §4). MEASURED: the same-endpoint
// constant-motive J returns Nat 20 within 5s after the kernel proves 16 and
// rejects 15 on the checked source. CLAIMED: erased equal endpoints need no
// repeated computation. THE GAP: f2/f3 independently exercise the two halves.
#[test]
fn statically_regular_same_endpoint_j_returns_base_without_eager_cost() {
    check_statically_regular_j_cost(
        "f1",
        "theorem rk (k : Nat) : Equal Nat k k = Refl\n\
         fn f1 (n : Nat) : Nat = match n {\n\
           Zero ↦ Zero; Suc k ↦ J (λp _. Nat) (Suc (f1 k)) (rk (f1 k))\n\
         }",
    );
}

// Promise class: durable invariant (42 §3.2). MEASURED: an index-dependent
// motive with identical endpoint terms returns Nat 20 within 5s; the kernel
// independently proves 16 and rejects 15. CLAIMED: structural equality of
// erased endpoints establishes regularity even when the motive uses its index.
// THE GAP: this does not cover distinct endpoints with dependent motives (f4).
#[test]
fn statically_regular_dependent_motive_same_endpoint_j_avoids_eager_cost() {
    check_statically_regular_j_cost(
        "f2",
        "fn K (a : Type) (n : Nat) : Type = a\n\
         theorem rk (k : Nat) : Equal Nat k k = Refl\n\
         fn f2 (n : Nat) : Nat = match n {\n\
           Zero ↦ Zero; Suc k ↦ J (λp _. K Nat p) (Suc (f2 k)) (rk (f2 k))\n\
         }",
    );
}

// Promise class: durable invariant (42 §3.2, §4). MEASURED: a constant motive
// with distinct endpoint terms still returns Nat 20 within 5s, and the kernel
// proves 16 and refuses 15. CLAIMED: a motive independent of its index needs
// no endpoint evaluation even when the equation uses a nontrivial lemma. THE
// GAP: an index-mentioning motive with distinct endpoints still needs C5.
#[test]
fn statically_regular_constant_motive_distinct_endpoints_avoids_eager_cost() {
    check_statically_regular_j_cost(
        "f3",
        "fn idn (n : Nat) : Nat = n\n\
         theorem lemi (x : Nat) : Equal Nat (idn x) x = Refl\n\
         fn f3 (n : Nat) : Nat = match n {\n\
           Zero ↦ Zero; Suc k ↦ J (λp _. Nat) (Suc (f3 k)) (lemi (f3 k))\n\
         }",
    );
}
