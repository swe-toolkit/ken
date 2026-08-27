//! CAT-BSEARCH ordered-search acceptance.
//!
//! Public names are normative compatibility vectors. The concrete decision
//! tags, generic `Dec` result, loader reachability, and zero-trust delta are
//! durable semantic invariants.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::ElabEnv;
use ken_interp::eval::{eval, EvalStore, EvalVal};
use ken_kernel::Decl;

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
            let is_import = line.trim() == "import Core.Classes.LawfulClasses (Ord)";
            removed_import += usize::from(is_import);
            !is_import
        })
        .collect::<Vec<_>>()
        .join("\n");
    assert_eq!(
        removed_import, 1,
        "the legacy harness removes exactly the declared Ord import"
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

#[test]
fn entry_loads_through_declared_import_and_registers_decision_surface() {
    let env = roots_env();
    for name in ["elem", "sorted_for_search", "search"] {
        let qualified = format!("{MODULE}.{name}");
        assert!(
            env.globals.contains_key(&qualified),
            "`{qualified}` must be a real kernel-checked global"
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
                    (ordered_search_leq Bool Ord_instance_Bool True x) \
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
                    (ordered_search_leq Bool Ord_instance_Bool False x) \
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
                      (ordered_search_leq Bool Ord_instance_Bool False x) True) \
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
