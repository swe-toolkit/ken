//! Durable behavioral invariants for spec/20-verification/22 §2.2.
//! The predicate is opaque so no reduction can conceal whole-body emission.

use ken_elaborator::{ElabEnv, ObligationKind};
use ken_kernel::{GlobalId, Level, Term};

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

/// Durable invariant. A recursive contract depending on an early parameter
/// must instantiate its IH with the direct call's full parameter and requires
/// telescope. MEASURED: the recursive path closes while the base case stays
/// open, with a non-final `tag` binder before `n` and `Q tag` in both goals.
/// CLAIMED: the IH substitutes actual call arguments, not the raw predicate.
/// THE GAP: mutating the IH collector to retain `psi` uninstantiated must
/// redden the recursive-path assertion while the original suite stays green.
#[test]
fn recursive_postcondition_substitutes_nonfinal_parameter_and_requirement() {
    let mut env = predicate_env();
    let int = Term::const_(env.globals["Int"], vec![]);
    let omega = Term::omega(Level::Zero);
    env.declare_postulate_raw(
        "P2",
        Term::pi(int.clone(), Term::pi(int.clone(), omega.clone())),
    )
    .expect("P2 : Int -> Int -> Omega");
    env.declare_postulate_raw("Q", Term::pi(int.clone(), omega))
        .expect("Q : Int -> Omega");
    let result = env.elaborate_decl_v1(
        "fn rec_dep (tag : Int) (n : Nat) : Int requires Q tag ensures P2 tag result = match n { Zero |-> 5 ; Suc m |-> rec_dep tag m }",
    ).expect("parameter-dependent recursive contract");
    let [zero, suc] = result.obligations.as_slice() else {
        panic!("exactly one contract goal per leaf; no extra requires hole")
    };
    let q_tag = Term::app(Term::const_(env.globals["Q"], vec![]), Term::var(1));
    for obligation in [zero, suc] {
        assert!(matches!(obligation.kind, ObligationKind::Ensures));
        let Term::Pi(tag_ty, rest) = &obligation.goal_closed else {
            panic!("missing first parameter binder")
        };
        assert_eq!(tag_ty.as_ref(), &int);
        let Term::Pi(_, rest) = rest.as_ref() else {
            panic!("missing second parameter binder")
        };
        let Term::Pi(requirement, _) = rest.as_ref() else {
            panic!("missing requires binder")
        };
        assert_eq!(requirement.as_ref(), &q_tag);
        assert!(!contains_elim(&obligation.goal_closed));
    }
    assert!(
        env.is_open_hole(zero.hole_id),
        "opaque base P2 tag 5 remains open"
    );
    assert!(
        !env.is_open_hole(suc.hole_id),
        "P2 tag (rec_dep tag m) must use the instantiated IH"
    );
}

/// Durable invariant. A non-flat constructor pattern splits again inside
/// the Suc bucket. MEASURED: each emitted goal has one equation per split,
/// and the recursive leaf has its contract IH. CLAIMED: obligations are
/// localized under every constructor path. THE GAP: an unthreaded matrix
/// would still produce three goals; the Eq-domain census closes that gap.
#[test]
fn nested_matrix_postconditions_carry_every_constructor_equation() {
    fn equation_constructors(goal: &Term) -> Vec<GlobalId> {
        let mut constructors = Vec::new();
        let mut current = goal;
        while let Term::Pi(domain, rest) = current {
            if let Term::Eq(_, _, right) = domain.as_ref() {
                let mut head = right.as_ref();
                while let Term::App(function, _) = head {
                    head = function;
                }
                let Term::Constructor { id, .. } = head else {
                    panic!("path equation lost its constructor: {domain:?}")
                };
                constructors.push(*id);
            }
            current = rest;
        }
        constructors
    }
    let mut env = predicate_env();
    for source in [
        "fn nested_rec (n : Nat) : Int ensures P result = match n { Zero |-> 5 ; Suc Zero |-> 6 ; Suc (Suc m) |-> nested_rec m }",
        "fn lit_nested_rec (n : Nat) : { x : Int | P x } = match n { Zero |-> 5 ; Suc Zero |-> 6 ; Suc (Suc m) |-> lit_nested_rec m }",
    ] {
        let result = env.elaborate_decl_v1(source)
            .unwrap_or_else(|error| panic!("{source}: {error:?}"));
        assert_eq!(
            result.obligations.len(),
            3,
            "one obligation per matrix path: {source}"
        );
        for (obligation, (constructors, open)) in result.obligations.iter().zip([
            (vec![env.globals["Zero"]], true),
            (vec![env.globals["Suc"], env.globals["Zero"]], true),
            (vec![env.globals["Suc"], env.globals["Suc"]], false),
        ]) {
            assert!(matches!(obligation.kind, ObligationKind::Ensures));
            assert_eq!(
                equation_constructors(&obligation.goal_closed),
                constructors,
                "every split contributes its own equation: {:?}",
                obligation.goal_closed
            );
            assert_eq!(
                env.is_open_hole(obligation.hole_id),
                open,
                "only the recursive path has a postcondition IH: {source}"
            );
            assert!(!contains_elim(&obligation.goal_closed));
        }
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
