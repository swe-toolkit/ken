//! Dependent-result recursion retains checked private workers and explicit trust.

use ken_elaborator::{ElabEnv, ObligationKind};
use ken_kernel::Decl;
use std::collections::BTreeSet;

const D2: &str = "fn g (n : AnyNat) : { y : Nat | Equal Nat y n } = \
    match n { Zero |-> Zero ; Suc m |-> g m }";
const D2S: &str = "fn g (n : AnyNat) : { y : Nat | Equal Nat y n } = \
    match n { Zero |-> Zero ; Suc m |-> Suc (g m) }";
const R2: &str = "fn g (n : AnyNat) : Nat ensures Equal Nat result n = \
    match n { Zero |-> Zero ; Suc m |-> Suc (g m) }";
const R4: &str = "fn g (n : AnyNat) (acc : Nat) : { y : Nat | Equal Nat y n } = \
    match n { Zero |-> Zero ; Suc m |-> g m (Suc acc) }";

fn check_row(source: &str) {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_decl("def AnyNat = { x : Nat | Top }")
        .expect("refined alias");
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let result = env
        .elaborate_decl_v1(source)
        .expect("descending dependent-result recursion accepts");
    assert_eq!(result.auxiliary_ids.len(), 1, "one checked private worker");
    let worker = result.auxiliary_ids[0];
    assert!(matches!(
        env.env.lookup(worker),
        Some(Decl::Transparent { .. })
    ));
    assert!(env.env.is_recursive_transparent(worker));
    assert!(!env.env.is_recursive_transparent(result.def_id));
    assert!(!env.globals.values().any(|id| *id == worker));
    assert_eq!(
        result.obligations.len(),
        3,
        "the source retains three goals"
    );
    assert!(result.obligations.iter().all(|goal| matches!(
        goal.kind,
        ObligationKind::RefinementIntroduction | ObligationKind::Ensures
    )));
    let open = result
        .obligations
        .iter()
        .filter(|goal| env.is_open_hole(goal.hole_id))
        .map(|goal| goal.hole_id)
        .collect::<BTreeSet<_>>();
    assert_eq!(
        open.len(),
        2,
        "one goal solved, two remain open in direct elaboration"
    );
    let added = env
        .env
        .trusted_base()
        .into_iter()
        .filter(|id| !before.contains(id))
        .collect::<BTreeSet<_>>();
    assert_eq!(
        added, open,
        "the exact trust delta is the open holes, not the worker"
    );
}

/// Promise class: durable invariant (spec 17 §4; spec 21 §5).
/// MEASURED: the direct elaborator checks each member/worker and records the
/// exact open-hole set in trusted_base(). CLAIMED: accepting dependent-result
/// descent adds no invisible assumptions. THE GAP: the CLI's three displayed
/// goal sites include a goal solved in the direct harness, pinned separately.
#[test]
fn dependent_result_direct_call() {
    check_row(D2);
}

#[test]
fn dependent_result_suc_call() {
    check_row(D2S);
}

#[test]
fn dependent_ensures_suc_call() {
    check_row(R2);
}

#[test]
fn dependent_result_with_accumulator() {
    check_row(R4);
}
