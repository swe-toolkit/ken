//! EffectfulClasses standalone-import acceptance controls.
//!
//! Promise class: durable invariants. The real catalog root resolves EC through
//! its exact semantic provider closure. Imported provider identities remain the
//! canonical declarations, EC-local dictionaries remain locally synthesized,
//! imports do not become EC exports, and module resolution erases to the same
//! kernel declarations as the equivalent flat environment.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use ken_elaborator::{parser, Decl, ElabEnv, ElabError, ImportKind};
use ken_kernel::{Decl as KernelDecl, GlobalId, Term};

const EFFECTFUL_CLASSES: &str = "Core.Classes.EffectfulClasses";
const LAWFUL_FUNCTORS: &str = "Core.Classes.LawfulFunctors";
const DERIVED: &str = "Data.Collections.Derived";
const TRANSPORT: &str = "Core.Logic.Transport";
const EFFECTFUL_CLASSES_KEN_MD: &str =
    include_str!("../../../catalog/packages/Core/Classes/EffectfulClasses.ken.md");

fn catalog_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn extracted_source() -> String {
    ken_elaborator::literate::extract_ken_md(EFFECTFUL_CLASSES_KEN_MD)
        .expect("EffectfulClasses literate source must extract")
        .source
}

fn parsed_import_inventory() -> BTreeMap<String, BTreeSet<String>> {
    parser::parse_decls(&extracted_source())
        .expect("EffectfulClasses extracted source must parse")
        .into_iter()
        .filter_map(|declaration| match declaration.unwrap_pub() {
            Decl::ImportDecl {
                module,
                kind: ImportKind::Selective(items),
                ..
            } => Some((
                module.clone(),
                items
                    .iter()
                    .map(|item| {
                        assert_eq!(
                            item.rename, None,
                            "EC's provider closure needs no renamed import"
                        );
                        item.name.clone()
                    })
                    .collect(),
            )),
            Decl::ImportDecl { module, .. } => {
                panic!("EC dependency {module} must remain selective")
            }
            _ => None,
        })
        .collect()
}

fn expected_import_inventory() -> BTreeMap<String, BTreeSet<String>> {
    BTreeMap::from([
        (
            LAWFUL_FUNCTORS.to_owned(),
            BTreeSet::from([
                "Foldable".to_owned(),
                "Foldable_instance_List".to_owned(),
                "Foldable_instance_Option".to_owned(),
                "Functor".to_owned(),
                "Functor_instance_List".to_owned(),
                "Functor_instance_Option".to_owned(),
                "comp".to_owned(),
                "idf".to_owned(),
                "list_map".to_owned(),
            ]),
        ),
        (
            TRANSPORT.to_owned(),
            BTreeSet::from(["cong".to_owned(), "sym".to_owned(), "trans".to_owned()]),
        ),
        (
            DERIVED.to_owned(),
            BTreeSet::from(["concat_map".to_owned(), "list_append".to_owned()]),
        ),
    ])
}

fn load_effectful_classes() -> (ElabEnv, BTreeSet<GlobalId>) {
    let mut env = ElabEnv::new().expect("base environment");
    let ids = env
        .elaborate_module_from_roots(&[catalog_root()], EFFECTFUL_CLASSES)
        .expect("EffectfulClasses must elaborate through the real roots loader")
        .into_iter()
        .collect();
    (env, ids)
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

fn transparent_mentions(env: &ElabEnv, consumer: GlobalId, provider: GlobalId) -> bool {
    match env.env.lookup(consumer) {
        Some(KernelDecl::Transparent { ty, body, .. }) => {
            term_mentions(ty, provider) || term_mentions(body, provider)
        }
        other => panic!("consumer must be transparent, got {other:?}"),
    }
}

fn global(env: &ElabEnv, name: &str) -> GlobalId {
    *env.globals
        .get(name)
        .unwrap_or_else(|| panic!("missing global `{name}`"))
}

fn local_global(env: &ElabEnv, name: &str) -> GlobalId {
    global(env, &format!("{EFFECTFUL_CLASSES}.{name}"))
}

fn class_id(env: &ElabEnv, name: &str) -> GlobalId {
    env.class_env
        .class(name)
        .unwrap_or_else(|| panic!("missing class `{name}`"))
        .projection
        .type_id
}

fn instance_id(env: &ElabEnv, class: &str, head: &str) -> GlobalId {
    env.class_env
        .instances
        .get(&(class.to_owned(), head.to_owned()))
        .unwrap_or_else(|| panic!("missing instance `{class}/{head}`"))
        .instance_id
}

/// MEASURED: a fresh production roots-loader environment reaches an EC-owned
/// class declaration without preloading any dependency or installing flat
/// aliases. CLAIMED: EC is standalone-loadable from its own declared edges.
/// THE GAP: provider-by-provider closure and trust are asserted separately.
#[test]
fn effectful_classes_roots_loads_standalone() {
    let (env, ec_ids) = load_effectful_classes();
    assert!(
        ec_ids.contains(&class_id(&env, "Applicative")),
        "standalone loading must reach EC's own Applicative declaration"
    );
}

/// MEASURED: the parser reports an exact three-module selective-import
/// inventory; the production roots loader resolves EC; and one real EC
/// declaration mentions each ordinary, class, generated-dictionary, and
/// attached-proof provider by exact `GlobalId`. CLAIMED: EC declares its whole
/// direct provider closure rather than borrowing ambient bindings. THE GAP:
/// legacy resolution can still resolve some loaded class names ambiently, so
/// the parsed import relation is the closure assertion while the term witnesses
/// establish that every listed provider is a real consumer dependency.
#[test]
fn effectful_classes_import_closure_is_exact_and_identity_preserving() {
    assert_eq!(
        parsed_import_inventory(),
        expected_import_inventory(),
        "EC must declare exactly its measured selective provider closure"
    );

    let (env, ec_ids) = load_effectful_classes();
    let functor = class_id(&env, "Functor");
    let foldable = class_id(&env, "Foldable");
    let provider_instances = [
        ("Functor", "List", "Functor_instance_List"),
        ("Functor", "Option", "Functor_instance_Option"),
        ("Foldable", "List", "Foldable_instance_List"),
        ("Foldable", "Option", "Foldable_instance_Option"),
    ];
    for (class, head, alias) in provider_instances {
        let provider = instance_id(&env, class, head);
        assert_eq!(
            global(&env, alias),
            provider,
            "the imported dictionary alias must retain LF's canonical identity"
        );
        assert!(
            !ec_ids.contains(&provider),
            "EC must not own imported dictionary `{alias}`"
        );
    }

    let witnesses = [
        (class_id(&env, "Applicative"), functor, "Functor"),
        (class_id(&env, "Traversable"), foldable, "Foldable"),
        (
            instance_id(&env, "Applicative", "List"),
            instance_id(&env, "Functor", "List"),
            "Functor_instance_List",
        ),
        (
            instance_id(&env, "Applicative", "Option"),
            instance_id(&env, "Functor", "Option"),
            "Functor_instance_Option",
        ),
        (
            instance_id(&env, "Traversable", "List"),
            instance_id(&env, "Foldable", "List"),
            "Foldable_instance_List",
        ),
        (
            instance_id(&env, "Traversable", "Option"),
            instance_id(&env, "Foldable", "Option"),
            "Foldable_instance_Option",
        ),
        (
            local_global(&env, "identity_map::fusion"),
            global(&env, &format!("{LAWFUL_FUNCTORS}.comp")),
            "comp",
        ),
        (
            local_global(&env, "list_ap_id"),
            global(&env, &format!("{LAWFUL_FUNCTORS}.idf")),
            "idf",
        ),
        (
            local_global(&env, "list_ap"),
            global(&env, &format!("{LAWFUL_FUNCTORS}.list_map")),
            "list_map",
        ),
        (
            local_global(&env, "list_ap_id"),
            global(&env, &format!("{LAWFUL_FUNCTORS}.list_map::id")),
            "list_map::id",
        ),
        (
            local_global(&env, "pf_probe"),
            global(&env, &format!("{LAWFUL_FUNCTORS}.list_map::fusion")),
            "list_map::fusion",
        ),
        (
            local_global(&env, "list_bind"),
            global(&env, &format!("{DERIVED}.concat_map")),
            "concat_map",
        ),
        (
            local_global(&env, "concat_map_append_distrib"),
            global(&env, &format!("{DERIVED}.list_append")),
            "list_append",
        ),
        (
            local_global(&env, "concat_map_append_distrib"),
            global(&env, &format!("{DERIVED}.list_append::assoc")),
            "list_append::assoc",
        ),
        (
            local_global(&env, "list_bind_lid"),
            global(&env, &format!("{DERIVED}.list_append::right_unit")),
            "list_append::right_unit",
        ),
        (
            local_global(&env, "list_bind_rid"),
            global(&env, &format!("{TRANSPORT}.cong")),
            "cong",
        ),
        (
            local_global(&env, "list_map_coh"),
            global(&env, &format!("{TRANSPORT}.sym")),
            "sym",
        ),
        (
            local_global(&env, "list_ap_id"),
            global(&env, &format!("{TRANSPORT}.trans")),
            "trans",
        ),
    ];
    for (consumer, provider, surface) in witnesses {
        assert!(
            transparent_mentions(&env, consumer, provider),
            "EC must retain a real exact-identity consumer of `{surface}`"
        );
    }
}

/// MEASURED: the root loader reports exactly eight EC-owned instance
/// dictionaries, including qualified `Identity` heads, and none appears in the
/// import inventory. CLAIMED: generated dictionaries split cleanly between LF
/// providers and EC-local synthesis. THE GAP: this does not re-prove instance
/// laws; the existing DS-7/DS-8 suites own those behaviors.
#[test]
fn effectful_classes_local_dictionary_inventory_is_exact() {
    let (env, ec_ids) = load_effectful_classes();
    let actual = env
        .class_env
        .instances
        .iter()
        .filter_map(|((class, head), instance)| {
            ec_ids
                .contains(&instance.instance_id)
                .then_some((class.clone(), head.clone()))
        })
        .collect::<BTreeSet<_>>();
    let expected = BTreeSet::from([
        ("Applicative".to_owned(), "List".to_owned()),
        ("Applicative".to_owned(), "Option".to_owned()),
        (
            "Applicative".to_owned(),
            format!("{EFFECTFUL_CLASSES}.Identity"),
        ),
        (
            "Functor".to_owned(),
            format!("{EFFECTFUL_CLASSES}.Identity"),
        ),
        ("Monad".to_owned(), "List".to_owned()),
        ("Monad".to_owned(), "Option".to_owned()),
        ("Traversable".to_owned(), "List".to_owned()),
        ("Traversable".to_owned(), "Option".to_owned()),
    ]);
    assert_eq!(
        actual, expected,
        "EC's locally synthesized instances changed"
    );

    let imported = parsed_import_inventory()
        .values()
        .flatten()
        .cloned()
        .collect::<BTreeSet<_>>();
    for local in [
        "Applicative_instance_Identity",
        "Applicative_instance_List",
        "Applicative_instance_Option",
        "Functor_instance_Identity",
        "Monad_instance_List",
        "Monad_instance_Option",
        "Traversable_instance_List",
        "Traversable_instance_Option",
    ] {
        assert!(
            !imported.contains(local),
            "EC-local dictionary `{local}` must not be imported"
        );
    }
}

/// MEASURED: after loading the provider closure in the same production order,
/// flat elaboration of EC and roots-loader elaboration emit pairwise-equal
/// kernel declarations; EC adds no trusted entry. CLAIMED: the import delta is
/// surface-only and preserves computation, proofs, and trust. THE GAP: equality
/// is scoped to the checked EC declaration trees; checked example/reject fences
/// remain owned by the existing package suites.
#[test]
fn effectful_classes_imports_erase_to_identical_kernel_trees_and_zero_trust() {
    let (root_env, root_ids) = load_effectful_classes();

    let mut flat_env = ElabEnv::new().expect("flat baseline environment");
    for dependency in [LAWFUL_FUNCTORS, TRANSPORT, DERIVED] {
        flat_env
            .elaborate_module_from_roots(&[catalog_root()], dependency)
            .unwrap_or_else(|error| panic!("dependency {dependency} must roots-load: {error:?}"));
    }
    let before = flat_env.env.trusted_base();
    let flat_ids = flat_env
        .elaborate_file(&extracted_source())
        .expect("EC must elaborate in the equivalent flat provider environment");
    let after = flat_env.env.trusted_base();
    assert_eq!(before, after, "EC must add zero trusted authority");
    assert_eq!(
        root_ids.len(),
        flat_ids.len(),
        "module elaboration must preserve EC's declaration population"
    );
    for (root_id, flat_id) in root_ids.iter().zip(flat_ids) {
        assert_eq!(
            root_env.env.lookup(*root_id),
            flat_env.env.lookup(flat_id),
            "module imports must erase before the pairwise kernel declaration"
        );
    }
}

/// MEASURED: selecting every EC dependency name from EC itself rejects at the
/// EC-qualified surface. CLAIMED: imports are body scope, never a facade
/// re-export. THE GAP: EC's own publication policy is independently private;
/// this test covers only accidental dependency republishing.
#[test]
fn effectful_classes_does_not_reexport_imported_dependencies() {
    let (mut env, _) = load_effectful_classes();
    for surface in expected_import_inventory().values().flatten() {
        let source = format!("import {EFFECTFUL_CLASSES} ({surface})");
        match env.elaborate_file(&source) {
            Err(ElabError::UnboundName { name, .. }) => assert_eq!(
                name,
                format!("{EFFECTFUL_CLASSES}.{surface}"),
                "dependency rejection must be attributed to EC's interface"
            ),
            Err(other) => panic!("dependency import failed at an unrelated error: {other:?}"),
            Ok(_) => panic!("EC accidentally re-exported dependency `{surface}`"),
        }
    }
}
