//! KERNEL-INT-DIV-MOD-NATIVE: fixed `/`/`%` operator and obligation controls.
//! Promise class: durable type-directed dispatch and proof-obligation invariants
//! (35 §3.1, 18a §5.2); lexer spellings/fixity are compatibility vectors.

use std::collections::HashSet;

use ken_elaborator::extract::{v2_extract, ProvKind};
use ken_elaborator::lexer::{Lexer, Token};
use ken_elaborator::parser::{parse_decls, parse_expr};
use ken_elaborator::{BinOp, ElabEnv, ElabError, Expr, ObligationKind};
use ken_kernel::env::Context;
use ken_kernel::{convert_type, whnf, Term};

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
            peel_function_body(&body),
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

fn assert_operation_obligations(spelling: &str, case: &str, source: &str, expected: usize) {
    let mut env = ElabEnv::new().expect("numeric prelude");
    let before = env.env.trusted_base();
    let result = env
        .elaborate_decl_v1(source)
        .unwrap_or_else(|error| panic!("{spelling} {case} must elaborate: {error:?}"));
    assert_eq!(
        result.obligations.len(),
        expected,
        "{spelling} {case}: operation-site obligation count"
    );
    let new_trust = env
        .env
        .trusted_base()
        .into_iter()
        .filter(|id| !before.contains(id))
        .collect::<Vec<_>>();
    assert_eq!(
        new_trust.len(),
        expected,
        "{spelling} {case}: an unreported postulate is not zero obligations"
    );
    if expected == 1 {
        let obligation = &result.obligations[0];
        assert!(matches!(obligation.kind, ObligationKind::PartialPrim));
        assert_eq!(new_trust[0], obligation.hole_id);
        assert!(env.is_open_hole(obligation.hole_id));
        let extracted = v2_extract(&result);
        assert_eq!(extracted.obligations.len(), 1);
        assert!(matches!(
            extracted.obligations[0].provenance.kind,
            ProvKind::PartialPrim
        ));
    }
}

#[test]
fn refined_divisor_keeps_the_open_operation_hole_without_an_unchecked_hypothesis() {
    for spelling in ["/", "%"] {
        let mut env = ElabEnv::new().unwrap();
        let before = env.env.trusted_base();
        let source = format!(
            "fn f (n : Int) (d : {{z : Int | Not (Equal Int z 0)}}) : Int = n {spelling} d"
        );
        let result = env
            .elaborate_decl_v1(&source)
            .expect("refined-domain function");
        let [obligation] = result.obligations.as_slice() else {
            panic!("{spelling}: carrier-only refinement must not discharge division")
        };
        assert!(matches!(obligation.kind, ObligationKind::PartialPrim));
        assert!(env.is_open_hole(obligation.hole_id));
        let added = env
            .env
            .trusted_base()
            .into_iter()
            .filter(|id| !before.contains(id))
            .collect::<Vec<_>>();
        assert_eq!(added, [obligation.hole_id]);
        let int = Term::const_(env.globals["Int"], vec![]);
        let Term::Pi(n, rest) = &obligation.goal_closed else {
            panic!("{spelling}: n binder missing")
        };
        let Term::Pi(d, goal) = rest.as_ref() else {
            panic!("{spelling}: d binder missing")
        };
        assert_eq!((n.as_ref(), d.as_ref()), (&int, &int));
        let nonzero = env.numeric_env.classify_div(&int).unwrap().nonzero_id;
        assert_eq!(
            goal.as_ref(),
            &Term::app(Term::const_(nonzero, vec![]), Term::var(0)),
            "{spelling}: an unestablished refinement must not appear as a Pi hypothesis"
        );
    }
}

#[test]
fn refined_divisor_with_requires_closes_only_over_the_real_proof_premise() {
    for spelling in ["/", "%"] {
        let mut env = ElabEnv::new().unwrap();
        let before = env.env.trusted_base();
        let result = env
            .elaborate_decl_v1(&format!(
                "fn f (n : Int) (d : {{z : Int | Not (Equal Int z 0)}}) : Int \
                 requires Equal Int n 5 = n {spelling} d"
            ))
            .expect("refined argument with a non-direct requires");
        let [obligation] = result.obligations.as_slice() else {
            panic!("{spelling}: an unrelated proof premise must not discharge the divisor")
        };
        assert!(matches!(obligation.kind, ObligationKind::PartialPrim));
        assert_eq!(
            env.env
                .trusted_base()
                .into_iter()
                .filter(|id| !before.contains(id))
                .collect::<Vec<_>>(),
            [obligation.hole_id]
        );
        let int = Term::const_(env.globals["Int"], vec![]);
        let Term::Pi(n, rest) = &obligation.goal_closed else {
            panic!("n binder")
        };
        let Term::Pi(d, rest) = rest.as_ref() else {
            panic!("d carrier binder")
        };
        assert_eq!((n.as_ref(), d.as_ref()), (&int, &int));
        let Term::Pi(real_req, goal) = rest.as_ref() else {
            panic!("the actual requires premise must be present")
        };
        let mut ctx = Context::new();
        ctx.push(int.clone());
        ctx.push(int.clone());
        assert!(convert_type(
            &env.env,
            &ctx,
            real_req,
            &Term::Eq(
                Box::new(int.clone()),
                Box::new(Term::var(1)),
                Box::new(Term::IntLit(5.into()))
            )
        ));
        let nonzero = env.numeric_env.classify_div(&int).unwrap().nonzero_id;
        assert_eq!(
            goal.as_ref(),
            &Term::app(Term::const_(nonzero, vec![]), Term::var(1)),
            "{spelling}: only the proved requires may extend the goal telescope"
        );
    }
}

#[test]
fn refined_divisor_caller_can_pass_zero_but_the_callee_keeps_its_hole() {
    for spelling in ["/", "%"] {
        let mut env = ElabEnv::new().unwrap();
        let before = env.env.trusted_base();
        let f = env
            .elaborate_decl_v1(&format!(
                "fn f (n : Int) (d : {{z : Int | Not (Equal Int z 0)}}) : Int = n {spelling} d"
            ))
            .expect("callee with erased refined domain");
        let [hole] = f.obligations.as_slice() else {
            panic!("{spelling}: f must retain its operation-site side condition")
        };
        assert!(matches!(hole.kind, ObligationKind::PartialPrim));
        assert!(env.is_open_hole(hole.hole_id));
        let after_f = env.env.trusted_base();
        assert_eq!(
            after_f
                .iter()
                .filter(|id| !before.contains(id))
                .copied()
                .collect::<Vec<_>>(),
            [hole.hole_id]
        );
        let g = env
            .elaborate_decl_v1("fn g (u : Int) : Int = f 1 0")
            .expect("current carrier encoding accepts the zero argument");
        assert!(
            g.obligations.is_empty(),
            "a caller does not currently establish the erased refinement"
        );
        assert_eq!(
            env.env.trusted_base(),
            after_f,
            "no unreported argument-introduction hole can justify callee recognition"
        );
    }
}

#[test]
fn requires_call_site_emits_one_hole_without_reopening_body_division() {
    for spelling in ["/", "%"] {
        let mut env = ElabEnv::new().unwrap();
        let before = env.env.trusted_base().into_iter().collect::<HashSet<_>>();
        let f = env
            .elaborate_decl_v1(&format!(
                "fn f (n : Int) (d : Int) : Int requires Not (Equal Int d 0) = n {spelling} d"
            ))
            .expect("the callee body uses its requires binder");
        assert!(f.obligations.is_empty());
        assert_eq!(
            env.env.trusted_base().into_iter().collect::<HashSet<_>>(),
            before
        );

        let g = env
            .elaborate_decl_v1("fn g (u : Int) : Int = f 1 0")
            .expect("the caller is admitted with an open call-site proof");
        let [obligation] = g.obligations.as_slice() else {
            panic!("the call emits exactly one Requires obligation")
        };
        assert!(matches!(obligation.kind, ObligationKind::Requires));
        let after = env.env.trusted_base().into_iter().collect::<HashSet<_>>();
        assert_eq!(
            after.difference(&before).copied().collect::<HashSet<_>>(),
            [obligation.hole_id].into_iter().collect(),
            "the call-site hole is the only trusted-base addition"
        );
    }
}

#[test]
fn directly_required_nonzero_divisor_needs_no_new_operation_obligation_or_hole() {
    // Start with % so an unconditional-push mutation is observed at both
    // operation identities across the two independent zero-obligation tests.
    for spelling in ["%", "/"] {
        assert_operation_obligations(
            spelling,
            "direct requires",
            &format!(
                "fn f (n : Int) (d : Int) : Int requires Not (Equal Int d 0) = n {spelling} d"
            ),
            0,
        );
    }
}

#[test]
fn possibly_zero_non_direct_and_unrelated_cases_each_emit_one_obligation() {
    for spelling in ["/", "%"] {
        for (case, source) in [
            (
                "possibly zero",
                format!("fn f (n : Int) (d : Int) : Int = n {spelling} d"),
            ),
            (
                "non-direct requires",
                format!(
                    "fn f (n : Int) (d : Int) : Int requires Equal Int d 5 = n {spelling} d"
                ),
            ),
            (
                "unrelated requires",
                format!(
                    "fn f (n : Int) (d : Int) (e : Int) : Int requires Not (Equal Int e 0) = n {spelling} d"
                ),
            ),
        ] {
            assert_operation_obligations(spelling, case, &source, 1);
        }
    }
}

#[test]
fn non_direct_requires_stays_in_closed_divisor_goal_at_its_binder_depth() {
    for spelling in ["/", "%"] {
        let mut env = ElabEnv::new().unwrap();
        let source =
            format!("fn f (n : Int) (d : Int) : Int requires Equal Int d 5 = n {spelling} d");
        let result = env.elaborate_decl_v1(&source).expect("non-direct requires");
        let [obligation] = result.obligations.as_slice() else {
            panic!("non-direct {spelling} requires must retain one body obligation")
        };
        let int = Term::const_(env.globals["Int"], vec![]);
        let Term::Pi(n, after_n) = &obligation.goal_closed else {
            panic!("{spelling}: n binder")
        };
        let Term::Pi(d, after_d) = after_n.as_ref() else {
            panic!("{spelling}: d binder")
        };
        assert_eq!((n.as_ref(), d.as_ref()), (&int, &int));
        let Term::Pi(hypothesis, goal) = after_d.as_ref() else {
            panic!("{spelling}: requires hypothesis at parameter depth")
        };
        let mut params = Context::new();
        params.push(int.clone());
        params.push(int.clone());
        let expected_hypothesis = Term::Eq(
            Box::new(int.clone()),
            Box::new(Term::var(0)),
            Box::new(Term::IntLit(5.into())),
        );
        assert!(
            convert_type(&env.env, &params, hypothesis, &expected_hypothesis),
            "{spelling}: closed Γ must contain the real Eq Int d 5 hypothesis"
        );
        params.push(*hypothesis.clone());
        let nonzero_id = env
            .numeric_env
            .classify_div(&int)
            .expect("Int dispatch")
            .nonzero_id;
        assert_eq!(
            goal.as_ref(),
            &Term::app(Term::const_(nonzero_id, vec![]), Term::var(1)),
            "{spelling}: h must shift the goal's divisor, never change it"
        );
    }
}

#[test]
fn requires_body_and_ensures_obligations_are_all_reported_in_order() {
    let mut env = ElabEnv::new().unwrap();
    let before = env.env.trusted_base();
    let result = env
        .elaborate_decl_v1(
            "fn f (n : Int) (d : Int) : Int \
             requires Equal Int (n / d) 0 \
             ensures Equal Int result n = n / d",
        )
        .expect("all three obligation producers");
    assert_eq!(result.obligations.len(), 3);
    let kinds = result
        .obligations
        .iter()
        .map(|o| &o.kind)
        .collect::<Vec<_>>();
    assert!(matches!(
        kinds.as_slice(),
        [
            ObligationKind::PartialPrim,
            ObligationKind::PartialPrim,
            ObligationKind::Ensures
        ]
    ));
    for (index, obligation) in result.obligations.iter().enumerate() {
        assert_eq!(obligation.id as usize, index);
        assert!(env.is_open_hole(obligation.hole_id));
    }
    let after = env.env.trusted_base();
    let added = after
        .into_iter()
        .filter(|id| !before.contains(id))
        .collect::<Vec<_>>();
    assert_eq!(
        added.len(),
        3,
        "every newly trusted hole must be visible in the result"
    );
    assert_eq!(v2_extract(&result).obligations.len(), 3);
}

#[test]
fn ensures_expression_operation_precedes_its_generated_ensures_hole() {
    for spelling in ["/", "%"] {
        let mut env = ElabEnv::new().unwrap();
        let before = env.env.trusted_base();
        let source =
            format!("fn f (n : Int) (d : Int) : Int ensures Equal Int result (n {spelling} d) = n");
        let result = env
            .elaborate_decl_v1(&source)
            .expect("an operation in the ensures expression must elaborate");
        let [operation, ensures] = result.obligations.as_slice() else {
            panic!("{spelling}: ensures-expression operation plus generated ensures hole")
        };
        assert!(matches!(operation.kind, ObligationKind::PartialPrim));
        assert!(matches!(ensures.kind, ObligationKind::Ensures));
        assert_eq!((operation.id, ensures.id), (0, 1));
        assert!(env.is_open_hole(operation.hole_id));
        assert!(env.is_open_hole(ensures.hole_id));
        let added = env
            .env
            .trusted_base()
            .into_iter()
            .filter(|id| !before.contains(id))
            .collect::<Vec<_>>();
        assert_eq!(added.len(), 2);
        assert!(added.contains(&operation.hole_id));
        assert!(added.contains(&ensures.hole_id));
        let extracted = v2_extract(&result);
        assert!(matches!(
            extracted.obligations[0].provenance.kind,
            ProvKind::PartialPrim
        ));
        assert!(matches!(
            extracted.obligations[1].provenance.kind,
            ProvKind::Ensures { index: 1 }
        ));
        let body = env
            .env
            .transparent_body(result.def_id)
            .expect("f is checked")
            .1;
        assert_eq!(
            peel_function_body(&body),
            &Term::var(1),
            "{spelling}: the operation is in the ensures proposition, not the body"
        );
        let int = Term::const_(env.globals["Int"], vec![]);
        let Term::Pi(n, rest) = &operation.goal_closed else {
            panic!("operation goal: n binder")
        };
        let Term::Pi(d, rest) = rest.as_ref() else {
            panic!("operation goal: d binder")
        };
        let Term::Pi(result_ty, goal) = rest.as_ref() else {
            panic!("operation goal: ensures-local result binder")
        };
        assert_eq!(
            (n.as_ref(), d.as_ref(), result_ty.as_ref()),
            (&int, &int, &int)
        );
        let nonzero = env.numeric_env.classify_div(&int).unwrap().nonzero_id;
        assert_eq!(
            goal.as_ref(),
            &Term::app(Term::const_(nonzero, vec![]), Term::var(1)),
            "{spelling}: the ensures-local goal must depend on d"
        );
    }
}

#[test]
fn requires_and_ensures_still_accept_propositions_and_reject_bool() {
    for (clause, prop, expected_obligations) in [
        ("requires", "Equal Int n n", 0),
        ("ensures", "Equal Int result n", 1),
    ] {
        let mut env = ElabEnv::new().unwrap();
        let valid = format!("fn f (n : Int) : Int {clause} {prop} = n");
        let result = env
            .elaborate_decl_v1(&valid)
            .unwrap_or_else(|error| panic!("Ω₀ {clause} must elaborate: {error:?}"));
        assert_eq!(result.obligations.len(), expected_obligations);
        let invalid = format!("fn bad (n : Int) : Int {clause} True = n");
        assert!(
            matches!(
                env.elaborate_decl_v1(&invalid),
                Err(ElabError::TypeMismatch { reason, .. })
                    if reason == "spec proposition must have type Ω, found non-proposition"
            ),
            "{clause}: the Bool constructor must fail the proposition gate"
        );
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
        assert_eq!(Lexer::lex(spelling).expect("lex")[0].0, expected);
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
