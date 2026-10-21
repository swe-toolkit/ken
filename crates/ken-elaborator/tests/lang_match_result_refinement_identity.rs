//! A match result retains the checked subset type at every leaf. Open proof
//! demand under an unsupported indexed split refuses by type; recursive
//! unindexed case-only paths carry checked equations and keep open goals unknown.

use ken_elaborator::{ElabEnv, ElabError, ElabResult, ObligationKind};
use ken_kernel::{check, normalize, Context, Decl, GlobalId, Term};

const FIVE: &str = "def Five = { n : Int | Equal Int n 5 }\n";
const INDEX: &str =
    "data Ix : Nat -> Type where { Z : List Int -> Ix Zero; S : List Int -> Ix (Suc Zero) }\n";
const ROSE: &str = "data DRose : Type where { DLeaf : DRose; DNode : List DRose -> DRose }\n";

fn checked(source: &str, expected: usize) -> (ElabEnv, ElabResult) {
    let mut env = ElabEnv::new().expect("prelude");
    let results = env
        .elaborate_file_v1(source)
        .unwrap_or_else(|error| panic!("{source}: {error:?}"));
    let result = results.last().expect("checked declaration").clone();
    assert_eq!(result.obligations.len(), expected, "{source}");
    (env, result)
}

fn refuses_open_split(source: &str, rejected_name: &str) {
    let mut env = ElabEnv::new().expect("prelude");
    let before_trust = env.env.trusted_base();
    let error = env
        .elaborate_file_v1(source)
        .expect_err("an open subset hole cannot demand an unsupported indexed split");
    assert!(
        matches!(error, ElabError::NonConvoyableRefinementSplit { .. }),
        "split refusal must be typed: {error:?}; source={source}"
    );
    assert!(
        !env.globals.contains_key(rejected_name),
        "failed result admitted a name"
    );
    assert_eq!(
        env.env.trusted_base(),
        before_trust,
        "failed result admitted a proof hole or other trust"
    );
}

#[test]
fn nonrecursive_result_matrix_keeps_each_open_leaf_checked() {
    let (env, result) = checked(
        &format!(
            "{FIVE}fn g (x : Int) (b : Bool) : Five = \
             match b {{ True ↦ x; False ↦ x }}"
        ),
        2,
    );
    assert!(result
        .obligations
        .iter()
        .all(|o| matches!(o.kind, ObligationKind::RefinementIntroduction)
            && env.is_open_hole(o.hole_id)));
    let ty = match env.env.lookup(result.def_id) {
        Some(ken_kernel::Decl::Transparent { ty, .. }) => ty,
        other => panic!("checked result type: {other:?}"),
    };
    let Term::Pi(_, after_x) = ty else {
        panic!("first parameter must have a Pi")
    };
    let Term::Pi(_, codomain) = after_x.as_ref() else {
        panic!("Bool parameter must have a Pi")
    };
    assert!(
        matches!(
            ken_kernel::whnf(&env.env, &Context::new(), codomain),
            Term::Sigma(_, _)
        ),
        "the function returns a checked subset"
    );
}

fn has_equation_for_family(term: &Term, family: GlobalId) -> bool {
    if let Term::Eq(carrier, _, _) = term {
        let mut head = carrier.as_ref();
        while let Term::App(function, _) = head {
            head = function;
        }
        if matches!(head, Term::IndFormer { id, .. } if *id == family) {
            return true;
        }
    }
    term.children()
        .iter()
        .any(|child| has_equation_for_family(child, family))
}

/// Promise class: durable invariant for case-only recursive splits (22 §3).
/// MEASURED: open goals remain open while the emitted checked case-only
/// eliminator carries their constructor equation. CLAIMED: the equation has
/// a term binder, not a fabricated proof. THE GAP: consuming-IH paths are
/// kept on the original elaboration route and pinned separately.
#[test]
fn recursive_unindexed_case_only_result_goals_carry_checked_equations() {
    for (source, name, family, count) in [
        (
            format!(
                "{FIVE}fn list_case (xs : List Bool) (x : Int) : Five = \
             match xs {{ Nil ↦ x; Cons b tail ↦ x }}"
            ),
            "list_case",
            "List",
            2,
        ),
        (
            format!(
                "{FIVE}fn rec (n : Nat) (x : Int) : Five = \
             match n {{ Zero ↦ x; Suc m ↦ rec m x }}"
            ),
            "rec",
            "Nat",
            1,
        ),
    ] {
        let (env, result) = checked(&source, count);
        assert_eq!(
            env.globals[name], result.def_id,
            "recursive result admitted"
        );
        assert!(
            result.obligations.iter().all(|goal| matches!(
                goal.kind,
                ObligationKind::RefinementIntroduction
            ) && env.is_open_hole(goal.hole_id)
                && has_equation_for_family(&goal.goal_closed, env.globals[family])),
            "open goals carry the checked recursive equation: {source}"
        );
        let Some(Decl::Transparent { ty, body, .. }) = env.env.lookup(result.def_id) else {
            panic!("recursive result is a checked transparent definition: {source}")
        };
        check(&env.env, &Context::new(), body, ty).expect("kernel checks recursive subset results");
    }
}

/// Promise class: transition sentinel for a future structured-IH convoy.
/// MEASURED: a demanding, non-consuming `Former`-field match retains the
/// existing checked body and open goals without a `DRose` constructor Eq.
/// CLAIMED: non-direct fields take the base method builder, even though
/// direct recursive fields can now carry an equation. THE GAP: this does not
/// by itself control Pi/Sigma shapes; mutation of the shared shape guard
/// must reject this exact fixture during declaration admission.
#[test]
fn structured_recursive_family_under_demand_keeps_base_path() {
    let source = format!(
        "{FIVE}{ROSE}fn nested (r : DRose) (x : Int) : Five = \
         match r {{ DLeaf ↦ x; DNode kids ↦ x }}"
    );
    let (env, result) = checked(&source, 2);
    assert_eq!(env.globals["nested"], result.def_id);
    assert!(
        result.obligations.iter().all(|goal| matches!(
            goal.kind,
            ObligationKind::RefinementIntroduction
        ) && env.is_open_hole(goal.hole_id)
            && !has_equation_for_family(&goal.goal_closed, env.globals["DRose"])),
        "a structured recursive field retains the base open goals"
    );
    let Some(Decl::Transparent { ty, body, .. }) = env.env.lookup(result.def_id) else {
        panic!("structured result is a checked transparent definition")
    };
    check(&env.env, &Context::new(), body, ty).expect("kernel checks the structured result");
}

/// Promise class: transition sentinel for future consuming-IH convoys
/// (22 §3, 34 §3.3). MEASURED: a nested former's generated List lift and
/// `recursive result for rest` are consumed beneath a refined result. The
/// Nil obligation is discharged, the Cons obligation remains open, and
/// neither gains a constructor equation for the lifted List or NRose.
/// CLAIMED: an IH consumer never enters the case-only convoy. THE GAP: this has
/// both selector and lift-binding provenance; the classifier's two arms
/// need separate controls if they are later refactored independently.
#[test]
fn recursive_result_selector_under_refined_result_keeps_consuming_base_path() {
    let source = r#"
        data NRose : Type where {
            NLeaf : NRose ; NNode : List (Pair String NRose) -> NRose
        }
        fn rsize (r : NRose) : { n : Nat | Equal Nat n (Suc Zero) } =
            match r {
                NLeaf |-> Suc Zero ;
                NNode members |-> match members {
                    Nil |-> Zero ;
                    Cons member rest |-> recursive result for rest
                }
            }
    "#;
    let (env, result) = checked(source, 2);
    assert_eq!(env.globals["rsize"], result.def_id);
    assert_eq!(
        result
            .obligations
            .iter()
            .map(|goal| env.is_open_hole(goal.hole_id))
            .collect::<Vec<_>>(),
        [false, true],
        "the trivial Nil proof and open Cons proof retain their base dispositions"
    );
    for goal in &result.obligations {
        assert!(matches!(goal.kind, ObligationKind::RefinementIntroduction));
        assert!(
            !has_equation_for_family(&goal.goal_closed, env.globals["List"]),
            "lifted List must not receive a convoy Eq: {goal:?}"
        );
        assert!(
            !has_equation_for_family(&goal.goal_closed, env.globals["NRose"]),
            "enclosing NRose must not receive a convoy Eq: {goal:?}"
        );
    }
    let Some(Decl::Transparent { ty, body, .. }) = env.env.lookup(result.def_id) else {
        panic!("nested P2 result is a checked transparent definition")
    };
    check(&env.env, &Context::new(), body, ty).expect("kernel checks the consuming result");
}

/// Promise class: transition sentinel for the lift-binding P2 route (22 §3).
/// MEASURED: the inner List match's scrutinee has a generated lift, but no
/// arm contains a recursive-result selector. Its checked result goals keep
/// their base status without a List or NRose constructor equation.
/// CLAIMED: a lift alone excludes the case-only convoy. THE GAP: the
/// independent lift-arm deletion must redden this same fixture.
#[test]
fn lifted_scrutinee_without_selector_keeps_consuming_base_path() {
    let source = r#"
        data NRose : Type where {
            NLeaf : NRose ; NNode : List (Pair String NRose) -> NRose
        }
        fn rsize (r : NRose) : { n : Nat | Equal Nat n (Suc Zero) } =
            match r {
                NLeaf |-> Suc Zero ;
                NNode members |-> match members {
                    Nil |-> Zero ;
                    Cons member rest |-> Zero
                }
            }
    "#;
    let (env, result) = checked(source, 3);
    assert_eq!(env.globals["rsize"], result.def_id);
    assert_eq!(
        result
            .obligations
            .iter()
            .map(|goal| env.is_open_hole(goal.hole_id))
            .collect::<Vec<_>>(),
        [false, true, true],
        "the closed NLeaf and open List leaves retain their base statuses"
    );
    for goal in &result.obligations {
        assert!(matches!(goal.kind, ObligationKind::RefinementIntroduction));
        assert!(
            !has_equation_for_family(&goal.goal_closed, env.globals["List"]),
            "the lifted List must not receive a case-only convoy: {goal:?}"
        );
        assert!(
            !has_equation_for_family(&goal.goal_closed, env.globals["NRose"]),
            "the structured host stays on its base route: {goal:?}"
        );
    }
    let Some(Decl::Transparent { ty, body, .. }) = env.env.lookup(result.def_id) else {
        panic!("lifted result is a checked transparent definition")
    };
    check(&env.env, &Context::new(), body, ty).expect("kernel checks lifted result");
}

/// Promise class: transition sentinel for the source-selector P2 route
/// (22 §3). MEASURED: the eligible outer Nat split has open refined-result
/// goals, a plain scrutinee without a lift, and a recursive-result selector
/// only inside one nested-family arm. Its goals have no Nat constructor Eq.
/// CLAIMED: an IH selector anywhere in an arm tree excludes that match from
/// case-only convoy. THE GAP: the independent selector-arm deletion must
/// redden this fixture without modifying the lift-binding arm.
#[test]
fn nested_selector_without_lift_on_outer_scrutinee_keeps_base_path() {
    let source = r#"
        data Bag (a : Type) : Type where {
            Empty : Bag a ; One : a -> Bag a ; Join : Bag a -> Bag a -> Bag a
        }
        data LiftRose = LiftLeaf | LiftNode (Bag LiftRose)
        fn f (n : Nat) (r : LiftRose) : { x : Int | Equal Int x 5 } =
            match n {
                Zero |-> 6 ;
                Suc m |-> match r {
                    LiftLeaf |-> 6 ;
                    LiftNode b |-> match b {
                        Empty |-> 5 ;
                        One x |-> 5 ;
                        Join xs ys |-> recursive result for xs
                    }
                }
            }
    "#;
    let (env, result) = checked(source, 4);
    assert_eq!(env.globals["f"], result.def_id);
    assert_eq!(
        result
            .obligations
            .iter()
            .map(|goal| env.is_open_hole(goal.hole_id))
            .collect::<Vec<_>>(),
        [true, true, false, false],
        "outer open goals and nested checked goals retain their base statuses"
    );
    for goal in &result.obligations {
        assert!(matches!(goal.kind, ObligationKind::RefinementIntroduction));
        assert!(
            !has_equation_for_family(&goal.goal_closed, env.globals["Nat"]),
            "the outer Nat match must not receive a case-only convoy: {goal:?}"
        );
    }
    let Some(Decl::Transparent { ty, body, .. }) = env.env.lookup(result.def_id) else {
        panic!("selector result is a checked transparent definition")
    };
    check(&env.env, &Context::new(), body, ty).expect("kernel checks nested selector result");
}

/// Promise class: durable indexed refusal. The homogeneous fixed-index
/// branch has no heterogeneous index equation that could support its open
/// result hole, so demand retains the typed refusal.
#[test]
fn unsupported_homogeneous_indexed_open_result_split_refuses_transactionally() {
    refuses_open_split(
        &format!(
            "{FIVE}{INDEX}fn homogeneous (i : Ix Zero) (x : Int) : Five = \
             match i {{ Z xs ↦ x }}"
        ),
        "homogeneous",
    );
}

/// Promise class: durable checked-index path. A heterogeneous `Ix n` split
/// has its existing checked index evidence; the new filter must not remove
/// its three independently emitted introduction obligations.
#[test]
fn indexed_nested_split_preserves_checked_index_path() {
    let (env, result) = checked(
        &format!(
            "{FIVE}{INDEX}fn indexed_nested (n : Nat) (i : Ix n) (x : Int) : Five = \
             match i {{ Z (Cons y ys) ↦ x; Z Nil ↦ x; S ys ↦ x }}"
        ),
        3,
    );
    assert!(
        result.obligations.iter().all(|goal| matches!(
            goal.kind,
            ObligationKind::RefinementIntroduction
        ) && env.is_open_hole(goal.hole_id)),
        "the indexed split retains each open result leaf"
    );
    let Some(Decl::Transparent { ty, body, .. }) = env.env.lookup(result.def_id) else {
        panic!("indexed result remains a checked transparent definition")
    };
    check(&env.env, &Context::new(), body, ty).expect("kernel checks indexed result pairs");
}

#[test]
fn heterogeneous_indexed_paths_with_no_usable_equation_need_no_convoy() {
    let (env, result) = checked(
        &format!(
            "{FIVE}{INDEX}fn indexed (n : Nat) (i : Ix n) (x : Int) : Five = \
             match i {{ Z xs ↦ x; S ys ↦ x }}"
        ),
        2,
    );
    assert!(result
        .obligations
        .iter()
        .all(|o| matches!(o.kind, ObligationKind::RefinementIntroduction)
            && env.is_open_hole(o.hole_id)));
}

#[test]
fn closed_certificates_need_no_recursive_or_indexed_convoy() {
    for (source, count) in [
        (
            format!(
                "{FIVE}fn list_closed (xs : List Bool) : Five = \
             match xs {{ Nil ↦ 5; Cons b tail ↦ 5 }}"
            ),
            2,
        ),
        (
            format!(
                "{FIVE}{INDEX}fn indexed_closed (n : Nat) (i : Ix n) : Five = \
             match i {{ Z xs ↦ 5; S ys ↦ 5 }}"
            ),
            2,
        ),
        (
            format!(
                "{FIVE}{ROSE}fn rose_closed (r : DRose) : Five = \
             match r {{ DLeaf ↦ 5; DNode kids ↦ 5 }}"
            ),
            2,
        ),
    ] {
        let (env, result) = checked(&source, count);
        assert!(
            result
                .obligations
                .iter()
                .all(|o| !env.is_open_hole(o.hole_id)),
            "closed branch evidence must be admitted without a convoy: {source}"
        );
    }
}

#[test]
fn indexed_branch_requires_remains_a_live_obligation() {
    let (_, result) = checked(
        &format!(
            "{FIVE}{INDEX}\
             fn need (x : Int) : Int requires Equal Int x 5 = x\n\
             fn g (n : Nat) (i : Ix n) (x : Int) : Int =\n\
             match i {{ Z xs ↦ need x; S ys ↦ x }}"
        ),
        1,
    );
    assert!(matches!(
        result.obligations[0].kind,
        ObligationKind::Requires
    ));
}

#[test]
fn seeded_leaf_preserves_alias_through_index_impossible_inner_match() {
    let vec = "data Vec (a : Type) : Nat → Type where {\n\
        VNil : Vec a Zero; VCons : (n : Nat) → a → Vec a n → Vec a (Suc n)\n}\n";
    for body in [
        "match xs { VNil ↦ saved; VCons m e tl ↦ saved }",
        "let r = match xs { VNil ↦ saved; VCons m e tl ↦ saved } in r",
    ] {
        let source = format!(
            "{vec}\n\
            fn f (n : Nat) (xs : Vec Nat Zero) (x : Nat) : Nat =\n\
            match x {{ Zero ↦ Zero; (Suc j) as saved ↦ {body} }}\n\
            const observed : Nat = f Zero (VNil Nat) (Suc (Suc (Suc Zero)))\n\
            const expected : Nat = Suc (Suc (Suc Zero))"
        );
        let (env, _) = checked(&source, 0);
        let normal = |name: &str| {
            let id = env.globals[name];
            let term = env.env.transparent_body(id).expect("checked value").1;
            normalize(&env.env, &Context::new(), &term)
        };
        assert_eq!(normal("observed"), normal("expected"));
    }
}
