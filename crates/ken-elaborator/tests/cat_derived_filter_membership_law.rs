//! Owner-scope controls for the private Derived filter-membership laws.
//! The real package is roots-loaded without flat aliases. Its checked example
//! fences exercise private operations without adding catalog exports.

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
/// without a public alias, export, or trust extension. THE GAP: example
/// declarations are checked in the entry fence, not loader-owned; the
/// absent-before, owned-ID, and trust checks defend that boundary.
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

/// Promise class: durable semantic discriminator. The same checked x and xs
/// yield True/False without comparator compatibility and False/False with
/// structural Nat equality; the attempted proof of the unequal endpoints
/// must reach KernelRejected(TypeMismatch), not fail at source resolution.
#[test]
fn compatibility_premise_distinguishes_true_and_false_instances() {
    let (mut env, owned) = load_derived_owned();
    let derived_filter = derived_id(&env, &owned, "filter");
    let derived_mem = derived_id(&env, &owned, "mem");
    let names = [
        "derived_example_equal_any",
        "derived_example_equal_nat",
        "derived_example_is_zero",
        "derived_example_x",
        "derived_example_xs",
        "derived_example_unconstrained_left",
        "derived_example_unconstrained_right",
        "derived_example_nat_equality_left",
        "derived_example_nat_equality_right",
        "derived_example_unconstrained_left_true",
        "derived_example_unconstrained_right_false",
        "derived_example_nat_equality_left_false",
        "derived_example_nat_equality_right_false",
        "derived_example_compatible_nat_equality",
    ];
    let qualified_before = derived_qualified_bindings(&env, &owned);
    let trust_before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    for name in names {
        assert!(
            !env.globals.contains_key(name),
            "{name} must come from the fence"
        );
    }

    env.execute_loaded_entry_checked_fences(DERIVED)
        .expect("both endpoint pairs and the compatible equation check in owner scope");
    assert_eq!(derived_qualified_bindings(&env, &owned), qualified_before);
    assert_eq!(
        env.env.trusted_base().into_iter().collect::<BTreeSet<_>>(),
        trust_before
    );
    for name in names {
        let id = *env
            .globals
            .get(name)
            .unwrap_or_else(|| panic!("missing {name}"));
        assert!(
            !owned.contains(&id),
            "{name} must remain an example, not a provider name"
        );
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    }
    for name in [
        "derived_example_unconstrained_left",
        "derived_example_nat_equality_left",
    ] {
        let id = env.globals[name];
        let Some(Decl::Transparent { body, .. }) = env.env.lookup(id) else {
            unreachable!()
        };
        assert!(
            reference_count(body, derived_filter) > 0,
            "{name} must apply checked filter"
        );
        assert!(
            reference_count(body, derived_mem) > 0,
            "{name} must apply private checked mem"
        );
    }
    for name in [
        "derived_example_unconstrained_right",
        "derived_example_nat_equality_right",
    ] {
        let id = env.globals[name];
        let Some(Decl::Transparent { body, .. }) = env.env.lookup(id) else {
            unreachable!()
        };
        assert!(
            reference_count(body, derived_mem) > 0,
            "{name} must apply private checked mem"
        );
    }

    let rejected = env
        .elaborate_decl(
            "theorem incompatible_unconstrained_instance \
             : Equal Bool derived_example_unconstrained_left \
                 derived_example_unconstrained_right = Proved",
        )
        .expect_err("the checked unequal endpoints must make the proof false");
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
