//! Tier-D Process.Environment publication and strict-import controls.

#[path = "support/catalog_or.rs"]
mod catalog_or;
#[path = "support/catalog_publication.rs"]
mod catalog_publication;

use std::collections::BTreeSet;

use ken_elaborator::{Decl as SurfaceDecl, ElabEnv, ElabError, ImportKind, literate, parser};
use ken_kernel::{Decl, GlobalId, Term};

const ENVIRONMENT: &str = "Capability.Process.Environment";
const ENVIRONMENT_SOURCE: &str =
    include_str!("../../../catalog/packages/Capability/Process/Environment.ken.md");

fn names(items: &[&str]) -> BTreeSet<String> {
    items.iter().map(|item| (*item).to_owned()).collect()
}

fn load_environment() -> (ElabEnv, BTreeSet<GlobalId>, BTreeSet<GlobalId>) {
    let mut env = ElabEnv::new().expect("base environment");
    let mut base_ids = env.globals.values().copied().collect::<BTreeSet<_>>();
    base_ids.extend(env.env.declarations().iter().map(Decl::id));
    let before_trust = env.env.trusted_base();
    let before_classes = env.class_env.class_entries().count();
    let before_instances = env.class_env.instances.len();
    let mut owned = env
        .elaborate_module_from_roots(&[catalog_or::catalog_root()], ENVIRONMENT)
        .expect("Process.Environment must roots-load over only the prelude")
        .into_iter()
        .collect::<BTreeSet<_>>();
    owned.extend(
        env.globals
            .iter()
            .filter(|(name, _)| name.starts_with("Capability.Process.Environment."))
            .map(|(_, identity)| *identity),
    );
    assert_eq!(
        env.env.trusted_base(),
        before_trust,
        "Process.Environment publication must add no trust"
    );
    assert_eq!(
        env.class_env.class_entries().count(),
        before_classes,
        "Process.Environment must not mint a class"
    );
    assert_eq!(
        env.class_env.instances.len(),
        before_instances,
        "Process.Environment must not mint an instance"
    );
    (env, owned, base_ids)
}

fn collect_term_globals(term: &Term, out: &mut BTreeSet<GlobalId>) {
    match term {
        Term::Const { id, .. } | Term::IndFormer { id, .. } | Term::Constructor { id, .. } => {
            out.insert(*id);
        }
        Term::Elim { fam, .. } => {
            out.insert(*fam);
        }
        _ => {}
    }
    for child in term.children() {
        collect_term_globals(child, out);
    }
}

fn collect_decl_globals(declaration: &Decl, out: &mut BTreeSet<GlobalId>) {
    match declaration {
        Decl::Transparent { ty, body, .. } => {
            collect_term_globals(ty, out);
            collect_term_globals(body, out);
        }
        Decl::Opaque { ty, .. } | Decl::Primitive { ty, .. } => {
            collect_term_globals(ty, out);
        }
        Decl::Inductive(inductive) => {
            for term in &inductive.params {
                collect_term_globals(term, out);
            }
            for term in &inductive.indices {
                collect_term_globals(term, out);
            }
            collect_term_globals(&inductive.former_type, out);
            for constructor in &inductive.constructors {
                for term in &constructor.args {
                    collect_term_globals(term, out);
                }
                for term in &constructor.target_indices {
                    collect_term_globals(term, out);
                }
                collect_term_globals(&constructor.type_, out);
            }
        }
    }
}

/// Promise class: normative compatibility vector.
///
/// MEASURED: the real parser finds no import declaration. CLAIMED: the D0
/// import ledger is exactly empty. THE GAP: an undeclared dependency would not
/// appear in syntax, so checked identity closure is measured separately.
#[test]
fn process_environment_import_ledger_is_empty() {
    let extracted =
        literate::extract_ken_md(ENVIRONMENT_SOURCE).expect("Process.Environment extraction");
    let declarations =
        parser::parse_decls(&extracted.source).expect("Process.Environment source must parse");
    let imports = declarations
        .iter()
        .filter_map(|declaration| match declaration.unwrap_pub() {
            SurfaceDecl::ImportDecl {
                module,
                kind: ImportKind::Selective(items),
                ..
            } => Some((module.clone(), items.len())),
            SurfaceDecl::ImportDecl { .. } => {
                panic!("Process.Environment dependency imports must be selective")
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    assert!(imports.is_empty(), "the D0 import ledger is empty");
}

/// Promise class: durable invariant.
///
/// MEASURED: checked terms retain no identity outside the module and prelude.
/// CLAIMED: Process.Environment has no undeclared provider edge. THE GAP: an
/// unused import would not reach checked terms, so parsed imports are measured
/// separately.
#[test]
fn process_environment_checked_provider_ledger_is_empty() {
    let (env, owned, base_ids) = load_environment();
    let mut resolved = BTreeSet::new();
    for identity in &owned {
        if let Some(declaration) = env.env.lookup(*identity) {
            collect_decl_globals(declaration, &mut resolved);
        }
    }
    let external = resolved
        .difference(&owned)
        .copied()
        .collect::<BTreeSet<_>>()
        .difference(&base_ids)
        .copied()
        .collect::<BTreeSet<_>>();
    assert!(
        external.is_empty(),
        "Process.Environment checked terms must depend only on the prelude"
    );
}

/// Promise class: normative compatibility vector.
///
/// MEASURED: loader publication queries return exactly the one D0-consumed
/// projection, and a real external selective import preserves its canonical
/// GlobalId. CLAIMED: Decoder can consume the provider without ambient
/// resolution. THE GAP: the exact inventory query covers every sibling whose
/// accidental publication the single positive client would not observe.
#[test]
fn process_environment_loader_visible_surface_is_exact_and_usable() {
    let expected = names(&["process_environment"]);
    assert_eq!(
        catalog_publication::published_module_surfaces(
            ENVIRONMENT_SOURCE,
            ENVIRONMENT,
            "process_environment",
        ),
        expected
    );
    let (mut env, _, _) = load_environment();
    let canonical = env.globals["Capability.Process.Environment.process_environment"];
    env.elaborate_file(
        "import Capability.Process.Environment \
           (process_environment as cat_tier_d_process_environment_projection) \
         fn cat_tier_d_process_environment_client (input : ProcessInput) \
           : List (Prod Bytes Bytes) = cat_tier_d_process_environment_projection input",
    )
    .expect("the Decoder-shaped projection must import from its real provider");
    let client = env.globals["cat_tier_d_process_environment_client"];
    let declaration = env.env.lookup(client).expect("checked external client");
    let mut referenced = BTreeSet::new();
    collect_decl_globals(declaration, &mut referenced);
    assert!(
        referenced.contains(&canonical),
        "the checked client must retain the provider's canonical identity"
    );
}

/// Promise class: durable invariant.
///
/// MEASURED: the unconsumed replacement rejects through an external import;
/// the exact publication enumerator separately queries the attached proof while
/// roots loading preserves trust, class, and instance populations. CLAIMED:
/// publication changes only the one consumer-required visibility edge. THE GAP:
/// byte preservation of the transparent bodies is a one-shot object audit.
#[test]
fn process_environment_publication_is_visibility_only() {
    let (mut env, _, _) = load_environment();
    match env.elaborate_file(&format!(
        "import {ENVIRONMENT} (replace_process_environment)"
    )) {
        Err(ElabError::UnboundName { name, .. }) => {
            assert_eq!(name, format!("{ENVIRONMENT}.replace_process_environment"));
        }
        other => panic!("replacement helper must stay private, got {other:?}"),
    }
}
