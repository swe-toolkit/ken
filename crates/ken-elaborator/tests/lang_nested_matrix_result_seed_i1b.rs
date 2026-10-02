//! A check-mode matrix must not turn an unsolved result-level metavariable
//! into Zero before its first leaf can solve the level from the arm type.
//! Spec: spec/30-surface/39-elaboration.md §5.7;
//! spec/30-surface/34-data-match.md §3.1–3.2, §4.4.
//! Promise class: durable value invariants, except the explicitly labelled
//! plain-match transition sentinel owned by LANG-MATCH-MOTIVE-LATE-LEVEL-SOLVE.

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{normalize, Context, KernelError, Level, Term};

const TUPLE: &str = "fn pick (a : Bool) (b : Bool) : Type = match (a, b) { \
                     (True, _) ↦ Type; (False, _) ↦ Type }";
const OR: &str = "fn pick (b : Bool) : Type = match b { True | False ↦ Type }";
const NESTED: &str = "fn pick (x : Nat) : Type = match x { \
                      Zero ↦ Type; Suc Zero ↦ Type; Suc (Suc k) ↦ Type }";
const RECORD: &str = "record Flags { first : Bool, second : Bool }\n\
                      fn pick (r : Flags) : Type = match r { \
                        { first = True } ↦ Type; { first = False } ↦ Type }";

fn checked_value(source: &str, call: &str, observed_ty: &str) -> (ElabEnv, Term) {
    let mut env = ElabEnv::new().expect("prelude");
    let trusted = env.env.trusted_base();
    env.elaborate_file(source)
        .unwrap_or_else(|error| panic!("{source}: {error:?}"));
    env.elaborate_decl(&format!("const observed : {observed_ty} = {call}"))
        .unwrap_or_else(|error| panic!("{call}: {error:?}"));
    let id = env.globals["observed"];
    let body = env
        .env
        .transparent_body(id)
        .expect("checked transparent value")
        .1;
    let normal = normalize(&env.env, &Context::new(), &body);
    assert_eq!(env.env.trusted_base(), trusted);
    (env, normal)
}

fn assert_type_zero_value(source: &str, call: &str) {
    let (_env, observed) = checked_value(source, call, "Type 1");
    assert_eq!(observed, Term::ty(Level::Zero));
}

#[test]
fn tuple_match_result_level_is_discovered_by_its_first_leaf() {
    // MEASURED: the tuple-pattern result checks at Type 1 and computes to
    // Type 0. CLAIMED: a check-mode goal with an unsolved level is not seeded
    // as Zero. THE GAP: this value reaches the tuple matrix; an always-seed
    // mutation must refuse it at the original TypeMismatch.
    assert_type_zero_value(TUPLE, "pick True False");
}

#[test]
fn or_match_result_level_is_discovered_by_its_first_leaf() {
    // MEASURED: an or-pattern result checks and computes to Type 0.
    // CLAIMED: the or-matrix does not freeze its unsolved goal level.
    // THE GAP: this path is a distinct match dispatcher from the tuple row;
    // the always-seed mutation must refuse this row independently.
    assert_type_zero_value(OR, "pick True");
}

#[test]
fn nested_match_result_level_is_discovered_by_its_first_leaf() {
    // MEASURED: a nested constructor split checks and computes to Type 0.
    // CLAIMED: its first leaf can solve the expected level metavariable.
    // THE GAP: a flat Bool match does not take this route; the always-seed
    // mutation must make the nested row refuse at the kernel boundary.
    assert_type_zero_value(NESTED, "pick Zero");
}

#[test]
fn record_match_result_level_is_discovered_by_its_first_leaf() {
    // MEASURED: an open named-record pattern checks and computes to Type 0.
    // CLAIMED: the record dispatcher shares the result-seed boundary.
    // THE GAP: before increment 1 this record source checked; after it
    // landed it refused with KernelRejected TypeMismatch, so this row is a
    // regression pin rather than a presumed extension of the tuple result.
    assert_type_zero_value(RECORD, "pick { first = True, second = False }");
}

#[test]
fn explicit_level_still_seeds_and_normalizes() {
    // MEASURED: an explicitly solved Type 1 goal checks and returns Type 0.
    // CLAIMED: the new guard does not discard every check-mode seed.
    // THE GAP: the first-bucket trace in elab.rs additionally witnesses
    // the populated slot before a nested split for a ground expected type.
    assert_type_zero_value(
        "fn pick (b : Bool) : Type 1 = match b { True | False ↦ Type }",
        "pick False",
    );
}

#[test]
fn type_zero_arms_and_inference_mode_remain_accepted() {
    // MEASURED: Nat-valued type arms and inference-mode Type-valued arms
    // check at their own appropriate universe levels. CLAIMED: unseeded
    // discovery and solved low-level results remain available.
    // THE GAP: the ground check-mode seed is separately witnessed by the
    // preceding explicit-level control and the existing first-bucket pin.
    let (mut env, observed) = checked_value(
        "fn pick (b : Bool) : Type = match b { True | False ↦ Nat }",
        "pick True",
        "Type 0",
    );
    env.elaborate_decl("const expected : Type 0 = Nat")
        .expect("Nat lives at Type 0");
    let body = env
        .env
        .transparent_body(env.globals["expected"])
        .expect("body")
        .1;
    assert_eq!(observed, normalize(&env.env, &Context::new(), &body));
    assert_type_zero_value(
        "fn pick (b : Bool) : Type = let r = match b { True | False ↦ Type } in r",
        "pick False",
    );
}

#[test]
fn plain_flat_match_late_level_solve_stays_a_transition_refusal() {
    // Transition sentinel for LANG-MATCH-MOTIVE-LATE-LEVEL-SOLVE, not an I-1b
    // repair claim. MEASURED: this flat match still refuses at its own span.
    // CLAIMED: the pre-existing later-solve gap does not silently accept a
    // wrong motive. THE GAP: a future repair of that separate WP must flip
    // this sentinel deliberately, not weaken the result-level regression pins.
    let mut env = ElabEnv::new().expect("prelude");
    let error = env
        .elaborate_file("fn pick (b : Bool) : Type = match b { True ↦ Type; False ↦ Type }")
        .expect_err("flat late-level result remains a separate refusal");
    assert!(
        matches!(
            error,
            ElabError::KernelRejected {
                error: KernelError::TypeMismatch { .. },
                ref span,
            } if span.start == 0
        ),
        "{error:?}"
    );
}
