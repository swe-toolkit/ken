//! CAT-MIGRATE-TIER-C-DATA-VALUE Deque closeout controls.
//!
//! Promise class: durable invariants. Deque owns its exact checked carrier,
//! operation, and law family; publishes no catalog surface; consumes only
//! `Data.Collections.Derived`'s canonical `list_append` and `reverse`; and adds
//! no trust beyond that provider closure. The existing
//! `transparent_deque_bodies_have_exact_derived_head_occurrence_populations`
//! and concrete sequence tests in `cat_deque_acceptance` retain the per-body
//! provider-use and behavioral obligations.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::{parser, Decl as SurfaceDecl, ElabEnv, ElabError, ExportForm, ImportKind};
use ken_kernel::{Decl, GlobalId, Term};

const DEQUE: &str = "Data.Collections.Deque";
const DERIVED: &str = "Data.Collections.Derived";
const DEQUE_KEN_MD: &str = include_str!("../../../catalog/packages/Data/Collections/Deque.ken.md");

fn catalog_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn load(module: &str) -> (ElabEnv, Vec<GlobalId>) {
    let mut env = ElabEnv::new().expect("base environment");
    let owned = env
        .elaborate_module_from_roots(&[catalog_root()], module)
        .unwrap_or_else(|error| panic!("{module} must isolated-roots-load: {error:?}"));
    (env, owned)
}

fn expected_owned_names() -> BTreeSet<String> {
    [
        "Deque",
        "MkDeque",
        "MkPopPreserves",
        "PopPreserves",
        "deque_append_snoc_assoc",
        "deque_cong",
        "empty",
        "popBack",
        "popBack_pushBack",
        "popFront",
        "popFront_pushFront",
        "pushBack",
        "pushFront",
        "toList",
        "toList_pushBack",
        "toList_pushFront",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
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

fn declaration_references(declaration: &Decl) -> BTreeSet<GlobalId> {
    let mut references = BTreeSet::new();
    match declaration {
        Decl::Transparent { ty, body, .. } => {
            collect_references(ty, &mut references);
            collect_references(body, &mut references);
        }
        Decl::Opaque { ty, .. } | Decl::Primitive { ty, .. } => {
            collect_references(ty, &mut references);
        }
        Decl::Inductive(inductive) => {
            for term in &inductive.params {
                collect_references(term, &mut references);
            }
            for term in &inductive.indices {
                collect_references(term, &mut references);
            }
            collect_references(&inductive.former_type, &mut references);
            for constructor in &inductive.constructors {
                for term in &constructor.args {
                    collect_references(term, &mut references);
                }
                for term in &constructor.target_indices {
                    collect_references(term, &mut references);
                }
                collect_references(&constructor.type_, &mut references);
            }
        }
    }
    references
}

#[derive(Debug)]
struct PackageShape {
    providers: BTreeSet<(String, String)>,
    public_declarations: BTreeSet<String>,
    exports: BTreeSet<String>,
}

fn package_shape() -> PackageShape {
    let extracted = ken_elaborator::literate::extract_ken_md(DEQUE_KEN_MD)
        .expect("Deque literate source must extract");
    let declarations =
        parser::parse_decls(&extracted.source).expect("Deque extracted source must parse");
    let mut providers = BTreeSet::new();
    let mut public_declarations = BTreeSet::new();
    let mut exports = BTreeSet::new();

    for declaration in &declarations {
        if declaration.is_pub() {
            public_declarations.insert(declaration.unwrap_pub().name().to_owned());
        }
        match declaration.unwrap_pub() {
            SurfaceDecl::ImportDecl { module, kind, .. } => match kind {
                ImportKind::Selective(items) => {
                    providers.extend(items.iter().map(|item| (module.clone(), item.name.clone())));
                }
                ImportKind::Qualified | ImportKind::Aliased(_) => {
                    providers.insert((module.clone(), "*".to_owned()));
                }
            },
            SurfaceDecl::ExportDecl { form, .. } => {
                let items = match form {
                    ExportForm::Facade { items, .. } | ExportForm::InScope { items } => items,
                };
                exports.extend(
                    items
                        .iter()
                        .map(|item| item.rename.clone().unwrap_or_else(|| item.name.clone())),
                );
            }
            _ => {}
        }
    }

    PackageShape {
        providers,
        public_declarations,
        exports,
    }
}

fn qualified_owned_names(env: &ElabEnv) -> BTreeSet<String> {
    let prefix = format!("{DEQUE}.");
    env.globals
        .keys()
        .filter_map(|name| name.strip_prefix(&prefix).map(str::to_owned))
        .collect()
}

fn qualified_owned_ids(env: &ElabEnv) -> BTreeSet<GlobalId> {
    let prefix = format!("{DEQUE}.");
    env.globals
        .iter()
        .filter_map(|(name, id)| name.starts_with(&prefix).then_some(*id))
        .collect()
}

/// MEASURED: ordinary isolated roots loading installs exactly the sixteen named
/// Deque identities, returns only identities from that population, and executes
/// every checked fence. The resulting trusted base equals a fresh load of the
/// Derived provider closure. CLAIMED: Deque is standalone, owns exactly its
/// checked family, and adds no consumer-local trust. THE GAP: constructors are
/// not separate loader results; the qualified environment inventory closes that
/// part of the population independently.
#[test]
fn deque_owned_inventory_is_exact_and_standalone_with_zero_local_trust() {
    let (mut via_deque, loader_results) = load(DEQUE);
    let (via_derived, _) = load(DERIVED);
    assert_eq!(
        qualified_owned_names(&via_deque),
        expected_owned_names(),
        "Deque owned declaration inventory changed"
    );
    let owned_ids = qualified_owned_ids(&via_deque);
    assert!(
        loader_results.iter().all(|id| owned_ids.contains(id)),
        "every Deque loader result must belong to its qualified identity population"
    );
    assert_eq!(
        via_deque.env.trusted_base(),
        via_derived.env.trusted_base(),
        "Deque must add no trust beyond its Derived provider closure"
    );
    via_deque
        .execute_loaded_entry_checked_fences(DEQUE)
        .expect("Deque Definition and every checked fence must elaborate");
}

/// MEASURED: every checked Deque type and body refers to the exact nine-name
/// compiler floor plus the two canonical Derived identities. Independent roots
/// loads assign those providers the same `GlobalId`, and the semantic import
/// boundary selects exactly those two items. CLAIMED: Deque's complete direct
/// catalog dependency is `{Derived.list_append, Derived.reverse}`. THE GAP:
/// source forms `J` and `Refl` elaborate into kernel terms without provider
/// globals; the source-level free-name D0 records them as compiler syntax.
#[test]
fn deque_direct_provider_inventory_is_exact_and_canonical() {
    let base = ElabEnv::new().expect("base environment");
    let (via_deque, _) = load(DEQUE);
    let (via_derived, _) = load(DERIVED);
    let append_name = format!("{DERIVED}.list_append");
    let reverse_name = format!("{DERIVED}.reverse");
    let append = via_deque.globals[&append_name];
    let reverse = via_deque.globals[&reverse_name];
    assert_eq!(append, via_derived.globals[&append_name]);
    assert_eq!(reverse, via_derived.globals[&reverse_name]);

    let owned_ids = qualified_owned_ids(&via_deque);
    let mut external = BTreeSet::new();
    for id in &owned_ids {
        if let Some(declaration) = via_deque.env.lookup(*id) {
            external.extend(declaration_references(declaration));
        }
    }
    for id in &owned_ids {
        external.remove(id);
    }
    let expected_floor = [
        "Cons", "Equal", "List", "Nil", "None", "Option", "Pair", "Some", "mk_pair",
    ]
    .into_iter()
    .map(|name| base.globals[name])
    .collect::<BTreeSet<_>>();
    let expected_external = expected_floor
        .into_iter()
        .chain([append, reverse])
        .collect::<BTreeSet<_>>();
    assert_eq!(
        external, expected_external,
        "Deque's checked external identity inventory changed"
    );
    assert_eq!(
        package_shape().providers,
        BTreeSet::from([
            (DERIVED.to_owned(), "list_append".to_owned()),
            (DERIVED.to_owned(), "reverse".to_owned()),
        ]),
        "Deque must select exactly Derived.list_append and Derived.reverse"
    );
}

/// MEASURED: the semantic interface inventory contains no direct publication
/// or re-export, and selective-import queries reject every direct Deque surface
/// by its exact qualified name. CLAIMED: Deque publishes no catalog surface.
/// THE GAP: none; the exact owned inventory supplies the complete direct-name
/// population, including both inductive constructors.
#[test]
fn deque_loader_visible_inventory_is_empty() {
    let shape = package_shape();
    assert_eq!(
        shape.public_declarations,
        BTreeSet::new(),
        "Deque must not directly publish a declaration"
    );
    assert_eq!(
        shape.exports,
        BTreeSet::new(),
        "Deque must not re-export a declaration"
    );

    let (mut env, _) = load(DEQUE);
    for (index, surface) in expected_owned_names().iter().enumerate() {
        let source = format!("import {DEQUE} ({surface} as deque_private_{index})");
        match env.elaborate_file(&source) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{DEQUE}.{surface}"));
            }
            Err(other) => {
                panic!("Deque import of {surface} failed for the wrong reason: {other:?}")
            }
            Ok(_) => panic!("Deque unexpectedly published {surface}"),
        }
    }
}

/// MEASURED: independently withholding either item from Deque's sole semantic
/// import makes the unchanged extracted module reach that item's exact
/// `UnresolvedCon` boundary, while the ordinary source loads above. CLAIMED:
/// both Derived imports are individually necessary rather than decorative. THE
/// GAP: none; each negative changes only one import selection and preloads the
/// unchanged provider closure as a positive availability control.
#[test]
fn both_deque_provider_imports_are_individually_load_bearing() {
    let extracted = ken_elaborator::literate::extract_ken_md(DEQUE_KEN_MD)
        .expect("Deque literate source must extract");
    let declarations =
        parser::parse_decls(&extracted.source).expect("Deque extracted source must parse");
    let imports = declarations
        .iter()
        .filter_map(|declaration| match declaration.unwrap_pub() {
            SurfaceDecl::ImportDecl {
                kind: ImportKind::Selective(items),
                span,
                ..
            } => Some((span.start..span.end, items)),
            SurfaceDecl::ImportDecl { .. } => {
                panic!("Deque's sole import must remain selective")
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(
        imports.len(),
        1,
        "Deque must retain one semantic import declaration"
    );
    let local_binding = |provider_name: &str| {
        imports[0]
            .1
            .iter()
            .find(|item| item.name == provider_name)
            .map(|item| item.rename.clone().unwrap_or_else(|| item.name.clone()))
            .unwrap_or_else(|| panic!("Deque import must contain {provider_name}"))
    };

    for (retained, missing) in [("reverse", "list_append"), ("list_append", "reverse")] {
        let retained_local = local_binding(retained);
        let missing_local = local_binding(missing);
        let retained_item = if retained_local == retained {
            retained.to_owned()
        } else {
            format!("{retained} as {retained_local}")
        };
        let mut source = extracted.source.clone();
        source.replace_range(
            imports[0].0.clone(),
            &format!("import {DERIVED} ({retained_item})"),
        );
        let mut env = ElabEnv::new().expect("base environment");
        env.elaborate_module_from_roots(&[catalog_root()], DERIVED)
            .expect("Derived provider closure must be available");
        match env.elaborate_file(&source) {
            Err(ElabError::UnresolvedCon { name, .. }) => assert_eq!(name, missing_local),
            Err(other) => {
                panic!("withholding {missing} failed for the wrong reason: {other:?}")
            }
            Ok(_) => panic!("Deque unexpectedly elaborated without imported {missing}"),
        }
    }
}
