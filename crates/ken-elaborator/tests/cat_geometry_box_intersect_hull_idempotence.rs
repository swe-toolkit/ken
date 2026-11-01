//! Structural box idempotence: spec/50-stdlib/67-geometry.md §13.
//! Source: conformance/stdlib/geometry/seed-box-intersect-hull-idempotence.md.
//! Promise class: durable invariant. The accepted and refused clients use
//! kernel-checked box equality, including on inverted corners.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{extract_ken_md, Decl as SourceDecl, ElabEnv, ElabError, KenMdFenceRole};
use ken_kernel::Decl;

const GEOMETRY: &str = "Algorithm.Geometry.NatBox";
const SEED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../conformance/stdlib/geometry/seed-box-intersect-hull-idempotence.md"
));

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn loaded_geometry() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], GEOMETRY)
        .expect("NatBox root-loads");
    env
}

const GENERIC_CLIENT: &str = r#"
import Algorithm.Geometry.NatBox (Box, intersect, hull, intersect_idempotent, hull_idempotent)

theorem intersect_generic (b : Box) : Equal Box (intersect b b) b =
  intersect_idempotent b

theorem hull_generic (b : Box) : Equal Box (hull b b) b =
  hull_idempotent b
"#;

/// MEASURED: provider-closure and NatBox trusted GlobalId sets are identical;
/// both owned public proofs are Transparent and applied on an open `Box`.
/// CLAIMED: §13's two generic structural identities add no new trust.
/// THE GAP: this does not certify the inherited providers or unrelated laws.
#[test]
fn generic_idempotence_laws_are_checked_owned_and_zero_trust() {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        "Core.Classes.LawfulClasses",
        "Core.Logic.Conjunction",
        "Core.Logic.Transport",
        "Data.Numeric.Nat.Arithmetic",
        "Data.Numeric.Nat.Order",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider} root-loads: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], GEOMETRY)
        .expect("the §13 NatBox proofs root-load");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!("§13 trusted GlobalIds: {} → {}", before.len(), after.len());
    assert_eq!(after, before, "NatBox adds no trusted GlobalId");
    for name in ["intersect_idempotent", "hull_idempotent"] {
        let id = env.globals[&format!("{GEOMETRY}.{name}")];
        assert!(owned.contains(&id), "{name} must be NatBox-owned");
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("both open public §13 signatures check");
    for helper in ["point_equal", "box_equal", "box_with_lo", "box_with_hi"] {
        match loaded_geometry().elaborate_file(&format!("import {GEOMETRY} ({helper})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{GEOMETRY}.{helper}"));
            }
            other => panic!("{helper} must remain private: {other:?}"),
        }
    }
}

// Independently bind the seed's fixture labels to their claimed corners;
// checking only the seed's self-equalities would not detect a weakened fixture.
const FIXTURE_VALUES: &str = r#"
theorem large_is_assigned : Equal Box large (MkBox (MkPoint zero zero) (MkPoint four four)) = Refl
theorem small_is_assigned : Equal Box small (MkBox (MkPoint one one) (MkPoint two two)) = Refl
theorem inverted_is_assigned : Equal Box inverted (MkBox (MkPoint three three) (MkPoint one one)) = Refl
"#;

// The seed programs are identified by checked Ken declarations, not by
// Markdown fence order or incidental prose.
fn seed_case(theorem_name: &str) -> &'static str {
    let mut selected = None;
    for fence in extract_ken_md(SEED)
        .expect("§13 seed extracts")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
    {
        let source = &SEED[fence.body_range];
        let contains = parse_decls(source)
            .expect("§13 seed fence parses")
            .into_iter()
            .any(
                |decl| matches!(decl, SourceDecl::TheoremDecl { name, .. } if name == theorem_name),
            );
        if contains {
            assert!(
                selected.replace(source).is_none(),
                "{theorem_name} must identify exactly one Ken program"
            );
        }
    }
    selected.unwrap_or_else(|| panic!("missing §13 seed theorem {theorem_name}"))
}

/// MEASURED: the full source client checks via catalog roots; a fresh full
/// prefix includes regular and inverted law applications and the true Refl
/// control. The isolated false small/large claim rejects at its own Refl.
/// CLAIMED: §13's finite applications include inverted boxes, while unequal
/// nested boxes do not satisfy an accidental overgeneralization.
/// THE GAP: finite inputs do not prove the open laws; the generic client above
/// tests their public signatures, and the provider checks their bodies.
#[test]
fn complete_seed_checks_and_small_intersection_as_large_refuses() {
    let accepted = seed_case("large_intersection_law");
    let refused = seed_case("seed_neg");
    let mut positive = loaded_geometry();
    positive
        .elaborate_file(accepted)
        .expect("the complete accepted §13 client checks");
    positive
        .elaborate_file(FIXTURE_VALUES)
        .expect("seed labels denote the regular, nested and inverted boxes");
    for name in [
        "large_intersection_value",
        "large_hull_value",
        "large_intersection_law",
        "large_hull_law",
        "inverted_intersection_value",
        "inverted_hull_value",
        "inverted_intersection_law",
        "inverted_hull_law",
        "small_large_intersection_value",
        "same_form_true_control",
        "large_is_assigned",
        "small_is_assigned",
        "inverted_is_assigned",
    ] {
        let id = positive.globals[name];
        assert!(matches!(
            positive.env.lookup(id),
            Some(Decl::Transparent { .. })
        ));
    }
    let parsed = parse_decls(refused).expect("isolated §13 refusal parses");
    let [SourceDecl::TheoremDecl { name, span, .. }] = parsed.as_slice() else {
        panic!("isolated §13 refusal must contain exactly one theorem");
    };
    assert_eq!(name, "seed_neg");
    let mut rejecting = loaded_geometry();
    rejecting
        .elaborate_file(accepted)
        .expect("fresh full accepted prefix checks before the refusal");
    rejecting
        .elaborate_file(FIXTURE_VALUES)
        .expect("fresh seed fixture values check before the refusal");
    match rejecting.elaborate_file(refused) {
        Err(ElabError::TypeMismatch {
            span: error_span,
            reason,
        }) if error_span.start >= span.start
            && error_span.start < error_span.end
            && error_span.end <= span.end
            && reason.contains("Refl expects an `Eq`-shaped goal") => {}
        other => panic!("false small/large equality must refuse at its own Refl: {other:?}"),
    }
}
