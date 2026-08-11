use std::collections::BTreeSet;

use ken_elaborator::error::ElabError;
use ken_elaborator::layout::format_ken;
use ken_elaborator::lossless::parse_lossless;
use ken_elaborator::parser::parse_expr;
use ken_elaborator::ElabEnv;
use ken_kernel::{whnf, Context, GlobalId, Term};

fn reduced_body(env: &ElabEnv, id: GlobalId) -> Term {
    let (_, body) = env
        .env
        .transparent_body(id)
        .expect("test constant must be transparent");
    whnf(&env.env, &Context::new(), &body)
}

fn assert_int(term: &Term, expected: &str) {
    match term {
        Term::IntLit(value) => assert_eq!(value.to_string(), expected),
        other => panic!("expected IntLit({expected}), got {other:?}"),
    }
}

fn find_elim_family(term: &Term) -> Option<GlobalId> {
    match term {
        Term::Elim { fam, .. } => Some(*fam),
        Term::App(function, argument)
        | Term::Pi(function, argument)
        | Term::Lam(function, argument)
        | Term::Sigma(function, argument)
        | Term::Ascript(function, argument) => {
            find_elim_family(function).or_else(|| find_elim_family(argument))
        }
        Term::Let { ty, val, body } => find_elim_family(ty)
            .or_else(|| find_elim_family(val))
            .or_else(|| find_elim_family(body)),
        _ => None,
    }
}

/// Promise class: semantic positive.
/// MEASURED: both preregistered Bool constructors select their corresponding
/// branch and reduce to distinct closed integer values.
/// CLAIMED: `if` elaborates as ordinary Bool case analysis.
/// THE GAP: successful parsing alone would not detect swapped methods.
#[test]
fn true_and_false_select_their_respective_branches() {
    let mut env = ElabEnv::new().expect("base environment");
    let true_id = env
        .elaborate_decl("const when_true : Int = if True then 11 else 22")
        .expect("True conditional");
    let false_id = env
        .elaborate_decl("const when_false : Int = if False then 11 else 22")
        .expect("False conditional");

    assert_int(&reduced_body(&env, true_id), "11");
    assert_int(&reduced_body(&env, false_id), "22");
}

/// Promise class: identity/negative-control.
/// MEASURED: source constructors named `True` and `False` replace the surface
/// name map, while the conditional still targets the original Bool family and
/// computes with the captured constructor identities.
/// CLAIMED: conditionals cannot be retargeted by source-level shadowing.
/// THE GAP: a fixture without the name-map inequality would not prove that
/// shadowing actually occurred.
#[test]
fn shadowed_constructor_names_cannot_retarget_if() {
    let mut env = ElabEnv::new().expect("base environment");
    let bool_id = env.numeric_env.bool_id;
    let true_id = env.numeric_env.bool_true_id;
    let false_id = env.numeric_env.bool_false_id;
    env.elaborate_decl("const canonical_true : Bool = True")
        .expect("capture a Bool value before source shadowing");
    let ids = env
        .elaborate_file(
            "data Shadow = True | False\n\
             const shadow_safe : Int = if canonical_true then 31 else 47",
        )
        .expect("shadowing fixture must elaborate");

    assert_ne!(
        env.globals["True"], true_id,
        "captured True id: {true_id:?}"
    );
    assert_ne!(
        env.globals["False"], false_id,
        "captured False id: {false_id:?}"
    );
    let result_id = *ids.last().expect("result declaration");
    let (_, body) = env.env.transparent_body(result_id).expect("result body");
    assert_eq!(find_elim_family(&body), Some(bool_id));
    assert_int(&whnf(&env.env, &Context::new(), &body), "31");
}

/// Promise class: diagnostic negative.
/// MEASURED: a non-Bool condition raises the dedicated conditional diagnostic.
/// CLAIMED: match exhaustiveness diagnostics do not leak through `if`.
/// THE GAP: `is_err()` would accept an unrelated parser or match failure.
#[test]
fn non_bool_condition_has_if_specific_diagnostic() {
    let mut env = ElabEnv::new().expect("base environment");
    let error = env
        .elaborate_decl("const bad_if : Int = if 0 then 1 else 2")
        .expect_err("Int condition must be rejected");
    assert!(matches!(error, ElabError::IfConditionNotBool { .. }));
}

/// Promise class: syntax preservation.
/// MEASURED: token/trivia replay is byte exact and canonical formatting retains
/// conditional syntax rather than printing its core case analysis.
/// CLAIMED: `if` is a real lossless surface node.
/// THE GAP: checking only the elaborated core cannot distinguish surface sugar.
#[test]
fn lossless_and_layout_keep_if_surface_syntax() {
    let source = "const answer : Int = if True then 1 else 2 -- kept\n";
    let parsed = parse_lossless(source).expect("conditional parses losslessly");
    assert_eq!(parsed.reconstruct(), source);
    let formatted = format_ken(source).expect("conditional formats");
    assert!(formatted.contains("if True then 1 else 2"));
    assert!(!formatted.contains("match"));
}

/// Promise class: grammar binding and contextual reachability.
/// MEASURED: argument and let-value positions elaborate, while nested AST
/// shapes and computation pin each `else` to its nearest unmatched `if`.
/// CLAIMED: conditionals are contextual primaries with recursive else branches.
/// THE GAP: a single top-level conditional would miss both precedence seams.
#[test]
fn argument_let_and_nested_else_binding_are_reachable() {
    parse_expr("if False then 1 else if True then 2 else 3").expect("outer else chain parses");
    parse_expr("if True then if False then 1 else 2 else 3").expect("inner else chain parses");

    let mut env = ElabEnv::new().expect("base environment");
    let before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    let ids = env
        .elaborate_file(
            "fn id_int (x : Int) : Int = x\n\
             const arg_if : Int = id_int if True then 5 else 6\n\
             const let_if : Int = let x : Int = if False then 7 else 8 in x\n\
             const outer_else : Int = if False then 1 else if True then 2 else 3\n\
             const inner_else : Int = if True then if False then 1 else 2 else 3",
        )
        .expect("contextual conditional fixture");
    let after: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(before, after, "surface conditional must not change the TCB");
    for (id, expected) in ids[1..].iter().zip(["5", "8", "2", "2"]) {
        assert_int(&reduced_body(&env, *id), expected);
    }
}
