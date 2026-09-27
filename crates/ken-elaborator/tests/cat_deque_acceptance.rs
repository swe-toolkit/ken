//! CAT-DEQUE persistent two-list deque acceptance.
//!
//! Public names are normative compatibility vectors. The abstraction laws,
//! concrete end-order observations, and zero-trust delta are durable invariants.

use std::collections::BTreeSet;
use std::path::PathBuf;

#[path = "support/catalog_or.rs"]
mod catalog_or;

use ken_elaborator::{ElabEnv, ElabError};
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

fn loaded_env_with_owned() -> (ElabEnv, Vec<GlobalId>, Vec<GlobalId>) {
    let mut env = base_env();
    let derived_owned = env.elaborate_module_from_roots(&[catalog_root()], DERIVED)
        .expect("the Deque provider must roots-load before Deque");
    let deque_owned = env.elaborate_module_from_roots(&[catalog_root()], DEQUE)
        .expect("Data.Collections.Deque must roots-load with its real provider closure");
    (env, deque_owned, derived_owned)
}

fn checked_deque_examples() -> ElabEnv {
    let (mut env, _, _) = loaded_env_with_owned();
    env.execute_loaded_entry_checked_fences(DEQUE)
        .expect("Deque's private laws and concrete examples must check");
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

/// Promise class: durable checked-identity invariant.
/// MEASURED: forged flat aliases for every named Deque global cannot replace
/// the provider-owned IDs selected by the host. CLAIMED: host probes inspect checked Deque ownership,
/// not a mutable flat fixture binding. THE GAP: source access is separately
/// governed by the loader-visible closeout pin.
#[test]
fn entry_elaborates_and_registers_operations_and_laws() {
    let (mut env, owned, derived_owned) = loaded_env_with_owned();
    let forged = catalog_or::provider_owned_id(&env, &derived_owned, DERIVED, "reverse")
        .expect("Derived must own the checked reverse used as a forgery input");
    let names = [
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
    ];
    for name in names {
        env.globals.insert(name.to_owned(), forged);
    }
    for name in names {
        let qualified = format!("{DEQUE}.{name}");
        let id = if name == "MkDeque" {
            // Constructor IDs are inside the loader-owned inductive, not
            // separate results of elaborate_module_from_roots.
            let carrier = catalog_or::provider_owned_id(&env, &owned, DEQUE, "Deque")
                .expect("Deque must own its checked carrier");
            let candidate = env.globals[&qualified];
            assert!(
                matches!(env.env.lookup(carrier), Some(Decl::Inductive(decl))
                    if decl.constructors.iter().any(|ctor| ctor.id == candidate)),
                "`{qualified}` must be a constructor of the owned Deque carrier"
            );
            candidate
        } else {
            catalog_or::provider_owned_id(&env, &owned, DEQUE, name)
                .unwrap_or_else(|error| panic!("`{qualified}` must be Deque-owned: {error}"))
        };
        if name != "MkDeque" {
            assert!(env.env.lookup(id).is_some(), "`{qualified}` must be checked");
        }
        assert_ne!(id, forged, "flat alias must not spoof Deque.{name}");
    }
}

/// Promise class: durable checked-identity invariant.
/// MEASURED: a forged qualified key for an otherwise checked Deque function
/// fails ownership despite naming a valid transparent Derived declaration.
/// CLAIMED: the host read authenticates loader ownership, not just spelling.
/// THE GAP: the per-name host census above supplies the positive owner set.
#[test]
fn deque_host_read_rejects_forged_qualified_provider_alias() {
    let (mut env, owned, derived_owned) = loaded_env_with_owned();
    let canonical = catalog_or::provider_owned_id(&env, &owned, DEQUE, "pushFront")
        .expect("Deque must own pushFront");
    let forged = catalog_or::provider_owned_id(&env, &derived_owned, DERIVED, "reverse")
        .expect("Derived must own the checked reverse used as a forgery input");
    assert_ne!(canonical, forged);
    env.globals.insert(format!("{DEQUE}.pushFront"), forged);
    assert!(
        catalog_or::provider_owned_id(&env, &owned, DEQUE, "pushFront").is_err(),
        "qualified key must not grant Derived.reverse Deque ownership"
    );
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
/// import pair pin the elaboration-visible migration shape. The Deque-local
/// checked examples below pin concrete behavior. The WP census and affected-
/// target closure own the remaining frame obligations.
#[test]
fn transparent_deque_bodies_have_exact_derived_head_occurrence_populations() {
    let (env, _, derived_owned) = loaded_env_with_owned();
    let append = catalog_or::provider_owned_id(&env, &derived_owned, DERIVED, "list_append")
        .expect("Derived must own checked list_append");
    let reverse = catalog_or::provider_owned_id(&env, &derived_owned, DERIVED, "reverse")
        .expect("Derived must own checked reverse");

    for retired in ["deque_list_append", "deque_list_reverse"] {
        assert!(
            !env.globals.contains_key(&format!("{DEQUE}.{retired}")),
            "retired local reimplementation {DEQUE}.{retired} must be absent"
        );
    }
    assert_eq!(
        transparent_bodies_with_saturated_provider_head_occurrence(&env, reverse),
        BTreeSet::from([
            "deque_pop_back_nil_view".to_owned(),
            "deque_pop_front_nil_view".to_owned(),
            "popBack".to_owned(),
            "popBack_list_view".to_owned(),
            "popFront".to_owned(),
            "popFront_list_view".to_owned(),
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
            "deque_pop_back_nil_view".to_owned(),
            "deque_pop_front_nil_view".to_owned(),
            "popFront_list_view".to_owned(),
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

/// Promise class: durable checked-law invariant.
/// MEASURED: Deque's own example fence checks all four generic applications,
/// then the host finds four transparent witnesses. CLAIMED: the private
/// homomorphism and pop-inverse laws apply to arbitrary inputs without a
/// client importing private Deque names. THE GAP: the checked provider laws
/// establish the universal statements; this checks only their applications.
#[test]
fn both_homomorphisms_and_both_pop_inverses_instantiate_generically() {
    let env = checked_deque_examples();
    for name in [
        "deque_example_front_homomorphism",
        "deque_example_back_homomorphism",
        "deque_example_front_inverse",
        "deque_example_back_inverse",
    ] {
        let id = env.globals[name];
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "in-module generic Deque application {name} must remain checked"
        );
    }
}

/// Promise class: durable closed computation invariant.
/// MEASURED: Deque's example fence checks five exact List Bool equalities,
/// including direct and rebalancing pop orders, with transparent proof terms.
/// CLAIMED: these five histories preserve their specified sequence orders.
/// THE GAP: these are closed histories, not a universal invariant over every
/// deque; the generic laws above own their respective quantified statements.
#[test]
fn front_back_and_rebalancing_paths_preserve_sequence_order() {
    let env = checked_deque_examples();
    for name in [
        "deque_example_push_order",
        "deque_example_front_direct_order",
        "deque_example_front_rebalance_order",
        "deque_example_back_direct_order",
        "deque_example_back_rebalance_order",
    ] {
        let id = env.globals[name];
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "in-module concrete Deque order {name} must remain checked"
        );
    }
}
