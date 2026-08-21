//! `V3-FO-CHECKER-SOUNDNESS` D1b reflection controls.
//!
//! Promise class: normative compatibility vector. `23 §4.3` fixes the exact
//! proposition types and bodies: derivability is the truncation of the indexed
//! rule tree, and classical validity is derivability of the empty-to-singleton
//! sequent. Changing either definition requires changing that contract.
//!
//! MEASURED: the registered transparent definitions have the same kernel types
//! and bodies as independently elaborated declarations of the two specified
//! forms. CLAIMED: `FoKripke.ken` realizes the D1b proof-theoretic reflection.
//! THE GAP: both sides share the landed constructors and surface elaborator, but
//! the expected declarations do not read or unfold the production bodies; a
//! production-body drift therefore changes only the measured artifact side.
//! The trusted-base control separately compares the kernel registry before and
//! after loading the complete file.

use std::collections::BTreeSet;

use ken_elaborator::ElabEnv;
use ken_kernel::Term;

const FOK_SOURCE: &str =
    include_str!("../../../catalog/packages/Tooling/Verification/FoKripke.ken");

fn load_fok() -> (ElabEnv, BTreeSet<ken_kernel::GlobalId>) {
    let mut env = ElabEnv::new().expect("base env construction failed");
    let before = env.env.trusted_base().into_iter().collect();
    env.elaborate_file(FOK_SOURCE)
        .expect("FoKripke.ken with D1b reflection must elaborate and kernel-check");
    (env, before)
}

fn transparent_type_and_body(env: &ElabEnv, name: &str) -> (Term, Term) {
    let id = *env
        .globals
        .get(name)
        .unwrap_or_else(|| panic!("{name} must be registered globally"));
    let (_, ty) = env
        .env
        .const_type(id)
        .unwrap_or_else(|| panic!("{name} must have a registered type"));
    let (_, body) = env
        .env
        .transparent_body(id)
        .unwrap_or_else(|| panic!("{name} must be a transparent definition"));
    (ty, body)
}

#[test]
fn fok_derives_is_exact_truncation_of_the_indexed_rule_tree() {
    let (mut env, _) = load_fok();
    env.elaborate_decl("fn d1b_expected_fok_derives (s : FokSequent) : Omega = ‖FokDerivation s‖")
        .expect("the independently stated §4.3 derivability definition must elaborate");

    assert_eq!(
        transparent_type_and_body(&env, "fok_derives"),
        transparent_type_and_body(&env, "d1b_expected_fok_derives"),
        "fok_derives must retain the exact FokSequent-indexed truncation type and body"
    );
}

#[test]
fn fok_classically_valid_is_empty_to_singleton_derivability() {
    let (mut env, _) = load_fok();
    env.elaborate_decl(
        "fn d1b_expected_fok_classically_valid (q : FokForm) : Omega = \
         fok_derives (FokMkSequent (Nil FokForm) (Cons FokForm q (Nil FokForm)))",
    )
    .expect("the independently stated §4.3 classical-validity definition must elaborate");

    assert_eq!(
        transparent_type_and_body(&env, "fok_classically_valid"),
        transparent_type_and_body(&env, "d1b_expected_fok_classically_valid"),
        "fok_classically_valid must retain the empty-antecedent singleton-conclusion body"
    );
}

#[test]
fn d1b_reflection_adds_no_trusted_base_entry() {
    let (env, before) = load_fok();
    let after: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(
        before, after,
        "ordinary transparent D1b definitions must add no trusted-base entry"
    );
}
