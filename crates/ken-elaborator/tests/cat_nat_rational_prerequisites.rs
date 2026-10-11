//! Contract: spec/50-stdlib/64-rational.md §4. Promise class: durable
//! invariant. Public natural algebra and order laws are independently usable
//! by generic clients without adding a trusted GlobalId.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::Decl;

const ARITHMETIC: &str = "Data.Numeric.Nat.Arithmetic";
const ORDER: &str = "Data.Numeric.Nat.Order";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

/// MEASURED: both real packages root-load, their nine exports are transparent
/// owned identities, and a separate generic client applies all nine. CLAIMED:
/// §4's nine obligations are checked Nat-provider exports. THE GAP: the
/// positive `Suc` condition is measured by the independent zero-factor
/// rejection below; the client does not claim to exercise all closed inputs.
#[test]
fn nine_provider_laws_serve_a_generic_importer_without_trust_growth() {
    let mut env = ElabEnv::new().expect("base environment");
    for provider in ["Core.Logic.Transport", "Core.Classes.LawfulClasses"] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider} must load: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let arithmetic_owned = env
        .elaborate_module_from_roots(&[root()], ARITHMETIC)
        .expect("Arithmetic must root-load checked proofs");
    let order_owned = env
        .elaborate_module_from_roots(&[root()], ORDER)
        .expect("Order must root-load checked proofs");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "trusted_base() count: before={}, after={}",
        before.len(),
        after.len()
    );
    assert_eq!(after, before, "Nat prerequisites must add no trusted IDs");

    for (module, name, owned) in [
        (ARITHMETIC, "mul::zero_l", &arithmetic_owned),
        (ARITHMETIC, "mul::one_l", &arithmetic_owned),
        (ARITHMETIC, "mul::one_r", &arithmetic_owned),
        (ARITHMETIC, "mul::comm", &arithmetic_owned),
        (ARITHMETIC, "mul::assoc", &arithmetic_owned),
        (ARITHMETIC, "mul_add_distrib_l", &arithmetic_owned),
        (ARITHMETIC, "mul::cancel_r", &arithmetic_owned),
        (ORDER, "leq_nat_mul_right_mono", &order_owned),
        (ORDER, "leq_nat_mul_right_cancel", &order_owned),
    ] {
        let qualified = format!("{module}.{name}");
        let id = *env
            .globals
            .get(&qualified)
            .unwrap_or_else(|| panic!("{qualified} must be registered"));
        assert!(
            owned.contains(&id),
            "{qualified} must retain its provider ID"
        );
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{qualified} must be a checked, transparent proof"
        );
    }

    env.elaborate_file(
        r#"import Data.Numeric.Nat.Arithmetic (add, mul, mul_add_distrib_l)
import Data.Numeric.Nat.Order
  (leq_nat, leq_nat_mul_right_mono, leq_nat_mul_right_cancel)

theorem client_zero_l (n : Nat) : Equal Nat (mul Zero n) Zero =
  (proof zero_l for mul) n

theorem client_one_l (n : Nat) : Equal Nat (mul (Suc Zero) n) n =
  (proof one_l for mul) n

theorem client_one_r (n : Nat) : Equal Nat (mul n (Suc Zero)) n =
  (proof one_r for mul) n

theorem client_comm (a : Nat) (b : Nat) : Equal Nat (mul a b) (mul b a) =
  (proof comm for mul) a b

theorem client_assoc (a : Nat) (b : Nat) (c : Nat) :
  Equal Nat (mul a (mul b c)) (mul (mul a b) c) =
  (proof assoc for mul) a b c

theorem client_distrib (a : Nat) (b : Nat) (c : Nat) :
  Equal Nat (mul (add a b) c) (add (mul a c) (mul b c)) =
  mul_add_distrib_l a b c

theorem client_cancel (a : Nat) (b : Nat) (k : Nat)
  (h : Equal Nat (mul a (Suc k)) (mul b (Suc k))) :
  Equal Nat a b = (proof cancel_r for mul) a b k h

theorem client_mono (a : Nat) (b : Nat) (k : Nat)
  (ab : Equal Bool (leq_nat a b) True) :
  Equal Bool (leq_nat (mul a (Suc k)) (mul b (Suc k))) True =
  leq_nat_mul_right_mono a b k ab

theorem client_order_cancel (a : Nat) (b : Nat) (k : Nat)
  (scaled : Equal Bool
    (leq_nat (mul a (Suc k)) (mul b (Suc k))) True) :
  Equal Bool (leq_nat a b) True =
  leq_nat_mul_right_cancel a b k scaled"#,
    )
    .expect("all nine checked provider laws must serve generic imported goals");
}

/// Promise class: durable invariant. Positive scaling preserves both the
/// forward comparison and its recovery at factor two, whereas the reversed
/// comparison is observably False on the same distinct input pair.
#[test]
fn positive_scaling_has_distinct_true_and_false_closed_outcomes() {
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_module_from_roots(&[root()], ORDER)
        .expect("Order must load the canonical multiplication and comparator");
    env.elaborate_file(
        r#"import Data.Numeric.Nat.Arithmetic (mul)
import Data.Numeric.Nat.Order
  (leq_nat, leq_nat_mul_right_mono, leq_nat_mul_right_cancel)
theorem base_order : Equal Bool (leq_nat (Suc Zero) (Suc (Suc Zero))) True = Proved
theorem scaled_order : Equal Bool
    (leq_nat (mul (Suc Zero) (Suc (Suc Zero)))
      (mul (Suc (Suc Zero)) (Suc (Suc Zero)))) True =
  leq_nat_mul_right_mono (Suc Zero) (Suc (Suc Zero)) (Suc Zero) base_order
theorem recovered_order : Equal Bool (leq_nat (Suc Zero) (Suc (Suc Zero))) True =
  leq_nat_mul_right_cancel (Suc Zero) (Suc (Suc Zero)) (Suc Zero) scaled_order
theorem reverse_scaled_false : Equal Bool
    (leq_nat (mul (Suc (Suc Zero)) (Suc (Suc Zero)))
      (mul (Suc Zero) (Suc (Suc Zero)))) False = Proved"#,
    )
    .expect("distinct scaled true/false outcomes and generic transports must check");
}

/// Promise class: durable invariant. The attached successor multiplication
/// helper is not a public client capability, and zero-factor cancellation
/// cannot impersonate the positive-factor checked theorem.
#[test]
fn private_successor_helper_and_zero_factor_cancellation_are_unavailable() {
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_module_from_roots(&[root()], ARITHMETIC)
        .expect("Arithmetic must load before checking its client boundary");
    env.elaborate_file("import Data.Numeric.Nat.Arithmetic (add, mul)")
        .expect("the public arithmetic subjects import");
    let private = env
        .elaborate_decl("theorem illicit_suc_l (a : Nat) (b : Nat) : Equal Nat (mul (Suc a) b) (add (mul a b) b) = (proof suc_l for mul) a b")
        .expect_err("private mul::suc_l must not cross the module boundary");
    assert!(
        matches!(private, ElabError::UnboundName { ref name, .. } if name.ends_with("mul::suc_l")),
        "expected the private mul::suc_l identity to be missing, not {private:?}"
    );
    let zero_factor = env
        .elaborate_decl(
            "theorem illicit_cancel_zero (h : Equal Nat (mul Zero Zero) (mul (Suc Zero) Zero)) : Equal Nat Zero (Suc Zero) = (proof cancel_r for mul) Zero (Suc Zero) Zero h",
        )
        .expect_err("the positive-factor proof must refuse a zero-factor hypothesis");
    assert!(
        matches!(zero_factor, ElabError::KernelRejected { .. }),
        "expected kernel refusal for the wrong factor, not {zero_factor:?}"
    );
}
