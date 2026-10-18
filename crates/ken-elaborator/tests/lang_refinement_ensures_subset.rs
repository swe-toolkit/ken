//! Contract results are checked subset Σ values, not detached obligations.

use ken_elaborator::{ElabEnv, ElabError, ObligationKind};
use ken_kernel::{Decl, Term};

/// Promise class: durable result-representation invariant.
/// MEASURED: the admitted function's kernel type has a subset-Σ result and
/// its checked body constructs a Pair, with an open hole visible when false.
/// CLAIMED: a written `ensures` is intrinsic to the checked result type.
/// THE GAP: the inspected pair must inhabit the *declared* type, not an
/// unrelated synthetic term; `Decl::Transparent` supplies that identity.
#[test]
fn ensures_is_a_checked_result_pair_even_when_its_proof_is_open() {
    for (value, open) in [(5, false), (6, true)] {
        let mut env = ElabEnv::new().expect("prelude");
        let source = format!("fn result_subset (n : Int) : Int ensures Equal Int result 5 = {value}");
        let result = env.elaborate_decl_v1(&source).expect("typed result pair");
        let decl = env.env.lookup(result.def_id).expect("admitted definition");
        let Decl::Transparent { ty: Term::Pi(_, cod), body: Term::Lam(_, body), .. } = decl else {
            panic!("function must be admitted at the checked Pi signature: {decl:?}");
        };
        assert!(matches!(cod.as_ref(), Term::Sigma(..)), "ensures is in the kernel result type");
        assert!(matches!(body.as_ref(), Term::Pair(..)), "the function constructs the pair");
        let ensures = result.obligations.iter().filter(|o| matches!(o.kind, ObligationKind::Ensures))
            .collect::<Vec<_>>();
        assert_eq!(ensures.len(), 1, "one clause, one site-local obligation");
        assert_eq!(env.is_open_hole(ensures[0].hole_id), open,
            "proof status must follow the actual returned value");
    }
}

/// Promise class: durable multiple-clause proposition-conjunction invariant.
/// MEASURED: two independent obligations inhabit one subset Σ whose second
/// component is an Ω-valued Sigma; its checked proof is a pair of proofs.
/// CLAIMED: multiple ensures share the result carrier, never nest it twice.
/// THE GAP: the second Sigma must occur *inside the proposition* of the
/// outer result Sigma, not in the function parameter telescope.
#[test]
fn multiple_ensures_share_one_carrier_and_keep_independent_goals() {
    let mut env = ElabEnv::new().expect("prelude");
    let result = env.elaborate_decl_v1(
        "fn both (n : Int) : Int ensures Equal Int result 5 ensures Equal Int result 6 = 5",
    ).expect("checked multi-postcondition");
    let Decl::Transparent { ty: Term::Pi(_, cod), body: Term::Lam(_, body), .. } =
        env.env.lookup(result.def_id).expect("admitted function") else { panic!("checked Pi") };
    assert!(matches!(cod.as_ref(), Term::Sigma(_, both) if matches!(both.as_ref(), Term::Sigma(..))),
        "both clauses must be one result subset with proof conjunction");
    assert!(matches!(body.as_ref(), Term::Pair(_, proof) if matches!(proof.as_ref(), Term::Pair(..))),
        "both proofs must be carried in the checked result pair");
    let goals = result.obligations.iter().filter(|o| matches!(o.kind, ObligationKind::Ensures))
        .collect::<Vec<_>>();
    assert_eq!(goals.len(), 2);
    assert!(!env.is_open_hole(goals[0].hole_id));
    assert!(env.is_open_hole(goals[1].hole_id));
}

/// Promise class: durable composition invariant for two independently written
/// return constraints. MEASURED: the explicit result refinement and written
/// ensures both survive in the checked nested subset type and returned pairs.
/// CLAIMED: neither source clause is erased while building the contract.
/// THE GAP: both clauses are provable on `5`, so requiring distinct obligation
/// kinds and checked nested pairs prevents a one-clause success from passing.
#[test]
fn explicit_refined_return_and_ensures_keep_both_checked_proofs() {
    let mut env = ElabEnv::new().expect("prelude");
    let result = env.elaborate_decl_v1(
        "fn combined (n : Int) : { x : Int | Equal Int x 5 } ensures Equal Int result 5 = 5",
    ).expect("both return clauses");
    let Decl::Transparent { ty: Term::Pi(_, cod), body: Term::Lam(_, body), .. } =
        env.env.lookup(result.def_id).expect("admitted function") else { panic!("checked Pi") };
    assert!(matches!(cod.as_ref(), Term::Sigma(base, _) if matches!(base.as_ref(), Term::Sigma(..))),
        "both source constraints must survive in the kernel type");
    assert!(matches!(body.as_ref(), Term::Pair(value, _) if matches!(value.as_ref(), Term::Pair(..))),
        "both source constraints have proof-bearing introduction pairs");
    assert_eq!(result.obligations.len(), 2);
    assert!(result.obligations.iter().any(|o| matches!(o.kind, ObligationKind::Ensures)));
    assert!(result.obligations.iter().any(|o| matches!(o.kind, ObligationKind::RefinementIntroduction)));
    assert!(result.obligations.iter().all(|o| !env.is_open_hole(o.hole_id)));
}

/// Promise class: durable recursive-evidence invariant.
/// MEASURED: a structurally recursive function with a closed Zero proof
/// admits both checked per-path result pairs; the Suc path uses the returned
/// subset proof, not a detached proposition in an obligation-only context.
/// CLAIMED: self calls are typed at the full checked result Σ.
/// THE GAP: making the Zero clause false must force a typed refusal under
/// the non-convoyable recursive split, rather than silently admitting it.
#[test]
fn recursive_ensures_reuses_the_checked_self_call_proof() {
    let mut env = ElabEnv::new().expect("prelude");
    let result = env.elaborate_decl_v1(
        "fn rec_subset (n : Nat) : Int ensures Equal Int result 5 = match n { Zero |-> 5 ; Suc m |-> rec_subset m }",
    ).expect("recursive body returns a checked subset on both paths");
    let goals = result.obligations.iter().filter(|o| matches!(o.kind, ObligationKind::Ensures))
        .collect::<Vec<_>>();
    assert_eq!(goals.len(), 2);
    assert!(goals.iter().all(|o| !env.is_open_hole(o.hole_id)));
    let Decl::Transparent { ty: Term::Pi(_, cod), .. } =
        env.env.lookup(result.def_id).expect("recursive checked body") else { panic!("checked Pi") };
    assert!(matches!(cod.as_ref(), Term::Sigma(..)));
}

/// Promise class: durable refusal control for non-convoyable recursion.
/// MEASURED: swapping the Zero result from 5 to 6 leaves an open proof
/// beneath the recursive Nat split and returns the typed split refusal.
/// CLAIMED: no recursive path condition is silently closed into a pair hole
/// when the split has no checked convoy equation.
/// THE GAP: the adjacent 5/5 case succeeds with the identical family,
/// signature and recursive arm, isolating the unsolved base proof.
#[test]
fn recursive_open_ensures_without_a_convoy_refuses() {
    let mut env = ElabEnv::new().expect("prelude");
    let start_id = env.env.next_global_id();
    let error = env.elaborate_decl_v1(
        "fn rec_subset (n : Nat) : Int ensures Equal Int result 5 = match n { Zero |-> 6 ; Suc m |-> rec_subset m }",
    ).expect_err("open base case cannot acquire unsupported recursive evidence");
    assert!(matches!(error, ElabError::NonConvoyableRefinementSplit { .. }), "{error:?}");
    assert_eq!(env.env.next_global_id(), start_id, "failed clause rolls back the staged self");
    assert!(!env.globals.contains_key("rec_subset"));
}
