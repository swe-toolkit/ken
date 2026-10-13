//! A stated equality is exposed by exactly the head reductions that WHNF
//! takes before observational equality reduction. Spec 16 §2.1, 17 §1, 34 §3.5.
//! Promise class: durable invariant. A new transparent head reduction should
//! preserve these proofs and refuse their unequal-endpoint twins.

use ken_elaborator::{ElabEnv, ElabError};

// As measured for the adjacent Refl predicate suite: a 1,982,464-byte bare
// prelude peak and 3 MiB nested-match minimum warrant an 8 MiB stated stack.
const ELABORATION_STACK_BYTES: usize = 8 * 1024 * 1024;

fn on_elaboration_stack(f: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new()
        .stack_size(ELABORATION_STACK_BYTES)
        .spawn(f)
        .expect("spawn stated-stack elaboration")
        .join()
        .expect("stated-stack elaboration panicked");
}

fn fixture_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("prelude");
    for declaration in [
        "const Id : Prop -> Prop = λt. t",
        "const K : Prop -> Prop -> Prop = λa. λb. a",
        "fn R (p : Prop) (n : Nat) : Prop = K p (match n { Zero ↦ Top; Suc k ↦ R p k })",
        "fn PE (b : Bool) (p : Pair Nat Nat) : Prop = match b { True ↦ Equal (Pair Nat Nat) p p; False ↦ Top }",
        "fn PEF (b : Bool) (p : Pair Nat Nat) (q : Pair Nat Nat) : Prop = match b { True ↦ Equal (Pair Nat Nat) p q; False ↦ Top }",
        "const LetEq : Pair Nat Nat -> Prop = λp. let x = p in Equal (Pair Nat Nat) x x",
        "fn PN (b : Bool) (n : Nat) : Prop = match b { True ↦ Equal Nat n n; False ↦ Top }",
        "const LetN : Nat -> Prop = λn. let x = n in Equal Nat x x",
    ] {
        env.elaborate_decl(declaration)
            .unwrap_or_else(|error| panic!("fixture {declaration}: {error:?}"));
    }
    env
}

const LIST_ARMS: &str = "match xs { Nil ↦ Refl; Cons h t2 ↦ Refl }";
const BOOL_ARMS: &str = "match b { True ↦ Refl; False ↦ t }";

fn theorem(name: &str, goal: &str, body: &str) -> String {
    format!(
        "theorem {name} (b : Bool) (t : Top) (p : Pair Nat Nat) \
         (q : Pair Nat Nat) (z : Nat) (xs : List Nat) : {goal} = {body}"
    )
}

// MEASURED: the three R and five PE/LetEq rows each kernel-check at a
// standalone, List-arm, or Bool-arm route with no added trust. CLAIMED: lawful
// head steps reveal Eq before observational reduction hides it. THE GAP: these
// are representative chains; the direct projection seam tests Σ-β, and QA's
// two independent mutations check the two head-walk families separately.
fn check_head_walk(name: &'static str, goal: &'static str, body: &'static str) {
    on_elaboration_stack(move || {
        let mut env = fixture_env();
        let trust = env.env.trusted_base();
        env.elaborate_decl(&theorem(name, goal, body))
            .unwrap_or_else(|error| panic!("{name} must check: {error:?}"));
        assert_eq!(env.env.trusted_base(), trust, "{name}: no new trust");
    });
}

#[test]
fn f1_closed_recursive_head_twice() {
    check_head_walk(
        "f1_closed",
        "R (R (Equal (Pair Nat Nat) p p) Zero) Zero",
        "Refl",
    );
}

#[test]
fn f1_neutral_recursive_head_twice() {
    check_head_walk("f1_neutral", "R (R (Equal (Pair Nat Nat) p p) z) z", "Refl");
}

#[test]
fn f1_neutral_recursive_head_twice_in_list_arm() {
    check_head_walk(
        "f1_neutral_list",
        "R (R (Equal (Pair Nat Nat) p p) z) z",
        LIST_ARMS,
    );
}

#[test]
fn f2_match_head_standalone() {
    check_head_walk("f2_pe", "PE True p", "Refl");
}

#[test]
fn f2_match_head_in_list_arm() {
    check_head_walk("f2_pe_list", "PE True p", LIST_ARMS);
}

#[test]
fn f2_match_head_in_bool_arm() {
    check_head_walk("f2_pe_arm", "PE b p", BOOL_ARMS);
}

#[test]
fn f2_let_head_standalone() {
    check_head_walk("f2_let", "LetEq p", "Refl");
}

#[test]
fn f2_let_head_in_list_arm() {
    check_head_walk("f2_let_list", "LetEq p", LIST_ARMS);
}

#[test]
fn unequal_pair_endpoints_reach_refl_conversion_and_refuse() {
    // MEASURED: both otherwise-valid routes refuse specifically for unequal
    // endpoints after exposing Eq, and do not add trust. CLAIMED: broadening
    // the head walk cannot admit a false Refl. THE GAP: the fixture uses
    // symbolic, nonconvertible p and q; convertible endpoints would check.
    on_elaboration_stack(|| {
        let mut env = fixture_env();
        let trust = env.env.trusted_base();
        for (name, goal) in [
            ("f1_false", "R (R (Equal (Pair Nat Nat) p q) z) z"),
            ("f2_false", "PEF True p q"),
        ] {
            let error = env
                .elaborate_decl(&theorem(name, goal, "Refl"))
                .expect_err("unequal Pair endpoints must refuse");
            assert!(
                matches!(error, ElabError::TypeMismatch { ref reason, .. }
                    if reason == "Refl: the two sides of the goal are not convertible"),
                "{name} must refuse for non-conversion: {error:?}"
            );
            assert_eq!(env.env.trusted_base(), trust, "{name}: no new trust");
        }
    });
}

#[test]
fn one_visit_wrappers_nat_match_and_nat_let_remain_valid() {
    // MEASURED: the five earlier single-visit and Nat-carrier routes still
    // check with unchanged trust. CLAIMED: only previously refused Pair
    // equality chains gain acceptance. THE GAP: the unchanged AC-2 suites
    // cover additional Refl goal shapes and the recursive P termination case.
    on_elaboration_stack(|| {
        let mut env = fixture_env();
        let trust = env.env.trusted_base();
        for (name, goal) in [
            ("f1_ctl_once", "R (Equal (Pair Nat Nat) p p) z"),
            ("f1_ctl_id", "Id (R (Id (Equal (Pair Nat Nat) p p)) z)"),
            ("f1_ctl_nat", "R (R (Equal Nat z z) z) z"),
            ("f2_ctl_pn", "PN True z"),
            ("f2_ctl_letn", "LetN z"),
        ] {
            env.elaborate_decl(&theorem(name, goal, "Refl"))
                .unwrap_or_else(|error| panic!("{name} control: {error:?}"));
            assert_eq!(env.env.trusted_base(), trust, "{name}: no new trust");
        }
    });
}
