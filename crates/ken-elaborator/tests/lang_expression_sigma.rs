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
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages");
    let owned = env
        .elaborate_module_from_roots(&[root], "Core.Logic.Conjunction")
        .expect("the real conjunction package loads");
    env.elaborate_file(
        "import Core.Logic.Conjunction (And as CAnd, and_intro as c_intro)\n\
         theorem prelude_to_catalog (a : Omega) (b : Omega) (p : And a b) : CAnd a b = p\n\
         theorem catalog_to_prelude (a : Omega) (b : Omega) (p : CAnd a b) : And a b = p\n\
         theorem mixed_projection (a : Omega) (b : Omega) (x : a) (y : b) :\n\
           a = and_fst a b (c_intro a b x y)",
    )
    .expect("real catalog And converts in both directions and mixes with prelude projection");
    let catalog_and = env.globals["Core.Logic.Conjunction.and"];
    assert!(
        owned.contains(&catalog_and),
        "And must use the owned catalog definition"
    );
    assert!(
        !env.globals.contains_key("Core.Logic.Conjunction.And"),
        "renamed export must not mint another GlobalId"
    );
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

/// Promise class: durable invariant. MEASURED: a dependent Sigma in a
/// function result checks with its List-valued binder in the equality.
/// CLAIMED: expression-position Sigma works at the CAT-REL nested-body site.
/// THE GAP: truncation and grouping inside its bars are separate rows below.
#[test]
fn nested_list_sigma_checks_in_expression_body() {
    check_nested_list_sigma(
        "fn NestedSigma (k : Type) (w : List k -> Bool) : Type = \
         (vs : List k) × Equal Bool (w vs) True",
        false,
    );
}

/// Promise class: durable invariant. MEASURED: a checked Trunc wraps the
/// dependent Sigma body, without a new trusted-base entry. CLAIMED: Sigma
/// parses and elaborates in a nested expression position inside truncation.
/// THE GAP: grouping within the bars needs its own independent case.
#[test]
fn nested_list_sigma_checks_inside_truncation() {
    check_nested_list_sigma(
        "fn NestedSigma (k : Type) (w : List k -> Bool) : Prop = \
         ‖(vs : List k) × Equal Bool (w vs) True‖",
        true,
    );
}

/// Promise class: durable invariant. MEASURED: parenthesized Sigma survives
/// both grouping and truncation as a checked kernel Sigma under Trunc.
/// CLAIMED: grouped Sigma in truncation is not an incidental parser success.
/// THE GAP: this does not establish arbitrary general infix ×, which rejects.
#[test]
fn nested_grouped_list_sigma_checks_inside_truncation() {
    check_nested_list_sigma(
        "fn NestedSigma (k : Type) (w : List k -> Bool) : Prop = \
         ‖((vs : List k) × Equal Bool (w vs) True)‖",
        true,
    );
}

fn check_nested_list_sigma(source: &str, truncated: bool) {
    let mut env = env();
    let baseline = env.env.trusted_base();
    let id = env
        .elaborate_decl(source)
        .unwrap_or_else(|error| panic!("nested Sigma must check: {error:?}"));
    let (_, body) = env.env.transparent_body(id).expect("checked definition");
    let Term::Lam(_, outer) = body else {
        panic!("k must be bound")
    };
    let Term::Lam(_, inner) = *outer else {
        panic!("w must be bound")
    };
    let sigma = if truncated {
        let Term::Trunc(sigma) = *inner else {
            panic!("expected kernel Trunc around nested Sigma")
        };
        sigma
    } else {
        inner
    };
    let Term::Sigma(_, codomain) = *sigma else {
        panic!("expected kernel Sigma with dependent codomain")
    };
    let true_id = env.globals["True"];
    assert!(
        matches!(codomain.as_ref(), Term::App(_, argument) if matches!(argument.as_ref(), Term::Constructor { id, .. } if *id == true_id)),
        "Sigma codomain must retain the exact checked True constructor"
    );
    fn contains_nearest_var(term: &Term) -> bool {
        matches!(term, Term::Var(0)) || term.children().into_iter().any(contains_nearest_var)
    }
    assert!(
        contains_nearest_var(&codomain),
        "the equality must use the freshly bound vs in w vs"
    );
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
