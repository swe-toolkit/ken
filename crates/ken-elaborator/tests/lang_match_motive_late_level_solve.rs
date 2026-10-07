//! A motive sort retains a bare-Type level until its declaration has solved it.
//! Spec: `spec/30-surface/39-elaboration.md §5.7`.
//! Promise class: durable invariant. Measured: the checked match motive carries
//! `Type 1`, the later conflicting use is rejected at declaration admission,
//! and the prelude and catalog root remain admitted under fallback. Claimed:
//! every accept/reject decision equals the defaulted base query's. The gap:
//! a late-solved sort that also needs rigid level conversion falls back to
//! the early default, as base did; that shape is not repaired here.

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{Decl, KernelError, Level, Term};
use std::path::PathBuf;

const TWO: &str = "fn two (y : Nat) (c : Type 1) : Nat = y";
const REPRO: &str = "fn m1 (a : Type) (n : Nat) (x : a) (h : a → Nat) : Nat = \
    two (h (match n { Zero ↦ x; Suc k ↦ x })) a";
const BOOL: &str = "fn m1 (a : Type) (n : Bool) (x : a) (h : a → Nat) : Nat = \
    two (h (match n { True ↦ x; False ↦ x })) a";
const ASCRIBED: &str = "fn m1 (a : Type) (n : Nat) (x : a) (h : a → Nat) : Nat = \
    two (h ((match n { Zero ↦ x; Suc k ↦ x }) : a)) a";
const NESTED: &str = "fn m1 (a : Type) (n : Nat) (b : Bool) (x : a) (h : a → Nat) : Nat = \
    two (h (match n { Zero ↦ match b { True ↦ x; False ↦ x }; Suc k ↦ x })) a";
const EXPLICIT: &str = "fn m1 (a : Type 1) (n : Nat) (x : a) (h : a → Nat) : Nat = \
    two (h (match n { Zero ↦ x; Suc k ↦ x })) a";
const SOLVE_BEFORE: &str = "fn m1 (a : Type) (n : Nat) (x : a) (h : a → Nat) : Nat = \
    let c : Type 1 = a in h (match n { Zero ↦ x; Suc k ↦ x })";
const NO_LATER: &str = "fn m1 (a : Type) (n : Nat) (x : a) (h : a → Nat) : Nat = \
    h (match n { Zero ↦ x; Suc k ↦ x })";
const NO_MATCH: &str = "fn m1 (a : Type) (n : Nat) (x : a) (h : a → Nat) : Nat = two (h x) a";
const THREE: &str = "fn three (y : Nat) (c : Type 0) : Nat = y";
const CONFLICT: &str = "fn m1 (a : Type) (n : Nat) (x : a) (h : a → Nat) : Nat = \
    two (three (h (match n { Zero ↦ x; Suc k ↦ x })) a) a";

fn env_with_two() -> ElabEnv {
    let mut env = ElabEnv::new().expect("prelude including forced-level match sorts");
    env.elaborate_decl(TWO).expect("Type 1 consumer");
    env
}

fn checked_body(env: &ElabEnv) -> &Term {
    let id = *env.globals.get("m1").expect("accepted declaration");
    let Some(Decl::Transparent { body, .. }) = env.env.lookup(id) else {
        panic!("m1 must carry checked core");
    };
    body
}

fn motive_has_sort(term: &Term, level: &Level) -> bool {
    match term {
        Term::Elim { motive, .. } => {
            let Term::Ascript(_, ty) = motive.as_ref() else {
                return false;
            };
            let mut sort = ty.as_ref();
            while let Term::Pi(_, codomain) = sort {
                sort = codomain;
            }
            matches!(sort, Term::Type(found) if found == level)
        }
        Term::Lam(_, body) | Term::Pi(_, body) | Term::Ascript(body, _) => {
            motive_has_sort(body, level)
        }
        Term::App(function, argument) => {
            motive_has_sort(function, level) || motive_has_sort(argument, level)
        }
        Term::Let { val, body, .. } => motive_has_sort(val, level) || motive_has_sort(body, level),
        _ => false,
    }
}

#[test]
fn prelude_initializes_with_open_motive_query() {
    ElabEnv::new().expect("open-query fallback must preserve prelude admission");
}

#[test]
fn effectful_classes_roots_loader_preserves_defaulted_match_admission() {
    let mut env = ElabEnv::new().expect("prelude");
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages");
    env.elaborate_module_from_roots(&[root], "Core.Classes.EffectfulClasses")
        .expect("EffectfulClasses option_traverse_composed must still be admitted");
    assert!(env
        .globals
        .contains_key("Core.Classes.EffectfulClasses.option_traverse_composed"));
}

#[test]
fn late_solved_motive_sorts_are_type_one_in_checked_core() {
    let one = Level::Suc(Box::new(Level::Zero));
    for (name, source) in [
        ("Nat", REPRO),
        ("Bool", BOOL),
        ("ascribed", ASCRIBED),
        ("nested", NESTED),
    ] {
        let mut env = env_with_two();
        let trust = env.env.trusted_base();
        env.elaborate_decl(source)
            .unwrap_or_else(|error| panic!("{name} must check: {error:?}"));
        assert!(
            motive_has_sort(checked_body(&env), &one),
            "{name} match motive must carry the later-solved Type 1"
        );
        assert_eq!(env.env.trusted_base(), trust, "{name} adds no trust");
    }
}

#[test]
fn earlier_solve_no_solve_no_match_and_explicit_level_still_check() {
    for (name, source) in [
        ("explicit", EXPLICIT),
        ("solve before", SOLVE_BEFORE),
        ("no later solve", NO_LATER),
        ("no match", NO_MATCH),
    ] {
        env_with_two()
            .elaborate_decl(source)
            .unwrap_or_else(|error| panic!("{name} must check: {error:?}"));
    }
}

#[test]
fn incompatible_later_universe_use_rejects_at_admission() {
    let mut env = env_with_two();
    env.elaborate_decl(THREE).expect("Type 0 consumer");
    let error = env
        .elaborate_decl(CONFLICT)
        .expect_err("Type 0 and Type 1 conflict");
    match error {
        ElabError::KernelRejected {
            error: KernelError::TypeMismatch { expected, found },
            span,
        } => {
            assert!(matches!(*expected, Term::Type(Level::Suc(_))));
            assert_eq!(*found, Term::Type(Level::Zero));
            assert_eq!(span.start, 0, "rejection belongs to the declaration");
            assert_eq!(span.end, CONFLICT.len(), "rejection belongs to admission");
        }
        other => panic!("must reject at declaration admission: {other:?}"),
    }
    assert!(
        !env.globals.contains_key("m1"),
        "rejected definition is not admitted"
    );
}
