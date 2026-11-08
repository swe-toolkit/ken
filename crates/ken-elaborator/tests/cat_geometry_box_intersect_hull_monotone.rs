//! Corner-wise monotonicity of intersection and hull: Spec 67 §17.
//! Promise class: durable invariant. The CV seed is authored separately.

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
        .expect("NatBox root-loads with both monotone laws");
    env
}

const CLIENT: &str = r#"
import Algorithm.Geometry.NatBox (Box, MkPoint, MkBox, inside, intersect, hull, intersect_monotone, hull_monotone)

theorem generic_meet (a : Box) (b : Box) (a2 : Box) (b2 : Box) : IsTrue (inside a a2) → IsTrue (inside b b2) → IsTrue (inside (intersect a b) (intersect a2 b2)) =
  intersect_monotone a b a2 b2

theorem generic_join (a : Box) (b : Box) (a2 : Box) (b2 : Box) : IsTrue (inside a a2) → IsTrue (inside b b2) → IsTrue (inside (hull a b) (hull a2 b2)) =
  hull_monotone a b a2 b2

const one : Nat = Suc Zero
const two : Nat = Suc one
const three : Nat = Suc two
const small : Box = MkBox (MkPoint one one) (MkPoint two two)
const big : Box = MkBox (MkPoint Zero Zero) (MkPoint three three)
const tall : Box = MkBox (MkPoint Zero Zero) (MkPoint three two)

theorem small_in_big : IsTrue (inside small big) = Proved
theorem small_in_tall : IsTrue (inside small tall) = Proved
theorem closed_meet : IsTrue (inside (intersect small small) (intersect big tall)) = intersect_monotone small small big tall small_in_big small_in_tall
theorem closed_join : IsTrue (inside (hull small small) (hull big tall)) = hull_monotone small small big tall small_in_big small_in_tall
"#;

/// SOURCE: Spec 67 §17 and the Architect's AC-0 public law types.
/// PROMISE CLASS: durable invariant.
/// MEASURED: NatBox root-loads after five providers with the exact trusted
/// GlobalId identity set unchanged; both laws are NatBox-owned Transparent
/// definitions, and a selective-import generic client consumes both exact
/// curried types alongside independently proved closed premises.
/// CLAIMED: intersection and hull are monotone under the two ordered inside
/// premises, without a validity premise or newly trusted assumption.
/// THE GAP: inherited provider trust is not erased; finite closed clients
/// do not replace the checked generic proofs or official CV seed execution.
#[test]
fn generic_monotone_laws_are_owned_transparent_and_trust_free() {
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
        .expect("NatBox proves both monotone laws");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "NatBox monotone trusted GlobalId identity sets: {} → {}",
        before.len(),
        after.len()
    );
    assert_eq!(after, before, "NatBox adds no trusted GlobalId identity");
    for law in ["intersect_monotone", "hull_monotone"] {
        let id = env.globals[&format!("{GEOMETRY}.{law}")];
        assert!(owned.contains(&id), "{law} must be NatBox-owned");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{law} must have a checked transparent body"
        );
    }
    env.elaborate_file(CLIENT)
        .expect("open and closed clients consume both exact public laws");
}

const ACCEPTED_PREFIX: &str = r#"
import Algorithm.Geometry.NatBox (Box, MkPoint, MkBox, inside, intersect, hull, intersect_monotone, hull_monotone)

const one : Nat = Suc Zero
const two : Nat = Suc one
const three : Nat = Suc two
const small : Box = MkBox (MkPoint one one) (MkPoint two two)
const point : Box = MkBox (MkPoint one one) (MkPoint one one)
const big : Box = MkBox (MkPoint Zero Zero) (MkPoint three three)
const tall : Box = MkBox (MkPoint Zero Zero) (MkPoint three two)
const short : Box = MkBox (MkPoint Zero Zero) (MkPoint three one)
const point_outer : Box = MkBox (MkPoint Zero Zero) (MkPoint one one)

theorem small_in_big : IsTrue (inside small big) = Proved
theorem small_in_tall : IsTrue (inside small tall) = Proved
theorem point_in_outer : IsTrue (inside point point_outer) = Proved
theorem meet_control : IsTrue (inside (intersect small small) (intersect big tall)) = intersect_monotone small small big tall small_in_big small_in_tall
theorem join_control : IsTrue (inside (hull small point) (hull tall point_outer)) = hull_monotone small point tall point_outer small_in_tall point_in_outer
theorem small_not_in_short : Equal Bool (inside small short) False = Proved
theorem meet_wrong_conclusion : Equal Bool (inside (intersect small small) (intersect big short)) False = Proved
theorem join_wrong_conclusion : Equal Bool (inside (hull small point) (hull short point_outer)) False = Proved
"#;

const REFUSE_MEET: &str = r#"
theorem meet_wrong_second_premise : IsTrue (inside (intersect small small) (intersect big short)) = intersect_monotone small small big short small_in_big small_in_tall
"#;
const REFUSE_JOIN: &str = r#"
theorem join_wrong_first_premise : IsTrue (inside (hull small point) (hull short point_outer)) = hull_monotone small point short point_outer small_in_tall point_in_outer
"#;

/// SOURCE: Spec 67 §17's two ordered corner-wise containment premises.
/// PROMISE CLASS: durable invariant.
/// MEASURED: positive controls apply each law; independent Bool equalities
/// establish that each wrong conclusion is False and `small` is not inside
/// `short`. Fresh accepted prefixes precede each wrong-premise application,
/// which refuses with a kernel TypeMismatch at its own theorem.
/// CLAIMED: neither law can use a proof of `inside small tall` as evidence of
/// `inside small short`, even where every other premise is checked.
/// THE GAP: two closed counterexamples are not an exhaustive theorem and do
/// not replace the CV-owned separate seed or official conformance execution.
#[test]
fn monotone_laws_require_each_live_premise_on_fresh_accepted_prefixes() {
    loaded_geometry()
        .elaborate_file(ACCEPTED_PREFIX)
        .expect("both controls and independently false conclusions check");
    for (name, claim) in [
        ("meet_wrong_second_premise", REFUSE_MEET),
        ("join_wrong_first_premise", REFUSE_JOIN),
    ] {
        let mut env = loaded_geometry();
        env.elaborate_file(ACCEPTED_PREFIX)
            .expect("fresh accepted prefix checks before each refusal");
        match env.elaborate_file(claim) {
            Err(ElabError::KernelRejected {
                error: KernelError::TypeMismatch { .. },
                span,
            }) => {
                let own = claim.find(&format!("theorem {name}")).expect("own theorem");
                assert!(
                    span.start >= own && span.start < span.end && span.end <= claim.len(),
                    "{name} must refuse at its own law application: {span:?}"
                );
            }
            other => panic!("{name} must refuse at its own law application: {other:?}"),
        }
    }
}
