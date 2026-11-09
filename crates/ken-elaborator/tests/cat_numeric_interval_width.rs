//! Spec 68 §15: truncated Rational subtraction and interval width.
//! Promise class: durable invariant. The public laws are checked at open
//! generic types; separately stated closed values pin the width definition.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{Decl, KernelError};

const RATIONAL: &str = "Data.Numeric.Rational";
const INTERVAL: &str = "Algorithm.Numeric.RationalInterval";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn interval_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], INTERVAL)
        .expect("§15 providers and width proofs root-load");
    env
}

const GENERIC_CLIENT: &str = r#"
import Data.Numeric.Rational
  (Rational, rational_zero, rational_add, rational_sub, rational_leq, rational_equiv)
import Algorithm.Numeric.RationalInterval
  (Interval, MkInterval, interval_add, interval_point, interval_width,
   interval_width_point, interval_width_add)

theorem sub_self (x : Rational)
    : rational_equiv (rational_sub x x) rational_zero =
  (proof self_is_zero for rational_sub) x
theorem sub_cancel (x : Rational) (y : Rational)
    : IsTrue (rational_leq y x)
      → rational_equiv (rational_add (rational_sub x y) y) x =
  (proof add_cancel for rational_sub) x y
theorem sub_distrib (x1 : Rational) (y1 : Rational)
      (x2 : Rational) (y2 : Rational)
    : IsTrue (rational_leq y1 x1)
      → IsTrue (rational_leq y2 x2)
      → rational_equiv
          (rational_sub (rational_add x1 x2) (rational_add y1 y2))
          (rational_add (rational_sub x1 y1) (rational_sub x2 y2)) =
  (proof add_distrib for rational_sub) x1 y1 x2 y2
theorem point (x : Rational)
    : rational_equiv (interval_width (interval_point x)) rational_zero =
  interval_width_point x
theorem sum (lo1 : Rational) (hi1 : Rational)
      (lo2 : Rational) (hi2 : Rational)
    : IsTrue (rational_leq lo1 hi1)
      → IsTrue (rational_leq lo2 hi2)
      → rational_equiv
          (interval_width (interval_add (MkInterval lo1 hi1) (MkInterval lo2 hi2)))
          (rational_add
            (interval_width (MkInterval lo1 hi1))
            (interval_width (MkInterval lo2 hi2))) =
  interval_width_add lo1 hi1 lo2 hi2
"#;

/// MEASURED: after explicit provider loading, exact trusted GlobalId identity
/// sets do not grow; the seven Rational/Interval exports have owned Transparent
/// bodies and an independent selective-import client checks their open types.
/// CLAIMED: the §15 operations and laws are generic, public and zero-delta.
/// THE GAP: inherited provider trust remains, and these types do not by
/// themselves characterize width; the separate closed value row below does.
#[test]
fn generic_subtraction_and_width_laws_are_public_checked_and_zero_delta() {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        "Core.Logic.Transport",
        "Core.Logic.Conjunction",
        "Core.Classes.LawfulClasses",
        "Data.Numeric.Nat.Arithmetic",
        "Data.Numeric.Nat.Order",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider} roots-loads: {error:?}"));
    }
    let baseline = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let rational = env
        .elaborate_module_from_roots(&[root()], RATIONAL)
        .expect("§15 Rational proofs check");
    assert_eq!(
        baseline,
        env.env.trusted_base().into_iter().collect(),
        "Rational adds no trusted GlobalId"
    );
    let interval = env
        .elaborate_module_from_roots(&[root()], INTERVAL)
        .expect("§15 Interval proofs check");
    assert_eq!(
        baseline,
        env.env.trusted_base().into_iter().collect(),
        "Interval adds no trusted GlobalId"
    );
    for name in [
        "rational_sub",
        "rational_sub::self_is_zero",
        "rational_sub::add_cancel",
        "rational_sub::add_distrib",
    ] {
        let id = env.globals[&format!("{RATIONAL}.{name}")];
        assert!(rational.contains(&id), "{name} is Rational-owned");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{name} has a checked body"
        );
    }
    for name in [
        "interval_width",
        "interval_width_point",
        "interval_width_add",
    ] {
        let id = env.globals[&format!("{INTERVAL}.{name}")];
        assert!(interval.contains(&id), "{name} is Interval-owned");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{name} has a checked body"
        );
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("five independently stated generic §15 signatures check");
}

const CLOSED_PREFIX: &str = r#"
import Data.Numeric.Rational
  (Rational, MkRational, rational_zero, rational_one, rational_add, rational_equiv)
import Algorithm.Numeric.RationalInterval
  (Interval, MkInterval, interval_width, interval_add, interval_point,
   interval_width_point, interval_width_add)
const half : Rational = MkRational (Suc Zero) (Suc Zero)
const one : Rational = rational_one
const two : Rational = MkRational (Suc (Suc Zero)) Zero
const three : Rational = MkRational (Suc (Suc (Suc Zero))) Zero
const four : Rational = MkRational (Suc (Suc (Suc (Suc Zero)))) Zero
const five : Rational = MkRational (Suc (Suc (Suc (Suc (Suc Zero))))) Zero
const three_halves : Rational = MkRational (Suc (Suc (Suc Zero))) (Suc Zero)
const i : Interval = MkInterval half three_halves
const j : Interval = MkInterval one two
const nonzero_lower : Interval = MkInterval one three
const ordered : Interval = MkInterval one two
const inverted : Interval = MkInterval two one
const j5 : Interval = MkInterval rational_zero five
"#;

fn closed_env() -> ElabEnv {
    let mut env = interval_env();
    env.elaborate_file(CLOSED_PREFIX)
        .expect("closed constants and selective imports check");
    env
}

/// MEASURED: with lower endpoint 1 (not zero), [1,3] has checked width 2;
/// an independently computed sum has width 2, the sum law applies with both
/// true order premises, and a point has width zero. CLAIMED: width computes
/// the specified truncated upper-minus-lower value, not just the two laws.
/// THE GAP: these fixed values do not prove generic behavior; the checked
/// generic proof terms above do. Constant-zero passes both laws but fails
/// the nonzero-value checks here.
#[test]
fn nonzero_lower_value_and_ordered_sum_point_check_independently() {
    let mut env = closed_env();
    for (name, declaration) in [
        (
            "nonzero_lower_value",
            "theorem width_one_three_is_two : rational_equiv (interval_width nonzero_lower) two = Proved",
        ),
        (
            "sum_value",
            "theorem sum_width_is_two : rational_equiv (interval_width (interval_add i j)) two = Proved",
        ),
        (
            "sum_law",
            "theorem sum_width_by_law : rational_equiv (interval_width (interval_add i j)) (rational_add (interval_width i) (interval_width j)) = interval_width_add half three_halves one two Proved Proved",
        ),
        (
            "point_law",
            "theorem point_width_zero : rational_equiv (interval_width (interval_point half)) rational_zero = interval_width_point half",
        ),
        (
            "point_value",
            "theorem point_width_zero_by_value : rational_equiv (interval_width (interval_point half)) rational_zero = Proved",
        ),
    ] {
        env.elaborate_decl(declaration)
            .unwrap_or_else(|error| panic!("{name} must check: {error:?}"));
    }
}

/// MEASURED: ordered sum and width-five same-form twins check, while inverted
/// sum and width-four false assertions each reject at Kernel TypeMismatch.
/// CLAIMED: truncated subtraction needs both order premises for the sum law,
/// and closed value assertions cannot be accepted simply by importing it.
/// THE GAP: these refusals measure two closed values, not universal law proof.
#[test]
fn inverted_sum_and_wrong_width_refuse_beside_same_form_controls() {
    for (name, accepted, refused) in [
        (
            "inverted_sum",
            "theorem ordered_sum_width : rational_equiv (interval_width (interval_add ordered j5)) (rational_add (interval_width ordered) (interval_width j5)) = Proved",
            "theorem inverted_sum_width : rational_equiv (interval_width (interval_add inverted j5)) (rational_add (interval_width inverted) (interval_width j5)) = Proved",
        ),
        (
            "width_four",
            "theorem j_width_is_five : rational_equiv (interval_width j5) five = Proved",
            "theorem j_width_is_four : rational_equiv (interval_width j5) four = Proved",
        ),
    ] {
        closed_env()
            .elaborate_decl(accepted)
            .unwrap_or_else(|error| panic!("{name} same-form twin must check: {error:?}"));
        let error = closed_env()
            .elaborate_decl(refused)
            .expect_err("closed false assertion must refuse");
        assert!(
            matches!(
                error,
                ElabError::KernelRejected {
                    error: KernelError::TypeMismatch { .. },
                    ..
                }
            ),
            "{name} must refuse at its own proof site: {error:?}"
        );
    }
}
