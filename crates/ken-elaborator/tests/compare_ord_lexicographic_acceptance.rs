//! Acceptance coverage for the derived three-way comparator and the
//! lexicographic `Ord (Pair a b)` / `Ord (List a)` instances.
//!
//! This deliberately drives the real catalog packages.  The concrete
//! examples discriminate all three `OrdResult` outcomes, both strict-negative
//! soundness directions, Pair head/tail lexicography, List prefix/head
//! lexicography, and every law field on nontrivial structural values.

#[path = "support/catalog_or.rs"]
mod catalog_or;

use std::collections::{BTreeMap, BTreeSet};

use ken_elaborator::{trusted_base_delta, ElabEnv, ElabError};
use ken_kernel::{env::Decl, GlobalId, Term};

const LAWFUL: &str = "Core.Classes.LawfulClasses";

fn mk_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("base env construction failed");
    let transport_owned = catalog_or::load_core_logic_compare(&mut env);
    catalog_or::expose_core_logic_transport(&mut env, &transport_owned);
    catalog_or::load_derived_fixture(&mut env);
    env
}

fn mk_env_with_lawful_owned() -> (ElabEnv, Vec<GlobalId>) {
    let mut env = mk_env();
    let lawful_owned = env
        .elaborate_module_from_roots(&[catalog_or::catalog_root()], "Core.Classes.LawfulClasses")
        .expect("LawfulClasses must return its checked IDs in the same comparison environment");
    (env, lawful_owned)
}

fn assert_bool_reduces_with_ord(
    env: &mut ElabEnv,
    name: &str,
    ty: &str,
    expression: &str,
    expected: &str,
) {
    env.elaborate_decl(&format!(
        "const {name} : Bool where Ord {ty} = {expression}"
    ))
    .unwrap_or_else(|error| panic!("{name} must resolve public Ord {ty}: {error}"));
    env.elaborate_decl(&format!(
        "theorem {name}_reduces : Equal Bool {name} {expected} = Proved"
    ))
    .unwrap_or_else(|error| panic!("{name} must reduce to {expected}: {error}"));
}

fn lawful_owner() -> (ElabEnv, Vec<GlobalId>) {
    let mut env = ElabEnv::new().expect("base environment");
    let owned = env
        .elaborate_module_from_roots(&[catalog_or::catalog_root()], LAWFUL)
        .expect("the actual LawfulClasses provider and its imports must roots-load");
    (env, owned)
}

fn lawful_id(env: &ElabEnv, owned: &[GlobalId], name: &str) -> GlobalId {
    catalog_or::provider_owned_id(env, owned, LAWFUL, name)
        .unwrap_or_else(|error| panic!("LawfulClasses provider identity: {error}"))
}

fn owned_ord_instance(env: &ElabEnv, owned: &[GlobalId], head: &str) -> GlobalId {
    let class = env
        .class_env
        .class("Ord")
        .expect("the canonical Ord registry entry must exist")
        .projection
        .type_id;
    assert!(owned.contains(&class), "the Ord class must be LC-owned");
    let instance = env
        .class_env
        .instance_search("Ord", head)
        .unwrap_or_else(|| panic!("Ord {head} must be registered"));
    assert!(
        owned.contains(&instance),
        "Ord {head} must be an LC-owned checked instance ID"
    );
    instance
}

fn qualified_lawful_bindings(env: &ElabEnv, owned: &[GlobalId]) -> BTreeMap<String, GlobalId> {
    let prefix = format!("{LAWFUL}.");
    env.globals
        .iter()
        .filter(|(name, _)| name.starts_with(&prefix))
        .map(|(name, id)| {
            assert!(
                owned.contains(id),
                "{name} must be an ID from the same roots-loaded LawfulClasses provider"
            );
            (name.clone(), *id)
        })
        .collect()
}

fn execute_owner_examples(env: &mut ElabEnv, owned: &[GlobalId], examples: &[&str]) {
    let bindings_before = qualified_lawful_bindings(env, owned);
    let trust_before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    for example in examples {
        assert!(
            !env.globals.contains_key(*example)
                && !bindings_before.contains_key(&format!("{LAWFUL}.{example}")),
            "{example} must not be a tangled owner declaration"
        );
    }
    env.execute_loaded_entry_checked_fences(LAWFUL)
        .expect("LawfulClasses owner-local checked examples must elaborate");
    assert_eq!(
        qualified_lawful_bindings(env, owned),
        bindings_before,
        "owner-local examples must preserve every qualified provider identity"
    );
    assert_eq!(
        env.env.trusted_base().into_iter().collect::<BTreeSet<_>>(),
        trust_before,
        "owner-local examples must not extend the trusted base"
    );
    for example in examples {
        let id = *env
            .globals
            .get(*example)
            .unwrap_or_else(|| panic!("{example} must be checked by the owner example fence"));
        assert!(
            !owned.contains(&id),
            "{example} must not tangle into the provider"
        );
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{example} must be a kernel-checked transparent declaration"
        );
    }
}

fn term_references(term: &Term, provider: GlobalId) -> bool {
    match term {
        Term::Const { id, .. } | Term::IndFormer { id, .. } | Term::Constructor { id, .. }
            if *id == provider =>
        {
            true
        }
        Term::Elim { fam, .. } if *fam == provider => true,
        _ => term
            .children()
            .into_iter()
            .any(|child| term_references(child, provider)),
    }
}

fn assert_example_references(
    env: &ElabEnv,
    owned: &[GlobalId],
    example: &str,
    provider: GlobalId,
    in_body: bool,
) {
    assert!(
        owned.contains(&provider),
        "the expected provider ID must be owned"
    );
    let id = env.globals[example];
    let Some(Decl::Transparent { ty, body, .. }) = env.env.lookup(id) else {
        panic!("{example} must be transparent");
    };
    let term = if in_body { body } else { ty };
    assert!(
        term_references(term, provider),
        "{example} must use the same-run owned provider ID {provider:?}"
    );
}

/// Promise class: durable checked-identity and semantic invariant. Three
/// constructor outcomes and five distinct owner-local private law applications
/// are kernel-checked without publishing a comparator or extending trust.
/// MEASURED: each example carries the authenticated provider ID in its checked
/// type or body, its transparent proof checks, and the complete qualified ID
/// map and trust set stay unchanged. CLAIMED: private compare coverage survives
/// the flat-alias door's removal. THE GAP: these are concrete Bool cases, not
/// generic theorems; the generic laws are provider-owned and checked above.
#[test]
fn raw_compare_discriminates_all_results_and_strict_negatives() {
    let (mut env, owned) = lawful_owner();
    let compare = lawful_id(&env, &owned, "compare_raw");
    let laws = [
        ("lc_example_raw_eq_sound", "compare_raw::eq_sound"),
        ("lc_example_raw_lt_sound", "compare_raw::lt_sound"),
        ("lc_example_raw_gt_sound", "compare_raw::gt_sound"),
        (
            "lc_example_raw_lt_reverse_false",
            "compare_raw::lt_reverse_false",
        ),
        (
            "lc_example_raw_gt_forward_false",
            "compare_raw::gt_forward_false",
        ),
    ];
    let examples = [
        "lc_example_raw_eq",
        "lc_example_raw_lt",
        "lc_example_raw_gt",
        "lc_example_raw_eq_sound",
        "lc_example_raw_lt_sound",
        "lc_example_raw_gt_sound",
        "lc_example_raw_lt_reverse_false",
        "lc_example_raw_gt_forward_false",
    ];
    let law_ids: Vec<_> = laws
        .iter()
        .map(|(_, name)| lawful_id(&env, &owned, name))
        .collect();
    execute_owner_examples(&mut env, &owned, &examples);
    for example in &examples[..3] {
        assert_example_references(&env, &owned, example, compare, false);
    }
    for ((example, _), law) in laws.iter().zip(law_ids) {
        assert_example_references(&env, &owned, example, law, true);
    }

    // A real outside importer can select the public class, not the private
    // raw comparator. Owner-local fence success must not widen the interface.
    let (mut client, _) = lawful_owner();
    client
        .elaborate_file("import Core.Classes.LawfulClasses (Ord)")
        .expect("the public Ord class remains importable");
    match client.elaborate_file("import Core.Classes.LawfulClasses (compare_raw)") {
        Err(ElabError::UnboundName { name, .. }) => {
            assert_eq!(name, format!("{LAWFUL}.compare_raw"))
        }
        Err(other) => panic!("private compare_raw must refuse import by identity: {other:?}"),
        Ok(_) => panic!("private compare_raw must not become publicly importable"),
    }
}

// MEASURED: an owner example checks and stays host-readable by owned ID;
// later source runs in that same fenced environment, never a fresh client.
// CLAIMED: successful fence execution retains checked artifacts without
// authorizing provider-private names for a later unit. THE GAP: only the
// bare row distinguishes this leak; the qualified row checks another route.
// Failed fences and other providers have independent controls below.
fn assert_lawful_fence_scope_is_restored(kind: &str, expression: &str, expected_name: &str) {
    let (mut env, owned) = lawful_owner();
    let provider = lawful_id(&env, &owned, "compare_raw");
    execute_owner_examples(&mut env, &owned, &["lc_example_raw_eq"]);
    assert_example_references(&env, &owned, "lc_example_raw_eq", provider, false);
    let source = format!(
        "import Core.Logic.OrdResult (OrdResult)\n\
         import Core.Classes.LawfulClasses (bool_leq)\n\
         const scope_probe_{kind} : OrdResult = {expression}"
    );
    match env.elaborate_file(&source) {
        Err(ElabError::UnboundName { name, .. } | ElabError::UnresolvedCon { name, .. }) => {
            assert_eq!(name, expected_name, "{kind} private reference must refuse");
        }
        other => panic!("{kind} private reference must refuse by name: {other:?}"),
    }
}

/// Promise class: durable private-scope isolation invariant (33 §3.3/§4).
/// MEASURED: after the real owner fence, bare `compare_raw` refuses by name.
/// CLAIMED: the executor did not leak its owner's local root scope.
/// THE GAP: this row distinguishes the unrepaired executor, which accepts the
/// same bare source; the Err-path and other-entry controls remain separate.
#[test]
fn checked_lawful_fences_do_not_leak_bare_private_scope() {
    assert_lawful_fence_scope_is_restored(
        "bare",
        "compare_raw Bool bool_leq True False",
        "compare_raw",
    );
}

/// Promise class: durable qualified-import privacy invariant (33 §3.3/§4).
/// MEASURED: qualified private `compare_raw` refuses after the owner fence.
/// CLAIMED: an unexported member is inaccessible by qualified client spelling.
/// THE GAP: this passes on the unrepaired executor as well; prefix/export
/// authorization, not root-scope restoration, guards this path. It is a
/// non-discriminating privacy control, not evidence of the item-2a repair.
#[test]
fn checked_lawful_fences_do_not_leak_qualified_private_scope() {
    assert_lawful_fence_scope_is_restored(
        "qualified",
        "Core.Classes.LawfulClasses.compare_raw Bool bool_leq True False",
        "Core.Classes.LawfulClasses.compare_raw",
    );
}

/// Promise class: durable behavior invariant. Client-resolved canonical Ord
/// Pair/List dictionaries distinguish head, tail, and prefix lexicography.
#[test]
fn pair_and_list_instances_compute_lexicographically() {
    let (mut env, lawful_owned) = mk_env_with_lawful_owned();
    env.elaborate_file("import Core.Classes.LawfulClasses (Ord)")
        .expect("the comparison client must import public Ord");
    let pair_ord = "d";
    let list_ord = "d";

    assert_bool_reduces_with_ord(
        &mut env,
        "pair_head_lt",
        "(Pair Bool Bool)",
        &format!("({pair_ord}).leq (mk_pair Bool Bool False True) (mk_pair Bool Bool True False)"),
        "True",
    );
    assert_bool_reduces_with_ord(
        &mut env,
        "pair_head_gt",
        "(Pair Bool Bool)",
        &format!("({pair_ord}).leq (mk_pair Bool Bool True False) (mk_pair Bool Bool False True)"),
        "False",
    );
    assert_bool_reduces_with_ord(
        &mut env,
        "pair_equal_head_tail_lt",
        "(Pair Bool Bool)",
        &format!("({pair_ord}).leq (mk_pair Bool Bool True False) (mk_pair Bool Bool True True)"),
        "True",
    );
    assert_bool_reduces_with_ord(
        &mut env,
        "pair_equal_head_tail_gt",
        "(Pair Bool Bool)",
        &format!("({pair_ord}).leq (mk_pair Bool Bool True True) (mk_pair Bool Bool True False)"),
        "False",
    );

    assert_bool_reduces_with_ord(
        &mut env,
        "list_prefix_lt",
        "(List Bool)",
        &format!(
            "({list_ord}).leq (Cons Bool False (Nil Bool)) (Cons Bool False (Cons Bool True (Nil Bool)))"
        ),
        "True",
    );
    assert_bool_reduces_with_ord(
        &mut env,
        "list_prefix_gt",
        "(List Bool)",
        &format!(
            "({list_ord}).leq (Cons Bool False (Cons Bool True (Nil Bool))) (Cons Bool False (Nil Bool))"
        ),
        "False",
    );
    assert_bool_reduces_with_ord(
        &mut env,
        "list_head_lt",
        "(List Bool)",
        &format!(
            "({list_ord}).leq (Cons Bool False (Cons Bool True (Nil Bool))) (Cons Bool True (Nil Bool))"
        ),
        "True",
    );
    assert_bool_reduces_with_ord(
        &mut env,
        "list_head_gt",
        "(List Bool)",
        &format!(
            "({list_ord}).leq (Cons Bool True (Nil Bool)) (Cons Bool False (Cons Bool True (Nil Bool)))"
        ),
        "False",
    );

    let ord_class = env
        .class_env
        .class("Ord")
        .expect("checked Ord class")
        .projection
        .type_id;
    assert!(
        lawful_owned.contains(&ord_class),
        "LawfulClasses must own Ord"
    );
    for (head, names) in [
        (
            "Pair",
            [
                "pair_head_lt",
                "pair_head_gt",
                "pair_equal_head_tail_lt",
                "pair_equal_head_tail_gt",
            ],
        ),
        (
            "List",
            [
                "list_prefix_lt",
                "list_prefix_gt",
                "list_head_lt",
                "list_head_gt",
            ],
        ),
    ] {
        let instance = env
            .class_env
            .instance_search("Ord", head)
            .unwrap_or_else(|| panic!("Ord {head} must remain registered"));
        assert!(
            lawful_owned.contains(&instance),
            "Ord {head} must be LC-owned"
        );
        for name in names {
            let references = catalog_or::declaration_references(
                env.env
                    .lookup(env.globals[name])
                    .expect("the checked order vector must exist"),
            );
            assert!(
                references.contains(&instance),
                "{name} must resolve the owner-checked Ord {head} instance"
            );
        }
    }
}

/// Promise class: durable structural-law invariant. Eight concrete instances
/// of the four Pair/List laws check inside the LC owner, not via flat names.
/// MEASURED: each example's checked body cites its owned instance; the Pair
/// and List dictionaries are transparent and have zero new trusted entries.
/// CLAIMED: the structural order instances carry real law proofs. THE GAP:
/// concrete applications do not alone prove generic laws; the owned generic
/// dictionaries are kernel-checked, and their trust deltas are measured here.
#[test]
fn structural_ord_instances_and_all_laws_are_checked_zero_delta() {
    let (mut env, owned) = lawful_owner();
    let pair = owned_ord_instance(&env, &owned, "Pair");
    let list = owned_ord_instance(&env, &owned, "List");
    for (name, id) in [("Ord Pair", pair), ("Ord List", list)] {
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{name} must be a checked transparent instance"
        );
        let mut delta = trusted_base_delta(&env.env, id);
        delta.remove(&env.class_env.record_nil_val_id);
        assert!(
            delta.is_empty(),
            "{name} must add no trusted base entries: {delta:?}"
        );
    }
    let examples = [
        "lc_example_pair_refl_law",
        "lc_example_pair_antisym_law",
        "lc_example_pair_trans_law",
        "lc_example_pair_total_law",
        "lc_example_list_refl_law",
        "lc_example_list_antisym_law",
        "lc_example_list_trans_law",
        "lc_example_list_total_law",
    ];
    execute_owner_examples(&mut env, &owned, &examples);
    for example in &examples[..4] {
        assert_example_references(&env, &owned, example, pair, true);
    }
    for example in &examples[4..] {
        assert_example_references(&env, &owned, example, list, true);
    }
}

/// Promise class: durable semantic discriminator. Four opposite Pair-element
/// comparisons force the nested List instance through both components of its
/// element comparator, and two Bool cases retain the primitive base floor.
/// MEASURED: owner-local checked values reference both owned instances and
/// their concrete result proofs cite those values. CLAIMED: List ordering
/// routes and consults its Pair comparator. THE GAP: the non-Bool and opposite
/// cases distinguish head/tail order, rather than proving all generic inputs.
#[test]
fn list_instance_routes_the_canonical_compare_into_raw_list_compare() {
    // Q-CLAIM-COMPARE-ORD (2026-07-23): restore the two claims the original
    // block carried, which the Q-RESIDUE rework dropped when it collapsed to a
    // Bool-only `list_ord_leq` reduction. The original asserted two things by
    // grepping the `.ken.md` (a source scan, which proves neither -- a spelling
    // can be dead code, a comment can lie); both are restored here as kernel
    // reductions:
    //
    //   CLAIM 1 (ROUTING): the `Ord (List a)` instance routes the *canonical*
    //     derived element comparator `compare a d` into `list_compare` at the
    //     instance layer (LawfulClasses `list_ord_leq a d =
    //     ord_result_leq (list_compare a (compare a d) ...)`, projected here as
    //     the instance's own `.leq` field -- so the test exercises the routing
    //     literally through the instance, not a hand-picked helper).
    //   CLAIM 2 (PARAMETERIZATION): `list_compare` is raw-comparator
    //     parameterized -- it takes the element comparator `cmp` as a parameter
    //     and actually consults it, rather than hardcoding one element type.
    //
    // The element type must be non-`Bool`. `Bool` hides both claims: its
    // canonical `compare Bool Ord_instance_Bool` bottoms out in the primitive
    // `bool_leq`, so a `list_compare` that ignored `cmp` and hardcoded a Bool
    // comparison -- or an instance that fed a *constant* comparator instead of
    // the canonical `compare a d` -- would give identical answers on Bool-element
    // lists. Using `Pair Bool Bool` elements forces the element's own *derived*
    // lexicographic comparator (`pair_compare`) to be genuinely routed and
    // consulted: it is reachable by no non-routing / non-parameterized path, and
    // a `cmp` hardcoded to `Bool` would not even typecheck at `Pair Bool Bool`.
    let (mut env, owned) = lawful_owner();
    let list = owned_ord_instance(&env, &owned, "List");
    let pair = owned_ord_instance(&env, &owned, "Pair");
    let bool_order = lawful_id(&env, &owned, "list_ord_leq");
    let comparisons = [
        (
            "lc_example_list_pair_head_lt",
            "lc_example_list_pair_head_lt_reduces",
        ),
        (
            "lc_example_list_pair_head_gt",
            "lc_example_list_pair_head_gt_reduces",
        ),
        (
            "lc_example_list_pair_tail_lt",
            "lc_example_list_pair_tail_lt_reduces",
        ),
        (
            "lc_example_list_pair_tail_gt",
            "lc_example_list_pair_tail_gt_reduces",
        ),
        ("lc_example_list_bool_lt", "lc_example_list_bool_lt_reduces"),
        ("lc_example_list_bool_gt", "lc_example_list_bool_gt_reduces"),
    ];
    let examples: Vec<_> = comparisons
        .iter()
        .flat_map(|(value, proof)| [*value, *proof])
        .collect();
    execute_owner_examples(&mut env, &owned, &examples);
    for (index, (value, proof)) in comparisons.into_iter().enumerate() {
        assert_example_references(
            &env,
            &owned,
            value,
            if index < 4 { list } else { bool_order },
            true,
        );
        let Some(Decl::Transparent { body, .. }) = env.env.lookup(env.globals[value]) else {
            panic!("{value} must be transparent");
        };
        if index < 4 {
            assert!(
                term_references(body, pair),
                "{value} must pass the owned Pair instance to the nested List instance"
            );
        }
        let Some(Decl::Transparent { ty, .. }) = env.env.lookup(env.globals[proof]) else {
            panic!("{proof} must be transparent");
        };
        assert!(
            term_references(ty, env.globals[value]),
            "{proof} must check the result of {value} rather than a separate comparator"
        );
    }
}
