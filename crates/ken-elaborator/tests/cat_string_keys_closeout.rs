//! CAT-MIGRATE-TIER-C-DATA-VALUE StringKeys closeout controls.
//!
//! Promise class: durable invariants. StringKeys remains a consumer-only
//! compatibility package: it owns and publishes no definitions, its checked
//! examples directly consume exactly the two String operations from the lawful
//! class owner, and imported names do not become facade exports. The transitive
//! StringBijection certificate identity is owned by the separately landed
//! `string_keys_closure_retains_the_published_injectivity_identity` control.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::{parser, Decl, ElabEnv, ElabError, ExportForm};
use ken_kernel::{GlobalId, Term};

const LAWFUL: &str = "Core.Classes.LawfulClasses";
const STRING_KEYS: &str = "Data.Text.StringKeys";
const STRING_KEYS_KEN_MD: &str =
    include_str!("../../../catalog/packages/Data/Text/StringKeys.ken.md");

fn catalog_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn load_string_keys() -> (ElabEnv, Vec<GlobalId>) {
    let mut env = ElabEnv::new().expect("base environment");
    let owned = env
        .elaborate_module_from_roots(&[catalog_root()], STRING_KEYS)
        .expect("StringKeys must elaborate through its real LawfulClasses import");
    (env, owned)
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
    let alias = format!("cat_string_keys_export_direct_{index}");
    PublicationQuery {
        surface: surface.to_owned(),
        source: format!("import {STRING_KEYS} ({surface} as {alias})"),
        unpublished_names: BTreeSet::from([format!("{STRING_KEYS}.{surface}")]),
    }
}

fn publication_queries() -> Vec<PublicationQuery> {
    let extracted = ken_elaborator::literate::extract_ken_md(STRING_KEYS_KEN_MD)
        .expect("StringKeys literate source must extract");
    let declarations =
        parser::parse_decls(&extracted.source).expect("StringKeys extracted source must parse");
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
                let probe = format!("cat_string_keys_export_probe_{index}");
                let alias = format!("cat_string_keys_export_subject_{index}");
                let theorem = rename_identifier(theorem, subject, &alias);
                let surface = format!("{subject}::{proof_name}");
                attached.push(PublicationQuery {
                    surface: surface.clone(),
                    source: format!(
                        "import {STRING_KEYS} ({subject} as {alias})\n\
                         theorem {probe} {binders} : {theorem} = \
                           {alias}::{proof_name} {arguments}"
                    ),
                    unpublished_names: BTreeSet::from([
                        format!("{STRING_KEYS}.{subject}"),
                        format!("{STRING_KEYS}.{surface}"),
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

/// MEASURED: the roots loader returns no definition identity for the entry unit,
/// no qualified StringKeys definition exists, and every checked fence executes.
/// CLAIMED: StringKeys is a standalone consumer-only package with an empty owned
/// inventory. THE GAP: none; the loader result is the entry unit's complete
/// definition set, while dependencies are recorded as separate loaded units.
#[test]
fn string_keys_owned_inventory_is_empty_and_standalone() {
    let (mut env, owned) = load_string_keys();
    assert!(
        owned.is_empty(),
        "StringKeys must remain consumer-only, but it owns {owned:?}"
    );
    let prefix = format!("{STRING_KEYS}.");
    assert!(
        env.globals.keys().all(|name| !name.starts_with(&prefix)),
        "StringKeys must not own a qualified definition identity"
    );
    env.execute_loaded_entry_checked_fences(STRING_KEYS)
        .expect("StringKeys Definition and every checked fence must elaborate");
}

/// MEASURED: every parsed loader-publishable declaration or export is queried
/// against the already-loaded module, and the successful set is empty. Imported
/// LC names and an independent would-be local name each reject as exact
/// `UnboundName`. CLAIMED: StringKeys publishes nothing and is not a facade.
/// THE GAP: none among the exhaustively classified current declaration forms;
/// constructors and instance declarations are not independently selectable.
#[test]
fn string_keys_loader_visible_inventory_is_empty() {
    let (mut env, _) = load_string_keys();
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
        BTreeSet::new(),
        "StringKeys loader-visible inventory must stay empty"
    );

    for name in [
        "DecEq",
        "Ord",
        "string_deceq_eq",
        "string_ord_leq",
        "string_keys_local_name",
    ] {
        let source = format!("import {STRING_KEYS} ({name} as closeout_{name})");
        match env.elaborate_file(&source) {
            Err(ElabError::UnboundName { name: rejected, .. }) => {
                assert_eq!(rejected, format!("{STRING_KEYS}.{name}"));
            }
            Err(other) => {
                panic!("StringKeys import of {name} failed for the wrong reason: {other:?}")
            }
            Ok(_) => panic!("StringKeys unexpectedly published {name}"),
        }
    }
}

/// MEASURED: after executing the actual checked fences, the union of direct
/// LawfulClasses identities in every newly elaborated fence body is exactly the
/// two String operations, while neither imported class type identity occurs.
/// CLAIMED: StringKeys directly consumes only `string_deceq_eq` and
/// `string_ord_leq`; `DecEq` and `Ord` are carried class names, not direct term
/// dependencies. THE GAP: none; the before/after identity difference covers
/// every declaration produced by the entry's checked fences.
#[test]
fn string_keys_direct_lawfulclasses_consumption_is_exact() {
    let (mut env, _) = load_string_keys();
    let before = env.globals.values().copied().collect::<BTreeSet<_>>();
    env.execute_loaded_entry_checked_fences(STRING_KEYS)
        .expect("StringKeys checked fences must elaborate");
    let checked_ids = env
        .globals
        .values()
        .copied()
        .filter(|id| !before.contains(id))
        .collect::<BTreeSet<_>>();
    assert!(
        !checked_ids.is_empty(),
        "the checked-fence population must be non-empty"
    );
    let checked_bodies = checked_ids
        .iter()
        .map(|id| {
            env.env
                .transparent_body(*id)
                .unwrap_or_else(|| panic!("checked-fence identity {id:?} must be transparent"))
                .1
        })
        .collect::<Vec<_>>();

    let lawful_prefix = format!("{LAWFUL}.");
    let direct_lawful = env
        .globals
        .iter()
        .filter_map(|(name, id)| {
            name.strip_prefix(&lawful_prefix)
                .filter(|_| checked_bodies.iter().any(|body| term_mentions(body, *id)))
                .map(str::to_owned)
        })
        .collect::<BTreeSet<_>>();
    assert_eq!(
        direct_lawful,
        BTreeSet::from(["string_deceq_eq".to_owned(), "string_ord_leq".to_owned(),]),
        "StringKeys checked fences must consume exactly the two LC String operations"
    );

    for class in ["DecEq", "Ord"] {
        let class_id = env
            .class_env
            .class(class)
            .unwrap_or_else(|| panic!("{class} must be registered by LawfulClasses"))
            .projection
            .type_id;
        assert!(
            checked_bodies
                .iter()
                .all(|body| !term_mentions(body, class_id)),
            "{class} is carried in the import but must not be a direct checked-fence term dependency"
        );
    }
}
