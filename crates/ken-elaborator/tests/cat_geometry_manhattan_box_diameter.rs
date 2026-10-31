//! Manhattan box diameter: spec/50-stdlib/67-geometry.md §10.
//! Promise class: durable invariant. A package-owned client checks the generic
//! law and closed tight/outside controls; CV owns the separate seed.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{Decl, KernelError};

const GEOMETRY: &str = "Algorithm.Geometry.NatBox";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn loaded_geometry() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], GEOMETRY)
        .expect("NatBox and §10 checked proof root-load");
    env
}

// Restate the entire public law type on arbitrary, separately named points.
// Closed distance computations cannot substitute for this generic client.
const GENERIC_CLIENT: &str = r#"
import Core.Classes.LawfulClasses (leq_nat)
import Algorithm.Geometry.NatBox
  (Point, MkBox, contains, manhattan, manhattan_box_diameter)

theorem diameter_for_contained_points
      (lo : Point) (hi : Point) (p : Point) (q : Point)
    : IsTrue (contains (MkBox lo hi) p)
      → IsTrue (contains (MkBox lo hi) q)
      → IsTrue (leq_nat (manhattan p q) (manhattan lo hi)) =
  manhattan_box_diameter lo hi p q
"#;

/// SOURCE: spec/50-stdlib/67-geometry.md §10.
/// PROMISE CLASS: durable invariant.
/// MEASURED: four explicit providers root-load before NatBox, which leaves
/// their exact trusted GlobalId set unchanged. The generic client checks the
/// full public type, and the theorem and all four private helpers are owned
/// Transparent checked definitions. Each helper's selective import refuses.
/// CLAIMED: the public diameter law is universally checked without new trust
/// or exposing per-axis proof scaffolding.
/// THE GAP: inherited provider trust remains. Closed witnesses below measure
/// tightness and false-premise refusal, not universal quantification.
#[test]
fn diameter_generic_law_is_checked_without_new_trust_or_public_helpers() {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        "Core.Classes.LawfulClasses",
        "Core.Logic.Transport",
        "Data.Numeric.Nat.Arithmetic",
        "Data.Numeric.Nat.Order",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider} root-loads: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], GEOMETRY)
        .expect("NatBox §10 proofs kernel-check");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!("§10 trusted GlobalIds: {} → {}", before.len(), after.len());
    assert_eq!(after, before, "NatBox adds no trusted GlobalId identity");

    let law = env.globals[&format!("{GEOMETRY}.manhattan_box_diameter")];
    assert!(owned.contains(&law), "diameter law is NatBox-owned");
    assert!(
        matches!(env.env.lookup(law), Some(Decl::Transparent { .. })),
        "diameter proof body is checked and transparent"
    );
    env.elaborate_file(GENERIC_CLIENT)
        .expect("independent generic client checks the full public theorem");

    for private in [
        "axis_distance",
        "axis_distance_within",
        "axis_sub_within",
        "axis_distance_bounded",
    ] {
        let id = env.globals[&format!("{GEOMETRY}.{private}")];
        assert!(owned.contains(&id), "{private} must exist in NatBox");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{private} must have a checked body"
        );
        match env.elaborate_file(&format!("import {GEOMETRY} ({private})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{GEOMETRY}.{private}"));
            }
            other => panic!("{private} must remain private: {other:?}"),
        }
    }
}

const CLOSED_PREFIX: &str = r#"
import Core.Classes.LawfulClasses (leq_nat)
import Algorithm.Geometry.NatBox
  (Point, MkPoint, MkBox, contains, manhattan, manhattan_box_diameter)

const one : Nat = Suc Zero
const two : Nat = Suc one
const three : Nat = Suc two
const four : Nat = Suc three
const five : Nat = Suc four
const six : Nat = Suc five

const lo : Point = MkPoint one one
const hi : Point = MkPoint four three
const inside_left : Point = MkPoint one three
const inside_right : Point = MkPoint four one
const outside_left : Point = MkPoint Zero three

theorem left_is_contained : IsTrue (contains (MkBox lo hi) inside_left) = Proved
theorem right_is_contained : IsTrue (contains (MkBox lo hi) inside_right) = Proved
theorem corner_distance_is_five : Equal Nat (manhattan lo hi) five = Proved
theorem tight_pair_distance_is_five : Equal Nat (manhattan inside_left inside_right) five = Proved

theorem tight_pair_bounded_by_law
    : IsTrue (leq_nat (manhattan inside_left inside_right) (manhattan lo hi)) =
  manhattan_box_diameter lo hi inside_left inside_right left_is_contained right_is_contained

theorem outside_is_excluded
    : Equal Bool (contains (MkBox lo hi) outside_left) False = Proved
theorem outside_pair_distance_is_six
    : Equal Nat (manhattan outside_left inside_right) six = Proved
"#;

const FALSE_OUTSIDE_BOUND: &str = r#"
theorem reject_outside_bound
    : IsTrue (leq_nat (manhattan outside_left inside_right) (manhattan lo hi)) =
  Proved
"#;

const FALSE_OUTSIDE_PREMISE: &str = r#"
theorem reject_outside_premise
    : IsTrue (leq_nat (manhattan outside_left inside_right) (manhattan lo hi)) =
  manhattan_box_diameter lo hi outside_left inside_right Proved right_is_contained
"#;

const FALSE_SMALLER_TIGHT_BOUND: &str = r#"
theorem reject_smaller_tight_bound
    : IsTrue (leq_nat (Suc (manhattan inside_left inside_right)) (manhattan lo hi)) =
  Proved
"#;

fn refuse_at_own_theorem(env: &mut ElabEnv, name: &str, false_claim: &str) {
    match env.elaborate_file(false_claim) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        }) => {
            let start = false_claim
                .find(&format!("theorem {name}"))
                .expect("isolated false theorem follows its positive prefix");
            assert!(
                span.start >= start && span.start < span.end && span.end <= false_claim.len(),
                "{name}: kernel refusal must target its own theorem: {span:?}"
            );
        }
        other => panic!("{name} must refuse with kernel TypeMismatch: {other:?}"),
    }
}

/// SOURCE: spec/50-stdlib/67-geometry.md §10; package-owned closed client.
/// PROMISE CLASS: durable invariant.
/// MEASURED: an accepted client independently proves the box memberships,
/// both distance-five computations, the application of the public law, and
/// the outside membership False and distance six. Each false bound or premise
/// is attempted after a fresh full accepted prefix and kernel-rejected with
/// TypeMismatch at its own theorem, including a below-five bound on the tight
/// pair.
/// CLAIMED: the upper bound is attained by a contained pair but cannot
/// certify an outside pair or a strictly smaller universal bound.
/// THE GAP: these finite cases supplement the checked generic proof. CV's
/// not-yet-authored seed and official conformance execution are separate.
#[test]
fn tight_pair_accepts_and_outside_or_smaller_bounds_refuse() {
    let mut positive = loaded_geometry();
    positive
        .elaborate_file(CLOSED_PREFIX)
        .expect("all independent tight and outside positive controls check");
    for (name, negative) in [
        ("reject_outside_bound", FALSE_OUTSIDE_BOUND),
        ("reject_outside_premise", FALSE_OUTSIDE_PREMISE),
        ("reject_smaller_tight_bound", FALSE_SMALLER_TIGHT_BOUND),
    ] {
        let mut isolated = loaded_geometry();
        isolated
            .elaborate_file(CLOSED_PREFIX)
            .expect("complete accepted client checks before each false theorem");
        refuse_at_own_theorem(&mut isolated, name, negative);
    }
}
