//! CC1 (`Data.Collections.NonEmpty` + `Data.Sums.Validation`) acceptance —
//! `docs/program/wp/cc1-nonempty-validation.md`.
//!
//! These packages depend on the existing catalog rather than the bare
//! prelude.  Match the catalog's established DS-7/DS-8 validation model:
//! elaborate the dependency closure in order into one shared `ElabEnv`, then
//! elaborate both CC1 entries (including every checked literate fence).

#[path = "support/catalog_or.rs"]
mod catalog_or;

use std::collections::BTreeSet;

use ken_elaborator::{parser, Decl, ElabEnv, ElabError, ExportForm};
use ken_kernel::{GlobalId, Term};
const NONEMPTY_KEN_MD: &str =
    include_str!("../../../catalog/packages/Data/Collections/NonEmpty.ken.md");
const VALIDATION_KEN_MD: &str =
    include_str!("../../../catalog/packages/Data/Sums/Validation.ken.md");
const NONEMPTY_MODULE: &str = "Data.Collections.NonEmpty";
const VALIDATION_MODULE: &str = "Data.Sums.Validation";

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

fn direct_publication_query(
    module: &str,
    label: &str,
    surface: &str,
    index: usize,
) -> PublicationQuery {
    let alias = format!("cc1_{label}_export_{index}");
    PublicationQuery {
        surface: surface.to_owned(),
        source: format!("import {module} ({surface} as {alias})"),
        unpublished_names: BTreeSet::from([format!("{module}.{surface}")]),
    }
}

fn module_publication_queries(source: &str, module: &str, label: &str) -> ModulePublicationQueries {
    let extracted = ken_elaborator::literate::extract_ken_md(source)
        .unwrap_or_else(|error| panic!("{module} literate source must extract: {error:?}"));
    let declarations = parser::parse_decls(&extracted.source)
        .unwrap_or_else(|error| panic!("{module} extracted source must parse: {error:?}"));
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
            | Decl::TypeAlias { .. }
            | Decl::ClassDecl { .. } => {
                direct.push(direct_publication_query(
                    module,
                    label,
                    declaration.name(),
                    direct.len(),
                ));
            }
            Decl::DataDecl { name, ctors, .. } => {
                direct.push(direct_publication_query(module, label, name, direct.len()));
                for constructor in ctors {
                    direct.push(direct_publication_query(
                        module,
                        label,
                        &constructor.name,
                        direct.len(),
                    ));
                }
            }
            Decl::ExplicitDataDecl { name, ctors, .. } => {
                direct.push(direct_publication_query(module, label, name, direct.len()));
                for constructor in ctors {
                    direct.push(direct_publication_query(
                        module,
                        label,
                        constructor.name(),
                        direct.len(),
                    ));
                }
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
                let probe = format!("cc1_{label}_proof_probe_{index}");
                let alias = format!("cc1_{label}_subject_{index}");
                let binders = rename_identifier(&binders, subject, &alias);
                let theorem = rename_identifier(theorem, subject, &alias);
                let surface = format!("{subject}::{proof_name}");
                attached.push(PublicationQuery {
                    surface: surface.clone(),
                    source: format!(
                        "import {module} ({subject} as {alias})\n\
                         theorem {probe} {binders} : {theorem} = {alias}::{proof_name} {arguments}"
                    ),
                    unpublished_names: BTreeSet::from([
                        format!("{module}.{subject}"),
                        format!("{module}.{surface}"),
                    ]),
                });
            }
            Decl::ExportDecl { form, .. } => {
                let items = match form {
                    ExportForm::Facade { items, .. } | ExportForm::InScope { items } => items,
                };
                for item in items {
                    let surface = item.rename.as_deref().unwrap_or(&item.name);
                    direct.push(direct_publication_query(
                        module,
                        label,
                        surface,
                        direct.len(),
                    ));
                }
            }
            Decl::BoundaryDecl { .. }
            | Decl::FixityDecl { .. }
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

fn published_module_surfaces(source: &str, module: &str, label: &str) -> BTreeSet<String> {
    let queries = module_publication_queries(source, module, label);
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_module_from_roots(&[catalog_or::catalog_root()], module)
        .unwrap_or_else(|error| {
            panic!("{module} must roots-load for publication probes: {error:?}")
        });
    env.elaborate_file(&queries.dependency_imports)
        .unwrap_or_else(|error| {
            panic!("{module} dependency imports must resolve for publication probes: {error:?}")
        });

    let mut probe = |query: &PublicationQuery| match env.elaborate_file(&query.source) {
        Ok(_) => true,
        Err(ElabError::UnboundName { name, .. }) => {
            assert!(
                query.unpublished_names.contains(&name),
                "publication query for {} failed at unrelated name `{name}`",
                query.surface
            );
            false
        }
        Err(other) => panic!(
            "publication query for {} failed unexpectedly: {other:?}\n{}",
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
                .expect("attached query has an import and theorem");
            let mut declaration = declaration.to_owned();
            let mut imports = Vec::new();
            for (index, dependency) in direct_imports
                .iter()
                .filter(|surface| surface.as_str() != subject)
                .enumerate()
            {
                let alias = format!("cc1_{label}_dependency_{query_index}_{index}");
                let renamed = rename_identifier(&declaration, dependency, &alias);
                if renamed != declaration {
                    imports.push(format!("import {module} ({dependency} as {alias})"));
                    declaration = renamed;
                }
            }
            imports.push(subject_import.to_owned());
            imports.push(declaration);
            query.source = imports.join("\n");
            probe(&query).then_some(query.surface)
        })
        .collect::<BTreeSet<_>>();

    direct.union(&attached).cloned().collect()
}

fn dependency_env() -> ElabEnv {
    let mut env = ElabEnv::empty().expect("prelude bootstrap");
    catalog_or::load_core_logic_compare(&mut env);
    catalog_or::expose_core_logic_transport(&mut env);
    // LawfulFunctors imports `Data.Collections.Derived (list_append)` after the
    // attached-proof migration; clear both imports and keep Derived importable.
    catalog_or::load_derived_importing_fixture_many(&mut env, &["concat_map", "list_append"]);
    catalog_or::load_lawful_functors_importing_fixture(&mut env);
    env.elaborate_module_from_roots(
        &[catalog_or::catalog_root()],
        "Core.Classes.EffectfulClasses",
    )
    .expect("Core.Classes.EffectfulClasses must roots-load fifth");
    catalog_or::expose_module(&mut env, "Core.Classes.EffectfulClasses");
    env
}

fn term_mentions_global(term: &Term, expected: GlobalId) -> bool {
    match term {
        Term::Const { id, .. } | Term::IndFormer { id, .. } | Term::Constructor { id, .. }
            if *id == expected =>
        {
            true
        }
        Term::Elim { fam, .. } if *fam == expected => true,
        _ => term
            .children()
            .into_iter()
            .any(|child| term_mentions_global(child, expected)),
    }
}

fn assert_transparent_globals(env: &ElabEnv, names: &[&str]) {
    for name in names {
        let id = *env
            .globals
            .get(*name)
            .unwrap_or_else(|| panic!("expected checked global `{name}`"));
        assert!(
            env.env.transparent_body(id).is_some(),
            "`{name}` must be a real transparent, kernel-checked term"
        );
    }
}

/// Promise class: normative compatibility vector.
///
/// MEASURED: every parsed NonEmpty declaration, constructor, explicit export,
/// and attached proof is queried through the real roots-loader interface, and
/// the successful names equal the stable §7 API. CLAIMED: NonEmpty exposes
/// exactly its authorized abstract surface. THE GAP: instance declarations
/// synthesize dictionary names rather than direct declaration heads, so their
/// explicit export declarations supply the inventory queries.
#[test]
fn nonempty_loader_visible_inventory_is_exact() {
    let expected = [
        "NonEmpty",
        "Semigroup_instance_NonEmpty",
        "nonempty_append",
        "nonempty_cons",
        "nonempty_head",
        "nonempty_map",
        "nonempty_singleton",
        "nonempty_tail",
        "nonempty_to_list",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect::<BTreeSet<_>>();
    assert_eq!(
        published_module_surfaces(NONEMPTY_KEN_MD, NONEMPTY_MODULE, "nonempty"),
        expected,
        "NonEmpty's loader-visible surface must equal its stable abstract API"
    );
}

/// Promise class: normative compatibility vector.
///
/// MEASURED: every parsed Validation declaration, raw constructor, explicit
/// export, and attached proof is queried through the real roots-loader
/// interface, and the successful set equals the stable §7 API. CLAIMED:
/// Validation's exact public surface stays transparent while private law
/// witnesses remain unimportable. THE GAP: synthesized instance names enter the
/// query population through their explicit export declarations.
#[test]
fn validation_loader_visible_inventory_is_exact_and_transparent() {
    let expected = [
        "Applicative_instance_Validation",
        "Functor_instance_Validation",
        "Invalid",
        "Valid",
        "Validation",
        "validation_ap",
        "validation_map",
        "validation_pure",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect::<BTreeSet<_>>();
    assert_eq!(
        published_module_surfaces(VALIDATION_KEN_MD, VALIDATION_MODULE, "validation"),
        expected,
        "Validation's loader-visible surface must equal its stable transparent API"
    );
}

/// Promise class: durable invariant.
///
/// MEASURED: a distinct client imports the Validation family and both raw
/// constructors, constructs both cases, and exhaustively matches them, while a
/// selective import of a private law witness gets exact `UnboundName`.
/// CLAIMED: Validation is transparent only through its explicit public import
/// edge; strict publication does not accidentally expose internal laws. THE
/// GAP: the real-client suites own the complementary import-removal controls.
#[test]
fn validation_clients_can_construct_and_match_only_the_public_transparent_surface() {
    let mut env = dependency_env();
    env.elaborate_module_from_roots(&[catalog_or::catalog_root()], VALIDATION_MODULE)
        .expect("Validation must roots-load through its declared dependencies");
    env.elaborate_file(
        "import Data.Sums.Validation (Invalid, Valid, Validation)\n\
         const cc1_invalid : Validation Nat Bool = Invalid Nat Bool Zero\n\
         const cc1_valid : Validation Nat Bool = Valid Nat Bool True\n\
         fn cc1_validation_match (x : Validation Nat Bool) : Bool = \
           match x { Invalid error ↦ False; Valid value ↦ value }",
    )
    .expect("a selective-import client must construct and match both Validation cases");

    match env.elaborate_file(
        "import Data.Sums.Validation (validation_ap_id as cc1_private_validation_law)",
    ) {
        Err(ElabError::UnboundName { name, .. }) => {
            assert_eq!(name, "Data.Sums.Validation.validation_ap_id")
        }
        other => panic!("a private Validation law must reject at its exact name: {other:?}"),
    }
}

/// Promise class: durable invariant.
///
/// MEASURED: the actual roots-loaded owner bodies mention the owner's raw
/// constructor, a client constructs and eliminates through the public API, and
/// construction, matching, and selective import of the raw constructor each
/// fail at its exact surface name. CLAIMED: the provider retains its transparent
/// owner face while clients receive only the abstract face. THE GAP: the exact
/// publication inventory above closes accidental publication of another owner
/// declaration; the CC legs below own the real downstream import paths.
#[test]
fn nonempty_owner_keeps_raw_constructor_while_clients_cannot_name_it() {
    let mut env = dependency_env();
    let trust_before = env.env.trusted_base();
    env.elaborate_module_from_roots(&[catalog_or::catalog_root()], NONEMPTY_MODULE)
        .expect("NonEmpty must roots-load through its abstract surface");
    let family = env.globals["Data.Collections.NonEmpty.NonEmpty"];
    let constructor = env.globals["Data.Collections.NonEmpty.NonEmptyCons"];
    for (owner, internal) in [
        ("Data.Collections.NonEmpty.nonempty_singleton", constructor),
        ("Data.Collections.NonEmpty.nonempty_cons", constructor),
        ("Data.Collections.NonEmpty.nonempty_head", family),
        ("Data.Collections.NonEmpty.nonempty_tail", family),
        ("Data.Collections.NonEmpty.nonempty_to_list", family),
        ("Data.Collections.NonEmpty.nonempty_map", constructor),
        ("Data.Collections.NonEmpty.nonempty_append", constructor),
        (
            "Data.Collections.NonEmpty.nonempty_append::assoc",
            constructor,
        ),
    ] {
        let (_, body) = env
            .env
            .transparent_body(env.globals[owner])
            .unwrap_or_else(|| panic!("owner declaration `{owner}` must remain transparent"));
        assert!(
            term_mentions_global(&body, internal),
            "owner declaration `{owner}` must retain its internal family or constructor identity"
        );
    }
    assert_eq!(
        env.env.trusted_base(),
        trust_before,
        "the abstract module boundary must add no trust"
    );

    env.elaborate_file(
        "import Data.Collections.NonEmpty (NonEmpty, nonempty_cons, nonempty_head)\n\
         const cc1_public_value : NonEmpty Nat = nonempty_cons Nat Zero (Nil Nat)\n\
         const cc1_public_head : Nat = nonempty_head Nat cc1_public_value",
    )
    .expect("a client must construct and eliminate through the public API");

    match env.elaborate_decl(
        "const cc1_forbidden_value : NonEmpty Nat = \
         Data.Collections.NonEmpty.NonEmptyCons Nat Zero (Nil Nat)",
    ) {
        Err(ElabError::UnboundName { name, .. }) => {
            assert_eq!(name, "Data.Collections.NonEmpty.NonEmptyCons")
        }
        other => panic!("qualified raw construction must reject at its exact name: {other:?}"),
    }
    match env.elaborate_decl(
        "fn cc1_forbidden_match (xs : NonEmpty Nat) : Nat = \
         match xs { NonEmptyCons x rest ↦ x }",
    ) {
        Err(ElabError::UnresolvedCon { name, .. }) => assert_eq!(name, "NonEmptyCons"),
        other => panic!("raw constructor matching must reject at its exact name: {other:?}"),
    }
    match env.elaborate_file(
        "import Data.Collections.NonEmpty (NonEmptyCons as cc1_forbidden_constructor)",
    ) {
        Err(ElabError::UnboundName { name, .. }) => {
            assert_eq!(name, "Data.Collections.NonEmpty.NonEmptyCons")
        }
        other => panic!("raw constructor import must reject at its exact name: {other:?}"),
    }
}

#[test]
fn ordered_dependency_closure_elaborates_both_packages_and_all_laws() {
    let mut env = dependency_env();

    env.elaborate_module_from_roots(&[catalog_or::catalog_root()], "Data.Collections.NonEmpty")
        .expect("Data.Collections.NonEmpty must roots-load");
    assert_transparent_globals(
        &env,
        &[
            "Data.Collections.NonEmpty.nonempty_singleton",
            "Data.Collections.NonEmpty.nonempty_cons",
            "Data.Collections.NonEmpty.nonempty_head",
            "Data.Collections.NonEmpty.nonempty_tail",
            "Data.Collections.NonEmpty.nonempty_to_list",
            "Data.Collections.NonEmpty.nonempty_map",
            "Data.Collections.NonEmpty.nonempty_append",
            "Data.Collections.NonEmpty.nonempty_append::assoc",
        ],
    );

    // Validation's roots-loaded import is the imported-head witness for the
    // synthesized Semigroup dictionary; it must not become a flat alias.

    env.elaborate_module_from_roots(&[catalog_or::catalog_root()], VALIDATION_MODULE)
        .expect("Data.Sums.Validation must roots-load");
    env.execute_loaded_entry_checked_fences(VALIDATION_MODULE)
        .expect("Data.Sums.Validation and every checked fence must elaborate");
    let canonical_dictionary = env.globals["Semigroup_instance_Data.Collections.NonEmpty.NonEmpty"];
    let (_, checked_record) = env
        .env
        .transparent_body(env.globals["checked_record"])
        .expect("the checked Validation example must remain transparent");
    assert!(
        term_mentions_global(&checked_record, canonical_dictionary),
        "the checked Validation example must consume the imported canonical dictionary"
    );
    assert_transparent_globals(
        &env,
        &[
            "Data.Sums.Validation.validation_map",
            "Data.Sums.Validation.validation_pure",
            "Data.Sums.Validation.validation_ap",
            "Data.Sums.Validation.validation_map::id",
            "Data.Sums.Validation.validation_map::fusion",
            "Data.Sums.Validation.validation_ap_id",
            "Data.Sums.Validation.validation_ap_hom",
            "Data.Sums.Validation.validation_ap_ich",
            "Data.Sums.Validation.validation_ap_cmp",
            "Data.Sums.Validation.validation_map_coh",
            "Functor_instance_Data.Sums.Validation.Validation",
            "Applicative_instance_Data.Sums.Validation.Validation",
            "expected_errors",
            "both_errors_accumulate",
        ],
    );
}

#[test]
fn cc1_packages_have_zero_trusted_base_delta() {
    let mut env = dependency_env();
    let before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    env.elaborate_module_from_roots(&[catalog_or::catalog_root()], "Data.Collections.NonEmpty")
        .expect("NonEmpty must roots-load");
    env.elaborate_module_from_roots(&[catalog_or::catalog_root()], VALIDATION_MODULE)
        .expect("Validation must roots-load");
    env.execute_loaded_entry_checked_fences(VALIDATION_MODULE)
        .expect("Validation's checked fences must elaborate");
    let after: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(
        before, after,
        "CC1 must add no primitive, opaque constant, postulate, or Axiom"
    );
}

/// Promise class: durable invariant.
///
/// The live resolver accepts Validation's lawful Applicative constraint while
/// rejecting the same type constructor under a Monad constraint. The positive
/// sibling makes an empty or unpopulated registry fail rather than vacuously
/// satisfying the negative assertion.
#[test]
fn validation_registry_has_applicative_but_no_monad_instance() {
    let mut env = dependency_env();
    env.elaborate_module_from_roots(&[catalog_or::catalog_root()], NONEMPTY_MODULE)
        .expect("NonEmpty must roots-load");
    env.elaborate_module_from_roots(&[catalog_or::catalog_root()], VALIDATION_MODULE)
        .expect("Validation must roots-load");
    env.elaborate_file(
        "import Data.Collections.NonEmpty (NonEmpty)\n\
         import Data.Sums.Validation (Validation)",
    )
    .expect("the registry probe must name its imported type constructors");

    assert_eq!(
        env.class_env
            .instance_search("Applicative", "Data.Sums.Validation.Validation"),
        Some(env.globals["Applicative_instance_Data.Sums.Validation.Validation"]),
        "Validation's lawful Applicative dictionary must populate the live registry"
    );
    env.elaborate_decl(
        "fn cc1_applicative_registry_probe \
           (x : Validation (NonEmpty String) Bool) \
         : Validation (NonEmpty String) Bool \
         where Applicative (Validation (NonEmpty String)) = x",
    )
    .expect("the live resolver must discharge Validation's Applicative constraint");

    match env.elaborate_decl(
        "fn cc1_monad_registry_probe \
           (x : Validation (NonEmpty String) Bool) \
         : Validation (NonEmpty String) Bool \
         where Monad (Validation (NonEmpty String)) = x",
    ) {
        Err(ElabError::NoInstance { class, ty, .. }) => {
            assert_eq!(class, "Monad");
            assert_eq!(ty, "Data.Sums.Validation.Validation");
        }
        other => panic!("Validation's Monad constraint must return exact NoInstance: {other:?}"),
    }
}
