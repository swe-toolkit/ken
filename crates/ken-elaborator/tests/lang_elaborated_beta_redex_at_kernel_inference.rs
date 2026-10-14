//! Applied λ redexes from a reduced branch goal must check at kernel handoffs.
//! Promise class: durable invariant. Spec 34 §3.5 and 39 §2.1.

use std::path::PathBuf;

use ken_elaborator::{literate::extract_ken_md, ElabEnv};

const MATRIX: &str = "Algorithm.LinearAlgebra.NatMatrix";
const NAMED_ARGUMENT: &str = "  map (Vec Nat n) Nat p (dot_with n row) columns";
const LAMBDA_ARGUMENT: &str = "  map (Vec Nat n) Nat p (λcolumn. dot n row column) columns";

fn catalog_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn matrix_source(lambda: bool) -> String {
    let named =
        std::fs::read_to_string(catalog_root().join("Algorithm/LinearAlgebra/NatMatrix.ken.md"))
            .expect("landed NatMatrix fixture");
    assert_eq!(
        named.matches(NAMED_ARGUMENT).count(),
        1,
        "the single function argument in the fixture must remain identifiable"
    );
    if lambda {
        named.replacen(NAMED_ARGUMENT, LAMBDA_ARGUMENT, 1)
    } else {
        named
    }
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
    let source = matrix_source(lambda);
    let extracted = extract_ken_md(&source).expect("NatMatrix source extracts");
    env.elaborate_file(&extracted.source)
        .unwrap_or_else(|error| panic!("{MATRIX} with lambda={lambda} must check: {error:?}"));
    assert!(
        env.globals.contains_key("left_head"),
        "the theorem was checked"
    );
    assert_eq!(env.env.trusted_base(), trust, "no added trust");
}

/// MEASURED: the complete NatMatrix source with the one `map` argument
/// replaced by an equivalent λ checks including `left_head`. CLAIMED: a
/// δ-exposed β redex cannot break a checked proof at kernel inference. THE
/// GAP: this one producer witness cannot prove all terms lack such redexes;
/// the exhaustive traversal and independent QA mutations close its route.
#[test]
fn lambda_argument_checks_left_head_without_new_trust() {
    check_matrix(true);
}

/// MEASURED: the otherwise identical named-helper package checks. CLAIMED:
/// contracting elaborated β redexes does not regress the original package.
/// THE GAP: adjacent dependent-match suites and the per-file catalog census
/// test other consumers of the reduced-goal producer separately.
#[test]
fn named_argument_still_checks_left_head_without_new_trust() {
    check_matrix(false);
}
