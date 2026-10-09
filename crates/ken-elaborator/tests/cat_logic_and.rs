//! Core.Logic.And: a proof-relevant conjunction of two Omega propositions.
//!
//! Contract: spec/50-stdlib/61-formal-languages.md §3.1.
//! Promise class: durable type and proof invariants; changes preserving the
//! conjunction and its checked constructor remain green.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::ElabEnv;

const AND: &str = "Core.Logic.And";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

/// MEASURED: the actual roots loader owns the generic family and constructor,
/// selective clients construct and eliminate both proofs, and its before/after
/// trusted-GlobalId sets are equal. CLAIMED: And is usable as a Type-valued
/// conjunction of arbitrary Omega propositions without adding assumptions.
/// THE GAP: the generic clients check both proof projections by type; they do
/// not establish an algorithm's separate use of the conjunction.
#[test]
fn generic_and_preserves_both_checked_proofs_without_new_trust() {
    let mut env = ElabEnv::new().expect("base environment");
    let before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    let owned = env
        .elaborate_module_from_roots(&[root()], AND)
        .expect("the new conjunction package must roots-load");
    let after: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(after, before, "Core.Logic.And adds no local trust");
    let family = env.globals[&format!("{AND}.And")];
    let constructor = env.globals[&format!("{AND}.Both")];
    assert!(
        owned.contains(&family),
        "the And family must be package-owned"
    );
    assert!(
        env.env
            .inductive(family)
            .expect("And is checked inductive data")
            .constructors
            .iter()
            .any(|item| item.id == constructor),
        "Both must be the checked constructor of that And identity"
    );
    env.elaborate_file(
        "import Core.Logic.And (And, Both)
         theorem project_left (p : Omega) (r : Omega) (both : And p r) : p =
           match both { Both left right ↦ left }
         theorem project_right (p : Omega) (r : Omega) (both : And p r) : r =
           match both { Both left right ↦ right }
         const actual_both : And (Equal Bool True True) (Equal Bool False False) =
           Both (Equal Bool True True) (Equal Bool False False) Proved Proved",
    )
    .expect("selective imports must construct and eliminate both Omega proofs");
}
