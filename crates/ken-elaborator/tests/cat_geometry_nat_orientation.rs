//! Natural-point orientation: spec/50-stdlib/67-geometry.md §18 (AC-0 design).
//! Promise class: durable invariant. The closed cross sums and turn directions
//! supplement the checked generic provider; false closed claims reject.

use std::path::PathBuf;

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::KernelError;

const GEOMETRY: &str = "Algorithm.Geometry.NatOrientation";
const ACCEPTED: &str = include_str!("fixtures/geo_orient/accepted.ken");

fn catalog_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages")
}

fn loaded_orientation() -> ElabEnv {
    let mut env = ElabEnv::new().expect("fresh elaborator");
    env.elaborate_module_from_roots(&[catalog_root()], GEOMETRY)
        .expect("checked orientation provider roots-loads");
    env
}

/// SOURCE: spec/50-stdlib/67-geometry.md §18, AC-0 ruled design.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the checked provider roots-loads, and an independent Ken client
/// checks three closed turn directions, their stated cross-sum values and
/// applications of all four exported generic laws.
/// CLAIMED: these public operations agree on the selected cross-sum examples,
/// and the four laws are usable by a separate module.
/// THE GAP: these closed cases do not establish generic completeness or a
/// conformance-runner result; the generic law bodies check in the provider.
#[test]
fn natural_orientation_closed_rows_and_all_four_law_applications_check() {
    loaded_orientation()
        .elaborate_file(ACCEPTED)
        .expect("closed cross sums, turns, and all four law applications check");
}

fn refuses_only_at_added_theorem(label: &str, appended: &str) {
    let mut env = loaded_orientation();
    env.elaborate_file(ACCEPTED)
        .expect("fresh complete accepted prefix must check before refusal");

    let candidate = format!("{ACCEPTED}{appended}");
    let theorem_start = candidate
        .find(&format!("theorem {label}"))
        .expect("candidate contains its added theorem");
    assert!(
        theorem_start >= ACCEPTED.len(),
        "false theorem must follow the accepted prefix"
    );
    // A fresh elaborator for every twin prevents one refusal's declarations
    // or failed elaboration from affecting the other twin.
    let mut fresh = loaded_orientation();
    match fresh.elaborate_file(&candidate) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        }) if span.start >= theorem_start
            && span.start < span.end
            && span.end <= candidate.len() => {}
        other => panic!("{label} must kernel-refuse at its added theorem: {other:?}"),
    }
}

/// SOURCE: spec/50-stdlib/67-geometry.md §18, AC-0 negative controls.
/// PROMISE CLASS: durable invariant.
/// MEASURED: a new client first checks the entire accepted prefix, then a
/// fresh copy kernel-refuses a wrong counter-clockwise claim at the new span.
/// CLAIMED: a wrong turn cannot be certified after the correct closed proofs.
/// THE GAP: only this specified nondegenerate triple is exercised.
#[test]
fn wrong_counterclockwise_turn_refuses_after_fresh_accepted_prefix() {
    refuses_only_at_added_theorem(
        "refuse_ccw_as_lt",
        "\ntheorem refuse_ccw_as_lt : Equal OrdResult (orient p00 p10 p01) Lt = Proved\n",
    );
}

/// SOURCE: spec/50-stdlib/67-geometry.md §18, AC-0 negative controls.
/// PROMISE CLASS: durable invariant.
/// MEASURED: a new client first checks the entire accepted prefix, then a
/// fresh copy kernel-refuses a non-cyclic swap masquerading as rotation.
/// CLAIMED: swapping two points cannot certify the cyclic-turn law.
/// THE GAP: only this specified nondegenerate triple is exercised.
#[test]
fn noncyclic_rotation_refuses_after_fresh_accepted_prefix() {
    refuses_only_at_added_theorem(
        "refuse_reverse_as_rotation",
        "\ntheorem refuse_reverse_as_rotation : Equal OrdResult (orient p00 p10 p01) (orient p10 p00 p01) = Proved\n",
    );
}
