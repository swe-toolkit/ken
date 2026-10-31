//! Contained-box corners: spec/50-stdlib/67-geometry.md §11.
//! Promise class: durable invariant. A package-owned client checks the two
//! generic public laws and closed witness/refusal controls. CV owns the seed.

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
        .expect("NatBox and §11 checked proofs root-load");
    env
}

const GENERIC_CLIENT: &str = r#"
import Algorithm.Geometry.NatBox
  (Point, MkBox, contains, contains_lo_corner, contains_hi_corner)

theorem generic_lo_corner
      (lo : Point) (hi : Point) (p : Point)
    : IsTrue (contains (MkBox lo hi) p)
      → IsTrue (contains (MkBox lo hi) lo) =
  contains_lo_corner lo hi p

theorem generic_hi_corner
      (lo : Point) (hi : Point) (p : Point)
    : IsTrue (contains (MkBox lo hi) p)
      → IsTrue (contains (MkBox lo hi) hi) =
  contains_hi_corner lo hi p
"#;

/// SOURCE: spec/50-stdlib/67-geometry.md §11.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the four provider modules root-load before NatBox. Loading NatBox
/// leaves their exact trusted GlobalId set unchanged. A public generic client
/// checks both curried witness types, their owned Transparent bodies, and both
/// private per-axis helpers' refusal on selective import.
/// CLAIMED: the two universal corner-membership laws add no new trust or
/// exposed per-axis proof scaffolding.
/// THE GAP: inherited provider trust remains; closed witnesses below do not
/// establish universal quantification without the checked generic bodies.
#[test]
fn generic_corner_laws_are_checked_without_new_trust_or_public_helpers() {
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
        .expect("NatBox §11 proofs kernel-check");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!("§11 trusted GlobalIds: {} → {}", before.len(), after.len());
    assert_eq!(after, before, "NatBox adds no trusted GlobalId identity");

    for law in ["contains_lo_corner", "contains_hi_corner"] {
        let id = env.globals[&format!("{GEOMETRY}.{law}")];
        assert!(owned.contains(&id), "{law} must be NatBox-owned");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{law} must have a checked transparent body"
        );
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("independent generic client checks both exact public law types");

    for private in ["axis_corner_low", "axis_corner_high"] {
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
import Algorithm.Geometry.NatBox
  (Point, MkPoint, MkBox, contains, contains_lo_corner, contains_hi_corner)

const one : Nat = Suc Zero
const two : Nat = Suc one
const three : Nat = Suc two
const four : Nat = Suc three

const lo : Point = MkPoint one one
const hi : Point = MkPoint four three
const witness : Point = MkPoint two two
const inverted_lo : Point = MkPoint three three
const inverted_hi : Point = MkPoint one one
const inverted_witness : Point = MkPoint two two
const outside : Point = MkPoint Zero one

theorem witness_is_contained : IsTrue (contains (MkBox lo hi) witness) = Proved
theorem lo_is_contained_independently : IsTrue (contains (MkBox lo hi) lo) = Proved
theorem hi_is_contained_independently : IsTrue (contains (MkBox lo hi) hi) = Proved

theorem lo_is_contained_by_law : IsTrue (contains (MkBox lo hi) lo) =
  contains_lo_corner lo hi witness witness_is_contained
theorem hi_is_contained_by_law : IsTrue (contains (MkBox lo hi) hi) =
  contains_hi_corner lo hi witness witness_is_contained

theorem inverted_lo_is_excluded :
  Equal Bool (contains (MkBox inverted_lo inverted_hi) inverted_lo) False = Proved
theorem inverted_witness_is_excluded :
  Equal Bool (contains (MkBox inverted_lo inverted_hi) inverted_witness) False = Proved
theorem outside_is_excluded :
  Equal Bool (contains (MkBox lo hi) outside) False = Proved
"#;

const FALSE_INVERTED_LO: &str = r#"
theorem reject_inverted_lo :
  IsTrue (contains (MkBox inverted_lo inverted_hi) inverted_lo) = Proved
"#;

const FALSE_INVERTED_PREMISE: &str = r#"
theorem reject_inverted_witness :
  IsTrue (contains (MkBox inverted_lo inverted_hi) inverted_lo) =
  contains_lo_corner inverted_lo inverted_hi inverted_witness Proved
"#;

const FALSE_OUTSIDE_PREMISE: &str = r#"
theorem reject_outside_witness :
  IsTrue (contains (MkBox lo hi) lo) =
  contains_lo_corner lo hi outside Proved
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

/// SOURCE: spec/50-stdlib/67-geometry.md §11; package-owned closed client.
/// PROMISE CLASS: durable invariant.
/// MEASURED: a fresh accepted client independently proves a witness and both
/// corners of (1,1)–(4,3), then applies both public laws to witness (2,2).
/// The prefix computes false both inverted-box memberships and the outside
/// point's membership. Three separate fresh prefixes each reject at their own
/// theorem with kernel TypeMismatch: inverted own-lo `Proved`, application of
/// the low-corner law at its own conclusion with a false inverted-box witness,
/// and valid-box law application with an outside witness.
/// CLAIMED: a point in a box entails both stored corners are in that box;
/// neither corner is automatically contained in an inverted box, and a false
/// witness cannot justify either law.
/// THE GAP: these finite controls complement, not replace, generic proofs.
/// The separate CV seed is authored but not Ken-checked or officially run;
/// package-owned tests do not establish official conformance-runner execution,
/// CI, publication, landing or landed trust.
#[test]
fn contained_witness_proves_both_corners_but_false_premises_refuse() {
    let mut positive = loaded_geometry();
    positive
        .elaborate_file(CLOSED_PREFIX)
        .expect("independent memberships and both law applications check");
    for (name, negative) in [
        ("reject_inverted_lo", FALSE_INVERTED_LO),
        ("reject_inverted_witness", FALSE_INVERTED_PREMISE),
        ("reject_outside_witness", FALSE_OUTSIDE_PREMISE),
    ] {
        let mut isolated = loaded_geometry();
        isolated
            .elaborate_file(CLOSED_PREFIX)
            .expect("complete accepted client checks before each false theorem");
        refuse_at_own_theorem(&mut isolated, name, negative);
    }
}
