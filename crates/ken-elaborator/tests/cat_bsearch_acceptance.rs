//! CAT-BSEARCH ordered-search acceptance.
//!
//! Public names are normative compatibility vectors. The concrete decision
//! tags, generic `Dec` result, loader reachability, and zero-trust delta are
//! durable semantic invariants.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::ElabEnv;
use ken_interp::eval::{eval, EvalStore, EvalVal};
use ken_kernel::{convert_type, Context, Decl, GlobalId, Term};

#[path = "support/catalog_or.rs"]
mod catalog_or;
#[path = "support/catalog_publication.rs"]
mod catalog_publication;

const MODULE: &str = "Algorithm.Searching.OrderedSearch";
const ORDERED_SEARCH_SOURCE: &str =
    include_str!("../../../catalog/packages/Algorithm/Searching/OrderedSearch.ken.md");
fn catalog_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn roots_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("prelude bootstrap");
    env.elaborate_module_from_roots(&[catalog_root()], MODULE)
        .expect("Algorithm.Searching.OrderedSearch must load with its declared imports");
    env
}

fn application_head_and_arity(mut term: &Term) -> (&Term, usize) {
    let mut arity = 0;
    while let Term::App(function, _) = term {
        arity += 1;
        term = function;
    }
    (term, arity)
}

fn declaration_type(declaration: &Decl) -> Option<&Term> {
    match declaration {
        Decl::Transparent { ty, .. } | Decl::Opaque { ty, .. } | Decl::Primitive { ty, .. } => {
            Some(ty)
        }
        Decl::Inductive(_) => None,
    }
}

fn collect_order_call_providers(
    env: &ElabEnv,
    term: &Term,
    provider_type: &Term,
    providers: &mut Vec<GlobalId>,
) {
    let (head, arity) = application_head_and_arity(term);
    if arity == 4 {
        if let Term::Const { id, .. } = head {
            if let Some(candidate_type) = env.env.lookup(*id).and_then(declaration_type) {
                if convert_type(&env.env, &Context::new(), candidate_type, provider_type) {
                    providers.push(*id);
                }
            }
        }
    }
    for child in term.children() {
        collect_order_call_providers(env, child, provider_type, providers);
    }
}

fn evaluate_bool(env: &ElabEnv, name: &str) -> bool {
    let body = match env.env.lookup(env.globals[name]) {
        Some(Decl::Transparent { body, .. }) => body,
        other => panic!("{name} must be transparent, got {other:?}"),
    };
    let mut store = EvalStore::new();
    store
        .num_values
        .insert(env.class_env.record_nil_val_id, EvalVal::Bool(false));
    match eval(&[], body, &env.env, &mut store) {
        EvalVal::Ctor { id, .. } if id == env.numeric_env.bool_true_id => true,
        EvalVal::Ctor { id, .. } if id == env.numeric_env.bool_false_id => false,
        other => panic!("expected a Boolean decision tag, got {other:?}"),
    }
}

/// Promise class: durable invariant. The exact module publication surface is
/// independently queried through selective imports, including re-exports.
/// MEASURED: the source declaration forms and roots loader agree on precisely
/// these five importable names. CLAIMED: internal `elem_step` stays private and
/// the repaired `search` operation remains public. THE GAP: the query helper
/// covers direct declarations, attached proofs, and both re-export forms;
/// consumers that merely read `globals` are not publication evidence.
#[test]
fn ordered_search_publishes_exactly_its_authorized_surface() {
    assert_eq!(
        catalog_publication::published_module_surfaces(
            ORDERED_SEARCH_SOURCE,
            MODULE,
            "cat_bsearch_ordered_search",
        ),
        ["ListMembership", "MkListMembership", "elem", "search", "sorted_for_search"]
            .into_iter()
            .map(str::to_owned)
            .collect::<BTreeSet<_>>(),
    );
}

/// Promise classes: durable provider-identity invariant; transition sentinels
/// for the three complete call-population counts. An authorized change to the
/// OrderedSearch algorithm must rederive those counts before retiring the red.
///
/// MEASURED: roots loading leaves no OrderedSearch-local wrapper. For every
/// fully applied four-argument global whose type converts to `ord_leq_at`'s type,
/// the complete per-body populations are `elem = 2`, `sorted_for_search = 1`,
/// and `search = 22`, and every member is the qualified LawfulClasses provider's
/// exact `GlobalId`. CLAIMED: the selective import, rather than any interleaved
/// same-behavior local definition, supplies every retained ordering call. THE
/// GAP: computational behavior is exercised independently below; identity alone
/// does not establish the search result.
#[test]
fn entry_loads_through_declared_import_and_uses_canonical_order_provider() {
    let env = roots_env();
    let provider = env.globals["Core.Classes.LawfulClasses.ord_leq_at"];
    assert!(
        !env.globals
            .contains_key("Algorithm.Searching.OrderedSearch.ordered_search_leq"),
        "the deleted OrderedSearch-local provider must remain absent"
    );

    let provider_type = declaration_type(
        env.env
            .lookup(provider)
            .expect("the canonical order provider must be declared"),
    )
    .expect("the canonical order provider must have a global type");
    for (name, expected_calls) in [("elem", 2), ("sorted_for_search", 1), ("search", 22)] {
        let qualified = format!("{MODULE}.{name}");
        let id = env.globals[&qualified];
        let (_, body) = env
            .env
            .transparent_body(id)
            .unwrap_or_else(|| panic!("`{qualified}` must remain transparent"));
        let mut call_providers = Vec::new();
        collect_order_call_providers(&env, &body, provider_type, &mut call_providers);
        assert_eq!(
            call_providers.len(),
            expected_calls,
            "`{qualified}` must retain its complete ordering-call population"
        );
        assert!(
            call_providers.iter().all(|id| *id == provider),
            "every `{qualified}` ordering call must name the canonical LawfulClasses provider GlobalId; got {call_providers:?}"
        );
    }
}

#[test]
fn entry_adds_no_trusted_declarations() {
    let mut env = ElabEnv::new().expect("prelude bootstrap");
    env.elaborate_module_from_roots(&[catalog_root()], "Core.Classes.LawfulClasses")
        .expect("the declared Ord provider must load");
    let before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    env.elaborate_module_from_roots(&[catalog_root()], MODULE)
        .expect("OrderedSearch must roots-load");
    let after: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(
        before, after,
        "ordered search must add zero trusted declarations"
    );
}

/// Promise class: durable behavior invariant. A public-import client resolves
/// the owned Ord Bool dictionary and checks both sortedness evidence and
/// positive/negative search decisions on distinct nondegenerate inputs.
#[test]
fn generic_decision_and_yes_no_evidence_instantiate() {
    let mut env = roots_env();
    let lawful_owned = env
        .elaborate_module_from_roots(&[catalog_root()], "Core.Classes.LawfulClasses")
        .expect("the class owner must roots-load in the same environment");
    let ord_class = env
        .class_env
        .class("Ord")
        .expect("Ord must remain a checked class")
        .projection
        .type_id;
    assert!(lawful_owned.contains(&ord_class), "LawfulClasses must own Ord");
    let bool_instance = env
        .class_env
        .instance_search("Ord", "Bool")
        .expect("Ord Bool must remain registered");
    assert!(lawful_owned.contains(&bool_instance), "Ord Bool must be owner-checked");
    catalog_or::expose_module(&mut env, "Core.Classes.LawfulClasses");
    catalog_or::expose_module(&mut env, MODULE);
    catalog_or::expose_module(&mut env, "Core.Logic.EmptyDec");
    env.elaborate_file(
        "import Core.Classes.LawfulClasses (Ord, ord_leq_at)\n\
         import Algorithm.Searching.OrderedSearch (elem, sorted_for_search, search)\n\
         import Core.Logic.EmptyDec (Dec as empty_dec_Dec, decide as empty_dec_decide)\n\
         fn cat_bsearch_decision \
             (a : Type) (d : Ord a) (x : a) (xs : List a) \
             (sorted : sorted_for_search a d xs) \
           : empty_dec_Dec (Equal Bool (elem a d x xs) True) = \
           search a d x xs sorted",
    )
    .expect("generic Dec result must elaborate from public imports");

    let sorted_witnesses = "let sorted_true = \
      and_intro \
        ((x : Bool) \
          -> Equal Bool (elem Bool d x (Nil Bool)) True \
          -> Equal Bool (ord_leq_at Bool d True x) True) \
        (sorted_for_search Bool d (Nil Bool)) \
        (\\x.match x { \
          True |-> \\member.Proved; \
          False |-> \\member.absurd member \
        }) Proved; \
      sorted_false_true = \
      and_intro \
        ((x : Bool) \
          -> Equal Bool (elem Bool d x (Cons Bool True (Nil Bool))) True \
          -> Equal Bool (ord_leq_at Bool d False x) True) \
        (sorted_for_search Bool d (Cons Bool True (Nil Bool))) \
        (\\x.match x { \
          True |-> \\member.Proved; \
          False |-> \\member.Proved \
        }) sorted_true \
      in ";

    for (name, query, list, sorted, expected) in [
        ("empty_absent", "False", "Nil Bool", "Proved", false),
        (
            "head_present",
            "False",
            "Cons Bool False (Cons Bool True (Nil Bool))",
            "sorted_false_true",
            true,
        ),
        (
            "tail_present",
            "True",
            "Cons Bool False (Cons Bool True (Nil Bool))",
            "sorted_false_true",
            true,
        ),
        (
            "pruned_absent",
            "False",
            "Cons Bool True (Nil Bool)",
            "sorted_true",
            false,
        ),
        (
            "tail_absent",
            "True",
            "Cons Bool False (Nil Bool)",
            "and_intro \
               ((x : Bool) \
                 -> Equal Bool \
                      (elem Bool d x (Nil Bool)) True \
                 -> Equal Bool \
                      (ord_leq_at Bool d False x) True) \
               (sorted_for_search Bool d (Nil Bool)) \
               (\\x.\\member.absurd member) Proved",
            false,
        ),
    ] {
        let proposition = format!("Equal Bool (elem Bool d {query} ({list})) True");
        let declaration = format!(
            "import Core.Classes.LawfulClasses (Ord)\n\
             import Algorithm.Searching.OrderedSearch (elem, sorted_for_search, search)\n\
             import Core.Logic.EmptyDec (decide as empty_dec_decide)\n\
             const cat_bsearch_{name} : Bool where Ord Bool = \
             {sorted_witnesses} empty_dec_decide ({proposition}) \
               (search Bool d {query} ({list}) ({sorted}))"
        );
        env.elaborate_file(&declaration)
            .unwrap_or_else(|error| panic!("{name} decision must elaborate: {error}"));
        let case_id = env.globals[&format!("cat_bsearch_{name}")];
        let references = catalog_or::declaration_references(
            env.env.lookup(case_id).expect("the checked decision must exist"),
        );
        assert!(
            references.contains(&bool_instance),
            "{name} must resolve its Ord Bool dictionary from the loaded class owner"
        );
        assert_eq!(
            evaluate_bool(&env, &format!("cat_bsearch_{name}")),
            expected,
            "{name}"
        );
    }
}
