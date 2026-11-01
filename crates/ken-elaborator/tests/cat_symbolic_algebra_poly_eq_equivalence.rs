//! Canonical polynomial equality is reflexive, symmetric and transitive: §11.
//! Promise class: durable invariant. Generic clients pin all three public
//! types; equal-form closed decisions distinguish true from false claims.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{Decl as SourceDecl, ElabEnv, ElabError};
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
        .expect("§11 NatPolynomial package roots-loads");
    env
}

const GENERIC_CLIENT: &str = r#"
import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, poly_eq, poly_eq_refl, poly_eq_sym, poly_eq_trans)

theorem client_refl (a : Expr) : IsTrue (poly_eq a a) = poly_eq_refl a

theorem client_sym (a : Expr) (b : Expr) (h : IsTrue (poly_eq a b))
  : IsTrue (poly_eq b a) = poly_eq_sym a b h

theorem client_trans
    (a : Expr) (b : Expr) (c : Expr)
    (hab : IsTrue (poly_eq a b)) (hbc : IsTrue (poly_eq b c))
  : IsTrue (poly_eq a c) = poly_eq_trans a b c hab hbc

theorem client_double_sym (a : Expr) (b : Expr) (h : IsTrue (poly_eq a b))
  : IsTrue (poly_eq a b) = poly_eq_sym b a (poly_eq_sym a b h)
"#;

/// MEASURED: the package owns all three Transparent generic §11 laws, and
/// roots-loading from the already present providers preserves the exact
/// trusted GlobalId identity set. The independent selective-import client
/// consumes reflexivity, symmetry, transitivity and double symmetry at open
/// expressions. The Horner reflexivity helper remains unimportable.
/// CLAIMED: the public laws are checked and reusable without a new trust entry
/// or public proof scaffolding. THE GAP: inherited provider trust persists;
/// closed examples below cannot replace the generic checked proof bodies.
#[test]
fn generic_equivalence_laws_are_checked_public_and_zero_trust() {
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
        .expect("§11 generic laws kernel-check");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!("§11 trusted GlobalIds: {} → {}", before.len(), after.len());
    assert_eq!(after, before, "§11 adds no trusted GlobalId identity");
    for public in ["poly_eq_refl", "poly_eq_sym", "poly_eq_trans"] {
        let id = env.globals[&format!("{PACKAGE}.{public}")];
        assert!(owned.contains(&id), "{public} is owned by NatPolynomial");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{public} has a checked body"
        );
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("open clients consume the precise §11 contracts");
    match env.elaborate_file(&format!("import {PACKAGE} (poly_same_refl)")) {
        Err(ElabError::UnboundName { name, .. }) => {
            assert_eq!(name, format!("{PACKAGE}.poly_same_refl"));
        }
        other => panic!("Horner reflexivity must remain private: {other:?}"),
    }
}

const CLOSED_PREFIX: &str = r#"
import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, EConst, EVar, EAdd, EMul, poly_eq,
   poly_eq_refl, poly_eq_sym, poly_eq_trans)

const one : Nat = Suc Zero
const two : Nat = Suc one
const x0 : Expr = EVar Zero
const x1 : Expr = EVar one
const x0_sum : Expr = EAdd x0 x0
const two_times_x0 : Expr = EMul (EConst two) x0

theorem forward : IsTrue (poly_eq x0_sum two_times_x0) = Proved
theorem back : IsTrue (poly_eq two_times_x0 x0_sum) =
  poly_eq_sym x0_sum two_times_x0 forward
theorem round_trip : IsTrue (poly_eq x0_sum x0_sum) =
  poly_eq_trans x0_sum two_times_x0 x0_sum forward back
theorem round_trip_back : IsTrue (poly_eq two_times_x0 two_times_x0) =
  poly_eq_trans two_times_x0 x0_sum two_times_x0 back forward
theorem refl_x1 : IsTrue (poly_eq x1 x1) = poly_eq_refl x1
"#;

const ACCEPTED_CONTROL: &str = r#"
theorem equal_form_control : IsTrue (poly_eq x0_sum two_times_x0) = Proved
"#;

const REFUSED_CLAIM: &str = r#"
theorem equal_form_refusal : IsTrue (poly_eq x0_sum x1) = Proved
"#;

/// MEASURED: the accepted prefix constructs a true different-syntax pair,
/// symmetrizes it, composes it both ways and instantiates reflexivity. Two
/// fresh package environments load that same prefix. The same-form `Proved`
/// control accepts, while `Proved` for x0_sum versus x1 refuses with a kernel
/// TypeMismatch at its own theorem. CLAIMED: the equality decision separates
/// semantic equivalence from a false claim independently of the law client's
/// generic proof. THE GAP: these closed inputs are not a conformance seed or
/// an official conformance-runner execution.
#[test]
fn closed_equal_form_control_accepts_and_false_claim_refuses() {
    let mut accepting = loaded_package();
    accepting
        .elaborate_file(CLOSED_PREFIX)
        .expect("common prefix and three law applications accept");
    accepting
        .elaborate_file(ACCEPTED_CONTROL)
        .expect("true poly_eq decision accepts Proved");

    let mut rejecting = loaded_package();
    rejecting
        .elaborate_file(CLOSED_PREFIX)
        .expect("fresh accepted prefix precedes refusal");
    let parsed = parse_decls(REFUSED_CLAIM).expect("refusal parses");
    let [SourceDecl::TheoremDecl {
        name,
        params,
        span: claim_span,
        ..
    }] = parsed.as_slice()
    else {
        panic!("exactly one isolated refused theorem");
    };
    assert_eq!(name, "equal_form_refusal");
    assert!(
        params.is_empty(),
        "refusal is closed over the common prefix"
    );
    match rejecting.elaborate_file(REFUSED_CLAIM) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        }) if claim_span.start <= span.start
            && span.start < span.end
            && span.end <= claim_span.end => {}
        other => panic!("false poly_eq must refuse at its own Proved: {other:?}"),
    }
}
