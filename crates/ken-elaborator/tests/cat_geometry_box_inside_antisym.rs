//! Corner-wise box inclusion antisymmetry: spec/50-stdlib/67-geometry.md §12.
//! Promise class: durable invariant. CV owns the separate seed.

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
        .expect("NatBox and §12 checked proofs root-load");
    env
}

const GENERIC_CLIENT: &str = r#"
import Algorithm.Geometry.NatBox (Box, inside, inside_antisym, inside_refl)

theorem client_antisym (a : Box) (b : Box)
    : IsTrue (inside a b) → IsTrue (inside b a) → Equal Box a b =
  inside_antisym a b

theorem client_self (a : Box) : Equal Box a a =
  inside_antisym a a (inside_refl a) (inside_refl a)
"#;

/// SOURCE: spec/50-stdlib/67-geometry.md §12; Architect AC-0 public type.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the five provider modules load before NatBox; exact trusted
/// GlobalId identities stay equal after NatBox loads. A generic public client
/// checks both curried premises and the structural equality result; the law
/// is NatBox-owned and transparent. Private assembly and axis helpers cannot
/// be selectively imported.
/// CLAIMED: mutual corner-wise inclusion yields structural box equality
/// without a validity premise or a new trust assumption.
/// THE GAP: the provider closure retains inherited trust; this local package
/// check is not an official conformance-runner execution or landed result.
#[test]
fn generic_antisym_is_checked_trust_free_and_keeps_helpers_private() {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        "Core.Classes.LawfulClasses",
        "Core.Logic.Transport",
        "Core.Logic.Conjunction",
        "Data.Numeric.Nat.Arithmetic",
        "Data.Numeric.Nat.Order",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider} root-loads: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], GEOMETRY)
        .expect("NatBox §12 proofs kernel-check");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!("§12 trusted GlobalIds: {} → {}", before.len(), after.len());
    assert_eq!(after, before, "NatBox adds no trusted GlobalId identity");

    let law = env.globals[&format!("{GEOMETRY}.inside_antisym")];
    assert!(owned.contains(&law), "inside_antisym must be NatBox-owned");
    assert!(
        matches!(env.env.lookup(law), Some(Decl::Transparent { .. })),
        "inside_antisym must have a checked transparent body"
    );
    env.elaborate_file(GENERIC_CLIENT)
        .expect("generic client checks the exact curried public law");

    for private in [
        "axis_within_antisym_low",
        "axis_within_antisym_high",
        "box_equal",
        "point_equal",
        "axis_within",
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
import Algorithm.Geometry.NatBox (Box, MkPoint, MkBox, inside, inside_antisym)

const one : Nat = Suc Zero
const two : Nat = Suc one
const three : Nat = Suc two

const small : Box = MkBox (MkPoint one one) (MkPoint two two)
const small_again : Box = MkBox (MkPoint one one) (MkPoint two two)
const big : Box = MkBox (MkPoint Zero Zero) (MkPoint three three)
const inv_a : Box = MkBox (MkPoint three three) (MkPoint one one)
const inv_b : Box = MkBox (MkPoint two two) (MkPoint Zero Zero)

theorem small_in_small : IsTrue (inside small small_again) = Proved
theorem small_in_small_rev : IsTrue (inside small_again small) = Proved
theorem small_equal_by_law : Equal Box small small_again =
  inside_antisym small small_again small_in_small small_in_small_rev
theorem small_in_big : IsTrue (inside small big) = Proved
theorem big_not_in_small : Equal Bool (inside big small) False = Proved
theorem inv_a_not_in_inv_b : Equal Bool (inside inv_a inv_b) False = Proved
theorem inv_b_not_in_inv_a : Equal Bool (inside inv_b inv_a) False = Proved
theorem inv_a_self_equal : Equal Box inv_a inv_a = Refl
"#;

const REFUSE_REVERSE: &str = r#"
theorem reject_reverse : IsTrue (inside big small) = Proved
"#;
const REFUSE_ONE_PREMISE: &str = r#"
theorem reject_one_premise : Equal Box small big =
  inside_antisym small big small_in_big Proved
"#;
const REFUSE_INVERTED_EQUALITY: &str = r#"
theorem reject_inverted_equality : Equal Box inv_a inv_b = Refl
"#;

fn refuse_at_own_theorem(env: &mut ElabEnv, name: &str, false_claim: &str) {
    match env.elaborate_file(false_claim) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        }) => {
            let start = false_claim
                .find(&format!("theorem {name}"))
                .expect("isolated false theorem follows its accepted prefix");
            assert!(
                span.start >= start && span.start < span.end && span.end <= false_claim.len(),
                "{name}: kernel refusal must target its own theorem: {span:?}"
            );
        }
        Err(ElabError::TypeMismatch { span, reason }) if name == "reject_inverted_equality" => {
            let proof = false_claim.find("Refl").expect("false equality uses Refl");
            assert_eq!(span.start..span.end, proof..proof + "Refl".len());
            assert!(
                reason.contains("Refl expects an `Eq`-shaped goal"),
                "distinct stored corners must refuse Refl: {reason}"
            );
        }
        other => panic!("{name} must refuse at its own false claim: {other:?}"),
    }
}

/// SOURCE: spec/50-stdlib/67-geometry.md §12; package-owned closed client.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the accepted prefix proves both directions for equal stored
/// corners, applies the public law, proves the small box is inside the big
/// box, computes the reverse inclusion and both inverted directions as False,
/// then proves a same-form reflexive `Equal Box inv_a inv_a` using Refl.
/// Separate fresh prefixes reject the false reverse direction, use of the
/// false second law premise, and `Equal Box` of distinct inverted boxes.
/// CLAIMED: one direction alone cannot show box equality; two boxes with
/// distinct inverted corners do not collapse merely because they are empty.
/// THE GAP: finite cases do not replace the checked generic theorem; the CV
/// seed is a separate turn and these tests are not official conformance.
#[test]
fn closed_mutual_inside_accepts_and_false_reverse_or_inverted_equalities_refuse() {
    loaded_geometry()
        .elaborate_file(CLOSED_PREFIX)
        .expect("closed same-corner proof and independent false controls check");
    for (name, claim) in [
        ("reject_reverse", REFUSE_REVERSE),
        ("reject_one_premise", REFUSE_ONE_PREMISE),
        ("reject_inverted_equality", REFUSE_INVERTED_EQUALITY),
    ] {
        let mut env = loaded_geometry();
        env.elaborate_file(CLOSED_PREFIX)
            .expect("positive client checks before isolated false theorem");
        refuse_at_own_theorem(&mut env, name, claim);
    }
}
