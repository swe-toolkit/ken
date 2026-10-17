//! Manhattan metric: spec/50-stdlib/67-geometry.md §2.
//! The two closed conformance refusals and the D0 identity refusal each
//! run after the same checked positive client. Promise class: durable invariant.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{Decl, KernelError};

const GEOMETRY: &str = "Algorithm.Geometry.NatBox";
const SEED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../conformance/stdlib/geometry/seed-manhattan-metric.md"
));

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn seed_programs() -> Vec<String> {
    let mut programs = SEED
        .split("\n```ken\n")
        .skip(1)
        .map(|fence| {
            fence
                .split_once("\n```")
                .expect("closed Ken fence")
                .0
                .to_owned()
        })
        .collect::<Vec<_>>();
    // The isolated compiler base already binds the canonical IsTrue identity.
    // Omit only the redundant seed import, retaining its proof propositions.
    let redundant = "import Core.Classes.LawfulClasses (IsTrue, leq_nat)";
    assert_eq!(programs[0].matches(redundant).count(), 1);
    programs[0] = programs[0].replacen(redundant, "import Core.Classes.LawfulClasses (leq_nat)", 1);
    programs
}

fn geometry_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], GEOMETRY)
        .expect("NatBox §2 roots-loads with checked providers");
    env
}

// The client states the §2 theorem interfaces with arbitrary point values,
// rather than merely asserting that the closed seed's propositions check.
const GENERIC_CLIENT: &str = r#"
import Core.Classes.LawfulClasses (leq_nat)
import Data.Numeric.Nat.Arithmetic (add)
import Algorithm.Geometry.NatBox
  (Point, manhattan, manhattan_triangle, manhattan_zero_equal)

theorem generic_triangle (p : Point) (q : Point) (r : Point) :
  IsTrue (leq_nat (manhattan p r) (add (manhattan p q) (manhattan q r))) =
  manhattan_triangle p q r

theorem generic_zero_equal (p : Point) (q : Point)
    (h : Equal Nat (manhattan p q) Zero) : Equal Point p q =
  manhattan_zero_equal p q h
"#;

/// SOURCE: spec/50-stdlib/67-geometry.md §2.1–2.2.
/// PROMISE CLASS: durable invariant.
/// MEASURED: after loading the four explicit providers, NatBox adds zero
/// trusted GlobalId identities; both law bodies belong to that module and
/// are transparent, and a separate public client restates their generic
/// types. Four chosen private per-axis imports are refused by the loader.
/// CLAIMED: the two metric laws are proved in the catalog without growing
/// trust or exporting the per-axis implementation to clients.
/// THE GAP: the provider assumptions remain inherited, while privacy of
/// other helpers is source-reviewed rather than established by four probes.
#[test]
fn metric_laws_are_generic_checked_and_trust_set_is_unchanged() {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        "Core.Classes.LawfulClasses",
        "Core.Logic.Transport",
        "Data.Numeric.Nat.Arithmetic",
        "Data.Numeric.Nat.Order",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider} roots-loads: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], GEOMETRY)
        .expect("metric source and both generic proofs kernel-check");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "trusted_base() count: before={}, after={}",
        before.len(),
        after.len()
    );
    assert_eq!(after, before, "metric source adds no trusted GlobalId");

    for law in ["manhattan_triangle", "manhattan_zero_equal"] {
        let id = env.globals[&format!("{GEOMETRY}.{law}")];
        assert!(owned.contains(&id), "{law} must belong to NatBox");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{law} must contain a checked proof body"
        );
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("public clients can consume both §2 laws at arbitrary points");

    for private in [
        "axis_distance",
        "axis_distance_triangle",
        "axis_distance_zero",
        "sub_triangle",
    ] {
        match env.elaborate_file(&format!("import {GEOMETRY} ({private})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{GEOMETRY}.{private}"));
            }
            other => panic!("{private} must remain private: {other:?}"),
        }
    }
}

/// SOURCE: conformance/stdlib/geometry/seed-manhattan-metric.md positives.
/// PROMISE CLASS: durable invariant.
/// MEASURED: every one of the eleven separately named seed proofs checks
/// from public client imports, including direct=2, leg lengths 5 and 3,
/// strict detour sum=8, reverse False, tight detour and distinct distance=2.
/// CLAIMED: the closed arithmetic discriminates both inequality directions,
/// and the metric law instances are usable outside the owning module.
/// THE GAP: the fixed vectors do not prove universal laws; the generic
/// theorem bodies and independent client types are checked above.
#[test]
fn metric_seed_clients_check_strict_and_tight_detours() {
    let programs = seed_programs();
    assert_eq!(
        programs.len(),
        3,
        "one positive and two independent refusals"
    );
    let mut env = geometry_env();
    env.elaborate_file(&programs[0])
        .expect("positive conformance client imports and checks §2 metrics");
    for name in [
        "direct_distance",
        "first_detour_leg",
        "second_detour_leg",
        "detour_distance",
        "strict_detour_bounds_direct",
        "strict_detour_does_not_bound_reverse",
        "monotone_detour_is_tight",
        "distinct_points_are_apart",
        "seed_law_triangle",
        "seed_law_zero_equal",
        "client_zero_equal",
    ] {
        let id = env.globals[name];
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{name} must have a kernel-checked body"
        );
    }
}

// Architect's checked D0 negative; the landed CV seed contains only the
// reverse-detour and distinct-at-zero refusals. This third case exercises
// the zero-distance premise of the public identity law on the same pair.
const REJECT_IDENTITY_ON_DISTINCT: &str = r#"
theorem reject_identity_on_distinct :
  Equal Point distinct_a distinct_b =
  manhattan_zero_equal distinct_a distinct_b Proved
"#;

/// SOURCE: conformance/stdlib/geometry/seed-manhattan-metric.md negatives;
/// Architect D0 evt_1nzebfnfzwq0, third identity-on-distinct refusal.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the seed's eleven positives load before each rejection. Then
/// the two seed negatives plus one D0 same-pair false identity theorem each
/// fail at their own proof with KernelRejected(TypeMismatch), not with an
/// import or parse error. The direct and reverse assertions provide positive
/// controls against an inert or accidentally reversed metric.
/// CLAIMED: the reverse bound, zero distance on distinct points, and point
/// identity derived with a fabricated zero-distance premise are unprovable.
/// THE GAP: those negatives cover only these closed points; the generic laws
/// are separately checked above, and QA owns the operation-only AC-3 mutant.
#[test]
fn three_false_metric_claims_are_kernel_refused_at_their_own_theorems() {
    let programs = seed_programs();
    assert_eq!(programs.len(), 3, "two seed refusals after positive client");
    for (negative, label) in [
        (programs[1].as_str(), "reverse detour"),
        (programs[2].as_str(), "distinct points at zero"),
        (REJECT_IDENTITY_ON_DISTINCT, "identity on distinct points"),
    ] {
        let mut env = geometry_env();
        env.elaborate_file(&programs[0])
            .expect("same positive client must check before each refusal");
        match env.elaborate_file(negative) {
            Err(ElabError::KernelRejected {
                error: KernelError::TypeMismatch { .. },
                span,
            }) => {
                let start = negative
                    .find("theorem reject_")
                    .expect("negative theorem declaration");
                assert!(
                    span.start >= start && span.end <= negative.len() && span.start < span.end,
                    "{label}: K0004 must target the false theorem: {span:?}"
                );
            }
            other => panic!("{label}: expected kernel TypeMismatch, got {other:?}"),
        }
    }
}
