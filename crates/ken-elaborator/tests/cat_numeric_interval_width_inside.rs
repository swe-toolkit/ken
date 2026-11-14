//! Spec 68 §16: interval width under inclusion and Rational subtraction order.
//! Promise classes: durable generic laws and normative closed value controls.
//! A separate open client applies each public law; closed control/refusal pairs
//! check each direction on fresh, shared prefixes.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{Decl, KernelError};

const ORDER: &str = "Data.Numeric.Nat.Order";
const BOX: &str = "Algorithm.Geometry.NatBox";
const GCD: &str = "Algorithm.Numeric.Gcd";
const RATIONAL: &str = "Data.Numeric.Rational";
const INTERVAL: &str = "Algorithm.Numeric.RationalInterval";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages")
}

fn loaded_interval() -> ElabEnv {
    let mut env = ElabEnv::new().expect("prelude bootstrap");
    env.elaborate_module_from_roots(&[root()], INTERVAL)
        .expect("width-inside package and its providers roots-load");
    env
}

/// SOURCE: spec 68 §16. PROMISE CLASS: durable invariant.
/// MEASURED: after provider-first loading, the exact trusted GlobalId set
/// stays equal through each of five package loads, and the seven new public
/// names have checked Transparent bodies owned by their defining modules.
/// CLAIMED: this law migration adds no local trust and publishes the seven
/// names from their canonical owners. THE GAP: inherited provider trust stays
/// outside this local delta; open/closed clients below check the law behavior.
#[test]
fn provider_first_trust_and_seven_owned_transparent_laws() {
    let mut env = ElabEnv::new().expect("prelude bootstrap");
    for provider in [
        "Core.Logic.Transport",
        "Core.Logic.Conjunction",
        "Core.Classes.LawfulClasses",
        "Data.Numeric.Nat.Arithmetic",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider} roots-loads: {error:?}"));
    }
    let initial = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    for (module, names) in [
        (
            ORDER,
            &[
                "sub::leq_left",
                "sub::left_mono",
                "sub::right_antitone",
                "mul_sub_distrib_l",
                "mul_sub_distrib_r",
            ][..],
        ),
        (BOX, &[][..]),
        (GCD, &[][..]),
        (RATIONAL, &["rational_sub_mono"][..]),
        (INTERVAL, &["interval_width_inside"][..]),
    ] {
        let owned = env
            .elaborate_module_from_roots(&[root()], module)
            .unwrap_or_else(|error| panic!("{module} roots-loads: {error:?}"));
        let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
        assert_eq!(after, initial, "{module} adds no trusted identity");
        for name in names {
            let qualified = format!("{module}.{name}");
            let id = *env
                .globals
                .get(&qualified)
                .unwrap_or_else(|| panic!("{qualified} must be public"));
            assert!(owned.contains(&id), "{qualified} owned by defining module");
            assert!(
                matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
                "{qualified} has a checked body"
            );
        }
    }
}

const OPEN_CLIENT: &str = r#"
import Core.Classes.LawfulClasses (leq_nat)
import Data.Numeric.Nat.Arithmetic (mul)
import Data.Numeric.Nat.Order (sub, mul_sub_distrib_r, mul_sub_distrib_l)
import Data.Numeric.Rational (Rational, rational_sub, rational_leq, rational_sub_mono)
import Algorithm.Numeric.RationalInterval (Interval, interval_inside, interval_width, interval_width_inside)

theorem open_sub_leq_left (a : Nat) (k : Nat)
  : IsTrue (leq_nat (sub a k) a) = (proof leq_left for sub) a k

theorem open_sub_left_mono (a : Nat) (b : Nat) (k : Nat)
  : IsTrue (leq_nat a b) → IsTrue (leq_nat (sub a k) (sub b k)) =
  (proof left_mono for sub) a b k

theorem open_sub_right_antitone (a : Nat) (k : Nat) (l : Nat)
  : IsTrue (leq_nat k l) → IsTrue (leq_nat (sub a l) (sub a k)) =
  (proof right_antitone for sub) a k l

theorem open_mul_sub_r (a : Nat) (b : Nat) (c : Nat)
  : Equal Nat (mul a (sub b c)) (sub (mul a b) (mul a c)) =
  mul_sub_distrib_r a b c

theorem open_mul_sub_l (a : Nat) (b : Nat) (c : Nat)
  : Equal Nat (mul (sub a b) c) (sub (mul a c) (mul b c)) =
  mul_sub_distrib_l a b c

theorem open_rational_sub_mono (x : Rational) (y : Rational) (z : Rational) (w : Rational)
  : Equal Bool (rational_leq x y) True
    → Equal Bool (rational_leq w z) True
    → Equal Bool (rational_leq (rational_sub x z) (rational_sub y w)) True =
  rational_sub_mono x y z w

theorem open_interval_width_inside (i : Interval) (k : Interval)
  : IsTrue (interval_inside i k)
    → IsTrue (rational_leq (interval_width i) (interval_width k)) =
  interval_width_inside i k
"#;

/// SOURCE: spec 68 §16. PROMISE CLASS: durable invariant.
/// MEASURED: a separately elaborated open client imports each public law,
/// selects all three attached sub proofs, and applies exact public types.
/// CLAIMED: all seven signatures can be used by arbitrary independent clients.
/// THE GAP: an open application cannot pin closed arithmetic values; the A
/// rows below check their conclusions independently by Proved.
#[test]
fn seven_public_open_law_shapes_and_two_private_refusals() {
    let mut env = loaded_interval();
    env.elaborate_file(OPEN_CLIENT)
        .expect("seven law shapes check at open arguments");
    for (module, helper) in [
        (RATIONAL, "rational_sub_cross_mono"),
        (ORDER, "sub_add_same"),
    ] {
        let source = format!("import {module} ({helper})");
        match env.elaborate_file(&source) {
            Err(ElabError::UnboundName { name, span }) => {
                assert_eq!(name, format!("{module}.{helper}"));
                assert!(span.start < span.end && span.end <= source.len());
                eprintln!("PRIVATE {module}.{helper}: UnboundName at {span:?}");
            }
            other => panic!("{module}.{helper} must not be selectable: {other:?}"),
        }
    }
}

const PREFIX: &str = r#"
import Data.Numeric.Rational (Rational, MkRational, rational_sub, rational_leq, rational_equiv)
import Algorithm.Numeric.RationalInterval
  (Interval, MkInterval, interval_inside, interval_width, interval_intersect,
   interval_intersect_inside_left, interval_width_inside)
const r0 : Rational = MkRational Zero Zero
const r1 : Rational = MkRational (Suc Zero) Zero
const r2 : Rational = MkRational (Suc (Suc Zero)) Zero
const r3 : Rational = MkRational (Suc (Suc (Suc Zero))) Zero
const r4 : Rational = MkRational (Suc (Suc (Suc (Suc Zero)))) Zero
const inner : Interval = MkInterval r1 r2
const outer : Interval = MkInterval r0 r4
const inverted : Interval = MkInterval r3 r1
const left : Interval = MkInterval r0 r3
const right : Interval = MkInterval r1 r4
"#;

const A1: &str = r#"
theorem a1_application
    : IsTrue (rational_leq (interval_width inner) (interval_width outer)) =
  interval_width_inside inner outer Proved
theorem a1_value
    : IsTrue (rational_leq (interval_width inner) (interval_width outer)) = Proved
theorem a1_inner_width : rational_equiv (interval_width inner) r1 = Proved
theorem a1_outer_width : rational_equiv (interval_width outer) r4 = Proved
"#;
const A2: &str = r#"
theorem a2_application
    : IsTrue (rational_leq (interval_width inverted) (interval_width outer)) =
  interval_width_inside inverted outer Proved
theorem a2_value
    : IsTrue (rational_leq (interval_width inverted) (interval_width outer)) = Proved
theorem a2_inverted_width : rational_equiv (interval_width inverted) r0 = Proved
"#;
const A3: &str = r#"
theorem a3_application
    : IsTrue (rational_leq
        (interval_width (interval_intersect left right)) (interval_width left)) =
  interval_width_inside (interval_intersect left right) left
    (interval_intersect_inside_left left right)
theorem a3_value
    : IsTrue (rational_leq
        (interval_width (interval_intersect left right)) (interval_width left)) = Proved
theorem a3_intersection_width
    : rational_equiv (interval_width (interval_intersect left right)) r2 = Proved
"#;
const A4: &str = r#"
import Data.Numeric.Rational (rational_sub_mono)
theorem a4_application
    : IsTrue (rational_leq (rational_sub r1 r1) (rational_sub r3 r0)) =
  rational_sub_mono r1 r3 r1 r0 Proved Proved
theorem a4_value
    : IsTrue (rational_leq (rational_sub r1 r1) (rational_sub r3 r0)) = Proved
"#;

/// SOURCE: spec 68 §16. PROMISE CLASS: normative compatibility vector.
/// MEASURED: A1–A4 each check a law application and an independently authored
/// closed Proved conclusion; additional values pin widths 1 and 4, inverted
/// width zero and intersection width 2. CLAIMED: the laws have nondegenerate,
/// separately checked closed instances.
/// THE GAP: four instances do not prove arbitrary inputs; checked open types
/// above carry the general claim, while controls below pin the false boundary.
#[test]
fn four_closed_instances_have_independent_values() {
    let mut env = loaded_interval();
    env.elaborate_file(PREFIX)
        .expect("r0–r4 and intervals check");
    for (tag, source) in [("A1", A1), ("A2", A2), ("A3", A3), ("A4", A4)] {
        env.elaborate_file(source)
            .unwrap_or_else(|error| panic!("{tag}: {error:?}"));
        eprintln!("{tag}: application and independent Proved value accepted");
    }
}

const T1_CONTROL: &str = "theorem t1_control : IsTrue (rational_leq (interval_width inner) (interval_width outer)) = Proved";
const T1_TWIN: &str = "theorem t1_false : IsTrue (rational_leq (interval_width outer) (interval_width inner)) = Proved";
const T2_CONTROL: &str =
    "theorem t2_control : IsTrue (rational_leq (rational_sub r2 r1) (rational_sub r3 r1)) = Proved";
const T2_TWIN: &str =
    "theorem t2_false : IsTrue (rational_leq (rational_sub r3 r1) (rational_sub r2 r1)) = Proved";
const T3_CONTROL: &str =
    "theorem t3_control : IsTrue (rational_leq (rational_sub r3 r1) (rational_sub r3 r0)) = Proved";
const T3_TWIN: &str =
    "theorem t3_false : IsTrue (rational_leq (rational_sub r3 r0) (rational_sub r3 r1)) = Proved";

fn negative_twin(tag: &str, control: &str, twin: &str) {
    let mut env = loaded_interval();
    env.elaborate_file(PREFIX).expect("fresh same-form prefix");
    env.elaborate_file(control)
        .unwrap_or_else(|error| panic!("{tag} positive control: {error:?}"));
    eprintln!("{tag} control accepted");
    let proof = twin.rfind("Proved").expect("twin proof location");
    match env.elaborate_file(twin) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        }) if span.start <= proof
            && span.end >= proof + "Proved".len()
            && span.start < span.end
            && span.end <= twin.len() =>
        {
            eprintln!("{tag} twin refused as KernelRejected(TypeMismatch) at {span:?}");
        }
        other => panic!("{tag} twin must refuse in its own theorem: {other:?}"),
    }
}

/// SOURCE: spec 68 §16. PROMISE CLASS: normative compatibility vector.
/// MEASURED: each false claim follows its distinct same-form true claim on a
/// fresh successfully loaded root and is rejected by the kernel in its own
/// theorem span. CLAIMED: T1 excludes reversed enclosure, T2 and T3 exclude
/// reversing either Rational subtraction variance. THE GAP: these are closed
/// values; they do not replace the open laws or official conformance execution.
#[test]
fn three_false_twins_refuse_after_accepted_same_form_controls() {
    negative_twin("T1 width", T1_CONTROL, T1_TWIN);
    negative_twin("T2 first", T2_CONTROL, T2_TWIN);
    negative_twin("T3 second", T3_CONTROL, T3_TWIN);
}
