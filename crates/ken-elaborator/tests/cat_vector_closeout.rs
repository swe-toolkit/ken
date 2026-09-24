//! CAT-MIGRATE-TIER-C-DATA-VALUE Vector closeout controls.
//!
//! Vector owns checked indexed families, operations, computation theorems,
//! and one private map identity law. It consumes only LawfulFunctors/Transport,
//! publishes no catalog surface, and adds no trust beyond those providers.
//! `cat_vec_acceptance` retains the family-index, computation, and
//! impossible-call behavior obligations.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::{parser, Decl as SurfaceDecl, ElabEnv, ElabError, ExportForm, ImportKind};
use ken_kernel::{Decl, GlobalId, Level, Term};

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
        "vec_map_identity",
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

/// Promise class: transition sentinel for the owned declarations in this
/// proof-only increment. Retire or rebaseline at the first separately authorized
/// Vector declaration extension; this inventory is not a permanent API promise.
/// MEASURED: ordinary isolated roots loading installs these seventeen checked
/// Vector identities, returns only identities from that population, and
/// executes every checked fence. Provider-closure trust is unchanged by Vector.
/// CLAIMED: only the one private law extends the owned inventory; it adds no
/// local trust. THE GAP: constructors are not separate loader results; the
/// qualified environment inventory closes that part of the population.
#[test]
fn vector_owned_inventory_transition_sentinel_and_zero_local_trust() {
    let mut provider_only = ElabEnv::new().expect("provider environment");
    for provider in ["Core.Classes.LawfulFunctors", "Core.Logic.Transport"] {
        provider_only
            .elaborate_module_from_roots(&[catalog_root()], provider)
            .unwrap_or_else(|error| panic!("provider {provider} must roots-load: {error:?}"));
    }
    let trust_before: BTreeSet<_> = provider_only.env.trusted_base().into_iter().collect();
    provider_only
        .elaborate_module_from_roots(&[catalog_root()], VECTOR)
        .expect("Vector must roots-load over the existing provider closure");
    let trust_after: BTreeSet<_> = provider_only.env.trusted_base().into_iter().collect();
    eprintln!(
        "Vector full-closure trust: before {}, after {}",
        trust_before.len(),
        trust_after.len()
    );
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
        trust_after, trust_before,
        "Vector must add no trust beyond its checked provider closure"
    );
    via_vector
        .execute_loaded_entry_checked_fences(VECTOR)
        .expect("Vector Definition and every checked fence must elaborate");
}

/// Promise class: transition sentinel for this proof-only dependency edge;
/// retire at the first separately authorized Vector provider change.
/// MEASURED: checked Vector references exactly the compiler floor plus the
/// canonical imported `idf`/`cong` identities; parsed imports list exactly
/// those two providers, with no public declaration or re-export. CLAIMED:
/// the private law uses the two authorized providers and Vector publishes no
/// catalog surface. THE GAP: `Type` and `Refl` elaborate without separate
/// provider globals; checked GlobalId comparisons close the provider edge.
#[test]
fn vector_imports_exact_checked_providers_and_publishes_nothing() {
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
    let mut expected_external = ["Proved", "Nat", "Zero", "Suc", "Equal"]
        .into_iter()
        .map(|name| base.globals[name])
        .collect::<BTreeSet<_>>();
    for name in [
        "Core.Classes.LawfulFunctors.idf",
        "Core.Logic.Transport.cong",
    ] {
        expected_external.insert(via_vector.globals[name]);
    }
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
        [
            ("Core.Classes.LawfulFunctors".to_owned(), "idf".to_owned()),
            ("Core.Logic.Transport".to_owned(), "cong".to_owned()),
        ]
        .into_iter()
        .collect(),
        "Vector must import exactly the two checked proof providers"
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

/// Promise class: durable invariant for the exact private proof obligation.
/// MEASURED: the real roots loader installs a transparent checked theorem whose
/// raw type is three binders followed by the exact `Equal (Vec a n)` endpoints,
/// with canonical `map`/`idf` identities and de Bruijn-bound arguments.
/// CLAIMED: the checked proof is about applying Vector's map to the public
/// identity function, not merely about an equality that happens to reduce.
/// THE GAP: this pins the stated raw theorem type, not an arbitrary equivalent
/// rewriting; the authored law has precisely this contract.
#[test]
fn vec_map_identity_raw_checked_proposition_is_not_reflexive_filler() {
    let (env, _) = load(VECTOR);
    let name = format!("{VECTOR}.vec_map_identity");
    let id = env.globals[&name];
    let Decl::Transparent { ty, .. } = env.env.lookup(id).expect("identity law must be loaded")
    else {
        panic!("{name} must be a checked transparent proof, not an assumption");
    };

    let global = |name: &str| Term::const_(env.globals[name], vec![]);
    let vec_at = |a: Term, n: Term| {
        Term::app(
            Term::app(
                Term::indformer(env.globals[&format!("{VECTOR}.Vec")], vec![]),
                a,
            ),
            n,
        )
    };
    let mapped = Term::app(
        Term::app(
            Term::app(
                Term::app(
                    Term::app(global(&format!("{VECTOR}.map")), Term::var(2)),
                    Term::var(2),
                ),
                Term::var(1),
            ),
            Term::app(global("Core.Classes.LawfulFunctors.idf"), Term::var(2)),
        ),
        Term::var(0),
    );
    let proposition = Term::app(
        Term::app(
            Term::app(global("Equal"), vec_at(Term::var(2), Term::var(1))),
            mapped,
        ),
        Term::var(0),
    );
    let expected = Term::pi(
        Term::ty(Level::Zero),
        Term::pi(
            Term::indformer(env.globals["Nat"], vec![]),
            Term::pi(vec_at(Term::var(1), Term::var(0)), proposition),
        ),
    );
    assert_eq!(
        ty, &expected,
        "the checked map identity law changed its exact raw proposition"
    );
}
