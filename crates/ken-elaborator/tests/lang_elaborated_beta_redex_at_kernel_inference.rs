//! Applied λ redexes from a reduced branch goal must check at kernel handoffs.
//! Promise class: durable invariant. Spec 34 §3.5 and 39 §2.1.

use std::path::PathBuf;

use ken_elaborator::{lexer::Lexer, literate::extract_ken_md, ElabEnv};
use ken_kernel::{Decl, GlobalId, Term};

const MATRIX: &str = "Algorithm.LinearAlgebra.NatMatrix";
const MAP_EXPRESSION: &str = "map (Vec Nat n) Nat p (dot_with n row) columns";
const LAMBDA_EXPRESSION: &str = "map (Vec Nat n) Nat p (λcolumn. dot n row column) columns";

fn catalog_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn matrix_source(lambda: bool) -> String {
    let named =
        std::fs::read_to_string(catalog_root().join("Algorithm/LinearAlgebra/NatMatrix.ken.md"))
            .expect("landed NatMatrix fixture");
    let code = extract_ken_md(&named)
        .expect("NatMatrix source extracts")
        .source;
    if !lambda {
        return code;
    }
    // Splice the argument's token sequence, not a line or a whitespace
    // spelling. Prose and formatting edits cannot change the fixture.
    let tokens = Lexer::lex(&code).expect("NatMatrix lexes");
    let mut map_tokens = Lexer::lex(MAP_EXPRESSION).expect("map expression lexes");
    map_tokens.pop(); // EOF is not part of the expression.
    let sites: Vec<_> = tokens
        .windows(map_tokens.len())
        .filter(|window| {
            window
                .iter()
                .zip(&map_tokens)
                .all(|((actual, _), (expected, _))| actual == expected)
        })
        .map(|window| (window[0].1.start, window.last().unwrap().1.end))
        .collect();
    let [(start, end)] = sites.as_slice() else {
        panic!("fixture needs one map application with the named function argument");
    };
    format!("{}{}{}", &code[..*start], LAMBDA_EXPRESSION, &code[*end..])
}

fn checked_left_head_core(ty: &Term, body: &Term, equality: GlobalId, row_product: GlobalId) {
    let mut conclusion = ty;
    while let Term::Pi(_, codomain) = conclusion {
        conclusion = codomain;
    }
    while let Term::App(function, _) = conclusion {
        conclusion = function;
    }
    assert!(
        matches!(conclusion, Term::Const { id, .. } if *id == equality),
        "the kernel-recorded goal is headed by the checked Equal identity"
    );
    let mut pending = vec![ty];
    let mut mentions_row_product = false;
    while let Some(term) = pending.pop() {
        mentions_row_product |= matches!(term, Term::Const { id, .. } if *id == row_product);
        pending.extend(term.children());
    }
    assert!(
        mentions_row_product,
        "the checked goal names the row-product identity"
    );

    let mut found_elim = false;
    let mut pending = vec![body];
    while let Some(term) = pending.pop() {
        assert!(
            !matches!(term, Term::App(head, _) if matches!(head.as_ref(), Term::Lam(..))),
            "a kernel-checked theorem body must not contain an applied λ"
        );
        found_elim |= matches!(term, Term::Elim { .. });
        pending.extend(term.children());
    }
    assert!(
        found_elim,
        "the checked proof must retain its dependent match"
    );
}

fn check_matrix(lambda: bool) {
    let mut env = ElabEnv::new().expect("prelude");
    let roots = [catalog_root()];
    // These are exactly the imports in NatMatrix, loaded from the real
    // catalog to keep the in-memory replacement scoped to its own module.
    for provider in [
        "Data.Vector.Vector",
        "Core.Logic.Transport",
        "Data.Numeric.Nat.Arithmetic",
    ] {
        env.elaborate_module_from_roots(&roots, provider)
            .unwrap_or_else(|error| panic!("provider {provider}: {error:?}"));
    }
    let trust = env.env.trusted_base();
    env.elaborate_file(&matrix_source(lambda))
        .unwrap_or_else(|error| panic!("{MATRIX} with lambda={lambda} must check: {error:?}"));
    let left_head = env.globals["left_head"];
    let Some(Decl::Transparent { ty, body, .. }) = env.env.lookup(left_head) else {
        panic!("left_head must have a checked transparent proof body");
    };
    checked_left_head_core(
        ty,
        body,
        env.globals["Equal"],
        env.globals["row_product_columns"],
    );
    assert_eq!(env.env.trusted_base(), trust, "no added trust");
}

/// MEASURED: the λ-spelled NatMatrix checks and the kernel records an Eq
/// theorem over row_product_columns with a transparent dependent-match proof
/// body containing no applied λ. CLAIMED: a δ-exposed β redex cannot break
/// the checked theorem at kernel inference. THE GAP: the final proof does not
/// expose every intermediate handoff; QA mutates both producer/consumer routes.
#[test]
fn lambda_argument_checks_left_head_without_new_trust() {
    check_matrix(true);
}

/// MEASURED: the named-helper package checks and records a transparent proof
/// with a dependent match and no applied λ. CLAIMED: contracting elaborated
/// β redexes does not regress the original package. THE GAP: adjacent suites
/// and the per-file catalog census test other consumer families separately.
#[test]
fn named_argument_still_checks_left_head_without_new_trust() {
    check_matrix(false);
}
