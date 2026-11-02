//! LET-4 — sequential multi-binding local `let` groups.

use ken_elaborator::{
    error::ElabError,
    parser::parse_expr,
    resolve::{resolve_expr_standalone, RExpr, RType},
    ElabEnv, Expr,
};
use ken_interp::eval::{eval, EvalStore, EvalVal};
use ken_kernel::{GlobalId, Term};

fn erase_spans(mut debug: String) -> String {
    const PREFIX: &str = "Span { start: ";
    while let Some(start) = debug.find(PREFIX) {
        let Some(relative_end) = debug[start..].find(" }") else {
            break;
        };
        debug.replace_range(start..start + relative_end + 2, "Span");
    }
    debug
}

fn resolved_shape(source: &str) -> String {
    let parsed = parse_expr(source).expect("fixture parses");
    let resolved = resolve_expr_standalone(&parsed).expect("fixture resolves");
    erase_spans(format!("{resolved:?}"))
}

/// Split a fully applied registered `bind E R A B action k` into
/// (action, k); None for any other term, including the pre-`36 §2.4`
/// `Let` shape.
fn bind_parts(term: &Term, bind: GlobalId) -> Option<(&Term, &Term)> {
    let mut args = Vec::new();
    let mut head = term;
    while let Term::App(function, arg) = head {
        args.push(arg.as_ref());
        head = function.as_ref();
    }
    args.reverse();
    match (head, args.as_slice()) {
        (Term::Const { id, .. }, [_, _, _, _, action, k]) if *id == bind => Some((*action, *k)),
        _ => None,
    }
}

fn assert_group_len(source: &str, expected: usize) {
    let parsed = parse_expr(source).expect("boundary fixture parses");
    let Expr::ELet(bindings, _, _) = parsed else {
        panic!("expected a let expression, got {parsed:?}");
    };
    assert_eq!(bindings.len(), expected);
    for binding in bindings {
        assert_eq!(binding.name_span.start, binding.span.start);
        assert!(binding.span.end >= binding.value.span().end);
        if let Some(annotation) = &binding.annotation {
            assert_eq!(binding.annotation_span.as_ref(), Some(annotation.span()));
        }
    }
}

#[test]
fn ac1_binding_separators_are_context_local() {
    assert_group_len("let x = Zero; y = x in y", 2);
    assert_group_len("let x = match Zero { Zero ↦ Zero; }; y = x in y", 2);
    assert_group_len("let x = let inner = Zero in inner; y = x in y", 2);
    assert_group_len("let f = Nat → Nat; x = f in x", 2);

    let arm_body = parse_expr("match Zero { Zero ↦ let x = Zero; y = x in y; Suc n ↦ n; }")
        .expect("a grouped let in an arm must not consume arm separators");
    let Expr::EMatch { arms, .. } = arm_body else {
        panic!("expected match fixture");
    };
    assert_eq!(arms.len(), 2);
    assert!(matches!(arms[0].body, Expr::ELet(ref bindings, _, _) if bindings.len() == 2));
}

#[test]
fn ac1_ac2_rejections_are_specific() {
    let trailing = parse_expr("let x = Zero; in x").unwrap_err();
    assert!(matches!(
        trailing,
        ElabError::ParseError { ref msg, .. }
            if msg == "trailing ';' is not allowed before 'in' in a let group"
    ));

    let duplicate = parse_expr("let x = Zero; x = x in x").unwrap_err();
    assert!(matches!(
        duplicate,
        ElabError::ParseError { ref msg, .. }
            if msg == "duplicate local binding 'x' in let group"
    ));

    let comma = parse_expr("let x = Zero, y = x in y").unwrap_err();
    assert!(matches!(comma, ElabError::ParseError { .. }));

    for (source, missing) in [
        ("let x = x; y = Zero in y", "x"),
        ("let x = y; y = Zero in x", "y"),
    ] {
        let mut env = ElabEnv::new().unwrap();
        let error = env
            .elaborate_expr("ac1_ac2_rejections_are_specific", source)
            .unwrap_err();
        assert!(matches!(
            error,
            ElabError::UnresolvedCon { ref name, .. } if name == missing
        ));
    }
}

#[test]
fn ac2_ac3_group_and_nested_forms_resolve_and_lower_identically() {
    let grouped = "let a : Type = Nat; x : a = Zero in x";
    let nested = "let a : Type = Nat in let x : a = Zero in x";
    assert_eq!(resolved_shape(grouped), resolved_shape(nested));

    let parsed = parse_expr(grouped).unwrap();
    let resolved = resolve_expr_standalone(&parsed).unwrap();
    let RExpr::RLet(a, _, _, body, _) = resolved else {
        panic!("outer binding must lower to RLet");
    };
    let RExpr::RLet(x, Some(ty), rhs, body, _) = *body else {
        panic!("second binding must lower to nested RLet");
    };
    assert_eq!(a, "a");
    assert_eq!(x, "x");
    assert!(matches!(ty, RType::RVarTy(0, ref name, _) if name == "a"));
    assert!(matches!(*rhs, RExpr::RCon(ref name, _) if name == "Zero"));
    assert!(matches!(*body, RExpr::RVar(0, ref name, _) if name == "x"));

    let mut grouped_env = ElabEnv::new().unwrap();
    let mut nested_env = ElabEnv::new().unwrap();
    let (grouped_core, grouped_ty) = grouped_env
        .elaborate_expr("ac2_ac3_group_and_nested_forms", grouped)
        .unwrap();
    let (nested_core, nested_ty) = nested_env
        .elaborate_expr("ac2_ac3_group_and_nested_forms", nested)
        .unwrap();
    assert_eq!(grouped_core, nested_core);
    assert_eq!(grouped_ty, nested_ty);
}

/// Promise class: durable invariant. Grouped and nested pure bindings have
/// identical core and strict value order; performed bindings produce checked
/// binds in source order, anchored by the first single-binding action.
#[test]
fn ac4_strict_evaluation_and_effectful_lowering_keep_source_order() {
    let grouped = "let x : Nat = Zero; y : Nat = Suc x in Suc y";
    let nested = "let x : Nat = Zero in let y : Nat = Suc x in Suc y";
    let mut grouped_env = ElabEnv::new().unwrap();
    let mut nested_env = ElabEnv::new().unwrap();
    let (grouped_core, _) = grouped_env
        .elaborate_expr("ac4_strict_evaluation_source_order", grouped)
        .unwrap();
    let (nested_core, _) = nested_env
        .elaborate_expr("ac4_strict_evaluation_source_order", nested)
        .unwrap();
    assert_eq!(grouped_core, nested_core);
    assert!(matches!(
        grouped_core,
        Term::Let { ref body, .. } if matches!(body.as_ref(), Term::Let { .. })
    ));
    let grouped_value = eval(&[], &grouped_core, &grouped_env.env, &mut EvalStore::new());
    let suc = grouped_env.globals["Suc"];
    let zero = grouped_env.globals["Zero"];
    assert!(matches!(
        grouped_value,
        EvalVal::Ctor { id, ref args, .. }
            if id == suc
                && matches!(args.as_slice(), [EvalVal::Ctor { id, args, .. }]
                    if *id == suc
                        && matches!(args.as_slice(), [EvalVal::Ctor { id, args, .. }]
                            if *id == zero && args.is_empty()))
    ));

    let effect_grouped =
        "let first = print_line \"first\"; second = print_line \"second\" in second";
    let effect_nested =
        "let first = print_line \"first\" in let second = print_line \"second\" in second";
    let mut effect_grouped_env = ElabEnv::new().unwrap();
    let mut effect_nested_env = ElabEnv::new().unwrap();
    let (effect_grouped_core, effect_grouped_ty) = effect_grouped_env
        .elaborate_expr("ac4_effectful_lowering_source_order", effect_grouped)
        .unwrap();
    let (effect_nested_core, effect_nested_ty) = effect_nested_env
        .elaborate_expr("ac4_effectful_lowering_source_order", effect_nested)
        .unwrap();
    assert_eq!(effect_grouped_core, effect_nested_core);
    assert_eq!(effect_grouped_ty, effect_nested_ty);
    // `36 §2.4`: both effects are performed, outer first. The anchor is a
    // single-binding let in a fresh environment, whose one action is
    // unambiguously `print_line "first"`.
    let bind = effect_grouped_env.prelude_env.bind_id;
    let mut anchor_env = ElabEnv::new().unwrap();
    let (anchor_core, _) = anchor_env
        .elaborate_expr(
            "ac4_effectful_lowering_source_order",
            "let only = print_line \"first\" in only",
        )
        .unwrap();
    let (first_anchor, _) = bind_parts(&anchor_core, anchor_env.prelude_env.bind_id)
        .expect("a performed single let lowers to bind");
    let (first, outer_k) = bind_parts(&effect_grouped_core, bind)
        .expect("first effect must be the outer bind's action");
    assert_eq!(
        first, first_anchor,
        "the outer bind performs the first binding"
    );
    let Term::Lam(_, outer_body) = outer_k else {
        panic!("the outer bind's continuation must bind `first`");
    };
    let (second, _) =
        bind_parts(outer_body, bind).expect("second effect must be the inner bind's action");
    assert_ne!(
        first, second,
        "distinguishable effects must retain their order"
    );
}

#[test]
fn ac2_separate_nested_body_may_shadow_a_group_name() {
    let source = "let x : Nat = Zero; y : Nat = x in let x : Nat = Suc y in x";
    let mut env = ElabEnv::new().unwrap();
    env.elaborate_expr("ac2_separate_nested_body_shadow", source)
        .expect("ordinary shadowing in a separately nested body remains legal");
}
