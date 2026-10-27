//! Spec/50-stdlib/67-geometry.md §6: structural box commutativity.
//! Promise class: durable invariant. Closed seed cases supplement four
//! checked generic proofs; an isolated false equality tests the same carrier.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{
    extract_ken_md, Decl as SourceDecl, ElabEnv, ElabError, Expr, KenMdFenceRole,
    Type as SourceType,
};
use ken_kernel::Decl;

const ORDER: &str = "Data.Numeric.Nat.Order";
const GEOMETRY: &str = "Algorithm.Geometry.NatBox";
const SEED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../conformance/stdlib/geometry/seed-box-intersect-hull-commutativity.md"
));

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn loaded() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], GEOMETRY)
        .expect("Order and NatBox commute proofs root-load");
    env
}

// Fence identity comes from parsed Ken theorem names, not source position or
// a brittle requirement that the Markdown have exactly two source fences.
fn seed_fence_containing(theorem_name: &str) -> String {
    let selected = extract_ken_md(SEED)
        .expect("§6 seed extracts")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
        .filter_map(|fence| {
            let source = &SEED[fence.body_range];
            let has_theorem = parse_decls(source)
                .expect("§6 source fence parses")
                .iter()
                .any(|decl| {
                    matches!(decl,
                    SourceDecl::TheoremDecl { name, .. } if name == theorem_name)
                });
            has_theorem.then(|| source.to_owned())
        })
        .collect::<Vec<_>>();
    let [source] = selected.as_slice() else {
        panic!("exactly one §6 source fence must define {theorem_name}");
    };
    source.clone()
}

fn shape(ty: &SourceType) -> String {
    match ty {
        SourceType::TCon(name, _) => format!("constructor:{name}"),
        SourceType::TVar(name, _) => format!("variable:{name}"),
        SourceType::TApp(head, arg, _) => format!("({} {})", shape(head), shape(arg)),
        other => panic!("unsupported closed §6 equality shape: {other:?}"),
    }
}

fn proposition(source: &str) -> String {
    let parsed = parse_decls(source).expect("independent §6 proposition parses");
    let [SourceDecl::TheoremDecl { theorem, .. }] = parsed.as_slice() else {
        panic!("one independent theorem required");
    };
    shape(theorem)
}

const GENERIC_CLIENT: &str = r#"
import Data.Numeric.Nat.Order (min, max)
import Algorithm.Geometry.NatBox
  (Box, intersect, hull, intersect_comm, hull_comm)

theorem min_comm_client (m : Nat) (n : Nat) :
  Equal Nat (min m n) (min n m) = (proof comm for min) m n

theorem max_comm_client (m : Nat) (n : Nat) :
  Equal Nat (max m n) (max n m) = (proof comm for max) m n

theorem intersect_comm_client (b : Box) (c : Box) :
  Equal Box (intersect b c) (intersect c b) = intersect_comm b c

theorem hull_comm_client (b : Box) (c : Box) :
  Equal Box (hull b c) (hull c b) = hull_comm b c
"#;

/// SOURCE: spec/50-stdlib/67-geometry.md §§6.1–6.2.
/// PROMISE CLASS: durable invariant.
/// MEASURED: root-loaded Order and NatBox own their four transparent laws,
/// a generic independently restated client consumes every ruled public type,
/// and exact trusted GlobalId sets are identical before and after each load.
/// CLAiMED: all four quantified commutativity laws are public checked proofs
/// with no additional assumptions and no exported private box scaffolding.
/// THE GAP: existing imported trust remains in both sets; this does not
/// enumerate every unrelated public name or prove a finite seed is general.
#[test]
fn generic_commutativity_laws_are_owned_checked_private_and_add_no_trust() {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        "Core.Classes.LawfulClasses",
        "Core.Logic.Transport",
        "Data.Numeric.Nat.Arithmetic",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider} roots-loads: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let order_owned = env
        .elaborate_module_from_roots(&[root()], ORDER)
        .expect("Order's two comm selectors root-load");
    let after_order = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    assert_eq!(after_order, before, "Order adds no trusted GlobalId");
    for selector in ["min::comm", "max::comm"] {
        let id = env.globals[&format!("{ORDER}.{selector}")];
        assert!(order_owned.contains(&id), "{selector} must be Order-owned");
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    }
    let geometry_owned = env
        .elaborate_module_from_roots(&[root()], GEOMETRY)
        .expect("NatBox's two comm laws root-load");
    let after_geometry = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "§6 trust GlobalIds: {} → {}",
        before.len(),
        after_geometry.len()
    );
    assert_eq!(after_geometry, before, "NatBox adds no trusted GlobalId");
    for law in ["intersect_comm", "hull_comm"] {
        let id = env.globals[&format!("{GEOMETRY}.{law}")];
        assert!(geometry_owned.contains(&id), "{law} must be NatBox-owned");
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("independent generic §6 client uses four complete public types");
    for private in ["box_with_lo", "box_with_hi", "box_equal"] {
        match env.elaborate_file(&format!("import {GEOMETRY} ({private})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{GEOMETRY}.{private}"))
            }
            other => panic!("{private} must be private: {other:?}"),
        }
    }
}

/// SOURCE: conformance/stdlib/geometry/seed-box-intersect-hull-commutativity.md.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the full named seed client checks against roots-loaded packages,
/// including both direct Order selectors and independent Nat/Box values.
/// CLAIMED: the four laws admit the prescribed overlapping closed instances.
/// THE GAP: generic proof and ownership are checked separately above.
#[test]
fn accepted_seed_with_order_selectors_checks() {
    let positive = seed_fence_containing("min_swapped");
    let declarations = parse_decls(&positive).expect("accepted seed parses");
    for name in [
        "min_value",
        "max_value",
        "min_swapped",
        "max_swapped",
        "intersect_value",
        "hull_value",
        "intersect_swapped",
        "hull_swapped",
        "hull_swapped_closed",
    ] {
        assert!(
            declarations.iter().any(|decl| matches!(decl,
            SourceDecl::TheoremDecl { name: found, .. } if found == name)),
            "complete accepted seed must include {name}"
        );
    }
    loaded()
        .elaborate_file(&positive)
        .expect("full §6 client checks after package import");
}

/// SOURCE: conformance/stdlib/geometry/seed-box-intersect-hull-commutativity.md.
/// PROMISE CLASS: durable invariant.
/// MEASURED: a fresh full positive client and true same-form Refl control
/// check; the isolated seed_neg is exactly the false hull-vs-intersection
/// proposition on those operands, and its own Refl fails TypeMismatch.
/// CLAIMED: the two distinct box operations cannot be identified by a swap.
/// THE GAP: one pair does not prove arbitrary inequality; production-law
/// swaps are tested at the generic checked theorem and mutation seam.
#[test]
fn false_hull_equals_intersection_refuses_at_its_own_refl() {
    let positive = seed_fence_containing("min_swapped");
    let accepted = parse_decls(&positive).expect("accepted §6 seed parses");
    let true_control =
        proposition("theorem control : Equal Box (hull b02 b13) (hull b13 b02) = Refl");
    assert!(
        accepted.iter().any(|decl| matches!(decl,
            SourceDecl::TheoremDecl { name, theorem, body: Expr::ECon(proof, _), .. }
            if name == "hull_swapped_closed" && shape(theorem) == true_control && proof == "Refl"
        )),
        "same-carrier, same-proof-form true control is in accepted seed"
    );
    let negative = seed_fence_containing("seed_neg");
    let parsed = parse_decls(&negative).expect("isolated false fence parses");
    let [SourceDecl::TheoremDecl {
        name,
        params,
        theorem,
        body: Expr::ECon(proof, _),
        span,
        ..
    }] = parsed.as_slice()
    else {
        panic!("isolated refusal must contain exactly one theorem");
    };
    assert_eq!(name, "seed_neg");
    assert!(params.is_empty(), "refusal is closed");
    assert_eq!(proof, "Refl");
    let false_goal =
        proposition("theorem expected : Equal Box (hull b02 b13) (intersect b13 b02) = Refl");
    assert_eq!(
        shape(theorem),
        false_goal,
        "refusal has the specified unequal boxes"
    );
    let mut env = loaded();
    env.elaborate_file(&positive)
        .expect("entire positive client checks before refusal");
    match env.elaborate_file(&negative) {
        Err(ElabError::TypeMismatch { span: failed, .. })
            if failed.start >= span.start
                && failed.end <= span.end
                && failed.start < failed.end => {}
        other => panic!("false equality must fail at seed_neg's own Refl: {other:?}"),
    }
}
