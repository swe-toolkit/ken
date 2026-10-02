//! KERNEL-INT-DIV-MOD-NATIVE: fixed `/`/`%` operator and obligation controls.
//! Promise class: durable type-directed dispatch and proof-obligation invariants
//! (35 §3.1, 18a §5.2); lexer spellings/fixity are compatibility vectors.

use ken_elaborator::extract::{v2_extract, ProvKind};
use ken_elaborator::lexer::{Lexer, Token};
use ken_elaborator::parser::{parse_decls, parse_expr};
use ken_elaborator::{BinOp, ElabEnv, ElabError, Expr, ObligationKind};
use ken_kernel::env::Context;
use ken_kernel::{whnf, Term};

fn peel_function_body(body: &Term) -> &Term {
    let Term::Lam(_, inner) = body else {
        panic!("first argument must be a lambda")
    };
    let Term::Lam(_, inner) = inner.as_ref() else {
        panic!("second argument must be a lambda")
    };
    inner
}

#[test]
fn int_dispatch_emits_one_nonzero_divisor_obligation_per_site() {
    for (symbol, spelling) in [("div_int", "/"), ("mod_int", "%")] {
        let mut env = ElabEnv::new().expect("numeric prelude");
        let source = format!("fn f (a : Int) (b : Int) : Int = a {spelling} b");
        let result = env
            .elaborate_decl_v1(&source)
            .unwrap_or_else(|error| panic!("Int {spelling} must elaborate: {error:?}"));
        let body = env
            .env
            .transparent_body(result.def_id)
            .expect("checked definition")
            .1;
        let expected = Term::app(
            Term::app(Term::const_(env.globals[symbol], vec![]), Term::var(1)),
            Term::var(0),
        );
        assert_eq!(
            peel_function_body(body),
            &expected,
            "the infix Int operation must lower to the matching registered Op"
        );
        assert_eq!(
            result.obligations.len(),
            1,
            "one {spelling} site must generate exactly one nonzero obligation"
        );
        let obligation = &result.obligations[0];
        assert!(matches!(obligation.kind, ObligationKind::PartialPrim));
        assert!(
            env.is_open_hole(obligation.hole_id),
            "undischarged side condition remains visible"
        );

        let div_entry = env
            .numeric_env
            .classify_div(&Term::const_(env.globals["Int"], vec![]))
            .expect("Int division dispatch");
        assert!(
            env.env.transparent_body(div_entry.nonzero_id).is_some(),
            "NonZeroDivisor must be an ordinary definition"
        );
        assert!(
            !env.globals.contains_key("NonZeroDivisor"),
            "the internal predicate must not add a surface name"
        );
        assert!(
            !env.env.trusted_base().contains(&div_entry.nonzero_id),
            "the predicate must not be a trusted postulate"
        );
        let Term::Pi(first, rest) = &obligation.goal_closed else {
            panic!("a must be quantified")
        };
        let Term::Pi(second, goal) = rest.as_ref() else {
            panic!("b must be quantified")
        };
        let int = Term::const_(env.globals["Int"], vec![]);
        assert_eq!(first.as_ref(), &int);
        assert_eq!(second.as_ref(), &int);
        assert_eq!(
            goal.as_ref(),
            &Term::app(Term::const_(div_entry.nonzero_id, vec![]), Term::var(0)),
            "goal must depend on the divisor, not the numerator"
        );
        let expanded = whnf(&env.env, &Context::new(), goal);
        let Term::Pi(eq_zero, bottom) = expanded else {
            panic!("NonZeroDivisor b must reduce to Eq Int b 0 → Bottom")
        };
        assert_eq!(
            *eq_zero,
            Term::Eq(
                Box::new(int),
                Box::new(Term::var(0)),
                Box::new(Term::IntLit(0.into()))
            )
        );
        assert_eq!(*bottom, Term::const_(env.env.bottom_id(), vec![]));

        let extracted = v2_extract(&result);
        assert_eq!(extracted.obligations.len(), 1);
        assert!(matches!(
            extracted.obligations[0].provenance.kind,
            ProvKind::PartialPrim
        ));
    }
}

#[test]
fn nat_dispatch_rejects_both_spellings_without_generic_name_lookup() {
    for spelling in ["/", "%"] {
        let mut env = ElabEnv::new().expect("numeric prelude");
        let source = format!("fn f (a : Nat) (b : Nat) : Nat = a {spelling} b");
        match env.elaborate_decl_v1(&source) {
            Err(ElabError::TypeMismatch { reason, .. }) => {
                assert_eq!(reason, format!("'{spelling}' not supported on this type"))
            }
            other => panic!("Nat {spelling} must be type-mismatch, never generic: {other:?}"),
        }
    }
}

#[test]
fn fixed_division_spellings_refuse_user_declarations_like_plus() {
    let source = |spelling: &str| format!("fn {spelling} (x : Nat) (y : Nat) : Nat = x");
    let plus = parse_decls(&source("+")).expect_err("fn + is a fixed-token declaration error");
    let ElabError::ParseError {
        msg: plus_msg,
        span: plus_span,
    } = plus
    else {
        panic!("fn + must produce the baseline parser diagnostic class");
    };
    assert!(plus_msg.starts_with("expected global name, found "));
    for (spelling, expected) in [("/", Token::Slash), ("%", Token::Percent)] {
        assert_eq!(Lexer::lex(spelling).expect("lex")[0].0, expected);
        let error = parse_decls(&source(spelling))
            .expect_err("a fixed arithmetic spelling cannot be declared as a user function");
        match error {
            ElabError::ParseError { msg, span } => {
                assert!(msg.starts_with("expected global name, found "));
                assert_eq!((span.start, span.end), (plus_span.start, plus_span.end));
                assert_ne!(msg, plus_msg, "diagnostic names the exact token");
            }
            other => panic!("fn {spelling} must have the fn + diagnostic class: {other:?}"),
        }
    }
    // Only exact glyphs are fixed; compound spellings remain user operators.
    for spelling in ["<", ">", "<+>"] {
        assert!(
            matches!(Lexer::lex(spelling).unwrap()[0].0, Token::Operator(_)),
            "{spelling} must retain its generic operator path"
        );
    }
    assert_eq!(
        Lexer::lex("/\\").unwrap()[0].0,
        Token::And,
        "logical conjunction must not be reclassified as division"
    );
    assert_eq!(
        Lexer::lex("/=").unwrap()[0].0,
        Token::Ne,
        "inequality must not be reclassified as division"
    );
}

#[test]
fn division_and_remainder_bind_at_multiplication_precedence_on_both_parser_paths() {
    for spelling in ["/", "%"] {
        let add = parse_expr(&format!("a + b {spelling} c")).expect("parse addition");
        assert!(
            matches!(add, Expr::EBinOp(BinOp::Add, _, rhs, _)
            if matches!(rhs.as_ref(), Expr::EBinOp(BinOp::Div | BinOp::Mod, _, _, _))),
            "{spelling} must bind tighter than addition"
        );
        let multiply = parse_expr(&format!("a {spelling} b * c")).expect("parse multiplication");
        assert!(
            matches!(multiply, Expr::EBinOp(BinOp::Mul, lhs, _, _)
            if matches!(lhs.as_ref(), Expr::EBinOp(BinOp::Div | BinOp::Mod, _, _, _))),
            "{spelling} and * must associate left at level 7"
        );
    }

    // Declaring a distinct user operator activates the mixed infix parser;
    // its fixed arithmetic arms must retain the same precedence and fixity.
    let mut mixed_env = ElabEnv::new().expect("numeric prelude");
    mixed_env
        .elaborate_file(
            "fn <+> (x : Int) (y : Int) : Int = x\n\
        fn f (a : Int) (b : Int) (c : Int) : Int = a + b / c <+> a",
        )
        .expect("mixed infix declaration must elaborate");
    let f = mixed_env
        .env
        .transparent_body(mixed_env.globals["f"])
        .expect("mixed definition")
        .1;
    let Term::Lam(_, body) = f else {
        panic!("a parameter")
    };
    let Term::Lam(_, body) = body.as_ref() else {
        panic!("b parameter")
    };
    let Term::Lam(_, body) = body.as_ref() else {
        panic!("c parameter")
    };
    let user_call = Term::app(
        Term::app(Term::const_(mixed_env.globals["<+>"], vec![]), Term::var(0)),
        Term::var(2),
    );
    let division = Term::app(
        Term::app(
            Term::const_(mixed_env.globals["div_int"], vec![]),
            Term::var(1),
        ),
        user_call,
    );
    let expected = Term::app(
        Term::app(
            Term::const_(mixed_env.globals["add_int"], vec![]),
            Term::var(2),
        ),
        division,
    );
    assert_eq!(
        body.as_ref(),
        &expected,
        "default user fixity 9 still binds tighter than fixed / at level 7"
    );

    let formatted =
        ken_elaborator::layout::format_ken("fn f (a : Int) (b : Int) (c : Int) : Int = a + b / c")
            .expect("fixed division formats");
    let mut env = ElabEnv::new().unwrap();
    let result = env
        .elaborate_decl_v1(&formatted)
        .expect("formatted fixed division elaborates");
    assert_eq!(result.obligations.len(), 1);
}
