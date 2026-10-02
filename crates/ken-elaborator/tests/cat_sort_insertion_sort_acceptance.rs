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
use ken_kernel::{Decl, GlobalId, Term};
const INSERTION_SORT_KEN_MD: &str =
    include_str!("../../../catalog/packages/Algorithm/Sorting/InsertionSort.ken.md");

fn base_env_with_lawful_owned() -> (ElabEnv, Vec<GlobalId>, Vec<GlobalId>) {
    let mut env = ElabEnv::empty().expect("prelude bootstrap");
    let transport_owned = catalog_or::load_core_logic_compare(&mut env);
    catalog_or::expose_core_logic_transport(&mut env, &transport_owned);
    let lawful_owned = env
        .elaborate_module_from_roots(&[catalog_or::catalog_root()], "Core.Classes.LawfulClasses")
        .expect("the class owner must roots-load in this environment");
    let (_, derived_owned) = catalog_or::load_derived_importing_fixture_many(&mut env, &[]);
    // The sequential harness has no module namespace. Hide Derived's generic
    // sort operation aliases while keeping their canonical qualified IDs, so
    // this package's names are inventoried as under the real module loader.
    for name in [
        "Perm",
        "insert",
        "sort",
        "insert::count",
        "insert::sorted",
        "sort::perm",
        "sort::sorted",
    ] {
        env.globals.remove(name);
    }
    (env, lawful_owned, derived_owned)
}

fn base_env() -> ElabEnv {
    base_env_with_lawful_owned().0
}

fn elaborate_insertion_sort(env: &mut ElabEnv) {
    let extracted = ken_elaborator::literate::extract_ken_md(INSERTION_SORT_KEN_MD)
        .expect("InsertionSort literate source must extract");
    env.elaborate_file(&extracted.source)
        .expect("Algorithm/Sorting/InsertionSort.ken.md must elaborate");
}

fn loaded_env_with_lawful_owned() -> (ElabEnv, Vec<GlobalId>) {
    let (mut env, lawful_owned, _) = base_env_with_lawful_owned();
    elaborate_insertion_sort(&mut env);
    (env, lawful_owned)
}

fn loaded_env() -> ElabEnv {
    loaded_env_with_lawful_owned().0
}

fn application_head_and_arguments(mut term: &Term) -> (&Term, Vec<&Term>) {
    let mut arguments = Vec::new();
    while let Term::App(function, argument) = term {
        arguments.push(argument.as_ref());
        term = function;
    }
    arguments.reverse();
    (term, arguments)
}

fn provider_application_head(body: &Term) -> &Term {
    application_head_and_arguments(body).0
}

fn boolean_decisions(term: &Term, bool_id: GlobalId) -> usize {
    usize::from(matches!(term, Term::Elim { fam, .. } if *fam == bool_id))
        + term
            .children()
            .into_iter()
            .map(|child| boolean_decisions(child, bool_id))
            .sum::<usize>()
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

fn evaluate_boolean_list(env: &ElabEnv, id: GlobalId) -> Vec<bool> {
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

fn nat_value(env: &ElabEnv, value: EvalVal) -> usize {
    match value {
        EvalVal::Ctor { id, args, .. } if id == env.prelude_env.zero_id && args.is_empty() => 0,
        EvalVal::Ctor { id, args, .. } if id == env.prelude_env.suc_id && args.len() == 1 => {
            1 + nat_value(env, args[0].clone())
        }
        other => panic!("expected a Nat constructor chain, got {other:?}"),
    }
}

fn evaluate_nat(env: &ElabEnv, id: GlobalId) -> usize {
    let body = match env.env.lookup(id) {
        Some(Decl::Transparent { body, .. }) => body,
        other => panic!("Nat vector must be transparent, got {other:?}"),
    };
    let mut store = EvalStore::new();
    store
        .num_values
        .insert(env.class_env.record_nil_val_id, EvalVal::Bool(false));
    nat_value(env, eval(&[], body, &env.env, &mut store))
}

/// MEASURED: the seven InsertionSort globals added to a flat fixture over
/// roots-loaded providers are exactly its public surface, and the eleven
/// retired private artifacts are absent. CLAIMED: the Ord package publishes
/// only its seven dictionary-specialized names. THE GAP: the fixture is flat,
/// not a standalone module closure; `ken check` on the real catalog path tests
/// its import boundary separately. Promise class: normative compatibility
/// vector for the seven public names and durable retirement invariant.
#[test]
fn entry_elaborates_with_exact_public_inventory() {
    let mut env = base_env();
    let before = env.globals.keys().cloned().collect::<BTreeSet<_>>();
    elaborate_insertion_sort(&mut env);
    let after = env.globals.keys().cloned().collect::<BTreeSet<_>>();
    let added = after.difference(&before).cloned().collect::<BTreeSet<_>>();
    let expected = BTreeSet::from([
        "sort".to_owned(),
        "sort::sorted".to_owned(),
        "sort::permutation".to_owned(),
        "permutation".to_owned(),
        "insert".to_owned(),
        "insert::sorted".to_owned(),
        "insert::permutation".to_owned(),
    ]);
    for retired in [
        "ordered_leq",
        "order_eq",
        "element_count",
        "sorted_cons",
        "head_ordered",
        "sorted_tail",
        "sorted_head",
        "leq_right_of_left_false",
        "head_ordered_after_insert",
        "count_cons_cong",
        "count_after_two",
        "count_swap_decisions",
        "count_cons_swap",
        "insert::count",
    ] {
        assert!(
            !env.globals.contains_key(retired),
            "retired local `{retired}` must remain absent"
        );
    }

    assert_eq!(
        added, expected,
        "the only client globals are the seven public wrappers"
    );
}

/// Promise class: durable provider-identity and no-local-decision invariants.
///
/// MEASURED: seven preloaded, owned provider identities head the seven checked
/// client bodies after their declared binders; no client body eliminates Bool.
/// CLAIMED: the consumer does not re-derive the generic sort computation or
/// proofs. THE GAP: a different indirect helper can itself cite Derived, but
/// the direct-head requirement rejects that indirection at the client boundary.
#[test]
fn entry_wrapper_bodies_directly_use_derived_without_boolean_decisions() {
    let (mut env, _, derived_owned) = base_env_with_lawful_owned();
    let providers = [
        ("sort", "sort", 3),
        ("sort::sorted", "sort::sorted", 3),
        ("sort::permutation", "sort::perm", 3),
        ("permutation", "Perm", 4),
        ("insert", "insert", 4),
        ("insert::sorted", "insert::sorted", 4),
        ("insert::permutation", "insert::count", 5),
    ]
    .map(|(client, provider, binders)| {
        let id = catalog_or::provider_owned_id(
            &env,
            &derived_owned,
            "Data.Collections.Derived",
            provider,
        )
        .unwrap_or_else(|error| panic!("sort provider {provider}: {error}"));
        (client, provider, binders, id)
    });
    elaborate_insertion_sort(&mut env);
    for (client, provider, binders, provider_id) in providers {
        let client_id = env.globals[client];
        assert_ne!(
            client_id, provider_id,
            "{client} must remain a distinct Ord wrapper"
        );
        let (_, body) = env
            .env
            .transparent_body(client_id)
            .unwrap_or_else(|| panic!("{client} must be a checked transparent wrapper"));
        let mut term = &body;
        for _ in 0..binders {
            let Term::Lam(_, inner) = term else {
                panic!("{client} must keep exactly {binders} declared binders: {term:?}");
            };
            term = inner;
        }
        assert!(
            matches!(provider_application_head(term), Term::Const { id, .. } if *id == provider_id),
            "{client} must directly call Derived.{provider} through its preloaded GlobalId; got {term:?}"
        );
        assert_eq!(
            boolean_decisions(&body, env.numeric_env.bool_id),
            0,
            "{client} must not make its own Boolean branching decision"
        );
    }
}

/// Promise class: durable invariant. Checked sort imports add no trust.
#[test]
fn entry_adds_no_trusted_declarations() {
    let mut env = base_env();
    let before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    elaborate_insertion_sort(&mut env);
    let after: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(
        before, after,
        "insertion sort must add zero trusted declarations"
    );
}

/// Promise class: durable behavior invariant. The owner-resolved Ord Bool
/// dictionary drives both checked laws and every concrete sort/count result.
#[test]
fn boolean_vectors_compute_and_both_generic_laws_instantiate() {
    let (mut env, lawful_owned) = loaded_env_with_lawful_owned();
    env.elaborate_file(
        "import Core.Classes.LawfulClasses (Ord, ord_leq_at)\n\
         import Data.Collections.Derived (count, eq_from_ord)\n\
         theorem cat_sort_bool_sorted_proof (d : Ord Bool) (xs : List Bool) : \
           is_sorted Bool (ord_leq_at Bool d) (sort Bool d xs) = \
           sort::sorted Bool d xs\n\
         theorem cat_sort_bool_perm_proof (d : Ord Bool) (xs : List Bool) : \
           permutation Bool d xs (sort Bool d xs) = \
           sort::permutation Bool d xs\n\
         fn cat_sort_bool_via_ord (xs : List Bool) : List Bool where Ord Bool = \
           let sorted = cat_sort_bool_sorted_proof d xs; \
               perm = cat_sort_bool_perm_proof d xs \
           in sort Bool d xs\n\
         fn cat_sort_bool_count (xs : List Bool) : Nat where Ord Bool = \
           count Bool (eq_from_ord Bool (ord_leq_at Bool d)) True xs",
    )
    .expect("public imports and `where Ord Bool` must resolve both laws and order vectors");

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
        .expect("the canonical Ord Bool dictionary must be registered");
    assert!(lawful_owned.contains(&bool_instance), "Ord Bool must be LawfulClasses-owned");
    let wrapper_id = env.globals["cat_sort_bool_via_ord"];
    let wrapper_references = catalog_or::declaration_references(
        env.env.lookup(wrapper_id).expect("the checked wrapper must exist"),
    );
    assert!(
        wrapper_references.contains(&bool_instance),
        "the checked sort/law wrapper must reach the owned Ord Bool dictionary"
    );
    for law in ["cat_sort_bool_sorted_proof", "cat_sort_bool_perm_proof"] {
        assert!(
            wrapper_references.contains(&env.globals[law]),
            "the concrete Ord Bool wrapper must instantiate its checked {law}"
        );
    }

    let empty_id = env
        .elaborate_decl(
            "const cat_sort_bool_empty : List Bool = \
             cat_sort_bool_via_ord (Nil Bool)",
        )
        .expect("empty Boolean sort vector must elaborate");
    let sorted_id = env
        .elaborate_decl(
            "const cat_sort_bool_sorted : List Bool = \
             cat_sort_bool_via_ord (Cons Bool False (Cons Bool True (Nil Bool)))",
        )
        .expect("already-sorted Boolean vector must elaborate");
    let duplicate_id = env
        .elaborate_decl(
            "const cat_sort_bool_vector : List Bool = \
             cat_sort_bool_via_ord (Cons Bool True (Cons Bool False (Cons Bool True (Nil Bool))))",
        )
        .expect("concrete Boolean sort vector must elaborate");
    let true_count_id = env
        .elaborate_decl(
            "const cat_sort_true_count : Nat = \
             cat_sort_bool_count (Cons Bool True (Cons Bool False (Cons Bool True (Nil Bool))))",
        )
        .expect("canonical count and order-derived equality must compute together");
    assert_eq!(evaluate_boolean_list(&env, empty_id), Vec::<bool>::new());
    assert_eq!(evaluate_boolean_list(&env, sorted_id), [false, true]);
    assert_eq!(evaluate_boolean_list(&env, duplicate_id), [false, true, true]);
    assert_eq!(evaluate_nat(&env, true_count_id), 2);
}
