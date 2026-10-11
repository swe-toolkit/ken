//! Late-solved motive levels across if and inferred match elaboration.
//! Spec: spec/30-surface/39-elaboration.md §5.7.
//! Promise class: durable invariant. MEASURED: each fixed source elaborates,
//! and its checked declaration carries a Type 1 motive after the later use
//! solves the bare-Type level; explicit Type 1 counterparts still elaborate.
//! CLAIMED: motive-sort inference preserves a level solved later in the same
//! declaration at these conditional and inferred-match paths. THE GAP: these
//! five expressions do not cover generated index convoys or every future
//! motive builder; those are separate sites and require their own witnesses.

use ken_elaborator::ElabEnv;
use ken_kernel::{Decl, Level, Term};

const TWO: &str = "fn two (y : Nat) (c : Type 1) : Nat = y";

fn motive_has_sort(term: &Term, expected: &Level) -> bool {
    match term {
        Term::Elim { motive, .. } => {
            let Term::Ascript(_, ty) = motive.as_ref() else {
                return false;
            };
            let mut sort = ty.as_ref();
            while let Term::Pi(_, codomain) = sort {
                sort = codomain;
            }
            matches!(sort, Term::Type(level) if level == expected)
        }
        Term::Lam(_, body) | Term::Pi(_, body) | Term::Ascript(body, _) => {
            motive_has_sort(body, expected)
        }
        Term::App(function, argument) => {
            motive_has_sort(function, expected) || motive_has_sort(argument, expected)
        }
        Term::Let { val, body, .. } => {
            motive_has_sort(val, expected) || motive_has_sort(body, expected)
        }
        _ => false,
    }
}

fn assert_late_solved_motive(source: &str) {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_decl(TWO).expect("Type 1 consumer");
    let owner = env
        .elaborate_decl(source)
        .unwrap_or_else(|error| panic!("late-solved declaration must check: {error:?}; {source}"));
    let Some(Decl::Transparent { body, .. }) = env.env.lookup(owner) else {
        panic!("checked declaration must carry a transparent body: {source}");
    };
    let one = Level::Suc(Box::new(Level::Zero));
    assert!(
        motive_has_sort(body, &one),
        "checked body must carry a Type 1 motive: {source}"
    );
}

macro_rules! row {
    ($name:ident, $sort:literal, $expression:literal) => {
        #[test]
        fn $name() {
            let source = format!(
                "fn m1 (a : {}) (n : Nat) (b : Bool) (x : a) (h : a → Nat) : Nat = two ({}) a",
                $sort, $expression
            );
            assert_late_solved_motive(&source);
        }
    };
}

row!(if_late_solved_motive, "Type", "h (if b then x else x)");
row!(
    if_explicit_type_one_control,
    "Type 1",
    "h (if b then x else x)"
);
row!(
    inferred_match_late_solved_motive,
    "Type",
    "(match n { Zero ↦ h; Suc k ↦ h }) x"
);
row!(
    inferred_match_explicit_type_one_control,
    "Type 1",
    "(match n { Zero ↦ h; Suc k ↦ h }) x"
);
row!(
    let_bound_match_late_solved_motive,
    "Type",
    "let y = match n { Zero ↦ x; Suc k ↦ x } in h y"
);
row!(
    let_bound_match_explicit_type_one_control,
    "Type 1",
    "let y = match n { Zero ↦ x; Suc k ↦ x } in h y"
);
row!(
    nested_matrix_late_solved_motive,
    "Type",
    "h (match n { Zero ↦ x; Suc Zero ↦ x; Suc (Suc k) ↦ x })"
);
row!(
    nested_matrix_explicit_type_one_control,
    "Type 1",
    "h (match n { Zero ↦ x; Suc Zero ↦ x; Suc (Suc k) ↦ x })"
);
row!(
    nested_inferred_match_late_solved_motive,
    "Type",
    "(match n { Zero ↦ h; Suc Zero ↦ h; Suc (Suc k) ↦ h }) x"
);
row!(
    nested_inferred_match_explicit_type_one_control,
    "Type 1",
    "(match n { Zero ↦ h; Suc Zero ↦ h; Suc (Suc k) ↦ h }) x"
);
