use std::collections::BTreeSet;

use ken_elaborator::error::ElabError;
use ken_elaborator::layout::format_ken;
use ken_elaborator::lexer::{Lexer, Token};
use ken_elaborator::lossless::parse_lossless;
use ken_elaborator::ElabEnv;
use ken_kernel::{whnf, Context, Term};

fn token_kinds(source: &str) -> Vec<Token> {
    Lexer::lex(source)
        .expect("fixture must lex")
        .into_iter()
        .map(|(token, _)| token)
        .collect()
}

/// Promise class: lexical discrimination.
/// MEASURED: a projection chain produces two dot/index pairs, while the same
/// digits at token start remain one float literal.
/// CLAIMED: the fraction decision belongs to the lexer and does not consume a
/// positional projection index.
/// THE GAP: parser-only success would not distinguish a lexer fix from a
/// projection-specific parser workaround.
#[test]
fn projection_chains_and_float_literals_split_at_the_lexer() {
    assert_eq!(
        token_kinds("p.1.2"),
        vec![
            Token::Ident("p".into()),
            Token::Dot,
            Token::Nat(1),
            Token::Dot,
            Token::Nat(2),
            Token::Eof,
        ]
    );
    assert_eq!(token_kinds("3.14"), vec![Token::FloatLit(3.14), Token::Eof]);
}

/// Promise class: semantic positive and written arity.
/// MEASURED: a three-component literal lowers to one right-nested pair, and
/// chained `.2.1`/`.2.2` projections reduce to the second and third values.
/// CLAIMED: surface arity is retained until elaboration and core nesting is
/// right associative.
/// THE GAP: two-component projection tests alone cannot detect left nesting or
/// premature AST desugaring of the third component.
#[test]
fn triples_lower_once_to_right_nested_core_pairs() {
    let mut env = ElabEnv::new().expect("base environment");
    let (triple, _) = env
        .elaborate_expr("triple", "(11, 22, 33)")
        .expect("triple must elaborate");
    let Term::Ascript(pair, _) = &triple else {
        panic!("inferred pair must carry its type, got {triple:?}");
    };
    let Term::Pair(first, tail) = pair.as_ref() else {
        panic!("triple must lower to Pair, got {pair:?}");
    };
    assert!(matches!(first.as_ref(), Term::IntLit(_)));
    assert!(matches!(
        tail.as_ref(),
        Term::Ascript(inner, _) if matches!(inner.as_ref(), Term::Pair(_, _))
    ));

    for (source, expected) in [("(11, 22, 33).2.1", "22"), ("(11, 22, 33).2.2", "33")] {
        let (term, _) = env.elaborate_expr("triple projection", source).unwrap();
        match whnf(&env.env, &Context::new(), &term) {
            Term::IntLit(value) => assert_eq!(value.to_string(), expected),
            other => panic!("expected IntLit({expected}), got {other:?}"),
        }
    }
}

/// Promise class: dependent checking and inference.
/// MEASURED: a checked Vec-indexed pair inhabits its dependent Sigma, while an
/// unannotated integer pair infers a nondependent Sigma.
/// CLAIMED: pair checking substitutes the first component into the codomain;
/// bare-pair inference constructs the nondependent special case.
/// THE GAP: either witness alone leaves the other elaboration direction open.
#[test]
fn checked_dependent_and_inferred_nondependent_pairs_elaborate() {
    let mut env = ElabEnv::new().expect("base environment");
    let before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();

    let (_, dependent_type) = env
        .elaborate_expr("dependent pair", "((Nat, Zero) : (x : Type) × x)")
        .expect("dependent pair must check");
    assert!(matches!(dependent_type, Term::Sigma(_, _)));

    let (_, inferred_type) = env
        .elaborate_expr("inferred pair", "(11, 22)")
        .expect("bare pair must infer");
    assert!(matches!(inferred_type, Term::Sigma(_, _)));

    let after: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(before, after, "surface pairs must not change the TCB");
}

/// Promise class: diagnostic negative.
/// MEASURED: projecting `.1` from a parenthesized integer reaches the dedicated
/// pair diagnostic and records the requested position.
/// CLAIMED: positional projection refusal is owned by pair elaboration rather
/// than leaking a generic type or named-record error.
/// THE GAP: `is_err()` would accept a lexer, parser, or unrelated type failure.
#[test]
fn non_pair_projection_has_pair_specific_diagnostic() {
    let mut env = ElabEnv::new().expect("base environment");
    let error = env
        .elaborate_expr("bad positional projection", "(3).1")
        .expect_err("Int is not a pair");
    assert!(matches!(
        error,
        ElabError::PositionalProjectionNotPair { projection: 1, .. }
    ));
}

/// Promise class: syntax preservation and parser seams.
/// MEASURED: lossless replay is byte exact, formatting retains the written
/// three-component literal, and grouping plus ascription still admit `.2`.
/// CLAIMED: pair syntax is a real surface node integrated with existing
/// parenthesized-expression paths.
/// THE GAP: checking only elaborated core cannot observe written arity or
/// grouping/ascription regressions.
#[test]
fn lossless_layout_grouping_and_ascription_preserve_pair_surface() {
    let source = "const triple : (x : Int) × (y : Int) × Int = (1, 2, 3) -- kept\n";
    let parsed = parse_lossless(source).expect("pair fixture parses losslessly");
    assert_eq!(parsed.reconstruct(), source);
    let formatted = format_ken(source).expect("pair fixture formats");
    assert!(formatted.contains("(1, 2, 3)"));
    assert!(!formatted.contains("(1, (2, 3))"));

    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_expr("grouped pair", "((1, 2)).2")
        .expect("grouped pair projection");
    env.elaborate_expr("ascribed pair", "((1, 2) : (x : Int) × Int).2")
        .expect("ascribed pair projection");
}
