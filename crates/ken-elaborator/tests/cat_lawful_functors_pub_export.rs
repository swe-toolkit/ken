//! EC closure-provider publication acceptance controls.
//!
//! Promise class: durable invariants. `Core.Classes.LawfulFunctors` exposes
//! exactly the ten ordinary and attached definitions in EC's signature-closed
//! provider set, while retaining their existing identities and trust posture.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::{parser, Decl, ElabEnv, ElabError, ExportForm};
use ken_kernel::{Decl as KernelDecl, GlobalId, Term};

const LAWFUL_FUNCTORS: &str = "Core.Classes.LawfulFunctors";
const LAWFUL_FUNCTORS_KEN_MD: &str =
    include_str!("../../../catalog/packages/Core/Classes/LawfulFunctors.ken.md");

fn catalog_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn load_lawful_functors() -> ElabEnv {
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_module_from_roots(&[catalog_root()], LAWFUL_FUNCTORS)
        .expect("LawfulFunctors must elaborate through the real roots loader");
    env
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
    match surface {
        "Functor" | "Foldable" | "Monoid" => {
            env.class_env
                .class(surface)
                .unwrap_or_else(|| panic!("published class {surface} must be registered"))
                .projection
                .type_id
        }
        _ => {
            let qualified = format!("{LAWFUL_FUNCTORS}.{surface}");
            *env.globals
                .get(&qualified)
                .unwrap_or_else(|| panic!("provider identity must exist at {qualified}"))
        }
    }
}

fn assert_declaration_mentions(env: &ElabEnv, consumer: &str, provider: GlobalId) {
    let consumer = env.globals[consumer];
    let (ty, body) = match env.env.lookup(consumer) {
        Some(KernelDecl::Transparent { ty, body, .. }) => (ty, body),
        other => panic!("consumer witness must remain transparent, got {other:?}"),
    };
    assert!(
        term_mentions(ty, provider) || term_mentions(body, provider),
        "consumer witness must retain the selected LawfulFunctors provider identity"
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
    let alias = format!("ec_closure_export_direct_{index}");
    PublicationQuery {
        surface: surface.to_owned(),
        source: format!("import {LAWFUL_FUNCTORS} ({surface} as {alias})"),
        unpublished_names: BTreeSet::from([format!("{LAWFUL_FUNCTORS}.{surface}")]),
    }
}

fn module_publication_queries() -> ModulePublicationQueries {
    let extracted = ken_elaborator::literate::extract_ken_md(LAWFUL_FUNCTORS_KEN_MD)
        .expect("LawfulFunctors literate source must extract");
    let declarations =
        parser::parse_decls(&extracted.source).expect("LawfulFunctors extracted source must parse");
    let dependency_imports = declarations
        .iter()
        .filter(|declaration| matches!(declaration.unwrap_pub(), Decl::ImportDecl { .. }))
        .map(|declaration| {
            extracted.source[declaration.span().start..declaration.span().end].to_owned()
        })
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
                    .map(|binder| &extracted.source[binder.span.start..binder.span.end])
                    .collect::<Vec<_>>()
                    .join(" ");
                let arguments = params
                    .iter()
                    .flat_map(|binder| binder.names.iter())
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(" ");
                let theorem_and_separator =
                    &extracted.source[theorem.span().start..body.span().start];
                let separator = theorem_and_separator
                    .rfind('=')
                    .expect("attached proof signature must end at its body separator");
                let theorem = theorem_and_separator[..separator].trim_end();
                let index = attached.len();
                let probe = format!("ec_closure_export_probe_{index}");
                let alias = format!("ec_closure_export_subject_{index}");
                let binders = rename_identifier(&binders, subject, &alias);
                let theorem = rename_identifier(theorem, subject, &alias);
                let surface = format!("{subject}::{proof_name}");
                attached.push(PublicationQuery {
                    surface: surface.clone(),
                    source: format!(
                        "import {LAWFUL_FUNCTORS} ({subject} as {alias})\n\
                         theorem {probe} {binders} : {theorem} = {alias}::{proof_name} {arguments}"
                    ),
                    unpublished_names: BTreeSet::from([
                        format!("{LAWFUL_FUNCTORS}.{subject}"),
                        format!("{LAWFUL_FUNCTORS}.{surface}"),
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
    let mut env = load_lawful_functors();
    if !queries.dependency_imports.is_empty() {
        env.elaborate_file(&queries.dependency_imports)
            .expect("LawfulFunctors dependency imports must resolve for publication probes");
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
                let alias = format!("ec_closure_export_dependency_{query_index}_{index}");
                let renamed = rename_identifier(&declaration, dependency, &alias);
                if renamed != declaration {
                    dependency_imports.push(format!(
                        "import {LAWFUL_FUNCTORS} ({dependency} as {alias})"
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
    [
        "Foldable",
        "Functor",
        "Monoid",
        "comp",
        "fold_map_step",
        "idf",
        "list_map",
        "list_map::fusion",
        "list_map::id",
        "monoid_mempty",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}

/// MEASURED: every parsed publishable definition and attached proof is queried
/// through the real roots loader, and the successful queries equal a literal
/// authorized set. CLAIMED: LawfulFunctors exposes exactly EC's ten eligible
/// signature-closed provider surfaces. THE GAP: generated instance dictionaries
/// are not publishable declaration forms and are deliberately outside this
/// provider-surface inventory.
#[test]
fn lawful_functors_loader_visible_inventory_is_exact() {
    assert_eq!(
        published_module_surfaces(),
        authorized_surfaces(),
        "LawfulFunctors must expose exactly EC's ten eligible closure providers"
    );
}

/// MEASURED: one real selective consumer resolves all eight direct imports and
/// both attached proofs to their existing qualified provider GlobalIds; private
/// direct and attached siblings still reject as `UnboundName`; loading the
/// provider adds no trusted entry. CLAIMED: publication changes visibility only,
/// preserving provider identity, the private boundary, and zero trust delta.
/// THE GAP: existing package acceptance owns the definitions' computational and
/// law behavior; the exact-inventory test owns surface closure.
#[test]
fn lawful_functors_selective_consumer_retains_provider_identity_and_trust() {
    let mut baseline = ElabEnv::new().expect("base environment");
    for dependency in ["Data.Collections.Derived", "Core.Logic.Transport"] {
        baseline
            .elaborate_module_from_roots(&[catalog_root()], dependency)
            .unwrap_or_else(|error| panic!("{dependency} must roots-load: {error:?}"));
    }
    let before: BTreeSet<_> = baseline.env.trusted_base().into_iter().collect();
    baseline
        .elaborate_module_from_roots(&[catalog_root()], LAWFUL_FUNCTORS)
        .expect("LawfulFunctors must roots-load for the trust differential");
    let after: BTreeSet<_> = baseline.env.trusted_base().into_iter().collect();
    assert_eq!(
        before, after,
        "LawfulFunctors must add no trusted authority beyond its provider closure"
    );

    let mut env = load_lawful_functors();
    let providers = [
        ("Functor", "ec_closure_functor_identity"),
        ("Foldable", "ec_closure_foldable_identity"),
        ("Monoid", "ec_closure_monoid_identity"),
        ("comp", "ec_closure_comp"),
        ("idf", "ec_closure_idf"),
        ("list_map", "ec_closure_list_map"),
        ("list_map::id", "ec_closure_list_map_id"),
        ("list_map::fusion", "ec_closure_list_map_fusion"),
        ("fold_map_step", "ec_closure_fold_map_step"),
        ("monoid_mempty", "ec_closure_monoid_mempty"),
    ]
    .map(|(surface, consumer)| {
        let provider = provider_identity(&env, surface);
        assert!(
            env.env.transparent_body(provider).is_some(),
            "{surface} must retain its existing transparent provider declaration"
        );
        (provider, consumer)
    });

    env.elaborate_file(
        "import Core.Classes.LawfulFunctors \
           (Functor, Foldable, Monoid, comp, idf, list_map, fold_map_step, monoid_mempty)\n\
         fn ec_closure_functor_identity \
           (f : Type → Type) (dict : Functor f) : Functor f = dict\n\
         fn ec_closure_foldable_identity \
           (f : Type → Type) (dict : Foldable f) : Foldable f = dict\n\
         fn ec_closure_monoid_identity (a : Type) (dict : Monoid a) : Monoid a = dict\n\
         fn ec_closure_comp \
           (a : Type) (b : Type) (c : Type) (g : b → c) (h : a → b) (x : a) : c = \
           comp a b c g h x\n\
         fn ec_closure_idf (a : Type) (x : a) : a = idf a x\n\
         fn ec_closure_list_map \
           (a : Type) (b : Type) (g : a → b) (xs : List a) : List b = \
           list_map a b g xs\n\
         theorem ec_closure_list_map_id \
           (a : Type) (xs : List a) \
           : Equal (List a) (list_map a a (idf a) xs) xs = \
           (proof id for list_map) a xs\n\
         theorem ec_closure_list_map_fusion \
           (a : Type) (b : Type) (c : Type) (g : b → c) (h : a → b) (xs : List a) \
           : Equal \
               (List c) \
               (list_map a c (comp a b c g h) xs) \
               (list_map b c g (list_map a b h xs)) = \
           (proof fusion for list_map) a b c g h xs\n\
         fn ec_closure_fold_map_step \
           (a : Type) (m : Type) (dict : Monoid m) (g : a → m) (x : a) (acc : m) : m = \
           fold_map_step a m dict g x acc\n\
         fn ec_closure_monoid_mempty (m : Type) (dict : Monoid m) : m = \
           monoid_mempty m dict",
    )
    .expect("all ten eligible closure providers must resolve in one selective consumer");

    for (provider, consumer) in providers {
        assert_declaration_mentions(&env, consumer, provider);
    }

    for private in ["Semigroup", "bool_and", "option_map"] {
        match env.elaborate_file(&format!("import {LAWFUL_FUNCTORS} ({private})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{LAWFUL_FUNCTORS}.{private}"));
            }
            Err(other) => panic!("private import must fail as UnboundName: {other:?}"),
            Ok(_) => panic!("{LAWFUL_FUNCTORS}.{private} became selectively importable"),
        }
    }

    let private_proof = module_publication_queries()
        .attached
        .into_iter()
        .find(|query| query.surface == "option_map::id")
        .expect("option_map::id must remain in the source population");
    match env.elaborate_file(&private_proof.source) {
        Err(ElabError::UnboundName { name, .. }) => assert!(
            private_proof.unpublished_names.contains(&name),
            "private attached proof rejected at unrelated name {name}"
        ),
        Err(other) => panic!("private attached proof must fail as UnboundName: {other:?}"),
        Ok(_) => panic!("option_map::id became loader-visible"),
    }
}
