//! CAT-DERIVED-PUB-EXPORT acceptance controls.
//!
//! Promise class: durable invariants. The ten public collection operations
//! retain their `Data.Collections.Derived` identities. The two `nth` bound
//! proofs, three `list_append` monoid-law attached proofs
//! (`list_append::{left_unit, assoc, right_unit}`), the `map::{id, fusion}`
//! proofs, and the checked `reverse::involutive` proof are published beside
//! their subjects.
//! The helper `reverse_snoc` and verified-sort carrier remain private.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::{parser, Decl, ElabEnv, ElabError};
use ken_kernel::{GlobalId, Term};

const DERIVED: &str = "Data.Collections.Derived";
const DERIVED_KEN_MD: &str =
    include_str!("../../../catalog/packages/Data/Collections/Derived.ken.md");

fn catalog_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn load_derived() -> ElabEnv {
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_module_from_roots(&[catalog_root()], DERIVED)
        .expect("Derived must elaborate through the real roots loader");
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
    let wrapper = env.globals[wrapper];
    let (_, body) = env
        .env
        .transparent_body(wrapper)
        .expect("consumer wrapper must remain transparent");
    assert!(
        term_mentions(&body, provider),
        "consumer wrapper must retain the selected Derived provider GlobalId"
    );
}

/// MEASURED: one real selective import accepts all ten public operations, and
/// each consumer wrapper retains the corresponding fully-qualified provider
/// identity. CLAIMED: visibility changes only the interface, never identity or
/// computation. THE GAP: existing package tests own the operations' behavior.
#[test]
fn derived_exports_all_ten_operation_identities() {
    let mut env = load_derived();
    let providers = [
        ("map", "cat_derived_pub_map"),
        ("map::id", "cat_derived_pub_map_id"),
        ("map::fusion", "cat_derived_pub_map_fusion"),
        ("filter", "cat_derived_pub_filter"),
        ("list_append", "cat_derived_pub_list_append"),
        ("nth", "cat_derived_pub_nth"),
        ("length", "cat_derived_pub_length"),
        ("reverse", "cat_derived_pub_reverse"),
        ("concat_map", "cat_derived_pub_concat_map"),
        ("eq_from_ord", "cat_derived_pub_eq_from_ord"),
        ("count", "cat_derived_pub_count"),
        ("bytes_nat_length", "cat_derived_pub_bytes_nat_length"),
    ]
    .map(|(name, wrapper)| {
        let qualified = format!("{DERIVED}.{name}");
        let id = env.globals[&qualified];
        assert!(
            env.env.transparent_body(id).is_some(),
            "{qualified} must retain its transparent provider artifact"
        );
        (id, wrapper)
    });

    env.elaborate_file(
        "import Data.Collections.Derived \
           (map, filter, list_append, nth, length, reverse, concat_map, eq_from_ord, count, bytes_nat_length)\n\
         import Core.Function.Combinators (comp, idf)\n\
         fn cat_derived_pub_map (xs : List Bool) : List Bool = \
           map Bool Bool (\\x. x) xs\n\
         theorem cat_derived_pub_map_id (a : Type) (xs : List a) : \
           Equal (List a) (map a a (idf a) xs) xs = map::id a xs\n\
         theorem cat_derived_pub_map_fusion \
           (a : Type) (b : Type) (c : Type) (g : b → c) (h : a → b) (xs : List a) : \
           Equal (List c) (map a c (comp a b c g h) xs) (map b c g (map a b h xs)) = \
           map::fusion a b c g h xs\n\
         fn cat_derived_pub_filter (xs : List Bool) : List Bool = \
           filter Bool (\\x. x) xs\n\
         fn cat_derived_pub_list_append (xs : List Bool) (ys : List Bool) : List Bool = \
           list_append Bool xs ys\n\
         fn cat_derived_pub_nth (n : Nat) (xs : List Bool) : Option Bool = nth Bool n xs\n\
         fn cat_derived_pub_length (xs : List Bool) : Nat = length Bool xs\n\
         fn cat_derived_pub_reverse (xs : List Bool) : List Bool = reverse Bool xs\n\
         fn cat_derived_pub_singleton (x : Bool) : List Bool = Cons Bool x (Nil Bool)\n\
         fn cat_derived_pub_concat_map (xs : List Bool) : List Bool = \
           concat_map Bool Bool cat_derived_pub_singleton xs\n\
         fn cat_derived_pub_leq (x : Bool) (y : Bool) : Bool = True\n\
         fn cat_derived_pub_eq (x : Bool) (y : Bool) : Bool = True\n\
         fn cat_derived_pub_eq_from_ord (x : Bool) (y : Bool) : Bool = \
           eq_from_ord Bool cat_derived_pub_leq x y\n\
         fn cat_derived_pub_count (x : Bool) (xs : List Bool) : Nat = \
           count Bool cat_derived_pub_eq x xs\n\
         fn cat_derived_pub_bytes_nat_length (bs : Bytes) : Nat = bytes_nat_length bs",
    )
    .expect("all ten Derived operations must be selectively importable together");

    for (provider, wrapper) in providers {
        assert_transparent_body_mentions(&env, wrapper, provider);
    }
}

struct PublicationQuery {
    surface: String,
    source: String,
    unpublished_names: BTreeSet<String>,
}

fn top_level_publication_queries() -> Vec<PublicationQuery> {
    let extracted = ken_elaborator::literate::extract_ken_md(DERIVED_KEN_MD)
        .expect("Derived literate source must extract");
    let declarations =
        parser::parse_decls(&extracted.source).expect("Derived extracted source must parse");
    let mut queries = Vec::new();

    for declaration in declarations {
        let declaration = declaration.unwrap_pub();
        let direct_name = match declaration {
            Decl::ViewDecl { .. }
            | Decl::LetDecl { .. }
            | Decl::PropDecl { .. }
            | Decl::TheoremDecl { .. }
            | Decl::AxiomDecl { .. }
            | Decl::DataDecl { .. }
            | Decl::ExplicitDataDecl { .. }
            | Decl::TypeAlias { .. }
            | Decl::ClassDecl { .. } => Some(declaration.name()),
            _ => None,
        };
        if let Some(name) = direct_name {
            queries.push(PublicationQuery {
                surface: name.to_owned(),
                source: format!("import {DERIVED} ({name})"),
                unpublished_names: BTreeSet::from([format!("{DERIVED}.{name}")]),
            });
            continue;
        }

        let Decl::AttachedProofDecl {
            proof_name,
            subject,
            params,
            theorem,
            body,
            ..
        } = declaration
        else {
            continue;
        };
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
        let theorem_and_separator = &extracted.source[theorem.span().start..body.span().start];
        let separator = theorem_and_separator
            .rfind('=')
            .expect("attached proof signature must end at its body separator");
        let theorem = theorem_and_separator[..separator].trim_end();
        let probe = format!("cat_derived_export_probe_{}", queries.len());
        let attached = format!("{subject}::{proof_name}");
        queries.push(PublicationQuery {
            surface: attached.clone(),
            source: format!(
                "import {DERIVED} ({subject})\n\
                 theorem {probe} {binders} : {theorem} = {attached} {arguments}"
            ),
            unpublished_names: BTreeSet::from([
                format!("{DERIVED}.{subject}"),
                format!("{DERIVED}.{attached}"),
            ]),
        });
    }

    queries.sort_by(|left, right| left.surface.cmp(&right.surface));
    queries
}

/// MEASURED: the real roots loader is asked whether every mechanically parsed,
/// publishable top-level definition is visible, including attached proofs via
/// their imported subjects; the successful set is compared with an independent
/// literal contract set. CLAIMED: Derived's complete loader-visible export
/// surface is exactly the ten authorized operations, two `nth` bound
/// proofs, three `list_append` monoid-law attached proofs, and the one
/// `map::{id, fusion}` proofs and `reverse::involutive` attached proof.
/// THE GAP: none
/// within the loader's publication forms represented by Derived's parsed
/// declarations.
#[test]
fn derived_loader_publishes_exactly_its_authorized_export_surface() {
    let mut env = load_derived();
    // Attached-proof probes restate the provider theorem outside Derived's
    // module namespace. `IsTrue` is already available in the base environment;
    // re-importing that package name would be ambiguous, so import only the
    // other foreign name from its canonical provider.
    env.elaborate_file(
        "import Core.Classes.LawfulClasses (leq_nat)\nimport Core.Function.Combinators (comp, idf)",
    )
    .expect("Derived proof dependencies must elaborate for publication queries");
    let published = top_level_publication_queries()
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
        BTreeSet::from([
            "bytes_nat_length".to_owned(),
            "concat_map".to_owned(),
            "count".to_owned(),
            "eq_from_ord".to_owned(),
            "filter".to_owned(),
            "length".to_owned(),
            "list_append".to_owned(),
            "list_append::assoc".to_owned(),
            "list_append::left_unit".to_owned(),
            "list_append::right_unit".to_owned(),
            "map".to_owned(),
            "map::fusion".to_owned(),
            "map::id".to_owned(),
            "nth".to_owned(),
            "nth::at_or_beyond_is_none".to_owned(),
            "nth::some_below_length".to_owned(),
            "reverse".to_owned(),
            "reverse::involutive".to_owned(),
        ]),
        "the roots loader must publish exactly Derived's authorized export surface: \
         ten operations, two map proofs, two nth proofs, three list_append proofs and reverse::involutive"
    );
}
