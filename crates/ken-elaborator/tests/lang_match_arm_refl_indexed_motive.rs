//! Indexed-family match arms must keep a stated equality before the Sigma
//! carrier is simplified. Promise class: durable invariant; spec 34 §3.2,
//! §3.5, and 16 §2.1. Kernel re-checks every emitted proof.

use ken_elaborator::{ElabEnv, ElabError};

// Same declared baseline as the landed Pair-goal acceptance suite: the
// prelude peaked at 1,982,464 bytes; dependent matches needed 3 MiB;
// 8 MiB leaves more than 5 MiB margin. This provisions a new test, not a
// stack change on a newly failing existing test.
const ELABORATION_STACK_BYTES: usize = 8 * 1024 * 1024;

fn on_elaboration_stack<T: Send + 'static>(f: impl FnOnce() -> T + Send + 'static) -> T {
    std::thread::Builder::new()
        .stack_size(ELABORATION_STACK_BYTES)
        .spawn(f)
        .expect("spawn stated-stack elaboration")
        .join()
        .expect("stated-stack elaboration panicked")
}

fn fixture_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("prelude");
    for decl in [
        "data DualFin : Nat -> Type where { DualFZ : (n : Nat) -> DualFin (Suc n); DualFS : (n : Nat) -> DualFin n -> DualFin (Suc n) }",
        "const Two : Type = Pair Nat Nat",
        "data Box = MkBox Nat",
    ] {
        env.elaborate_decl(decl).expect("indexed-fixture declaration");
    }
    env
}

fn checked(source: &str) {
    let source = source.to_owned();
    on_elaboration_stack(move || {
        let mut env = fixture_env();
        let trust_before = env.env.trusted_base();
        env.elaborate_decl(&source)
            .unwrap_or_else(|error| panic!("kernel-checking {source}: {error:?}"));
        assert_eq!(env.env.trusted_base(), trust_before);
    });
}

fn check_carrier(name: &str, carrier: &str) {
    // MEASURED: a standalone Refl and both indexed methods elaborate and
    // kernel-check at one symbolic, shared carrier with no new trusted entry.
    // CLAIMED: indexed motive beta exposes the same stated Eq as standalone.
    // THE GAP: the indexed source must reach the real branch simplifier;
    // standalone success alone cannot establish the indexed route.
    checked(&format!(
        "theorem standalone_{name} (p : {carrier}) : Equal ({carrier}) p p = Refl"
    ));
    checked(&format!(
        "theorem indexed_{name} (n : Nat) (i : DualFin n) (p : {carrier}) : Equal ({carrier}) p p = match i {{ DualFZ m ↦ Refl; DualFS m j ↦ Refl }}"
    ));
}

#[test]
fn indexed_pair_nat_nat_refl_checks() {
    check_carrier("pair", "Pair Nat Nat");
}

#[test]
fn indexed_alias_to_pair_refl_checks() {
    check_carrier("two", "Two");
}

#[test]
fn indexed_nested_pair_refl_checks() {
    check_carrier("nested_pair", "Pair (Pair Nat Nat) Nat");
}

#[test]
fn indexed_pair_with_box_component_refl_checks() {
    check_carrier("box_pair", "Pair Box Nat");
}

#[test]
fn indexed_kernel_eq_pair_refl_checks() {
    // MEASURED: the indexed motive applied to an explicit Eq reaches Refl.
    // CLAIMED: recognizing the motive redex is not tied to Equal's alias.
    // THE GAP: this uses the same Sigma carrier and indexed constructor
    // branches as the alias rows, with only the equality spelling changed.
    checked(
        "theorem indexed_eq (n : Nat) (i : DualFin n) (p : Pair Nat Nat) : Eq (Pair Nat Nat) p p = match i { DualFZ m ↦ Refl; DualFS m j ↦ Refl }",
    );
}

#[test]
fn indexed_swapped_pair_rejects_nonconvertible_sides() {
    // MEASURED: the same indexed Pair route rejects a flipped, symbolic
    // pair at Refl's not-convertible gate, not the Eq-shape guard.
    // CLAIMED: retaining equality shape cannot manufacture equality.
    // THE GAP: the error must be the specific conversion rejection;
    // generic rejection would still pass under the original defect.
    on_elaboration_stack(|| {
        let mut env = fixture_env();
        let trust_before = env.env.trusted_base();
        let result = env.elaborate_decl(
            "theorem indexed_false (n : Nat) (i : DualFin n) (a : Nat) (b : Nat) : Equal (Pair Nat Nat) (mk_pair Nat Nat a b) (mk_pair Nat Nat b a) = match i { DualFZ m ↦ Refl; DualFS m j ↦ Refl }",
        );
        assert_eq!(env.env.trusted_base(), trust_before);
        assert!(
            matches!(result, Err(ElabError::TypeMismatch { ref reason, .. })
                if reason == "Refl: the two sides of the goal are not convertible"),
            "swapped components must reject for non-convertibility: {result:?}"
        );
    });
}

#[test]
fn nonindexed_list_bool_and_let_pair_goals_still_check() {
    // MEASURED: each ordinary route kernel-checks the same symbolic Pair
    // equality without trust growth. CLAIMED: the indexed-motive repair
    // leaves standalone and non-indexed match/let behavior intact.
    // THE GAP: those controls are separate source routes, not aliases for
    // the indexed match; the landed Pair-goal suite covers deeper nesting.
    for source in [
        "theorem list_pair (p : Pair Nat Nat) (xs : List Nat) : Equal (Pair Nat Nat) p p = match xs { Nil ↦ Refl; Cons x rest ↦ Refl }",
        "theorem bool_pair (p : Pair Nat Nat) (b : Bool) : Equal (Pair Nat Nat) p p = match b { True ↦ Refl; False ↦ Refl }",
        "theorem let_pair (p : Pair Nat Nat) (n : Nat) : Equal (Pair Nat Nat) p p = let x = n in Refl",
    ] {
        checked(source);
    }
}
