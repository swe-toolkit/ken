//! LANG-EXPRESSION-SIGMA — expression-position dependent Sigma from 32 §3.
//! These controls test checked terms and diagnostic classes, not repository text.

use ken_elaborator::{layout::format_ken, parser::parse_expr, ElabEnv, ElabError, Expr};
use ken_kernel::{conv::convert_type, env::Context, Level, Term};

fn env() -> ElabEnv {
    ElabEnv::new().expect("checked prelude")
}

/// Promise class: durable invariant. MEASURED: a catalog-module And whose
/// expression body lowers to kernel Sigma and checks source-level conversions
/// in both directions against prelude And. CLAIMED: expression-position Sigma
/// denotes the already-landed conjunction, not a fresh type identity.
/// THE GAP: sort discrimination and parser selection are separate controls.
#[test]
fn catalog_and_expression_sigma_converts_with_prelude_both_ways() {
    let mut env = env();
    let baseline = env.env.trusted_base();
    let prelude_and = env.globals["And"];
    env.elaborate_file(
        "module SigmaCatalog {\n\
             pub fn And (a : Omega) (b : Omega) : Omega = (x : a) × b\n\
         }\n\
         theorem prelude_to_catalog (a : Omega) (b : Omega) (p : And a b) : SigmaCatalog.And a b = p\n\
         theorem catalog_to_prelude (a : Omega) (b : Omega) (p : SigmaCatalog.And a b) : And a b = p",
    )
    .expect("catalog And and two directions of conversion check without axioms");
    let catalog_and = env.globals["SigmaCatalog.And"];
    assert_ne!(catalog_and, prelude_and);
    let (_, catalog_body) = env
        .env
        .transparent_body(catalog_and)
        .expect("checked definition");
    let Term::Lam(_, first) = catalog_body else {
        panic!("catalog And needs its first lambda")
    };
    let Term::Lam(_, second) = *first else {
        panic!("catalog And needs its second lambda")
    };
    assert!(
        matches!(*second, Term::Sigma(..)),
        "And body must be the kernel Sigma"
    );
    let a = Term::var(1);
    let b = Term::var(0);
    let prelude = Term::app(
        Term::app(Term::const_(prelude_and, vec![]), a.clone()),
        b.clone(),
    );
    let catalog = Term::app(Term::app(Term::const_(catalog_and, vec![]), a), b);
    let context = Context {
        types: vec![Term::omega(Level::zero()), Term::omega(Level::zero())],
    };
    assert!(convert_type(&env.env, &context, &prelude, &catalog));
    assert!(convert_type(&env.env, &context, &catalog, &prelude));
    assert_eq!(env.env.trusted_base(), baseline);
}

/// Promise class: durable invariant. MEASURED: the same dependent Sigma with
/// a relevant Int first component checks at Type and is rejected at Omega
/// with TypeMismatch, not an incidental parse or resolution failure. CLAIMED:
/// a relevant carrier cannot be classified as a proposition. THE GAP: kernel
/// `sort_sigma` is trusted and is independently covered in sigma_sort.rs.
#[test]
fn relevant_first_component_checks_at_type_but_not_omega() {
    let mut env = env();
    let id = env
        .elaborate_decl("fn RelevantAtType (p : Int -> Omega) : Type = (x : Int) × p x")
        .expect("relevant-first Sigma is a Type and codomain sees x");
    let (_, checked) = env
        .env
        .transparent_body(id)
        .expect("checked relevant definition");
    let Term::Lam(_, sigma) = checked else {
        panic!("expected bound predicate")
    };
    let Term::Sigma(_, codomain) = *sigma else {
        panic!("expected kernel Sigma")
    };
    assert!(
        matches!(*codomain, Term::App(_, argument) if *argument == Term::Var(0)),
        "Sigma codomain must use its freshly bound x, not an outer binding"
    );
    let error = env
        .elaborate_decl("fn RelevantAtOmega (p : Int -> Omega) : Omega = (x : Int) × p x")
        .expect_err("relevant-first Sigma cannot be classified at Omega");
    assert!(
        matches!(&error, ElabError::KernelRejected {
            error: ken_kernel::error::KernelError::TypeMismatch { expected, found }, ..
        } if matches!(**expected, Term::Omega(_)) && matches!(**found, Term::Type(_))),
        "wrong relevant-first rejection: {error:?}"
    );
}

/// Promise class: durable invariant. MEASURED: formatting a Sigma body
/// preserves both the dependent binding and the checked kernel Sigma.
/// CLAIMED: the new AST node remains round-trippable through kenfmt.
/// THE GAP: this one fixture does not replace the general formatter corpus.
#[test]
fn formatted_expression_sigma_keeps_its_checked_body() {
    let source = "fn SigmaProduct (p : Int -> Omega) : Type = (x : Int) × p x";
    let formatted = format_ken(source).expect("Sigma body formats");
    assert_eq!(format_ken(&formatted).expect("fixed point"), formatted);
    let mut original_env = env();
    let original = original_env
        .elaborate_decl(source)
        .expect("original Sigma body");
    let mut formatted_env = env();
    let formatted_id = formatted_env
        .elaborate_decl(&formatted)
        .expect("formatted Sigma body");
    assert_eq!(
        original_env
            .env
            .transparent_body(original)
            .expect("original checked")
            .1,
        formatted_env
            .env
            .transparent_body(formatted_id)
            .expect("formatted checked")
            .1,
    );
}

/// Promise class: durable invariant. MEASURED: only a complete dependent
/// binder followed by × becomes ESigma; prior arrows, ascriptions, application
/// grouping and right-association retain their production. CLAIMED: general
/// infix × is not introduced and no previously valid expression changes parse.
/// THE GAP: examples guard named boundaries, not every possible expression.
#[test]
fn sigma_is_a_binder_form_not_general_expression_infix() {
    assert!(matches!(parse_expr("(x : a) × b"), Ok(Expr::ESigma(..))));
    assert!(matches!(parse_expr("(x : a) -> b"), Ok(Expr::EPi(..))));
    assert!(matches!(parse_expr("(x : a)"), Ok(Expr::EAsc(..))));
    let grouped = parse_expr("f ((x : a) × b)").expect("Sigma as grouped application argument");
    assert!(matches!(grouped, Expr::EApp(_, argument, _) if matches!(*argument, Expr::ESigma(..))));
    let nested = parse_expr("(x : a) × (y : b) × c").expect("right-associative dependent Sigma");
    assert!(
        matches!(nested, Expr::ESigma(_, _, codomain, _) if matches!(*codomain, Expr::ESigma(..)))
    );
    assert!(matches!(
        parse_expr("(x : a) × b : Omega"),
        Ok(Expr::EAsc(..))
    ));
    for invalid in ["a × b", "f (x : a) × b", "(x : a) ×"] {
        assert!(
            matches!(parse_expr(invalid), Err(ElabError::ParseError { .. })),
            "unexpected general infix Sigma admission for {invalid}"
        );
    }
}
