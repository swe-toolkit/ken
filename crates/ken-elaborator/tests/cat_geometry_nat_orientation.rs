//! Natural-point orientation: spec/50-stdlib/67-geometry.md §§18–19.
//! Promise class: durable invariant. Checked public laws and independently
//! reduced closed values distinguish shared-axis collinearity from an off-line
//! triple; false claims reject only after complete accepted prefixes.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{Decl, GlobalId, KernelError, Term};

const GEOMETRY: &str = "Algorithm.Geometry.NatOrientation";
const ACCEPTED: &str = include_str!("fixtures/geo_orient/accepted.ken");

const AXIS_ACCEPTED: &str = r#"import Algorithm.Geometry.NatOrientation (orient, orient_shared_x, orient_shared_y)
import Algorithm.Geometry.NatBox (Point, MkPoint)
import Core.Logic.OrdResult (OrdResult, Lt, Eq, Gt)

const n0 : Nat = Zero
const n1 : Nat = Suc n0
const n2 : Nat = Suc n1
const n3 : Nat = Suc n2
const n4 : Nat = Suc n3
const n5 : Nat = Suc n4
const n6 : Nat = Suc n5
const n7 : Nat = Suc n6
const n8 : Nat = Suc n7

theorem shared_x_closed
  : Equal OrdResult (orient (MkPoint n2 n1) (MkPoint n2 n3) (MkPoint n2 n7)) Eq = Proved

theorem shared_y_closed
  : Equal OrdResult (orient (MkPoint n1 n4) (MkPoint n3 n4) (MkPoint n8 n4)) Eq = Proved

theorem shared_x_law
  : Equal OrdResult (orient (MkPoint n2 n1) (MkPoint n2 n3) (MkPoint n2 n7)) Eq =
  orient_shared_x n2 n1 n3 n7

theorem shared_y_law
  : Equal OrdResult (orient (MkPoint n1 n4) (MkPoint n3 n4) (MkPoint n8 n4)) Eq =
  orient_shared_y n4 n1 n3 n8

theorem off_line_lt
  : Equal OrdResult (orient (MkPoint n2 n1) (MkPoint n2 n3) (MkPoint n3 n7)) Lt = Proved

theorem open_shared_x (x : Nat) (p : Nat) (q : Nat) (r : Nat)
  : Equal OrdResult (orient (MkPoint x p) (MkPoint x q) (MkPoint x r)) Eq =
  orient_shared_x x p q r

theorem open_shared_y (y : Nat) (p : Nat) (q : Nat) (r : Nat)
  : Equal OrdResult (orient (MkPoint p y) (MkPoint q y) (MkPoint r y)) Eq =
  orient_shared_y y p q r
"#;

const AXIS_R1: &str = r#"
theorem refuse_off_line_eq
  : Equal OrdResult (orient (MkPoint n2 n1) (MkPoint n2 n3) (MkPoint n3 n7)) Eq = Proved
"#;

const AXIS_R2: &str = r#"
theorem refuse_off_line_law
  : Equal OrdResult (orient (MkPoint n2 n1) (MkPoint n2 n3) (MkPoint n3 n7)) Eq =
  orient_shared_x n2 n1 n3 n7
"#;

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
/// applications of the four §18 laws and both §19 shared-axis laws. The
/// §19 client also checks two generic open law applications and an off-line
/// Lt value independently from the true shared-axis Eq values.
/// CLAIMED: the six laws are usable by separate modules, and shared-axis
/// triples are Eq while the selected off-line triple is Lt.
/// THE GAP: these closed cases do not establish generic completeness or a
/// conformance-runner result; the generic law bodies check in the provider.
#[test]
fn natural_orientation_closed_rows_and_all_six_law_applications_check() {
    loaded_orientation()
        .elaborate_file(ACCEPTED)
        .expect("closed cross sums, turns, and all four §18 laws check");
    loaded_orientation()
        .elaborate_file(AXIS_ACCEPTED)
        .expect("whole §19 accepted client: closed Eq/Lt and two open laws check");
}

fn term_contains_const(term: &Term, owner: GlobalId) -> bool {
    let mut pending = vec![term];
    while let Some(term) = pending.pop() {
        if matches!(term, Term::Const { id, .. } if *id == owner) {
            return true;
        }
        pending.extend(term.children());
    }
    false
}

/// SOURCE: spec/50-stdlib/67-geometry.md §19, Architect AC-0.
/// PROMISE CLASS: durable invariant.
/// MEASURED: actual providers root-load first; the exact trusted GlobalId
/// sets before and after NatOrientation load match; both new laws are owned
/// by NatOrientation with checked Transparent bodies. Complete open clients
/// cite each checked owner Const under selective imports.
/// CLAIMED: shared-axis geometry adds zero trust and exports both laws.
/// THE GAP: provider trust remains inherited; clients do not replace the
/// catalog's kernel-checked generic proofs.
#[test]
fn shared_axis_laws_have_checked_owners_and_zero_new_trust() {
    let mut env = ElabEnv::new().expect("fresh elaborator");
    for provider in [
        "Algorithm.Geometry.NatBox",
        "Core.Logic.OrdResult",
        "Core.Logic.Transport",
        "Data.Numeric.Nat.Arithmetic",
        "Data.Numeric.Nat.Order",
    ] {
        env.elaborate_module_from_roots(&[catalog_root()], provider)
            .unwrap_or_else(|error| panic!("{provider} root-loads: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[catalog_root()], GEOMETRY)
        .expect("NatOrientation root-loads after its providers");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    assert_eq!(after, before, "orientation adds no trusted identity");
    env.elaborate_file(AXIS_ACCEPTED)
        .expect("complete selective-imported client checks");
    for law in ["orient_shared_x", "orient_shared_y"] {
        let owner = env.globals[&format!("{GEOMETRY}.{law}")];
        assert!(owned.contains(&owner), "{law} is owned by NatOrientation");
        assert!(
            matches!(env.env.lookup(owner), Some(Decl::Transparent { .. })),
            "{law} is a checked Transparent body"
        );
        let client = if law == "orient_shared_x" {
            "open_shared_x"
        } else {
            "open_shared_y"
        };
        let (_, body) = env
            .env
            .transparent_body(env.globals[client])
            .expect("open law client has a checked body");
        assert!(term_contains_const(&body, owner), "{client} cites {law}");
    }
}

/// SOURCE: spec/50-stdlib/67-geometry.md §19, Architect AC-0 helper boundary.
/// PROMISE CLASS: durable invariant.
/// MEASURED: selective import of orient_reorder refuses only the exact
/// qualified UnboundName after a checked provider load.
/// CLAIMED: the shared proof step remains private, not an extra public law.
/// THE GAP: this guards its import visibility, not its internal use by proofs.
#[test]
fn orient_reorder_stays_private_to_the_checked_package() {
    let mut env = loaded_orientation();
    match env.elaborate_file("import Algorithm.Geometry.NatOrientation (orient_reorder)") {
        Err(ElabError::UnboundName { name, .. }) => {
            assert_eq!(name, format!("{GEOMETRY}.orient_reorder"))
        }
        other => panic!("orient_reorder must refuse selective import: {other:?}"),
    }
}

fn refuses_only_at_added_theorem(accepted: &str, label: &str, appended: &str) {
    let mut env = loaded_orientation();
    env.elaborate_file(accepted)
        .expect("fresh complete accepted prefix must check before refusal");

    let candidate = format!("{accepted}{appended}");
    let theorem_start = candidate
        .find(&format!("theorem {label}"))
        .expect("candidate contains its added theorem");
    assert!(
        theorem_start >= accepted.len(),
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
        ACCEPTED,
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
        ACCEPTED,
        "refuse_reverse_as_rotation",
        "\ntheorem refuse_reverse_as_rotation : Equal OrdResult (orient p00 p10 p01) (orient p10 p00 p01) = Proved\n",
    );
}

/// SOURCE: spec/50-stdlib/67-geometry.md §19, Architect D0 refusal r1.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the complete accepted §19 client proves this off-line triple
/// is Lt, then a fresh identical accepted prefix rejects an Eq-by-Proved
/// claim at its own appended theorem under KernelRejected(TypeMismatch).
/// CLAIMED: the off-line triple is not collinear merely because its first
/// two points share x. THE GAP: other triples are not exhaustively tested.
#[test]
fn off_line_equal_by_proved_refuses_at_own_theorem() {
    refuses_only_at_added_theorem(AXIS_ACCEPTED, "refuse_off_line_eq", AXIS_R1);
}

/// SOURCE: spec/50-stdlib/67-geometry.md §19, Architect D0 refusal r2.
/// PROMISE CLASS: durable invariant.
/// MEASURED: a fresh complete accepted §19 client precedes the same false
/// Eq claim by the shared-x law; KernelRejected(TypeMismatch) lies in the
/// appended theorem and not in the accepted prefix.
/// CLAIMED: the law's third-point x equality is load-bearing.
/// THE GAP: one nondegenerate triple supplements the generic checked type.
#[test]
fn off_line_equal_by_law_refuses_at_own_theorem() {
    refuses_only_at_added_theorem(AXIS_ACCEPTED, "refuse_off_line_law", AXIS_R2);
}
