//! Natural-point orientation: spec/50-stdlib/67-geometry.md §§18–19, 21–22.
//! Promise class: durable invariant. Checked public laws and independently
//! reduced closed values distinguish shared-axis collinearity from an off-line
//! triple and positive scaling from collapse to the origin. False claims
//! reject only after complete accepted prefixes.

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

const ORDER: &str = "Data.Numeric.Nat.Order";
const TRANSLATE_OPEN: &str = r#"import Algorithm.Geometry.NatOrientation (orient, orient_translate)
import Algorithm.Geometry.NatBox (MkPoint)
import Core.Logic.OrdResult (OrdResult)
import Data.Numeric.Nat.Arithmetic (add)
import Data.Numeric.Nat.Order (compare)

theorem open_compare_cancel (a : Nat) (b : Nat) (c : Nat)
  : Equal OrdResult (compare (add a c) (add b c)) (compare a b) =
  (proof add_cancel_r for compare) a b c

theorem open_translate
    (xa : Nat) (ya : Nat) (xb : Nat) (yb : Nat) (xc : Nat) (yc : Nat) (dx : Nat) (dy : Nat)
  : Equal OrdResult
      (orient (MkPoint (add xa dx) (add ya dy))
              (MkPoint (add xb dx) (add yb dy))
              (MkPoint (add xc dx) (add yc dy)))
      (orient (MkPoint xa ya) (MkPoint xb yb) (MkPoint xc yc)) =
  orient_translate xa ya xb yb xc yc dx dy
"#;

const TRANSLATE_ACCEPTED: &str = r#"import Algorithm.Geometry.NatOrientation (orient, orient_cross, orient_translate)
import Algorithm.Geometry.NatBox (MkPoint)
import Core.Logic.OrdResult (OrdResult, Lt, Gt)
import Data.Numeric.Nat.Arithmetic (add, mul)
import Data.Numeric.Nat.Order (compare)

const n0 : Nat = Zero
const n1 : Nat = Suc n0
const n2 : Nat = Suc n1
const n3 : Nat = Suc n2
const n4 : Nat = Suc n3
const n5 : Nat = Suc n4

theorem original_gt
  : Equal OrdResult (orient (MkPoint n0 n0) (MkPoint n1 n0) (MkPoint n0 n1)) Gt = Proved

theorem shared_translation_gt
  : Equal OrdResult (orient (MkPoint n2 n3) (MkPoint n3 n3) (MkPoint n2 n4)) Gt = Proved

theorem shared_translation_by_law
  : Equal OrdResult
      (orient (MkPoint n2 n3) (MkPoint n3 n3) (MkPoint n2 n4))
      (orient (MkPoint n0 n0) (MkPoint n1 n0) (MkPoint n0 n1)) =
  orient_translate n0 n0 n1 n0 n0 n1 n2 n3

theorem one_point_left_sum
  : Equal Nat (add (add (mul n2 n0) (mul n1 n1)) (mul n0 n3)) n1 = Proved

theorem one_point_right_sum
  : Equal Nat (add (add (mul n3 n1) (mul n0 n0)) (mul n1 n2)) n5 = Proved

theorem one_point_compare
  : Equal OrdResult (orient (MkPoint n2 n3) (MkPoint n1 n0) (MkPoint n0 n1)) (compare n1 n5) =
  orient_cross n2 n3 n1 n0 n0 n1

theorem one_point_lt
  : Equal OrdResult (orient (MkPoint n2 n3) (MkPoint n1 n0) (MkPoint n0 n1)) Lt = Proved
"#;

const TRANSLATE_REFUSED: &str = r#"
theorem refuse_one_point_translation
  : Equal OrdResult
      (orient (MkPoint n2 n3) (MkPoint n1 n0) (MkPoint n0 n1))
      (orient (MkPoint n0 n0) (MkPoint n1 n0) (MkPoint n0 n1)) = Proved
"#;

const SCALE_IMPORTS: &str = r#"import Algorithm.Geometry.NatOrientation (orient, orient_scale)
import Algorithm.Geometry.NatBox (MkPoint)
import Core.Logic.OrdResult (OrdResult, Eq, Gt)
import Core.Logic.Transport (trans)
import Data.Numeric.Nat.Arithmetic (mul)
import Data.Numeric.Nat.Order (compare)

"#;

const SCALE_COORDINATES: &str = r#"const n0 : Nat = Zero
const n1 : Nat = Suc n0

"#;

const SCALE_OPEN: &str = r#"theorem open_scale
    (xa : Nat) (ya : Nat) (xb : Nat) (yb : Nat) (xc : Nat) (yc : Nat) (k : Nat)
  : Equal OrdResult
      (orient (MkPoint (mul xa (Suc k)) (mul ya (Suc k)))
              (MkPoint (mul xb (Suc k)) (mul yb (Suc k)))
              (MkPoint (mul xc (Suc k)) (mul yc (Suc k))))
      (orient (MkPoint xa ya) (MkPoint xb yb) (MkPoint xc yc)) =
  orient_scale xa ya xb yb xc yc k

theorem open_compare_mul_cancel (a : Nat) (b : Nat) (k : Nat)
  : Equal OrdResult (compare (mul a (Suc k)) (mul b (Suc k))) (compare a b) =
  (proof mul_cancel_r for compare) a b k

"#;

const SCALE_ACCEPTED_ROWS: &str = r#"theorem unit_turn_gt
  : Equal OrdResult (orient (MkPoint n0 n0) (MkPoint n1 n0) (MkPoint n0 n1)) Gt = Proved

theorem doubled_turn_by_law
  : Equal OrdResult
      (orient (MkPoint (mul n0 (Suc n1)) (mul n0 (Suc n1)))
              (MkPoint (mul n1 (Suc n1)) (mul n0 (Suc n1)))
              (MkPoint (mul n0 (Suc n1)) (mul n1 (Suc n1))))
      Gt =
  trans
    OrdResult
    (orient (MkPoint (mul n0 (Suc n1)) (mul n0 (Suc n1)))
            (MkPoint (mul n1 (Suc n1)) (mul n0 (Suc n1)))
            (MkPoint (mul n0 (Suc n1)) (mul n1 (Suc n1))))
    (orient (MkPoint n0 n0) (MkPoint n1 n0) (MkPoint n0 n1))
    Gt
    (orient_scale n0 n0 n1 n0 n0 n1 n1)
    unit_turn_gt

theorem doubled_equation_closed
  : Equal OrdResult
      (orient (MkPoint (mul n0 (Suc n1)) (mul n0 (Suc n1)))
              (MkPoint (mul n1 (Suc n1)) (mul n0 (Suc n1)))
              (MkPoint (mul n0 (Suc n1)) (mul n1 (Suc n1))))
      (orient (MkPoint n0 n0) (MkPoint n1 n0) (MkPoint n0 n1)) =
  Proved

theorem zero_scaled_is_eq
  : Equal OrdResult
      (orient (MkPoint (mul n0 Zero) (mul n0 Zero))
              (MkPoint (mul n1 Zero) (mul n0 Zero))
              (MkPoint (mul n0 Zero) (mul n1 Zero)))
      Eq =
  Proved
"#;

const SCALE_REFUSED_TURN: &str = r#"
theorem zero_turn_by_law
  : Equal OrdResult
      (orient (MkPoint (mul n0 Zero) (mul n0 Zero))
              (MkPoint (mul n1 Zero) (mul n0 Zero))
              (MkPoint (mul n0 Zero) (mul n1 Zero)))
      Gt =
  trans
    OrdResult
    (orient (MkPoint (mul n0 Zero) (mul n0 Zero))
            (MkPoint (mul n1 Zero) (mul n0 Zero))
            (MkPoint (mul n0 Zero) (mul n1 Zero)))
    (orient (MkPoint n0 n0) (MkPoint n1 n0) (MkPoint n0 n1))
    Gt
    (orient_scale n0 n0 n1 n0 n0 n1 n1)
    unit_turn_gt
"#;

const SCALE_REFUSED_EQUATION: &str = r#"
theorem zero_equation_claim
  : Equal OrdResult
      (orient (MkPoint (mul n0 Zero) (mul n0 Zero))
              (MkPoint (mul n1 Zero) (mul n0 Zero))
              (MkPoint (mul n0 Zero) (mul n1 Zero)))
      (orient (MkPoint n0 n0) (MkPoint n1 n0) (MkPoint n0 n1)) =
  Proved
"#;

fn scale_accepted() -> String {
    format!("{SCALE_IMPORTS}{SCALE_COORDINATES}{SCALE_OPEN}{SCALE_ACCEPTED_ROWS}")
}

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

fn proof_uses_order_law(
    env: &ElabEnv,
    geometry_owned: &[GlobalId],
    theorem: GlobalId,
    target: GlobalId,
) -> bool {
    let mut pending = vec![theorem];
    let mut visited = BTreeSet::new();
    while let Some(id) = pending.pop() {
        if !visited.insert(id) {
            continue;
        }
        let Some((_, body)) = env.env.transparent_body(id) else {
            continue;
        };
        let mut terms = vec![&body];
        while let Some(term) = terms.pop() {
            if let Term::Const { id, .. } = term {
                if *id == target {
                    return true;
                }
                if geometry_owned.contains(id) {
                    pending.push(*id);
                }
            }
            terms.extend(term.children());
        }
    }
    false
}

fn owned_open_law(
    env: &ElabEnv,
    package: &str,
    owned: &[GlobalId],
    law: &str,
    client: &str,
) -> GlobalId {
    let owner = env.globals[&format!("{package}.{law}")];
    assert!(owned.contains(&owner), "{law} is owned by {package}");
    assert!(
        matches!(env.env.lookup(owner), Some(Decl::Transparent { .. })),
        "{law} is a checked Transparent body"
    );
    let (_, body) = env
        .env
        .transparent_body(env.globals[client])
        .expect("open law client has a checked body");
    assert!(term_contains_const(&body, owner), "{client} cites {law}");
    owner
}

/// SOURCE: spec/50-stdlib/67-geometry.md §§19, 21 and 22, Architect AC-0.
/// PROMISE CLASS: durable invariant.
/// MEASURED: provider-relative trust identity equality for each package;
/// owned Transparent law identities cited in open clients; and the respective
/// Order cancellation identity reachable from each geometry proof.
/// CLAIMED: the laws are available at their public types without new trust.
/// THE GAP: providers carry their own trust, measured before each load.
/// Adding unrelated exports or refactoring proofs while retaining the checked
/// Order dependencies stays green. New trust or a lost type, owner, or use fails.
#[test]
fn public_laws_have_checked_owners_and_zero_new_trust() {
    let mut env = ElabEnv::new().expect("fresh elaborator");
    for provider in [
        "Core.Classes.LawfulClasses",
        "Core.Logic.OrdResult",
        "Core.Logic.Transport",
        "Data.Numeric.Nat.Arithmetic",
    ] {
        env.elaborate_module_from_roots(&[catalog_root()], provider)
            .unwrap_or_else(|error| panic!("{provider} root-loads: {error:?}"));
    }
    let before_order = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let order = env
        .elaborate_module_from_roots(&[catalog_root()], ORDER)
        .expect("Order root-loads after its providers");
    let after_order = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "Order trusted GlobalIds: {} → {}",
        before_order.len(),
        after_order.len()
    );
    assert_eq!(after_order, before_order, "Order adds no trusted identity");

    env.elaborate_module_from_roots(&[catalog_root()], "Algorithm.Geometry.NatBox")
        .expect("NatBox root-loads before orientation");
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[catalog_root()], GEOMETRY)
        .expect("NatOrientation root-loads after its providers");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "NatOrientation trusted GlobalIds: {} → {}",
        before.len(),
        after.len()
    );
    assert_eq!(after, before, "orientation adds no trusted identity");
    env.elaborate_file(TRANSLATE_OPEN)
        .expect("open translation and comparison cancellation clients check");
    let translate = owned_open_law(&env, GEOMETRY, &owned, "orient_translate", "open_translate");
    let cancel = owned_open_law(
        &env,
        ORDER,
        &order,
        "compare::add_cancel_r",
        "open_compare_cancel",
    );
    assert!(
        proof_uses_order_law(&env, &owned, translate, cancel),
        "the translation proof consumes the Order law through geometry-owned proofs"
    );

    env.elaborate_file(&format!("{SCALE_IMPORTS}{SCALE_OPEN}"))
        .expect("open scaling and positive-factor cancellation clients check");
    let scale = owned_open_law(&env, GEOMETRY, &owned, "orient_scale", "open_scale");
    let mul_cancel = owned_open_law(
        &env,
        ORDER,
        &order,
        "compare::mul_cancel_r",
        "open_compare_mul_cancel",
    );
    assert!(
        proof_uses_order_law(&env, &owned, scale, mul_cancel),
        "the scaling proof consumes the Order law through geometry-owned proofs"
    );

    env.elaborate_file(AXIS_ACCEPTED)
        .expect("complete selective-imported axis client checks");
    for (law, client) in [
        ("orient_shared_x", "open_shared_x"),
        ("orient_shared_y", "open_shared_y"),
    ] {
        owned_open_law(&env, GEOMETRY, &owned, law, client);
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

/// SOURCE: spec/50-stdlib/67-geometry.md §21, Architect AC-0.
/// PROMISE CLASS: durable invariant.
/// MEASURED: each of the nine translation helpers refuses its own selective
/// import with its exact qualified UnboundName after a fresh provider load.
/// CLAIMED: the translation proof steps are not public laws.
/// THE GAP: this checks selective imports, not how the proof uses the helpers.
/// Unrelated exports or additional private helpers stay green; making any
/// of these helpers selectively importable fails at its own probe.
#[test]
fn translation_helpers_stay_private_to_the_checked_package() {
    for helper in [
        "orient_translate_y",
        "orient_step_x",
        "orient_step_y",
        "orient_common_addend",
        "orient_shift_product",
        "orient_sum_congr",
        "orient_interleave",
        "orient_swap4",
        "orient_rotate_sum",
    ] {
        match loaded_orientation().elaborate_file(&format!("import {GEOMETRY} ({helper})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{GEOMETRY}.{helper}"), "import of {helper}");
            }
            other => panic!("{helper} must refuse selective import: {other:?}"),
        }
    }
}

/// SOURCE: spec/50-stdlib/67-geometry.md §22, Architect AC-0.
/// PROMISE CLASS: durable invariant.
/// MEASURED: each scaling helper refuses selective import with its qualified
/// UnboundName after a fresh, successful load of its owner (including Order).
/// CLAIMED: the five proof steps do not extend the public API.
/// THE GAP: this is selective-import visibility, not a complete export roster.
/// Adding unrelated declarations stays green; exposing any named helper fails.
#[test]
fn scaling_helpers_stay_private_to_their_checked_packages() {
    for (package, helper) in [
        (GEOMETRY, "orient_compare_congr"),
        (GEOMETRY, "orient_scale_sum"),
        (GEOMETRY, "orient_factor_sum"),
        (GEOMETRY, "orient_scale_product"),
        (ORDER, "compare_bits"),
    ] {
        let mut env = ElabEnv::new().expect("fresh elaborator");
        env.elaborate_module_from_roots(&[catalog_root()], package)
            .expect("helper owner root-loads before the import probe");
        match env.elaborate_file(&format!("import {package} ({helper})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{package}.{helper}"), "import of {helper}");
            }
            other => panic!("{helper} must refuse selective import: {other:?}"),
        }
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
            && span.end <= candidate.len() =>
        {
            eprintln!("{label}: KernelRejected(TypeMismatch) at {span:?}");
        }
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

/// SOURCE: spec/50-stdlib/67-geometry.md §21, AC-2 shared-translation row.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the translation-law application checks, and independent direct
/// reductions give Gt before and after the common shift.
/// CLAIMED: a common shift on both axes preserves the observed turn.
/// THE GAP: the closed coordinates supplement rather than prove the open law.
/// Equivalent orientation algorithms stay green; a changed turn or a law
/// that no longer specializes to these moved points fails.
#[test]
fn shared_translation_preserves_a_counterclockwise_turn() {
    loaded_orientation()
        .elaborate_file(TRANSLATE_ACCEPTED)
        .expect("shared translation preserves Gt; shifting only one point gives Lt");
}

/// SOURCE: spec/50-stdlib/67-geometry.md §21, AC-2 one-point refusal.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the fresh accepted prefix checks L=1/R=5 (Lt) via `orient_cross`
/// and direct computation, then a separate load refuses equality with the
/// original Gt triple at the appended theorem with a kernel TypeMismatch.
/// CLAIMED: preserving the turn under common translation does not imply
/// preserving it under a shift of only one point.
/// THE GAP: some one-point shifts do preserve the turn; this is a counterexample.
/// Equivalent arithmetic implementations stay green; accepting the false
/// equality, or rejecting the accepted prefix instead, fails this test.
#[test]
fn translating_only_one_point_refuses_at_its_own_theorem() {
    refuses_only_at_added_theorem(
        TRANSLATE_ACCEPTED,
        "refuse_one_point_translation",
        TRANSLATE_REFUSED,
    );
}

/// SOURCE: spec/50-stdlib/67-geometry.md §22, AC-2 scaling rows.
/// PROMISE CLASS: durable invariant.
/// MEASURED: a closed unit turn gives Gt, the doubled turn uses the checked
/// scaling law, the doubled equation checks independently, and zero gives Eq.
/// CLAIMED: doubling preserves this turn; multiplying by zero does not.
/// THE GAP: the closed rows supplement the generic open type, not prove it.
/// Equivalent arithmetic is compatible; changing these results or bypassing
/// the public law in the doubled turn witness fails.
#[test]
fn positive_scaling_preserves_the_turn_but_zero_gives_eq() {
    let mut env = loaded_orientation();
    env.elaborate_file(&scale_accepted())
        .expect("unit turn, doubled turn by law, doubled equation, and zero Eq check");
    let scale = env.globals[&format!("{GEOMETRY}.orient_scale")];
    let (_, body) = env
        .env
        .transparent_body(env.globals["doubled_turn_by_law"])
        .expect("doubled turn has a checked body");
    assert!(
        term_contains_const(&body, scale),
        "doubled turn cites orient_scale"
    );
}

/// SOURCE: spec/50-stdlib/67-geometry.md §22, AC-2 scaling refusal.
/// PROMISE CLASS: durable invariant.
/// MEASURED: changing the scaled coordinates' factor from two to zero in
/// the Gt client rejects with a kernel TypeMismatch at the added theorem.
/// CLAIMED: the positive-factor law cannot prove Gt for these zero-scaled points.
/// THE GAP: this use could fail at the law application even with a wrong
/// generic law; the separate equation refusal pins the false equality.
/// Diagnostic offsets may move; accepting the claim or failing its prefix breaks.
#[test]
fn zero_scaled_gt_by_law_refuses_at_its_own_theorem() {
    refuses_only_at_added_theorem(&scale_accepted(), "zero_turn_by_law", SCALE_REFUSED_TURN);
}

/// SOURCE: spec/50-stdlib/67-geometry.md §22, AC-2 equation refusal.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the doubled equation checks directly, but the same equation
/// at factor zero rejects at its own theorem after a fresh accepted prefix.
/// CLAIMED: zero scaling does not preserve the unit turn, independently of
/// whether a client can apply the positive-factor law.
/// THE GAP: this is one counterexample, not a classification of all triples.
/// Equivalent proofs and shifted spans are compatible; accepting the zero
/// equation or failing its prefix breaks.
#[test]
fn zero_scaled_equation_refuses_at_its_own_theorem() {
    refuses_only_at_added_theorem(
        &scale_accepted(),
        "zero_equation_claim",
        SCALE_REFUSED_EQUATION,
    );
}
