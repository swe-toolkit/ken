//! CAT-MIGRATE-TIER-C-DATA-VALUE Deque closeout controls.
//!
//! Promise class: durable invariants. Deque owns its exact checked carrier,
//! operation, and law family; publishes no catalog surface; consumes
//! `Data.Collections.Derived`'s canonical `list_append` and `reverse` plus
//! `Core.Logic.Transport.sym`, adding no trust beyond those provider closures.
//! The existing
//! `transparent_deque_bodies_have_exact_derived_head_occurrence_populations`
//! and concrete sequence tests in `cat_deque_acceptance` retain the per-body
//! provider-use and behavioral obligations.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::{parser, Decl as SurfaceDecl, ElabEnv, ElabError, ExportForm, ImportKind};
use ken_kernel::{Decl, GlobalId, Term};

const DEQUE: &str = "Data.Collections.Deque";
const DERIVED: &str = "Data.Collections.Derived";
const TRANSPORT: &str = "Core.Logic.Transport";
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
        "MkPopFrontNone",
        "MkPopFrontSome",
        "MkPopPreserves",
        "PopFrontListView",
        "PopPreserves",
        "deque_append_snoc_assoc",
        "deque_cong",
        "deque_pop_front_nil_view",
        "deque_pop_front_reversed",
        "empty",
        "popBack",
        "popBack_pushBack",
        "popFront",
        "popFront_list_view",
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

/// MEASURED: ordinary isolated roots loading installs exactly the checked
/// identities in Deque's expected owned inventory, returns only identities
/// from that population, and executes every checked fence. The resulting
/// trusted base equals a fresh load of the Derived provider closure. CLAIMED: Deque is standalone, owns exactly its
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

/// Promise class: durable invariant.
///
/// MEASURED: ordinary isolated-roots loading checks the actual private Deque
/// law, and a second compilation of the extracted package plus two concrete
/// in-module applications checks the law over inhabited direct and rebalance
/// deques. CLAIMED: the indexed law is checked and applies inside its private
/// owner rather than merely naming a generic result family. THE GAP: this
/// instantiates two configurations; the arbitrary-`q` kernel-checked proof
/// in the loaded package supplies the universal quantifier.
#[test]
fn pop_front_list_view_checks_in_loaded_package_and_on_inhabited_deques() {
    let (loaded, _) = load(DEQUE);
    let law_id = loaded.globals[&format!("{DEQUE}.popFront_list_view")];
    assert!(
        matches!(loaded.env.lookup(law_id), Some(Decl::Transparent { .. })),
        "the arbitrary-deque law must be a checked transparent proof"
    );

    let extracted = ken_elaborator::literate::extract_ken_md(DEQUE_KEN_MD)
        .expect("Deque literate source must extract");
    let in_module = format!(
        "{}\n\
         const deque_direct : Deque Bool = \
           MkDeque Bool (Cons Bool True (Nil Bool)) (Cons Bool False (Nil Bool))\n\
         const deque_direct_view : \
           PopFrontListView Bool deque_direct (popFront Bool deque_direct) = \
           popFront_list_view Bool deque_direct\n\
         const deque_rebalanced : Deque Bool = \
           MkDeque Bool (Nil Bool) (Cons Bool True (Cons Bool False (Nil Bool)))\n\
         const deque_rebalanced_view : \
           PopFrontListView Bool deque_rebalanced (popFront Bool deque_rebalanced) = \
           popFront_list_view Bool deque_rebalanced",
        extracted.source
    );
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_module_from_roots(&[catalog_root()], DERIVED)
        .expect("Derived provider closure must be available");
    env.elaborate_file(&in_module)
        .expect("actual Deque package plus both concrete in-module law applications must check");
    for name in ["deque_direct_view", "deque_rebalanced_view"] {
        let id = env.globals[name];
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{name} must be a checked proof, not an assumption"
        );
    }
}

/// MEASURED: every checked Deque type and body refers to the exact nine-name
/// compiler floor plus three canonical Derived identities and Transport.sym.
/// Deque and Derived roots loads agree on their shared closure; in a separate
/// Transport-first environment, Deque's proof helper refers to that same
/// environment's canonical sym identity. CLAIMED: Deque's direct dependency is
/// `{Derived.list_append, Derived.list_append::right_unit, Derived.reverse,
/// Transport.sym}`. THE GAP: attached proof selection adds right-unit without
/// widening the Derived import; Transport.sym is explicitly imported. Source
/// forms `J` and `Refl` elaborate into kernel terms without provider globals.
#[test]
fn deque_direct_provider_inventory_is_exact_and_canonical() {
    let base = ElabEnv::new().expect("base environment");
    let (via_deque, _) = load(DEQUE);
    let (via_derived, _) = load(DERIVED);
    let (mut via_transport, _) = load(TRANSPORT);
    let append_name = format!("{DERIVED}.list_append");
    let reverse_name = format!("{DERIVED}.reverse");
    let right_unit_name = format!("{DERIVED}.list_append::right_unit");
    let sym_name = format!("{TRANSPORT}.sym");
    let append = via_deque.globals[&append_name];
    let reverse = via_deque.globals[&reverse_name];
    let right_unit = via_deque.globals[&right_unit_name];
    let sym = via_deque.globals[&sym_name];
    assert_eq!(append, via_derived.globals[&append_name]);
    assert_eq!(reverse, via_derived.globals[&reverse_name]);
    assert_eq!(right_unit, via_derived.globals[&right_unit_name]);
    assert_eq!(sym, via_derived.globals[&sym_name]);

    // GlobalIds are allocated per roots-load order. Preload the independent
    // Transport root into one environment, then load Deque in that same
    // environment and verify the law references exactly its canonical sym.
    let canonical_sym = via_transport.globals[&sym_name];
    via_transport
        .elaborate_module_from_roots(&[catalog_root()], DEQUE)
        .expect("Deque must reuse the independently loaded Transport provider");
    assert_eq!(via_transport.globals[&sym_name], canonical_sym);
    let helper = via_transport.globals[&format!("{DEQUE}.deque_pop_front_nil_view")];
    let helper_references = declaration_references(
        via_transport
            .env
            .lookup(helper)
            .expect("the private reverse-result proof helper must be checked"),
    );
    assert!(
        helper_references.contains(&canonical_sym),
        "Deque's checked reverse-result helper must use the independent Transport.sym identity"
    );

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
        .chain([append, reverse, right_unit, sym])
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
            (TRANSPORT.to_owned(), "sym".to_owned()),
        ]),
        "Deque must select exactly Derived.list_append/reverse and Transport.sym"
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

/// MEASURED: independently withholding each of the two Derived selections
/// and the explicit Transport.sym selection from the extracted module reaches
/// the exact missing name's `UnresolvedCon` boundary, while ordinary roots
/// loading above succeeds. CLAIMED: all three selections are individually
/// necessary, not ambient through a transitive provider. THE GAP: none; each
/// negative changes only one selective import, with the complete unchanged
/// provider closure loaded as a positive availability control.
#[test]
fn all_deque_provider_imports_are_individually_load_bearing() {
    let extracted = ken_elaborator::literate::extract_ken_md(DEQUE_KEN_MD)
        .expect("Deque literate source must extract");
    let declarations =
        parser::parse_decls(&extracted.source).expect("Deque extracted source must parse");
    let imports = declarations
        .iter()
        .filter_map(|declaration| match declaration.unwrap_pub() {
            SurfaceDecl::ImportDecl {
                module,
                kind: ImportKind::Selective(items),
                span,
                ..
            } => Some((module.as_str(), span.start..span.end, items)),
            SurfaceDecl::ImportDecl { .. } => {
                panic!("both Deque imports must remain selective")
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(
        imports.len(),
        2,
        "Deque must retain exactly two selective imports"
    );
    let derived_import = imports
        .iter()
        .find(|import| import.0 == DERIVED)
        .expect("Deque must explicitly import Derived");
    let transport_import = imports
        .iter()
        .find(|import| import.0 == TRANSPORT)
        .expect("Deque must explicitly import Transport");
    assert_eq!(derived_import.2.len(), 2, "Deque selects two Derived names");
    assert_eq!(
        transport_import.2.len(),
        1,
        "Deque selects only Transport.sym"
    );

    let derived_binding = |provider_name: &str| {
        derived_import
            .2
            .iter()
            .find(|item| item.name == provider_name)
            .map(|item| item.rename.clone().unwrap_or_else(|| item.name.clone()))
            .unwrap_or_else(|| panic!("Deque must import Derived.{provider_name}"))
    };
    for (retained, missing) in [("reverse", "list_append"), ("list_append", "reverse")] {
        let retained_local = derived_binding(retained);
        let missing_local = derived_binding(missing);
        let retained_item = if retained_local == retained {
            retained.to_owned()
        } else {
            format!("{retained} as {retained_local}")
        };
        let mut source = extracted.source.clone();
        source.replace_range(
            derived_import.1.clone(),
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

    let sym_binding = transport_import.2[0]
        .rename
        .clone()
        .unwrap_or_else(|| transport_import.2[0].name.clone());
    assert_eq!(transport_import.2[0].name, "sym");
    let mut source = extracted.source.clone();
    source.replace_range(
        transport_import.1.clone(),
        &format!("import {TRANSPORT} (cong)"),
    );
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_module_from_roots(&[catalog_root()], DERIVED)
        .expect("Derived's complete provider closure, including Transport.sym, must load");
    assert!(
        env.globals.contains_key(&format!("{TRANSPORT}.sym")),
        "Transport.sym must be available, but unimported in the negative"
    );
    match env.elaborate_file(&source) {
        Err(ElabError::UnresolvedCon { name, .. }) => assert_eq!(name, sym_binding),
        Err(other) => panic!("withholding Transport.sym failed for the wrong reason: {other:?}"),
        Ok(_) => panic!("Deque unexpectedly used Transport.sym without its import"),
    }
}
