//! Nested-box intersection and hull equations: spec/50-stdlib/67-geometry.md
//! §16, as ruled by the Architect's AC-0. Promise class: durable invariant.
//! The conformance seed is authored separately by the Spec enclave.

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
        .expect("NatBox and both inside equations root-load");
    env
}

const GENERIC_CLIENT: &str = r#"
import Algorithm.Geometry.NatBox (Box, inside, intersect, hull, intersect_eq_of_inside, hull_eq_of_inside)

theorem client_meet (b : Box) (c : Box) : IsTrue (inside b c) → Equal Box (intersect b c) b =
  intersect_eq_of_inside b c

theorem client_join (b : Box) (c : Box) : IsTrue (inside b c) → Equal Box (hull b c) c =
  hull_eq_of_inside b c
"#;

/// SOURCE: Spec 67 §16 and the Architect's ruled AC-0 public law types.
/// PROMISE CLASS: durable invariant.
/// MEASURED: NatBox root-loads after the five providers; the entire trusted
/// GlobalId identity set stays equal. Both law IDs are NatBox-owned checked
/// transparent declarations, and a generic client imports and applies each
/// public law at its exact curried type.
/// CLAIMED: corner-wise containment entails both structural box equations
/// without a validity premise or newly trusted assumption.
/// THE GAP: the provider closure retains inherited trust; the closed clients
/// below test discrimination but do not replace a generic proof or an official
/// conformance-runner execution.
#[test]
fn generic_inside_equations_are_owned_transparent_and_trust_free() {
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
        .expect("NatBox proves both inside equations");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "NatBox trusted GlobalId identity sets: {} → {}",
        before.len(),
        after.len()
    );
    assert_eq!(after, before, "NatBox adds no trusted GlobalId identity");

    for law in ["intersect_eq_of_inside", "hull_eq_of_inside"] {
        let id = env.globals[&format!("{GEOMETRY}.{law}")];
        assert!(owned.contains(&id), "{law} must be NatBox-owned");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{law} must have a checked transparent body"
        );
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("open client checks both exact curried public laws");
}

const CLOSED_PREFIX: &str = r#"
import Algorithm.Geometry.NatBox (Box, MkPoint, MkBox, inside, intersect, hull, intersect_eq_of_inside, hull_eq_of_inside)

const one : Nat = Suc Zero
const two : Nat = Suc one
const three : Nat = Suc two

const small : Box = MkBox (MkPoint one one) (MkPoint two two)
const big : Box = MkBox (MkPoint Zero Zero) (MkPoint three three)
const tall : Box = MkBox (MkPoint Zero Zero) (MkPoint three two)
const short : Box = MkBox (MkPoint Zero Zero) (MkPoint three one)

theorem small_in_big : IsTrue (inside small big) = Proved
theorem meet_big : Equal Box (intersect small big) small = intersect_eq_of_inside small big small_in_big
theorem join_big : Equal Box (hull small big) big = hull_eq_of_inside small big small_in_big
theorem small_in_tall : IsTrue (inside small tall) = Proved
theorem meet_tall : Equal Box (intersect small tall) small = intersect_eq_of_inside small tall small_in_tall
theorem meet_tall_refl : Equal Box (intersect small tall) small = Refl
theorem small_not_in_short : Equal Bool (inside small short) False = Proved
"#;

const REFUSE_PREMISE: &str = r#"
theorem small_in_short : IsTrue (inside small short) = Proved
"#;
const REFUSE_EQUATION: &str = r#"
theorem meet_short_refl : Equal Box (intersect small short) small = Refl
"#;

fn refuse_at_own_theorem(env: &mut ElabEnv, name: &str, claim: &str) {
    match env.elaborate_file(claim) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        }) if name == "small_in_short" => {
            let start = claim
                .find(&format!("theorem {name}"))
                .expect("the false premise has its own theorem");
            assert!(
                span.start >= start && span.start < span.end && span.end <= claim.len(),
                "{name} must be refused at its own claim: {span:?}"
            );
        }
        Err(ElabError::TypeMismatch { span, reason }) if name == "meet_short_refl" => {
            let proof = claim.find("Refl").expect("false equation uses Refl");
            assert_eq!(span.start..span.end, proof..proof + "Refl".len());
            assert!(
                reason.contains("Refl expects an `Eq`-shaped goal"),
                "the false equation must refuse its own Refl: {reason}"
            );
        }
        other => panic!("{name} must refuse at its own theorem: {other:?}"),
    }
}

/// SOURCE: Spec 67 §16, the Architect's AC-2 discrimination, and the separate
/// conformance seed's accepted/refused shape.
/// PROMISE CLASS: durable invariant.
/// MEASURED: a small box inside a larger one applies both checked laws; a
/// tall outer box allows the meet equation and a direct same-form `Refl`.
/// Lowering only tall's high y corner to one makes inside False; isolated
/// fresh environments reject the false containment premise at its theorem
/// and the false meet equality at its own `Refl` span.
/// CLAIMED: the meet equation needs actual corner-wise containment, not just
/// a syntactically valid law application or equivalence of point sets.
/// THE GAP: finite clients do not establish generic proof; the first test
/// checks the open public theorem, and these are not official conformance.
#[test]
fn nested_boxes_accept_and_one_axis_false_premise_and_equation_refuse() {
    loaded_geometry()
        .elaborate_file(CLOSED_PREFIX)
        .expect("closed cases and the same-form tall control check");
    for (name, claim) in [
        ("small_in_short", REFUSE_PREMISE),
        ("meet_short_refl", REFUSE_EQUATION),
    ] {
        let mut env = loaded_geometry();
        env.elaborate_file(CLOSED_PREFIX)
            .expect("accepted prefix checks before an isolated false theorem");
        refuse_at_own_theorem(&mut env, name, claim);
    }
}
