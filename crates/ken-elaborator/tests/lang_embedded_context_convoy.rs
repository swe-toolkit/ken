//! A context convoy and an embedded equation convoy share a coherent frame.
//!
//! Promise class: durable invariant. True two-list equalities check without
//! additional trust, while exchanging the two lookup components is refused at
//! the Refl leaf rather than at an enclosing match motive.

use ken_elaborator::{ElabEnv, ElabError};

// Match the adjacent embedded-method-convoy suite's stated elaboration stack.
const ELABORATION_STACK_BYTES: usize = 8 * 1024 * 1024;

fn on_elaboration_stack(f: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new()
        .stack_size(ELABORATION_STACK_BYTES)
        .spawn(f)
        .expect("spawn stated-stack elaboration")
        .join()
        .expect("stated-stack elaboration panicked");
}

fn env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("prelude");
    for declaration in [
        "data DualFin : Nat -> Type where { DualFZ : (n : Nat) -> DualFin (Suc n); DualFS : (n : Nat) -> DualFin n -> DualFin (Suc n) }",
        "data DualEnv (a : Type) : Nat -> Type where { DualNil : DualEnv a Zero; DualCons : (n : Nat) -> a -> DualEnv a n -> DualEnv a (Suc n) }",
        "fn dual_fin_to_nat (n : Nat) (i : DualFin n) : Nat = match i { DualFZ m ↦ Zero; DualFS m j ↦ Suc (dual_fin_to_nat m j) }",
        "fn dual_lookup (a : Type) (n : Nat) (xs : DualEnv a n) (i : DualFin n) : a = match i { DualFZ m ↦ match xs { DualCons k x rest ↦ x }; DualFS m j ↦ match xs { DualCons k x rest ↦ dual_lookup a m rest j } }",
        "fn dual_lookup_peer (a : Type) (n : Nat) (xs : DualEnv a n) (i : DualFin n) : a = match i { DualFZ m ↦ match xs { DualCons k x rest ↦ x }; DualFS m j ↦ match xs { DualCons k x rest ↦ dual_lookup_peer a m rest j } }",
        "data DualTagged (a : Type) : Type where { DualTag : Nat -> a -> DualTagged a }",
        "data DualPair (a : Type) : Type where { DP : a -> a -> DualPair a }",
    ] {
        env.elaborate_decl(declaration).expect("coherent-frame fixture");
    }
    env
}

fn r1(swap: bool) -> String {
    let left = "DP a (dual_lookup a (Suc m) xs (DualFZ m)) (dual_lookup a (Suc m) ys (DualFZ m))";
    let right = if swap {
        "DP a (dual_lookup a (Suc m) ys (DualFZ m)) (dual_lookup a (Suc m) xs (DualFZ m))"
    } else {
        left
    };
    format!("theorem r1 (a : Type) (m : Nat) (xs : DualEnv a (Suc m)) (ys : DualEnv a (Suc m)) : Equal (DualPair a) ({left}) ({right}) = match xs {{ DualCons k x rest ↦ match ys {{ DualCons k2 y rest2 ↦ Refl }} }}")
}

fn q1(swapped_order: bool, option: bool) -> String {
    let left = "DP a (dual_lookup a n xs i) (dual_lookup a n ys i)";
    let (ty, end) = if option {
        ("Option (DualPair a)", format!("Some (DualPair a) ({left})"))
    } else {
        ("DualPair a", left.into())
    };
    let body = if swapped_order {
        "match ys { DualCons k2 y rest2 ↦ match xs { DualCons k x rest ↦ Refl } }"
    } else {
        "match xs { DualCons k x rest ↦ match ys { DualCons k2 y rest2 ↦ Refl } }"
    };
    format!("theorem q1 (a : Type) (n : Nat) (xs : DualEnv a n) (ys : DualEnv a n) (i : DualFin n) : Equal ({ty}) ({end}) ({end}) = match i {{ DualFZ m ↦ {body}; DualFS m j ↦ {body} }}")
}

const T1: &str = "theorem t1 (a : Type) (n : Nat) (xs : DualEnv a n) (i : DualFin n) : Equal (DualTagged a) (DualTag a (dual_fin_to_nat n i) (dual_lookup a n xs i)) (DualTag a (dual_fin_to_nat n i) (dual_lookup a n xs i)) = match i { DualFZ m ↦ match xs { DualCons k x rest ↦ Refl }; DualFS m j ↦ match xs { DualCons k x rest ↦ Refl } }";

fn check_true(source: String) {
    let mut env = env();
    let trust = env.env.trusted_base();
    env.elaborate_decl(&source)
        .expect("true two-list equality must check");
    assert_eq!(env.env.trusted_base(), trust, "no added trust");
}

#[test]
fn r1_two_lists_at_suc_index() {
    on_elaboration_stack(|| check_true(r1(false)));
}

#[test]
fn q1_two_lists_in_both_fin_arms() {
    on_elaboration_stack(|| check_true(q1(false, false)));
}

#[test]
fn q1_swapped_inner_match_order() {
    on_elaboration_stack(|| check_true(q1(true, false)));
}

#[test]
fn q1_option_wrapped_pair_goal() {
    on_elaboration_stack(|| check_true(q1(false, true)));
}

#[test]
fn t1_one_list_control() {
    on_elaboration_stack(|| check_true(T1.into()));
}

#[test]
fn swapped_pair_components_fail_at_refl_leaf() {
    on_elaboration_stack(|| {
        let source = r1(true);
        let mut env = env();
        let trust = env.env.trusted_base();
        let error = env
            .elaborate_decl(&source)
            .expect_err("swapping distinct lookup components must reject");
        let ElabError::TypeMismatch { span, reason } = error else {
            panic!("false twin must fail at its Refl leaf: {error:?}");
        };
        assert_eq!(
            reason,
            "Refl: the two sides of the goal are not convertible"
        );
        let inner_arm = source
            .rfind("DualCons k2 y rest2 ↦ Refl")
            .expect("inner arm");
        let refl = source.rfind("Refl").expect("leaf Refl");
        assert!(
            span.start >= inner_arm,
            "failure must be inside the inner arm: {span:?}"
        );
        assert_eq!(span.end, refl + "Refl".len());
        assert_eq!(env.env.trusted_base(), trust, "no added trust");
    });
}
