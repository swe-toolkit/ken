//! Durable behavioral invariants for spec/20-verification/22 §2.2.
//! The predicate is opaque so no reduction can conceal whole-body emission.

use ken_elaborator::{ElabEnv, ElabError, ObligationKind};
use ken_kernel::{check, Context, Decl, GlobalId, Level, Term};

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

fn has_nat_equation(term: &Term, nat: GlobalId) -> bool {
    if let Term::Eq(carrier, _, _) = term {
        if matches!(carrier.as_ref(), Term::IndFormer { id, .. } if *id == nat) {
            return true;
        }
    }
    term.children().iter().any(|child| has_nat_equation(child, nat))
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
            let expected = if mode.starts_with("ensures") {
                ObligationKind::Ensures
            } else {
                ObligationKind::RefinementIntroduction
            };
            assert_eq!(std::mem::discriminant(&obligation.kind),
                std::mem::discriminant(&expected), "{mode}");
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
        let expected = if source.starts_with("fn es_line") {
            ObligationKind::Ensures
        } else {
            ObligationKind::RefinementIntroduction
        };
        assert_eq!(std::mem::discriminant(&obligation.kind),
            std::mem::discriminant(&expected));
        assert_eq!(
            obligation.goal_closed,
            Term::pi(int.clone(), Term::app(p.clone(), Term::var(0))),
            "{source}"
        );
        assert!(env.is_open_hole(obligation.hole_id));
    }
}

/// Durable invariant. A condition or argument that branches is not itself
/// the declaration's result position. The surrounding application is one
/// leaf, so its postcondition is neither lost nor duplicated in the argument.
#[test]
fn argument_branch_does_not_inherit_the_result_predicate() {
    let mut env = predicate_env();
    env.elaborate_decl("fn identity (n : Int) : Int = n")
        .expect("ordinary identity");
    let result = env
        .elaborate_decl_v1(
            "fn with_argument (b : Bool) : Int ensures P result = identity (if b then 5 else 6)",
        )
        .expect("branch in argument, not result position");
    let [obligation] = result.obligations.as_slice() else {
        panic!("one result leaf")
    };
    assert!(matches!(obligation.kind, ObligationKind::Ensures));
    assert!(
        contains_elim(&obligation.goal_closed),
        "argument still contains the conditional"
    );
}

/// Durable unknown boundary (22 §3). An opaque base goal under the
/// recursive Nat split remains open without an unproved branch equation.
/// The closed-base counterpart is pinned in
/// lang_refinement_ensures_subset::recursive_ensures_reuses_the_checked_self_call_proof.
#[test]
fn recursive_open_postcondition_omits_the_unsupported_nat_equation() {
    let mut env = predicate_env();
    for (source, name, kind, count) in [
        ("fn es_rec (n : Nat) : Int ensures P result = match n { Zero |-> 5 ; Suc m |-> es_rec m }", "es_rec", ObligationKind::Ensures, 2),
        ("fn lit_rec (n : Nat) : { x : Int | P x } = match n { Zero |-> 5 ; Suc m |-> lit_rec m }", "lit_rec", ObligationKind::RefinementIntroduction, 1),
    ] {
        let result = env.elaborate_decl_v1(source).expect("no invented Nat equation");
        assert_eq!(result.obligations.len(), count,
            "direct subset self-call reuses its proof; ensures records each leaf: {source}");
        assert!(result.obligations.iter().all(|goal|
            std::mem::discriminant(&goal.kind) == std::mem::discriminant(&kind)
            && !has_nat_equation(&goal.goal_closed, env.globals["Nat"])));
        assert!(env.is_open_hole(result.obligations[0].hole_id),
            "opaque base goal remains unknown: {source}");
        assert_eq!(env.globals[name], result.def_id, "admitted recursive result");
        let Some(Decl::Transparent { ty, body, .. }) = env.env.lookup(result.def_id) else {
            panic!("recursive result is checked and transparent: {source}")
        };
        check(&env.env, &Context::new(), body, ty).expect("kernel checks per-leaf subset pairs");
    }
}

/// Durable parameter-substitution invariant. Both recursive result pairs are
/// checked: the base uses the matching explicit premise and the step uses the
/// self-call's returned Proj2 at the non-final `tag`. A different tag cannot
/// borrow this proof via the opaque P2 family.
#[test]
fn recursive_checked_result_uses_nonfinal_parameter_and_requirement() {
    let mut env = predicate_env();
    let int = Term::const_(env.globals["Int"], vec![]);
    let omega = Term::omega(Level::Zero);
    env.declare_postulate_raw(
        "P2", Term::pi(int.clone(), Term::pi(int.clone(), omega.clone())),
    ).expect("P2 : Int -> Int -> Omega");
    env.declare_postulate_raw("Q", Term::pi(int.clone(), omega))
        .expect("Q : Int -> Omega");
    let result = env.elaborate_decl_v1(
        "fn rec_dep (tag : Int) (n : Nat) : Int requires Q tag requires P2 tag 5 ensures P2 tag result = match n { Zero |-> 5 ; Suc m |-> rec_dep tag m }",
    ).expect("parameter-dependent checked recursive result");
    let ensures = result.obligations.iter().filter(|o|
        matches!(o.kind, ObligationKind::Ensures)).collect::<Vec<_>>();
    assert_eq!(ensures.len(), 2, "one contract result site per method");
    assert!(result.obligations.iter().all(|o| !env.is_open_hole(o.hole_id)),
        "the recursive call and both written requirements carry checked proofs");
    let Decl::Transparent { ty: Term::Pi(_, tail), .. } =
        env.env.lookup(result.def_id).expect("admitted recursive function") else {
            panic!("checked parameterized function")
        };
    let Term::Pi(_, tail) = tail.as_ref() else { panic!("n binder") };
    let Term::Pi(q_req, tail) = tail.as_ref() else { panic!("Q requirement") };
    assert_eq!(q_req.as_ref(),
        &Term::app(Term::const_(env.globals["Q"], vec![]), Term::var(1)));
    let Term::Pi(p2_req, tail) = tail.as_ref() else { panic!("P2 requirement") };
    assert!(matches!(p2_req.as_ref(), Term::App(_, value)
        if matches!(value.as_ref(), Term::IntLit(n) if *n == 5.into())));
    assert!(matches!(tail.as_ref(), Term::Sigma(..)),
        "the postcondition is carried by the kernel result, not a detached hypothesis");
}

/// Durable nested-recursion boundary. Both recursive Nat splits lack a
/// checked branch equation, so each open result leaf is admitted as unknown
/// over evidenced Γ only. Nonrecursive checked-equation methods remain pinned
/// in lang_refinement_ensures_subset::nested_constructor_result_pairs_replay_checked_split_equations.
#[test]
fn nested_recursive_matrix_keeps_open_leaves_without_an_unsupported_equation() {
    let mut env = predicate_env();
    for (source, name, kind, count) in [
        ("fn nested_rec (n : Nat) : Int ensures P result = match n { Zero |-> 5 ; Suc Zero |-> 6 ; Suc (Suc m) |-> nested_rec m }", "nested_rec", ObligationKind::Ensures, 3),
        ("fn lit_nested_rec (n : Nat) : { x : Int | P x } = match n { Zero |-> 5 ; Suc Zero |-> 6 ; Suc (Suc m) |-> lit_nested_rec m }", "lit_nested_rec", ObligationKind::RefinementIntroduction, 2),
    ] {
        let result = env.elaborate_decl_v1(source).expect("recursive equations remain unsupported premises");
        assert_eq!(result.obligations.len(), count,
            "direct subset self-call reuses its proof; ensures records each leaf: {source}");
        assert!(result.obligations.iter().all(|goal|
            std::mem::discriminant(&goal.kind) == std::mem::discriminant(&kind)
            && !has_nat_equation(&goal.goal_closed, env.globals["Nat"])));
        assert!(env.is_open_hole(result.obligations[0].hole_id),
            "the opaque Zero leaf remains unknown: {source}");
        assert_eq!(env.globals[name], result.def_id, "admitted recursive result");
        let Some(Decl::Transparent { ty, body, .. }) = env.env.lookup(result.def_id) else {
            panic!("nested recursive result is checked and transparent: {source}")
        };
        check(&env.env, &Context::new(), body, ty).expect("kernel checks nested result pairs");
    }
}

/// Durable invariant. Literal-pattern branches use boolean comparison path
/// equations, including the false residual; no obligation is placed on the
/// entire match or silently discarded in its specialized matrix producer.
#[test]
fn literal_pattern_match_carries_the_comparator_path() {
    let mut env = predicate_env();
    let result = env
        .elaborate_decl_v1(
            "fn literal_case (n : Int) : Int ensures P result = match n { 5 |-> 5 ; 6 |-> 6 ; _ |-> 7 }",
        )
        .expect("two literal arms and catchall match");
    assert_eq!(result.obligations.len(), 3);
    let mut facts = Vec::new();
    for (obligation, expected_value) in result.obligations.iter().zip([5, 6, 7]) {
        let mut current = &obligation.goal_closed;
        let mut branch_facts = Vec::new();
        while let Term::Pi(domain, rest) = current {
            if let Term::Eq(ty, condition, right) = domain.as_ref() {
                if matches!(ty.as_ref(), Term::IndFormer { id, .. } if *id == env.globals["Bool"]) {
                    branch_facts.push((condition.as_ref().clone(), right.as_ref().clone()));
                }
            }
            current = rest;
        }
        let Term::App(_, value) = current else {
            panic!("opaque P at leaf: {current:?}")
        };
        assert_eq!(value.as_ref(), &Term::IntLit(expected_value.into()));
        assert!(!contains_elim(&obligation.goal_closed));
        facts.push(branch_facts);
    }
    let truth = Term::constructor(env.globals["True"], vec![]);
    let falsehood = Term::constructor(env.globals["False"], vec![]);
    let [first, second, fallback] = facts.as_slice() else {
        unreachable!()
    };
    assert_eq!(first.len(), 1);
    assert_eq!(second.len(), 2);
    assert_eq!(fallback.len(), 2);
    assert_eq!(first[0].1, truth);
    assert_eq!(second[0].1, falsehood);
    assert_eq!(second[1].1, truth);
    assert_eq!(fallback[0].1, falsehood);
    assert_eq!(fallback[1].1, falsehood);
    assert_eq!(
        first[0].0, second[0].0,
        "first comparator reused in arm two"
    );
    assert_eq!(
        first[0].0, fallback[0].0,
        "first comparator reused in fallback"
    );
    assert_eq!(
        second[1].0, fallback[1].0,
        "second comparator reused in fallback"
    );
    assert_ne!(first[0].0, second[1].0, "two different literal comparisons");
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
