//! CAT-MIGRATE-TIER-C-DATA-VALUE Vector closeout controls.
//!
//! Promise class: durable invariants. Vector owns its exact checked indexed
//! families, operations, and computation theorems; consumes no catalog
//! provider; publishes no catalog surface; and adds no trust. The existing
//! `cat_vec_acceptance` target retains the family-index, computation, and
//! impossible-call behavior obligations.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::{parser, Decl as SurfaceDecl, ElabEnv, ElabError, ExportForm, ImportKind};
use ken_kernel::{Decl, GlobalId, Term};

const VECTOR: &str = "Data.Vector.Vector";
const TRANSPORT: &str = "Core.Logic.Transport";
const VECTOR_KEN_MD: &str = include_str!("../../../catalog/packages/Data/Vector/Vector.ken.md");

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
        "FSuc",
        "FZero",
        "Fin",
        "VCons",
        "VNil",
        "Vec",
        "head",
        "head_vcons",
        "lookup",
        "lookup_fzero",
        "map",
        "map_vnil",
        "tail",
        "tail_vcons",
        "zip_with",
        "zip_with_vnil",
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
    let extracted = ken_elaborator::literate::extract_ken_md(VECTOR_KEN_MD)
        .expect("Vector literate source must extract");
    let declarations =
        parser::parse_decls(&extracted.source).expect("Vector extracted source must parse");
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
    let prefix = format!("{VECTOR}.");
    env.globals
        .keys()
        .filter_map(|name| name.strip_prefix(&prefix).map(str::to_owned))
        .collect()
}

fn qualified_owned_ids(env: &ElabEnv) -> BTreeSet<GlobalId> {
    let prefix = format!("{VECTOR}.");
    env.globals
        .iter()
        .filter_map(|(name, id)| name.starts_with(&prefix).then_some(*id))
        .collect()
}

/// MEASURED: ordinary isolated roots loading installs exactly the sixteen named
/// Vector identities, returns only identities from that population, and
/// executes every checked fence. The trusted base equals the compiler base.
/// CLAIMED: Vector is standalone, owns exactly its checked family, and adds no
/// local trust. THE GAP: constructors are not separate loader results; the
/// qualified environment inventory closes that part of the population.
#[test]
fn vector_owned_inventory_is_exact_and_standalone_with_zero_local_trust() {
    let base = ElabEnv::new().expect("base environment");
    let (mut via_vector, loader_results) = load(VECTOR);
    assert_eq!(
        qualified_owned_names(&via_vector),
        expected_owned_names(),
        "Vector owned declaration inventory changed"
    );
    let owned_ids = qualified_owned_ids(&via_vector);
    assert!(
        loader_results.iter().all(|id| owned_ids.contains(id)),
        "every Vector loader result must belong to its qualified identity population"
    );
    assert_eq!(
        via_vector.env.trusted_base(),
        base.env.trusted_base(),
        "Vector must add no trust beyond the compiler base"
    );
    via_vector
        .execute_loaded_entry_checked_fences(VECTOR)
        .expect("Vector Definition and every checked fence must elaborate");
}

/// MEASURED: every checked Vector type and body refers externally to exactly
/// the compiler identities `{Proved, Nat, Zero, Suc, Equal}`. The parsed module
/// contains no catalog import, public declaration, or re-export. CLAIMED:
/// Vector is already a provider-free, consumer-only catalog unit and publishes
/// no catalog surface. THE GAP: source forms `Type` and `Refl` elaborate into
/// kernel terms without separate provider globals.
#[test]
fn vector_has_exactly_the_compiler_floor_and_no_catalog_interface() {
    let base = ElabEnv::new().expect("base environment");
    let (via_vector, _) = load(VECTOR);
    let owned_ids = qualified_owned_ids(&via_vector);
    let mut external = BTreeSet::new();
    for id in &owned_ids {
        if let Some(declaration) = via_vector.env.lookup(*id) {
            external.extend(declaration_references(declaration));
        }
    }
    for id in &owned_ids {
        external.remove(id);
    }
    let expected_external = ["Proved", "Nat", "Zero", "Suc", "Equal"]
        .into_iter()
        .map(|name| base.globals[name])
        .collect::<BTreeSet<_>>();
    assert_eq!(
        external, expected_external,
        "Vector's checked external identity inventory changed"
    );
    for name in ["Proved", "Nat", "Zero", "Suc", "Equal"] {
        assert_eq!(
            via_vector.globals[name], base.globals[name],
            "Vector must retain the compiler's canonical `{name}` identity"
        );
    }
    assert_ne!(
        via_vector.globals["map"],
        via_vector.globals[&format!("{VECTOR}.map")],
        "the compiler `map` and Vector's private `map` must remain distinct identities"
    );

    let shape = package_shape();
    assert_eq!(
        shape.providers,
        BTreeSet::new(),
        "Vector must not acquire a catalog provider edge"
    );
    assert_eq!(
        shape.public_declarations,
        BTreeSet::new(),
        "Vector must not directly publish a declaration"
    );
    assert_eq!(
        shape.exports,
        BTreeSet::new(),
        "Vector must not re-export a declaration"
    );
}

/// MEASURED: a known public Transport item succeeds through the same selective
/// import path, while every direct Vector name rejects with its exact qualified
/// `UnboundName`. CLAIMED: Vector's loader-visible catalog inventory is empty.
/// THE GAP: none; the exact owned inventory supplies the complete direct-name
/// population, including both indexed families' constructors.
#[test]
fn vector_loader_visible_inventory_is_empty() {
    let mut positive = ElabEnv::new().expect("base environment");
    positive
        .elaborate_module_from_roots(&[catalog_root()], TRANSPORT)
        .expect("Transport positive-control provider must roots-load");
    positive
        .elaborate_file(&format!(
            "import {TRANSPORT} (cong as vector_closeout_public_control)"
        ))
        .expect("the selective-import positive control must succeed");

    let (mut env, _) = load(VECTOR);
    for (index, surface) in expected_owned_names().iter().enumerate() {
        let source = format!("import {VECTOR} ({surface} as vector_private_{index})");
        match env.elaborate_file(&source) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{VECTOR}.{surface}"));
            }
            Err(other) => {
                panic!("Vector import of {surface} failed for the wrong reason: {other:?}")
            }
            Ok(_) => panic!("Vector unexpectedly published {surface}"),
        }
    }
}
