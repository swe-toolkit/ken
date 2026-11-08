//! Canonical equality is preserved by normalization: spec/50-stdlib/66 §15.
//! Promise class: durable invariant. These package-owned controls do not run
//! or replace the independently authored conformance seed.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{Decl, KernelError};

const PACKAGE: &str = "Algorithm.SymbolicAlgebra.NatPolynomial";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn loaded_package() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], PACKAGE)
        .expect("NatPolynomial with normalization congruence roots-loads");
    env
}

const GENERIC_CLIENT: &str = r#"
import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, normalize, poly_eq, poly_eq_normalize)

theorem generic_normalize_congruence (e1 : Expr) (e2 : Expr)
    (h : IsTrue (poly_eq e1 e2))
  : IsTrue (poly_eq (normalize e1) (normalize e2)) =
  poly_eq_normalize e1 e2 h
"#;

/// MEASURED: provider load and package load produce identical sets of trusted
/// GlobalId identities; the new package-owned law is Transparent and an open
/// selective-import client consumes its one-premise result type.
/// CLAIMED: normalization preserves every checked canonical equality without
/// a new trust dependency or an extra premise. THE GAP: inherited provider
/// trust remains; the generic client's success depends on the kernel checking
/// the law body, not on the closed finite witness in the next test.
#[test]
fn generic_normalize_congruence_is_owned_checked_and_adds_no_trust() {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        "Core.Logic.Transport",
        "Data.Numeric.Nat.Arithmetic",
        "Data.Collections.Derived",
        "Core.Classes.LawfulClasses",
        "Data.Numeric.Nat.Order",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider} roots-loads: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], PACKAGE)
        .expect("normalization congruence kernel-checks");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "normalize congruence trusted GlobalIds: {} → {}",
        before.len(),
        after.len()
    );
    assert_eq!(after, before, "no new trusted GlobalId identity");
    let law = env.globals[&format!("{PACKAGE}.poly_eq_normalize")];
    assert!(owned.contains(&law), "NatPolynomial owns the new law");
    assert!(matches!(
        env.env.lookup(law),
        Some(Decl::Transparent { .. })
    ));
    env.elaborate_file(GENERIC_CLIENT)
        .expect("open client uses the exact one-premise public law");
}

const CLOSED_PREFIX: &str = r#"
import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, EVar, EAdd, normalize, poly_eq, poly_eq_normalize)

const x0 : Expr = EVar Zero
const x1 : Expr = EVar (Suc Zero)
const sum01 : Expr = EAdd x0 x1
const sum10 : Expr = EAdd x1 x0

theorem equal_sums_premise : IsTrue (poly_eq sum01 sum10) = Proved

theorem normalized_sums :
  IsTrue (poly_eq (normalize sum01) (normalize sum10)) =
  poly_eq_normalize sum01 sum10 equal_sums_premise

theorem normalized_sums_value :
  IsTrue (poly_eq (normalize sum01) (normalize sum10)) = Proved
"#;

const FALSE_PREMISE: &str = r#"
theorem distinct_vars_premise : IsTrue (poly_eq x0 x1) = Proved
"#;

fn refuses_at_own_theorem(env: &mut ElabEnv, name: &str, negative: &str) {
    match env.elaborate_file(negative) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        }) => {
            let theorem_start = negative
                .find(&format!("theorem {name}"))
                .expect("isolated refusal names its theorem");
            assert!(
                span.start >= theorem_start && span.start < span.end && span.end <= negative.len(),
                "{name} must refuse at its own proof: {span:?}"
            );
        }
        other => panic!("{name} must refuse with kernel TypeMismatch: {other:?}"),
    }
}

/// MEASURED: a commuted-sum premise, a normalization-congruence application
/// and the direct resulting decision all check. On a fresh accepted prefix,
/// `Proved` for distinct variables rejects with a kernel TypeMismatch whose
/// span is inside the false premise's own theorem, before an application.
/// CLAIMED: the law consumes a real `poly_eq` premise rather than permitting
/// any pair of expressions to normalize to canonically equal results.
/// THE GAP: the closed examples cannot prove the general law; the generic
/// kernel-checked proof and selective-import client establish that part.
#[test]
fn commuted_sums_accept_and_distinct_variables_refuse_at_their_premise() {
    loaded_package()
        .elaborate_file(CLOSED_PREFIX)
        .expect("commuted sums, law application and direct decision accept");
    let mut fresh = loaded_package();
    fresh
        .elaborate_file(CLOSED_PREFIX)
        .expect("entire accepted prefix checks before the isolated refusal");
    refuses_at_own_theorem(&mut fresh, "distinct_vars_premise", FALSE_PREMISE);
}
