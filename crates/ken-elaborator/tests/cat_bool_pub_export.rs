//! CAT-BOOL-PUB-EXPORT provider-surface acceptance controls.
//!
//! Promise class: durable invariants. The two provider modules expose exactly
//! their authorized loader-visible surfaces while retaining provider identity.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::{parser, Decl, ElabEnv, ElabError, ExportForm};
use ken_kernel::{GlobalId, Term};

const LAWFUL: &str = "Core.Classes.LawfulClasses";
const SUMS: &str = "Data.Sums.Combinators";
const LAWFUL_KEN_MD: &str =
    include_str!("../../../catalog/packages/Core/Classes/LawfulClasses.ken.md");
const SUMS_KEN_MD: &str = include_str!("../../../catalog/packages/Data/Sums/Combinators.ken.md");

fn catalog_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn load_module(module: &str) -> ElabEnv {
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_module_from_roots(&[catalog_root()], module)
        .unwrap_or_else(|error| {
            panic!("{module} must elaborate through the roots loader: {error:?}")
        });
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

fn assert_transparent_body_mentions(env: &ElabEnv, wrapper: &str, provider: GlobalId) {
    let wrapper_name = wrapper;
    let wrapper = env.globals[wrapper];
    let (_, body) = env
        .env
        .transparent_body(wrapper)
        .expect("consumer wrapper must remain transparent");
    assert!(
        term_mentions(&body, provider),
        "consumer wrapper `{wrapper_name}` must retain the selected provider GlobalId"
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

fn direct_publication_query(module: &str, surface: &str, index: usize) -> PublicationQuery {
    let alias = format!("cat_bool_export_direct_{index}");
    PublicationQuery {
        surface: surface.to_owned(),
        source: format!("import {module} ({surface} as {alias})"),
        unpublished_names: BTreeSet::from([format!("{module}.{surface}")]),
    }
}

fn module_publication_queries(module: &str, ken_md: &str) -> ModulePublicationQueries {
    let extracted = ken_elaborator::literate::extract_ken_md(ken_md)
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
            | Decl::DataDecl { .. }
            | Decl::ExplicitDataDecl { .. }
            | Decl::TypeAlias { .. }
            | Decl::ClassDecl { .. } => {
                let name = declaration.name();
                direct.push(direct_publication_query(module, name, direct.len()));
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
                let probe = format!("cat_bool_export_probe_{index}");
                let alias = format!("cat_bool_export_subject_{index}");
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
                    direct.push(direct_publication_query(module, surface, direct.len()));
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

fn published_module_surfaces(module: &str, ken_md: &str) -> BTreeSet<String> {
    let queries = module_publication_queries(module, ken_md);
    let mut env = load_module(module);
    if !queries.dependency_imports.is_empty() {
        env.elaborate_file(&queries.dependency_imports)
            .unwrap_or_else(|error| {
                panic!("{module} dependency imports must resolve for publication probes: {error:?}")
            });
    }

    queries
        .direct
        .into_iter()
        .chain(queries.attached)
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
        .collect()
}

/// MEASURED: real selective imports of the target definitions resolve to their
/// exact transparent provider identities, the loaded `DecEq Bool` dictionary is
/// the class registry's canonical instance, and private siblings still reject at
/// their qualified interface names. CLAIMED: the authorized visibility change
/// preserves definition identity, publishes the canonical class and dictionary,
/// and retains the loader's selective-import boundary. THE GAP: provider package
/// tests own computational behavior; the inventory equality below owns surface
/// closure.
#[test]
fn boolean_provider_selective_imports_retain_provider_identities() {
    let mut env = load_module(LAWFUL);
    env.elaborate_module_from_roots(&[catalog_root()], SUMS)
        .expect("Sums provider must roots-load beside LawfulClasses");
    let dec_eq = env
        .class_env
        .class("DecEq")
        .expect("DecEq class must be registered")
        .projection
        .type_id;
    let bool_and = env.globals[&format!("{LAWFUL}.bool_and")];
    let bool_eq = env.globals[&format!("{LAWFUL}.bool_eq")];
    let bool_leq = env.globals[&format!("{LAWFUL}.bool_leq")];
    let ord_leq_at = env.globals[&format!("{LAWFUL}.ord_leq_at")];
    let dec_eq_bool = env
        .class_env
        .instance_search("DecEq", "Bool")
        .expect("the roots loader must register LawfulClasses' DecEq Bool dictionary");
    let is_some = env.globals[&format!("{SUMS}.is_some")];
    for provider in [
        dec_eq,
        bool_and,
        bool_eq,
        bool_leq,
        ord_leq_at,
        dec_eq_bool,
        is_some,
    ] {
        assert!(
            env.env.transparent_body(provider).is_some(),
            "a public class, instance, Boolean, or order provider must retain its transparent body"
        );
    }
    assert_eq!(
        env.class_env
            .class("DecEq")
            .expect("DecEq class must be registered")
            .projection
            .type_id,
        dec_eq,
        "the loaded class registry must retain LawfulClasses' canonical DecEq identity"
    );
    assert_eq!(
        env.class_env.instance_search("DecEq", "Bool"),
        Some(dec_eq_bool),
        "the roots loader must register LawfulClasses' canonical DecEq Bool dictionary"
    );

    env.elaborate_file(
        "import Core.Classes.LawfulClasses \
           (DecEq, Ord, bool_and, bool_eq, bool_leq, ord_leq_at)\n\
         import Data.Sums.Combinators (is_some)\n\
         fn cat_bool_pub_and (x : Bool) (y : Bool) : Bool = bool_and x y\n\
         fn cat_bool_pub_eq (x : Bool) (y : Bool) : Bool = bool_eq x y\n\
         fn cat_bool_pub_leq (x : Bool) (y : Bool) : Bool = bool_leq x y\n\
         fn cat_ord_pub_leq_at \
           (a : Type) (d : Ord a) (x : a) (y : a) : Bool = ord_leq_at a d x y\n\
         fn cat_bool_pub_resolve (x : Bool) : Bool \
           where (db : DecEq Bool) = db.eq x x\n\
         const cat_bool_pub_resolved : Bool = cat_bool_pub_resolve True\n\
         fn cat_bool_pub_some (x : Option Bool) : Bool = is_some Bool x",
    )
    .expect("the Boolean and order providers must be selectively importable together");
    assert_transparent_body_mentions(&env, "cat_bool_pub_and", bool_and);
    assert_transparent_body_mentions(&env, "cat_bool_pub_eq", bool_eq);
    assert_transparent_body_mentions(&env, "cat_bool_pub_leq", bool_leq);
    assert_transparent_body_mentions(&env, "cat_ord_pub_leq_at", ord_leq_at);
    assert_transparent_body_mentions(&env, "cat_bool_pub_some", is_some);
    let resolution = env
        .class_env
        .resolution_provenance
        .iter()
        .rev()
        .find(|resolution| resolution.class_name == "DecEq" && resolution.head_type == "Bool")
        .expect("the selective consumer must record DecEq Bool resolution provenance");
    assert_eq!(resolution.instance_id, dec_eq_bool);
    assert_eq!(resolution.defining_package, LAWFUL);

    for (module, private) in [(LAWFUL, "Eq"), (LAWFUL, "int_leq"), (SUMS, "get_or_else")] {
        match env.elaborate_file(&format!("import {module} ({private})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{module}.{private}"));
            }
            Err(other) => panic!("private import must fail as UnboundName: {other:?}"),
            Ok(_) => panic!("{module}.{private} became selectively importable"),
        }
    }

    let private_proof = module_publication_queries(LAWFUL, LAWFUL_KEN_MD)
        .attached
        .into_iter()
        .find(|query| query.surface == "ord_leq_at::true_of_equal")
        .expect("ord_leq_at's attached proof must remain in the source population");
    match env.elaborate_file(&private_proof.source) {
        Err(ElabError::UnboundName { name, .. }) => {
            assert_eq!(name, format!("{LAWFUL}.ord_leq_at::true_of_equal"));
        }
        Err(other) => panic!("private attached proof must fail as UnboundName: {other:?}"),
        Ok(_) => panic!("ord_leq_at::true_of_equal became loader-visible"),
    }
}

/// MEASURED: for both real provider modules, every parsed loader-surface source
/// is queried through the roots loader: publishable top-level definitions,
/// attached definitions, and every facade or in-scope re-export item under its
/// selectable spelling. Successful queries are compared with literal authorized
/// interface sets. CLAIMED: the complete loader-visible surfaces are exactly the
/// authorized names. THE GAP: none among the exhaustively classified current
/// declaration variants; constructors and non-publishable declaration forms are
/// not loader-selectable definitions.
#[test]
fn boolean_provider_loader_visible_inventories_are_exact() {
    assert_eq!(
        published_module_surfaces(LAWFUL, LAWFUL_KEN_MD),
        BTreeSet::from([
            "DecEq".to_owned(),
            "IsTrue".to_owned(),
            "Ord".to_owned(),
            "bool_and".to_owned(),
            "bool_eq".to_owned(),
            "bool_leq".to_owned(),
            "bool_or".to_owned(),
            "bool_or::eq_true_of_or".to_owned(),
            "leq_nat".to_owned(),
            "leq_nat::antisym".to_owned(),
            "leq_nat::refl".to_owned(),
            "leq_nat::trans".to_owned(),
            "ord_leq_at".to_owned(),
        ]),
        "LawfulClasses loader-visible inventory must equal its authorized surface"
    );
    assert_eq!(
        published_module_surfaces(SUMS, SUMS_KEN_MD),
        BTreeSet::from(["is_some".to_owned()]),
        "Sums loader-visible inventory must equal its authorized surface"
    );
}
