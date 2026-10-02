//! LANG-MOD-CATALOG-COMPLETENESS partial provider-identity acceptance.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::ElabEnv;
use ken_kernel::{Decl, GlobalId, Term};

#[path = "support/catalog_or.rs"]
mod catalog_or;

const ORD_RESULT_MODULE: &str = "Core.Logic.OrdResult";
const COMPARE_MODULE: &str = "Core.Logic.Compare";
const DERIVED_MODULE: &str = "Data.Collections.Derived";
const LAWFUL_MODULE: &str = "Core.Classes.LawfulClasses";
const ORDER_MODULE: &str = "Data.Numeric.Nat.Order";

fn catalog_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn mentions_global(term: &Term, target: GlobalId) -> bool {
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
            .any(|child| mentions_global(child, target)),
    }
}

fn trusted_delta_qualified_names(
    env: &ElabEnv,
    before: &BTreeSet<GlobalId>,
    after: &BTreeSet<GlobalId>,
) -> BTreeSet<String> {
    after
        .difference(before)
        .map(|id| match env.env.lookup(*id) {
            Some(Decl::Opaque { name, .. }) => name.clone(),
            other => panic!("new trusted entry {id:?} must be named and opaque, got {other:?}"),
        })
        .collect()
}

fn named_axiom_trust_id(env: &ElabEnv, named: GlobalId) -> GlobalId {
    let (_, body) = env
        .env
        .transparent_body(named)
        .expect("named axiom must be transparent");
    let opaque = match body {
        Term::Const { id, .. } => id,
        other => panic!("named axiom must directly cite one opaque: {other:?}"),
    };
    assert_eq!(
        body,
        Term::const_(opaque, vec![]),
        "no wrapper or level arguments"
    );
    assert!(matches!(env.env.lookup(opaque), Some(Decl::Opaque { .. })));
    opaque
}

fn owned_ord_int_law_ids(env: &ElabEnv, lawful_owned: &[GlobalId]) -> BTreeSet<GlobalId> {
    let ord = env
        .class_env
        .class("Ord")
        .expect("registered Ord class")
        .projection
        .type_id;
    assert!(
        lawful_owned.contains(&ord),
        "the Ord class must belong to LawfulClasses"
    );
    let class = env.class_env.class_by_id(ord).expect("owned Ord class");
    assert_eq!(
        class.projection.field_names,
        ["leq", "refl", "antisym", "trans", "total"]
    );
    let ord_int_type = Term::app(
        Term::const_(ord, vec![]),
        Term::const_(env.numeric_env.int_id, vec![]),
    );
    let instances: Vec<_> = lawful_owned
        .iter()
        .copied()
        .filter(|id| matches!(env.env.lookup(*id), Some(Decl::Transparent { ty, .. }) if *ty == ord_int_type))
        .collect();
    assert_eq!(
        instances.len(),
        1,
        "one owned Ord Int dictionary by checked class/head IDs"
    );
    let (_, body) = env
        .env
        .transparent_body(instances[0])
        .expect("owned Ord Int record");
    let mut laws = BTreeSet::new();
    for (idx, name) in [
        "ord_int_refl",
        "ord_int_antisym",
        "ord_int_trans",
        "ord_int_total",
    ]
    .into_iter()
    .enumerate()
    {
        let mut field = &body;
        for _ in 0..idx + 1 {
            field = match field {
                Term::Pair(_, tail) => tail,
                other => panic!("Ord Int record must contain law {name}: {other:?}"),
            };
        }
        let field_id = match field {
            Term::Pair(value, _) => match value.as_ref() {
                Term::Const { id, .. } => *id,
                other => panic!("Ord Int law {name} must be a named constant: {other:?}"),
            },
            other => panic!("Ord Int law {name} must have a record field: {other:?}"),
        };
        let named = catalog_or::provider_owned_id(env, lawful_owned, LAWFUL_MODULE, name)
            .unwrap_or_else(|error| panic!("LawfulClasses axiom {name}: {error}"));
        assert_eq!(
            field_id, named,
            "Ord Int law {name} must use its owned axiom"
        );
        let opaque = named_axiom_trust_id(env, named);
        let aliases: BTreeSet<_> = lawful_owned
            .iter()
            .copied()
            .filter(|id| {
                env.env
                    .transparent_body(*id)
                    .is_some_and(|(_, body)| body == Term::const_(opaque, vec![]))
            })
            .collect();
        assert_eq!(
            aliases,
            BTreeSet::from([named]),
            "one owned name per Ord Int axiom"
        );
        assert!(
            laws.insert(opaque),
            "the four Ord Int trust IDs must be distinct"
        );
    }
    assert_eq!(laws.len(), 4, "exactly four Ord Int law identities");
    laws
}

/// Promise class: durable invariant.
///
/// MEASURED: the real strict roots loader produces one transparent canonical
/// constructor family and aliases that point at its exact constructor IDs.
/// CLAIMED: OrdResult publication preserves identity and adds no trust. THE GAP:
/// this test does not claim that every catalog consumer is strict-ready.
#[test]
fn canonical_ord_result_is_strict_standalone_and_exports_one_identity() {
    let mut env = ElabEnv::new().expect("base environment");
    let before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    env.elaborate_module_from_roots_strict(&[catalog_root()], ORD_RESULT_MODULE)
        .expect("canonical OrdResult must load standalone under strict resolution");

    let ty = env.globals["Core.Logic.OrdResult.OrdResult"];
    let lt = env.globals["Core.Logic.OrdResult.Lt"];
    let eq = env.globals["Core.Logic.OrdResult.Eq"];
    let gt = env.globals["Core.Logic.OrdResult.Gt"];
    assert!(matches!(env.env.lookup(ty), Some(Decl::Inductive { .. })));

    for (alias, constructor) in [("ord_eq", eq), ("ord_lt", lt), ("ord_gt", gt)] {
        let alias = env.globals[&format!("Core.Logic.OrdResult.{alias}")];
        let (_, body) = env
            .env
            .transparent_body(alias)
            .expect("OrdResult alias must remain transparent");
        assert!(
            matches!(body, Term::Constructor { id, .. } if id == constructor),
            "alias must reuse its canonical constructor identity"
        );
    }

    let after: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(before, after, "OrdResult must add zero trusted authority");
}

/// Promise class: durable invariant.
///
/// MEASURED: the real Derived roots load reaches the canonical OrdResult and
/// Compare globals, has no competing Derived identities, and reloads providers
/// idempotently. CLAIMED: the consumer closure reuses its providers. THE GAP:
/// later catalog leaves may still have unrelated unresolved dependencies.
#[test]
fn real_derived_consumer_reuses_canonical_logic_providers() {
    let mut env = ElabEnv::new().expect("base environment");
    let before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    env.elaborate_module_from_roots(&[catalog_root()], DERIVED_MODULE)
        .expect("real Derived consumer must legacy-roots-load through OrdResult");

    let canonical = env.globals["Core.Logic.OrdResult.OrdResult"];
    assert!(matches!(
        env.env.lookup(canonical),
        Some(Decl::Inductive { .. })
    ));
    assert!(env
        .globals
        .contains_key("Core.Logic.OrdResult.ord_result_leq"));
    assert!(!env
        .globals
        .contains_key("Data.Collections.Derived.ord_result_leq"));
    assert!(!env
        .globals
        .contains_key("Data.Collections.Derived.OrdResult"));

    let order_local = env.globals["Data.Numeric.Nat.Order.OrdResult"];
    assert!(matches!(
        env.env.lookup(order_local),
        Some(Decl::Inductive { .. })
    ));
    assert_ne!(
        order_local, canonical,
        "Order's package-local comparison codomain must remain distinct"
    );

    let mut derived_compare_bodies = Vec::new();
    for name in ["compare_char", "compare"] {
        let id = env.globals[&format!("Data.Collections.Derived.{name}")];
        let (ty, body) = match env.env.lookup(id) {
            Some(Decl::Transparent { ty, body, .. }) => (ty, body),
            other => panic!("Derived {name} must be transparent, got {other:?}"),
        };
        assert!(
            mentions_global(ty, canonical),
            "Derived {name} must return canonical Core.Logic OrdResult"
        );
        assert!(
            !mentions_global(ty, order_local),
            "Derived {name} must not return Order's package-local OrdResult"
        );
        assert!(
            !mentions_global(body, order_local),
            "Derived {name} body must exclude Order's package-local OrdResult"
        );
        derived_compare_bodies.push((name, id, body));
    }

    let compare_char_body = derived_compare_bodies[0].2;
    for provider in ["ord_eq", "ord_lt", "ord_gt"] {
        let provider_id = env.globals[&format!("Core.Logic.OrdResult.{provider}")];
        assert!(
            mentions_global(compare_char_body, provider_id),
            "Derived compare_char must use canonical {provider}"
        );
    }

    let compare_body = derived_compare_bodies[1].2;
    for (provider, provider_id) in [
        (
            "Core.Logic.Compare.list_compare",
            env.globals["Core.Logic.Compare.list_compare"],
        ),
        (
            "Data.Collections.Derived.compare_char",
            derived_compare_bodies[0].1,
        ),
    ] {
        assert!(
            mentions_global(compare_body, provider_id),
            "Derived compare must use exact provider {provider}"
        );
    }

    for name in [
        "pair_compare",
        "pair_compare_result_of",
        "pair_compare_lt_cases",
        "list_compare",
        "list_eq",
    ] {
        assert!(
            env.globals
                .contains_key(&format!("Core.Logic.Compare.{name}")),
            "canonical Compare provider must own {name}"
        );
        assert!(
            !env.globals
                .contains_key(&format!("Data.Collections.Derived.{name}")),
            "Derived must not retain a competing {name} identity"
        );
    }

    let loaded_before = env.loaded_module_count();
    env.elaborate_module_from_roots(&[catalog_root()], ORD_RESULT_MODULE)
        .expect("reloading OrdResult must be idempotent");
    env.elaborate_module_from_roots(&[catalog_root()], COMPARE_MODULE)
        .expect("reloading Compare must be idempotent");
    assert_eq!(env.loaded_module_count(), loaded_before);

    let after: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    let derived_delta = trusted_delta_qualified_names(&env, &before, &after);

    let mut canonical_order = ElabEnv::new().expect("canonical Order trust baseline");
    let order_before: BTreeSet<_> = canonical_order.env.trusted_base().into_iter().collect();
    canonical_order
        .elaborate_module_from_roots(&[catalog_root()], ORDER_MODULE)
        .expect("canonical Order must roots-load independently");
    let order_after: BTreeSet<_> = canonical_order.env.trusted_base().into_iter().collect();
    let order_delta = trusted_delta_qualified_names(&canonical_order, &order_before, &order_after);

    assert_eq!(
        derived_delta, order_delta,
        "the real consumer closure must inherit exactly canonical Order's qualified-name trust delta"
    );
}

/// Promise class: durable export identity invariant.
///
/// MEASURED: seven checked clients selectively import the generic sort
/// surface, and each body cites the owned transparent Derived GlobalId.
/// CLAIMED: publishing the operations and proofs preserves their provider
/// identities. THE GAP: the separate sort-law suite checks full theorem types.
#[test]
fn derived_generic_sort_exports_are_roots_owned_and_selectively_importable() {
    let mut env = ElabEnv::new().expect("base environment");
    let owned = env
        .elaborate_module_from_roots(&[catalog_root()], DERIVED_MODULE)
        .expect("Derived provider closure must roots-load");
    env.elaborate_file(
        r#"import Data.Collections.Derived (Perm, insert, sort, count)
import Core.Classes.LawfulClasses (bool_or)
fn cat_generic_perm (a : Type) (eqf : a → a → Bool) (xs : List a) (ys : List a) : Prop =
  Perm a eqf xs ys
fn cat_generic_insert (a : Type) (le : a → a → Bool) (x : a) (xs : List a) : List a =
  insert a le x xs
fn cat_generic_sort (a : Type) (le : a → a → Bool) (xs : List a) : List a =
  sort a le xs
theorem cat_generic_insert_sorted
    (a : Type) (le : a → a → Bool)
    (total : (x : a) → (y : a) → IsTrue (bool_or (le x y) (le y x)))
    (x : a) (xs : List a) :
    is_sorted a le xs → is_sorted a le (insert a le x xs) =
  insert::sorted a le total x xs
theorem cat_generic_insert_count
    (a : Type) (le : a → a → Bool) (x : a) (xs : List a)
    (eqf : a → a → Bool) (q : a) :
    Equal Nat (count a eqf q (Cons a x xs)) (count a eqf q (insert a le x xs)) =
  insert::count a le x xs eqf q
theorem cat_generic_sort_sorted
    (a : Type) (le : a → a → Bool)
    (total : (x : a) → (y : a) → IsTrue (bool_or (le x y) (le y x)))
    (xs : List a) : is_sorted a le (sort a le xs) =
  sort::sorted a le total xs
theorem cat_generic_sort_perm
    (a : Type) (le : a → a → Bool) (xs : List a) (eqf : a → a → Bool) :
    Perm a eqf xs (sort a le xs) =
  sort::perm a le xs eqf"#,
    )
    .expect("generic sort operations and all four proofs must be importable");

    // A flat prelude alias may share a spelling with a newly imported symbol;
    // the checked clients, not that mutable table, establish resolution.
    for (provider, client) in [
        ("Perm", "cat_generic_perm"),
        ("insert", "cat_generic_insert"),
        ("sort", "cat_generic_sort"),
        ("insert::sorted", "cat_generic_insert_sorted"),
        ("insert::count", "cat_generic_insert_count"),
        ("sort::sorted", "cat_generic_sort_sorted"),
        ("sort::perm", "cat_generic_sort_perm"),
    ] {
        let canonical = catalog_or::provider_owned_id(&env, &owned, DERIVED_MODULE, provider)
            .unwrap_or_else(|error| panic!("Derived sort owner {provider}: {error}"));
        let client_id = env.globals[client];
        let (_, body) = env
            .env
            .transparent_body(client_id)
            .unwrap_or_else(|| panic!("{client} must have a checked transparent body"));
        assert!(
            mentions_global(&body, canonical),
            "{client} must cite Derived.{provider} by its provider GlobalId"
        );
    }
}

/// Promise class: durable invariant.
///
/// MEASURED: real LawfulClasses terms use canonical comparison and its two local
/// ordinary proof globals, while all foreign attached spellings are absent.
/// CLAIMED: attachment ownership stays with the defining module and adds no
/// proof trust. THE GAP: Order remains held on the separate Nat provider.
#[test]
fn lawful_local_pair_proofs_do_not_extend_the_derived_subject_namespace() {
    let mut env = ElabEnv::new().expect("base environment");
    let before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    let lawful_owned = env
        .elaborate_module_from_roots(&[catalog_root()], LAWFUL_MODULE)
        .expect("LawfulClasses must load through its real provider closure");

    let eq_sound = env.globals["Core.Classes.LawfulClasses.pair_compare_eq_sound"];
    let lt_asym = env.globals["Core.Classes.LawfulClasses.pair_compare_lt_asym"];
    let antisym = env.globals["Core.Classes.LawfulClasses.pair_ord_leq::antisym"];
    let (_, body) = env
        .env
        .transparent_body(antisym)
        .expect("pair_ord_leq::antisym must be a checked transparent proof");
    assert!(mentions_global(&body, eq_sound));
    assert!(mentions_global(&body, lt_asym));

    let canonical_pair_compare = env.globals["Core.Logic.Compare.pair_compare"];
    assert!(env
        .globals
        .contains_key("Core.Logic.Compare.pair_compare::eq"));
    assert!(env
        .globals
        .contains_key("Core.Logic.Compare.pair_compare::eq_cases"));
    let pair_ord_leq = env.globals["Core.Classes.LawfulClasses.pair_ord_leq"];
    let (_, body) = env
        .env
        .transparent_body(pair_ord_leq)
        .expect("pair_ord_leq must remain transparent");
    assert!(mentions_global(&body, canonical_pair_compare));

    for forbidden in [
        "Data.Collections.Derived.pair_compare::eq",
        "Data.Collections.Derived.pair_compare::eq_cases",
        "Data.Collections.Derived.pair_compare::eq_sound",
        "Data.Collections.Derived.pair_compare::lt_asym",
        "Core.Logic.Compare.pair_compare::eq_sound",
        "Core.Logic.Compare.pair_compare::lt_asym",
    ] {
        assert!(
            !env.globals.contains_key(forbidden),
            "consumer-local proof must not extend the provider namespace"
        );
    }

    let after: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert!(!after.contains(&eq_sound));
    assert!(!after.contains(&lt_asym));
    let bijection_owned = env
        .elaborate_module_from_roots(&[catalog_root()], "Data.Text.StringBijection")
        .expect("the existing retraction provider must remain loaded");
    let retraction = catalog_or::provider_owned_id(
        &env,
        &bijection_owned,
        "Data.Text.StringBijection",
        "string_to_list_char_retraction",
    )
    .expect("the retraction must retain its own provider identity");
    let mut expected = owned_ord_int_law_ids(&env, &lawful_owned);
    assert!(
        expected.insert(named_axiom_trust_id(&env, retraction)),
        "the retraction is separate from Ord Int"
    );
    assert_eq!(expected.len(), 5, "four Ord Int axioms plus the retraction");
    let added: BTreeSet<_> = after.difference(&before).copied().collect();
    assert_eq!(
        added, expected,
        "the full trusted-base delta must be the exact owned ID set"
    );
}
