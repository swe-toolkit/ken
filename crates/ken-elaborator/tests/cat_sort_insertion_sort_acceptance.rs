//! CAT-SORT insertion-sort acceptance.
//!
//! The public-name assertion is a normative compatibility vector. The law,
//! behavior, and trust assertions are durable invariants: implementation and
//! proof structure may change while those properties remain fixed.

#[path = "support/catalog_or.rs"]
mod catalog_or;

use std::collections::BTreeSet;

use ken_elaborator::ElabEnv;
use ken_interp::eval::{eval, EvalStore, EvalVal};
use ken_kernel::Decl;

const TRANSPORT_KEN_MD: &str =
    include_str!("../../../catalog/packages/Core/Logic/Transport.ken.md");
const COLLECTIONS_KEN_MD: &str =
    include_str!("../../../catalog/packages/Data/Collections/Derived.ken.md");
const LAWFUL_CLASSES_KEN_MD: &str =
    include_str!("../../../catalog/packages/Core/Classes/LawfulClasses.ken.md");
const INSERTION_SORT_KEN_MD: &str =
    include_str!("../../../catalog/packages/Algorithm/Sorting/InsertionSort.ken.md");

fn base_env() -> ElabEnv {
    let mut env = ElabEnv::empty().expect("prelude bootstrap");
    catalog_or::load_core_logic_or(&mut env);
    env.elaborate_ken_md_file(TRANSPORT_KEN_MD)
        .expect("Core/Logic/Transport.ken.md must elaborate");
    env.elaborate_ken_md_file(COLLECTIONS_KEN_MD)
        .expect("Data/Collections/Derived.ken.md must elaborate");
    env.elaborate_ken_md_file(LAWFUL_CLASSES_KEN_MD)
        .expect("Core/Classes/LawfulClasses.ken.md must elaborate");
    // The current sequential package harness has no module namespace. Hide the
    // earlier generic operations so this package can own its public `insert`
    // and `sort` names, as a real module import would.
    env.globals.remove("insert");
    env.globals.remove("sort");
    env
}

fn loaded_env() -> ElabEnv {
    let mut env = base_env();
    env.elaborate_ken_md_file(INSERTION_SORT_KEN_MD)
        .expect("Algorithm/Sorting/InsertionSort.ken.md must elaborate");
    env
}

fn boolean_list(env: &ElabEnv, value: EvalVal) -> Vec<bool> {
    let mut current = value;
    let mut result = Vec::new();
    loop {
        match current {
            EvalVal::Ctor { id, .. } if id == env.prelude_env.nil_id => return result,
            EvalVal::Ctor { id, args, .. } if id == env.prelude_env.cons_id => {
                let head = match &args[1] {
                    EvalVal::Ctor { id, .. } if *id == env.numeric_env.bool_true_id => true,
                    EvalVal::Ctor { id, .. } if *id == env.numeric_env.bool_false_id => false,
                    other => panic!("expected a Boolean list head, got {other:?}"),
                };
                result.push(head);
                current = args[2].clone();
            }
            other => panic!("expected a Boolean List constructor chain, got {other:?}"),
        }
    }
}

fn evaluate_boolean_list(env: &ElabEnv, id: ken_kernel::GlobalId) -> Vec<bool> {
    let body = match env.env.lookup(id) {
        Some(Decl::Transparent { body, .. }) => body,
        other => panic!("sort vector must be transparent, got {other:?}"),
    };
    let mut store = EvalStore::new();
    // Class dictionaries end in the structural `record_nil_val` postulate.
    // Give that erased tail a closed runtime sentinel so evaluation can reach
    // the computational `leq` field; no program projection observes the tail.
    store
        .num_values
        .insert(env.class_env.record_nil_val_id, EvalVal::Bool(false));
    boolean_list(env, eval(&[], body, &env.env, &mut store))
}

#[test]
fn entry_elaborates_and_registers_operations_and_proofs() {
    let env = loaded_env();
    for name in [
        "insert",
        "sort",
        "permutation",
        "insert::sorted",
        "sort::sorted",
        "insert::count",
        "insert::permutation",
        "sort::permutation",
    ] {
        assert!(
            env.globals.contains_key(name),
            "`{name}` must be a real kernel-checked global"
        );
    }
}

#[test]
fn entry_adds_no_trusted_declarations() {
    let mut env = base_env();
    let before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    env.elaborate_ken_md_file(INSERTION_SORT_KEN_MD)
        .expect("Algorithm/Sorting/InsertionSort.ken.md must elaborate");
    let after: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(
        before, after,
        "insertion sort must add zero trusted declarations"
    );
}

#[test]
fn boolean_vectors_compute_and_both_generic_laws_instantiate() {
    let mut env = loaded_env();
    env.elaborate_file(
        "theorem sort_bool_vector_sorted : \
           is_sorted Bool (ordered_leq Bool Ord_instance_Bool) \
             (sort Bool Ord_instance_Bool \
               (Cons Bool True (Cons Bool False (Cons Bool True (Nil Bool))))) = \
           sort::sorted Bool Ord_instance_Bool \
             (Cons Bool True (Cons Bool False (Cons Bool True (Nil Bool))))\n\
         theorem sort_bool_vector_permutation : \
           permutation Bool Ord_instance_Bool \
             (Cons Bool True (Cons Bool False (Cons Bool True (Nil Bool)))) \
             (sort Bool Ord_instance_Bool \
               (Cons Bool True (Cons Bool False (Cons Bool True (Nil Bool))))) = \
           sort::permutation Bool Ord_instance_Bool \
             (Cons Bool True (Cons Bool False (Cons Bool True (Nil Bool))))",
    )
    .expect("concrete behavior and both generic correctness laws must instantiate");

    let empty_id = env
        .elaborate_decl(
            "const cat_sort_bool_empty : List Bool = \
             sort Bool Ord_instance_Bool (Nil Bool)",
        )
        .expect("empty Boolean sort vector must elaborate");
    let sorted_id = env
        .elaborate_decl(
            "const cat_sort_bool_sorted : List Bool = \
             sort Bool Ord_instance_Bool \
               (Cons Bool False (Cons Bool True (Nil Bool)))",
        )
        .expect("already-sorted Boolean vector must elaborate");
    let duplicate_id = env
        .elaborate_decl(
            "const cat_sort_bool_vector : List Bool = \
             sort Bool Ord_instance_Bool \
               (Cons Bool True (Cons Bool False (Cons Bool True (Nil Bool))))",
        )
        .expect("concrete Boolean sort vector must elaborate");
    assert_eq!(evaluate_boolean_list(&env, empty_id), Vec::<bool>::new());
    assert_eq!(evaluate_boolean_list(&env, sorted_id), [false, true]);
    assert_eq!(
        evaluate_boolean_list(&env, duplicate_id),
        [false, true, true]
    );
}
