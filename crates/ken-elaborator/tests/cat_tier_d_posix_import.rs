//! Tier-D Filesystem.Path.Posix publication and strict-import controls.

#[path = "support/catalog_or.rs"]
mod catalog_or;
#[path = "support/catalog_publication.rs"]
mod catalog_publication;

use std::collections::{BTreeMap, BTreeSet};

use ken_elaborator::{parser, Decl, ElabEnv, ElabError, ImportKind};
use ken_kernel::{Decl as KernelDecl, GlobalId, Term};

const COMPARE: &str = "Core.Logic.Compare";
const LAWFUL: &str = "Core.Classes.LawfulClasses";
const DERIVED: &str = "Data.Collections.Derived";
const POSIX: &str = "Capability.Filesystem.Path.Posix";
const POSIX_SOURCE: &str =
    include_str!("../../../catalog/packages/Capability/Filesystem/Path/Posix.ken.md");

fn names(items: &[&str]) -> BTreeSet<String> {
    items.iter().map(|item| (*item).to_owned()).collect()
}

fn provider_modules(module: &str) -> &'static [&'static str] {
    match module {
        POSIX => &[COMPARE, LAWFUL, DERIVED],
        _ => &[],
    }
}

fn module_ids(env: &ElabEnv, module: &str) -> BTreeSet<GlobalId> {
    let prefix = format!("{module}.");
    env.globals
        .iter()
        .filter_map(|(name, id)| name.starts_with(&prefix).then_some(*id))
        .collect()
}

struct LoadedPosix {
    env: ElabEnv,
    compare: BTreeSet<GlobalId>,
    lawful: BTreeSet<GlobalId>,
    derived: BTreeSet<GlobalId>,
    posix: BTreeSet<GlobalId>,
}

fn load_posix() -> LoadedPosix {
    let root = catalog_or::catalog_root();
    let mut env = ElabEnv::new().expect("base environment");
    for provider in provider_modules(POSIX) {
        env.elaborate_module_from_roots(std::slice::from_ref(&root), provider)
            .unwrap_or_else(|error| panic!("Posix provider {provider} must load: {error:?}"));
    }
    let compare = module_ids(&env, COMPARE);
    let lawful = module_ids(&env, LAWFUL);
    let derived = module_ids(&env, DERIVED);
    let before_trust = env.env.trusted_base();
    let before_classes = env.class_env.class_entries().count();
    let before_instances = env.class_env.instances.len();
    let posix = env
        .elaborate_module_from_roots(std::slice::from_ref(&root), POSIX)
        .expect("Filesystem.Path.Posix must roots-load standalone")
        .into_iter()
        .collect();
    assert_eq!(
        env.env.trusted_base(),
        before_trust,
        "Posix publication and imports must add no trust"
    );
    assert_eq!(
        env.class_env.class_entries().count(),
        before_classes,
        "Posix must not mint a class"
    );
    assert_eq!(
        env.class_env.instances.len(),
        before_instances,
        "Posix must not mint an instance"
    );
    LoadedPosix {
        env,
        compare,
        lawful,
        derived,
        posix,
    }
}

fn selective_imports() -> BTreeMap<String, BTreeSet<String>> {
    let extracted = ken_elaborator::literate::extract_ken_md(POSIX_SOURCE)
        .expect("Posix literate source must extract");
    let declarations = parser::parse_decls(&extracted.source).expect("Posix source must parse");
    declarations
        .iter()
        .filter_map(|declaration| match declaration.unwrap_pub() {
            Decl::ImportDecl {
                module,
                kind: ImportKind::Selective(items),
                ..
            } => {
                assert!(
                    items.iter().all(|item| item.rename.is_none()),
                    "Posix imports must preserve provider spellings"
                );
                Some((
                    module.clone(),
                    items.iter().map(|item| item.name.clone()).collect(),
                ))
            }
            Decl::ImportDecl { module, .. } => {
                panic!("Posix import from {module} must be selective")
            }
            _ => None,
        })
        .collect()
}

fn referenced_globals(term: &Term, found: &mut BTreeSet<GlobalId>) {
    match term {
        Term::Const { id, .. } | Term::IndFormer { id, .. } | Term::Constructor { id, .. } => {
            found.insert(*id);
        }
        Term::Elim { fam, .. } => {
            found.insert(*fam);
        }
        _ => {}
    }
    for child in term.children() {
        referenced_globals(child, found);
    }
}

fn posix_references(loaded: &LoadedPosix) -> BTreeSet<GlobalId> {
    let mut referenced = BTreeSet::new();
    for id in &loaded.posix {
        if let Some(KernelDecl::Transparent { ty, body, .. }) = loaded.env.env.lookup(*id) {
            referenced_globals(ty, &mut referenced);
            referenced_globals(body, &mut referenced);
        }
    }
    referenced
}

fn provider_names(
    loaded: &LoadedPosix,
    provider: &str,
    provider_ids: &BTreeSet<GlobalId>,
    referenced: &BTreeSet<GlobalId>,
) -> BTreeSet<String> {
    let prefix = format!("{provider}.");
    loaded
        .env
        .globals
        .iter()
        .filter_map(|(name, id)| {
            (name.starts_with(&prefix) && provider_ids.contains(id) && referenced.contains(id))
                .then(|| {
                    name.strip_prefix(&prefix)
                        .expect("checked prefix")
                        .to_owned()
                })
        })
        .collect()
}

fn assert_private(surface: &str) {
    let mut loaded = load_posix();
    match loaded
        .env
        .elaborate_file(&format!("import {POSIX} ({surface})"))
    {
        Err(ElabError::UnboundName { name, .. }) => {
            assert_eq!(name, format!("{POSIX}.{surface}"));
        }
        other => panic!("{POSIX}.{surface} must stay private, got {other:?}"),
    }
}

fn assert_lawful_bool_and_eliminations_are_public() {
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_module_from_roots(&[catalog_or::catalog_root()], LAWFUL)
        .expect("LawfulClasses must roots-load");
    let left = env.globals[&format!("{LAWFUL}.bool_and::left")];
    let right = env.globals[&format!("{LAWFUL}.bool_and::right")];
    let before_trust = env.env.trusted_base();
    let before_classes = env.class_env.class_entries().count();
    let before_instances = env.class_env.instances.len();
    env.elaborate_file(
        "import Core.Classes.LawfulClasses (IsTrue as Truth, bool_and as conjunction)\n\
         theorem strict_bool_and_left (a : Bool) (b : Bool)\n\
           : Truth (conjunction a b) -> Truth a = conjunction::left a b\n\
         theorem strict_bool_and_right (a : Bool) (b : Bool)\n\
           : Truth (conjunction a b) -> Truth b = conjunction::right a b",
    )
    .expect("public bool_and elimination proofs must resolve through an imported subject");
    assert_eq!(env.globals[&format!("{LAWFUL}.bool_and::left")], left);
    assert_eq!(env.globals[&format!("{LAWFUL}.bool_and::right")], right);
    assert_eq!(env.env.trusted_base(), before_trust);
    assert_eq!(env.class_env.class_entries().count(), before_classes);
    assert_eq!(env.class_env.instances.len(), before_instances);
}

/// Promise class: normative compatibility vector.
///
/// MEASURED: the real roots loader queries every Posix declaration and
/// constructor, and the successful surface equals the Architect-curated
/// fifteen-name contract. One strict client imports the whole surface and uses
/// the transparent carrier and public operations without reminting an identity;
/// a second strict client resolves both public bool_and elimination proofs.
/// CLAIMED: both WP legs publish coherent, usable surfaces. THE GAP: declaration
/// queries do not establish dependency necessity or the private boundary, which
/// the sibling test and its population-side mutation campaign cover.
#[test]
fn filesystem_path_posix_loader_visible_inventory_is_exact_and_usable() {
    let expected = names(&[
        "MkPath",
        "Path",
        "path_is_absolute",
        "path_join",
        "path_normalize",
        "path_normalize_absolute_has_no_dotdot",
        "path_normalize_has_no_dot",
        "path_normalize_idempotent",
        "path_parent",
        "path_parse",
        "path_parse_render_parse",
        "path_parse_render_valid",
        "path_parse_valid",
        "path_render",
        "path_valid",
    ]);
    assert_eq!(
        catalog_publication::published_module_surfaces(POSIX_SOURCE, POSIX, "posix"),
        expected
    );

    let mut loaded = load_posix();
    let canonical = expected
        .iter()
        .map(|surface| {
            (
                surface.clone(),
                loaded.env.globals[&format!("{POSIX}.{surface}")],
            )
        })
        .collect::<BTreeMap<_, _>>();
    loaded
        .env
        .elaborate_file(&format!(
            "import {POSIX} ({})\n\
             const strict_posix_path : Path = MkPath False (Nil (List UInt8))\n\
             const strict_posix_absolute : Bool = path_is_absolute strict_posix_path\n\
             fn strict_posix_round_trip (raw : Bytes) : Bytes = path_render (path_parse raw)\n\
             fn strict_posix_valid (raw : Bytes) : Bool = path_valid (path_parse raw)",
            expected.iter().cloned().collect::<Vec<_>>().join(", ")
        ))
        .expect("the complete Posix surface must be usable by a strict client");
    for (surface, before) in canonical {
        assert_eq!(
            loaded.env.globals[&format!("{POSIX}.{surface}")],
            before,
            "selective import must not remint {POSIX}.{surface}"
        );
    }
    assert_lawful_bool_and_eliminations_are_public();
}

/// Promise class: durable invariant.
///
/// MEASURED: Posix roots-loads after exactly three providers with zero
/// trust/class/instance growth; its parsed imports equal the D0 selective
/// closure; its checked terms intersect each provider at the exact canonical
/// identities below; and representative interior names reject at their fully
/// qualified import names. CLAIMED: the three-provider boundary is exact,
/// visibility-only, and excludes the three loader-forced non-contract names.
/// THE GAP: DecEq is consumed during instance resolution and pattern constructors
/// disappear during lowering, so per-import necessity is established by the
/// population-side removal campaign rather than checked-term occurrence alone.
#[test]
fn filesystem_path_posix_imports_are_exact_canonical_and_internals_private() {
    assert_eq!(
        selective_imports(),
        BTreeMap::from([
            (COMPARE.to_owned(), names(&["list_eq"])),
            (
                LAWFUL.to_owned(),
                names(&["DecEq", "bool_and", "uint8_deceq_eq"]),
            ),
            (DERIVED.to_owned(), names(&["list_append"])),
        ])
    );

    let loaded = load_posix();
    let referenced = posix_references(&loaded);
    assert_eq!(
        provider_names(&loaded, COMPARE, &loaded.compare, &referenced),
        names(&["list_eq"])
    );
    assert_eq!(
        provider_names(&loaded, LAWFUL, &loaded.lawful, &referenced),
        names(&[
            "bool_and",
            "bool_and::intro",
            "bool_and::left",
            "bool_and::right",
            "uint8_deceq_eq",
        ])
    );
    assert_eq!(
        provider_names(&loaded, DERIVED, &loaded.derived, &referenced),
        names(&[
            "list_append",
            "list_append::assoc",
            "list_append::right_unit",
        ])
    );

    for surface in [
        "path_split_render_segments",
        "path_dot_segment",
        "path_segment_eq",
        "path_finish_segment",
        "path_list_tail",
    ] {
        assert_private(surface);
    }
}
