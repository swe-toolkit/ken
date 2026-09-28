//! Consumer-view controls for the private Derived filter-membership laws.
//! The fixture roots-loads the real package, then exposes its private checked
//! identities only inside the test environment; no catalog export is added.

#[path = "support/catalog_or.rs"]
mod catalog_or;

use std::collections::{BTreeMap, BTreeSet};

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{Decl, GlobalId, KernelError, Term};

const DERIVED: &str = "Data.Collections.Derived";

fn reference_count(term: &Term, target: GlobalId) -> usize {
    usize::from(matches!(term, Term::Const { id, .. } if *id == target))
        + term
            .children()
            .into_iter()
            .map(|child| reference_count(child, target))
            .sum::<usize>()
}

fn load_derived_owned() -> (ElabEnv, Vec<GlobalId>) {
    let mut env = ElabEnv::new().expect("base environment");
    assert!(
        !env.globals.contains_key("filter"),
        "filter is not a prelude name"
    );
    let owned = env
        .elaborate_module_from_roots(&[catalog_or::catalog_root()], DERIVED)
        .expect("Derived must load its real provider closure");
    (env, owned)
}

fn derived_id(env: &ElabEnv, owned: &[GlobalId], name: &str) -> GlobalId {
    catalog_or::provider_owned_id(env, owned, DERIVED, name)
        .unwrap_or_else(|error| panic!("Derived provider identity: {error}"))
}

fn load() -> (ElabEnv, GlobalId) {
    let (mut env, owned) = load_derived_owned();
    let derived_filter = derived_id(&env, &owned, "filter");
    // The second, independent false-proof discriminator retains four aliases
    // pending its separately ruled private-mem disposition.
    for (module, names) in [
        (DERIVED, &["filter", "mem"][..]),
        ("Core.Classes.LawfulClasses", &["IsTrue", "bool_and"][..]),
    ] {
        for name in names {
            let id = env.globals[&format!("{module}.{name}")];
            env.globals.insert((*name).to_owned(), id);
        }
    }
    assert_eq!(env.globals["filter"], derived_filter);
    (env, derived_filter)
}

fn derived_qualified_bindings(env: &ElabEnv, owned: &[GlobalId]) -> BTreeMap<String, GlobalId> {
    let prefix = format!("{DERIVED}.");
    env.globals
        .iter()
        .filter(|(name, _)| name.starts_with(&prefix))
        .map(|(name, id)| {
            assert!(
                owned.contains(id),
                "{name} is not a loader-owned Derived identity"
            );
            (name.clone(), *id)
        })
        .collect()
}

/// Promise class: durable checked-identity invariant.
///
/// MEASURED: each private law's raw type cites loader-owned Derived.filter;
/// in the owner scope, both generic law applications check as transparent
/// examples citing the corresponding loader-owned law and filter identity.
/// The complete Derived-qualified name/ID map and trust are unchanged by
/// entry-fence execution. CLAIMED: the laws retain their original contracts
/// without a public alias, export, or trust extension. THE GAP: the second
/// false-proof discriminator still has four separately dispositioned aliases.
#[test]
fn private_law_statements_resolve_the_derived_filter() {
    let (mut env, owned) = load_derived_owned();
    let derived_filter = derived_id(&env, &owned, "filter");
    let laws = [
        (
            "derived_example_filter_membership_generic_consumer",
            "mem_filter",
        ),
        (
            "derived_example_filter_sound_generic_consumer",
            "mem_filter_sound",
        ),
    ];
    for (_, name) in laws {
        let id = derived_id(&env, &owned, name);
        let ty = match env.env.lookup(id) {
            Some(Decl::Transparent { ty, .. }) => ty,
            other => panic!("{name} must remain a checked transparent theorem: {other:?}"),
        };
        assert!(
            reference_count(ty, derived_filter) > 0,
            "{name}'s raw type must use checked Derived.filter"
        );
    }

    let qualified_before = derived_qualified_bindings(&env, &owned);
    let trust_before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    for (example, _) in laws {
        assert!(
            !env.globals.contains_key(example),
            "{example} must not precede the fence"
        );
        assert!(
            !qualified_before.contains_key(&format!("{DERIVED}.{example}")),
            "{example} must not be a loader-visible Derived declaration"
        );
    }
    env.execute_loaded_entry_checked_fences(DERIVED)
        .expect("both private law applications must check in Derived's own scope");
    assert_eq!(
        derived_qualified_bindings(&env, &owned),
        qualified_before,
        "examples must preserve every Derived qualified name and checked identity"
    );
    assert_eq!(
        env.env.trusted_base().into_iter().collect::<BTreeSet<_>>(),
        trust_before,
        "examples must add no trusted base"
    );
    for (example, name) in laws {
        let id = derived_id(&env, &owned, name);
        let example_id = *env
            .globals
            .get(example)
            .unwrap_or_else(|| panic!("{example} must be created by the checked entry fence"));
        assert!(
            !owned.contains(&example_id),
            "{example} must not be tangled"
        );
        let Some(Decl::Transparent { ty, body, .. }) = env.env.lookup(example_id) else {
            panic!("{example} must be kernel-checked, not opaque");
        };
        assert!(
            reference_count(ty, derived_filter) > 0,
            "{example}'s checked type must cite Derived.filter"
        );
        assert!(
            reference_count(body, id) > 0,
            "{example}'s checked body must apply loader-owned {name}"
        );
    }
}

/// Promise class: durable semantic discriminator. On the same x and xs, an
/// arbitrary comparator that matches Zero while p disagrees with x gives
/// unequal endpoints; Nat structural equality gives equal endpoints. Rejection
/// must arise at the false proof, after every fixture declaration elaborates.
#[test]
fn compatibility_premise_distinguishes_true_and_false_instances() {
    let (mut env, _) = load();
    env.elaborate_file(
        "fn cat_eq_any (x : Nat) (y : Nat) : Bool = True\n\
         fn cat_eq_nat (x : Nat) (y : Nat) : Bool = \
           match x { Zero ↦ match y { Zero ↦ True; Suc k ↦ False }; \
                     Suc k ↦ match y { Zero ↦ False; Suc j ↦ cat_eq_nat k j } }\n\
         fn cat_is_zero (y : Nat) : Bool = \
           match y { Zero ↦ True; Suc k ↦ False }\n\
         const cat_x : Nat = Suc Zero\n\
         const cat_xs : List Nat = Cons Nat Zero (Nil Nat)",
    )
    .expect("shared, nontrivial fixture must elaborate");

    for (name, equation) in [
        (
            "unconstrained_left_true",
            "Equal Bool (mem Nat cat_eq_any cat_x (filter Nat cat_is_zero cat_xs)) True",
        ),
        (
            "unconstrained_right_false",
            "Equal Bool (bool_and (mem Nat cat_eq_any cat_x cat_xs) (cat_is_zero cat_x)) False",
        ),
        (
            "nat_equality_left_false",
            "Equal Bool (mem Nat cat_eq_nat cat_x (filter Nat cat_is_zero cat_xs)) False",
        ),
        (
            "nat_equality_right_false",
            "Equal Bool (bool_and (mem Nat cat_eq_nat cat_x cat_xs) (cat_is_zero cat_x)) False",
        ),
    ] {
        env.elaborate_decl(&format!("theorem {name} : {equation} = Proved"))
            .unwrap_or_else(|error| panic!("{name} must reduce as stated: {error:?}"));
    }
    env.elaborate_decl(
        "theorem compatible_nat_equality_instance \
         : Equal Bool (mem Nat cat_eq_nat cat_x (filter Nat cat_is_zero cat_xs)) \
             (bool_and (mem Nat cat_eq_nat cat_x cat_xs) (cat_is_zero cat_x)) = Proved",
    )
    .expect("the compatible Nat equality instance must accept Proved");

    let rejected = env
        .elaborate_decl(
            "theorem incompatible_unconstrained_instance \
             : Equal Bool (mem Nat cat_eq_any cat_x (filter Nat cat_is_zero cat_xs)) \
                 (bool_and (mem Nat cat_eq_any cat_x cat_xs) (cat_is_zero cat_x)) = Proved",
        )
        .expect_err("the incompatible no-compat equation must be false");
    assert!(
        matches!(
            rejected,
            ElabError::KernelRejected {
                error: KernelError::TypeMismatch { .. },
                ..
            }
        ),
        "the failure must reach the false proof obligation, got {rejected:?}"
    );
}
