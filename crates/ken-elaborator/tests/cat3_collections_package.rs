//! CAT-3 acceptance for the structural collection-law slice.
//!
//! This file checks the real package source, not hand-copied snippets. The D1
//! surface is deliberately bounded to structural list ops plus proof-returning
//! `take`/`drop`, `map` length, and `take` length/min laws.
//! D2 adds the verified `List Bool` insertion-sort/count-permutation slice.

#[path = "support/catalog_or.rs"]
mod catalog_or;

use std::collections::BTreeSet;

use ken_elaborator::{foreign::trusted_base_delta, ElabEnv, NumericLitVal};
use ken_interp::eval::{eval, EvalStore, EvalVal, ListCharIds};
use ken_kernel::{Decl, GlobalId, Term};

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

fn mk_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("base env");
    catalog_or::load_core_logic_compare(&mut env);
    let provider_state = catalog_or::core_logic_or_module_state(&env);
    catalog_or::expose_core_logic_transport(&mut env);
    catalog_or::restore_core_logic_or_module_state(&mut env, &provider_state);
    catalog_or::load_derived_fixture(&mut env);
    catalog_or::assert_transparent_result_uses_core_logic_or(&env, "pair_compare_lt_cases");
    env
}

#[test]
fn cat3_d1_structural_collections_package_elaborates_zero_delta() {
    let env = mk_env();

    for name in [
        "map",
        "filter",
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
        let id = env
            .globals
            .get(name)
            .copied()
            .unwrap_or_else(|| panic!("{name} should be exported by Derived.ken"));
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
        let id = env
            .globals
            .get(name)
            .copied()
            .unwrap_or_else(|| panic!("{name} should be exported by Derived.ken"));
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
/// MEASURED: the real roots-loaded Derived law types and slice body retain the
/// canonical Order provider GlobalIds, no distinct local operation, and no
/// trusted-base growth after the provider closure is loaded. CLAIMED: Derived
/// reuses canonical `min` and saturating `sub` directly. THE GAP: the existing
/// proof and behavior tests separately establish the laws and slice semantics.
#[test]
fn derived_reuses_canonical_nat_order_operations_with_zero_trust_delta() {
    let mut env = ElabEnv::new().expect("base env");
    catalog_or::load_core_logic_compare(&mut env);
    catalog_or::expose_core_logic_transport(&mut env);
    env.elaborate_module_from_roots(&[catalog_or::catalog_root()], "Data.Numeric.Nat.Order")
        .expect("canonical Nat order provider must roots-load");
    let before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    catalog_or::load_derived_fixture(&mut env);
    let after: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(before, after, "Derived reuse must add zero trust");

    let min = env.globals["Data.Numeric.Nat.Order.min"];
    let sub = env.globals["Data.Numeric.Nat.Order.sub"];
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
        let id = env.globals[law];
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

    let slice = env.globals["slice"];
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
/// Derived-local `bool_and`/`bool_leq`, and the direct `eq_from_ord` and
/// `bool_head_leq` bodies contain the exact transparent provider identities.
/// CLAIMED: the two retired local bindings now resolve through the canonical
/// provider import without changing their direct consumer computations. THE GAP:
/// this is an identity-occurrence control, not a claim that every occurrence is
/// evaluated; the existing concrete sort and law tests own behavior.
#[test]
fn derived_bool_operations_resolve_to_lawful_provider_with_zero_trust_delta() {
    let mut env = ElabEnv::new().expect("base env");
    env.elaborate_module_from_roots(&[catalog_or::catalog_root()], "Core.Classes.LawfulClasses")
        .expect("the canonical Boolean provider must roots-load");
    let before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    env.elaborate_module_from_roots(&[catalog_or::catalog_root()], "Data.Collections.Derived")
        .expect("Derived must roots-load through the Boolean provider import");
    let after: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(before, after, "Derived Boolean reuse must add zero trust");

    let bool_and = env.globals["Core.Classes.LawfulClasses.bool_and"];
    let bool_leq = env.globals["Core.Classes.LawfulClasses.bool_leq"];
    assert!(env.env.transparent_body(bool_and).is_some());
    assert!(env.env.transparent_body(bool_leq).is_some());
    assert!(!env
        .globals
        .contains_key("Data.Collections.Derived.bool_and"));
    assert!(!env
        .globals
        .contains_key("Data.Collections.Derived.bool_leq"));

    for (consumer, provider) in [
        ("Data.Collections.Derived.eq_from_ord", bool_and),
        ("Data.Collections.Derived.bool_head_leq", bool_leq),
    ] {
        let (_, body) = env
            .env
            .transparent_body(env.globals[consumer])
            .unwrap_or_else(|| panic!("{consumer} must remain transparent"));
        assert_eq!(
            term_reference_count(&body, provider),
            1,
            "{consumer} must retain one exact canonical provider occurrence"
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

#[test]
fn slice_width_is_end_minus_start_through_production_slice() {
    let mut env = mk_env();
    let ordinary = env
        .elaborate_decl(
            "const cat3_slice_ordinary : String = \
             slice (Suc Zero) (Suc (Suc (Suc Zero))) \"abcde\"",
        )
        .expect("ordinary production slice witness must elaborate");
    let underflow = env
        .elaborate_decl(
            "const cat3_slice_underflow : String = \
             slice (Suc (Suc (Suc Zero))) (Suc Zero) \"abcde\"",
        )
        .expect("underflow production slice control must elaborate");
    let mut store = make_store(&env);

    assert_eq!(
        eval_transparent(&env, &mut store, ordinary),
        EvalVal::Str("bc".into()),
        "slice 1 3 must use end minus start and return bc"
    );
    assert_eq!(
        eval_transparent(&env, &mut store, underflow),
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

#[test]
fn cat3_d1_positive_surfaces_check_against_real_package_defs() {
    let mut env = mk_env();
    let canonical_min = env.globals["Data.Numeric.Nat.Order.min"];
    env.globals
        .insert("cat3_canonical_min".to_owned(), canonical_min);
    env.elaborate_decl("fn cat3_to_true (x : Nat) : Bool = True")
        .expect("helper predicate should elaborate");
    env.elaborate_decl("fn cat3_nat_eq_all (x : Nat) (y : Nat) : Bool = True")
        .expect("helper equality predicate should elaborate");

    env.elaborate_decl(
        "theorem cat3_take_drop_sample \
           : Equal (List Bool) \
              (list_append Bool \
                (take Bool (Suc Zero) (Cons Bool True (Cons Bool False (Nil Bool)))) \
                (drop Bool (Suc Zero) (Cons Bool True (Cons Bool False (Nil Bool))))) \
              (Cons Bool True (Cons Bool False (Nil Bool))) \
           = take_drop_decomposition Bool (Suc Zero) (Cons Bool True (Cons Bool False (Nil Bool)))",
    )
    .expect("take/drop decomposition proof should check on a concrete list");

    env.elaborate_decl(
        "theorem cat3_map_length_sample \
           : Equal Nat \
              (length Bool (map Nat Bool cat3_to_true (Cons Nat Zero (Cons Nat (Suc Zero) (Nil Nat))))) \
              (length Nat (Cons Nat Zero (Cons Nat (Suc Zero) (Nil Nat)))) \
           = map_length Nat Bool cat3_to_true (Cons Nat Zero (Cons Nat (Suc Zero) (Nil Nat)))",
    )
    .expect("map length proof should check on a concrete list");

    env.elaborate_decl(
        "theorem cat3_length_take_min_sample \
           : Equal Nat \
              (length Nat (take Nat (Suc Zero) (Cons Nat Zero (Cons Nat (Suc Zero) (Nil Nat))))) \
              (cat3_canonical_min (Suc Zero) (length Nat (Cons Nat Zero (Cons Nat (Suc Zero) (Nil Nat))))) \
           = length_take_min Nat (Suc Zero) (Cons Nat Zero (Cons Nat (Suc Zero) (Nil Nat)))",
    )
    .expect("length/take/min proof should check on a concrete list");

    env.elaborate_decl(
        "theorem cat3_filter_mem_sample \
           : Equal Bool \
              (mem Nat cat3_nat_eq_all Zero (filter Nat cat3_to_true (Cons Nat (Suc Zero) (Nil Nat)))) \
              True \
           = Proved",
    )
    .expect("filter and mem operations should reduce on concrete Bool decisions");
}

#[test]
fn cat3_d2_bool_sort_surfaces_check_against_real_package_defs() {
    let mut env = mk_env();
    let sample = "(Cons Bool True (Cons Bool False (Cons Bool True (Nil Bool))))";

    env.elaborate_decl(&format!(
        "theorem cat3_sort_bool_sorted_sample \
           : is_sorted Bool bool_leq (sort_bool {sample}) = sort_bool_sorted {sample}"
    ))
    .expect("sort_bool_sorted should prove the sortedness surface");

    env.elaborate_decl(&format!(
        "theorem cat3_sort_bool_perm_sample \
           : Perm Bool (eq_from_ord Bool bool_leq) {sample} (sort_bool {sample}) = \
             sort_bool_perm {sample}"
    ))
    .expect("sort_bool_perm should prove count/multiset equality");
}

#[test]
fn cat3_d1_wrong_take_drop_witness_rejected() {
    let mut env = mk_env();
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
        .expect_err("wrong take/drop endpoint must not typecheck");
    let msg = format!("{err}");
    assert!(
        msg.contains("Type mismatch")
            || msg.contains("type mismatch")
            || msg.contains("Kernel rejected"),
        "wrong witness should reject during type/proof checking, got {msg}"
    );
}

#[test]
fn cat3_d2_bad_sorted_and_bad_perm_witnesses_rejected() {
    let mut env = mk_env();

    let err = env
        .elaborate_decl(
            "theorem cat3_bad_sorted_bool \
               : is_sorted Bool bool_leq (Cons Bool True (Cons Bool False (Nil Bool))) = Proved",
        )
        .expect_err("descending Bool list must not satisfy is_sorted");
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
        .expect_err("dropping True must not satisfy count-based Perm");
    let msg = format!("{err}");
    assert!(
        msg.contains("Type mismatch")
            || msg.contains("type mismatch")
            || msg.contains("Kernel rejected"),
        "bad permutation witness should reject during proof checking, got {msg}"
    );
}

#[test]
fn cat3_d3_view_lens_records_and_flavors_check_against_real_package_defs() {
    let mut env = mk_env();

    for class_name in [
        "View",
        "Lens",
        "Iso",
        "Representation",
        "RefinementView",
        "IndexedView",
        "SetoidMorphism",
    ] {
        assert!(
            env.class_env.class(class_name).is_some(),
            "{class_name} should be registered as an ordinary class/record"
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

    env.elaborate_decl(
        "theorem cat3_d3_get_set_sample \
           : Equal Bool \
               (fst_pair_bool_bool (set_fst_pair_bool_bool False (mk_pair Bool Bool True True))) \
               False \
           = fst_lens_get_set False (mk_pair Bool Bool True True)",
    )
    .expect("get-set lens law should be proof-returning and check");

    env.elaborate_decl(
        "theorem cat3_d3_set_get_sample \
           : Equal (Pair Bool Bool) \
              (set_fst_pair_bool_bool (fst_pair_bool_bool (mk_pair Bool Bool True False)) (mk_pair Bool Bool True False)) \
              (mk_pair Bool Bool True False) \
           = fst_lens_set_get (mk_pair Bool Bool True False)",
    )
    .expect("set-get lens law should be proof-returning and check as full pair equality");

    env.elaborate_decl(
        "theorem cat3_d3_set_set_sample \
           : Equal (Pair Bool Bool) \
              (set_fst_pair_bool_bool False (set_fst_pair_bool_bool True (mk_pair Bool Bool True False))) \
              (set_fst_pair_bool_bool False (mk_pair Bool Bool True False)) \
           = set_fst_pair_bool_bool::set_set True False (mk_pair Bool Bool True False)",
    )
    .expect("set-set lens law should be proof-returning and check as full pair equality");

    env.elaborate_decl(
        "theorem cat3_d3_indexed_project_sample \
           : Equal Bool \
               (bool_pair_index_project (mk_pair Bool Bool True False) True) \
               False \
           = Proved",
    )
    .expect("indexed flavor should expose a concrete project operation");

    env.elaborate_decl(
        "theorem cat3_d3_setoid_project_sample \
           : Equal Bool (id_bool True) (id_bool True) = \
             id_bool::respects True True Proved",
    )
    .expect("setoid-morphism respects law should check through project");
}

#[test]
fn cat3_d3_wrong_lens_endpoint_rejected() {
    let mut env = mk_env();
    let err = env
        .elaborate_decl(
            "theorem cat3_bad_lens_get_set \
               : Equal Bool \
                   (fst_pair_bool_bool (set_fst_pair_bool_bool False (mk_pair Bool Bool True True))) \
                   True \
               = fst_lens_get_set False (mk_pair Bool Bool True True)",
        )
        .expect_err("wrong get-set endpoint must not typecheck");
    let msg = format!("{err}");
    assert!(
        msg.contains("Type mismatch")
            || msg.contains("type mismatch")
            || msg.contains("Kernel rejected"),
        "wrong lens law endpoint should reject during proof checking, got {msg}"
    );
}
