//! EffectfulClasses Applicative-provider publication controls.
//!
//! Promise class: durable invariants. The real roots loader publishes exactly
//! the four Validation-facing EC providers, preserves their canonical identities
//! and the existing class/instance owner set, retains a private sibling, and
//! adds no trusted authority.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use ken_elaborator::{parser, Decl, ElabEnv, ElabError, ExportForm};
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

fn provider_identity(env: &ElabEnv, surface: &str) -> GlobalId {
    if surface == "Applicative" {
        env.class_env
            .class(surface)
            .expect("Applicative must remain a registered class")
            .projection
            .type_id
    } else {
        env.globals[&format!("{EFFECTFUL_CLASSES}.{surface}")]
    }
}

fn assert_transparent_mentions(env: &ElabEnv, consumer: &str, provider: GlobalId) {
    let consumer = env.globals[consumer];
    let (ty, body) = match env.env.lookup(consumer) {
        Some(KernelDecl::Transparent { ty, body, .. }) => (ty, body),
        other => panic!("consumer must remain transparent, got {other:?}"),
    };
    assert!(
        term_mentions(ty, provider) || term_mentions(body, provider),
        "consumer must retain the selected EC provider identity"
    );
}

struct PublicationQuery {
    surface: String,
    source: String,
    unpublished_names: BTreeSet<String>,
}

struct ModulePublicationQueries {
    dependency_imports: String,
    direct: Vec<PublicationQuery>,
    attached: Vec<PublicationQuery>,
}

fn rename_identifier(source: &str, from: &str, to: &str) -> String {
    let mut renamed = String::with_capacity(source.len());
    let mut token = String::new();
    let flush = |token: &mut String, renamed: &mut String| {
        if token == from {
            renamed.push_str(to);
        } else {
            renamed.push_str(token);
        }
        token.clear();
    };
    for character in source.chars() {
        if character.is_alphanumeric() || character == '_' {
            token.push(character);
        } else {
            flush(&mut token, &mut renamed);
            renamed.push(character);
        }
    }
    flush(&mut token, &mut renamed);
    renamed
}

fn direct_publication_query(surface: &str, index: usize) -> PublicationQuery {
    let alias = format!("cat_ec_publication_direct_{index}");
    PublicationQuery {
        surface: surface.to_owned(),
        source: format!("import {EFFECTFUL_CLASSES} ({surface} as {alias})"),
        unpublished_names: BTreeSet::from([format!("{EFFECTFUL_CLASSES}.{surface}")]),
    }
}

fn module_publication_queries() -> ModulePublicationQueries {
    let source = extracted_source();
    let declarations =
        parser::parse_decls(&source).expect("EffectfulClasses extracted source must parse");
    let dependency_imports = declarations
        .iter()
        .filter(|declaration| matches!(declaration.unwrap_pub(), Decl::ImportDecl { .. }))
        .map(|declaration| source[declaration.span().start..declaration.span().end].to_owned())
        .collect::<Vec<_>>()
        .join("\n");
    let mut direct = Vec::new();
    let mut attached = Vec::new();

    for declaration in &declarations {
        let declaration = declaration.unwrap_pub();
        match declaration {
            Decl::ViewDecl { .. }
            | Decl::LetDecl { .. }
            | Decl::PropDecl { .. }
            | Decl::TheoremDecl { .. }
            | Decl::AxiomDecl { .. }
            | Decl::DataDecl { .. }
            | Decl::ExplicitDataDecl { .. }
            | Decl::TypeAlias { .. }
            | Decl::ClassDecl { .. } => {
                direct.push(direct_publication_query(declaration.name(), direct.len()));
            }
            Decl::AttachedProofDecl {
                proof_name,
                subject,
                params,
                theorem,
                body,
                ..
            } => {
                let binders = params
                    .iter()
                    .map(|binder| &source[binder.span.start..binder.span.end])
                    .collect::<Vec<_>>()
                    .join(" ");
                let arguments = params
                    .iter()
                    .flat_map(|binder| binder.names.iter())
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(" ");
                let theorem_and_separator = &source[theorem.span().start..body.span().start];
                let separator = theorem_and_separator
                    .rfind('=')
                    .expect("attached proof signature must end at its body separator");
                let theorem = theorem_and_separator[..separator].trim_end();
                let index = attached.len();
                let probe = format!("cat_ec_publication_probe_{index}");
                let alias = format!("cat_ec_publication_subject_{index}");
                let binders = rename_identifier(&binders, subject, &alias);
                let theorem = rename_identifier(theorem, subject, &alias);
                let surface = format!("{subject}::{proof_name}");
                attached.push(PublicationQuery {
                    surface: surface.clone(),
                    source: format!(
                        "import {EFFECTFUL_CLASSES} ({subject} as {alias})\n\
                         theorem {probe} {binders} : {theorem} = \
                         {alias}::{proof_name} {arguments}"
                    ),
                    unpublished_names: BTreeSet::from([
                        format!("{EFFECTFUL_CLASSES}.{subject}"),
                        format!("{EFFECTFUL_CLASSES}.{surface}"),
                    ]),
                });
            }
            Decl::ExportDecl { form, .. } => {
                let items = match form {
                    ExportForm::Facade { items, .. } | ExportForm::InScope { items } => items,
                };
                for item in items {
                    let surface = item.rename.as_deref().unwrap_or(&item.name);
                    direct.push(direct_publication_query(surface, direct.len()));
                }
            }
            Decl::BoundaryDecl { .. }
            | Decl::SpaceDecl { .. }
            | Decl::ProveDecl { .. }
            | Decl::LawDecl { .. }
            | Decl::ForeignDecl { .. }
            | Decl::TemporalDecl { .. }
            | Decl::RecordDecl { .. }
            | Decl::InstanceDecl { .. }
            | Decl::DeriveDecl { .. }
            | Decl::ModuleDecl { .. }
            | Decl::ImportDecl { .. } => {}
            Decl::Pub(_) => panic!("unwrap_pub must remove the visibility wrapper"),
        }
    }

    ModulePublicationQueries {
        dependency_imports,
        direct,
        attached,
    }
}

fn published_module_surfaces() -> BTreeSet<String> {
    let queries = module_publication_queries();
    let (mut env, _) = load_effectful_classes();
    if !queries.dependency_imports.is_empty() {
        env.elaborate_file(&queries.dependency_imports)
            .expect("EC dependency imports must resolve for publication probes");
    }
    let mut probe = |query: &PublicationQuery| match env.elaborate_file(&query.source) {
        Ok(_) => true,
        Err(ElabError::UnboundName { name: rejected, .. }) => {
            assert!(
                query.unpublished_names.contains(&rejected),
                "publication query for {} failed at unrelated name {rejected}",
                query.surface
            );
            false
        }
        Err(other) => panic!(
            "loader publication query for {} failed: {other:?}\n{}",
            query.surface, query.source
        ),
    };

    let direct = queries
        .direct
        .into_iter()
        .filter(|query| probe(query))
        .map(|query| query.surface)
        .collect::<BTreeSet<_>>();
    let direct_imports = direct.iter().cloned().collect::<Vec<_>>();
    let attached = queries
        .attached
        .into_iter()
        .enumerate()
        .filter_map(|(query_index, mut query)| {
            let subject = query
                .surface
                .split_once("::")
                .expect("attached surface has a subject")
                .0;
            let (subject_import, declaration) = query
                .source
                .split_once('\n')
                .expect("attached query has a subject import and theorem");
            let mut declaration = declaration.to_owned();
            let mut dependency_imports = Vec::new();
            for (index, dependency) in direct_imports
                .iter()
                .filter(|surface| surface.as_str() != subject)
                .enumerate()
            {
                let alias = format!("cat_ec_publication_dependency_{query_index}_{index}");
                let renamed = rename_identifier(&declaration, dependency, &alias);
                if renamed != declaration {
                    dependency_imports.push(format!(
                        "import {EFFECTFUL_CLASSES} ({dependency} as {alias})"
                    ));
                    declaration = renamed;
                }
            }
            dependency_imports.push(subject_import.to_owned());
            dependency_imports.push(declaration);
            query.source = dependency_imports.join("\n");
            probe(&query).then_some(query.surface)
        })
        .collect::<BTreeSet<_>>();

    direct.union(&attached).cloned().collect()
}

fn authorized_surfaces() -> BTreeSet<String> {
    ["Applicative", "apply_to", "compose", "functor_map_of"]
        .into_iter()
        .map(str::to_owned)
        .collect()
}

/// MEASURED: the real roots loader is queried for every parsed publishable EC
/// declaration and attached proof, and the successful set is compared with an
/// independent literal contract set. CLAIMED: EC exposes exactly the four
/// Validation-facing providers. THE GAP: generated instance dictionaries are
/// not publishable declaration forms and are guarded by the owner inventory.
#[test]
fn effectful_classes_loader_visible_inventory_is_exact() {
    assert_eq!(
        published_module_surfaces(),
        authorized_surfaces(),
        "EC must expose exactly its authorized Applicative-provider inventory"
    );
}

/// MEASURED: one real selective consumer binds all four public names to their
/// existing EC GlobalIds, while a neighboring helper still rejects at EC's
/// qualified interface. CLAIMED: publication changes visibility only and keeps
/// the private boundary. THE GAP: exact inventory and owner preservation are
/// asserted independently below.
#[test]
fn effectful_classes_selective_consumer_retains_provider_identities() {
    let (mut env, _) = load_effectful_classes();
    let providers = [
        ("apply_to", "cat_ec_apply_to_consumer"),
        ("compose", "cat_ec_compose_consumer"),
        ("functor_map_of", "cat_ec_functor_map_of_consumer"),
        ("Applicative", "cat_ec_applicative_consumer"),
    ]
    .map(|(surface, consumer)| (provider_identity(&env, surface), consumer));

    env.elaborate_file(
        "import Core.Classes.LawfulFunctors (Functor)\n\
         import Core.Classes.EffectfulClasses \
           (apply_to as selected_apply_to, \
            compose as selected_compose, \
            functor_map_of as selected_functor_map_of, \
            Applicative as SelectedApplicative)\n\
         fn cat_ec_apply_to_consumer \
           (a : Type) (b : Type) (y : a) (g : a → b) : b = \
           selected_apply_to a b y g\n\
         fn cat_ec_compose_consumer \
           (a : Type) (b : Type) (c : Type) \
           (g : b → c) (h : a → b) (x : a) : c = \
           selected_compose a b c g h x\n\
         fn cat_ec_functor_map_of_consumer \
           (f : Type → Type) (d : Functor f) \
           (a : Type) (b : Type) (g : a → b) (x : f a) : f b = \
           selected_functor_map_of f d a b g x\n\
         fn cat_ec_applicative_consumer \
           (f : Type → Type) (d : SelectedApplicative f) : SelectedApplicative f = d",
    )
    .expect("all four EC Applicative providers must selectively import together");

    for (provider, consumer) in providers {
        assert_transparent_mentions(&env, consumer, provider);
    }

    match env.elaborate_file(&format!("import {EFFECTFUL_CLASSES} (applicative_pure_of)")) {
        Err(ElabError::UnboundName { name, .. }) => {
            assert_eq!(name, format!("{EFFECTFUL_CLASSES}.applicative_pure_of"));
        }
        Err(other) => panic!("private sibling failed at an unrelated error: {other:?}"),
        Ok(_) => panic!("EC's private `applicative_pure_of` became loader-visible"),
    }
}

/// MEASURED: roots loading owns exactly the existing three EC class identities
/// and eight EC instance heads; the published Applicative identity is the same
/// class-registry entry mentioned by its three instance dictionaries. CLAIMED:
/// publication mints no second class or instance and retains one canonical EC
/// owner. THE GAP: source-byte preservation is a one-shot differential because
/// a live test must not pin repository text.
#[test]
fn effectful_classes_publication_preserves_class_and_instance_owners() {
    let (env, ec_ids) = load_effectful_classes();
    let owned_classes = env
        .class_env
        .class_entries()
        .filter(|class| ec_ids.contains(&class.projection.type_id))
        .map(|class| {
            (
                class.projection.owner_name.to_owned(),
                class.projection.type_id,
            )
        })
        .collect::<BTreeMap<_, _>>();
    assert_eq!(
        owned_classes
            .keys()
            .map(String::as_str)
            .collect::<BTreeSet<_>>(),
        BTreeSet::from(["Applicative", "Monad", "Traversable"]),
        "EC's owned class population changed"
    );

    let applicative = provider_identity(&env, "Applicative");
    assert_eq!(
        applicative, owned_classes["Applicative"],
        "the public name and class registry must retain one Applicative identity"
    );
    let owned_instances = env
        .class_env
        .instances
        .iter()
        .filter_map(|((class, head), instance)| {
            ec_ids.contains(&instance.instance_id).then_some((
                class.clone(),
                head.clone(),
                instance.instance_id,
            ))
        })
        .collect::<BTreeSet<_>>();
    assert_eq!(
        owned_instances
            .iter()
            .map(|(class, head, _)| (class.as_str(), head.as_str()))
            .collect::<BTreeSet<_>>(),
        BTreeSet::from([
            ("Applicative", "Core.Classes.EffectfulClasses.Identity"),
            ("Applicative", "List"),
            ("Applicative", "Option"),
            ("Functor", "Core.Classes.EffectfulClasses.Identity"),
            ("Monad", "List"),
            ("Monad", "Option"),
            ("Traversable", "List"),
            ("Traversable", "Option"),
        ]),
        "EC's owned instance population changed"
    );
    let applicative_instances = owned_instances
        .iter()
        .filter(|(class, _, _)| class == "Applicative")
        .collect::<Vec<_>>();
    assert_eq!(applicative_instances.len(), 3);
    for (_, _, instance) in applicative_instances {
        let ty = match env.env.lookup(*instance) {
            Some(KernelDecl::Transparent { ty, .. }) => ty,
            other => panic!("Applicative instance must be transparent, got {other:?}"),
        };
        assert!(
            term_mentions(ty, applicative),
            "each Applicative dictionary must retain the one EC class identity"
        );
    }
}

/// MEASURED: after loading EC's exact dependency closure, loading EC adds no
/// trusted-base entry. CLAIMED: the visibility-only publication changes no
/// proof authority. THE GAP: the existing EC import-closure target separately
/// compares all 339 emitted kernel declarations with the flat baseline.
#[test]
fn effectful_classes_publication_adds_zero_trust() {
    let mut env = ElabEnv::new().expect("base environment");
    for dependency in [LAWFUL_FUNCTORS, TRANSPORT, DERIVED] {
        env.elaborate_module_from_roots(&[catalog_root()], dependency)
            .unwrap_or_else(|error| panic!("{dependency} must roots-load: {error:?}"));
    }
    let before = env.env.trusted_base();
    env.elaborate_module_from_roots(&[catalog_root()], EFFECTFUL_CLASSES)
        .expect("EffectfulClasses must roots-load after its provider closure");
    assert_eq!(
        env.env.trusted_base(),
        before,
        "publishing EC providers must add no trusted authority"
    );
}
