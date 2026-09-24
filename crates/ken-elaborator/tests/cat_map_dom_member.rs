//! CAT-MAP-DOM-MEMBER-LAW: checked private domain-membership law and
//! unchanged loader-visible Map surface and trusted-base boundary.

#[path = "support/catalog_or.rs"]
mod catalog_or;
#[path = "support/catalog_publication.rs"]
mod catalog_publication;

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::{foreign::trusted_base_delta, ElabEnv};
use ken_kernel::Decl;

const MAP: &str = "Data.Collections.Map";
const MAP_KEN_MD: &str = include_str!("../../../catalog/packages/Data/Collections/Map.ken.md");

fn catalog_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn load_map() -> ElabEnv {
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_module_from_roots(&[catalog_root()], MAP)
        .expect("Map must roots-load through its declared imports");
    env
}

/// Promise class: transition sentinel for this proof-only Map increment.
/// Retire or rebaseline it at the first separately authorized Map public-API
/// extension. The current exact set is a backfill boundary, not a permanent
/// compatibility promise during initial development.
///
/// MEASURED: the real roots loader admits exactly these nine independently
/// base-captured Map surfaces through selective imports, including the two
/// explicitly exported constructors and no inferred data constructors.
/// CLAIMED: this proof backfill adds no loader-visible public API. THE GAP:
/// the shared publication-query helper probes every candidate declaration
/// and attached proof; this increment authorizes no public expansion. Private
/// proof/helper additions and changes preserving the same visible set stay
/// green. A newly public name, constructor, or attached proof must turn this
/// sentinel red, prompting review and retirement or a fresh base capture at
/// that separately authorized Map public-API extension, not a bug waiver.
///
/// Known blast radius on this base: Map's `reachable_plus` uses `size`/`dom`;
/// `lang_membership_operator_surface.rs` selectively imports `Tree`, both
/// nominal views, and both constructors. `map_build_acceptance.rs`,
/// `cat_map_bool_and_owner.rs`, and `es2_acceptance.rs` elaborate Map;
/// this test probes its loader-visible set. `lang_mod_strict_resolution_d0.rs`
/// inventories the Map module, while `n2_in_repo_loader.rs` and
/// `dotted_module_path_parser.rs` use its path only, not its exports.
#[test]
fn map_dom_member_public_surface_transition_sentinel() {
    let expected = [
        "Tree",
        "OrderedKeyMembership",
        "MkOrderedKeyMembership",
        "RelationEdgeMembership",
        "MkRelationEdgeMembership",
        "size",
        "dom",
        "reachable_within",
        "reachable_plus",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect::<BTreeSet<_>>();
    let visible = catalog_publication::published_module_surfaces(MAP_KEN_MD, MAP, "map_dom_member");
    eprintln!("Map loader-visible exports: {visible:?}");
    assert_eq!(
        visible, expected,
        "Map proof-backfill sentinel: review any new export at the separately authorized public-API extension"
    );
}

/// Promise class: durable invariant.
///
/// MEASURED: independently preload Map's complete direct provider closure,
/// snapshot its kernel trusted-base identities, then load Map and compare the
/// ledger. CLAIMED: the new Map law adds no unchecked trust. THE GAP: the
/// catalog provider's inherited trust is not mistaken for a Map-local delta;
/// the full provider closure is loaded before the before-snapshot.
#[test]
fn map_trusted_base_is_unchanged_from_provider_closure() {
    let mut env = ElabEnv::new().expect("base environment");
    for provider in [
        "Core.Classes.LawfulClasses",
        "Core.Classes.Membership",
        "Core.Logic.Or",
        "Core.Logic.Transport",
        "Data.Collections.Derived",
        "Data.Numeric.Nat.Arithmetic",
        "Data.Sums.Combinators",
    ] {
        env.elaborate_module_from_roots(&[catalog_root()], provider)
            .unwrap_or_else(|error| panic!("provider {provider} must roots-load: {error:?}"));
    }
    let before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    let owned = env
        .elaborate_module_from_roots(&[catalog_root()], MAP)
        .expect("Map must roots-load after its providers");
    assert!(!owned.is_empty(), "Map must declare a real checked package");
    let after: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    eprintln!(
        "Map trusted base: provider closure {}, after Map {}",
        before.len(),
        after.len()
    );
    assert_eq!(after, before, "Map must mint no trust beyond providers");
}

/// Promise class: durable invariant.
///
/// MEASURED: isolated roots loading admits a kernel-checked transparent
/// private proof for the stated arbitrary-tree law, with an empty direct
/// trusted-base delta. CLAIMED: neither a postulate nor a concrete example
/// stands in for the general `dom` membership result. THE GAP: privacy is
/// checked by the independent exact loader-visible export inventory above.
#[test]
fn map_dom_membership_law_is_checked_without_direct_trust() {
    let env = load_map();
    let law_name = format!("{MAP}.dom_preserves_membership");
    let law = env.globals[&law_name];
    assert!(
        matches!(env.env.lookup(law), Some(Decl::Transparent { .. })),
        "{law_name} must be a checked proof, not an assumed fact"
    );
    assert!(
        trusted_base_delta(&env.env, law).is_empty(),
        "the arbitrary-tree law must add no trust"
    );
}
