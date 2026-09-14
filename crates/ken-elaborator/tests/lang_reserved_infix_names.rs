//! `LANG-RESERVED-INFIX-NAMES` A0 acceptance.
//!
//! Spec sources: `spec/30-surface/31-lexical.md` §1c and
//! `spec/30-surface/32-grammar.md` §§1, 3, and 6.
//! Promise classes: the six-name/five-alias roster is a normative compatibility
//! vector; ordinary application, GlobalId-keyed fixity, grammar exclusions, and
//! the absence of implicit bindings are durable invariants.

use std::collections::BTreeSet;

use ken_elaborator::lexer::{Lexer, Token};
use ken_elaborator::parser::{parse_decls, parse_expr};
use ken_elaborator::{BinOp, ElabEnv, ElabError, Expr};
use ken_interp::eval::{eval, EvalStore, EvalVal};
use ken_kernel::{GlobalId, Term};

const RESERVED: [(&str, Option<&str>); 6] = [
    ("≤", Some("<=")),
    ("≥", Some(">=")),
    ("≠", Some("/=")),
    ("∧", Some("/\\")),
    ("∨", Some("\\/")),
    ("∈", None),
];

fn body(env: &ElabEnv, name: &str) -> Term {
    let id = env.globals[name];
    env.env
        .transparent_body(id)
        .unwrap_or_else(|| panic!("{name:?} must be transparent"))
        .1
        .clone()
}

fn eval_int(env: &ElabEnv, name: &str) -> i128 {
    let mut store = EvalStore::new();
    for (id, value) in &env.num_values {
        if let ken_elaborator::NumericLitVal::Int(value) = value {
            store.num_values.insert(*id, EvalVal::from(value.clone()));
        }
    }
    match eval(&[], &body(env, name), &env.env, &mut store) {
        EvalVal::Int(value) => value as i128,
        other => panic!("{name:?} must evaluate to Int, got {other:?}"),
    }
}

fn common_fixture(spelling: &str, canonical: &str) -> (GlobalId, Term, Term) {
    let mut env = ElabEnv::new().expect("base environment");
    let source = format!(
        "fn {spelling} (x : Nat) (y : Nat) : Nat = x\n\
         infixr 5 {spelling}\n\
         fn prefix (x : Nat) (y : Nat) : Nat = {spelling} x y\n\
         fn infix_once (x : Nat) (y : Nat) : Nat = x {spelling} y\n\
         fn infix_chain (x : Nat) (y : Nat) (z : Nat) : Nat = x {spelling} y {spelling} z\n\
         fn expected_chain (x : Nat) (y : Nat) (z : Nat) : Nat = {spelling} x ({spelling} y z)"
    );
    let tokens = Lexer::lex(&source).expect("operator fixture must lex");
    if RESERVED.iter().any(|(glyph, _)| *glyph == canonical) {
        assert!(
            tokens
                .iter()
                .all(|(token, _)| !matches!(token, Token::Operator(_))),
            "reserved fixture {spelling:?} must not rely on the generic Operator token"
        );
    }
    env.elaborate_file(&source)
        .unwrap_or_else(|error| panic!("reserved fixture {spelling:?}: {error:?}"));

    let id = env.globals[canonical];
    assert_eq!(
        env.fixities.get(&id).map(|fixity| fixity.precedence),
        Some(5)
    );
    assert_eq!(body(&env, "prefix"), body(&env, "infix_once"));
    assert_eq!(body(&env, "infix_chain"), body(&env, "expected_chain"));
    let definition = env
        .env
        .transparent_body(id)
        .expect("reserved operator definition")
        .1
        .clone();
    (id, definition, body(&env, "infix_chain"))
}

fn assert_parse_error_at(source: &str, marked: &str, expected: &str) {
    let start = source.find(marked).expect("marked token occurs");
    match parse_decls(source) {
        Err(ElabError::ParseError { msg, span }) => {
            assert_eq!(msg, expected, "wrong diagnostic for {source:?}");
            assert_eq!(
                (span.start, span.end),
                (start, start + marked.len()),
                "wrong primary span for {source:?}"
            );
        }
        other => panic!("expected parser refusal for {source:?}, got {other:?}"),
    }
}

#[test]
fn every_reserved_name_and_alias_reaches_declaration_prefix_infix_and_fixity() {
    for (canonical, ascii) in RESERVED {
        let glyph = common_fixture(canonical, canonical);
        if let Some(ascii) = ascii {
            let alias = common_fixture(ascii, canonical);
            assert_eq!(glyph, alias, "{canonical:?}/{ascii:?} must be one name");
        }
    }
}

#[test]
fn simultaneous_reserved_names_remain_distinct_without_generic_operator_tokens() {
    let source = r#"
fn ≤ (x : Int) (y : Int) : Int = 11
fn ≥ (x : Int) (y : Int) : Int = 22
fn ≠ (x : Int) (y : Int) : Int = 33
fn ∧ (x : Int) (y : Int) : Int = 44
fn ∨ (x : Int) (y : Int) : Int = 55
fn ∈ (x : Int) (y : Int) : Int = 66
const le_glyph : Int = ≤ 0 0
const le_ascii : Int = <= 0 0
const ge_glyph : Int = ≥ 0 0
const ge_ascii : Int = >= 0 0
const ne_glyph : Int = ≠ 0 0
const ne_ascii : Int = /= 0 0
const and_glyph : Int = ∧ 0 0
const and_ascii : Int = /\ 0 0
const or_glyph : Int = ∨ 0 0
const or_ascii : Int = \/ 0 0
const member_glyph : Int = ∈ 0 0
"#;
    let tokens = Lexer::lex(source).expect("simultaneous source lexes");
    assert!(tokens
        .iter()
        .all(|(token, _)| !matches!(token, Token::Operator(_))));

    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_file(source)
        .expect("six distinct reserved definitions elaborate together");
    let ids = RESERVED
        .iter()
        .map(|(canonical, _)| env.globals[*canonical])
        .collect::<BTreeSet<_>>();
    assert_eq!(ids.len(), 6);
    for (name, expected) in [
        ("le_glyph", 11),
        ("le_ascii", 11),
        ("ge_glyph", 22),
        ("ge_ascii", 22),
        ("ne_glyph", 33),
        ("ne_ascii", 33),
        ("and_glyph", 44),
        ("and_ascii", 44),
        ("or_glyph", 55),
        ("or_ascii", 55),
        ("member_glyph", 66),
    ] {
        assert_eq!(eval_int(&env, name), expected, "wrong route for {name}");
    }
}

#[test]
fn reserved_names_preserve_qualified_selected_and_renamed_global_identity() {
    for (canonical, ascii) in RESERVED {
        let written = ascii.unwrap_or(canonical);
        let mut env = ElabEnv::new().expect("base environment");
        env.elaborate_file(&format!(
            "module Provider {{\n\
               pub fn {canonical} (x : Nat) (y : Nat) : Nat = x\n\
               infixr 5 {written}\n\
             }}\n\
             import Provider\n\
             fn qualified (a : Nat) (b : Nat) (c : Nat) : Nat = Provider.{written} a (Provider.{canonical} b c)\n\
             fn expected_qualified (a : Nat) (b : Nat) (c : Nat) : Nat = Provider.{canonical} a (Provider.{canonical} b c)\n\
             import Provider ({written})\n\
             fn selected (a : Nat) (b : Nat) (c : Nat) : Nat = a {canonical} b {written} c\n\
             fn expected_selected (a : Nat) (b : Nat) (c : Nat) : Nat = {canonical} a ({canonical} b c)\n\
             import Provider ({canonical} as <+>)\n\
             fn renamed (a : Nat) (b : Nat) (c : Nat) : Nat = a <+> b <+> c\n\
             fn expected_renamed (a : Nat) (b : Nat) (c : Nat) : Nat = <+> a (<+> b c)"
        ))
        .unwrap_or_else(|error| panic!("module route for {canonical:?}: {error:?}"));
        assert_eq!(body(&env, "qualified"), body(&env, "expected_qualified"));
        assert_eq!(body(&env, "selected"), body(&env, "expected_selected"));
        assert_eq!(body(&env, "renamed"), body(&env, "expected_renamed"));
        let provider = env.globals[&format!("Provider.{canonical}")];
        assert_eq!(
            env.fixities.get(&provider).map(|fixity| fixity.precedence),
            Some(5)
        );
    }
}

#[test]
fn rename_and_reexport_may_target_a_reserved_global_name() {
    for (canonical, _) in RESERVED {
        let mut env = ElabEnv::new().expect("base environment");
        env.elaborate_file(&format!(
            "module Provider {{\n\
               pub fn <+> (x : Nat) (y : Nat) : Nat = x\n\
               infixr 5 <+>\n\
             }}\n\
             module Facade {{ export Provider (<+> as {canonical}) }}\n\
             import Facade ({canonical})\n\
             fn use_facade (a : Nat) (b : Nat) (c : Nat) : Nat = a {canonical} b {canonical} c\n\
             fn expected (a : Nat) (b : Nat) (c : Nat) : Nat = {canonical} a ({canonical} b c)"
        ))
        .unwrap_or_else(|error| panic!("reserved rename target {canonical:?}: {error:?}"));
        assert_eq!(body(&env, "use_facade"), body(&env, "expected"));
        let provider = env.globals["Provider.<+>"];
        assert_eq!(
            env.fixities.get(&provider).map(|fixity| fixity.precedence),
            Some(5)
        );
    }
}

#[test]
fn reserved_fixities_keep_left_right_and_nonassociative_behavior() {
    let mut left = ElabEnv::new().expect("base environment");
    left.elaborate_file(
        "fn ≤ (x : Nat) (y : Nat) : Nat = x\n\
         infixl 5 ≤\n\
         fn actual (a : Nat) (b : Nat) (c : Nat) : Nat = a ≤ b ≤ c\n\
         fn expected (a : Nat) (b : Nat) (c : Nat) : Nat = ≤ (≤ a b) c",
    )
    .expect("left reserved fixity");
    assert_eq!(body(&left, "actual"), body(&left, "expected"));

    let mut right = ElabEnv::new().expect("base environment");
    right
        .elaborate_file(
            "fn ≥ (x : Nat) (y : Nat) : Nat = x\n\
             infixr 5 ≥\n\
             fn actual (a : Nat) (b : Nat) (c : Nat) : Nat = a ≥ b ≥ c\n\
             fn expected (a : Nat) (b : Nat) (c : Nat) : Nat = ≥ a (≥ b c)",
        )
        .expect("right reserved fixity");
    assert_eq!(body(&right, "actual"), body(&right, "expected"));

    let rejected = ElabEnv::new()
        .expect("base environment")
        .elaborate_file(
            "fn ≠ (x : Nat) (y : Nat) : Nat = x\n\
             infix 4 ≠\n\
             fn bad (a : Nat) (b : Nat) (c : Nat) : Nat = a ≠ b ≠ c",
        )
        .expect_err("non-associative reserved chain rejects");
    assert!(matches!(
        rejected,
        ElabError::NonAssociativeInfix { ref operator, .. } if operator == "≠"
    ));

    for expression in ["(a ≠ b) ≠ c", "a ≠ (b ≠ c)"] {
        ElabEnv::new()
            .expect("base environment")
            .elaborate_file(&format!(
                "fn ≠ (x : Nat) (y : Nat) : Nat = x\n\
                 infix 4 ≠\n\
                 fn grouped (a : Nat) (b : Nat) (c : Nat) : Nat = {expression}"
            ))
            .unwrap_or_else(|error| panic!("grouped {expression:?}: {error:?}"));
    }
}

#[test]
fn reserved_precedence_alias_collision_and_privacy_use_existing_diagnostics() {
    let positive = "fn ∧ (x : Nat) (y : Nat) : Nat = x\ninfixl 9 ∧";
    ElabEnv::new()
        .expect("base environment")
        .elaborate_file(positive)
        .expect("precedence 9 remains admitted");
    assert!(matches!(
        parse_decls("infixl 10 ∧"),
        Err(ElabError::InvalidFixityPrecedence { ref written, .. }) if written == "10"
    ));

    let duplicate = ElabEnv::new()
        .expect("base environment")
        .elaborate_file(
            "fn ≤ (x : Nat) (y : Nat) : Nat = x\n\
             fn <= (x : Nat) (y : Nat) : Nat = y",
        )
        .expect_err("alias pair cannot bind twice");
    assert!(matches!(
        duplicate,
        ElabError::DuplicateDefinition { ref name, .. } if name == "≤"
    ));

    let conflicting = ElabEnv::new()
        .expect("base environment")
        .elaborate_file(
            "fn ≤ (x : Nat) (y : Nat) : Nat = x\n\
             infixl 5 ≤\n\
             infixr 5 <=",
        )
        .expect_err("alias pair cannot carry conflicting fixity");
    assert!(matches!(
        conflicting,
        ElabError::ConflictingFixity { ref operator, .. } if operator == "≤"
    ));

    let mut public = ElabEnv::new().expect("base environment");
    public
        .elaborate_file(
            "module Public { pub fn ∈ (x : Nat) (y : Nat) : Nat = x }\n\
             import Public\n\
             const allowed : Nat = Public.∈ Zero Zero",
        )
        .expect("public reserved global remains accessible");

    let private = ElabEnv::new()
        .expect("base environment")
        .elaborate_file(
            "module Private { fn ∈ (x : Nat) (y : Nat) : Nat = x }\n\
             import Private\n\
             const forbidden : Nat = Private.∈ Zero Zero",
        )
        .expect_err("private reserved global must not escape");
    assert!(matches!(
        private,
        ElabError::UnboundName { ref name, .. } if name == "Private.∈"
    ));
}

#[test]
fn unbound_reserved_names_reach_resolution_without_implicit_meaning() {
    for (canonical, _) in RESERVED {
        let error = ElabEnv::new()
            .expect("base environment")
            .elaborate_decl(&format!(
                "const unbound : Nat = Zero {canonical} (Suc Zero)"
            ))
            .expect_err("A0 supplies no default binding");
        match error {
            ElabError::UnresolvedCon { name, .. } if name == canonical => {}
            other => panic!("unbound {canonical:?} must reach resolution, got {other:?}"),
        }
    }
}

#[test]
fn refused_aliases_keep_live_reserved_and_let_controls() {
    match Lexer::lex("!=").expect_err("bang is outside operator alphabet") {
        ElabError::ParseError { msg, span } => {
            assert_eq!(msg, "unexpected character '!'");
            assert_eq!((span.start, span.end), (0, 1));
        }
        other => panic!("wrong `!=` rejection: {other:?}"),
    }

    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_file(
        "fn ≠ (x : Nat) (y : Nat) : Nat = x\n\
         fn /= (x : Nat) (y : Nat) : Nat = x",
    )
    .expect_err("the admitted Ne aliases are one binding");
    ElabEnv::new()
        .expect("base environment")
        .elaborate_decl("const keep : Nat = let x = Zero in x")
        .expect("in remains the local-let separator");
    let in_error = parse_decls("fn in (x : Nat) : Nat = x")
        .expect_err("in is not a Member alias or a global name");
    assert!(matches!(
        in_error,
        ElabError::ParseError { ref msg, .. } if msg.contains("expected global name")
    ));
}

#[test]
fn reserved_names_do_not_widen_excluded_identifier_grammars() {
    for (source, marked, message) in [
        (
            "const p : Nat = let ≤ = Zero in Zero",
            "≤",
            "expected identifier, found Le",
        ),
        (
            "const p : Nat = let ≥ = Zero in Zero",
            "≥",
            "expected identifier, found Ge",
        ),
        (
            "const p : Nat = let ≠ = Zero in Zero",
            "≠",
            "expected identifier, found Ne",
        ),
        (
            "const p : Nat = let ∧ = Zero in Zero",
            "∧",
            "expected identifier, found And",
        ),
        (
            "const p : Nat = let ∨ = Zero in Zero",
            "∨",
            "expected identifier, found Or",
        ),
        (
            "const p : Nat = let ∈ = Zero in Zero",
            "∈",
            "expected identifier, found Member",
        ),
        (
            "def ≤ = Nat",
            "≤",
            "expected uppercase constructor name, found Le",
        ),
        (
            "data Marker = Only | ≥",
            "≥",
            "expected uppercase constructor name, found Ge",
        ),
        (
            "module ≠ {}",
            "≠",
            "expected uppercase constructor name, found Ne",
        ),
        (
            "import Provider as ∧",
            "∧",
            "expected identifier, found And",
        ),
        (
            "record Box { ∨ : Nat }",
            "∨",
            "expected identifier, found Or",
        ),
        (
            "proof ∈ for id (x : Nat) : Equal Nat (id x) x = Refl",
            "∈",
            "expected identifier, found Member",
        ),
    ] {
        assert_parse_error_at(source, marked, message);
    }

    for source in [
        "const p : Nat = let x = Zero in Zero",
        "def Alias = Nat",
        "data Marker = Only | Another",
        "module Provider {}",
        "import Provider as Alias",
        "record Box { value : Nat }",
        "proof id_self for id (x : Nat) : Equal Nat (id x) x = Refl",
    ] {
        parse_decls(source).unwrap_or_else(|error| panic!("control {source:?}: {error:?}"));
    }
}

#[test]
fn formatter_canonicalizes_alias_tokens_and_protects_comments_and_literals() {
    let source = r#"fn <= (x : Nat) (y : Nat) : Nat = x
fn >= (x : Nat) (y : Nat) : Nat = x
fn /= (x : Nat) (y : Nat) : Nat = x
fn /\ (x : Nat) (y : Nat) : Nat = x
fn \/ (x : Nat) (y : Nat) : Nat = x
fn ∈ (x : Nat) (y : Nat) : Nat = x
infixr 5 <=
infixl 4 /\
const le_result : Nat = Zero <= Zero
const ge_result : Nat = Zero >= Zero
const ne_result : Nat = Zero /= Zero
const and_result : Nat = Zero /\ Zero
const or_result : Nat = Zero \/ Zero
const member_result : Nat = Zero ∈ Zero
const literal : String = "<= /\\ \\/ /= >="
-- <= /\ \/ /= >= remain literal comment bytes
"#;
    let canonical = ken_elaborator::format::canonical_unicode(source);
    assert_ne!(
        canonical,
        source,
        "canonicalization must parse the fixture: {:?}",
        parse_decls(source)
    );
    for canonical_name in ["≤", "≥", "≠", "∧", "∨", "∈"] {
        assert!(
            canonical.contains(&format!("fn {canonical_name} ")),
            "formatter omitted {canonical_name:?}"
        );
    }
    assert!(canonical.contains("infixr 5 ≤"));
    assert!(canonical.contains("infixl 4 ∧"));
    assert!(canonical.contains("Zero ≤ Zero"));
    assert!(canonical.contains("Zero ≥ Zero"));
    assert!(canonical.contains("Zero ≠ Zero"));
    assert!(canonical.contains("Zero ∧ Zero"));
    assert!(canonical.contains("Zero ∨ Zero"));
    assert!(canonical.contains("Zero ∈ Zero"));
    assert!(canonical.contains(r#""<= /\\ \\/ /= >=""#));
    assert!(canonical.contains("-- <= /\\ \\/ /= >= remain literal comment bytes"));
    ElabEnv::new()
        .expect("base environment")
        .elaborate_file(&canonical)
        .expect("canonicalized source re-elaborates");

    let formatted = ken_elaborator::layout::format_ken(source).expect("layout formatting");
    ElabEnv::new()
        .expect("base environment")
        .elaborate_file(&formatted)
        .expect("layout-formatted source re-elaborates");
}

#[test]
fn generic_and_fixed_operator_paths_remain_distinct() {
    for operator in ["<", ">", "/", "%"] {
        common_fixture(operator, operator);
    }

    let parsed = parse_expr("a + b * c == d").expect("fixed arithmetic parses");
    assert!(matches!(
        parsed,
        Expr::EBinOp(BinOp::EqEq, left, _, _)
            if matches!(left.as_ref(), Expr::EBinOp(BinOp::Add, _, product, _)
                if matches!(product.as_ref(), Expr::EBinOp(BinOp::Mul, _, _, _)))
    ));
    for (source, expected) in [
        ("in", Token::KwIn),
        ("=", Token::Eq),
        (":", Token::Colon),
        ("::", Token::DoubleColon),
        (".", Token::Dot),
        ("↦", Token::MapsTo),
        ("λ", Token::Lambda),
        ("→", Token::Arrow),
        ("‖", Token::TruncBar),
        ("⊔", Token::Join),
        ("⊓", Token::Meet),
    ] {
        let tokens = Lexer::lex(source).expect("fixed token lexes");
        assert_eq!(tokens[0].0, expected, "fixed role changed for {source:?}");
    }
}
