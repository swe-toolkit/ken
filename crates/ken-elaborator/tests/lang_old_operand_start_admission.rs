//! Acceptance for `LANG-OLD-OPERAND-START-ADMISSION`.
//!
//! Architect ruling `evt_5x6r1tsgvjwsc`: `old` consumes exactly one
//! `application_atom`. Head-only expression forms therefore require grouping
//! when they are its operand.

use ken_elaborator::parser::parse_expr;
use ken_elaborator::{ElabError, Expr};

const OLD_OPERAND_MARKER: &str = "`old` requires an application_atom operand";
const GROUPING_MARKER: &str = "group it as `old (...)`";

struct Case {
    name: &'static str,
    bare: &'static str,
    grouped: &'static str,
    leading: &'static str,
}

const CASES: [Case; 2] = [
    Case {
        name: "if expression",
        bare: "old if c then a else b",
        grouped: "old (if c then a else b)",
        leading: "if",
    },
    Case {
        name: "attached-proof selector",
        bare: "old proof p for s",
        grouped: "old (proof p for s)",
        leading: "proof",
    },
];

fn rejection(source: &str) -> (String, usize) {
    match parse_expr(source).expect_err("bare head-only old operand must reject") {
        ElabError::ParseError { msg, span } => (msg, span.start),
        other => panic!("expected a parse error for {source:?}, got {other:?}"),
    }
}

#[test]
fn bare_head_only_operands_reject_locally_and_grouped_forms_parse() {
    for case in CASES {
        let (message, start) = rejection(case.bare);
        assert!(
            message.contains(OLD_OPERAND_MARKER),
            "{} rejection came from the wrong parser site: {message}",
            case.name
        );
        assert!(
            message.contains(GROUPING_MARKER),
            "{} rejection must direct the author to grouping: {message}",
            case.name
        );
        assert_eq!(
            start,
            case.bare
                .find(case.leading)
                .expect("fixture has leading token"),
            "{} must reject at its leading token",
            case.name
        );
        assert!(
            matches!(parse_expr(case.grouped), Ok(Expr::EOld(_, _))),
            "grouped {} must remain an old operand",
            case.name
        );
    }
}

#[test]
fn existing_old_atom_and_projection_shapes_are_preserved() {
    match parse_expr("old (f x)").expect("grouped application operand must parse") {
        Expr::EOld(argument, _) => assert!(matches!(*argument, Expr::EApp(_, _, _))),
        other => panic!("expected old of one grouped application atom, got {other:?}"),
    }

    match parse_expr("(old x).field").expect("projection from grouped old must parse") {
        Expr::EProj(subject, field, _) => {
            assert_eq!(field, "field");
            assert!(matches!(*subject, Expr::EOld(_, _)));
        }
        other => panic!("expected projection from old, got {other:?}"),
    }

    for source in ["old x", "old C", "old 7"] {
        assert!(
            matches!(parse_expr(source), Ok(Expr::EOld(_, _))),
            "ordinary atomic operand must remain admitted: {source}"
        );
    }

    match parse_expr("keep old x").expect("old must remain an application argument") {
        Expr::EApp(_, argument, _) => assert!(matches!(*argument, Expr::EOld(_, _))),
        other => panic!("ExprAtomForm::Old must remain an argument form, got {other:?}"),
    }
}
