//! Structural box absorption: spec/50-stdlib/67-geometry.md §8.
//! Source: conformance/stdlib/geometry/seed-box-intersect-hull-absorption.md.
//! Promise class: durable invariant. Both mixed laws restore the left box,
//! even for inverted bounds; the same-form false right-operand claim refuses.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{
    extract_ken_md, Decl as SourceDecl, ElabEnv, ElabError, Expr as SourceExpr, KenMdFenceRole,
    Type as SourceType,
};
use ken_kernel::Decl;

const ORDER: &str = "Data.Numeric.Nat.Order";
const GEOMETRY: &str = "Algorithm.Geometry.NatBox";
const SEED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../conformance/stdlib/geometry/seed-box-intersect-hull-absorption.md"
));

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn loaded_geometry() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], GEOMETRY)
        .expect("Nat Order and NatBox roots-load");
    env
}

const GENERIC_CLIENT: &str = r#"
import Data.Numeric.Nat.Order (min, max)
import Algorithm.Geometry.NatBox (Box, intersect, hull, intersect_hull_absorb, hull_intersect_absorb)

theorem min_absorb_client (m : Nat) (n : Nat)
    : Equal Nat (min m (max m n)) m = (proof absorb for min) m n

theorem max_absorb_client (m : Nat) (n : Nat)
    : Equal Nat (max m (min m n)) m = (proof absorb for max) m n

theorem intersect_hull_client (a : Box) (b : Box)
    : Equal Box (intersect a (hull a b)) a = intersect_hull_absorb a b

theorem hull_intersect_client (a : Box) (b : Box)
    : Equal Box (hull a (intersect a b)) a = hull_intersect_absorb a b
"#;

/// MEASURED: the exact trusted GlobalId set is unchanged after root-loading
/// Nat Order and again after NatBox. All four new public identities are owned
/// and Transparent; their independent generic clients check with open inputs.
/// The new Nat helper and existing box congruences refuse selective import.
/// CLAIMED: §8 exposes two unconditional box laws backed by public Nat laws
/// without changing trusted assumptions or publishing their scaffolding.
/// THE GAP: provider assumptions are inherited; this does not census every
/// unrelated export or substitute closed examples for generic proofs.
#[test]
fn generic_absorption_laws_are_checked_private_and_add_no_trust() {
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
        .expect("two §8 Nat selectors root-load");
    let after_order = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    assert_eq!(after_order, before, "Nat Order adds no trusted GlobalId");
    for selector in ["min::absorb", "max::absorb"] {
        let id = env.globals[&format!("{ORDER}.{selector}")];
        assert!(order_owned.contains(&id), "{selector} must be Order-owned");
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    }
    let helper = env.globals[&format!("{ORDER}.min::idempotent")];
    assert!(order_owned.contains(&helper), "min owns its private helper");
    assert!(matches!(
        env.env.lookup(helper),
        Some(Decl::Transparent { .. })
    ));
    let geometry_owned = env
        .elaborate_module_from_roots(&[root()], GEOMETRY)
        .expect("two §8 NatBox laws root-load");
    let after_geometry = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "§8 trusted GlobalIds: {} → {} → {}",
        before.len(),
        after_order.len(),
        after_geometry.len()
    );
    assert_eq!(after_geometry, before, "NatBox adds no trusted GlobalId");
    for law in ["intersect_hull_absorb", "hull_intersect_absorb"] {
        let id = env.globals[&format!("{GEOMETRY}.{law}")];
        assert!(geometry_owned.contains(&id), "{law} must be NatBox-owned");
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("all four independent generic §8 signatures check");
    match loaded_geometry().elaborate_file(
        "import Data.Numeric.Nat.Order (min)\n\
         theorem private_min_client (n : Nat) : Equal Nat (min n n) n =\n\
           (proof idempotent for min) n",
    ) {
        Err(ElabError::UnboundName { name, .. }) => {
            assert_eq!(name, format!("{ORDER}.min::idempotent"));
        }
        other => panic!("min::idempotent must stay private: {other:?}"),
    }
    for private in ["point_equal", "box_equal", "box_with_lo", "box_with_hi"] {
        match loaded_geometry().elaborate_file(&format!("import {GEOMETRY} ({private})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{GEOMETRY}.{private}"));
            }
            other => panic!("{private} must remain private: {other:?}"),
        }
    }
}

// Identify source programs by checked Ken theorem names, never fence order.
fn seed_case(theorem_name: &str) -> &'static str {
    let mut selected = None;
    for fence in extract_ken_md(SEED)
        .expect("§8 seed extracts")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
    {
        let source = &SEED[fence.body_range];
        let contains = parse_decls(source)
            .expect("§8 seed fence parses")
            .into_iter()
            .any(
                |decl| matches!(decl, SourceDecl::TheoremDecl { name, .. } if name == theorem_name),
            );
        if contains {
            assert!(
                selected.replace(source).is_none(),
                "{theorem_name} must identify one source program"
            );
        }
    }
    selected.unwrap_or_else(|| panic!("missing §8 theorem {theorem_name}"))
}

fn type_shape(ty: &SourceType) -> String {
    match ty {
        SourceType::TCon(name, _) | SourceType::TVar(name, _) => format!("name:{name}"),
        SourceType::TApp(function, argument, _) => {
            format!("({} {})", type_shape(function), type_shape(argument))
        }
        other => panic!("unexpected §8 closed proposition: {other:?}"),
    }
}

fn expr_shape(expr: &SourceExpr) -> String {
    match expr {
        SourceExpr::ECon(name, _) | SourceExpr::EVar(name, _) => format!("name:{name}"),
        SourceExpr::EApp(function, argument, _) => {
            format!("({} {})", expr_shape(function), expr_shape(argument))
        }
        SourceExpr::EAttachedProofRef {
            subject,
            proof_name,
            ..
        } => format!("attached:{subject}::{proof_name}"),
        other => panic!("unexpected §8 closed proof: {other:?}"),
    }
}

fn theorem_shape(source: &str, name: &str) -> (String, String) {
    let found = parse_decls(source)
        .expect("§8 theorem parses")
        .into_iter()
        .filter_map(|decl| match decl {
            SourceDecl::TheoremDecl {
                name: actual,
                theorem,
                body,
                ..
            } if actual == name => Some((type_shape(&theorem), expr_shape(&body))),
            _ => None,
        })
        .collect::<Vec<_>>();
    let [one] = found.as_slice() else {
        panic!("exactly one theorem {name} required");
    };
    one.clone()
}

const EXPECTED_CONTROLS: &str = r#"
theorem expect_min : Equal Nat (min one (max one three)) one = Proved
theorem expect_max : Equal Nat (max four (min four two)) four = Proved
theorem expect_min_law : Equal Nat (min one (max one three)) one =
  (proof absorb for min) one three
theorem expect_max_law : Equal Nat (max four (min four two)) four =
  (proof absorb for max) four two

theorem expect_hull : Equal Box (hull box_a box_b) hull_ab = Refl
theorem expect_intersection : Equal Box (intersect box_a box_b) intersection_ab = Refl
theorem expect_intersect_hull : Equal Box (intersect box_a (hull box_a box_b)) box_a = Refl
theorem expect_hull_intersect : Equal Box (hull box_a (intersect box_a box_b)) box_a = Refl
theorem expect_intersect_law : Equal Box (intersect box_a (hull box_a box_b)) box_a =
  intersect_hull_absorb box_a box_b
theorem expect_hull_law : Equal Box (hull box_a (intersect box_a box_b)) box_a =
  hull_intersect_absorb box_a box_b

theorem expect_inverted_intersect : Equal Box (intersect inverted_a box_b) inverted_a = Refl
theorem expect_inverted_hull : Equal Box (hull inverted_a box_b) box_b = Refl
theorem expect_inverted_intersect_hull :
  Equal Box (intersect inverted_a (hull inverted_a box_b)) inverted_a = Refl
theorem expect_inverted_hull_intersect :
  Equal Box (hull inverted_a (intersect inverted_a box_b)) inverted_a = Refl
theorem expect_inverted_intersect_law :
  Equal Box (intersect inverted_a (hull inverted_a box_b)) inverted_a =
  intersect_hull_absorb inverted_a box_b
theorem expect_inverted_hull_law :
  Equal Box (hull inverted_a (intersect inverted_a box_b)) inverted_a =
  hull_intersect_absorb inverted_a box_b

theorem expect_same_form : Equal Box (intersect (hull box_a box_b) box_b) box_b = Refl
"#;
const EXPECTED_REFUSAL: &str =
    "theorem expected_false : Equal Box (intersect (hull box_a box_b) box_b) box_a = Refl";

// These kernel-checked facts use the actual seed boxes, not duplicated box
// constructors. Both `inside` directions exclude nesting. The SAME point
// belongs to both via `contains`, independently of the seed's intersection
// result constant: disjoint boxes can still satisfy that value equation.
const FIXTURE_GEOMETRY_CONTROLS: &str = r#"
import Algorithm.Geometry.NatBox (inside, contains, MkPoint)

theorem box_a_not_inside_box_b : Equal Bool (inside box_a box_b) False = Proved

theorem box_b_not_inside_box_a : Equal Bool (inside box_b box_a) False = Proved

theorem shared_point_inside_box_a :
  Equal Bool (contains box_a (MkPoint three one)) True = Proved

theorem shared_point_inside_box_b :
  Equal Bool (contains box_b (MkPoint three one)) True = Proved
"#;

/// MEASURED: the complete accepted client root-loads both packages and
/// checks direct and public-law Nat controls, independent hull/intersection
/// corners, both mixed results and applications, and inverted-box instances.
/// After the full seed, two checked `inside` controls reject containment
/// in either direction. Two checked `contains` controls require one shared
/// point of both actual seed boxes, establishing overlap independently of
/// the declared intersection-result constant. The same-form right-operand
/// true control uses Refl. The isolated false equality is AST-pinned and
/// refused at its own Refl after a fresh accepted prefix. CLAIMED: the left
/// box is structurally recovered, and the regular seed pair overlaps without
/// either box containing the other.
/// THE GAP: these finite clients do not establish the universal law, which
/// instead rests on the generic kernel-checked bodies above.
#[test]
fn full_absorption_seed_checks_and_right_operand_false_claim_refuses() {
    let accepted = seed_case("nat_min_absorption_value");
    let refused = seed_case("seed_neg");
    for (actual, expected) in [
        ("nat_min_absorption_value", "expect_min"),
        ("nat_max_absorption_value", "expect_max"),
        ("nat_min_absorption_law", "expect_min_law"),
        ("nat_max_absorption_law", "expect_max_law"),
        ("hull_ab_value", "expect_hull"),
        ("intersection_ab_value", "expect_intersection"),
        ("intersect_hull_ab_value", "expect_intersect_hull"),
        ("hull_intersect_ab_value", "expect_hull_intersect"),
        ("intersect_hull_ab_law", "expect_intersect_law"),
        ("hull_intersect_ab_law", "expect_hull_law"),
        ("inverted_intersection_value", "expect_inverted_intersect"),
        ("inverted_hull_value", "expect_inverted_hull"),
        (
            "inverted_intersect_hull_value",
            "expect_inverted_intersect_hull",
        ),
        (
            "inverted_hull_intersect_value",
            "expect_inverted_hull_intersect",
        ),
        (
            "inverted_intersect_hull_law",
            "expect_inverted_intersect_law",
        ),
        ("inverted_hull_intersect_law", "expect_inverted_hull_law"),
        ("same_form_true_control", "expect_same_form"),
    ] {
        assert_eq!(
            theorem_shape(accepted, actual),
            theorem_shape(EXPECTED_CONTROLS, expected),
            "accepted control {actual} must preserve its §8 obligation"
        );
    }
    assert_eq!(
        theorem_shape(refused, "seed_neg"),
        theorem_shape(EXPECTED_REFUSAL, "expected_false"),
        "false right-operand equality must retain its specific Refl claim"
    );
    let parsed = parse_decls(refused).expect("isolated §8 refusal parses");
    let [SourceDecl::TheoremDecl {
        name,
        params,
        span: claim_span,
        body: SourceExpr::ECon(proof, _),
        ..
    }] = parsed.as_slice()
    else {
        panic!("isolated refusal has only one closed theorem with Refl");
    };
    assert_eq!(name, "seed_neg");
    assert!(params.is_empty(), "false result is closed");
    assert_eq!(proof, "Refl");
    let mut env = loaded_geometry();
    env.elaborate_file(accepted)
        .expect("full accepted §8 client and true same-form control check");
    env.elaborate_file(FIXTURE_GEOMETRY_CONTROLS)
        .expect("regular seed boxes must overlap without containing each other");
    for name in [
        "box_a_not_inside_box_b",
        "box_b_not_inside_box_a",
        "shared_point_inside_box_a",
        "shared_point_inside_box_b",
        "nat_min_absorption_value",
        "nat_min_absorption_law",
        "nat_max_absorption_value",
        "nat_max_absorption_law",
        "hull_ab_value",
        "intersection_ab_value",
        "intersect_hull_ab_value",
        "hull_intersect_ab_value",
        "intersect_hull_ab_law",
        "hull_intersect_ab_law",
        "inverted_intersection_value",
        "inverted_hull_value",
        "inverted_intersect_hull_value",
        "inverted_hull_intersect_value",
        "inverted_intersect_hull_law",
        "inverted_hull_intersect_law",
        "same_form_true_control",
    ] {
        let id = env.globals[name];
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    }
    let mut rejecting = loaded_geometry();
    rejecting
        .elaborate_file(accepted)
        .expect("fresh complete accepted prefix checks before refusal");
    rejecting
        .elaborate_file(FIXTURE_GEOMETRY_CONTROLS)
        .expect("fresh accepted pair overlaps without either containment");
    match rejecting.elaborate_file(refused) {
        Err(ElabError::TypeMismatch { span, .. })
            if span.start >= claim_span.start
                && span.start < span.end
                && span.end <= claim_span.end => {}
        other => panic!("false right-operand Refl must reject at its own claim: {other:?}"),
    }
}
