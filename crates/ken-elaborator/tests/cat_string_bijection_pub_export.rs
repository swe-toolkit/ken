//! CAT-MIGRATE-TIER-B-PROVIDERS StringBijection surface controls.
//!
//! Promise class: durable invariants. The roots-loaded module exposes exactly
//! its authorized injectivity certificate while retaining the canonical
//! Transport provider identities and keeping its premise private.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::{parser, Decl, ElabEnv, ElabError, ExportForm};
use ken_kernel::{GlobalId, Term};

const STRING_BIJECTION: &str = "Data.Text.StringBijection";
const STRING_BIJECTION_KEN_MD: &str =
    include_str!("../../../catalog/packages/Data/Text/StringBijection.ken.md");
const TRANSPORT: &str = "Core.Logic.Transport";

fn catalog_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn load_string_bijection() -> ElabEnv {
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_module_from_roots(&[catalog_root()], STRING_BIJECTION)
        .expect("StringBijection must elaborate through its real import closure");
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

struct PublicationQuery {
    surface: String,
    source: String,
    unpublished_names: BTreeSet<String>,
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
    let alias = format!("cat_string_export_direct_{index}");
    PublicationQuery {
        surface: surface.to_owned(),
        source: format!("import {STRING_BIJECTION} ({surface} as {alias})"),
        unpublished_names: BTreeSet::from([format!("{STRING_BIJECTION}.{surface}")]),
    }
}

fn publication_queries() -> Vec<PublicationQuery> {
    let extracted = ken_elaborator::literate::extract_ken_md(STRING_BIJECTION_KEN_MD)
        .expect("StringBijection literate source must extract");
    let declarations = parser::parse_decls(&extracted.source)
        .expect("StringBijection extracted source must parse");
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
                let probe = format!("cat_string_export_probe_{index}");
                let alias = format!("cat_string_export_subject_{index}");
                let theorem = rename_identifier(theorem, subject, &alias);
                let surface = format!("{subject}::{proof_name}");
                attached.push(PublicationQuery {
                    surface: surface.clone(),
                    source: format!(
                        "import {STRING_BIJECTION} ({subject} as {alias})\n\
                         theorem {probe} {binders} : {theorem} = \
                           {alias}::{proof_name} {arguments}"
                    ),
                    unpublished_names: BTreeSet::from([
                        format!("{STRING_BIJECTION}.{subject}"),
                        format!("{STRING_BIJECTION}.{surface}"),
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

    direct.extend(attached);
    direct
}

/// MEASURED: a roots load succeeds from the module's declared imports, the
/// injectivity body retains the exact `cong`, `sym`, and `trans` provider IDs,
/// and a selective consumer keeps the injectivity certificate's provider ID.
/// CLAIMED: StringBijection is standalone and publishes the certificate without
/// replacing or duplicating its dependencies. THE GAP: its private premise and
/// complete public surface are guarded separately below.
#[test]
fn string_bijection_standalone_imports_retain_provider_identities() {
    let mut env = load_string_bijection();
    let injective = env.globals[&format!("{STRING_BIJECTION}.string_to_list_char_injective")];
    let (_, injective_body) = env
        .env
        .transparent_body(injective)
        .expect("the injectivity certificate must remain transparent");
    for name in ["cong", "sym", "trans"] {
        let provider = env.globals[&format!("{TRANSPORT}.{name}")];
        assert!(
            term_mentions(&injective_body, provider),
            "the injectivity certificate must retain Transport.{name}'s exact provider identity"
        );
    }

    env.elaborate_file(
        "import Data.Text.StringBijection \
           (string_to_list_char_injective as imported_string_injective)\n\
         theorem cat_string_pub_injective \
             (left : String) \
             (right : String) \
             (same_chars : Equal \
               (List Char) \
               (string_to_list_char left) \
               (string_to_list_char right)) \
           : Equal String left right = \
         imported_string_injective left right same_chars",
    )
    .expect("the injectivity certificate must be selectively importable");
    let (_, wrapper_body) = env
        .env
        .transparent_body(env.globals["cat_string_pub_injective"])
        .expect("the selective-import wrapper must remain transparent");
    assert!(
        term_mentions(&wrapper_body, injective),
        "the consumer wrapper must retain the StringBijection provider GlobalId"
    );

    match env.elaborate_file("import Data.Text.StringBijection (string_to_list_char_retraction)") {
        Err(ElabError::UnboundName { name, .. }) => assert_eq!(
            name,
            "Data.Text.StringBijection.string_to_list_char_retraction"
        ),
        Err(other) => panic!("private premise import must fail as UnboundName: {other:?}"),
        Ok(_) => panic!("string_to_list_char_retraction became selectively importable"),
    }
}

/// MEASURED: every parsed loader-publishable definition is queried through the
/// roots loader, and the successful set is compared with an independent literal
/// contract set. CLAIMED: the complete loader-visible StringBijection surface is
/// exactly the injectivity certificate. THE GAP: none among the exhaustively
/// classified current declaration variants; constructors and non-publishable
/// declaration forms are not loader-selectable definitions.
#[test]
fn string_bijection_loader_visible_inventory_is_exact() {
    let mut env = load_string_bijection();
    let published = publication_queries()
        .into_iter()
        .filter(|query| match env.elaborate_file(&query.source) {
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
        })
        .map(|query| query.surface)
        .collect::<BTreeSet<_>>();

    assert_eq!(
        published,
        BTreeSet::from(["string_to_list_char_injective".to_owned()]),
        "StringBijection loader-visible inventory must equal its authorized surface"
    );
}
