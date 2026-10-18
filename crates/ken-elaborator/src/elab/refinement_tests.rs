//! Mechanism controls for subset-Σ evidence selection. Promise class:
//! durable invariants of actual certificate demand and typed refusal.

use super::*;
use crate::ElabEnv;

#[test]
fn nested_bool_demand_reads_the_selected_evidence_not_the_hole_telescope() {
    let mut env = ElabEnv::new().expect("checked prelude");
    let mut cx = ElabCtx::new(
        &mut env.env, &env.globals, &mut env.num_values,
        &env.numeric_env, &env.refinement_facts, "nested Bool demand",
    );
    let bool_ty = Term::indformer(cx.numeric_env.bool_id, vec![]);
    let yes = Term::constructor(cx.numeric_env.bool_true_id, vec![]);
    let no = Term::constructor(cx.numeric_env.bool_false_id, vec![]);
    let outer = Term::Eq(
        Box::new(bool_ty.clone()), Box::new(yes.clone()), Box::new(yes.clone()),
    );
    let inner = Term::Eq(
        Box::new(bool_ty), Box::new(yes), Box::new(no),
    );
    cx.ctx.push(outer.clone());
    cx.ctx.push(inner.clone());
    let frames = vec![
        PathFrame {
            condition: weaken(&outer, 2), install_depth: 2, split_id: 7,
            kind: SplitKind::If, witness_position: Some(0),
        },
        PathFrame {
            condition: weaken(&inner, 1), install_depth: 2, split_id: 8,
            kind: SplitKind::If, witness_position: Some(1),
        },
    ];
    let used_outer = Term::var(1);
    kernel_check_raw(cx.env, &cx.ctx, &used_outer, &weaken(&outer, 2))
        .expect("outer condition really is usable evidence");
    let closed = Term::const_(cx.env.tt_id(), vec![]);
    kernel_check_raw(cx.env, &cx.ctx, &closed, &Term::const_(cx.env.top_id(), vec![]))
        .expect("closed certificate really checks");

    assert_eq!(demanded_refinement_splits(&frames, Some(&used_outer), cx.ctx.len()), [7]);
    assert!(demanded_refinement_splits(&frames, Some(&closed), cx.ctx.len()).is_empty());
    assert_eq!(demanded_refinement_splits(&frames, None, cx.ctx.len()), [7, 8]);
}

#[test]
fn open_subset_proof_under_recursive_nat_split_refuses_without_minting_a_hole() {
    let mut env = ElabEnv::new().expect("checked prelude");
    let mut cx = ElabCtx::new(
        &mut env.env, &env.globals, &mut env.num_values,
        &env.numeric_env, &env.refinement_facts, "recursive split refusal",
    ).with_preconditions(&env.preconditions, PremiseHoles::Reported);
    let nat = Term::indformer(*cx.globals.get("Nat").expect("prelude Nat"), vec![]);
    let zero = Term::constructor(*cx.globals.get("Zero").expect("prelude Zero"), vec![]);
    cx.ctx.push(nat.clone());
    let split_id = cx.mint_split();
    cx.path_conditions.push(PathFrame {
        condition: Term::Eq(
            Box::new(nat), Box::new(Term::var(0)), Box::new(zero),
        ),
        install_depth: cx.ctx.len(), split_id,
        kind: SplitKind::Constructor, witness_position: None,
    });
    let trust = cx.env.trusted_base();
    let goal = Term::const_(cx.env.bottom_id(), vec![]);
    let error = emit_subset_evidence(
        &mut cx, goal, &Span::zero(), ObligationKind::RefinementIntroduction,
        None,
    ).expect_err("recursive split has no convoy evidence");
    assert!(matches!(error, ElabError::NonConvoyableRefinementSplit { .. }), "{error:?}");
    assert!(cx.obligations.is_empty(), "refusal must precede hole admission");
    assert_eq!(cx.env.trusted_base(), trust, "refusal added trust");
}
