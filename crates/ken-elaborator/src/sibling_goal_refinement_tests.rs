//! Constructed dependent contexts for the sibling-goal closure seam.

use super::*;
use crate::ElabEnv;

fn nat(id: GlobalId) -> Term {
    Term::IndFormer {
        id,
        level_args: vec![],
    }
}

fn eq(a: Term, x: Term, y: Term) -> Term {
    Term::Eq(Box::new(a), Box::new(x), Box::new(y))
}

fn app2(head: Term, a: Term, b: Term) -> Term {
    Term::app(Term::app(head, a), b)
}

fn dummy_arm(body: RExpr) -> RMatchArm {
    let span = Span { start: 0, end: 0 };
    RMatchArm {
        pat: RPattern {
            kind: RPatKind::Wild,
            span: span.clone(),
        },
        guard: None,
        body,
        span,
    }
}

#[test]
fn whole_pi_restores_existing_alias_once_at_original_goal() {
    // Promise class: durable invariant. This is the in-crate constructed
    // context authorized by the Architect: no natural source on ba2 has both
    // a goal-referenced existing alias and a leaf-dependent raw type.
    // MEASURED: a live h0 alias is the actual argument of one h1-keyed
    // whole-Pi restoration, and the emitted result checks at the raw goal.
    // CLAIMED: h0 is preserved, while the h1 transport occurs exactly once.
    // THE GAP: this pins the reachable private refinement seam, not every
    // alternative source syntax that could lead to the same context.
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_file(
        "data NestedVec (a : Type) : Nat → Type where { \
         Empty : NestedVec a Zero; \
         Step : (n : Nat) → a → NestedVec a n → NestedVec a (Suc n) }",
    )
    .expect("indexed family");
    let nat_ty = nat(env.globals["Nat"]);
    let vec_id = env.globals["NestedVec"];
    let vec_ty = |a: Term, i: Term| app2(nat(vec_id), a, i);
    let mut cx = ElabCtx::new(
        &mut env.env,
        &env.globals,
        &mut env.num_values,
        &env.numeric_env,
        "double-refinement-control",
    );
    // Context: n,m,k,l:Nat; h0:Eq Nat n m; d:Vec (Vec Nat n) k.
    for _ in 0..4 {
        cx.ctx.push(nat_ty.clone());
    }
    cx.ctx.push(eq(nat_ty.clone(), Term::var(3), Term::var(2)));
    let raw_ty = vec_ty(vec_ty(nat_ty.clone(), Term::var(4)), Term::var(2));
    cx.ctx.push(raw_ty);
    let old_ty = vec_ty(vec_ty(nat_ty.clone(), Term::var(5)), Term::var(3));
    let alias_ty = vec_ty(vec_ty(nat_ty.clone(), Term::var(4)), Term::var(3));
    let (h0_alias, classified_ty) = try_reindex_cast(
        None,
        cx.env,
        &cx.ctx,
        &nat_ty,
        &Term::var(5),
        &Term::var(4),
        &old_ty,
        Term::var(0),
        Term::var(1),
    )
    .expect("h0 alias classification")
    .expect("n occurs in d type");
    assert!(convert_type(cx.env, &cx.ctx, &classified_ty, &alias_ty));
    cx.var_refinements
        .insert(5, (h0_alias.clone(), alias_ty.clone(), cx.ctx.len()));
    let goal = eq(alias_ty, h0_alias.clone(), h0_alias.clone());
    cx.match_field_regions.push(0..cx.ctx.len());
    cx.active_index_premise_frames
        .push(ActiveIndexPremiseFrame {
            sentinel_region: 0,
            premise_domains: vec![eq(nat_ty.clone(), Term::var(2), Term::var(3))],
            install_depth: cx.ctx.len(),
        });
    let fake_family = InductiveDecl {
        id: GlobalId(u32::MAX - 5),
        level_params: vec![],
        params: vec![],
        parameter_polarities: vec![],
        indices: vec![nat_ty],
        level: Level::Zero,
        constructors: vec![],
        former_type: Term::ty(Level::Zero),
    };
    let (refined, restorations) = refine_branch_goal(
        &cx,
        &fake_family,
        &[],
        &[Term::var(2)],
        &[Term::var(3)],
        0,
        &goal,
    )
    .expect("classify one whole-Pi refinement");
    assert_eq!(restorations.len(), 1, "one h1-keyed restoration");
    let BranchGoalRestoration::Generalized { binders, .. } = &restorations[0] else {
        panic!("leaf restoration must generalize its dependent");
    };
    assert_eq!(binders.len(), 1, "only the goal-referenced dependent");
    assert_eq!(
        binders[0].source_value, h0_alias,
        "apply the h0 alias itself"
    );
    assert!(matches!(
        binders[0].source,
        GeneralizedGoalSource::Original(5)
    ));
    let arm = dummy_arm(RExpr::RCon(SUGAR_REFL.into(), Span { start: 0, end: 0 }));
    let body = check_generalized_branch_goal(&mut cx, &arm, refined, restorations, &goal, true)
        .expect("restore the checked body to its unrefined goal");
    kernel_check_current(&cx, &body, &goal).expect("emitted method checks at G(d,s)");
    assert_eq!(cx.ctx.len(), 6, "the temporary telescope was popped");
    assert!(cx.scoped_premise_aliases.is_empty());
}

#[test]
fn scoped_premise_redirects_consumed_proof_and_restores_on_failure() {
    // Promise class: durable invariant. MEASURED: a generated premise proof
    // is consumed at the generalized binder's different type, not merely
    // passed through as a term; an invalid body also restores its scope.
    // CLAIMED: premise redirection is scoped to body checking and survives
    // neither success nor failure. THE GAP: this constructs the private
    // proof-consumer seam; the source-facing f6 row remains separate.
    let mut env = ElabEnv::new().expect("prelude");
    let nat_ty = nat(env.globals["Nat"]);
    let mut cx = ElabCtx::new(
        &mut env.env,
        &env.globals,
        &mut env.num_values,
        &env.numeric_env,
        "premise-redirect-control",
    );
    cx.ctx.push(nat_ty.clone()); // n
    cx.ctx.push(nat_ty.clone()); // m
    let raw_type = eq(nat_ty.clone(), Term::var(1), Term::var(0));
    cx.active_index_premise_frames
        .push(ActiveIndexPremiseFrame {
            sentinel_region: 7,
            premise_domains: vec![raw_type],
            install_depth: 2,
        });
    let unrefined = eq(nat_ty.clone(), Term::var(1), Term::var(0));
    let refined = eq(nat_ty.clone(), Term::var(0), Term::var(0));
    assert!(!convert_type(cx.env, &cx.ctx, &unrefined, &refined));
    let sentinel = index_refinement_sentinel(7, 0);
    let proof_at_refined = Term::Ascript(Box::new(sentinel.clone()), Box::new(refined.clone()));
    assert!(kernel_infer_current(&cx, &proof_at_refined).is_err());
    cx.ctx.push(refined.clone());
    cx.hidden_positions.push(2);
    cx.scoped_premise_aliases.insert((7, 0), 2);
    let at_binder = eq(nat_ty.clone(), Term::var(1), Term::var(1));
    let proof = Term::Ascript(Box::new(weaken(&sentinel, 1)), Box::new(at_binder.clone()));
    kernel_check_current(&cx, &proof, &at_binder)
        .expect("generated premise proof consumed at the generalized type");
    cx.scoped_premise_aliases.clear();
    assert!(kernel_check_current(&cx, &proof, &at_binder).is_err());
    cx.ctx.pop();
    cx.hidden_positions.pop();
}

#[test]
fn scoped_premise_inference_pairs_redirected_term_with_its_binder_type() {
    // Promise class: durable invariant. The nested dependent-match path
    // infers its scrutinee before constructing a motive. Its inferred type
    // must come from the same temporary binder as its redirected proof term.
    // MEASURED: while scoped, a surface alias's type follows the new binder;
    // before and after it has the old premise type. CLAIMED: nested scrutinee
    // inference is one-view, not term-only redirection. THE GAP: the separate
    // exact f4 source additionally reaches the equation-convoy overlap guard.
    let mut env = ElabEnv::new().expect("prelude");
    let nat_ty = nat(env.globals["Nat"]);
    let mut cx = ElabCtx::new(
        &mut env.env,
        &env.globals,
        &mut env.num_values,
        &env.numeric_env,
        "nested-premise-inference-control",
    );
    cx.ctx.push(nat_ty.clone()); // n
    cx.ctx.push(nat_ty.clone()); // m
    cx.ctx.push(eq(nat_ty.clone(), Term::var(1), Term::var(0))); // d
    let old_ty = eq(nat_ty.clone(), Term::var(2), Term::var(1));
    let new_ty = eq(nat_ty.clone(), Term::var(1), Term::var(1));
    let premise = index_refinement_sentinel(4, 0);
    cx.active_index_premise_frames
        .push(ActiveIndexPremiseFrame {
            sentinel_region: 4,
            premise_domains: vec![old_ty.clone()],
            install_depth: cx.ctx.len(),
        });
    cx.var_refinements
        .insert(2, (premise.clone(), old_ty.clone(), cx.ctx.len()));
    let expr = RExpr::RVar(0, "d".into(), Span { start: 0, end: 0 });
    let (before, before_ty) = infer(&mut cx, &expr).expect("original premise inference");
    assert_eq!(before, premise);
    assert_eq!(before_ty, old_ty);
    cx.ctx.push(new_ty.clone());
    cx.hidden_positions.push(3);
    cx.scoped_premise_aliases.insert((4, 0), 3);
    let (inside, inside_ty) = infer(&mut cx, &expr).expect("generalized premise inference");
    let generalized_ty = weaken(&new_ty, 1);
    assert_eq!(
        inside_ty, generalized_ty,
        "term and inferred type use one binder"
    );
    assert!(!convert_type(
        cx.env,
        &cx.ctx,
        &weaken(&old_ty, 1),
        &inside_ty,
    ));
    kernel_check_current(&cx, &inside, &inside_ty).expect("redirected proof is well-typed");
    cx.scoped_premise_aliases.clear();
    cx.ctx.pop();
    cx.hidden_positions.pop();
    let (after, after_ty) = infer(&mut cx, &expr).expect("original premise restored");
    assert_eq!((after, after_ty), (premise, old_ty));
}

#[test]
fn generalized_premise_generated_proof_is_consumed_by_body() {
    // Promise class: durable invariant. A generated premise p is reached
    // through a refined variable's proof and consumed at its new binder type
    // while the whole function is transported along a separate leaf h1.
    // MEASURED: the production body checker and original-goal kernel check
    // both accept, and neither temporary scope survives. CLAIMED: generated
    // proof consumers see the Pi-bound premise rather than its old domain.
    // THE GAP: the context is constructed at the private seam, not parsed
    // from a surface declaration with the same two-leaf shape.
    let mut env = ElabEnv::new().expect("prelude");
    let nat_ty = nat(env.globals["Nat"]);
    let mut cx = ElabCtx::new(
        &mut env.env,
        &env.globals,
        &mut env.num_values,
        &env.numeric_env,
        "generated-proof-body-control",
    );
    cx.ctx.push(nat_ty.clone()); // n
    cx.ctx.push(nat_ty.clone()); // m
    cx.ctx.push(eq(nat_ty.clone(), Term::var(1), Term::var(1))); // d
    let p = index_refinement_sentinel(0, 0);
    let h1 = index_refinement_sentinel(0, 1);
    let old = eq(nat_ty.clone(), Term::var(2), Term::var(2));
    let new = eq(nat_ty.clone(), Term::var(1), Term::var(1));
    cx.active_index_premise_frames
        .push(ActiveIndexPremiseFrame {
            sentinel_region: 0,
            premise_domains: vec![old.clone(), eq(nat_ty.clone(), Term::var(1), Term::var(2))],
            install_depth: cx.ctx.len(),
        });
    cx.var_refinements
        .insert(2, (p.clone(), new.clone(), cx.ctx.len()));
    let source_type = Term::pi(new.clone(), weaken(&new, 1));
    let restoration = || BranchGoalRestoration::Generalized {
        whole: Box::new(BranchGoalRestoration::OmegaJ {
            index_type: nat_ty.clone(),
            old_index: Term::var(1),
            new_index: Term::var(2),
            source_type: source_type.clone(),
            omega_level: Level::Zero,
            equality: h1.clone(),
        }),
        binders: vec![GeneralizedGoalBinder {
            source: GeneralizedGoalSource::Premise { region: 0, slot: 0 },
            source_value: p.clone(),
        }],
    };
    let refined_goal = Term::pi(new.clone(), weaken(&new, 1));
    let arm = dummy_arm(RExpr::RVar(0, "d".into(), Span { start: 0, end: 0 }));
    let body =
        check_generalized_branch_goal(&mut cx, &arm, refined_goal, vec![restoration()], &old, true)
            .expect("generated proof consumed through body at generalized binder");
    kernel_check_current(&cx, &body, &old).expect("original method goal");
    assert_eq!(cx.ctx.len(), 3);
    assert!(cx.scoped_premise_aliases.is_empty());
    assert!(matches!(cx.var_refinements.get(&2), Some((term, _, _)) if *term == p));
    let rejected = dummy_arm(RExpr::RCon(
        "not_a_constructor".into(),
        Span { start: 0, end: 0 },
    ));
    check_generalized_branch_goal(
        &mut cx,
        &rejected,
        Term::pi(new.clone(), weaken(&new, 1)),
        vec![restoration()],
        &old,
        true,
    )
    .expect_err("a failed body must not leak its generalized scope");
    assert_eq!(cx.ctx.len(), 3, "error popped the generalized binder");
    assert!(
        cx.scoped_premise_aliases.is_empty(),
        "error restored sentinel ownership"
    );
    assert!(matches!(cx.var_refinements.get(&2), Some((term, _, _)) if *term == p));
}
