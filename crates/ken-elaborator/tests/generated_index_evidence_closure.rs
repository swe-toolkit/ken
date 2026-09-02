//! `LANG-GENERATED-INDEX-EVIDENCE-CLOSURE` acceptance.
//!
//! Generated dependent-match index evidence is closed under the three shapes it
//! may recursively expose: `Eq`, observational `Sigma`, and `Top`. User-facing
//! `Refl` remains guarded by equality origin and must not inherit generated
//! evidence's `Top` case.

use ken_elaborator::{ElabEnv, ElabError};

#[test]
fn generated_record_index_reflexivity_closes_nested_top_field() {
    // Promise class: durable invariant. Future observational equality changes
    // may alter the nesting, but generated reflexive index evidence must remain
    // closed under every Eq/Sigma/Top child it produces.
    // MEASURED: a real dependent match over a record index whose closed List
    // field makes recursive observational equality reach Top inside Sigma.
    // CLAIMED: generated index evidence handles Eq/Sigma/Top uniformly at all
    // nesting depths. THE GAP: the fixture relies on the record/List equality
    // reduction path continuing to expose the nested Top; restoring the old
    // recursive calls is the causality control for that exact path.
    let mut env = ElabEnv::new().expect("base env");
    env.elaborate_decl("data GenSeq = GenMkSeq (List Nat) (List Nat)")
        .expect("GenSeq");
    env.elaborate_decl(
        "data GenDerivation : GenSeq -> Type where { \
           GenDeriv : (gamma : List Nat) -> \
             GenDerivation \
               (GenMkSeq gamma (Cons Nat Zero (Nil Nat))) \
         }",
    )
    .expect("GenDerivation");
    env.elaborate_decl(
        "data GenWitness (gamma : List Nat) : Type where { \
           GenMkWitness : GenWitness gamma \
         }",
    )
    .expect("GenWitness");

    env.elaborate_decl(
        "fn generated_nested_top \
           (gamma : List Nat) \
           (derivation : GenDerivation \
             (GenMkSeq gamma (Cons Nat Zero (Nil Nat)))) \
           : GenWitness gamma = \
         match derivation { \
           GenDeriv actual_gamma ↦ GenMkWitness gamma \
         }",
    )
    .expect(
        "generated reflexive record-index evidence must close when a nested \
         closed List equality reduces to Top",
    );
}

#[test]
fn generated_record_index_reflexivity_closes_nested_top_first_field() {
    // Promise class: durable invariant. The first and second Sigma children are
    // independent recursive dispatch sites; this fixture puts the closed List
    // equality in the first field, complementing the second-field fixture above.
    // MEASURED: a real dependent match whose first record field equality WHNFs
    // to Top while the result depends on the variable second field. CLAIMED:
    // generated evidence dispatches its first Sigma child through Eq/Sigma/Top.
    // THE GAP: restoring only the first old recursive call is the independent
    // causality mutation for this fixture.
    let mut env = ElabEnv::new().expect("base env");
    env.elaborate_decl("data GenFirstSeq = GenFirstMkSeq (List Nat) (List Nat)")
        .expect("GenFirstSeq");
    env.elaborate_decl(
        "data GenFirstDerivation : GenFirstSeq -> Type where { \
           GenFirstDeriv : (delta : List Nat) -> \
             GenFirstDerivation (GenFirstMkSeq (Nil Nat) delta) \
         }",
    )
    .expect("GenFirstDerivation");
    env.elaborate_decl(
        "data GenFirstWitness (delta : List Nat) : Type where { \
           GenFirstMkWitness : GenFirstWitness delta \
         }",
    )
    .expect("GenFirstWitness");

    env.elaborate_decl(
        "fn generated_nested_top_first \
           (delta : List Nat) \
           (derivation : GenFirstDerivation \
             (GenFirstMkSeq (Nil Nat) delta)) \
           : GenFirstWitness delta = \
         match derivation { \
           GenFirstDeriv actual_delta ↦ GenFirstMkWitness delta \
         }",
    )
    .expect(
        "generated reflexive record-index evidence must close when its first \
         field's closed List equality reduces to Top",
    );
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
