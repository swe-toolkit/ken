//! `LANG-SYMBOLIC-OPERATOR-NAMES` D1/D2 acceptance.
//!
//! Spec sources: `31-lexical.md` §1b, §2, and §4; `33-declarations.md` §6.
//! Promise class: durable invariants. Extending infix application or declared
//! fixity must not change these prefix-name, fixed-token, and identity results.

use ken_elaborator::lexer::{Lexer, Token};
use ken_elaborator::parser::parse_decls;
use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{Decl as KernelDecl, Term};

fn lex_one(source: &str) -> Token {
    let tokens = Lexer::lex(source).expect("symbolic source must lex");
    assert_eq!(
        tokens.len(),
        2,
        "one token plus EOF expected for {source:?}"
    );
    assert!(matches!(tokens[1].0, Token::Eof));
    tokens[0].0.clone()
}

fn assert_fixed_operator(source: &str, expected: Token, declaration: &str) {
    assert_eq!(
        lex_one(source),
        expected,
        "fixed token changed for {source:?}"
    );
    ElabEnv::new()
        .expect("base environment")
        .elaborate_decl(declaration)
        .unwrap_or_else(|error| panic!("fixed operator {source:?} stopped elaborating: {error:?}"));
}

/// MEASURED: every ASCII symbolic character derived from §1b's operator
/// transliterations plus §4's `+%` and fixed arithmetic spellings participates
/// in one carried-lexeme token. CLAIMED: the user-name lexer admits the bounded
/// symbolic alphabet. THE GAP: fixed spellings could pre-empt the run; the
/// separate exact fixed-token inventory below covers that collision axis.
#[test]
fn symbolic_character_run_lexes_to_one_carried_name() {
    let source = ":<+*/%=|\\->";
    assert_eq!(
        lex_one(source),
        Token::Operator(source.to_string()),
        "the symbolic alphabet is `+-*/%=<>|\\:`"
    );
}

/// MEASURED: each pre-existing ASCII token spelling on the widened character
/// surface retains its exact token discriminant. CLAIMED: symbolic-name lexing
/// preserves all fixed punctuation/operator behavior. THE GAP: token identity
/// alone does not prove arithmetic elaboration; the five named fixtures below
/// drive Plus, PlusPercent, Minus, Star, and EqEq through elaboration.
#[test]
fn fixed_ascii_symbolic_token_inventory_is_unchanged() {
    let cases = [
        (":", Token::Colon),
        ("::", Token::DoubleColon),
        ("|", Token::Pipe),
        ("|->", Token::MapsTo),
        ("||", Token::TruncBar),
        ("=", Token::Eq),
        ("==", Token::EqEq),
        ("===", Token::PropEq),
        ("\\", Token::Lambda),
        ("\\/", Token::Or),
        ("+", Token::Plus),
        ("+%", Token::PlusPercent),
        ("*", Token::Star),
        ("-", Token::Minus),
        ("->", Token::Arrow),
        ("<=", Token::Le),
        ("<:", Token::FlowsTo),
        (">=", Token::Ge),
        ("><", Token::Times),
        ("/=", Token::Ne),
        ("/\\", Token::And),
    ];
    for (source, expected) in cases {
        assert_eq!(
            lex_one(source),
            expected,
            "fixed token changed for {source:?}"
        );
    }

    let tokens = Lexer::lex("=-- comment\n+").expect("adjacent comment remains trivia");
    assert!(matches!(tokens[0].0, Token::Eq));
    assert!(matches!(tokens[1].0, Token::Plus));
    assert!(matches!(tokens[2].0, Token::Eof));
}

#[test]
fn fixed_plus_still_elaborates_as_addition() {
    assert_fixed_operator(
        "+",
        Token::Plus,
        "fn fixedPlus (a : Int) (b : Int) : Int = a + b",
    );
}

#[test]
fn fixed_plus_percent_still_elaborates_as_wrapping_addition() {
    assert_fixed_operator(
        "+%",
        Token::PlusPercent,
        "fn fixedWrappingPlus (a : UInt8) (b : UInt8) : UInt8 = a +% b",
    );
}

#[test]
fn fixed_minus_still_elaborates_as_subtraction() {
    assert_fixed_operator(
        "-",
        Token::Minus,
        "fn fixedMinus (a : Int) (b : Int) : Int = a - b",
    );
}

#[test]
fn fixed_star_still_elaborates_as_multiplication() {
    assert_fixed_operator(
        "*",
        Token::Star,
        "fn fixedStar (a : Int) (b : Int) : Int = a * b",
    );
}

#[test]
fn fixed_eqeq_still_elaborates_as_structural_equality() {
    assert_fixed_operator(
        "==",
        Token::EqEq,
        "fn fixedEqEq (a : Decimal) (b : Decimal) : Bool = a == b",
    );
}

/// MEASURED: the admitted declaration has the exact ordinary Pi/Lam/Var kernel
/// form and is registered under its carried symbolic spelling. CLAIMED: a
/// symbolic `fn` is semantically an ordinary function. THE GAP: a bespoke
/// elaboration path could mimic this shape; production uses the unchanged
/// ViewDecl/string-name path and the module-identity test below exercises its
/// existing resolution consumer.
#[test]
fn symbolic_fn_elaborates_to_the_ordinary_function_form() {
    let mut env = ElabEnv::new().expect("base environment");
    let id = env
        .elaborate_decl("fn <+> (a : Nat) (b : Nat) : Nat = a")
        .expect("symbolic function definition must elaborate");
    assert_eq!(env.globals.get("<+>"), Some(&id));

    let nat = Term::IndFormer {
        id: env.prelude_env.nat_id,
        level_args: vec![],
    };
    let expected_type = Term::Pi(
        Box::new(nat.clone()),
        Box::new(Term::Pi(Box::new(nat.clone()), Box::new(nat.clone()))),
    );
    let expected_body = Term::Lam(
        Box::new(nat.clone()),
        Box::new(Term::Lam(Box::new(nat), Box::new(Term::Var(1)))),
    );
    match env.env.lookup(id) {
        Some(KernelDecl::Transparent { ty, body, .. }) => {
            assert_eq!(ty, &expected_type);
            assert_eq!(body, &expected_body);
        }
        other => panic!("symbolic fn must be an ordinary transparent definition: {other:?}"),
    }
}

/// MEASURED: public export, selective import, prefix reference, and the
/// consumer body all retain the provider's exact GlobalId. CLAIMED: symbolic
/// names traverse existing resolution/export unchanged. THE GAP: qualified
/// `M.<+>` syntax is not claimed by D1/D2; the ordinary selective-import route
/// is the identity-bearing module path exercised here.
#[test]
fn symbolic_name_uses_existing_export_import_and_resolution_identity() {
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_file("module M { pub fn <+> (a : Nat) (b : Nat) : Nat = a }")
        .expect("symbolic provider module must elaborate");
    let provider = env.globals["M.<+>"];

    env.elaborate_file("import M (<+>)")
        .expect("symbolic selective import must elaborate");
    let consumer = env
        .elaborate_decl("fn useSymbolic (a : Nat) (b : Nat) : Nat = <+> a b")
        .expect("prefix symbolic reference must resolve");
    let (_, body) = env
        .env
        .transparent_body(consumer)
        .expect("transparent consumer");
    assert!(
        matches!(&body,
            Term::Lam(_, first)
                if matches!(first.as_ref(),
                    Term::Lam(_, second)
                        if matches!(second.as_ref(),
                            Term::App(function, argument)
                                if matches!(argument.as_ref(), Term::Var(0))
                                    && matches!(function.as_ref(),
                                        Term::App(head, first_argument)
                                            if matches!(first_argument.as_ref(), Term::Var(1))
                                                && matches!(head.as_ref(),
                                                    Term::Const { id, .. } if *id == provider))))),
        "consumer must apply the exact exported provider identity, got {body:?}"
    );
}

/// MEASURED: duplicate symbolic declarations reach the existing typed
/// duplicate-definition diagnostic with the carried spelling as payload.
/// CLAIMED: diagnostics consume symbolic names as ordinary global names. THE
/// GAP: a parser-local rejection could impersonate collision handling; matching
/// the exact post-parse error variant excludes that route.
#[test]
fn symbolic_name_uses_existing_duplicate_definition_diagnostic() {
    let mut env = ElabEnv::new().expect("base environment");
    match env.elaborate_file(
        "fn <+> (a : Nat) (b : Nat) : Nat = a\n\
         fn <+> (a : Nat) (b : Nat) : Nat = b",
    ) {
        Err(ElabError::DuplicateDefinition { name, .. }) => assert_eq!(name, "<+>"),
        other => panic!("symbolic collision must use DuplicateDefinition, got {other:?}"),
    }
}

/// MEASURED: prefix `<+> a b` elaborates, while the same operands in
/// `a <+> b` position produce a ParseError naming the unconsumed operator.
/// CLAIMED: D1/D2 add names and definitions without adding infix application.
/// THE GAP: any accidental inclusion in the application-argument start set
/// would accept the negative form; this paired fixture differs only by order.
#[test]
fn symbolic_prefix_reference_accepts_but_infix_application_still_rejects() {
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_decl("fn <+> (a : Nat) (b : Nat) : Nat = a")
        .expect("symbolic definition");
    env.elaborate_decl("fn prefixOk (a : Nat) (b : Nat) : Nat = <+> a b")
        .expect("prefix application is ordinary application");

    match parse_decls("fn infixStillOut (a : Nat) (b : Nat) : Nat = a <+> b") {
        Err(ElabError::ParseError { msg, .. }) => assert!(
            msg.contains("Operator(\"<+>\")"),
            "rejection must be caused by the unconsumed user operator, got {msg}"
        ),
        other => panic!("infix user application must remain outside D1/D2, got {other:?}"),
    }
}
