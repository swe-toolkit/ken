//! Structural box associativity: spec/50-stdlib/67-geometry.md §7.
//! SOURCE: conformance/stdlib/geometry/seed-box-intersect-hull-associativity.md.
//! PROMISE CLASS: durable invariant. Generic checked proofs own the four
//! regroupings; the seed fixes independent value and mixed-operation controls.

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
    "/../../conformance/stdlib/geometry/seed-box-intersect-hull-associativity.md"
));

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn loaded_geometry() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], GEOMETRY)
        .expect("Order and NatBox root-load");
    env
}

// A theorem identity, not the seed's fence position or number of fences,
// selects the complete accepted client and the isolated refused theorem.
fn seed_case(theorem_name: &str) -> &'static str {
    let mut selected = None;
    for fence in extract_ken_md(SEED)
        .expect("§7 seed fences extract")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
    {
        let source = &SEED[fence.body_range];
        let found = parse_decls(source)
            .expect("§7 source fence parses")
            .into_iter()
            .any(
                |decl| matches!(decl, SourceDecl::TheoremDecl { name, .. } if name == theorem_name),
            );
        if found {
            assert!(
                selected.replace(source).is_none(),
                "{theorem_name} must identify exactly one seed client"
            );
        }
    }
    selected.unwrap_or_else(|| panic!("missing §7 seed theorem {theorem_name}"))
}

const GENERIC_CLIENT: &str = r#"
import Data.Numeric.Nat.Order (min, max)
import Algorithm.Geometry.NatBox (Box, intersect, hull, intersect_assoc, hull_assoc)

theorem min_assoc_client (k : Nat) (m : Nat) (n : Nat)
    : Equal Nat (min (min k m) n) (min k (min m n)) =
  (proof assoc for min) k m n

theorem max_assoc_client (k : Nat) (m : Nat) (n : Nat)
    : Equal Nat (max (max k m) n) (max k (max m n)) =
  (proof assoc for max) k m n

theorem intersect_assoc_client (a : Box) (b : Box) (c : Box)
    : Equal Box (intersect (intersect a b) c) (intersect a (intersect b c)) =
  intersect_assoc a b c

theorem hull_assoc_client (a : Box) (b : Box) (c : Box)
    : Equal Box (hull (hull a b) c) (hull a (hull b c)) =
  hull_assoc a b c
"#;

/// MEASURED: after checked providers, Order and NatBox each retain the exact
/// trusted GlobalId set. The two attached selectors and two box laws have
/// distinct module-owned Transparent identities; an arbitrary-input client
/// consumes all four generic types. Existing box proof helpers refuse import.
/// CLAIMED: each §7 law is a checked public proof with no newly trusted axiom.
/// THE GAP: the provider closure can retain inherited trust, and closed seed
/// values cannot substitute for the four generic kernel-checked bodies.
#[test]
fn four_generic_associativity_laws_are_owned_checked_and_add_no_trust() {
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
        .expect("Order's two assoc selectors root-load");
    let after_order = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    assert_eq!(after_order, before, "Order adds no trusted GlobalId");
    for selector in ["min::assoc", "max::assoc"] {
        let id = env.globals[&format!("{ORDER}.{selector}")];
        assert!(order_owned.contains(&id), "{selector} must be Order-owned");
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    }
    let geometry_owned = env
        .elaborate_module_from_roots(&[root()], GEOMETRY)
        .expect("NatBox's two assoc laws root-load");
    let after_geometry = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "§7 trusted GlobalIds: {} → {} → {}",
        before.len(),
        after_order.len(),
        after_geometry.len()
    );
    assert_eq!(after_geometry, before, "NatBox adds no trusted GlobalId");
    for law in ["intersect_assoc", "hull_assoc"] {
        let id = env.globals[&format!("{GEOMETRY}.{law}")];
        assert!(geometry_owned.contains(&id), "{law} must be NatBox-owned");
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("all four independent generic §7 client types check");
    for private in ["point_equal", "box_equal", "box_with_lo", "box_with_hi"] {
        match loaded_geometry().elaborate_file(&format!("import {GEOMETRY} ({private})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{GEOMETRY}.{private}"));
            }
            other => panic!("{private} must remain private: {other:?}"),
        }
    }
}

fn shape(ty: &SourceType) -> String {
    match ty {
        SourceType::TCon(name, _) | SourceType::TVar(name, _) => format!("name:{name}"),
        SourceType::TApp(function, argument, _) => {
            format!("({} {})", shape(function), shape(argument))
        }
        other => panic!("unexpected closed §7 proposition: {other:?}"),
    }
}

/// MEASURED: a complete accepted §7 client checks the two public Nat selector
/// applications, all four independent box grouping values, both generic-law
/// applications, same-form closed Refl regroupings and distinct mixed sides.
/// The isolated `seed_neg` has the exact false mixed-operation proposition
/// parsed as Ken, then its own `Refl` fails with TypeMismatch after positives.
/// CLAIMED: individual intersection and hull associativity does not imply
/// equality of a mixed regrouping, even on overlapping boxes.
/// THE GAP: one false mixed equation does not characterize every mixed case;
/// the universal positive claims rely on the generic checked proofs above.
#[test]
fn full_associativity_seed_checks_and_mixed_regrouping_refuses() {
    let accepted = seed_case("min_regrouped");
    let refused = seed_case("seed_neg");
    let parsed = parse_decls(refused).expect("isolated §7 refusal parses");
    let [SourceDecl::TheoremDecl {
        name,
        params,
        theorem,
        body: SourceExpr::ECon(proof, _),
        span,
        ..
    }] = parsed.as_slice()
    else {
        panic!("isolated §7 refusal must have one theorem and direct proof");
    };
    assert_eq!(name, "seed_neg");
    assert!(params.is_empty(), "the false equation must be closed");
    assert_eq!(proof, "Refl", "the same-form true controls use Refl");
    let expected = parse_decls(
        "theorem expected_false : Equal Box \
         (intersect (hull box_a box_b) box_c) \
         (hull box_a (intersect box_b box_c)) = Refl",
    )
    .expect("independent mixed-operation proposition parses");
    let [SourceDecl::TheoremDecl {
        theorem: expected_goal,
        ..
    }] = expected.as_slice()
    else {
        panic!("expected false proposition has one theorem");
    };
    assert_eq!(
        shape(theorem),
        shape(expected_goal),
        "seed_neg must assert the mixed intersection/hull regrouping"
    );
    let mut env = loaded_geometry();
    env.elaborate_file(accepted)
        .expect("complete accepted seed and opposite mixed results check");
    for name in [
        "min_value",
        "max_value",
        "min_regrouped",
        "max_regrouped",
        "intersect_left_value",
        "intersect_right_value",
        "hull_left_value",
        "hull_right_value",
        "intersect_regrouped",
        "hull_regrouped",
        "intersect_regrouped_closed",
        "hull_regrouped_closed",
        "mixed_left_value",
        "mixed_right_value",
    ] {
        let id = env.globals[name];
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    }
    match env.elaborate_file(refused) {
        Err(ElabError::TypeMismatch { span: failure, .. })
            if failure.start >= span.start
                && failure.end <= span.end
                && failure.start < failure.end => {}
        other => panic!("mixed regrouping must fail at seed_neg's own Refl: {other:?}"),
    }
}
