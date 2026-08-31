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
use ken_kernel::{convert_type, subst::subst0, Context, Decl, GlobalId, Term};
const INSERTION_SORT_KEN_MD: &str =
    include_str!("../../../catalog/packages/Algorithm/Sorting/InsertionSort.ken.md");

fn base_env() -> ElabEnv {
    let mut env = ElabEnv::empty().expect("prelude bootstrap");
    catalog_or::load_core_logic_compare(&mut env);
    catalog_or::expose_core_logic_transport(&mut env);
    catalog_or::load_derived_fixture(&mut env);
    // The current sequential package harness has no module namespace. Hide the
    // earlier generic operations so this package can own its public `insert`
    // and `sort` names, as a real module import would.
    env.globals.remove("insert");
    env.globals.remove("sort");
    env
}

fn elaborate_insertion_sort(env: &mut ElabEnv) {
    let extracted = ken_elaborator::literate::extract_ken_md(INSERTION_SORT_KEN_MD)
        .expect("InsertionSort literate source must extract");
    let expected_imports = BTreeSet::from([
        "import Core.Classes.LawfulClasses (ord_leq_at)",
        "import Data.Collections.Derived (count, eq_from_ord)",
    ]);
    let mut removed_imports = BTreeSet::new();
    let source = extracted
        .source
        .lines()
        .filter(|line| {
            let trimmed = line.trim();
            if expected_imports.contains(trimmed) {
                removed_imports.insert(trimmed);
                false
            } else {
                true
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    assert_eq!(
        removed_imports, expected_imports,
        "the fixture must remove exactly the two declared provider imports"
    );
    env.elaborate_file(&source)
        .expect("Algorithm/Sorting/InsertionSort.ken.md must elaborate");
}

fn loaded_env() -> ElabEnv {
    let mut env = base_env();
    elaborate_insertion_sort(&mut env);
    env
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

fn declaration_type(declaration: &Decl) -> Option<&Term> {
    match declaration {
        Decl::Transparent { ty, .. } | Decl::Opaque { ty, .. } | Decl::Primitive { ty, .. } => {
            Some(ty)
        }
        Decl::Inductive(_) => None,
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
struct OrderingDecisionHeads {
    canonical_ordering: usize,
    direct_first_field: Vec<String>,
    canonical_equality: usize,
    variable: usize,
    constructor: usize,
    computed: usize,
    unclassified: Vec<String>,
}

fn collect_ordering_decision_heads(
    term: &Term,
    bool_id: GlobalId,
    ordering_provider: GlobalId,
    equality_provider: GlobalId,
    heads: &mut OrderingDecisionHeads,
) {
    if let Term::Let { val, body, .. } = term {
        // Follow the let-bound value to each use before classifying decisions.
        // An unused value is intentionally absent: computing and discarding a
        // comparison cannot pay for the branch decision that governs `insert`.
        let body_with_value = subst0(body, val);
        collect_ordering_decision_heads(
            &body_with_value,
            bool_id,
            ordering_provider,
            equality_provider,
            heads,
        );
        return;
    }

    if let Term::Elim { fam, scrut, .. } = term {
        if *fam == bool_id {
            let (head, arguments) = application_head_and_arguments(scrut);
            match head {
                Term::Const { id, .. } if *id == ordering_provider && !arguments.is_empty() => {
                    heads.canonical_ordering += 1;
                }
                Term::Proj1(record)
                    if !arguments.is_empty() && !matches!(record.as_ref(), Term::Proj2(_)) =>
                {
                    heads.direct_first_field.push(format!("{scrut:?}"));
                }
                Term::Const { id, .. } if *id == equality_provider && !arguments.is_empty() => {
                    heads.canonical_equality += 1;
                }
                Term::Var(_) => heads.variable += 1,
                Term::Constructor { .. } => heads.constructor += 1,
                Term::Elim { .. } => heads.computed += 1,
                _ => heads.unclassified.push(format!("{scrut:?}")),
            }
        }
    }
    for child in term.children() {
        collect_ordering_decision_heads(
            child,
            bool_id,
            ordering_provider,
            equality_provider,
            heads,
        );
    }
}

fn collect_provider_typed_calls(
    env: &ElabEnv,
    term: &Term,
    provider_type: &Term,
    providers: &mut Vec<GlobalId>,
) {
    let (head, arguments) = application_head_and_arguments(term);
    if !arguments.is_empty() {
        if let Term::Const { id, .. } = head {
            if let Some(candidate_type) = env.env.lookup(*id).and_then(declaration_type) {
                if convert_type(&env.env, &Context::new(), candidate_type, provider_type) {
                    providers.push(*id);
                }
            }
        } else {
            collect_provider_typed_calls(env, head, provider_type, providers);
        }
        for argument in arguments {
            collect_provider_typed_calls(env, argument, provider_type, providers);
        }
    } else {
        for child in term.children() {
            collect_provider_typed_calls(env, child, provider_type, providers);
        }
    }
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

/// Promise classes: durable provider-identity and exact-removal invariants;
/// transition sentinels for the candidate declaration and provider-call counts.
/// An authorized InsertionSort declaration/body change must rederive the counts
/// before retiring the red.
///
/// MEASURED: the fixture loads the real providers through catalog roots, then
/// elaborates the real consumer after removing exactly its two import lines.
/// Relative to that provider environment, the candidate adds exactly the base
/// declaration population minus the three retired locals. Across every added
/// transparent body, every applied global convertible to each imported
/// operation's type has that operation's exact qualified provider `GlobalId`.
/// The complete Boolean-elimination decision population is also classified per
/// declaration after zeta-substituting every let-bound value at its uses. Every
/// head is assigned to an explicit canonical-ordering, direct-projection,
/// canonical-equality, variable, constructor, or computed category; the
/// unclassified population must stay empty. An unused canonical comparison is
/// removed with its unused let, so it cannot pay for a different operative
/// decision. CLAIMED: no renamed, mixed, direct, hidden-let, unclassified, or
/// count-balanced bypass survives. THE GAP: the fixture does not prove raw
/// standalone success; the authorized raw boundary remains the separately
/// measured `bool_or` refusal.
#[test]
fn entry_elaborates_with_exact_inventory_and_canonical_providers() {
    let mut env = base_env();
    let before = env.globals.keys().cloned().collect::<BTreeSet<_>>();
    elaborate_insertion_sort(&mut env);
    let after = env.globals.keys().cloned().collect::<BTreeSet<_>>();
    let added = after.difference(&before).cloned().collect::<BTreeSet<_>>();
    let expected = BTreeSet::from([
        "count_after_two".to_owned(),
        "count_cons_cong".to_owned(),
        "count_cons_swap".to_owned(),
        "count_swap_decisions".to_owned(),
        "head_ordered".to_owned(),
        "head_ordered_after_insert".to_owned(),
        "insert".to_owned(),
        "insert::count".to_owned(),
        "insert::permutation".to_owned(),
        "insert::sorted".to_owned(),
        "leq_right_of_left_false".to_owned(),
        "permutation".to_owned(),
        "sort".to_owned(),
        "sort::permutation".to_owned(),
        "sort::sorted".to_owned(),
        "sorted_cons".to_owned(),
        "sorted_head".to_owned(),
        "sorted_tail".to_owned(),
    ]);
    for retired in ["ordered_leq", "order_eq", "element_count"] {
        assert!(
            !env.globals.contains_key(retired),
            "retired local `{retired}` must remain absent"
        );
    }

    let ord_leq_at = env.globals["Core.Classes.LawfulClasses.ord_leq_at"];
    let eq_from_ord = env.globals["Data.Collections.Derived.eq_from_ord"];
    let mut ordering_decisions = Vec::new();
    for name in &added {
        let id = env.globals[name];
        if let Some(Decl::Transparent { body, .. }) = env.env.lookup(id) {
            let mut heads = OrderingDecisionHeads::default();
            collect_ordering_decision_heads(
                body,
                env.numeric_env.bool_id,
                ord_leq_at,
                eq_from_ord,
                &mut heads,
            );
            if heads != OrderingDecisionHeads::default() {
                ordering_decisions.push((name.clone(), heads));
            }
        }
    }
    let expected_ordering_decisions = vec![
        (
            "count_after_two".to_owned(),
            OrderingDecisionHeads {
                variable: 3,
                ..OrderingDecisionHeads::default()
            },
        ),
        (
            "count_cons_cong".to_owned(),
            OrderingDecisionHeads {
                canonical_equality: 1,
                variable: 4,
                ..OrderingDecisionHeads::default()
            },
        ),
        (
            "count_swap_decisions".to_owned(),
            OrderingDecisionHeads {
                variable: 7,
                constructor: 4,
                ..OrderingDecisionHeads::default()
            },
        ),
        (
            "head_ordered_after_insert".to_owned(),
            OrderingDecisionHeads {
                canonical_ordering: 1,
                direct_first_field: vec!["((@10.1 @8) @6)".to_owned()],
                variable: 2,
                ..OrderingDecisionHeads::default()
            },
        ),
        (
            "insert".to_owned(),
            OrderingDecisionHeads {
                canonical_ordering: 1,
                ..OrderingDecisionHeads::default()
            },
        ),
        (
            "insert::count".to_owned(),
            OrderingDecisionHeads {
                canonical_ordering: 1,
                direct_first_field: vec![
                    "((@3.1 @0) @2)".to_owned(),
                    "((@8.1 @5) @7)".to_owned(),
                    "((@8.1 @7) @4)".to_owned(),
                ],
                canonical_equality: 1,
                variable: 2,
                computed: 2,
                ..OrderingDecisionHeads::default()
            },
        ),
        (
            "insert::sorted".to_owned(),
            OrderingDecisionHeads {
                canonical_ordering: 1,
                direct_first_field: vec!["((@8.1 @7) @5)".to_owned()],
                variable: 2,
                ..OrderingDecisionHeads::default()
            },
        ),
    ];
    assert_eq!(
        ordering_decisions, expected_ordering_decisions,
        "every Boolean decision site must retain its exact exhaustive classification after let provenance is substituted"
    );

    for (provider_name, expected_calls) in [
        ("Core.Classes.LawfulClasses.ord_leq_at", 136),
        ("Data.Collections.Derived.eq_from_ord", 60),
        ("Data.Collections.Derived.count", 50),
    ] {
        let provider = env.globals[provider_name];
        let provider_type = declaration_type(
            env.env
                .lookup(provider)
                .unwrap_or_else(|| panic!("missing provider `{provider_name}`")),
        )
        .unwrap_or_else(|| panic!("provider `{provider_name}` must have a global type"));
        let mut call_providers = Vec::new();
        for name in &added {
            let id = env.globals[name];
            if let Some(Decl::Transparent { body, .. }) = env.env.lookup(id) {
                collect_provider_typed_calls(&env, body, provider_type, &mut call_providers);
            }
        }
        assert!(
            call_providers.iter().all(|id| *id == provider),
            "every call convertible to `{provider_name}` must use its exact GlobalId; got {call_providers:?}"
        );
        assert_eq!(
            call_providers.len(),
            expected_calls,
            "provider `{provider_name}` must retain its complete elaborated call population"
        );
    }
    assert_eq!(
        added, expected,
        "candidate inventory must equal the base population minus ordered_leq, order_eq, and element_count"
    );
}

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

#[test]
fn boolean_vectors_compute_and_both_generic_laws_instantiate() {
    let mut env = loaded_env();
    env.elaborate_file(
        "theorem sort_bool_vector_sorted : \
           is_sorted Bool (ord_leq_at Bool Ord_instance_Bool) \
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
    let true_count_id = env
        .elaborate_decl(
            "const cat_sort_true_count : Nat = \
             count Bool (eq_from_ord Bool (ord_leq_at Bool Ord_instance_Bool)) True \
               (Cons Bool True (Cons Bool False (Cons Bool True (Nil Bool))))",
        )
        .expect("canonical count and order-derived equality must compute together");
    assert_eq!(evaluate_boolean_list(&env, empty_id), Vec::<bool>::new());
    assert_eq!(evaluate_boolean_list(&env, sorted_id), [false, true]);
    assert_eq!(
        evaluate_boolean_list(&env, duplicate_id),
        [false, true, true]
    );
    assert_eq!(evaluate_nat(&env, true_count_id), 2);
}
