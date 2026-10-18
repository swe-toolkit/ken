//! A match result retains the checked subset type at every leaf. Open proof
//! demand under a recursive or unsupported indexed split refuses by type;
//! a closed certificate does not demand a split witness.

use ken_elaborator::{ElabEnv, ElabError, ElabResult, ObligationKind};
use ken_kernel::{normalize, Context, Term};

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
        .expect_err("an open subset hole cannot demand a recursive or indexed split");
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

#[test]
fn recursive_and_indexed_open_result_splits_refuse_transactionally() {
    for (source, name) in [
        (
            format!(
                "{FIVE}fn list_case (xs : List Bool) (x : Int) : Five = \
             match xs {{ Nil ↦ x; Cons b tail ↦ x }}"
            ),
            "list_case",
        ),
        (
            format!(
                "{FIVE}fn rec (n : Nat) (x : Int) : Five = \
             match n {{ Zero ↦ x; Suc m ↦ rec m x }}"
            ),
            "rec",
        ),
        (
            format!(
                "{FIVE}{INDEX}fn indexed_nested (n : Nat) (i : Ix n) (x : Int) : Five = \
             match i {{ Z (Cons y ys) ↦ x; Z Nil ↦ x; S ys ↦ x }}"
            ),
            "indexed_nested",
        ),
        (
            format!(
                "{FIVE}{INDEX}fn homogeneous (i : Ix Zero) (x : Int) : Five = \
             match i {{ Z xs ↦ x }}"
            ),
            "homogeneous",
        ),
        (
            format!(
                "{FIVE}{ROSE}fn nested (r : DRose) (x : Int) : Five = \
             match r {{ DLeaf ↦ x; DNode kids ↦ x }}"
            ),
            "nested",
        ),
    ] {
        refuses_open_split(&source, name);
    }
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
