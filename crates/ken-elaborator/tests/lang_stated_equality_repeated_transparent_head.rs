//! Repeated non-recursive transparent heads must not hide a stated equality.
//! Promise class: durable invariant. Spec 16 §2.1, 17 §1, 34 §3.5.

use ken_elaborator::{ElabEnv, ElabError};

// As in the adjacent Refl predicate suite: a measured 1,982,464-byte
// bare-prelude peak and 3 MiB nested-match minimum warrant 8 MiB here.
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
        "const MyEq : Pair Nat Nat -> Pair Nat Nat -> Prop = λp. λq. Eq (Pair Nat Nat) p q",
        "const Id : Prop -> Prop = λt. t",
        "const W2 : Pair Nat Nat -> Pair Nat Nat -> Prop = λp. λq. Id (Id (Eq (Pair Nat Nat) p q))",
        "const Both : Nat -> Nat -> Prop = λa. λb. And (Equal Nat a a) (Equal Nat b b)",
    ] {
        env.elaborate_decl(declaration)
            .unwrap_or_else(|error| panic!("fixture {declaration}: {error:?}"));
    }
    env
}

fn theorem(name: &str, goal: &str, in_list_arm: bool) -> String {
    let body = if in_list_arm {
        "match xs { Nil ↦ Refl; Cons h t ↦ Refl }"
    } else {
        "Refl"
    };
    format!("theorem {name} (p : Pair Nat Nat) (q : Pair Nat Nat) (z : Nat) (xs : List Nat) : {goal} = {body}")
}

fn check_repeated(name: &'static str, goal: &'static str, in_list_arm: bool) {
    // MEASURED: a theorem at this head chain and route kernel-checks without
    // new trust. CLAIMED: non-recursive transparent heads may recur.
    // THE GAP: one concrete head chain does not cover every possible alias.
    on_elaboration_stack(move || {
        let mut env = fixture_env();
        let trust = env.env.trusted_base();
        env.elaborate_decl(&theorem(name, goal, in_list_arm))
            .unwrap_or_else(|error| panic!("{name} must check: {error:?}"));
        assert_eq!(env.env.trusted_base(), trust, "{name}: no new trust");
    });
}

#[test]
fn nested_pair_equality_standalone() {
    check_repeated("nested_pair", "Id (Id (Equal (Pair Nat Nat) p p))", false);
}

#[test]
fn nested_pair_equality_list_arm() {
    check_repeated(
        "nested_pair_arm",
        "Id (Id (Equal (Pair Nat Nat) p p))",
        true,
    );
}

#[test]
fn wrapper_pair_equality_standalone() {
    check_repeated("wrapper_pair", "W2 p p", false);
}

#[test]
fn wrapper_pair_equality_list_arm() {
    check_repeated("wrapper_pair_arm", "W2 p p", true);
}

#[test]
fn nested_alias_equality_standalone() {
    check_repeated("nested_my_eq", "Id (Id (MyEq p p))", false);
}

#[test]
fn first_visit_and_nat_equality_controls_still_check() {
    // MEASURED: one-level alias and direct WHNF Eq routes still check.
    // CLAIMED: admitting repeated heads has not lost their earlier route.
    // THE GAP: route-specific behavior is covered only by these fixtures.
    on_elaboration_stack(|| {
        let mut env = fixture_env();
        let trust = env.env.trusted_base();
        for (name, goal) in [
            ("first_pair", "Id (Equal (Pair Nat Nat) p p)"),
            ("first_alias", "Id (MyEq p p)"),
            ("nat_direct", "Id (Id (Equal Nat z z))"),
        ] {
            env.elaborate_decl(&theorem(name, goal, false))
                .unwrap_or_else(|error| panic!("{name} control: {error:?}"));
            assert_eq!(env.env.trusted_base(), trust, "{name}: no new trust");
        }
    });
}

#[test]
fn unequal_endpoints_reach_conversion_on_both_routes() {
    // MEASURED: unequal symbolic endpoints refuse for non-conversion, not
    // merely because the wrapper no longer states Eq. CLAIMED: exposing the
    // equality does not grant false Refl proofs. THE GAP: symbolic Pair
    // variables differ; a deliberately convertible endpoint pair would pass.
    on_elaboration_stack(|| {
        let mut env = fixture_env();
        let trust = env.env.trusted_base();
        for (name, goal, in_list_arm) in [
            ("false_nested", "Id (Id (Equal (Pair Nat Nat) p q))", false),
            ("false_wrapper_arm", "W2 p q", true),
        ] {
            let error = env
                .elaborate_decl(&theorem(name, goal, in_list_arm))
                .expect_err("unequal symbolic Pair endpoints must refuse");
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
fn conjunction_of_equalities_still_refuses_on_both_routes() {
    // MEASURED: the Sigma-shaped And cannot masquerade as a stated Eq.
    // CLAIMED: broadening transparent unfolding is not broadening Refl.
    // THE GAP: the fixture's two conjuncts are true, so rejection is shape.
    on_elaboration_stack(|| {
        let mut env = fixture_env();
        let trust = env.env.trusted_base();
        for (name, in_list_arm) in [("both_standalone", false), ("both_arm", true)] {
            let error = env
                .elaborate_decl(&theorem(name, "Both z z", in_list_arm))
                .expect_err("And states no equality");
            assert!(
                matches!(error, ElabError::TypeMismatch { ref reason, .. }
                    if reason == "Refl expects an `Eq`-shaped goal"),
                "{name} must refuse at the shape gate: {error:?}"
            );
            assert_eq!(env.env.trusted_base(), trust, "{name}: no new trust");
        }
    });
}
