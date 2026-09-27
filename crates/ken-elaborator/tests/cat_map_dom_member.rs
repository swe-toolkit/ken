//! CAT-MAP-DOM-MEMBER-LAW: checked private domain-membership law and
//! unchanged loader-visible Map surface and trusted-base boundary.

#[path = "support/catalog_or.rs"]
mod catalog_or;
#[path = "support/catalog_publication.rs"]
mod catalog_publication;

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use ken_elaborator::{foreign::trusted_base_delta, ElabEnv};
use ken_kernel::{Decl, GlobalId, Term};

const MAP: &str = "Data.Collections.Map";
const MAP_KEN_MD: &str = include_str!("../../../catalog/packages/Data/Collections/Map.ken.md");

fn catalog_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn load_map_with_owned() -> (ElabEnv, Vec<GlobalId>) {
    let mut env = ElabEnv::new().expect("base environment");
    let owned = env
        .elaborate_module_from_roots(&[catalog_root()], MAP)
        .expect("Map must roots-load through its declared imports");
    (env, owned)
}

fn load_map() -> ElabEnv {
    load_map_with_owned().0
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

fn map_qualified_bindings(env: &ElabEnv) -> BTreeMap<String, GlobalId> {
    let prefix = format!("{MAP}.");
    env.globals
        .iter()
        .filter(|(name, _)| name.starts_with(&prefix))
        .map(|(name, id)| (name.clone(), *id))
        .collect()
}

fn map_instance_id(env: &ElabEnv, owned: &[GlobalId], carrier: &str) -> GlobalId {
    let spelling = format!("Membership_instance_{MAP}.{carrier}");
    let matches = owned
        .iter()
        .copied()
        .filter(|id| env.globals.get(&spelling) == Some(id))
        .collect::<Vec<_>>();
    assert_eq!(
        matches.len(),
        1,
        "{spelling} must select one loader-owned Map instance"
    );
    matches[0]
}

fn references(term: &Term, id: GlobalId) -> bool {
    matches!(term, Term::Const { id: found, .. } if *found == id)
        || term
            .children()
            .into_iter()
            .any(|child| references(child, id))
}

fn projects_member_from(term: &Term, dictionary: GlobalId) -> bool {
    matches!(term, Term::Proj1(field) if matches!(field.as_ref(),
        Term::Proj2(record) if matches!(record.as_ref(), Term::Const { id, .. } if *id == dictionary)))
        || term
            .children()
            .into_iter()
            .any(|child| projects_member_from(child, dictionary))
}

/// Promise class: durable checked-identity invariant.
///
/// MEASURED: real roots loading omits the private example declarations from
/// Map's compiled module, then entry-fence execution checks every example
/// while preserving every Map-qualified name and ID and the trusted-base set.
/// CLAIMED: private Map law applications and ground comparator witnesses are
/// kernel checked in their owner, not published or paid for with new trust.
/// The key/edge instance observations reach `.member` through their two
/// carrier-resolved `Membership` dictionaries, not through the adapter helpers.
/// THE GAP: the exact nine-name loader-visible public interface is measured
/// separately by `map_dom_member_public_surface_transition_sentinel`; the
/// checked examples are intentionally not runtime observations. Both new
/// rows return True, so a constant-True instance remains outside their
/// no-use-site-comparator discriminator; absent-query rows still use helpers.
#[test]
fn map_private_checked_examples_preserve_names_ids_and_trust() {
    let (mut env, owned) = load_map_with_owned();
    let before = map_qualified_bindings(&env);
    let trust_before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    assert!(
        !env.globals.contains_key("map_example_size_node"),
        "example-only laws must not be tangled into the loaded module"
    );
    let instance_examples = [
        (
            "map_example_instance_key_observed",
            "map_example_key_instance_dictionary",
            "map_example_instance_key_stored_comparator_finds_key",
            "OrderedKeyMembership",
        ),
        (
            "map_example_instance_relation_observed",
            "map_example_relation_instance_dictionary",
            "map_example_instance_relation_stored_comparator_finds_edge",
            "RelationEdgeMembership",
        ),
    ];
    for (observed, dictionary, theorem, _) in instance_examples {
        for name in [observed, dictionary, theorem] {
            assert!(
                !env.globals.contains_key(name),
                "{name} must be minted by the fence"
            );
            assert!(
                !before.contains_key(&format!("{MAP}.{name}")),
                "{name} must not be a loader-visible Map declaration"
            );
        }
    }
    env.execute_loaded_entry_checked_fences(MAP)
        .expect("Map's generic laws and instance-resolved ground examples must check");
    for name in [
        "map_example_size_node",
        "map_example_dom_node",
        "map_example_empty_to_list",
        "map_example_empty_ordered",
        "map_example_empty_key_view",
        "map_example_down_tree_ordered",
        "map_example_down_key_view",
        "map_example_down_relation_view",
        "map_example_stored_comparator_finds_the_key",
        "map_example_fresh_canonical_comparator_misses_the_same_key",
        "map_example_stored_comparator_rejects_an_absent_key",
        "map_example_stored_relation_comparator_rejects_an_absent_source",
        "map_example_stored_relation_comparator_rejects_an_absent_target",
        "map_example_stored_relation_comparator_finds_the_edge",
        "map_example_fresh_canonical_comparator_misses_the_same_edge",
        "map_example_key_instance_dictionary",
        "map_example_relation_instance_dictionary",
        "map_example_instance_key_observed",
        "map_example_instance_relation_observed",
        "map_example_instance_key_stored_comparator_finds_key",
        "map_example_instance_relation_stored_comparator_finds_edge",
    ] {
        let id = env.globals[name];
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "Map checked example {name} must be a transparent declaration"
        );
    }
    assert_eq!(
        map_qualified_bindings(&env),
        before,
        "Map checked examples must not leak or rebind qualified identities"
    );
    assert_eq!(
        env.env.trusted_base().into_iter().collect::<BTreeSet<_>>(),
        trust_before,
        "Map checked examples must not add trusted assumptions"
    );
    let key = map_instance_id(&env, &owned, "OrderedKeyMembership");
    let relation = map_instance_id(&env, &owned, "RelationEdgeMembership");
    assert_ne!(
        key, relation,
        "distinct Map carriers must use distinct instances"
    );
    for (observed, dictionary, _, carrier) in instance_examples {
        let instance = map_instance_id(&env, &owned, carrier);
        let observed_id = env.globals[observed];
        let dictionary_id = env.globals[dictionary];
        for (name, id) in [(observed, observed_id), (dictionary, dictionary_id)] {
            assert!(!owned.contains(&id), "{name} must not be tangled into Map");
        }
        let Some(Decl::Transparent {
            body: observed_body,
            ..
        }) = env.env.lookup(observed_id)
        else {
            panic!("{observed} must be a checked ground observation");
        };
        assert!(
            projects_member_from(observed_body, dictionary_id),
            "{observed} must project Membership.member from {dictionary}"
        );
        let Some(Decl::Transparent {
            body: dictionary_body,
            ..
        }) = env.env.lookup(dictionary_id)
        else {
            panic!("{dictionary} must be a checked carrier-resolved dictionary");
        };
        assert!(
            references(dictionary_body, instance),
            "{dictionary} must resolve to Map's loader-owned {carrier} instance"
        );
    }
}
