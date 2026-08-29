//! CAT-DEQUE persistent two-list deque acceptance.
//!
//! Public names are normative compatibility vectors. The abstraction laws,
//! concrete end-order observations, and zero-trust delta are durable invariants.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::{ElabEnv, ElabError};
use ken_interp::eval::{eval, EvalStore, EvalVal};
use ken_kernel::{Decl, GlobalId, Term};

const DEQUE: &str = "Data.Collections.Deque";
const DERIVED: &str = "Data.Collections.Derived";

fn base_env() -> ElabEnv {
    ElabEnv::empty().expect("prelude bootstrap")
}

fn catalog_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn expose_module(env: &mut ElabEnv, module: &str) {
    let prefix = format!("{module}.");
    let aliases = env
        .globals
        .iter()
        .filter_map(|(name, id)| {
            name.strip_prefix(&prefix)
                .map(|suffix| (suffix.to_owned(), *id))
        })
        .collect::<Vec<_>>();
    env.globals.extend(aliases);
}

fn loaded_env() -> ElabEnv {
    let mut env = base_env();
    env.elaborate_module_from_roots(&[catalog_root()], DEQUE)
        .expect("Data.Collections.Deque must roots-load with its real provider closure");
    expose_module(&mut env, DEQUE);
    env
}

fn leading_pi_count(term: &Term) -> usize {
    let mut count = 0;
    let mut current = term;
    while let Term::Pi(_, body) = current {
        count += 1;
        current = body;
    }
    count
}

fn provider_arity(env: &ElabEnv, provider: GlobalId) -> usize {
    let ty = match env.env.lookup(provider) {
        Some(Decl::Transparent { ty, .. }) => ty,
        other => panic!("Derived provider must be transparent, got {other:?}"),
    };
    let arity = leading_pi_count(ty);
    assert!(arity > 0, "Derived provider must have a function type");
    arity
}

fn term_contains_saturated_provider_head_occurrence(
    term: &Term,
    provider: GlobalId,
    arity: usize,
) -> bool {
    if matches!(term, Term::App(_, _)) {
        let mut argument_count = 0;
        let mut head = term;
        while let Term::App(function, _) = head {
            argument_count += 1;
            head = function;
        }
        if argument_count >= arity && matches!(head, Term::Const { id, .. } if *id == provider) {
            return true;
        }
    }
    term.children()
        .into_iter()
        .any(|child| term_contains_saturated_provider_head_occurrence(child, provider, arity))
}

fn transparent_bodies_with_saturated_provider_head_occurrence(
    env: &ElabEnv,
    provider: GlobalId,
) -> BTreeSet<String> {
    let prefix = format!("{DEQUE}.");
    let arity = provider_arity(env, provider);
    env.globals
        .iter()
        .filter_map(|(qualified, id)| {
            let local = qualified.strip_prefix(&prefix)?;
            let (_, body) = env.env.transparent_body(*id)?;
            term_contains_saturated_provider_head_occurrence(&body, provider, arity)
                .then(|| local.to_owned())
        })
        .collect()
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
fn entry_adds_no_consumer_local_trusted_declarations() {
    let mut env = base_env();
    env.elaborate_module_from_roots(&[catalog_root()], DERIVED)
        .expect("the canonical Derived provider closure must roots-load");
    let before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    env.elaborate_module_from_roots(&[catalog_root()], DEQUE)
        .expect("Data.Collections.Deque must roots-load");
    let after: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(
        before, after,
        "Deque must add zero consumer-local trusted declarations beyond its provider closure"
    );
}

/// Promise class: durable invariant.
///
/// MEASURED: roots-loaded transparent `Data.Collections.Deque.*` bodies that
/// contain at least one saturated application-head occurrence of an exact
/// Derived provider identity form closed, literal expected populations.
///
/// LIMITATION: this is a syntactic occurrence-population pin. It does not prove
/// that an occurrence is evaluated, lies on every reachable route, reaches the
/// body's result, or excludes unrelated or local computation elsewhere in the
/// body.
///
/// EVIDENCE DIVISION: exact provider identity and occurrence population are
/// measured here. Retired named globals plus the positive/negative selective-
/// import pair pin the elaboration-visible migration shape. Concrete
/// observations below pin behavior. The WP census and affected-target closure
/// own the remaining frame obligations.
#[test]
fn transparent_deque_bodies_have_exact_derived_head_occurrence_populations() {
    let env = loaded_env();
    let append = env.globals[&format!("{DERIVED}.list_append")];
    let reverse = env.globals[&format!("{DERIVED}.reverse")];

    for retired in ["deque_list_append", "deque_list_reverse"] {
        assert!(
            !env.globals.contains_key(&format!("{DEQUE}.{retired}")),
            "retired local reimplementation {DEQUE}.{retired} must be absent"
        );
    }
    assert_eq!(
        transparent_bodies_with_saturated_provider_head_occurrence(&env, reverse),
        BTreeSet::from([
            "popBack".to_owned(),
            "popFront".to_owned(),
            "toList".to_owned(),
            "toList_pushBack".to_owned(),
            "toList_pushFront".to_owned(),
        ]),
        "transparent Deque bodies containing a saturated exact reverse-provider \
         application-head occurrence must match the closed expected population"
    );
    assert_eq!(
        transparent_bodies_with_saturated_provider_head_occurrence(&env, append),
        BTreeSet::from([
            "deque_append_snoc_assoc".to_owned(),
            "toList".to_owned(),
            "toList_pushBack".to_owned(),
            "toList_pushFront".to_owned(),
        ]),
        "transparent Deque bodies containing a saturated exact list_append-provider \
         application-head occurrence must match the closed expected population"
    );

    let mut imported = base_env();
    imported
        .elaborate_module_from_roots(&[catalog_root()], DERIVED)
        .expect("Derived provider must roots-load");
    imported
        .elaborate_file(
            "import Data.Collections.Derived (list_append)\n\
             fn cat_deque_selective_positive \
                 (xs : List Bool) (ys : List Bool) : List Bool = \
               list_append Bool xs ys",
        )
        .expect("the selectively imported list_append binding must resolve");

    let mut omitted = base_env();
    omitted
        .elaborate_module_from_roots(&[catalog_root()], DERIVED)
        .expect("Derived provider must roots-load");
    let error = omitted
        .elaborate_file(
            "import Data.Collections.Derived (list_append)\n\
             fn cat_deque_selective_negative (xs : List Bool) : List Bool = \
               reverse Bool xs",
        )
        .expect_err("available but unimported reverse must not resolve");
    assert!(
        matches!(error, ElabError::UnresolvedCon { ref name, .. } if name == "reverse"),
        "the non-import control must fail at the omitted binding, got {error:?}"
    );
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
               (Data.Collections.Derived.list_append a (toList a q) (Cons a x (Nil a))) = \
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
