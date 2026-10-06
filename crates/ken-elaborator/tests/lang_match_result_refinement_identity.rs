//! Durable checked-behavior pins for match-result refinement identity.
//! Spec: 20-surface/21 §2 and 20-verification/22 §2.1. A checked arm
//! introduces the source result's predicate, not its inferred carrier.

use ken_elaborator::{ElabEnv, ElabResult};
use ken_kernel::{GlobalId, Term};

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

/// MEASURED: all three nested-pattern leaves introduce Five, including the
/// first seed. CLAIMED: a known match result is never replaced by the first
/// inferred arm's carrier. GAP: indexed and nested host routes have separate
/// compilation paths, exercised below.
#[test]
fn seeded_matrix_checks_first_and_later_leaves() {
    checked(
        &format!(
            "{FIVE}fn g (x : Int) (ys : List Bool) : Five =\n\
        match ys {{ Nil ↦ x; Cons True t ↦ x; Cons False t ↦ x }}"
        ),
        3,
    );
    checked(
        &format!(
            "{FIVE}fn g (x : Int) (ys : List Bool) : Five =\n\
        match ys {{ Nil ↦ x; Cons a t ↦ x }}"
        ),
        2,
    );
    checked(
        &format!(
            "{FIVE}fn g (x : Int) (b : Bool) : Five =\n\
        if b then x else x"
        ),
        2,
    );
    checked(
        &format!(
            "{FIVE}fn g (x : Int) (b : Bool) : Five =\n\
        match b {{ True ↦ x; False ↦ x }}"
        ),
        2,
    );
    checked(
        &format!(
            "{FIVE}fn g (x : Int) : Five =\n\
        let y : Int = x in y"
        ),
        1,
    );
}

/// MEASURED: both heterogeneous indexed branches yield an open predicate,
/// while the same path with a literal result exercises the non-named route.
/// CLAIMED: an ill-typed constructor path equation cannot prevent a valid
/// refinement obligation from being stated. GAP: the same-index equation
/// must still be kept; the next pin inspects its checked goal.
#[test]
fn indexed_branch_obligations_survive_heterogeneous_constructor_paths() {
    let prefix = format!("{FIVE}{INDEX}");
    let (env, named) = checked(
        &format!(
            "{prefix}fn g (n : Nat) (i : Ix n) (x : Int) : Five =\n\
        match i {{ Z xs ↦ x; S ys ↦ 6 }}"
        ),
        2,
    );
    for obligation in &named.obligations {
        assert!(
            !contains_ix_equation(&obligation.goal_closed, env.globals["Ix"], None),
            "heterogeneous constructor path was closed into goal: {:?}",
            obligation.goal_closed
        );
    }
    checked(
        &format!(
            "{prefix}fn g (n : Nat) (i : Ix n) (x : Int) :\n\
        {{ r : Int | Equal Int r 5 }} =\n\
        match i {{ Z xs ↦ x; S ys ↦ 6 }}"
        ),
        2,
    );
    checked(
        &format!(
            "{prefix}fn g (n : Nat) (i : Ix n) (x : Int) : Five =\n\
        match i {{ Z (Cons y ys) ↦ x; Z Nil ↦ x; S ys ↦ 6 }}"
        ),
        3,
    );
    checked(
        &format!(
            "{prefix}fn g (n : Nat) (i : Ix n) (x : Int) :\n\
        {{ r : Int | Equal Int r 5 }} =\n\
        match i {{ Z (Cons y ys) ↦ x; Z Nil ↦ x; S ys ↦ 6 }}"
        ),
        3,
    );
    checked(
        &format!(
            "{prefix}fn g (n : Nat) (i : Ix n) (x : Int) (b : Bool) : Five =\n\
        match i {{ Z xs ↦ if b then x else 5; S ys ↦ 6 }}"
        ),
        3,
    );
}

fn contains_constructor(term: &Term, constructor: GlobalId) -> bool {
    matches!(term, Term::Constructor { id, .. } if *id == constructor)
        || term
            .children()
            .into_iter()
            .any(|child| contains_constructor(child, constructor))
}

fn contains_ix_equation(term: &Term, family: GlobalId, constructor: Option<GlobalId>) -> bool {
    let matching = if let Term::Eq(carrier, _, rhs) = term {
        let head = match carrier.as_ref() {
            Term::App(head, _) => head.as_ref(),
            other => other,
        };
        matches!(head, Term::IndFormer { id, .. } if *id == family)
            && constructor.is_none_or(|id| contains_constructor(rhs, id))
    } else {
        false
    };
    matching
        || term
            .children()
            .into_iter()
            .any(|child| contains_ix_equation(child, family, constructor))
}

/// A homogeneous indexed equation remains an obligation hypothesis. Dropping
/// all indexed equations would make this pin red even if the caller still
/// emits one nominal RefinementIntroduction hole.
#[test]
fn homogeneous_constructor_equation_remains_in_closed_goal() {
    let (env, result) = checked(
        &format!(
            "{FIVE}{INDEX}\
        fn g (i : Ix Zero) (x : Int) : Five = match i {{ Z xs ↦ x }}"
        ),
        1,
    );
    let family = env.globals["Ix"];
    let constructor = env.globals["Z"];
    assert!(
        contains_ix_equation(
            &result.obligations[0].goal_closed,
            family,
            Some(constructor)
        ),
        "the well-typed path equation was dropped: {:?}",
        result.obligations[0].goal_closed
    );
    checked(
        &format!(
            "{FIVE}{INDEX}\
        fn g (i : Ix Zero) (x : Int) : {{ r : Int | Equal Int r 5 }} =\n\
        match i {{ Z xs ↦ x }}"
        ),
        1,
    );
}

/// The structured-method boundary includes nested-former fields; its source
/// result stays named across each child and the nested match's leaves.
#[test]
fn structured_nested_former_checks_each_leaf_against_five() {
    let prefix = format!("{FIVE}{ROSE}");
    checked(
        &format!(
            "{prefix}fn r0 (r : DRose) (x : Int) : Five =\n\
        match r {{ DLeaf ↦ x; DNode kids ↦ x }}"
        ),
        2,
    );
    checked(
        &format!(
            "{prefix}fn r1 (r : DRose) (x : Int) : Five =\n\
        match r {{ DLeaf ↦ x; DNode kids ↦ match kids {{ Nil ↦ x; Cons k rest ↦ x }} }}"
        ),
        3,
    );
    checked(
        &format!(
            "{prefix}fn r2 (r : DRose) (x : Int) : Five =\n\
        match r {{ DLeaf ↦ x; DNode kids ↦ match kids {{ Nil ↦ 6; Cons k rest ↦ 7 }} }}"
        ),
        3,
    );
    checked(
        &format!(
            "{prefix}fn r3 (r : DRose) : Five =\n\
        match r {{ DLeaf ↦ 6; DNode kids ↦ 7 }}"
        ),
        2,
    );
}
