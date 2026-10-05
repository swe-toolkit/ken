//! Durable behavioral invariants for spec/20-verification/22 §2.2.
//! The predicate is opaque so no reduction can conceal whole-body emission.

use ken_elaborator::{ElabEnv, ObligationKind};
use ken_kernel::{Level, Term};

fn predicate_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("prelude");
    let int_ty = Term::const_(env.globals["Int"], vec![]);
    env.declare_postulate_raw("P", Term::pi(int_ty, Term::omega(Level::Zero)))
        .expect("opaque P : Int -> Omega");
    env
}

fn contains_elim(term: &Term) -> bool {
    matches!(term, Term::Elim { .. }) || term.children().iter().any(|child| contains_elim(child))
}

/// Durable invariant. MEASURED: each independently opaque clause emits a
/// path-local leaf goal. CLAIMED: no branchy return is checked only as a
/// whole-body eliminator. THE GAP: the two leaf values must be distinguished,
/// not merely counted; the exact goal and absence of Elim close that gap.
#[test]
fn result_postconditions_follow_match_and_if_leaves() {
    let mut env = predicate_env();
    let p = Term::const_(env.globals["P"], vec![]);
    let bool_ty = Term::indformer(env.globals["Bool"], vec![]);
    for (mode, source) in [
        (
            "ensures/match",
            "fn es_match (b : Bool) : Int ensures P result = match b { True |-> 5 ; False |-> 6 }",
        ),
        (
            "literal/match",
            "fn lit_match (b : Bool) : { x : Int | P x } = match b { True |-> 5 ; False |-> 6 }",
        ),
        (
            "ensures/if",
            "fn es_if (b : Bool) : Int ensures P result = if b then 5 else 6",
        ),
        (
            "literal/if",
            "fn lit_if (b : Bool) : { x : Int | P x } = if b then 5 else 6",
        ),
    ] {
        let result = env
            .elaborate_decl_v1(source)
            .unwrap_or_else(|error| panic!("{mode}: {error:?}"));
        assert_eq!(result.obligations.len(), 2, "{mode}: one per branch");
        for (obligation, (branch, value)) in
            result.obligations.iter().zip([("True", 5), ("False", 6)])
        {
            assert!(matches!(obligation.kind, ObligationKind::Ensures), "{mode}");
            let path = Term::Eq(
                Box::new(bool_ty.clone()),
                Box::new(Term::var(0)),
                Box::new(Term::constructor(env.globals[branch], vec![])),
            );
            assert_eq!(
                obligation.goal_closed,
                Term::pi(
                    bool_ty.clone(),
                    Term::pi(path, Term::app(p.clone(), Term::IntLit(value.into())),),
                ),
                "{mode}/{branch}: exactly this branch's goal"
            );
            assert!(
                !contains_elim(&obligation.goal_closed),
                "{mode}: no whole-body eliminator"
            );
            assert!(env.is_open_hole(obligation.hole_id), "opaque P stays open");
        }
    }
}

/// Durable invariant. Straight-line obligations are the degenerately single
/// result leaf: exact goal equality, not merely a count, guards the encoding.
#[test]
fn straight_line_postcondition_preserves_the_original_goal() {
    let mut env = predicate_env();
    let int = Term::const_(env.globals["Int"], vec![]);
    let p = Term::const_(env.globals["P"], vec![]);
    for source in [
        "fn es_line (n : Int) : Int ensures P result = n",
        "fn lit_line (n : Int) : { x : Int | P x } = n",
    ] {
        let result = env.elaborate_decl_v1(source).expect("straight-line return");
        let [obligation] = result.obligations.as_slice() else {
            panic!("one goal: {source}")
        };
        assert!(matches!(obligation.kind, ObligationKind::Ensures));
        assert_eq!(
            obligation.goal_closed,
            Term::pi(int.clone(), Term::app(p.clone(), Term::var(0))),
            "{source}"
        );
        assert!(env.is_open_hole(obligation.hole_id));
    }
}

/// Durable invariant. A recursive call's postcondition is assumed only in
/// that leaf's obligation: Zero remains open and Suc is discharged by its IH.
#[test]
fn recursive_postcondition_uses_direct_self_call_hypothesis() {
    let mut env = predicate_env();
    for source in [
        "fn es_rec (n : Nat) : Int ensures P result = match n { Zero |-> 5 ; Suc m |-> es_rec m }",
        "fn lit_rec (n : Nat) : { x : Int | P x } = match n { Zero |-> 5 ; Suc m |-> lit_rec m }",
    ] {
        let result = env
            .elaborate_decl_v1(source)
            .unwrap_or_else(|error| panic!("{source}: {error:?}"));
        assert_eq!(result.obligations.len(), 2, "one per constructor");
        let [zero, suc] = result.obligations.as_slice() else {
            unreachable!()
        };
        assert!(matches!(zero.kind, ObligationKind::Ensures));
        assert!(matches!(suc.kind, ObligationKind::Ensures));
        assert!(
            env.is_open_hole(zero.hole_id),
            "P 5 cannot discharge from P alone"
        );
        assert!(
            !env.is_open_hole(suc.hole_id),
            "Suc self-call postcondition has IH"
        );
        assert!(!contains_elim(&zero.goal_closed));
        assert!(!contains_elim(&suc.goal_closed));
        assert!(
            format!("{:?}", suc.goal_closed).contains("Eq"),
            "Suc path equation"
        );
    }
}

/// Durable invariant. Literal annotations introduce an obligation at each
/// value returned from their RHS, not at the value of the complete match.
#[test]
fn literal_let_and_ascription_push_into_branch_values() {
    let mut env = predicate_env();
    for source in [
        "const literal_let_match : Int = let y : { x : Int | P x } = match True { True |-> 5 ; False |-> 6 } in y",
        "const literal_ascribed_match : Int = (match True { True |-> 5 ; False |-> 6 } : { x : Int | P x })",
    ] {
        let result = env.elaborate_decl_v1(source)
            .unwrap_or_else(|error| panic!("{source}: {error:?}"));
        assert_eq!(result.obligations.len(), 2, "{source}");
        assert!(result.obligations.iter().all(|obligation|
            matches!(obligation.kind, ObligationKind::RefinementIntroduction)
                && !contains_elim(&obligation.goal_closed)));
    }
}
