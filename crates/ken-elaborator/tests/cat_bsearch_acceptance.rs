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

const MODULE: &str = "Algorithm.Searching.OrderedSearch";
const ORDERED_SEARCH_KEN_MD: &str =
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

fn legacy_env() -> ElabEnv {
    let mut env = ElabEnv::empty().expect("prelude bootstrap");
    catalog_or::load_core_logic_compare(&mut env);
    catalog_or::expose_core_logic_transport(&mut env);
    catalog_or::load_derived_fixture(&mut env);

    let extracted = ken_elaborator::literate::extract_ken_md(ORDERED_SEARCH_KEN_MD)
        .expect("OrderedSearch literate source must extract");
    let mut removed_import = 0;
    let source = extracted
        .source
        .lines()
        .filter(|line| {
            let is_import = line.trim() == "import Core.Classes.LawfulClasses (Ord, ord_leq_at)";
            removed_import += usize::from(is_import);
            !is_import
        })
        .collect::<Vec<_>>()
        .join("\n");
    assert_eq!(
        removed_import, 1,
        "the legacy harness removes exactly the declared order import"
    );
    env.elaborate_file(&source)
        .expect("OrderedSearch body must elaborate against the loaded Ord provider");
    env
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

#[test]
fn generic_decision_and_yes_no_evidence_instantiate() {
    let mut env = legacy_env();
    env.elaborate_file(
        "fn cat_bsearch_decision \
             (a : Type) (d : Ord a) (x : a) (xs : List a) \
             (sorted : sorted_for_search a d xs) \
           : Dec (Equal Bool (elem a d x xs) True) = \
           search a d x xs sorted\n\
         theorem cat_bsearch_sorted_true : \
           sorted_for_search Bool Ord_instance_Bool \
             (Cons Bool True (Nil Bool)) = \
           and_intro \
             ((x : Bool) \
               -> Equal Bool \
                    (elem Bool Ord_instance_Bool x (Nil Bool)) \
                    True \
               -> Equal Bool \
                    (ord_leq_at Bool Ord_instance_Bool True x) \
                    True) \
             (sorted_for_search Bool Ord_instance_Bool (Nil Bool)) \
             (\\x.match x { \
               True |-> \\member.Proved; \
               False |-> \\member.absurd member \
             }) \
             Proved\n\
         theorem cat_bsearch_sorted_false_true : \
           sorted_for_search Bool Ord_instance_Bool \
             (Cons Bool False (Cons Bool True (Nil Bool))) = \
           and_intro \
             ((x : Bool) \
               -> Equal Bool \
                    (elem Bool Ord_instance_Bool x \
                      (Cons Bool True (Nil Bool))) \
                    True \
               -> Equal Bool \
                    (ord_leq_at Bool Ord_instance_Bool False x) \
                    True) \
             (sorted_for_search Bool Ord_instance_Bool \
               (Cons Bool True (Nil Bool))) \
             (\\x.match x { \
               True |-> \\member.Proved; \
               False |-> \\member.Proved \
             }) \
             cat_bsearch_sorted_true",
    )
    .expect("generic Dec result and concrete sortedness witnesses must elaborate");

    for (name, query, list, sorted, expected) in [
        ("empty_absent", "False", "Nil Bool", "Proved", false),
        (
            "head_present",
            "False",
            "Cons Bool False (Cons Bool True (Nil Bool))",
            "cat_bsearch_sorted_false_true",
            true,
        ),
        (
            "tail_present",
            "True",
            "Cons Bool False (Cons Bool True (Nil Bool))",
            "cat_bsearch_sorted_false_true",
            true,
        ),
        (
            "pruned_absent",
            "False",
            "Cons Bool True (Nil Bool)",
            "cat_bsearch_sorted_true",
            false,
        ),
        (
            "tail_absent",
            "True",
            "Cons Bool False (Nil Bool)",
            "and_intro \
               ((x : Bool) \
                 -> Equal Bool \
                      (elem Bool Ord_instance_Bool x (Nil Bool)) True \
                 -> Equal Bool \
                      (ord_leq_at Bool Ord_instance_Bool False x) True) \
               (sorted_for_search Bool Ord_instance_Bool (Nil Bool)) \
               (\\x.\\member.absurd member) Proved",
            false,
        ),
    ] {
        let proposition = format!("Equal Bool (elem Bool Ord_instance_Bool {query} ({list})) True");
        let declaration = format!(
            "const cat_bsearch_{name} : Bool = \
             decide ({proposition}) \
               (search Bool Ord_instance_Bool {query} ({list}) ({sorted}))"
        );
        env.elaborate_decl(&declaration)
            .unwrap_or_else(|error| panic!("{name} decision must elaborate: {error}"));
        assert_eq!(
            evaluate_bool(&env, &format!("cat_bsearch_{name}")),
            expected,
            "{name}"
        );
    }
}
