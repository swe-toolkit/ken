//! CAT-DEQUE persistent two-list deque acceptance.
//!
//! Public names are normative compatibility vectors. The abstraction laws,
//! concrete end-order observations, and zero-trust delta are durable invariants.

use std::collections::BTreeSet;

use ken_elaborator::ElabEnv;
use ken_interp::eval::{eval, EvalStore, EvalVal};
use ken_kernel::Decl;

const DEQUE_KEN_MD: &str = include_str!("../../../catalog/packages/Data/Collections/Deque.ken.md");

fn base_env() -> ElabEnv {
    ElabEnv::empty().expect("prelude bootstrap")
}

fn loaded_env() -> ElabEnv {
    let mut env = base_env();
    env.elaborate_ken_md_file(DEQUE_KEN_MD)
        .expect("Data/Collections/Deque.ken.md must elaborate");
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

fn evaluate_boolean_list(env: &ElabEnv, name: &str) -> Vec<bool> {
    let body = match env.env.lookup(env.globals[name]) {
        Some(Decl::Transparent { body, .. }) => body,
        other => panic!("{name} must be transparent, got {other:?}"),
    };
    boolean_list(env, eval(&[], body, &env.env, &mut EvalStore::new()))
}

#[test]
fn entry_elaborates_and_registers_operations_and_laws() {
    let env = loaded_env();
    for name in [
        "Deque",
        "MkDeque",
        "empty",
        "pushFront",
        "pushBack",
        "popFront",
        "popBack",
        "toList",
        "toList_pushFront",
        "toList_pushBack",
        "PopPreserves",
        "popFront_pushFront",
        "popBack_pushBack",
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
    env.elaborate_ken_md_file(DEQUE_KEN_MD)
        .expect("Data/Collections/Deque.ken.md must elaborate");
    let after: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(before, after, "deque must add zero trusted declarations");
}

#[test]
fn both_homomorphisms_and_both_pop_inverses_instantiate_generically() {
    let mut env = loaded_env();
    env.elaborate_file(
        "theorem cat_deque_ac1_front \
             (a : Type) (x : a) (q : Deque a) \
           : Equal (List a) \
               (toList a (pushFront a x q)) \
               (Cons a x (toList a q)) = \
           toList_pushFront a x q\n\
         theorem cat_deque_ac1_back \
             (a : Type) (x : a) (q : Deque a) \
           : Equal (List a) \
               (toList a (pushBack a x q)) \
               (deque_list_append a (toList a q) (Cons a x (Nil a))) = \
           toList_pushBack a x q\n\
         fn cat_deque_ac2_front \
             (a : Type) (x : a) (q : Deque a) \
           : PopPreserves a x q (popFront a (pushFront a x q)) = \
           popFront_pushFront a x q\n\
         fn cat_deque_ac2_back \
             (a : Type) (x : a) (q : Deque a) \
           : PopPreserves a x q (popBack a (pushBack a x q)) = \
           popBack_pushBack a x q",
    )
    .expect("both abstraction homomorphisms and both pop inverses must instantiate generically");
}

#[test]
fn front_back_and_rebalancing_paths_preserve_sequence_order() {
    let mut env = loaded_env();
    env.elaborate_file(
        "fn cat_deque_observe_pop \
             (a : Type) (popped : Option (Pair a (Deque a))) \
           : List a = \
           match popped { \
             None ↦ Nil a; \
             Some item ↦ \
               Cons a \
                 (pair_fst a (Deque a) item) \
                 (toList a (pair_snd a (Deque a) item)) \
           }\n\
         const cat_deque_push_vector : List Bool = \
           toList Bool \
             (pushBack Bool False \
               (pushFront Bool False \
                 (pushBack Bool True (empty Bool))))\n\
         const cat_deque_pop_front_direct : List Bool = \
           cat_deque_observe_pop Bool \
             (popFront Bool \
               (pushFront Bool False \
                 (pushBack Bool True (empty Bool))))\n\
         const cat_deque_pop_front_rebalance : List Bool = \
           cat_deque_observe_pop Bool \
             (popFront Bool \
               (pushBack Bool False \
                 (pushBack Bool True (empty Bool))))\n\
         const cat_deque_pop_back_direct : List Bool = \
           cat_deque_observe_pop Bool \
             (popBack Bool \
               (pushBack Bool False \
                 (pushFront Bool True (empty Bool))))\n\
         const cat_deque_pop_back_rebalance : List Bool = \
           cat_deque_observe_pop Bool \
             (popBack Bool \
               (pushFront Bool False \
                 (pushFront Bool True (empty Bool))))",
    )
    .expect("both direct and rebalancing pop paths must elaborate");

    for (name, expected) in [
        ("cat_deque_push_vector", vec![false, true, false]),
        ("cat_deque_pop_front_direct", vec![false, true]),
        ("cat_deque_pop_front_rebalance", vec![true, false]),
        ("cat_deque_pop_back_direct", vec![false, true]),
        ("cat_deque_pop_back_rebalance", vec![true, false]),
    ] {
        assert_eq!(evaluate_boolean_list(&env, name), expected, "{name}");
    }
}
