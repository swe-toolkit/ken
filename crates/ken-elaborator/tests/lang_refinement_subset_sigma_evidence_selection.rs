//! First-found certificate order controls both the emitted proof and demand.

use ken_elaborator::ElabEnv;
use ken_kernel::{check, Context, Decl, Term};

fn collect_pair_proofs<'a>(term: &'a Term, proofs: &mut Vec<&'a Term>) {
    if let Term::Pair(_, proof) = term {
        proofs.push(proof);
    }
    for child in term.children() {
        collect_pair_proofs(child, proofs);
    }
}

// Promise class: durable invariant. MEASURED: tt occurs before a matching
// local Eq witness in the candidate chain and is the actual emitted proof.
// CLAIMED: certificate selection keeps its first-found order, including when
// both proofs would check, so split demand cannot silently move to a binder.
// THE GAP: the paired Var-first mutation must pick h instead and fail here.
#[test]
fn closed_equality_prefers_tt_over_matching_local_proof() {
    let mut env = ElabEnv::new().expect("checked prelude");
    let result = env
        .elaborate_decl_v1(
            "fn f (h : Equal Nat Zero Zero) : { x : Nat | Equal Nat Zero Zero } = Zero",
        )
        .expect("closed equality admits a checked subset pair");
    assert_eq!(result.obligations.len(), 1);
    assert!(!env.is_open_hole(result.obligations[0].hole_id));
    let Some(Decl::Transparent { body, ty, .. }) = env.env.lookup(result.def_id) else {
        panic!("checked subset function")
    };
    let mut proofs = Vec::new();
    collect_pair_proofs(body, &mut proofs);
    assert_eq!(proofs.len(), 1, "exactly one subset introduction: {body:?}");
    assert_eq!(
        proofs[0],
        &Term::const_(env.env.tt_id(), vec![]),
        "tt must win before the local matching Eq certificate"
    );
    check(&env.env, &Context::new(), body, ty).expect("kernel rechecks the emitted proof");
}
