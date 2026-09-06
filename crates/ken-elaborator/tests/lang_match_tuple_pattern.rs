//! `LANG-MATCH-TUPLE-PATTERN`: projection-only controls for `32 §4` and
//! `34 §3.1`.
//!
//! Promise class: durable invariants. These controls distinguish negative
//! Sigma projection from data elimination, retain the written tuple arity, and
//! pin componentwise coverage and reachability.

use ken_elaborator::{error::ElabError, ArmDeadCause, ElabEnv};
use ken_kernel::{whnf, Context, GlobalId, Term};

fn elaborate(env: &mut ElabEnv, source: &str) -> GlobalId {
    env.elaborate_decl(source)
        .unwrap_or_else(|error| panic!("elaboration failed: {error}"))
}

fn body(env: &ElabEnv, id: GlobalId) -> Term {
    env.env
        .transparent_body(id)
        .expect("test declaration is transparent")
        .1
}

fn constructor(id: GlobalId, arguments: impl IntoIterator<Item = Term>) -> Term {
    arguments.into_iter().fold(
        Term::Constructor {
            id,
            level_args: Vec::new(),
        },
        Term::app,
    )
}

fn count_projection_nodes(term: &Term) -> (usize, usize) {
    let here = match term {
        Term::Proj1(_) => (1, 0),
        Term::Proj2(_) => (0, 1),
        _ => (0, 0),
    };
    term.children().into_iter().fold(here, |counts, child| {
        let child_counts = count_projection_nodes(child);
        (counts.0 + child_counts.0, counts.1 + child_counts.1)
    })
}

fn normalize_pairs(env: &ElabEnv, term: &Term) -> Term {
    match whnf(&env.env, &Context::new(), term) {
        Term::Pair(first, second) => {
            Term::pair(normalize_pairs(env, &first), normalize_pairs(env, &second))
        }
        reduced => reduced,
    }
}

fn count_eliminators(term: &Term) -> usize {
    usize::from(matches!(term, Term::Elim { .. }))
        + term
            .children()
            .into_iter()
            .map(count_eliminators)
            .sum::<usize>()
}

#[test]
fn top_level_tuple_binds_components_by_projection_without_pair_elimination() {
    // MEASURED: the body reconstructs the source pair from both component
    // binders, its unreduced core contains both projections, and it contains no
    // Elim. CLAIMED: negative Sigma matching binds componentwise by projection.
    // THE GAP: normalization alone would not distinguish projections from a
    // hypothetical positive pair eliminator, so the raw inventory is asserted.
    let mut env = ElabEnv::new().expect("base environment");
    let trusted_before = env.env.trusted_base();
    elaborate(
        &mut env,
        "const tuple_input : (x : Nat) × Nat = (Zero, Suc Zero)",
    );
    let rebound = elaborate(
        &mut env,
        "const tuple_rebound : (x : Nat) × Nat = \
         match tuple_input { (first, second) |-> (first, second) }",
    );
    let captured = elaborate(
        &mut env,
        "const tuple_captured : (x : Nat) × Nat = \
         match tuple_input { (first, second) as whole |-> whole }",
    );

    let raw = body(&env, rebound);
    let (proj1, proj2) = count_projection_nodes(&raw);
    assert!(
        proj1 >= 1,
        "tuple lowering must project the first component"
    );
    assert!(
        proj2 >= 1,
        "tuple lowering must project the second component"
    );
    assert_eq!(
        count_eliminators(&raw),
        0,
        "Sigma matching must not emit elim_D"
    );

    let zero = constructor(env.globals["Zero"], []);
    let suc_zero = constructor(env.globals["Suc"], [zero.clone()]);
    let expected = Term::pair(zero, suc_zero);
    assert_eq!(normalize_pairs(&env, &raw), expected);
    assert_eq!(normalize_pairs(&env, &body(&env, captured)), expected);
    assert_eq!(env.env.trusted_base(), trusted_before);
}

#[test]
fn arity_three_right_nests_and_supplies_every_body_binding() {
    // MEASURED: `(a,b,c)` reassembles the right-nested carrier and normalizes
    // to `(Zero,(Suc Zero,Suc (Suc Zero)))`. CLAIMED: surface arity is retained
    // while lowering revisits the second component as a nested tuple.
    let mut env = ElabEnv::new().expect("base environment");
    elaborate(
        &mut env,
        "const triple_input : (x : Nat) × (y : Nat) × Nat = \
         (Zero, Suc Zero, Suc (Suc Zero))",
    );
    let rebound = elaborate(
        &mut env,
        "const triple_rebound : (x : Nat) × (y : Nat) × Nat = \
         match triple_input { (a, b, c) |-> (a, b, c) }",
    );

    let zero = constructor(env.globals["Zero"], []);
    let one = constructor(env.globals["Suc"], [zero.clone()]);
    let two = constructor(env.globals["Suc"], [one.clone()]);
    assert_eq!(
        normalize_pairs(&env, &body(&env, rebound)),
        Term::pair(zero, Term::pair(one, two))
    );
}

#[test]
fn dependent_sigma_second_component_uses_the_first_projection() {
    // MEASURED: the second binder checks as a Nat only after the first
    // projection selects Nat from `(A : Type) × A`. CLAIMED: the second matrix
    // column retains the Sigma codomain dependency rather than flattening it.
    let mut env = ElabEnv::new().expect("base environment");
    elaborate(
        &mut env,
        "const dependent_input : (a : Type) × a = (Nat, Suc Zero)",
    );
    let selected = elaborate(
        &mut env,
        "const dependent_selected : Nat = match dependent_input { \
         (a, value) |-> let checked : a = value in Zero }",
    );

    let zero = constructor(env.globals["Zero"], []);
    assert_eq!(whnf(&env.env, &Context::new(), &body(&env, selected)), zero);
}

#[test]
fn one_tuple_arm_is_exhaustive_but_an_inner_constructor_gap_is_named() {
    // MEASURED: the all-variable tuple accepts, while the componentwise Bool
    // grid names the missing False constructor. CLAIMED: coverage descends into
    // tuple components instead of treating the whole tuple as uninhabited.
    let mut env = ElabEnv::new().expect("base environment");
    elaborate(
        &mut env,
        "const bool_pair : (x : Bool) × Bool = (True, False)",
    );
    elaborate(
        &mut env,
        "const exhaustive_tuple : Bool = \
         match bool_pair { (left, right) |-> left }",
    );

    match env.elaborate_decl(
        "const incomplete_tuple : Nat = match bool_pair { \
         (True, True) |-> Zero ; (False, other) |-> Zero }",
    ) {
        Err(ElabError::ExhaustivenessError { missing, .. }) => {
            assert_eq!(missing.constructor, "False");
            assert_eq!(missing.arity, 0);
        }
        other => panic!("expected the missing inner False component, got {other:?}"),
    }
}

#[test]
fn redundant_tuple_arm_is_subsumed_never_no_inhabitants() {
    // MEASURED: the second all-variable tuple reaches the same component leaf
    // already claimed by the first. CLAIMED: redundancy preserves the existing
    // Subsumed cause and never falls through to NoInhabitants.
    let mut env = ElabEnv::new().expect("base environment");
    elaborate(
        &mut env,
        "const redundant_input : (x : Nat) × Nat = (Zero, Zero)",
    );
    match env.elaborate_decl(
        "const redundant_tuple : Nat = match redundant_input { \
         (first, second) |-> first ; (other_first, other_second) |-> other_second }",
    ) {
        Err(ElabError::ReachabilityError {
            cause: ArmDeadCause::Subsumed { rest, .. },
            ..
        }) => assert!(rest.is_empty()),
        Err(ElabError::ReachabilityError {
            cause: ArmDeadCause::NoInhabitants,
            ..
        }) => panic!("a redundant tuple arm must not be classified NoInhabitants"),
        other => panic!("expected tuple-arm subsumption, got {other:?}"),
    }
}

fn assert_top_level_refusal(pattern: &str) {
    let mut env = ElabEnv::new().expect("base environment");
    let source = format!("const refused : Nat = match Zero {{ {pattern} |-> Zero }}");
    match env.elaborate_decl(&source) {
        Err(ElabError::Internal(msg)) => assert_eq!(
            msg,
            "non-constructor pattern in match (wildcard/var not yet supported at top level; \
             use constructor patterns)"
        ),
        other => panic!("top-level wildcard/variable must remain refused, got {other:?}"),
    }
}

#[test]
fn grouping_is_not_a_tuple_and_top_level_catchalls_remain_refused() {
    // `(value)` reaches the same refusal as bare `value`; if it became a
    // one-tuple it would take the newly accepted projection path instead.
    assert_top_level_refusal("value");
    assert_top_level_refusal("(value)");
    assert_top_level_refusal("_");
}

#[test]
fn empty_and_trailing_comma_tuple_patterns_have_surface_diagnostics() {
    let mut env = ElabEnv::new().expect("base environment");
    for (pattern, expected) in [
        ("()", "empty tuple patterns are not allowed"),
        ("(first,)", "tuple patterns require a pattern after ','"),
    ] {
        let source = format!("const malformed : Nat = match Zero {{ {pattern} |-> Zero }}");
        match env.elaborate_decl(&source) {
            Err(ElabError::ParseError { msg, .. }) => assert_eq!(msg, expected),
            other => panic!("expected tuple surface diagnostic, got {other:?}"),
        }
    }
}

#[test]
fn tuple_descent_composes_with_outer_constructors_and_inner_aliases() {
    // MEASURED: a tuple nested in a constructor field reaches its Suc branch,
    // and the as-pattern returns the complete Suc occurrence rather than its
    // child. CLAIMED: projected tuple columns reuse the landed general matrix
    // occurrence through both enclosing constructor and alias consumers.
    let mut env = ElabEnv::new().expect("base environment");
    elaborate(&mut env, "data TupleBox = MkTupleBox ((x : Nat) × Nat)");
    elaborate(
        &mut env,
        "const boxed_tuple : TupleBox = MkTupleBox (Suc Zero, Zero)",
    );
    let selected = elaborate(
        &mut env,
        "const nested_tuple_selected : Nat = match boxed_tuple { \
         MkTupleBox (Zero, other) |-> other ; \
         MkTupleBox (Suc child as whole, other) |-> whole }",
    );

    let zero = constructor(env.globals["Zero"], []);
    assert_eq!(
        whnf(&env.env, &Context::new(), &body(&env, selected)),
        constructor(env.globals["Suc"], [zero])
    );
}

#[test]
fn module_rewriting_recurses_through_tuple_components() {
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_file(
        "module TupleOwner { \
         pub data TupleFlag = TupleOn | TupleOff ; \
         pub const make : TupleFlag = TupleOn ; \
         pub fn select (pair : (x : TupleFlag) × Nat) : Nat = match pair { \
           (TupleOn, value) |-> value ; (TupleOff, value) |-> value \
         } \
         }",
    )
    .expect("module tuple pattern elaborates");
    elaborate(
        &mut env,
        "const module_tuple_result : Nat = \
         TupleOwner.select (TupleOwner.make, Suc Zero)",
    );
}
