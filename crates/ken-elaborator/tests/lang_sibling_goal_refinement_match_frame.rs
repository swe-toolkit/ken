//! Checked Vector law and the genuinely ambient binder counterexample.
//! The fixtures are compiled as declarations after the current catalog Vector
//! source, so they exercise its real zip_with, lookup, Fin, and Vec identities.

use std::path::PathBuf;

use ken_elaborator::{ElabEnv, ElabError};

const VECTOR: &str = include_str!("../../../catalog/packages/Data/Vector/Vector.ken.md");
const LAW: &str = include_str!("fixtures/sibling_goal_refinement/lookup_zip_with.ken.md");
const AMBIENT: &str =
    include_str!("fixtures/sibling_goal_refinement/lookup_zip_with_user_local.ken.md");
const POINTWISE: &str =
    include_str!("fixtures/sibling_goal_refinement/zip_with_map_pointwise.ken.md");

// The Vector law follows three nested dependent matches. On this exact test,
// libtest's 2 MiB worker overflows while a stated 4 MiB worker checks both
// fixtures: the measured peak is below 4 MiB. Provision 8 MiB, over 4 MiB
// above that sufficient bound, independent of libtest's ambient default.
const ELABORATION_STACK_BYTES: usize = 8 * 1024 * 1024;

fn catalog_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn verdict(fixture: &'static str) -> Result<(), ElabError> {
    std::thread::Builder::new()
        .stack_size(ELABORATION_STACK_BYTES)
        .spawn(move || {
            let mut env = ElabEnv::new().expect("checked prelude");
            for provider in [
                "Core.Function.Combinators",
                "Core.Logic.Transport",
                "Data.Collections.List",
            ] {
                env.elaborate_module_from_roots(&[catalog_root()], provider)
                    .unwrap_or_else(|error| panic!("Vector provider {provider}: {error:?}"));
            }
            let trusted_before = env.env.trusted_base();
            let result = env.elaborate_ken_md_file(&format!("{VECTOR}\n{fixture}"));
            assert_eq!(
                env.env.trusted_base(),
                trusted_before,
                "Vector law adds no trust"
            );
            result.map(|_| ())
        })
        .expect("spawn stated-stack elaboration")
        .join()
        .expect("stated-stack elaboration panicked")
}

#[test]
fn lookup_zip_with_checks_both_index_arms_against_catalog_vector() {
    // Promise class: durable invariant. MEASURED: the entire checked theorem
    // elaborates and its FZero and FSuc methods are kernel-admitted against
    // the current catalog Vector operations. CLAIMED: nested matches retain
    // their refined target and the recursive FSuc proof consumes its tail
    // telescope. THE GAP: no runtime behavior is claimed by this proof test.
    verdict(LAW).expect("lookup_zip_with must check without axioms");
}

#[test]
fn pointwise_zip_with_map_remains_checked_against_catalog_vector() {
    // Promise class: durable invariant. MEASURED: the checked theorem
    // composes the pointwise hypothesis with both map and zip_with, including
    // the recursive constructor case, without new trust. CLAIMED: the frame
    // migration preserves existing pointwise transport. THE GAP: this law
    // alone does not prove lookup_zip_with, which has its own test above.
    verdict(POINTWISE).expect("pointwise zip-with-map law must check");
}

#[test]
fn a_user_local_sibling_checks_against_catalog_vector() {
    // Promise class: durable invariant. MEASURED: one user `let` inserted
    // between an enclosing field and a nested match elaborates and is
    // kernel-admitted against the current catalog Vector without new trust.
    // CLAIMED: an index-only user-local dependent is not a dependent of the
    // scrutinee, so the route needs no convoy for it. THE GAP: the private
    // seam test pins the coupled route for an abstractable index separately.
    verdict(AMBIENT).expect("a user-local index-only sibling must check");
}
