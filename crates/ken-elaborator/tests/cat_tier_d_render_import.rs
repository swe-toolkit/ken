//! Tier-D Diagnostics.Render publication and strict-import controls.

#[path = "support/catalog_or.rs"]
mod catalog_or;
#[path = "support/catalog_publication.rs"]
mod catalog_publication;

use std::collections::{BTreeMap, BTreeSet};

use ken_elaborator::{parser, Decl, ElabEnv, ElabError, ImportKind};
use ken_kernel::{Decl as KernelDecl, GlobalId, Term};

const DIAGNOSTICS_CORE: &str = "Capability.Diagnostics.Core";
const FORMATTING_DOC: &str = "Capability.Formatting.Doc";
const DIAGNOSTICS_RENDER: &str = "Capability.Diagnostics.Render";
const DIAGNOSTICS_RENDER_SOURCE: &str =
    include_str!("../../../catalog/packages/Capability/Diagnostics/Render.ken.md");

fn names(items: &[&str]) -> BTreeSet<String> {
    items.iter().map(|item| (*item).to_owned()).collect()
}

struct LoadedRender {
    env: ElabEnv,
    core_owned: BTreeSet<GlobalId>,
    doc_owned: BTreeSet<GlobalId>,
    render_owned: BTreeSet<GlobalId>,
}

fn module_ids(env: &ElabEnv, module: &str) -> BTreeSet<GlobalId> {
    let prefix = format!("{module}.");
    env.globals
        .iter()
        .filter_map(|(name, id)| name.starts_with(&prefix).then_some(*id))
        .collect()
}

fn load_render() -> LoadedRender {
    let root = catalog_or::catalog_root();
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_module_from_roots(std::slice::from_ref(&root), DIAGNOSTICS_CORE)
        .expect("Diagnostics.Core provider must roots-load");
    let core_owned = module_ids(&env, DIAGNOSTICS_CORE);
    env.elaborate_module_from_roots(std::slice::from_ref(&root), FORMATTING_DOC)
        .expect("Formatting.Doc provider must roots-load");
    let doc_owned = module_ids(&env, FORMATTING_DOC);
    let before_trust = env.env.trusted_base();
    let before_classes = env.class_env.class_entries().count();
    let before_instances = env.class_env.instances.len();
    let render_owned: BTreeSet<GlobalId> = env
        .elaborate_module_from_roots(std::slice::from_ref(&root), DIAGNOSTICS_RENDER)
        .expect("Diagnostics.Render must roots-load standalone")
        .into_iter()
        .collect();
    assert_eq!(
        env.env.trusted_base(),
        before_trust,
        "Render publication and imports must add no trust"
    );
    assert_eq!(
        env.class_env.class_entries().count(),
        before_classes,
        "Render must not mint a class"
    );
    assert_eq!(
        env.class_env.instances.len(),
        before_instances,
        "Render must not mint an instance"
    );
    assert_eq!(
        render_owned.len(),
        3,
        "Render must retain exactly its three transparent functions"
    );
    LoadedRender {
        env,
        core_owned,
        doc_owned,
        render_owned,
    }
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

fn render_references(loaded: &LoadedRender) -> BTreeSet<GlobalId> {
    let mut referenced = BTreeSet::new();
    for id in &loaded.render_owned {
        let Some(KernelDecl::Transparent { ty, body, .. }) = loaded.env.env.lookup(*id) else {
            panic!("every Render-owned declaration must stay transparent");
        };
        referenced_globals(ty, &mut referenced);
        referenced_globals(body, &mut referenced);
    }
    referenced
}

fn selective_imports() -> BTreeMap<String, BTreeSet<String>> {
    let extracted = ken_elaborator::literate::extract_ken_md(DIAGNOSTICS_RENDER_SOURCE)
        .expect("Render literate source must extract");
    let declarations = parser::parse_decls(&extracted.source).expect("Render source must parse");
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
                    "Render imports must preserve provider spellings"
                );
                Some((
                    module.clone(),
                    items.iter().map(|item| item.name.clone()).collect(),
                ))
            }
            Decl::ImportDecl { module, .. } => {
                panic!("Render import from {module} must be selective")
            }
            _ => None,
        })
        .collect()
}

fn assert_private(surface: &str) {
    let mut loaded = load_render();
    match loaded
        .env
        .elaborate_file(&format!("import {DIAGNOSTICS_RENDER} ({surface})"))
    {
        Err(ElabError::UnboundName { name, .. }) => {
            assert_eq!(name, format!("{DIAGNOSTICS_RENDER}.{surface}"));
        }
        other => panic!("{DIAGNOSTICS_RENDER}.{surface} must stay private, got {other:?}"),
    }
}

/// Promise class: normative compatibility vector.
///
/// MEASURED: the real roots loader queries all three Render declarations and a
/// selective client resolves exactly `diagnostic_to_doc` to the canonical
/// Render identity, then constructs its provider-owned input and result types.
/// CLAIMED: Render publishes its sole client-consumed operation as a coherent
/// strict surface. THE GAP: generated dictionaries are outside the declaration
/// query, but `load_render` independently pins zero class and instance growth.
#[test]
fn diagnostics_render_loader_visible_inventory_is_exact_and_usable() {
    let expected = names(&["diagnostic_to_doc"]);
    assert_eq!(
        catalog_publication::published_module_surfaces(
            DIAGNOSTICS_RENDER_SOURCE,
            DIAGNOSTICS_RENDER,
            "diagnostics_render",
        ),
        expected
    );

    let mut loaded = load_render();
    let canonical = loaded.env.globals[&format!("{DIAGNOSTICS_RENDER}.diagnostic_to_doc")];
    loaded
        .env
        .elaborate_file(&format!(
            "import {DIAGNOSTICS_CORE} (\
               ArgumentOrigin, Diagnostic, MkByteRange, MkDiagnostic, MkDiagnosticCode)\n\
             import {FORMATTING_DOC} (Doc)\n\
             import {DIAGNOSTICS_RENDER} (diagnostic_to_doc as render_diagnostic)\n\
             const cat_tier_d_render_client : Doc =\n\
               render_diagnostic\n\
                 (MkDiagnostic\n\
                   (ArgumentOrigin Zero (MkByteRange Zero Zero))\n\
                   (MkDiagnosticCode \"render\"))"
        ))
        .expect("Render public operation must be usable by a strict client");
    assert_eq!(
        loaded.env.globals[&format!("{DIAGNOSTICS_RENDER}.diagnostic_to_doc")],
        canonical,
        "selective import must not remint Render's public operation"
    );
}

/// Promise class: durable invariant.
///
/// MEASURED: Render roots-loads after its two published providers with zero
/// trust/class/instance growth; its parsed import declarations equal the exact
/// loader-derived D0 closure; its checked terms reference only canonical Core
/// and Doc identities; and both helper imports reject at their exact names.
/// CLAIMED: the two-provider dependency boundary is exact and the migration is
/// visibility-only. THE GAP: pattern constructors are resolved while lowering
/// and therefore do not remain as term nodes; exact necessity is established
/// by the per-item population-side removal campaign.
#[test]
fn diagnostics_render_imports_are_exact_canonical_and_helpers_private() {
    let expected_imports = BTreeMap::from([
        (
            DIAGNOSTICS_CORE.to_owned(),
            names(&[
                "ArgumentOrigin",
                "ConfigKeyOrigin",
                "Diagnostic",
                "DiagnosticCode",
                "EnvironmentOrigin",
                "MkDiagnosticCode",
                "Origin",
                "SourceOrigin",
                "diagnostic_code",
                "diagnostic_origin",
            ]),
        ),
        (
            FORMATTING_DOC.to_owned(),
            names(&["Concat", "Doc", "Group", "Line", "text_string"]),
        ),
    ]);
    assert_eq!(selective_imports(), expected_imports);

    let loaded = load_render();
    let referenced = render_references(&loaded);
    let actual_core = referenced
        .intersection(&loaded.core_owned)
        .copied()
        .collect::<BTreeSet<_>>();
    let expected_core = [
        "ByteRange",
        "Diagnostic",
        "DiagnosticCode",
        "Origin",
        "SourceId",
        "diagnostic_code",
        "diagnostic_origin",
    ]
    .into_iter()
    .map(|surface| loaded.env.globals[&format!("{DIAGNOSTICS_CORE}.{surface}")])
    .collect::<BTreeSet<_>>();
    assert_eq!(actual_core, expected_core);

    let actual_doc = referenced
        .intersection(&loaded.doc_owned)
        .copied()
        .collect::<BTreeSet<_>>();
    let expected_doc = ["Concat", "Doc", "Group", "Line", "text_string"]
        .into_iter()
        .map(|surface| loaded.env.globals[&format!("{FORMATTING_DOC}.{surface}")])
        .collect::<BTreeSet<_>>();
    assert_eq!(actual_doc, expected_doc);

    assert_private("diagnostic_code_string");
    assert_private("diagnostic_origin_label");
}
