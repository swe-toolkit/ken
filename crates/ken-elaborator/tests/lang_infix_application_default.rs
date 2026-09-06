//! `LANG-INFIX-APPLICATION-DEFAULT` acceptance.
//!
//! Spec source: `spec/30-surface/32-grammar.md` §3 and §6.
//! Promise class: durable invariants. Declared fixity may replace the default
//! for explicitly declared operators, but must preserve the default `infixl 9`
//! shape for operators without a declaration.

use ken_elaborator::parser::parse_expr;
use ken_elaborator::{BinOp, ElabEnv, Expr};

fn assert_var(expr: &Expr, expected: &str) {
    assert!(
        matches!(expr, Expr::EVar(name, _) if name == expected),
        "expected variable {expected:?}, got {expr:?}"
    );
}

fn infix_application<'a>(expr: &'a Expr, expected_operator: &str) -> (&'a Expr, &'a Expr) {
    let Expr::EApp(operator_and_lhs, rhs, _) = expr else {
        panic!("expected infix application, got {expr:?}");
    };
    let Expr::EApp(operator, lhs, _) = operator_and_lhs.as_ref() else {
        panic!("expected saturated infix operator application, got {expr:?}");
    };
    assert_var(operator, expected_operator);
    (lhs, rhs)
}

/// MEASURED: infix and prefix consumers elaborate through one environment to
/// byte-for-byte equal kernel terms. CLAIMED: default infix syntax is notation
/// for ordinary prefix application. THE GAP: equal evaluated results could hide
/// a distinct elaboration, so this compares the admitted terms themselves.
#[test]
fn infix_elaborates_to_the_same_term_as_prefix_application() {
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_decl("fn <+> (a : Nat) (b : Nat) : Nat = a")
        .expect("symbolic operator definition");
    let infix = env
        .elaborate_decl("fn infixUse (a : Nat) (b : Nat) : Nat = a <+> b")
        .expect("infix consumer");
    let prefix = env
        .elaborate_decl("fn prefixUse (a : Nat) (b : Nat) : Nat = <+> a b")
        .expect("prefix consumer");

    let (_, infix_body) = env.env.transparent_body(infix).expect("infix body");
    let (_, prefix_body) = env.env.transparent_body(prefix).expect("prefix body");
    assert_eq!(infix_body, prefix_body);
}

/// MEASURED: the parsed tree for a three-operand chain nests the first operator
/// application under the second. CLAIMED: default fixity is `infixl 9`.
/// THE GAP: a value oracle could make both associations equal, so this inspects
/// only the syntax tree and uses no evaluator.
#[test]
fn default_symbolic_infix_is_structurally_left_associative() {
    let parsed = parse_expr("a <+> b <+> c").expect("infix chain parses");
    let (lhs, rhs) = infix_application(&parsed, "<+>");
    assert_var(rhs, "c");
    let (first, second) = infix_application(lhs, "<+>");
    assert_var(first, "a");
    assert_var(second, "b");
}

/// MEASURED: one parsed tree places `<+>` below `*` and both below `+` in the
/// nesting direction corresponding to tighter binding. CLAIMED: level 9 binds
/// tighter than the existing levels 7 and 6. THE GAP: arithmetic values could
/// agree accidentally, so the operands are distinct names and the tree itself
/// is the oracle.
#[test]
fn default_symbolic_infix_binds_tighter_than_star_and_plus() {
    let parsed = parse_expr("a + b <+> c * d").expect("mixed precedence parses");
    let Expr::EBinOp(BinOp::Add, add_lhs, product, _) = parsed else {
        panic!("expected additive root, got {parsed:?}");
    };
    assert_var(&add_lhs, "a");

    let Expr::EBinOp(BinOp::Mul, product_lhs, product_rhs, _) = product.as_ref() else {
        panic!("expected multiplicative right operand, got {product:?}");
    };
    let (infix_lhs, infix_rhs) = infix_application(product_lhs, "<+>");
    assert_var(infix_lhs, "b");
    assert_var(infix_rhs, "c");
    assert_var(product_rhs, "d");
}

/// MEASURED: both operands of an infix application retain their complete
/// ordinary-application subtrees. CLAIMED: ordinary application still binds
/// tighter than default level 9. THE GAP: merely accepting the source would not
/// distinguish reassociation, so this checks all four leaves structurally.
#[test]
fn ordinary_application_still_binds_tighter_than_default_infix() {
    let parsed = parse_expr("f a <+> g b").expect("application/infix mixture parses");
    let (lhs, rhs) = infix_application(&parsed, "<+>");

    let Expr::EApp(lhs_head, lhs_arg, _) = lhs else {
        panic!("expected left application operand, got {lhs:?}");
    };
    assert_var(lhs_head, "f");
    assert_var(lhs_arg, "a");

    let Expr::EApp(rhs_head, rhs_arg, _) = rhs else {
        panic!("expected right application operand, got {rhs:?}");
    };
    assert_var(rhs_head, "g");
    assert_var(rhs_arg, "b");
}
