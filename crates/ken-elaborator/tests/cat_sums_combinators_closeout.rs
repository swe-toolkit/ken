//! CAT-MIGRATE-TIER-C-DATA-VALUE Sums.Combinators closeout controls.
//!
//! Promise class: durable invariants. Sums.Combinators owns the exact checked
//! combinator floor, depends only on compiler-floor sum/equality identities,
//! adds no trust, and directly publishes only its canonical `is_some` identity.
//! The full attached-surface inventory remains owned by
//! `class_owner_provider_loader_visible_inventories_are_exact` in
//! `cat_bool_pub_export`; Map's direct edge and withdrawal controls remain in
//! `map_build_acceptance`.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::{parser, Decl as SurfaceDecl, ElabEnv, ElabError};
use ken_kernel::{GlobalId, Term};

const SUMS: &str = "Data.Sums.Combinators";
const SUMS_KEN_MD: &str = include_str!("../../../catalog/packages/Data/Sums/Combinators.ken.md");

fn catalog_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn load_sums() -> (ElabEnv, Vec<GlobalId>) {
    let mut env = ElabEnv::new().expect("base environment");
    let owned = env
        .elaborate_module_from_roots(&[catalog_root()], SUMS)
        .expect("Sums.Combinators must isolated-roots-load");
    (env, owned)
}

fn collect_references(term: &Term, references: &mut BTreeSet<GlobalId>) {
    match term {
        Term::Const { id, .. } | Term::IndFormer { id, .. } | Term::Constructor { id, .. } => {
            references.insert(*id);
        }
        Term::Elim { fam, .. } => {
            references.insert(*fam);
        }
        _ => {}
    }
    for child in term.children() {
        collect_references(child, references);
    }
}

fn term_mentions(term: &Term, target: GlobalId) -> bool {
    match term {
        Term::Const { id, .. } | Term::IndFormer { id, .. } | Term::Constructor { id, .. }
            if *id == target =>
        {
            true
        }
        Term::Elim { fam, .. } if *fam == target => true,
        _ => term
            .children()
            .into_iter()
            .any(|child| term_mentions(child, target)),
    }
}

fn direct_imports() -> BTreeSet<String> {
    let extracted = ken_elaborator::literate::extract_ken_md(SUMS_KEN_MD)
        .expect("Sums.Combinators literate source must extract");
    parser::parse_decls(&extracted.source)
        .expect("Sums.Combinators extracted source must parse")
        .iter()
        .filter_map(|declaration| match declaration.unwrap_pub() {
            SurfaceDecl::ImportDecl { module, .. } => Some(module.clone()),
            _ => None,
        })
        .collect()
}

fn expected_owned_names() -> BTreeSet<String> {
    [
        "Either",
        "Left",
        "Right",
        "and_then",
        "and_then::err",
        "and_then::ok",
        "either",
        "either::left",
        "either::right",
        "get_or_else",
        "get_or_else::none",
        "get_or_else::some",
        "is_some",
        "is_some::none",
        "is_some::some",
        "map_err",
        "map_err::err",
        "map_err::ok",
        "map_left",
        "map_left::left",
        "map_left::right",
        "map_right",
        "map_right::left",
        "map_right::right",
        "or_else",
        "or_else::none",
        "or_else::none_rhs",
        "or_else::some",
        "swap",
        "swap::involutive",
        "unwrap_or",
        "unwrap_or::err",
        "unwrap_or::ok",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}

/// MEASURED: isolated roots loading yields exactly the 33 named combinator,
/// proof, family, and constructor identities; all direct term references
/// outside that population are exactly the compiler-floor sum/equality set;
/// the semantic import set and trusted-base delta are empty. Every checked
/// fence executes. CLAIMED: Sums.Combinators is standalone, owns exactly its
/// advertised floor, and has no catalog-provider or trust dependency. THE GAP:
/// none; the loader identities and their checked types/bodies form the complete
/// entry-unit population, including generated constructors.
#[test]
fn sums_owned_inventory_and_compiler_floor_closure_are_exact() {
    let mut env = ElabEnv::new().expect("base environment");
    let before_trust = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned_results = env
        .elaborate_module_from_roots(&[catalog_root()], SUMS)
        .expect("Sums.Combinators must isolated-roots-load");
    let after_trust = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let prefix = format!("{SUMS}.");
    let owned_names = env
        .globals
        .keys()
        .filter_map(|name| name.strip_prefix(&prefix).map(str::to_owned))
        .collect::<BTreeSet<_>>();
    assert_eq!(
        owned_names,
        expected_owned_names(),
        "Sums owned declaration inventory changed"
    );
    let owned_ids = env
        .globals
        .iter()
        .filter_map(|(name, id)| name.starts_with(&prefix).then_some(*id))
        .collect::<BTreeSet<_>>();
    assert!(
        owned_results.iter().all(|id| owned_ids.contains(id)),
        "every loader result must belong to the Sums identity population"
    );

    let mut external = BTreeSet::new();
    for id in &owned_ids {
        if let Some((_, ty)) = env.env.const_type(*id) {
            collect_references(&ty, &mut external);
        }
        if let Some((_, body)) = env.env.transparent_body(*id) {
            collect_references(&body, &mut external);
        }
    }
    for id in &owned_ids {
        external.remove(id);
    }
    let expected_floor = [
        "Proved", "Bool", "True", "False", "Option", "None", "Some", "Result", "Err", "Ok", "Equal",
    ]
    .into_iter()
    .map(|name| env.globals[name])
    .collect::<BTreeSet<_>>();
    assert_eq!(
        external, expected_floor,
        "Sums direct external identity closure must remain compiler-floor only"
    );
    assert_eq!(
        direct_imports(),
        BTreeSet::new(),
        "Sums must have no catalog provider import"
    );
    assert_eq!(after_trust, before_trust, "Sums must add zero trust");

    env.execute_loaded_entry_checked_fences(SUMS)
        .expect("Sums Definition and every checked fence must elaborate");
}

/// MEASURED: selective-import queries for every owned direct surface succeed
/// only for `is_some`; a typed wrapper retains the exact qualified provider
/// `GlobalId`. CLAIMED: the direct loader-visible inventory is exactly the
/// canonical `is_some`, with no private carrier or combinator exposed. THE GAP:
/// attached surfaces are intentionally delegated to the existing exhaustive
/// publication-query control named in this file's module documentation.
#[test]
fn sums_direct_loader_surface_is_exact_canonical_is_some() {
    let (mut env, _) = load_sums();
    let direct_surfaces = expected_owned_names()
        .into_iter()
        .filter(|name| !name.contains("::"))
        .collect::<BTreeSet<_>>();
    let mut published = BTreeSet::new();
    for (index, surface) in direct_surfaces.iter().enumerate() {
        let source = format!("import {SUMS} ({surface} as sums_surface_{index})");
        match env.elaborate_file(&source) {
            Ok(_) => {
                published.insert(surface.clone());
            }
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{SUMS}.{surface}"));
            }
            Err(other) => panic!("Sums import of {surface} failed for the wrong reason: {other:?}"),
        }
    }
    assert_eq!(
        published,
        BTreeSet::from(["is_some".to_owned()]),
        "Sums direct loader-visible inventory must be exactly is_some"
    );

    let provider = env.globals[&format!("{SUMS}.is_some")];
    env.elaborate_file(
        "import Data.Sums.Combinators (is_some as sums_is_some)\n\
         fn sums_closeout_use (a : Type) (x : Option a) : Bool = sums_is_some a x",
    )
    .expect("the canonical Sums is_some must be usable through selective import");
    let wrapper = env.globals["sums_closeout_use"];
    let (_, body) = env
        .env
        .transparent_body(wrapper)
        .expect("the import consumer must be transparent");
    assert!(
        term_mentions(&body, provider),
        "the direct import must retain Sums's owned is_some GlobalId"
    );
}
