//! `LANG-GENERATED-INDEX-EVIDENCE-CLOSURE` acceptance.
//!
//! Generated dependent-match index evidence is closed under the three shapes it
//! may recursively expose: `Eq`, observational `Sigma`, and `Top`. User-facing
//! `Refl` remains guarded by equality origin and must not inherit generated
//! evidence's `Top` case.

use ken_elaborator::{ElabEnv, ElabError};

#[test]
fn generated_record_index_reflexivity_closes_top_second_child() {
    // Promise class: durable invariant. Generated reflexive index evidence must
    // remain closed under every Eq/Sigma/Top child it produces.
    // MEASURED: a real dependent match whose record equality has a variable Nat
    // first child (still Eq) and a closed True/True second child (Top).
    // CLAIMED: generated evidence dispatches the Sigma codomain through
    // Eq/Sigma/Top. THE GAP: restoring only the second old recursive call is the
    // independent causality mutation for this fixture.
    let mut env = ElabEnv::new().expect("base env");
    env.elaborate_decl("data GenSeq = GenMkSeq Nat Bool")
        .expect("GenSeq");
    env.elaborate_decl(
        "data GenDerivation : GenSeq -> Type where { \
           GenDeriv : (index : Nat) -> GenDerivation (GenMkSeq index True) \
         }",
    )
    .expect("GenDerivation");
    env.elaborate_decl(
        "data GenWitness (index : Nat) : Type where { \
           GenMkWitness : GenWitness index \
         }",
    )
    .expect("GenWitness");

    env.elaborate_decl(
        "fn generated_top_second \
           (index : Nat) \
           (derivation : GenDerivation (GenMkSeq index True)) \
           : GenWitness index = \
         match derivation { \
           GenDeriv actual_index ↦ GenMkWitness index \
         }",
    )
    .expect("generated evidence must close when the Sigma second child is Top");
}

#[test]
fn generated_record_index_reflexivity_closes_top_first_child() {
    // Promise class: durable invariant. The first and second Sigma children are
    // independent recursive dispatch sites; this fixture complements the
    // second-child case above.
    // MEASURED: a real dependent match whose record equality has a closed
    // True/True first child (Top) and a variable Nat second child (still Eq).
    // CLAIMED: generated evidence dispatches its first Sigma child through
    // Eq/Sigma/Top. THE GAP: restoring only the first old recursive call is the
    // independent causality mutation for this fixture.
    let mut env = ElabEnv::new().expect("base env");
    env.elaborate_decl("data GenFirstSeq = GenFirstMkSeq Bool Nat")
        .expect("GenFirstSeq");
    env.elaborate_decl(
        "data GenFirstDerivation : GenFirstSeq -> Type where { \
           GenFirstDeriv : (index : Nat) -> \
             GenFirstDerivation (GenFirstMkSeq True index) \
         }",
    )
    .expect("GenFirstDerivation");
    env.elaborate_decl(
        "data GenFirstWitness (index : Nat) : Type where { \
           GenFirstMkWitness : GenFirstWitness index \
         }",
    )
    .expect("GenFirstWitness");

    env.elaborate_decl(
        "fn generated_top_first \
           (index : Nat) \
           (derivation : GenFirstDerivation (GenFirstMkSeq True index)) \
           : GenFirstWitness index = \
         match derivation { \
           GenFirstDeriv actual_index ↦ GenFirstMkWitness index \
         }",
    )
    .expect("generated evidence must close when the Sigma first child is Top");
}

#[test]
fn bare_user_refl_on_top_level_top_remains_rejected() {
    // Promise class: durable invariant. Extensions may add generated-evidence
    // shapes, but bare user Refl remains accepted only for equality-origin
    // goals; Top alone is not an equality-origin goal.
    // MEASURED: the public theorem-body Refl path on a direct Top goal.
    // CLAIMED: generated evidence's Top support is not exposed through user
    // Refl. THE GAP: this fixture covers the user path; the generated positive
    // above separately proves Top remains available at its intended caller.
    let mut env = ElabEnv::new().expect("base env");
    let error = env
        .elaborate_decl("theorem user_refl_top : Top = Refl")
        .expect_err("bare user Refl must not inhabit a direct Top goal");

    assert!(
        matches!(error, ElabError::TypeMismatch { ref reason, .. }
            if reason == "Refl expects an `Eq`-shaped goal"),
        "bare user Refl on Top must retain the equality-origin diagnostic, got {error:?}"
    );
}

#[test]
fn generated_evidence_rejection_vocabulary_is_not_user_refl_vocabulary() {
    // Promise class: durable invariant. Non-equality result types must remain
    // outside user Refl's vocabulary even while generated evidence gains nested
    // Top closure.
    // MEASURED: user Refl checked against the ordinary carrier Nat, an input
    // outside Eq/Sigma/Top evidence. CLAIMED: the repair does not install a
    // general proof synthesizer. THE GAP: one outside-shape fixture exercises
    // the default rejection arm; Rust review must still confirm no additional
    // production match arm was added.
    let mut env = ElabEnv::new().expect("base env");
    let error = env
        .elaborate_decl("const user_refl_nat : Nat = Refl")
        .expect_err("bare user Refl must not synthesize a Nat");

    assert!(
        matches!(error, ElabError::TypeMismatch { ref reason, .. }
            if reason == "Refl expects an `Eq`-shaped goal"),
        "bare user Refl outside Eq/Sigma/Top must hit the closed default, got {error:?}"
    );
}
