//! Expression-power multiplication laws: spec/50-stdlib/66-symbolic-algebra.md §17.
//! Promise class: durable invariant. Open, arbitrary exponents and expression
//! arguments pin the checked types, independently of the closed seed values.

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

// A provider load alone does not bind IsTrue in this separate client.
const GENERIC_CLIENT: &str = r#"
import Core.Classes.LawfulClasses (IsTrue)
import Data.Numeric.Nat.Arithmetic (mul)

import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, EMul, poly_eq, expr_pow, expr_pow_mul, expr_pow_emul)

theorem pp_generic (e : Expr) (m : Nat) (n : Nat)
    : IsTrue (poly_eq (expr_pow e (mul m n)) (expr_pow (expr_pow e m) n)) =
  expr_pow_mul e m n

theorem prod_generic (a : Expr) (b : Expr) (n : Nat)
    : IsTrue (poly_eq (expr_pow (EMul a b) n)
        (EMul (expr_pow a n) (expr_pow b n))) =
  expr_pow_emul a b n
"#;

/// MEASURED: the same elaboration environment roots-loads five existing
/// providers, then the package; its exact trusted GlobalId set is unchanged.
/// The package owns both named laws as Transparent declarations, and a
/// selective-import client applies both at arbitrary expressions and Nat
/// exponents. CLAIMED: these two public law types are fully checked, have no
/// ordering or distinctness premise, and introduce no local trust.
/// THE GAP: inherited provider trust remains; this open application checks
/// the signatures, while the package's checked proof bodies establish them.
#[test]
fn expression_power_multiplication_laws_are_owned_checked_and_open() {
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
        .expect("both expression-power multiplication laws kernel-check");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "expression-power multiplication trusted GlobalIds: {} → {}",
        before.len(),
        after.len()
    );
    assert_eq!(after, before, "no new trusted GlobalId identity");
    for name in ["expr_pow_mul", "expr_pow_emul"] {
        let id = env.globals[&format!("{PACKAGE}.{name}")];
        assert!(owned.contains(&id), "NatPolynomial owns {name}");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{name} has a checked body"
        );
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("both arbitrary power-multiplication laws check through selective imports");
}
