//! Acceptance pins for `LANG-MOD-OR-OMEGA-PARAM-ELAB`, implementing the
//! explicit-data telescope rules in `spec/30-surface/34-data-match.md` §2.1
//! while preserving its Type-ending family codomain.

use std::collections::BTreeSet;

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{Level, Term};

fn mk_env() -> ElabEnv {
    ElabEnv::new().expect("base env construction failed")
}

fn trusted_base(env: &ElabEnv) -> BTreeSet<ken_kernel::GlobalId> {
    env.env.trusted_base().into_iter().collect()
}

/// Promise class: durable invariant. Omega-sorted parameters and indices are
/// ordinary explicit-data telescope binders when the family result remains in
/// `Type`; neither spelling allocates trust.
///
/// **MEASURED:** the real explicit-data path emits `Term::Omega(Level::Zero)`
/// telescope entries, admits both family shapes, and preserves the exact
/// trusted-base set. **CLAIMED:** explicit Omega binders are spellable without
/// new trust. **THE GAP:** successful elaboration alone could misclassify
/// `Omega` as `Type`; the kernel-term assertions distinguish those sorts.
#[test]
fn omega_sorted_parameters_and_indices_elaborate_for_type_families() {
    let mut env = mk_env();
    let trust_before = trusted_base(&env);

    env.elaborate_file(
        "module Candidate { \
           data Or (a : Omega) (b : Omega) : Type where { \
             Inl : a -> Or a b; \
             Inr : b -> Or a b \
           } \
         }",
    )
    .expect("Omega-sorted parameters of a Type family must elaborate");
    let or_decl = env
        .env
        .inductive(env.globals["Candidate.Or"])
        .expect("Candidate.Or must be a kernel inductive");
    assert_eq!(
        or_decl.params,
        vec![Term::omega(Level::Zero), Term::omega(Level::Zero)]
    );
    assert_eq!(or_decl.level, Level::Zero);
    assert_eq!(or_decl.constructors.len(), 2);
    assert_eq!(or_decl.constructors[0].args, vec![Term::var(1)]);
    assert_eq!(or_decl.constructors[1].args, vec![Term::var(0)]);

    let indexed_id = env
        .elaborate_decl(
            "data IndexedByProp (p : Omega) : (w : p) -> Type where { \
               AtProof : (w : p) -> IndexedByProp p w \
             }",
        )
        .expect("an Omega-sorted index of a Type family must elaborate");
    let indexed = env
        .env
        .inductive(indexed_id)
        .expect("IndexedByProp must be a kernel inductive");
    assert_eq!(indexed.params, vec![Term::omega(Level::Zero)]);
    assert_eq!(indexed.indices, vec![Term::var(0)]);
    assert_eq!(indexed.level, Level::Zero);

    assert_eq!(
        trusted_base(&env),
        trust_before,
        "explicit Omega telescope binders must add zero trusted-base entries"
    );
}

/// Promise class: durable invariant. The family codomain, not its telescope
/// binders, controls proof relevance: the same two-constructor shape is
/// admitted at `Type` and refused at `Omega`.
///
/// **MEASURED:** two otherwise parallel surface declarations split at the
/// Type-only result-sort gate. **CLAIMED:** enabling Omega telescope binders
/// does not admit Omega-result data. **THE GAP:** a generic parse failure could
/// hide an unreachable control; the Type-result twin proves the declaration
/// shape reaches elaboration, and disabling only the codomain guard makes the
/// Omega-result arm elaborate and this test fail.
#[test]
fn multi_constructor_omega_result_remains_rejected() {
    let mut env = mk_env();
    env.elaborate_decl(
        "data TwoAtType : Type where { \
           TypeLeft : TwoAtType; \
           TypeRight : TwoAtType \
         }",
    )
    .expect("the Type-result control must elaborate");

    let result = env.elaborate_decl(
        "data TwoAtOmega : Omega where { \
           OmegaLeft : TwoAtOmega; \
           OmegaRight : TwoAtOmega \
         }",
    );
    assert!(
        matches!(result, Err(ElabError::ParseError { .. })),
        "an Omega-result data family must be refused by the existing surface result-sort gate"
    );
    assert!(!env.globals.contains_key("TwoAtOmega"));
    assert!(!env.globals.contains_key("OmegaLeft"));
    assert!(!env.globals.contains_key("OmegaRight"));
}
