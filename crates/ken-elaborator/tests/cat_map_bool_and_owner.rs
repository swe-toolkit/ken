//! CAT-MIGRATE-TIER-C-DATA-VALUE Map `bool_and` owner-migration controls.
//!
//! Promise class: durable invariants. Map consumes the canonical
//! LawfulClasses `bool_and` function and attached laws, owns no equivalent
//! function or proof identity, and uses LC's existing `intro` proposition in
//! place of its former proposition-equivalent `true_intro`. The private LF
//! duplicate remains outside this increment.

#[path = "support/catalog_or.rs"]
mod catalog_or;

use std::collections::BTreeSet;

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{convert, convert_type, Context, Decl, GlobalId, Term};

const MAP_KEN_MD: &str = include_str!("../../../catalog/packages/Data/Collections/Map.ken.md");
const LAWFUL: &str = "Core.Classes.LawfulClasses";
const LC_BOOL_AND_SURFACES: [&str; 7] = [
    "bool_and",
    "bool_and::assoc",
    "bool_and::comm",
    "bool_and::idempotent",
    "bool_and::intro",
    "bool_and::left_identity",
    "bool_and::right_identity",
];

fn term_reference_count(term: &Term, target: GlobalId) -> usize {
    let here = usize::from(matches!(term, Term::Const { id, .. } if *id == target));
    here + term
        .children()
        .into_iter()
        .map(|child| term_reference_count(child, target))
        .sum::<usize>()
}

fn module_transparent_kernel_equivalents(
    env: &ElabEnv,
    module: &str,
    provider: GlobalId,
) -> BTreeSet<String> {
    let (provider_level_params, provider_ty, provider_body) = match env.env.lookup(provider) {
        Some(Decl::Transparent {
            level_params,
            ty,
            body,
            ..
        }) => (level_params, ty, body),
        other => panic!("provider must be transparent, got {other:?}"),
    };
    assert!(provider_level_params.is_empty());

    let prefix = format!("{module}.");
    let context = Context::new();
    env.globals
        .iter()
        .filter_map(|(name, id)| {
            let local_name = name.strip_prefix(&prefix)?;
            let (level_params, ty, body) = match env.env.lookup(*id) {
                Some(Decl::Transparent {
                    level_params,
                    ty,
                    body,
                    ..
                }) => (level_params, ty, body),
                _ => return None,
            };
            if !level_params.is_empty() {
                return None;
            }
            (convert_type(&env.env, &context, ty, provider_ty)
                && convert(&env.env, &context, provider_ty, body, provider_body))
            .then(|| local_name.to_owned())
        })
        .collect()
}

fn map_dependency_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("base environment");
    catalog_or::load_core_logic_compare(&mut env);
    catalog_or::expose_core_logic_transport(&mut env);
    catalog_or::load_derived_fixture(&mut env);
    for imported in LC_BOOL_AND_SURFACES {
        assert!(
            env.globals.remove(imported).is_some(),
            "fixture must withhold selectively imported `{imported}`"
        );
    }
    env.elaborate_module_from_roots(&[catalog_or::catalog_root()], "Data.Sums.Combinators")
        .expect("Map's canonical is_some provider must roots-load");
    env
}

/// MEASURED: the dependency fixture retains LC's qualified module identities
/// while withholding every flat `bool_and` alias. Map's real selective import
/// leaves direct body references to the exact LC function and four laws it uses;
/// the two relocated identity laws remain LC-owned and unused by Map. No LC
/// identity belongs to Map's result set, no Map-owned transparent definition is
/// kernel-equivalent to the provider, and the trust set is unchanged. The
/// former `true_intro` consumer directly mentions LC's `intro`. CLAIMED: the
/// migration leaves one
/// Map-visible canonical family and mints no competing identity. THE GAP: LF's
/// private duplicate is explicitly deferred to its sequenced consolidation WP.
#[test]
fn map_resolves_exact_lawfulclasses_bool_and_family_without_local_identity() {
    let mut env = map_dependency_env();
    let providers = LC_BOOL_AND_SURFACES.map(|surface| env.globals[&format!("{LAWFUL}.{surface}")]);
    for surface in LC_BOOL_AND_SURFACES {
        assert!(!env.globals.contains_key(surface));
    }

    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let map_ids = env
        .elaborate_ken_md_file(MAP_KEN_MD)
        .expect("Map must elaborate through its canonical LC bool_and import")
        .into_iter()
        .collect::<BTreeSet<_>>();
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    assert_eq!(before, after, "the bool_and relocation must add zero trust");

    let mut reference_counts = [0; LC_BOOL_AND_SURFACES.len()];
    for id in &map_ids {
        if let Some((_, ty)) = env.env.const_type(*id) {
            for (index, provider) in providers.iter().enumerate() {
                reference_counts[index] += term_reference_count(&ty, *provider);
            }
        }
        if let Some((_, body)) = env.env.transparent_body(*id) {
            for (index, provider) in providers.iter().enumerate() {
                reference_counts[index] += term_reference_count(&body, *provider);
            }
        }
    }
    for ((surface, provider), references) in LC_BOOL_AND_SURFACES
        .into_iter()
        .zip(providers)
        .zip(reference_counts)
    {
        assert!(
            !map_ids.contains(&provider),
            "Map must not absorb imported `{surface}` into its owned identities"
        );
        if matches!(
            surface,
            "bool_and::left_identity" | "bool_and::right_identity"
        ) {
            assert_eq!(references, 0, "Map does not directly consume `{surface}`");
        } else {
            assert!(
                references > 0,
                "Map bodies must retain LC's exact `{surface}` identity"
            );
        }
    }
    let owned_family = env
        .globals
        .iter()
        .filter_map(|(name, id)| {
            (map_ids.contains(id) && (name == "bool_and" || name.contains("bool_and::")))
                .then_some(name.clone())
        })
        .collect::<BTreeSet<_>>();
    assert_eq!(
        owned_family,
        BTreeSet::new(),
        "Map must own no bool_and family identity"
    );
    assert!(
        !env.globals.contains_key("bool_and::true_intro"),
        "the proposition-equivalent true_intro identity must collapse into LC intro"
    );
    let intro = env.globals[&format!("{LAWFUL}.bool_and::intro")];
    let intro_consumer = env.globals["order_equiv_key_true_from_order_equiv"];
    let (_, consumer_body) = env
        .env
        .transparent_body(intro_consumer)
        .expect("the Map intro consumer must be transparent");
    assert!(
        term_reference_count(&consumer_body, intro) > 0,
        "Map's former true_intro consumer must retain LC's intro GlobalId"
    );

    let owned_bindings = env
        .globals
        .iter()
        .filter(|(_, id)| map_ids.contains(id))
        .map(|(name, id)| (name.clone(), *id))
        .collect::<Vec<_>>();
    for (name, id) in owned_bindings {
        env.globals
            .insert(format!("Data.Collections.Map.{name}"), id);
    }
    assert_eq!(
        module_transparent_kernel_equivalents(
            &env,
            "Data.Collections.Map",
            env.globals[&format!("{LAWFUL}.bool_and")],
        ),
        BTreeSet::new(),
        "Map must retain no kernel-equivalent local bool_and definition"
    );
    let local_family = env
        .globals
        .keys()
        .filter_map(|name| {
            name.strip_prefix("Data.Collections.Map.")
                .filter(|local| *local == "bool_and" || local.starts_with("bool_and::"))
                .map(str::to_owned)
        })
        .collect::<BTreeSet<_>>();
    assert_eq!(
        local_family,
        BTreeSet::new(),
        "Map must retain no local identity attached to bool_and"
    );
}

fn replace_exactly_once(source: &str, from: &str, to: &str) -> String {
    assert_eq!(source.matches(from).count(), 1);
    source.replacen(from, to, 1)
}

fn assert_import_mutation_fails(source: &str, label: &str) {
    let mut env = map_dependency_env();
    match env.elaborate_ken_md_file(source) {
        Err(ElabError::UnresolvedCon { name, .. }) => assert_eq!(
            name, "bool_and",
            "{label} must fail at Map's unqualified bool_and use"
        ),
        other => panic!("{label} must fail with UnresolvedCon bool_and, got {other:?}"),
    }
}

/// MEASURED: withdrawing Map's LC import or binding the canonical subject under
/// the wrong local spelling makes the real Map source fail specifically at
/// `bool_and`, while the fixture retains every unrelated legacy dependency.
/// CLAIMED: the LC edge is load-bearing and cannot be replaced by fixture
/// ambient resolution. THE GAP: the provider's exact public inventory is owned
/// by `cat_bool_pub_export`; this is the consumer-side control.
#[test]
fn map_bool_and_import_withdrawal_and_wrong_name_fail() {
    let import = "import Core.Classes.LawfulClasses (bool_and)\n\n";
    let withdrawn = replace_exactly_once(MAP_KEN_MD, import, "");
    assert_import_mutation_fails(&withdrawn, "withdrawn LC import");

    let wrong_name = replace_exactly_once(
        MAP_KEN_MD,
        import,
        "import Core.Classes.LawfulClasses (bool_and as not_bool_and)\n\n",
    );
    assert_import_mutation_fails(&wrong_name, "wrong-name LC import");
}
