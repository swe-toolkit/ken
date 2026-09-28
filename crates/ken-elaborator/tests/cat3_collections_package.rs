//! CAT-3 acceptance for the structural collection-law slice.
//!
//! This file checks the real package source, not hand-copied snippets. The D1
//! surface is deliberately bounded to structural list ops plus proof-returning
//! `take`/`drop`, `map` length, and `take` length/min laws.
//! D2 adds the verified `List Bool` insertion-sort/count-permutation slice.

#[path = "support/catalog_or.rs"]
mod catalog_or;

use std::collections::{BTreeMap, BTreeSet};

use ken_elaborator::{foreign::trusted_base_delta, ElabEnv, ElabError, NumericLitVal};
use ken_interp::eval::{eval, EvalStore, EvalVal, ListCharIds};
use ken_kernel::{convert, convert_type, Context, Decl, GlobalId, Term};

const COLLECTIONS_KEN_MD: &str =
    include_str!("../../../catalog/packages/Data/Collections/Derived.ken.md");

fn term_reference_count(term: &Term, target: GlobalId) -> usize {
    let here = usize::from(matches!(term, Term::Const { id, .. } if *id == target));
    here + term
        .children()
        .into_iter()
        .map(|child| term_reference_count(child, target))
        .sum::<usize>()
}

fn module_transparent_kernel_equivalents(
    env: &ElabEnv,
    module: &str,
    provider: GlobalId,
) -> BTreeSet<String> {
    let (provider_level_params, provider_ty, provider_body) = match env.env.lookup(provider) {
        Some(Decl::Transparent {
            level_params,
            ty,
            body,
            ..
        }) => (level_params, ty, body),
        other => panic!("provider must be transparent, got {other:?}"),
    };
    assert!(
        provider_level_params.is_empty(),
        "the D1 Boolean providers must be monomorphic"
    );

    let prefix = format!("{module}.");
    let context = Context::new();
    env.globals
        .iter()
        .filter_map(|(name, id)| {
            let local_name = name.strip_prefix(&prefix)?;
            let (level_params, ty, body) = match env.env.lookup(*id) {
                Some(Decl::Transparent {
                    level_params,
                    ty,
                    body,
                    ..
                }) => (level_params, ty, body),
                _ => return None,
            };
            if !level_params.is_empty() {
                return None;
            }
            (convert_type(&env.env, &context, ty, provider_ty)
                && convert(&env.env, &context, provider_ty, body, provider_body))
            .then(|| local_name.to_owned())
        })
        .collect()
}

fn lit_to_eval(value: &NumericLitVal, mkdecimalpair_id: GlobalId) -> EvalVal {
    match value {
        NumericLitVal::Int(n) => EvalVal::from(n.clone()),
        NumericLitVal::Float(f) => EvalVal::Float(*f),
        NumericLitVal::Float32(f) => EvalVal::Float32(*f),
        NumericLitVal::Decimal { coeff, exp } => {
            ken_interp::decimal_value(mkdecimalpair_id, coeff.clone(), *exp)
        }
        NumericLitVal::Str(s) => EvalVal::Str(s.clone()),
        NumericLitVal::Bytes(b) => EvalVal::Bytes(b.clone()),
    }
}

fn make_store(env: &ElabEnv) -> EvalStore {
    let mut store = EvalStore::new();
    let mkdecimalpair_id = env.prelude_env.mkdecimalpair_id;
    for (id, value) in &env.num_values {
        store
            .num_values
            .insert(*id, lit_to_eval(value, mkdecimalpair_id));
    }
    store.list_char_ids = Some(ListCharIds {
        nil_id: env.prelude_env.nil_id,
        cons_id: env.prelude_env.cons_id,
    });
    store
}

fn eval_transparent(env: &ElabEnv, store: &mut EvalStore, id: GlobalId) -> EvalVal {
    match env.env.lookup(id) {
        Some(Decl::Transparent { body, .. }) => eval(&[], body, &env.env, store),
        other => panic!("evaluation witness must be transparent, got {other:?}"),
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

fn mk_env_with_derived_owned() -> (ElabEnv, Vec<GlobalId>) {
    let mut env = ElabEnv::new().expect("base env");
    let (transport_owned, or_owned) =
        catalog_or::load_core_logic_compare_with_or_owned(&mut env);
    let provider_state = catalog_or::core_logic_or_module_state(&env);
    catalog_or::expose_core_logic_transport(&mut env, &transport_owned);
    catalog_or::restore_core_logic_or_module_state(&mut env, &provider_state);
    let owned = catalog_or::load_derived_fixture(&mut env);
    catalog_or::assert_transparent_result_uses_core_logic_or(
        &env, &or_owned, "pair_compare_lt_cases",
    );
    (env, owned)
}

fn mk_env() -> ElabEnv {
    mk_env_with_derived_owned().0
}

const DERIVED: &str = "Data.Collections.Derived";

// MEASURED: roots loading returns Derived's provider-owned IDs without
// installing the test-only flat-alias fixture.
// CLAIMED: the owner, rather than the fixture, supplies these declarations.
// THE GAP: the returned ID set alone does not establish any client's scope;
// individual example checks and the no-op-door runs test those routes.
fn derived_cat3_owner() -> (ElabEnv, Vec<GlobalId>) {
    let mut env = ElabEnv::new().expect("base environment");
    let owned = env
        .elaborate_module_from_roots(&[catalog_or::catalog_root()], DERIVED)
        .expect("the real Derived provider and dependency closure must roots-load");
    (env, owned)
}

// MEASURED: the qualified name resolves to an ID in this loader's owned set.
// CLAIMED: host reads observe the checked Derived declaration, not a flat alias.
// THE GAP: ID ownership does not prove the client's Ken source used this ID;
// assert_cat3_example_reference checks that separately on the checked term.
fn derived_cat3_id(env: &ElabEnv, owned: &[GlobalId], name: &str) -> GlobalId {
    catalog_or::provider_owned_id(env, owned, DERIVED, name)
        .unwrap_or_else(|error| panic!("Derived owner identity {name}: {error}"))
}

// MEASURED: every currently qualified Derived binding maps to an owned ID.
// CLAIMED: executing examples does not rebind a provider-qualified identity.
// THE GAP: the before/after comparison below guards that interval only;
// these qualified bindings do not inventory unqualified client scope.
fn derived_cat3_bindings(env: &ElabEnv, owned: &[GlobalId]) -> BTreeMap<String, GlobalId> {
    let prefix = format!("{DERIVED}.");
    env.globals
        .iter()
        .filter(|(name, _)| name.starts_with(&prefix))
        .map(|(name, id)| {
            assert!(owned.contains(id), "{name} must be loader-owned by Derived");
            (name.clone(), *id)
        })
        .collect()
}

fn cat3_owner_examples(examples: &[&str]) -> (ElabEnv, Vec<GlobalId>) {
    let (mut env, owned) = derived_cat3_owner();
    // MEASURED: each requested example is absent before fence execution, then
    // present by ID as a transparent, non-provider declaration afterwards.
    // CLAIMED: these witnesses are checked owner-local examples, not tangled
    // exports, declarations in the provider, or assumed proof constants.
    // THE GAP: this only covers the named examples requested by each caller;
    // it does not certify other fences or prevent post-fence scope leakage.
    let bindings_before = derived_cat3_bindings(&env, &owned);
    let trust_before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    for example in examples {
        assert!(
            !env.globals.contains_key(*example)
                && !bindings_before.contains_key(&format!("{DERIVED}.{example}")),
            "{example} must not be tangled into the provider"
        );
    }
    env.execute_loaded_entry_checked_fences(DERIVED)
        .expect("Derived owner-local examples and paired rejects must all check");
    // MEASURED: every qualified Derived binding retains its owned ID and the
    // trusted-base set is identical before and after this fence execution.
    // CLAIMED: owner examples preserve provider identity and add zero trust.
    // THE GAP: qualified bindings and trust do not capture the active client
    // root_scope; item 2a must separately restore it after the owner fences.
    assert_eq!(
        derived_cat3_bindings(&env, &owned),
        bindings_before,
        "checked examples must preserve every Derived-qualified owner identity"
    );
    assert_eq!(
        env.env.trusted_base().into_iter().collect::<BTreeSet<_>>(),
        trust_before,
        "owner examples must not introduce trust"
    );
    for example in examples {
        let id = *env
            .globals
            .get(*example)
            .unwrap_or_else(|| panic!("{example} must elaborate inside the owner fence"));
        assert!(!owned.contains(&id), "{example} must not be a provider declaration");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{example} must be a transparent checked result"
        );
    }
    (env, owned)
}

// MEASURED: a loader-owned private GlobalId appears as Term::Const in the
// transparent example's checked type or body, as selected by the caller.
// CLAIMED: this example refers to the actual private provider declaration.
// THE GAP: occurrence alone is not semantic necessity; the checked theorem,
// concrete evaluation where applicable, and law-erasure mutation provide
// independent evidence, but do not prove every occurrence is indispensable.
fn assert_cat3_example_reference(
    env: &ElabEnv,
    owned: &[GlobalId],
    example: &str,
    private: &str,
    in_body: bool,
) {
    let provider = derived_cat3_id(env, owned, private);
    let Some(Decl::Transparent { ty, body, .. }) = env.env.lookup(env.globals[example]) else {
        panic!("{example} must be transparent");
    };
    assert!(
        term_reference_count(if in_body { body } else { ty }, provider) > 0,
        "{example} must cite loader-owned Derived.{private} in its checked artifact"
    );
}

#[test]
fn cat3_d1_structural_collections_package_elaborates_zero_delta() {
    let (env, derived_owned) = mk_env_with_derived_owned();

    for name in [
        "mem",
        "length",
        "take_drop_decomposition",
        "map_length",
        "length_take_min",
        "eq_from_ord",
        "count",
        "Perm",
        "insert",
        "sort",
        "insert_true_bool",
        "sort_bool",
        "sort_bool_sorted",
        "sort_bool_perm",
        "id_bool",
        "fst_pair_bool_bool",
        "set_fst_pair_bool_bool",
        "fst_lens_get_set",
        "fst_lens_set_get",
        "set_fst_pair_bool_bool::set_set",
        "bool_iso_to",
        "bool_iso_from",
        "bool_iso_to_from",
        "bool_iso_from_to",
        "true_refinement_project",
        "bool_pair_index_project",
        "id_bool::respects",
    ] {
        let id = catalog_or::provider_owned_id(&env, &derived_owned, "Data.Collections.Derived", name)
            .unwrap_or_else(|error| panic!("{name} must be checked by Derived.ken: {error}"));
        match env.env.lookup(id) {
            Some(Decl::Transparent { .. }) => {}
            other => panic!("{name} must be a transparent checked definition, got {other:?}"),
        }
        let delta = trusted_base_delta(&env.env, id);
        assert!(
            delta.is_empty(),
            "{name} must add zero trusted_base delta, got {delta:?}"
        );
    }

    for name in [
        "View",
        "Lens",
        "Iso",
        "Representation",
        "RefinementView",
        "IndexedView",
        "SetoidMorphism",
    ] {
        let matches = derived_owned
            .iter()
            .copied()
            .filter(|id| {
                env.class_env.class_by_id(*id).is_some_and(|class| {
                    class.projection.owner_name == name && class.projection.type_id == *id
                })
            })
            .collect::<Vec<_>>();
        let [id] = matches.as_slice() else {
            panic!("Derived must own exactly one checked class `{name}`, got {matches:?}")
        };
        let id = *id;
        match env.env.lookup(id) {
            Some(Decl::Transparent { .. }) => {}
            other => panic!("{name} must be a transparent checked record type, got {other:?}"),
        }
        assert!(
            !env.env.trusted_base().contains(&id),
            "{name}'s own class-type id must never enter trusted_base()"
        );
    }
}

/// Promise class: durable invariant.
///
/// MEASURED: the bare prelude lacks `map`/`filter`. Real roots loading makes
/// both checked Derived identities available, and the consumer's selective
/// import, `map_length` and generic wrappers reference exactly those identities.
/// Nondegenerate Nil/Cons examples establish both recursive operations.
/// CLAIMED: Derived supplies the unique structural List map/filter pair with
/// zero new trust. THE GAP: a differently named, behaviorally isomorphic
/// helper is outside this identity pin; the complete catalog census and
/// reviewed declaration inventory own that residual.
#[test]
fn derived_owns_checked_map_and_filter_identities() {
    let mut env = ElabEnv::new().expect("base env");
    assert!(!env.globals.contains_key("map"));
    assert!(!env.globals.contains_key("filter"));

    let transport_owned = catalog_or::load_core_logic_compare(&mut env);
    let provider_state = catalog_or::core_logic_or_module_state(&env);
    catalog_or::expose_core_logic_transport(&mut env, &transport_owned);
    catalog_or::restore_core_logic_or_module_state(&mut env, &provider_state);
    let (_, owned) =
        catalog_or::load_derived_importing_fixture_many(&mut env, &["map", "filter"]);
    let derived_map = catalog_or::provider_owned_id(&env, &owned, "Data.Collections.Derived", "map")
        .expect("Derived must own checked map");
    let derived_filter = catalog_or::provider_owned_id(&env, &owned, "Data.Collections.Derived", "filter")
        .expect("Derived must own checked filter");
    assert_ne!(derived_map, derived_filter);
    assert!(!env.globals.contains_key("map"));
    assert!(!env.globals.contains_key("filter"));
    for (name, id) in [("map", derived_map), ("filter", derived_filter)] {
        assert!(
            env.env.transparent_body(id).is_some(),
            "Derived.{name} must be a checked transparent definition"
        );
        assert!(
            !env.env.trusted_base().contains(&id),
            "Derived.{name} must not add an assumption"
        );
    }

    let map_length = catalog_or::provider_owned_id(&env, &owned, "Data.Collections.Derived", "map_length")
        .expect("Derived must own checked map_length");
    let (map_length_ty, map_length_body) = match env.env.lookup(map_length) {
        Some(Decl::Transparent { ty, body, .. }) => (ty, body),
        other => panic!("map_length must remain transparent, got {other:?}"),
    };
    assert!(
        term_reference_count(map_length_ty, derived_map) > 0,
        "map_length's statement must resolve the checked Derived.map identity"
    );
    assert!(
        term_reference_count(map_length_body, derived_map) > 0,
        "map_length's proof must resolve the checked Derived.map identity"
    );

    env.elaborate_file(
        "import Data.Collections.Derived (map, filter)\n\
         fn cat_derived_map_wrapper \
           (a : Type) (b : Type) (f : a → b) (xs : List a) : List b = \
           map a b f xs\n\
         fn cat_derived_filter_wrapper \
           (a : Type) (p : a → Bool) (xs : List a) : List a = \
           filter a p xs",
    )
    .expect("selectively imported wrappers must elaborate through Derived");
    assert!(!env.globals.contains_key("map"));
    assert!(!env.globals.contains_key("filter"));
    for (wrapper, provider) in [
        ("cat_derived_map_wrapper", derived_map),
        ("cat_derived_filter_wrapper", derived_filter),
    ] {
        let id = env.globals[wrapper];
        let body = match env.env.lookup(id) {
            Some(Decl::Transparent { body, .. }) => body,
            other => panic!("{wrapper} must remain transparent, got {other:?}"),
        };
        assert_eq!(
            term_reference_count(body, provider),
            1,
            "{wrapper} must retain exactly the Derived provider identity"
        );
    }

    for name in [
        "map_length",
        "sort_bool_sorted",
        "sort_bool_perm",
        "concat",
        "slice",
        "char_at",
        "eq",
        "compare",
        "bytes_nat_length",
    ] {
        let id = catalog_or::provider_owned_id(&env, &owned, "Data.Collections.Derived", name)
            .unwrap_or_else(|error| panic!("Derived dependent {name}: {error}"));
        assert!(
            env.env.transparent_body(id).is_some(),
            "Data.Collections.Derived.{name} must elaborate as a retained transparent dependent"
        );
    }

    env.elaborate_file(
        "import Data.Collections.Derived (map, filter)\n\
         fn cat_derived_flip (x : Bool) : Bool = \
           match x { False ↦ True; True ↦ False }\n\
         fn cat_derived_keep_true (x : Bool) : Bool = x\n\
         const cat_derived_map_nil : List Bool = \
           map Bool Bool cat_derived_flip (Nil Bool)\n\
         const cat_derived_map_recursive_cons : List Bool = \
           map Bool Bool cat_derived_flip \
             (Cons Bool True (Cons Bool False (Nil Bool)))\n\
         const cat_derived_filter_nil : List Bool = \
           filter Bool cat_derived_keep_true (Nil Bool)\n\
         const cat_derived_filter_recursive_cons_both_outcomes : List Bool = \
           filter Bool cat_derived_keep_true \
             (Cons Bool False (Cons Bool True (Cons Bool False (Nil Bool))))",
    )
    .expect("Derived map/filter must retain nondegenerate Nil/Cons behavior");

    let mut store = make_store(&env);
    for (name, expected) in [
        ("cat_derived_map_nil", vec![]),
        ("cat_derived_map_recursive_cons", vec![false, true]),
        ("cat_derived_filter_nil", vec![]),
        (
            "cat_derived_filter_recursive_cons_both_outcomes",
            vec![true],
        ),
    ] {
        let value = eval_transparent(&env, &mut store, env.globals[name]);
        assert_eq!(
            boolean_list(&env, value),
            expected,
            "{name} must preserve the Derived provider's recursive behavior"
        );
    }
}

/// Promise class: durable invariant.
///
/// MEASURED: the real roots-loaded Derived law types and slice body retain the
/// canonical Order provider GlobalIds, no distinct local operation, and no
/// trusted-base growth after the provider closure is loaded. CLAIMED: Derived
/// reuses canonical `min` and saturating `sub` directly. THE GAP: the existing
/// proof and behavior tests separately establish the laws and slice semantics.
#[test]
fn derived_reuses_canonical_nat_order_operations_with_zero_trust_delta() {
    let mut env = ElabEnv::new().expect("base env");
    let transport_owned = catalog_or::load_core_logic_compare(&mut env);
    catalog_or::expose_core_logic_transport(&mut env, &transport_owned);
    let nat_order_owned = env.elaborate_module_from_roots(&[catalog_or::catalog_root()], "Data.Numeric.Nat.Order")
        .expect("canonical Nat order provider must roots-load");
    let before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    let owned = catalog_or::load_derived_fixture(&mut env);
    let after: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(before, after, "Derived reuse must add zero trust");

    let min = catalog_or::provider_owned_id(&env, &nat_order_owned, "Data.Numeric.Nat.Order", "min")
        .expect("Nat.Order must own checked min");
    let sub = catalog_or::provider_owned_id(&env, &nat_order_owned, "Data.Numeric.Nat.Order", "sub")
        .expect("Nat.Order must own checked sub");
    assert!(env.env.transparent_body(min).is_some());
    assert!(env.env.transparent_body(sub).is_some());
    assert!(
        !env.globals.contains_key("Data.Collections.Derived.nat_sub"),
        "Derived must not mint local `nat_sub`"
    );
    for (local_name, provider) in [
        ("Data.Collections.Derived.min", min),
        ("Data.Collections.Derived.sub", sub),
    ] {
        if let Some(local_binding) = env.globals.get(local_name) {
            assert_eq!(
                *local_binding, provider,
                "a module-local imported binding must preserve provider identity"
            );
        }
    }

    for law in ["length_take_min", "zip_length"] {
        let id = catalog_or::provider_owned_id(&env, &owned, "Data.Collections.Derived", law)
            .unwrap_or_else(|error| panic!("Derived law {law}: {error}"));
        let ty = match env.env.lookup(id) {
            Some(Decl::Transparent { ty, .. }) => ty,
            other => panic!("{law} must be transparent, got {other:?}"),
        };
        assert_eq!(
            term_reference_count(ty, min),
            1,
            "{law}'s checked statement must use canonical min directly"
        );
    }

    let slice = catalog_or::provider_owned_id(&env, &owned, "Data.Collections.Derived", "slice")
        .expect("Derived must own checked slice");
    let body = match env.env.lookup(slice) {
        Some(Decl::Transparent { body, .. }) => body,
        other => panic!("slice must be transparent, got {other:?}"),
    };
    assert_eq!(
        term_reference_count(body, sub),
        1,
        "slice must use canonical saturating sub directly"
    );
}

/// Promise class: durable invariant.
///
/// MEASURED: roots-loading Derived after LawfulClasses adds no trust, mints no
/// Derived-local `bool_and`/`bool_leq`, and leaves no module-owned admitted
/// transparent zero-level declaration whose checked type and body at the provider
/// type are kernel-definitionally equal to either canonical provider. CLAIMED:
/// this is a narrow anti-duplication inventory only. THE GAP: it establishes no
/// causal flow or route authority and no extensional uniqueness across
/// non-convertible bodies; the concrete sort and law tests separately own behavior.
#[test]
fn derived_has_no_definitionally_equivalent_local_bool_reimplementation() {
    let mut env = ElabEnv::new().expect("base env");
    let lawful_owned = env.elaborate_module_from_roots(&[catalog_or::catalog_root()], "Core.Classes.LawfulClasses")
        .expect("the canonical Boolean provider must roots-load");
    let before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    env.elaborate_module_from_roots(&[catalog_or::catalog_root()], "Data.Collections.Derived")
        .expect("Derived must roots-load through the Boolean provider import");
    let after: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(before, after, "Derived Boolean reuse must add zero trust");

    let bool_and = catalog_or::provider_owned_id(&env, &lawful_owned, "Core.Classes.LawfulClasses", "bool_and")
        .expect("LawfulClasses must own checked bool_and");
    let bool_leq = catalog_or::provider_owned_id(&env, &lawful_owned, "Core.Classes.LawfulClasses", "bool_leq")
        .expect("LawfulClasses must own checked bool_leq");
    assert!(env.env.transparent_body(bool_and).is_some());
    assert!(env.env.transparent_body(bool_leq).is_some());
    assert!(!env
        .globals
        .contains_key("Data.Collections.Derived.bool_and"));
    assert!(!env
        .globals
        .contains_key("Data.Collections.Derived.bool_leq"));

    for (provider_name, provider) in [("bool_and", bool_and), ("bool_leq", bool_leq)] {
        assert_eq!(
            module_transparent_kernel_equivalents(&env, "Data.Collections.Derived", provider),
            BTreeSet::new(),
            "Derived must define no transparent local with kernel-definitionally equal checked type and body at the provider type for canonical {provider_name}"
        );
    }

    env.elaborate_file(
        "import Data.Collections.Derived (eq_from_ord as derived_eq_from_ord)\n\
         import Core.Classes.LawfulClasses (bool_leq as lawful_bool_leq)\n\
         theorem cat_bool_reuse_distinct \
           : Equal Bool (derived_eq_from_ord Bool lawful_bool_leq False True) False = Proved\n\
         theorem cat_bool_reuse_reflexive \
           : Equal Bool (derived_eq_from_ord Bool lawful_bool_leq True True) True = Proved",
    )
    .expect("Derived equality must retain its nontrivial Boolean behavior");
}

/// Promise class: durable semantic discriminator.
///
/// MEASURED: two transparent owner examples cite the owned `slice` body and
/// evaluate to `"bc"` for (1, 3) and empty for the reversed (3, 1) bounds.
/// CLAIMED: the real Derived slice uses end-minus-start width with saturating
/// underflow rather than start-minus-end or an unguarded subtraction.
/// THE GAP: these two closed examples distinguish the stated wrong formulas,
/// not every possible slice input; provider laws own the general contract.
#[test]
fn slice_width_is_end_minus_start_through_production_slice() {
    let names = [
        "derived_example_cat3_slice_ordinary",
        "derived_example_cat3_slice_underflow",
    ];
    let (env, owned) = cat3_owner_examples(&names);
    for name in names {
        assert_cat3_example_reference(&env, &owned, name, "slice", true);
    }
    let mut store = make_store(&env);
    assert_eq!(
        eval_transparent(&env, &mut store, env.globals[names[0]]),
        EvalVal::Str("bc".into()),
        "slice 1 3 must use end minus start and return bc"
    );
    assert_eq!(
        eval_transparent(&env, &mut store, env.globals[names[1]]),
        EvalVal::Str(String::new().into()),
        "slice 3 1 must saturate end minus start to zero"
    );
}

#[test]
fn cat3_d1_law_surfaces_are_proof_returning_not_prop_wrappers() {
    let compact = COLLECTIONS_KEN_MD
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    assert!(
        compact.contains("theorem take_drop_decomposition")
            && compact.contains(": Equal (List a) (list_append a (take a n xs) (drop a n xs)) xs"),
        "take/drop decomposition must be a proof-returning Equal surface"
    );
    assert!(
        compact.contains("theorem map_length")
            && compact.contains(": Equal Nat (length b (map a b f xs)) (length a xs)"),
        "map length preservation must be a proof-returning Equal surface"
    );
    assert!(
        compact.contains("theorem length_take_min")
            && compact.contains(": Equal Nat (length a (take a n xs)) (min n (length a xs))"),
        "take length/min law must be a proof-returning Equal surface"
    );
    assert!(
        !COLLECTIONS_KEN_MD.contains(": Prop = Equal"),
        "CAT-3 D1 laws must not be `fn law : Prop = Equal ...` wrappers"
    );
    assert!(
        !COLLECTIONS_KEN_MD.contains("= Axiom"),
        "collections CAT-3 slice must not use Axiom"
    );
    assert!(
        !COLLECTIONS_KEN_MD.contains("data Perm"),
        "CAT-3 D2 permutation must be count equality, not a raw proof-relevant data family"
    );
    assert!(
        compact.contains("fn Perm (a : Type) (eqf : a → a → Bool)")
            && compact.contains("(x : a) → Equal Nat (count a eqf x xs) (count a eqf x ys)"),
        "CAT-3 D2 Perm must be the comparator-indexed count/multiset equality surface"
    );
    assert!(
        compact.contains("fn eq_from_ord") && compact.contains("bool_and (le x y) (le y x)"),
        "eq_from_ord must be the pinned bool_and (le x y) (le y x) definition"
    );
    assert!(
        compact.contains("class View A")
            && compact.contains("class Lens A")
            && compact.contains("class SetoidMorphism A")
            && compact.contains("project : Bool → Bool"),
        "CAT-3 D3 must expose capitalized View/Lens records and a setoid-morphism project field"
    );
    assert!(
        !COLLECTIONS_KEN_MD.contains("class view")
            && !COLLECTIONS_KEN_MD.contains("fn view")
            && !COLLECTIONS_KEN_MD.contains("const view")
            && !COLLECTIONS_KEN_MD.contains("\nview "),
        "CAT-3 D3 must not introduce a lowercase `view` identifier or retired view declaration"
    );
}

/// Promise class: durable checked-proof invariant.
///
/// MEASURED: owner-checked concrete examples cite the loader-owned three D1
/// laws in their bodies and private take/drop/mem in their types; a fresh
/// client imports public `map` but refuses private `take` as UnboundName.
/// CLAIMED: D1 proofs and private operations work in the owner without the
/// flat door, while only the declared public name is importable by a client.
/// THE GAP: these examples instantiate, rather than re-prove, the generic
/// laws. A fresh import test does not detect post-fence root_scope leakage;
/// the separate item-2a repair must restore scope on every executor exit.
#[test]
fn cat3_d1_positive_surfaces_check_against_real_package_defs() {
    let names = [
        "derived_example_cat3_take_drop",
        "derived_example_cat3_map_length",
        "derived_example_cat3_length_take_min",
        "derived_example_cat3_filter_mem",
    ];
    let (env, owned) = cat3_owner_examples(&names);
    for (example, private) in [
        (names[0], "take_drop_decomposition"),
        (names[1], "map_length"),
        (names[2], "length_take_min"),
    ] {
        assert_cat3_example_reference(&env, &owned, example, private, true);
    }
    for private in ["take", "drop"] {
        assert_cat3_example_reference(&env, &owned, names[0], private, false);
    }
    assert_cat3_example_reference(&env, &owned, names[3], "mem", false);

    // MEASURED: a fresh client accepts selective `map` and refuses `take`.
    // CLAIMED: owner-local take does not become a public Derived export.
    // THE GAP: fresh-client privacy is blind to the post-fence root_scope leak;
    // item 2a owns the post-execution refusal test and scope restoration.
    let (mut client, _) = derived_cat3_owner();
    client
        .elaborate_file("import Data.Collections.Derived (map)")
        .expect("the existing public map stays selectively importable");
    match client.elaborate_file("import Data.Collections.Derived (take)") {
        Err(ElabError::UnboundName { name, .. }) => assert_eq!(name, format!("{DERIVED}.take")),
        Err(other) => panic!("private take must refuse import as UnboundName: {other:?}"),
        Ok(_) => panic!("private take must not become an export"),
    }
}

/// Promise class: durable checked-proof invariant.
///
/// MEASURED: owner-checked sort examples cite the owned sortedness and
/// permutation laws in their bodies, and sort_bool/Perm in their types.
/// CLAIMED: concrete Bool sort witnesses use the real private Derived laws,
/// not flat aliases, while public bool_leq resolves in Derived's own imports.
/// THE GAP: the example citations pin identity and checked instantiation,
/// not all inputs or semantic indispensability of every cited constant.
#[test]
fn cat3_d2_bool_sort_surfaces_check_against_real_package_defs() {
    let names = [
        "derived_example_cat3_sort_bool_sorted",
        "derived_example_cat3_sort_bool_perm",
    ];
    let (env, owned) = cat3_owner_examples(&names);
    for (example, private) in [
        (names[0], "sort_bool_sorted"),
        (names[1], "sort_bool_perm"),
    ] {
        assert_cat3_example_reference(&env, &owned, example, private, true);
    }
    assert_cat3_example_reference(&env, &owned, names[0], "sort_bool", false);
    assert_cat3_example_reference(&env, &owned, names[1], "Perm", false);
}

/// Promise class: durable negative discriminator.
///
/// MEASURED: the owner rejects the wrong Nil endpoint paired with the valid
/// take/drop-law application, and direct owner-scoped elaboration rejects the
/// original false `Proved` counterexample at the proof-checking boundary.
/// CLAIMED: the private take/drop law cannot prove that false endpoint.
/// THE GAP: `ken reject` alone accepts any error, so the direct check guards
/// the error family. Its owner scope currently persists after fence execution;
/// item 2a must keep this counterexample live when that leak is removed.
#[test]
fn cat3_d1_wrong_take_drop_witness_rejected() {
    let example = "derived_example_cat3_take_drop_negative_control";
    let (mut env, owned) = cat3_owner_examples(&[example]);
    assert_cat3_example_reference(&env, &owned, example, "take_drop_decomposition", true);
    for private in ["take", "drop"] {
        assert_cat3_example_reference(&env, &owned, example, private, false);
    }
    let err = env
        .elaborate_decl(
            "theorem cat3_bad_take_drop \
               : Equal (List Bool) \
                  (list_append Bool \
                    (take Bool (Suc Zero) (Cons Bool True (Nil Bool))) \
                    (drop Bool (Suc Zero) (Cons Bool True (Nil Bool)))) \
                  (Nil Bool) \
               = Proved",
        )
        .expect_err("wrong owner-local take/drop endpoint must not typecheck");
    let msg = format!("{err}");
    assert!(
        msg.contains("Type mismatch")
            || msg.contains("type mismatch")
            || msg.contains("Kernel rejected"),
        "wrong owner-local witness must reject during proof checking, got {msg}"
    );
}

/// Promise class: durable negative discriminator.
///
/// MEASURED: the owner rejects descending sortedness and lost-True Perm;
/// their valid-neighbor fence mutations become accepted and red the tests.
/// Direct owner-scoped checks also reject both original proof claims as a
/// type mismatch or kernel refusal, rather than an unresolved private name.
/// CLAIMED: these false concrete order/count propositions cannot be proved.
/// THE GAP: the two inputs are not a full sorting/permutation proof oracle;
/// direct checks presently rely on post-fence owner-scope persistence, so
/// item 2a must preserve their proof-checking discriminator after its fix.
#[test]
fn cat3_d2_bad_sorted_and_bad_perm_witnesses_rejected() {
    let names = [
        "derived_example_cat3_sorted_negative_control",
        "derived_example_cat3_perm_negative_control",
    ];
    let (mut env, owned) = cat3_owner_examples(&names);
    assert_cat3_example_reference(&env, &owned, names[0], "sort_bool_sorted", true);
    assert_cat3_example_reference(&env, &owned, names[1], "Perm", false);
    assert_cat3_example_reference(&env, &owned, names[1], "eq_from_ord", false);

    let err = env
        .elaborate_decl(
            "theorem cat3_bad_sorted_bool \
               : is_sorted Bool bool_leq (Cons Bool True (Cons Bool False (Nil Bool))) = Proved",
        )
        .expect_err("descending Bool list must not satisfy owner-local is_sorted");
    let msg = format!("{err}");
    assert!(
        msg.contains("Type mismatch")
            || msg.contains("type mismatch")
            || msg.contains("Kernel rejected"),
        "bad sorted witness should reject during proof checking, got {msg}"
    );

    let err = env
        .elaborate_decl(
            "theorem cat3_bad_perm_bool \
               : Perm Bool (eq_from_ord Bool bool_leq) \
                   (Cons Bool True (Nil Bool)) \
                   (Nil Bool) = \
                 \\q. match q { False |-> Proved ; True |-> Proved }",
        )
        .expect_err("dropping True must not satisfy owner-local count-based Perm");
    let msg = format!("{err}");
    assert!(
        msg.contains("Type mismatch")
            || msg.contains("type mismatch")
            || msg.contains("Kernel rejected"),
        "bad permutation witness should reject during proof checking, got {msg}"
    );
}

/// Promise class: durable checked-class and proof invariant.
///
/// MEASURED: seven registered class metadata IDs are owned transparent
/// records; SetoidMorphism has `project`; five checked owner examples cite
/// the corresponding owned lens, indexed and setoid operations or laws.
/// CLAIMED: the concrete D3 witnesses use Derived's real classes and laws,
/// not spellings installed by the flat fixture.
/// THE GAP: registry metadata and constant occurrence are not a proof that
/// all possible clients resolve these classes, or that every cited law is
/// semantically necessary; only these checked examples and fields are pinned.
#[test]
fn cat3_d3_view_lens_records_and_flavors_check_against_real_package_defs() {
    let names = [
        "derived_example_cat3_lens_get_set",
        "derived_example_cat3_lens_set_get",
        "derived_example_cat3_lens_set_set",
        "derived_example_cat3_indexed_project",
        "derived_example_cat3_setoid_project",
    ];
    let (env, owned) = cat3_owner_examples(&names);
    for class_name in [
        "View",
        "Lens",
        "Iso",
        "Representation",
        "RefinementView",
        "IndexedView",
        "SetoidMorphism",
    ] {
        let class = env
            .class_env
            .class(class_name)
            .unwrap_or_else(|| panic!("{class_name} must be a checked class"));
        assert_eq!(class.projection.owner_name, class_name);
        let id = class.projection.type_id;
        assert!(owned.contains(&id), "Derived must own {class_name}");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{class_name} must be a checked record"
        );
    }
    assert!(
        env.class_env
            .class("SetoidMorphism")
            .unwrap()
            .projection
            .field_names
            .iter()
            .any(|name| name == "project"),
        "setoid-morphism flavor must use field name `project`"
    );
    for (example, law) in [
        (names[0], "fst_lens_get_set"),
        (names[1], "fst_lens_set_get"),
        (names[2], "set_fst_pair_bool_bool::set_set"),
        (names[4], "id_bool::respects"),
    ] {
        assert_cat3_example_reference(&env, &owned, example, law, true);
    }
    assert_cat3_example_reference(&env, &owned, names[3], "bool_pair_index_project", false);
}

/// Promise class: durable negative discriminator.
///
/// MEASURED: the owner rejects the get-set example with only its endpoint
/// changed from False to True; restoring False makes that reject fence red.
/// Direct owner-scoped elaboration rejects the original wrong proof claim
/// at a type-checking or kernel-refusal error, not an unresolved name.
/// CLAIMED: the checked lens get-set law does not prove the wrong endpoint.
/// THE GAP: the fence alone can reject for an unrelated reason; the direct
/// check is currently supported by leaked post-fence owner scope, which item
/// 2a must remove while retaining the exact-error discriminator.
#[test]
fn cat3_d3_wrong_lens_endpoint_rejected() {
    let example = "derived_example_cat3_lens_get_set";
    let (mut env, owned) = cat3_owner_examples(&[example]);
    assert_cat3_example_reference(&env, &owned, example, "fst_lens_get_set", true);
    for private in ["fst_pair_bool_bool", "set_fst_pair_bool_bool"] {
        assert_cat3_example_reference(&env, &owned, example, private, false);
    }
    let err = env
        .elaborate_decl(
            "theorem cat3_bad_lens_get_set \
               : Equal Bool \
                   (fst_pair_bool_bool (set_fst_pair_bool_bool False (mk_pair Bool Bool True True))) \
                   True \
               = fst_lens_get_set False (mk_pair Bool Bool True True)",
        )
        .expect_err("wrong owner-local get-set endpoint must not typecheck");
    let msg = format!("{err}");
    assert!(
        msg.contains("Type mismatch")
            || msg.contains("type mismatch")
            || msg.contains("Kernel rejected"),
        "wrong lens law endpoint should reject during proof checking, got {msg}"
    );
}
