//! Structural distribution of box intersection and hull: geometry §9.
//! Promise class: durable invariant for the four checked generic laws and
//! refusal; normative compatibility vector for the seed's assigned triple.
//! The closed examples do not establish the generic laws.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{extract_ken_md, Decl as SourceDecl, ElabEnv, ElabError, KenMdFenceRole};
use ken_kernel::Decl;

const ORDER: &str = "Data.Numeric.Nat.Order";
const GEOMETRY: &str = "Algorithm.Geometry.NatBox";
const SEED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../conformance/stdlib/geometry/seed-box-intersect-hull-distributivity.md"
));

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn loaded_geometry() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], GEOMETRY)
        .expect("Nat Order and NatBox root-load");
    env
}

const WRONG_SELECTOR: &str = r#"
import Data.Numeric.Nat.Order (min, max)
theorem wrong_selector (m : Nat) (n : Nat) (k : Nat)
  : Equal Nat (min m (max n k)) (max (min m n) (min m k)) =
  (proof distrib for max) m n k
"#;

const GENERIC_CLIENT: &str = r#"
import Data.Numeric.Nat.Order (min, max)
import Algorithm.Geometry.NatBox (Box, intersect, hull, intersect_hull_distrib, hull_intersect_distrib)

theorem min_distrib_client (m : Nat) (n : Nat) (k : Nat)
  : Equal Nat (min m (max n k)) (max (min m n) (min m k)) =
  (proof distrib for min) m n k

theorem max_distrib_client (m : Nat) (n : Nat) (k : Nat)
  : Equal Nat (max m (min n k)) (min (max m n) (max m k)) =
  (proof distrib for max) m n k

theorem intersect_hull_client (a : Box) (b : Box) (c : Box)
  : Equal Box (intersect a (hull b c)) (hull (intersect a b) (intersect a c)) =
  intersect_hull_distrib a b c

theorem hull_intersect_client (a : Box) (b : Box) (c : Box)
  : Equal Box (hull a (intersect b c)) (intersect (hull a b) (hull a c)) =
  hull_intersect_distrib a b c
"#;

/// MEASURED: both packages load from roots with exact trusted GlobalId sets
/// unchanged, their four module-owned laws are Transparent, and an open client
/// checks each public type. Private box congruences refuse external import.
/// CLAIMED: the generic structural distribution laws add no trust or helper
/// exports. GAP: this does not prove properties of unrelated package exports.
#[test]
fn generic_distributivity_laws_are_checked_owned_and_zero_trust() {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        "Core.Classes.LawfulClasses",
        "Core.Logic.Conjunction",
        "Core.Logic.Transport",
        "Data.Numeric.Nat.Arithmetic",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider} root-loads: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let order_owned = env
        .elaborate_module_from_roots(&[root()], ORDER)
        .expect("two public Nat distribution selectors load");
    let after_order = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    assert_eq!(after_order, before, "Nat Order adds no trusted GlobalId");
    for selector in ["min::distrib", "max::distrib"] {
        let id = env.globals[&format!("{ORDER}.{selector}")];
        assert!(order_owned.contains(&id), "{selector} must be Order-owned");
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    }
    let geometry_owned = env
        .elaborate_module_from_roots(&[root()], GEOMETRY)
        .expect("two public box distribution laws load");
    let after_geometry = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "§9 trusted GlobalIds: {} → {} → {}",
        before.len(),
        after_order.len(),
        after_geometry.len()
    );
    assert_eq!(after_geometry, before, "NatBox adds no trusted GlobalId");
    for law in ["intersect_hull_distrib", "hull_intersect_distrib"] {
        let id = env.globals[&format!("{GEOMETRY}.{law}")];
        assert!(geometry_owned.contains(&id), "{law} must be NatBox-owned");
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("all four open generic public clients check");
    match loaded_geometry().elaborate_file(WRONG_SELECTOR) {
        Err(ElabError::KernelRejected {
            error: ken_kernel::KernelError::TypeMismatch { .. },
            ..
        }) => {}
        other => panic!("the max selector must not prove the min law: {other:?}"),
    }
    for helper in ["point_equal", "box_equal"] {
        match loaded_geometry().elaborate_file(&format!("import {GEOMETRY} ({helper})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{GEOMETRY}.{helper}"));
            }
            other => panic!("{helper} must stay private: {other:?}"),
        }
    }
}

// Select complete Ken programs by declaration identity rather than fence order.
fn seed_case(theorem_name: &str) -> &'static str {
    let mut selected = None;
    for fence in extract_ken_md(SEED)
        .expect("§9 seed extracts")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
    {
        let source = &SEED[fence.body_range];
        let contains = parse_decls(source)
            .expect("§9 seed fence parses")
            .into_iter()
            .any(
                |decl| matches!(decl, SourceDecl::TheoremDecl { name, .. } if name == theorem_name),
            );
        if contains {
            assert!(
                selected.replace(source).is_none(),
                "{theorem_name} identifies one Ken program"
            );
        }
    }
    selected.unwrap_or_else(|| panic!("missing §9 theorem {theorem_name}"))
}

// The seed's exact assigned coordinates are a compatibility vector, not a
// surrogate proof of either generic distribution theorem. These obligations
// are checked against the seed's actual boxes and both computed sides.
const INDEPENDENT_VALUES: &str = r#"
theorem assigned_a : Equal Box box_a (MkBox (MkPoint one one) (MkPoint four four)) = Refl
theorem assigned_b : Equal Box box_b (MkBox (MkPoint three zero) (MkPoint six two)) = Refl
theorem assigned_c : Equal Box box_c (MkBox (MkPoint zero three) (MkPoint two three)) = Refl

theorem computed_inversion : Equal Box (intersect box_b box_c) inverted_bc = Refl
theorem inverted_is_exact : Equal Box inverted_bc (MkBox (MkPoint three three) (MkPoint two two)) = Refl

theorem computed_first_left : Equal Box (intersect box_a (hull box_b box_c)) first_result = Refl
theorem computed_first_right : Equal Box (hull (intersect box_a box_b) (intersect box_a box_c)) first_result = Refl
theorem first_result_is_new : Equal Box first_result (MkBox (MkPoint one one) (MkPoint four three)) = Refl

theorem computed_second_left : Equal Box (hull box_a (intersect box_b box_c)) box_a = Refl
theorem computed_second_right : Equal Box (intersect (hull box_a box_b) (hull box_a box_c)) box_a = Refl

theorem computed_mixed_right : Equal Box (hull (intersect box_a box_b) box_c) mixed_right_result = Refl
theorem mixed_result_is_distinct : Equal Box mixed_right_result (MkBox (MkPoint zero one) (MkPoint four three)) = Refl
"#;

/// MEASURED: the full accepted seed and independent assigned-triple, inverted
/// intermediate, and each-side controls kernel-check; the sole false mixed
/// regrouping refuses at its Refl after a fresh accepted prefix. CLAIMED: §9
/// seed exercises valid laws on its fixed triple while rejecting a same-form
/// false claim. GAP: closed instances cannot prove the four generic laws.
#[test]
fn complete_seed_accepts_and_isolated_mixed_regrouping_refuses() {
    let accepted = seed_case("first_distributes_by_law");
    let refused = seed_case("seed_neg");
    let mut positive = loaded_geometry();
    positive
        .elaborate_file(accepted)
        .expect("complete accepted §9 client checks");
    positive
        .elaborate_file(INDEPENDENT_VALUES)
        .expect("fixed triple and both sides check independently");
    for name in [
        "first_left_value",
        "first_right_value",
        "first_distributes_by_law",
        "inverted_intersection_value",
        "second_left_value",
        "second_right_value",
        "second_distributes_by_law",
        "same_form_true_control",
        "mixed_right_value",
        "assigned_a",
        "assigned_b",
        "assigned_c",
        "computed_inversion",
        "inverted_is_exact",
        "computed_first_left",
        "computed_first_right",
        "first_result_is_new",
        "computed_second_left",
        "computed_second_right",
        "computed_mixed_right",
        "mixed_result_is_distinct",
    ] {
        let id = positive.globals[name];
        assert!(matches!(
            positive.env.lookup(id),
            Some(Decl::Transparent { .. })
        ));
    }
    let parsed = parse_decls(refused).expect("isolated §9 claim parses");
    let [SourceDecl::TheoremDecl { name, span, .. }] = parsed.as_slice() else {
        panic!("isolated refusal must contain only seed_neg");
    };
    assert_eq!(name, "seed_neg");
    let mut rejecting = loaded_geometry();
    rejecting
        .elaborate_file(accepted)
        .expect("fresh complete accepted prefix checks before refusal");
    rejecting
        .elaborate_file(INDEPENDENT_VALUES)
        .expect("fresh triple and independently computed sides check");
    let err = rejecting
        .elaborate_file(refused)
        .expect_err("the sole false mixed regrouping must be refused");
    match err {
        ElabError::TypeMismatch {
            span: error_span,
            reason,
        } if error_span.start >= span.start
            && error_span.end <= span.end
            && reason.contains("Refl expects an `Eq`-shaped goal") => {}
        other => panic!("false mixed regrouping must reject at its own Refl: {other:?}"),
    }
}
