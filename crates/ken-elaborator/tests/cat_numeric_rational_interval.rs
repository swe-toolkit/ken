//! Exact rational interval clients: spec/50-stdlib/68-numerical.md §1.
//! Source: conformance/stdlib/numerical/seed-rational-interval.md.
//! Promise class: durable invariant. Independently spelled endpoint and
//! rational-value expectations guard closed behavior; the generic public
//! theorems guard universal enclosure, not merely these closed instances.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{Decl, KernelError};

const INTERVAL: &str = "Algorithm.Numeric.RationalInterval";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn loaded_interval() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], INTERVAL)
        .expect("RationalInterval roots-loads from its checked providers");
    env
}

/// SOURCE: spec/50-stdlib/68-numerical.md §§1.3–1.4.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the exact trusted GlobalId inventory is unchanged after loading
/// Rational from its explicit providers and then Interval from Rational;
/// two monotonicity laws and six interval laws resolve with transparent
/// checked bodies, and six independently stated generic client types check.
/// CLAIMED: generic Rational monotonicity and interval enclosure add no
/// new trust and all six interval laws have the specified universal types.
/// THE GAP: identity-set equality preserves inherited assumptions; closed
/// result behavior is independently tested below, not proved by this census.
#[test]
fn generic_interval_laws_and_rational_monotonicity_add_no_trust() {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        "Core.Logic.Transport",
        "Core.Classes.LawfulClasses",
        "Data.Numeric.Nat.Arithmetic",
        "Data.Numeric.Nat.Order",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider} roots-load: {error:?}"));
    }
    let baseline = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let rational = env
        .elaborate_module_from_roots(&[root()], "Data.Numeric.Rational")
        .expect("Rational monotonicity proofs check from providers");
    assert_eq!(
        env.env.trusted_base().into_iter().collect::<BTreeSet<_>>(),
        baseline,
        "new Rational laws may add no trusted GlobalId"
    );
    for name in ["rational_add_mono", "rational_mul_mono"] {
        let id = env.globals[&format!("Data.Numeric.Rational.{name}")];
        assert!(rational.contains(&id), "{name} must belong to Rational");
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], INTERVAL)
        .expect("RationalInterval generic proof bodies kernel-check");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "trusted_base() count: before={}, after={}",
        before.len(),
        after.len()
    );
    assert_eq!(after, before, "Interval may add no trusted GlobalId");
    for law in [
        "interval_add_encloses",
        "interval_mul_encloses",
        "interval_point_contains",
        "interval_point_sound",
        "interval_add_point",
        "interval_mul_point",
    ] {
        let id = env.globals[&format!("{INTERVAL}.{law}")];
        assert!(owned.contains(&id), "{law} must belong to Interval");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{law} must have a checked body"
        );
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("six independently restated §1.3 generic client types check");
}

// Every §1.3 signature is restated at arbitrary intervals and rationals;
// a weaker declaration cannot pass by retaining the same theorem name.
const GENERIC_CLIENT: &str = r##"
import Data.Numeric.Rational (Rational, rational_add, rational_mul, rational_equiv)
import Algorithm.Numeric.RationalInterval
  (Interval, interval_contains, interval_add, interval_mul, interval_point,
   interval_add_encloses, interval_mul_encloses, interval_point_contains,
   interval_point_sound, interval_add_point, interval_mul_point)

theorem generic_add_encloses
  (i : Interval) (j : Interval) (x : Rational) (y : Rational) :
  IsTrue (interval_contains i x) → IsTrue (interval_contains j y) →
  IsTrue (interval_contains (interval_add i j) (rational_add x y)) =
  interval_add_encloses i j x y
theorem generic_mul_encloses
  (i : Interval) (j : Interval) (x : Rational) (y : Rational) :
  IsTrue (interval_contains i x) → IsTrue (interval_contains j y) →
  IsTrue (interval_contains (interval_mul i j) (rational_mul x y)) =
  interval_mul_encloses i j x y
theorem generic_point_contains (x : Rational) :
  IsTrue (interval_contains (interval_point x) x) = interval_point_contains x
theorem generic_point_sound (x : Rational) (y : Rational) :
  IsTrue (interval_contains (interval_point x) y) → rational_equiv x y =
  interval_point_sound x y
theorem generic_add_point (x : Rational) (y : Rational) :
  Equal Interval (interval_add (interval_point x) (interval_point y))
    (interval_point (rational_add x y)) = interval_add_point x y
theorem generic_mul_point (x : Rational) (y : Rational) :
  Equal Interval (interval_mul (interval_point x) (interval_point y))
    (interval_point (rational_mul x y)) = interval_mul_point x y
"##;

// All 24 seed assertions and their fixtures are verbatim; the redundant
// IsTrue import is omitted for an in-memory client because the compiler
// base already binds the same canonical IsTrue GlobalId and reimporting
// it under the same short name gives AmbiguousReference.
const SEED_POSITIVE: &str = r##"
import Data.Numeric.Rational
  (Rational, MkRational, rational_add, rational_mul, rational_equiv)
import Algorithm.Numeric.RationalInterval
  (Interval, MkInterval, interval_contains, interval_add, interval_mul,
   interval_point, interval_add_encloses, interval_mul_encloses,
   interval_point_contains, interval_point_sound, interval_add_point,
   interval_mul_point)

const zero : Rational = MkRational Zero Zero
const half : Rational = MkRational (Suc Zero) (Suc Zero)
const one : Rational = MkRational (Suc Zero) Zero
const two : Rational = MkRational (Suc (Suc Zero)) Zero
const three : Rational = MkRational (Suc (Suc (Suc Zero))) Zero
const two_quarters : Rational =
  MkRational (Suc (Suc Zero)) (Suc (Suc (Suc Zero)))
const three_quarters : Rational =
  MkRational (Suc (Suc (Suc Zero))) (Suc (Suc (Suc Zero)))
const three_halves : Rational = MkRational (Suc (Suc (Suc Zero))) (Suc Zero)

const left_box : Interval = MkInterval half one
const right_box : Interval = MkInterval one two
const wrong_sum : Interval =
  MkInterval (rational_add half one) (rational_add one one)
const member_sum : Rational = rational_add three_quarters three_halves
const member_product : Rational = rational_mul three_quarters three_halves

theorem lower_in : Equal Bool (interval_contains left_box half) True = Proved
theorem upper_in : Equal Bool (interval_contains left_box one) True = Proved
theorem below_out : Equal Bool (interval_contains left_box zero) False = Proved
theorem above_out : Equal Bool (interval_contains left_box two) False = Proved

theorem sum_endpoints :
  Equal Interval (interval_add left_box right_box)
    (MkInterval three_halves three) = Refl

theorem product_endpoints :
  Equal Interval (interval_mul left_box right_box)
    (MkInterval half two) = Refl

theorem point_endpoints :
  Equal Interval (interval_point half) (MkInterval half half) = Refl

theorem left_selected_member :
  Equal Bool (interval_contains left_box three_quarters) True = Proved

theorem right_selected_member :
  Equal Bool (interval_contains right_box three_halves) True = Proved

theorem empty_input_sum_point :
  Equal Bool
    (interval_contains (interval_add (MkInterval one half) left_box) three_halves)
    True = Proved

theorem sum_member :
  Equal Bool (interval_contains (interval_add left_box right_box) member_sum) True = Proved

theorem product_member :
  Equal Bool (interval_contains (interval_mul left_box right_box) member_product) True = Proved

theorem point_member :
  Equal Bool (interval_contains (interval_point half) two_quarters) True = Proved

theorem value_equivalent : rational_equiv half two_quarters = Proved

theorem point_excludes_one :
  Equal Bool (interval_contains (interval_point half) one) False = Proved

theorem empty_excludes_half :
  Equal Bool (interval_contains (MkInterval one half) half) False = Proved

theorem empty_excludes_one :
  Equal Bool (interval_contains (MkInterval one half) one) False = Proved

theorem wrong_sum_excludes_member_sum :
  Equal Bool (interval_contains wrong_sum member_sum) False = Proved

theorem law_add_encloses :
  IsTrue (interval_contains (interval_add left_box right_box) member_sum) =
  interval_add_encloses left_box right_box three_quarters three_halves Proved Proved

theorem law_mul_encloses :
  IsTrue (interval_contains (interval_mul left_box right_box) member_product) =
  interval_mul_encloses left_box right_box three_quarters three_halves Proved Proved

theorem law_point_contains :
  IsTrue (interval_contains (interval_point three_halves) three_halves) =
  interval_point_contains three_halves

theorem law_point_sound :
  rational_equiv half two_quarters = interval_point_sound half two_quarters Proved

theorem law_add_point :
  Equal Interval (interval_add (interval_point half) (interval_point one))
    (interval_point (rational_add half one)) = interval_add_point half one

theorem law_mul_point :
  Equal Interval (interval_mul (interval_point half) (interval_point two))
    (interval_point (rational_mul half two)) = interval_mul_point half two
"##;

/// SOURCE: conformance/stdlib/numerical/seed-rational-interval.md positives.
/// PROMISE CLASS: durable invariant.
/// MEASURED: all 24 seed theorems elaborate through the public client import,
/// with independent rational and endpoint spellings, both Boolean directions,
/// empty arithmetic, value equivalence and six closed public-law uses.
/// CLAIMED: inclusive exact rational interval operations have the seed's
/// specified closed results and reachable public constructor/law exports.
/// THE GAP: finite seeds alone do not establish generic enclosure; the
/// checked generic clients and provider proof bodies are pinned above.
#[test]
fn twenty_four_seed_assertions_check_as_a_client() {
    let mut env = loaded_interval();
    env.elaborate_file(SEED_POSITIVE)
        .expect("24 positive source-shaped interval assertions check");
    for name in [
        "lower_in",
        "upper_in",
        "below_out",
        "above_out",
        "sum_endpoints",
        "product_endpoints",
        "point_endpoints",
        "left_selected_member",
        "right_selected_member",
        "empty_input_sum_point",
        "sum_member",
        "product_member",
        "point_member",
        "value_equivalent",
        "point_excludes_one",
        "empty_excludes_half",
        "empty_excludes_one",
        "wrong_sum_excludes_member_sum",
        "law_add_encloses",
        "law_mul_encloses",
        "law_point_contains",
        "law_point_sound",
        "law_add_point",
        "law_mul_point",
    ] {
        let id = env.globals[name];
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{name} must be a checked seed assertion"
        );
    }
}

// Each negative program is submitted after the identical positive client,
// so neither import nor fixture failure can masquerade as the refusal.
const REJECT_SUM_EXCLUDING_SUM: &str = r##"
-- Reject: 9/4 exceeds the incorrect upper endpoint 2.
theorem reject_sum_excluding_sum : IsTrue (interval_contains wrong_sum member_sum) = Proved
"##;

const REJECT_POINT_CONTAINS_ONE: &str = r##"
-- Reject: one is not equivalent to half.
theorem reject_point_contains_one : IsTrue (interval_contains (interval_point half) one) = Proved
"##;

const REJECT_INTERVAL_CONTAINS_TWO: &str = r##"
-- Reject: two exceeds the inclusive upper endpoint one.
theorem reject_interval_contains_two : IsTrue (interval_contains left_box two) = Proved
"##;

const REJECT_EQUAL_POINT_SPELLINGS: &str = r##"
-- Reject: equivalent values have distinct stored endpoint spellings.
theorem reject_equal_point_spellings : Equal Interval (interval_point half) (interval_point two_quarters) = Refl
"##;

const REJECT_WRONG_SUM_ENDPOINTS: &str = r##"
-- Reject: the upper endpoint must be three, not two.
theorem reject_wrong_sum_endpoints : Equal Interval (interval_add left_box right_box) (MkInterval three_halves two) = Refl
"##;

/// SOURCE: conformance/stdlib/numerical/seed-rational-interval.md B1–B5.
/// PROMISE CLASS: durable invariant.
/// MEASURED: all 24 positive assertions check first; B1–B3 independently
/// kernel-reject false membership proofs at their submitted theorem, and
/// B4–B5 reject the false structural equality at the Refl proof token;
/// B4 also kernel-rejects when Proved replaces that Refl.
/// CLAIMED: wrong sums, excluded points/upper bounds and unequal stored
/// endpoint spellings cannot inhabit their false seed propositions.
/// THE GAP: these are five concrete counterexamples, not a quantifier proof.
#[test]
fn five_false_seed_propositions_are_semantically_refused() {
    for (bad, label, refl) in [
        (REJECT_SUM_EXCLUDING_SUM, "sum excludes a member", false),
        (
            REJECT_POINT_CONTAINS_ONE,
            "point wrongly contains one",
            false,
        ),
        (REJECT_INTERVAL_CONTAINS_TWO, "above upper bound", false),
        (
            REJECT_EQUAL_POINT_SPELLINGS,
            "distinct point spellings",
            true,
        ),
        (REJECT_WRONG_SUM_ENDPOINTS, "wrong sum upper endpoint", true),
    ] {
        let mut env = loaded_interval();
        env.elaborate_file(SEED_POSITIVE)
            .expect("positive seed client must check before each negative");
        match env.elaborate_file(bad) {
            Err(ElabError::TypeMismatch { span, .. }) if refl => assert_eq!(
                bad.get(span.start..span.end),
                Some("Refl"),
                "{label}: refusal must target the false proof"
            ),
            Err(ElabError::KernelRejected {
                error: KernelError::TypeMismatch { .. },
                span,
            }) if !refl => {
                let start = bad.find("theorem reject_").expect("negative theorem");
                assert!(
                    span.start >= start && span.end <= bad.len() && span.start < span.end,
                    "{label}: kernel rejection must target its false theorem, got {span:?}"
                );
            }
            other => panic!("{label}: expected semantic proof rejection, got {other:?}"),
        }
    }

    let alternative = REJECT_EQUAL_POINT_SPELLINGS.replace("= Refl", "= Proved");
    assert_ne!(alternative, REJECT_EQUAL_POINT_SPELLINGS);
    let mut env = loaded_interval();
    env.elaborate_file(SEED_POSITIVE)
        .expect("same positive client must check before the alternate B4 terminal");
    match env.elaborate_file(&alternative) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        }) => assert!(
            span.start >= alternative.find("theorem reject_").expect("B4 theorem")
                && span.end <= alternative.len(),
            "B4's alternate proof must be refused at its false theorem"
        ),
        other => panic!("B4's false structural equality with Proved must reject: {other:?}"),
    }
}
