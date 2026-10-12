//! Spec: spec/50-stdlib/64-rational.md §§1–2.2.
//! Conformance: conformance/stdlib/numeric/seed-rational.md.
//! Each seed oracle has promise class durable invariant. Closed arithmetic
//! expectations are independent fixtures; generic obligations are proved by
//! the imported public laws, not inferred from these examples.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::Decl;

const MODULE: &str = "Data.Numeric.Rational";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn loaded() -> ElabEnv {
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_module_from_roots(&[root()], MODULE)
        .expect("Rational provider must roots-load and kernel-check");
    env
}

const FIXTURE: &str = r#"
import Data.Numeric.Rational
  (Rational, MkRational, rational_zero, rational_one,
   numerator, denominator, rational_of_nat,
   rational_add, rational_mul, rational_equiv, rational_leq)
import Data.Numeric.Nat.Order (lt_nat)

const half : Rational = MkRational (Suc Zero) (Suc Zero)
const two_quarters : Rational =
  MkRational (Suc (Suc Zero)) (Suc (Suc (Suc Zero)))
const third : Rational = MkRational (Suc Zero) (Suc (Suc Zero))
const five_sixths : Rational =
  MkRational (Suc (Suc (Suc (Suc (Suc Zero)))))
    (Suc (Suc (Suc (Suc (Suc Zero)))))
const one_sixth : Rational =
  MkRational (Suc Zero) (Suc (Suc (Suc (Suc (Suc Zero)))))
const zero_sevenths : Rational =
  MkRational Zero (Suc (Suc (Suc (Suc (Suc (Suc Zero))))))
"#;

fn fixture() -> ElabEnv {
    let mut env = loaded();
    env.elaborate_file(FIXTURE)
        .expect("independent rational seed spellings must elaborate");
    env
}

fn checked(env: &mut ElabEnv, name: &str, proposition: &str) {
    env.elaborate_decl(&format!("theorem seed_{name} : {proposition} = Proved"))
        .unwrap_or_else(|error| panic!("{name}: independent closed oracle must check: {error:?}"));
}

/// Promise class: durable invariant. MEASURED: the provider's 10 public
/// declarations, exported constructor, and 23 public checked laws have owned
/// GlobalIds; an independent generic client unpacks every law at its exact
/// proposition; the exact trusted-ID set stays equal. CLAIMED: §§1–2.2 add a
/// zero-trust importable rational value and all its generic obligations. THE
/// GAP: the five separate seed cases exercise concrete value distinctions;
/// neither the owned-ID census nor this client's type alone is a seed oracle.
#[test]
fn all_public_values_and_laws_import_generically_without_trust_growth() {
    let mut env = ElabEnv::new().expect("base environment");
    for provider in [
        "Core.Logic.Transport",
        "Core.Classes.LawfulClasses",
        "Data.Numeric.Nat.Arithmetic",
        "Data.Numeric.Nat.Order",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider} must load: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], MODULE)
        .expect("Rational package must be checked at its real root");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "trusted_base() count: before={}, after={}",
        before.len(),
        after.len()
    );
    assert_eq!(after, before, "rational values may add no trusted IDs");

    for name in [
        "Rational",
        "numerator",
        "denominator",
        "rational_zero",
        "rational_one",
        "rational_of_nat",
        "rational_equiv",
        "rational_add",
        "rational_mul",
        "rational_leq",
        "denominator::positive",
        "rational_equiv::refl",
        "rational_equiv::sym",
        "rational_equiv::trans",
        "rational_add::denominator_product",
        "rational_mul::denominator_product",
        "rational_add::respect",
        "rational_mul::respect",
        "rational_leq::respect",
        "rational_add::comm",
        "rational_add::assoc",
        "rational_add::zero_l",
        "rational_add::zero_r",
        "rational_mul::comm",
        "rational_mul::assoc",
        "rational_mul::one_l",
        "rational_mul::one_r",
        "rational_mul::zero_l",
        "rational_mul::distrib_l",
        "rational_leq::refl",
        "rational_leq::trans",
        "rational_leq::antisym",
        "rational_leq::total",
    ] {
        let qualified = format!("{MODULE}.{name}");
        let id = *env
            .globals
            .get(&qualified)
            .unwrap_or_else(|| panic!("{qualified} must resolve"));
        assert!(owned.contains(&id), "{qualified} must be provider-owned");
        if name != "Rational" {
            assert!(
                matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
                "{qualified} must be a checked transparent declaration"
            );
        }
    }
    let rational = env.globals[&format!("{MODULE}.Rational")];
    let constructor = env.globals[&format!("{MODULE}.MkRational")];
    assert!(
        env.env
            .inductive(rational)
            .expect("Rational is an inductive carrier")
            .constructors
            .iter()
            .any(|member| member.id == constructor),
        "public MkRational constructs the unique positive-denominator carrier"
    );

    env.elaborate_file(
        r#"import Data.Numeric.Rational
  (Rational, MkRational, numerator, denominator, rational_zero, rational_one,
   rational_of_nat, rational_equiv, rational_add, rational_mul, rational_leq)
import Data.Numeric.Nat.Arithmetic (mul)
import Data.Numeric.Nat.Order (lt_nat)

theorem client_positive (x : Rational) :
  Equal Bool (lt_nat Zero (denominator x)) True =
  (proof positive for denominator) x
theorem client_equiv_refl (x : Rational) : rational_equiv x x =
  (proof refl for rational_equiv) x
theorem client_equiv_sym (x : Rational) (y : Rational)
  (xy : rational_equiv x y) : rational_equiv y x =
  (proof sym for rational_equiv) x y xy
theorem client_equiv_trans (x : Rational) (y : Rational) (z : Rational)
  (xy : rational_equiv x y) (yz : rational_equiv y z) :
  rational_equiv x z = (proof trans for rational_equiv) x y z xy yz
theorem client_den_add (x : Rational) (y : Rational) :
  Equal Nat (denominator (rational_add x y))
    (mul (denominator x) (denominator y)) =
  (proof denominator_product for rational_add) x y
theorem client_den_mul (x : Rational) (y : Rational) :
  Equal Nat (denominator (rational_mul x y))
    (mul (denominator x) (denominator y)) =
  (proof denominator_product for rational_mul) x y
theorem client_add_respect (x : Rational) (xp : Rational)
  (y : Rational) (yp : Rational) (hx : rational_equiv x xp)
  (hy : rational_equiv y yp) :
  rational_equiv (rational_add x y) (rational_add xp yp) =
  (proof respect for rational_add) x xp y yp hx hy
theorem client_mul_respect (x : Rational) (xp : Rational)
  (y : Rational) (yp : Rational) (hx : rational_equiv x xp)
  (hy : rational_equiv y yp) :
  rational_equiv (rational_mul x y) (rational_mul xp yp) =
  (proof respect for rational_mul) x xp y yp hx hy
theorem client_leq_respect (x : Rational) (xp : Rational)
  (y : Rational) (yp : Rational) (hx : rational_equiv x xp)
  (hy : rational_equiv y yp) :
  Equal Bool (rational_leq x y) (rational_leq xp yp) =
  (proof respect for rational_leq) x xp y yp hx hy
theorem client_add_comm (x : Rational) (y : Rational) :
  rational_equiv (rational_add x y) (rational_add y x) =
  (proof comm for rational_add) x y
theorem client_add_assoc (x : Rational) (y : Rational) (z : Rational) :
  rational_equiv (rational_add x (rational_add y z))
    (rational_add (rational_add x y) z) =
  (proof assoc for rational_add) x y z
theorem client_add_zero_l (x : Rational) :
  rational_equiv (rational_add rational_zero x) x =
  (proof zero_l for rational_add) x
theorem client_add_zero_r (x : Rational) :
  rational_equiv (rational_add x rational_zero) x =
  (proof zero_r for rational_add) x
theorem client_mul_comm (x : Rational) (y : Rational) :
  rational_equiv (rational_mul x y) (rational_mul y x) =
  (proof comm for rational_mul) x y
theorem client_mul_assoc (x : Rational) (y : Rational) (z : Rational) :
  rational_equiv (rational_mul x (rational_mul y z))
    (rational_mul (rational_mul x y) z) =
  (proof assoc for rational_mul) x y z
theorem client_mul_one_l (x : Rational) :
  rational_equiv (rational_mul rational_one x) x =
  (proof one_l for rational_mul) x
theorem client_mul_one_r (x : Rational) :
  rational_equiv (rational_mul x rational_one) x =
  (proof one_r for rational_mul) x
theorem client_mul_zero_l (x : Rational) :
  rational_equiv (rational_mul rational_zero x) rational_zero =
  (proof zero_l for rational_mul) x
theorem client_distrib_l (x : Rational) (y : Rational) (z : Rational) :
  rational_equiv (rational_mul x (rational_add y z))
    (rational_add (rational_mul x y) (rational_mul x z)) =
  (proof distrib_l for rational_mul) x y z
theorem client_leq_refl (x : Rational) : Equal Bool (rational_leq x x) True =
  (proof refl for rational_leq) x
theorem client_leq_trans (x : Rational) (y : Rational) (z : Rational)
  (xy : Equal Bool (rational_leq x y) True)
  (yz : Equal Bool (rational_leq y z) True) :
  Equal Bool (rational_leq x z) True =
  (proof trans for rational_leq) x y z xy yz
theorem client_leq_antisym (x : Rational) (y : Rational)
  (xy : Equal Bool (rational_leq x y) True)
  (yx : Equal Bool (rational_leq y x) True) :
  rational_equiv x y =
  (proof antisym for rational_leq) x y xy yx
theorem client_leq_total (x : Rational) (y : Rational)
  (xy_false : Equal Bool (rational_leq x y) False) :
  Equal Bool (rational_leq y x) True =
  (proof total for rational_leq) x y xy_false"#,
    )
    .expect("all 23 public checked laws must inhabit generic client claims");

    for private in [
        "positive_product_tail",
        "positive_product_factor",
        "mul_cross_respect",
        "leq_cross_respect",
        "rational_leq_cross_trans",
        "rational_leq_false_opposite",
        "mul_shuffle",
    ] {
        let id = env.globals[&format!("{MODULE}.{private}")];
        assert!(
            owned.contains(&id),
            "{private} must really exist in the package"
        );
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{private} must have a checked body, not an absent spelling"
        );
        match env.elaborate_file(&format!("import {MODULE} ({private})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{MODULE}.{private}"));
            }
            Err(other) => panic!("{private}: wrong private-import failure: {other:?}"),
            Ok(_) => panic!("{private} must stay package-local"),
        }
    }
}

/// Promise class: durable invariant. MEASURED: fixed cross products distinguish
/// two spellings of one value from another value with the same numerator;
/// wrong structural carrier equality and false equivalence both kernel-reject.
/// CLAIMED: equivalence is value equality, not pair identity. THE GAP: generic
/// transitivity and symmetry are separately pinned by the imported client.
#[test]
fn seed_equivalent_not_identical() {
    let mut env = fixture();
    checked(
        &mut env,
        "half_quarters_equivalent",
        "rational_equiv half two_quarters",
    );
    checked(
        &mut env,
        "half_denominator",
        "Equal Nat (denominator half) (Suc (Suc Zero))",
    );
    checked(
        &mut env,
        "quarters_denominator",
        "Equal Nat (denominator two_quarters) (Suc (Suc (Suc (Suc Zero))))",
    );
    checked(
        &mut env,
        "half_positive",
        "Equal Bool (lt_nat Zero (denominator half)) True",
    );
    checked(
        &mut env,
        "quarters_positive",
        "Equal Bool (lt_nat Zero (denominator two_quarters)) True",
    );
    for (label, claim) in [
        ("different_values", "rational_equiv half third"),
        ("different_carriers", "Equal Rational half two_quarters"),
    ] {
        let error = env
            .elaborate_decl(&format!("theorem false_{label} : {claim} = Proved"))
            .expect_err("false closed claim must kernel-reject");
        assert!(
            matches!(error, ElabError::KernelRejected { .. }),
            "{label}: expected kernel rejection, not {error:?}"
        );
    }
}

/// Promise class: durable invariant. MEASURED: the sum's independent expected
/// value and denominator six, including an equivalent input spelling, check
/// by kernel reduction. CLAIMED: cross-denominator addition is exact and
/// spelling-invariant. THE GAP: generic add respect is checked above.
#[test]
fn seed_sum_one_half_one_third() {
    let mut env = fixture();
    checked(
        &mut env,
        "sum",
        "rational_equiv (rational_add half third) five_sixths",
    );
    checked(&mut env, "sum_denominator", "Equal Nat (denominator (rational_add half third)) (Suc (Suc (Suc (Suc (Suc (Suc Zero))))))");
    checked(
        &mut env,
        "sum_spelling",
        "rational_equiv (rational_add two_quarters third) five_sixths",
    );
}

/// Promise class: durable invariant. MEASURED: both the independently fixed
/// one-sixth value and denominator six, with an alternative equivalent input.
/// CLAIMED: multiplication forms the cross-denominator exact product. THE GAP:
/// generic multiplication respect and algebraic laws are checked above.
#[test]
fn seed_product_one_sixth() {
    let mut env = fixture();
    checked(
        &mut env,
        "product",
        "rational_equiv (rational_mul half third) one_sixth",
    );
    checked(&mut env, "product_denominator", "Equal Nat (denominator (rational_mul half third)) (Suc (Suc (Suc (Suc (Suc (Suc Zero))))))");
    checked(
        &mut env,
        "product_spelling",
        "rational_equiv (rational_mul two_quarters third) one_sixth",
    );
}

/// Promise class: durable invariant. MEASURED: the same strictly unequal
/// inputs yield True and False in opposite directions, while different
/// equivalent spellings tie both ways; the law applications below inhabit
/// their generic claims from these real fixed comparisons. CLAIMED: canonical
/// cross-product order is exact. THE GAP: generic order and respect laws are
/// independently unpacked in the client above; these closed fixtures cannot
/// replace those checked arbitrary-input proofs.
#[test]
fn seed_order_opposite_directions() {
    let mut env = fixture();
    checked(
        &mut env,
        "third_leq_half",
        "Equal Bool (rational_leq third half) True",
    );
    checked(
        &mut env,
        "half_not_leq_third",
        "Equal Bool (rational_leq half third) False",
    );
    let incorrect_reverse = env
        .elaborate_decl(
            "theorem false_order_direction : Equal Bool (rational_leq half third) True = Proved",
        )
        .expect_err("the opposite Boolean result must kernel-reject");
    assert!(
        matches!(incorrect_reverse, ElabError::KernelRejected { .. }),
        "false reverse order must kernel-reject, not {incorrect_reverse:?}"
    );
    checked(
        &mut env,
        "half_leq_quarters",
        "Equal Bool (rational_leq half two_quarters) True",
    );
    checked(
        &mut env,
        "quarters_leq_half",
        "Equal Bool (rational_leq two_quarters half) True",
    );
    checked(
        &mut env,
        "third_leq_quarters",
        "Equal Bool (rational_leq third two_quarters) True",
    );
    checked(
        &mut env,
        "quarters_not_leq_third",
        "Equal Bool (rational_leq two_quarters third) False",
    );
    checked(
        &mut env,
        "half_quarters_equivalent",
        "rational_equiv half two_quarters",
    );
    env.elaborate_file(
        r#"theorem seed_order_refl : Equal Bool (rational_leq half half) True =
  (proof refl for rational_leq) half
theorem seed_order_trans : Equal Bool (rational_leq third two_quarters) True =
  (proof trans for rational_leq) third half two_quarters Proved Proved
theorem seed_order_antisym : rational_equiv half two_quarters =
  (proof antisym for rational_leq) half two_quarters Proved Proved
theorem seed_order_total : Equal Bool (rational_leq third half) True =
  (proof total for rational_leq) half third Proved"#,
    )
    .expect("order seed law instances must kernel-check over unequal and tied fixtures");
}

/// Promise class: durable invariant. MEASURED: two distinct zero spellings
/// give equivalent values; addition and multiplication preserve their units
/// on both sides of one nonzero sample with positive result denominators.
/// CLAIMED: zero and one act as two-sided value identities. THE GAP: the
/// imported generic proofs cover arbitrary inputs and left zero annihilation.
#[test]
fn seed_units_across_spellings() {
    let mut env = fixture();
    for (label, proposition) in [
        (
            "zero_spelling",
            "rational_equiv zero_sevenths rational_zero",
        ),
        (
            "add_zero_left",
            "rational_equiv (rational_add rational_zero half) half",
        ),
        (
            "add_zero_right",
            "rational_equiv (rational_add half rational_zero) half",
        ),
        (
            "add_zero_sevenths",
            "rational_equiv (rational_add zero_sevenths half) half",
        ),
        (
            "mul_one_left",
            "rational_equiv (rational_mul rational_one half) half",
        ),
        (
            "mul_one_right",
            "rational_equiv (rational_mul half rational_one) half",
        ),
        (
            "add_zero_positive",
            "Equal Bool (lt_nat Zero (denominator (rational_add zero_sevenths half))) True",
        ),
        (
            "mul_one_positive",
            "Equal Bool (lt_nat Zero (denominator (rational_mul half rational_one))) True",
        ),
        (
            "add_zero_left_positive",
            "Equal Bool (lt_nat Zero (denominator (rational_add rational_zero half))) True",
        ),
        (
            "add_zero_right_positive",
            "Equal Bool (lt_nat Zero (denominator (rational_add half rational_zero))) True",
        ),
        (
            "mul_one_left_positive",
            "Equal Bool (lt_nat Zero (denominator (rational_mul rational_one half))) True",
        ),
    ] {
        checked(&mut env, label, proposition);
    }
}
