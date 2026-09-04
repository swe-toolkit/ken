//! CAT-MIGRATE-TIER-C-DATA-VALUE BytesKeys closeout controls.
//!
//! Promise class: durable invariants. BytesKeys remains a consumer-only
//! compatibility package whose only direct provider surface is LawfulClasses'
//! canonical `DecEq` class. It owns and publishes no definitions. The broader
//! no-shadowing registry control remains
//! `primitive_class_owner_instances_are_canonical_across_isolated_consumers`
//! in `cat_bool_pub_export`.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::{parser, Decl, ElabEnv, ElabError, ExportForm, ImportKind};
use ken_kernel::GlobalId;

const BYTES_KEYS: &str = "Data.Binary.BytesKeys";
const LAWFUL: &str = "Core.Classes.LawfulClasses";
const BYTES_KEYS_KEN_MD: &str =
    include_str!("../../../catalog/packages/Data/Binary/BytesKeys.ken.md");

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

#[derive(Debug)]
struct PackageShape {
    providers: BTreeSet<(String, String)>,
    exports: BTreeSet<String>,
    non_import_declarations: usize,
}

fn package_shape() -> PackageShape {
    let extracted = ken_elaborator::literate::extract_ken_md(BYTES_KEYS_KEN_MD)
        .expect("BytesKeys literate source must extract");
    let declarations =
        parser::parse_decls(&extracted.source).expect("BytesKeys extracted source must parse");
    let mut providers = BTreeSet::new();
    let mut exports = BTreeSet::new();
    let mut non_import_declarations = 0;

    for declaration in &declarations {
        match declaration.unwrap_pub() {
            Decl::ImportDecl { module, kind, .. } => match kind {
                ImportKind::Selective(items) => {
                    providers.extend(items.iter().map(|item| (module.clone(), item.name.clone())));
                }
                ImportKind::Qualified | ImportKind::Aliased(_) => {
                    providers.insert((module.clone(), "*".to_owned()));
                }
            },
            Decl::ExportDecl { form, .. } => {
                non_import_declarations += 1;
                let items = match form {
                    ExportForm::Facade { items, .. } | ExportForm::InScope { items } => items,
                };
                exports.extend(
                    items
                        .iter()
                        .map(|item| item.rename.clone().unwrap_or_else(|| item.name.clone())),
                );
            }
            _ => non_import_declarations += 1,
        }
    }

    PackageShape {
        providers,
        exports,
        non_import_declarations,
    }
}

fn instance_id(env: &ElabEnv, class: &str, head: &str) -> GlobalId {
    env.class_env
        .instance_search(class, head)
        .unwrap_or_else(|| panic!("{class} {head} must be registered"))
}

fn instance_package(env: &ElabEnv, id: GlobalId) -> &str {
    env.class_env
        .instances
        .values()
        .find(|instance| instance.instance_id == id)
        .unwrap_or_else(|| panic!("instance {id:?} must have registry metadata"))
        .defining_package
        .as_str()
}

/// MEASURED: isolated roots loading returns no entry-unit identity, creates no
/// qualified BytesKeys global, and executes the complete checked-fence set.
/// CLAIMED: BytesKeys is standalone and owns no definitions or dictionaries.
/// THE GAP: none; loader results are the entry unit's complete definition set,
/// while dependency units are tracked independently.
#[test]
fn bytes_keys_owned_inventory_is_empty_and_standalone() {
    let (mut env, owned) = load(BYTES_KEYS);
    assert!(
        owned.is_empty(),
        "BytesKeys must remain consumer-only, but it owns {owned:?}"
    );
    let prefix = format!("{BYTES_KEYS}.");
    assert!(
        env.globals.keys().all(|name| !name.starts_with(&prefix)),
        "BytesKeys must not own a qualified definition identity"
    );
    assert!(
        env.class_env
            .instances
            .values()
            .all(|instance| instance.defining_package != BYTES_KEYS),
        "BytesKeys must not own a class dictionary"
    );
    env.execute_loaded_entry_checked_fences(BYTES_KEYS)
        .expect("BytesKeys Definition and every checked fence must elaborate");
}

/// MEASURED: the parsed semantic import boundary selects exactly one provider
/// identity, LC's `DecEq`; loading through BytesKeys yields the same class and
/// two relevant dictionary IDs as loading LC directly. Import aliases are
/// deliberately ignored because they cannot change provider identity. CLAIMED:
/// BytesKeys's exact direct provider set is `{LawfulClasses.DecEq}`, with the
/// canonical UInt8 and Bytes dictionaries. THE GAP: none; eager dependency
/// loading cannot reveal a selective item set, so the parsed import population
/// is joined to the loader's identity verdict.
#[test]
fn bytes_keys_direct_lawfulclasses_provider_is_exact() {
    let (via_bytes_keys, _) = load(BYTES_KEYS);
    let (direct_lawful, _) = load(LAWFUL);
    let via_class = via_bytes_keys
        .class_env
        .class("DecEq")
        .expect("BytesKeys must register LC's DecEq class")
        .projection
        .type_id;
    let direct_class = direct_lawful
        .class_env
        .class("DecEq")
        .expect("LawfulClasses must register DecEq")
        .projection
        .type_id;
    assert_eq!(
        via_class, direct_class,
        "BytesKeys must retain LawfulClasses' canonical DecEq identity"
    );

    for head in ["UInt8", "Bytes"] {
        let via = instance_id(&via_bytes_keys, "DecEq", head);
        let direct = instance_id(&direct_lawful, "DecEq", head);
        assert_eq!(
            via, direct,
            "BytesKeys must register LC's canonical DecEq {head} dictionary"
        );
        assert_eq!(instance_package(&via_bytes_keys, via), LAWFUL);
    }

    let shape = package_shape();
    assert_eq!(
        shape.providers,
        BTreeSet::from([(LAWFUL.to_owned(), "DecEq".to_owned())]),
        "BytesKeys must select exactly LawfulClasses.DecEq"
    );
    assert_eq!(
        shape.non_import_declarations, 0,
        "BytesKeys must remain a pure selective-import compatibility package"
    );
}

/// MEASURED: every semantic re-export item is queried through the already
/// loaded BytesKeys module, and the successful set is empty. Relevant imported
/// provider spellings plus an independent would-be local name each reject as
/// exact qualified `UnboundName`. CLAIMED: BytesKeys publishes nothing and does
/// not facade-export its provider. THE GAP: local publications are subsumed by
/// the stronger empty-owned-inventory control; this test exhausts re-exports.
#[test]
fn bytes_keys_loader_visible_inventory_is_empty() {
    let shape = package_shape();
    let (mut env, _) = load(BYTES_KEYS);
    let published = shape
        .exports
        .iter()
        .filter(|surface| {
            let alias = surface.replace("::", "_");
            let source = format!("import {BYTES_KEYS} ({surface} as closeout_{alias})");
            match env.elaborate_file(&source) {
                Ok(_) => true,
                Err(error) => {
                    panic!("declared BytesKeys export {surface} was not loader-visible: {error:?}")
                }
            }
        })
        .cloned()
        .collect::<BTreeSet<_>>();
    assert_eq!(
        published,
        BTreeSet::new(),
        "BytesKeys loader-visible inventory must stay empty"
    );

    for name in [
        "DecEq",
        "uint8_deceq_eq",
        "bytes_deceq_eq",
        "bytes_keys_local_name",
    ] {
        let source = format!("import {BYTES_KEYS} ({name} as closeout_{name})");
        match env.elaborate_file(&source) {
            Err(ElabError::UnboundName { name: rejected, .. }) => {
                assert_eq!(rejected, format!("{BYTES_KEYS}.{name}"));
            }
            Err(other) => {
                panic!("BytesKeys import of {name} failed for the wrong reason: {other:?}")
            }
            Ok(_) => panic!("BytesKeys unexpectedly published {name}"),
        }
    }
}
