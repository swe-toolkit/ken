//! Expression powers: spec/50-stdlib/66-symbolic-algebra.md §16 (pending).
//! Promise class: durable invariant. The two-times decision is a closed
//! behavioral discriminator, not a substitute for either generic checked law.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::ElabEnv;
use ken_kernel::Decl;

const PACKAGE: &str = "Algorithm.SymbolicAlgebra.NatPolynomial";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn loaded_package() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], PACKAGE)
        .expect("NatPolynomial expression powers roots-load");
    env
}

const GENERIC_CLIENT: &str = r#"
import Data.Numeric.Nat.Arithmetic (add)

import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, deriv, expr_pow, poly_eq, deriv_expr_pow, expr_pow_add, EConst, EMul)

theorem arbitrary_derivative (v : Nat) (e : Expr) (n : Nat)
    : IsTrue (poly_eq (deriv v (expr_pow e (Suc n)))
        (EMul (EMul (EConst (Suc n)) (expr_pow e n)) (deriv v e))) =
  deriv_expr_pow v e n

theorem arbitrary_exponent_addition (e : Expr) (m : Nat) (n : Nat)
    : IsTrue (poly_eq (expr_pow e (add m n))
        (EMul (expr_pow e m) (expr_pow e n))) =
  expr_pow_add e m n
"#;

/// MEASURED: five existing providers roots-load first; the exact trusted
/// GlobalId identity set is unchanged by this package. The package owns the
/// operation and both public checked laws with Transparent bodies, and a
/// selective-import client applies the laws at arbitrary terms and exponents.
/// CLAIMED: expression power and its two generic laws add no local trust and
/// can be imported with their stated unrestricted types. THE GAP: existing
/// provider trust remains inherited; the closed decision below independently
/// checks one nontrivial value, not the universal law.
#[test]
fn generic_expression_power_laws_are_owned_checked_and_add_no_trust() {
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
        .expect("expression power and its two laws kernel-check");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "expression power trusted GlobalIds: {} → {}",
        before.len(),
        after.len()
    );
    assert_eq!(after, before, "no new trusted GlobalId identity");
    for name in ["expr_pow", "deriv_expr_pow", "expr_pow_add"] {
        let id = env.globals[&format!("{PACKAGE}.{name}")];
        assert!(owned.contains(&id), "NatPolynomial owns {name}");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{name} has a checked body"
        );
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("open client uses both public power laws through selective imports");
}

const TWO_TIMES_CONTROL: &str = r#"
import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, EVar, EConst, EMul, deriv, expr_pow, poly_eq)

theorem two_times_x : IsTrue (poly_eq
    (deriv Zero (expr_pow (EVar Zero) (Suc (Suc Zero))))
    (EMul (EMul (EConst (Suc (Suc Zero))) (expr_pow (EVar Zero) (Suc Zero)))
      (deriv Zero (EVar Zero)))) = Proved
"#;

/// MEASURED: the actual package provider checks before a standalone closed
/// `Proved` decision for derivative of x squared with coefficient two; this
/// decision has no application to deriv_expr_pow that could mask a bad value.
/// CLAIMED: the power rule yields the correct nonzero coefficient on this
/// instance. THE GAP: one closed instance cannot replace the generic proof;
/// a definition mutation is first blocked at the body-coupled provider proof.
#[test]
fn two_times_x_closed_decision_checks_independently_of_law_application() {
    loaded_package()
        .elaborate_file(TWO_TIMES_CONTROL)
        .expect("closed two-times Proved power-rule instance checks");
}
