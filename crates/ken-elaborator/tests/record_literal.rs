use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{convert, Context, Decl, Term};
use num_bigint::BigInt;

fn body(env: &ElabEnv, name: &str) -> Term {
    match env.env.lookup(env.globals[name]) {
        Some(Decl::Transparent { body, .. }) => body.clone(),
        other => panic!("expected transparent body for {name}, got {other:?}"),
    }
}

fn pair_depth(term: &Term) -> usize {
    match term {
        Term::Pair(_, tail) => 1 + pair_depth(tail),
        _ => 0,
    }
}

#[test]
fn named_literals_follow_declaration_order_not_written_order() {
    // Promise class: durable invariant. MEASURED: the two written orders produce
    // the same three-pair core term. CLAIMED: name-directed placement follows the
    // declaration vector. THE GAP: equality alone cannot distinguish a shared
    // wrong order, so the non-symmetric three-field values and pair depth pin the
    // declaration-order shape independently.
    let mut env = ElabEnv::empty().expect("prelude");
    let trusted_before = env.env.trusted_base();
    env.elaborate_file(
        "record Triple { first : Int, second : Int, third : Int }\n\
         view ordered : Triple = { first = 1, second = 2, third = 3 }\n\
         view permuted : Triple = { third = 3, first = 1, second = 2 }",
    )
    .expect("three-field literals elaborate");
    let expected = Term::pair(
        Term::IntLit(BigInt::from(1)),
        Term::pair(
            Term::IntLit(BigInt::from(2)),
            Term::pair(
                Term::IntLit(BigInt::from(3)),
                Term::const_(env.class_env.record_nil_val_id, vec![]),
            ),
        ),
    );
    assert_eq!(body(&env, "ordered"), expected);
    assert_eq!(body(&env, "permuted"), expected);
    assert_eq!(pair_depth(&body(&env, "ordered")), 3);
    assert_eq!(env.env.trusted_base(), trusted_before);
}

#[test]
fn dependent_field_values_use_prior_literal_fields() {
    // Promise class: durable invariant. MEASURED: a later value checks against
    // the type supplied by the earlier field. CLAIMED: literal checking applies
    // the named-field telescope outermost-first. THE GAP: independent Int fields
    // would not exercise substitution, so this fixture makes the second field's
    // expected type depend directly on the first field's value.
    let mut env = ElabEnv::empty().expect("prelude");
    env.elaborate_file(
        "record Dependent { carrier : Type, value : carrier }\n\
         view dependent : Dependent = { value = 1, carrier = Int }",
    )
    .expect("dependent literal fields elaborate in declaration order");
    assert_eq!(pair_depth(&body(&env, "dependent")), 2);
}

#[test]
fn puns_resolve_in_the_enclosing_scope_and_follow_field_names() {
    // Promise class: durable invariant. MEASURED: a reversed binder order gives
    // the same core term for explicit and punned fields. CLAIMED: pun RHS names
    // resolve in the enclosing scope. THE GAP: closed values would not exercise
    // capture, so both RHS values are open parameters whose order is reversed.
    let mut env = ElabEnv::empty().expect("prelude");
    env.elaborate_file(
        "record Point { x : Int, y : Int }\n\
         view explicit (y : Int) (x : Int) : Point = { x = x, y = y }\n\
         view punned (y : Int) (x : Int) : Point = { y, x }",
    )
    .expect("shadowed puns elaborate");
    assert_eq!(body(&env, "explicit"), body(&env, "punned"));
}

#[test]
fn record_literals_are_unparenthesized_application_arguments() {
    // Promise class: durable invariant. MEASURED: a record literal parses and
    // elaborates as the unparenthesized argument of an ordinary application.
    // CLAIMED: expression-position record literals are atom expressions. THE
    // GAP: a first-position literal would not exercise application lookahead,
    // so the atom-start mutation must make this unchanged fixture fail.
    let mut env = ElabEnv::empty().expect("prelude");
    env.elaborate_file(
        "record Point { x : Int, y : Int }\n\
         view consume (p : Point) : Int = p.x\n\
         view applied : Int = consume { x = 1, y = 2 }",
    )
    .expect("unparenthesized record-literal application argument elaborates");
}

#[test]
fn update_rebuilds_from_projections_and_is_eta_respecting() {
    // Promise class: durable invariant. MEASURED: the kernel converts an empty
    // update to identity while non-empty update forms elaborate. CLAIMED: omitted
    // fields are rebuilt through projections. THE GAP: conversion does not name
    // the lowering route; the parser-arm mutation proves these forms depend on
    // the expression-record production, while inspection pins the projection path.
    let mut env = ElabEnv::empty().expect("prelude");
    env.elaborate_file(
        "record Point { x : Int, y : Int }\n\
         view unchanged (p : Point) : Point = { p | }\n\
         view replaced (p : Point) : Point = { p | y = 3 }\n\
         view rebuilt (p : Point) : Point = { x = p.x, y = 3 }\n\
         view replaced_both (p : Point) : Point = { p | x = 1, y = 3 }\n\
         view rebuilt_both (p : Point) : Point = { x = 1, y = 3 }\n\
         view identity (p : Point) : Point = p",
    )
    .expect("empty and single-field updates elaborate");
    let unchanged = env.env.lookup(env.globals["unchanged"]).unwrap();
    let identity = env.env.lookup(env.globals["identity"]).unwrap();
    let (
        Decl::Transparent {
            ty,
            body: rebuilt_core,
            ..
        },
        Decl::Transparent {
            body: original_core,
            ..
        },
    ) = (unchanged, identity)
    else {
        panic!("eta controls must be transparent");
    };
    assert!(convert(
        &env.env,
        &Context::new(),
        ty,
        rebuilt_core,
        original_core
    ));
    assert_eq!(body(&env, "replaced"), body(&env, "rebuilt"));
    assert_eq!(body(&env, "replaced_both"), body(&env, "rebuilt_both"));
}

#[test]
fn missing_duplicate_and_foreign_fields_refuse_at_source_spans() {
    // Promise class: durable invariant. MEASURED: each malformed population
    // reaches its specific diagnostic, with duplicate/foreign names at their own
    // spans. CLAIMED: completeness, uniqueness, and ownership are checked after
    // parsing. THE GAP: rejection alone can pass for the wrong reason, so each
    // case asserts its concrete error family and the valid tests are controls.
    let mut env = ElabEnv::empty().expect("prelude");
    env.elaborate_decl("record Point { x : Int, y : Int }")
        .unwrap();

    let missing = "{ x = 1 }";
    let missing_source = format!("view bad : Point = {missing}");
    let missing_start = missing_source.find(missing).unwrap();
    let error = env
        .elaborate_decl(&missing_source)
        .expect_err("missing field");
    match error {
        ElabError::TypeMismatch { span, reason } => {
            assert_eq!(
                (span.start, span.end),
                (missing_start, missing_start + missing.len())
            );
            assert!(reason.contains("Point") && reason.contains("y"));
        }
        other => panic!("wrong missing-field error: {other:?}"),
    }

    let duplicate = "view bad_dup : Point = { x = 1, x = 2, y = 3 }";
    let second = duplicate.rfind("x = 2").unwrap();
    match env.elaborate_decl(duplicate).expect_err("duplicate field") {
        ElabError::TypeMismatch { span, reason } => {
            assert_eq!((span.start, span.end), (second, second + 1));
            assert!(reason.contains("Point") && reason.contains("x"));
        }
        other => panic!("wrong duplicate error: {other:?}"),
    }

    let foreign = "view bad_foreign : Point = { x = 1, z = 2, y = 3 }";
    let z = foreign.find("z = 2").unwrap();
    match env.elaborate_decl(foreign).expect_err("foreign field") {
        ElabError::UnresolvedCon { name, span } => {
            assert_eq!(name, "Point.z");
            assert_eq!((span.start, span.end), (z, z + 1));
        }
        other => panic!("wrong foreign error: {other:?}"),
    }
}

#[test]
fn brace_neighbours_and_declaration_regressions_remain_live() {
    // Promise class: durable invariant. MEASURED: every AC-5 neighbour enumerated
    // by the frame elaborates beside a record expression. CLAIMED: the new atom
    // arm does not steal those declaration/type entry points. THE GAP: no current
    // pre-existing expression form opens with a brace, so that cell is measured
    // empty rather than represented by a fabricated syntax.
    let mut env = ElabEnv::empty().expect("prelude");
    env.elaborate_file(
        "record Point { x : Int, y : Int }\n\
         view literal : Point = { x = 1, y = 2 }\n\
         def Nonnegative = { n : Int | Equal Int n n }\n\
         class Pick A { select : A }\n\
         instance Pick Bool { select = True }\n\
         module M { const ok : Bool = True }\n\
         view positional : Point = (1, 2)",
    )
    .expect("all AC-5 neighbours and positional construction elaborate");
}

#[test]
fn refinement_braces_parse_without_expression_record_braces() {
    // Promise class: durable invariant. MEASURED: a refinement type elaborates
    // without any expression record in its source. CLAIMED: type refinements use
    // a separate parser entry point. THE GAP: the parser-arm mutation supplies
    // the discriminating A/B run: record expressions fail while this stays green.
    let mut env = ElabEnv::empty().expect("prelude");
    env.elaborate_decl("def Nonnegative = { n : Int | Equal Int n n }")
        .expect("refinement braces remain a type form");
}
