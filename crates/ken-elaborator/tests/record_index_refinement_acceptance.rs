//! `LANG-RECORD-INDEX-REFINEMENT` acceptance — the exact reported case.
//!
//! The dependent-match index-refinement transports (`build_sym`,
//! `build_index_type_cong`, `build_index_omega_transport`) now form their `J`
//! under an ABSTRACT index type and apply at the concrete index (the generic
//! `sym`/`cong`/`subst` pattern). So a family indexed by a RECORD
//! (single-constructor) type — whose observational reflexive equality reduces
//! to a Sigma of field equalities — can be ELIMINATED with index refinement,
//! not only a `Nat`-indexed family.
//!
//! The reported failure was `FokDerivation : FokSequent -> Type` (`FoKripke.ken`,
//! unchanged here): every attempt to invert it toward a `FokMkSequent gamma
//! delta` target reddened with a kernel type mismatch, while the bare-variable
//! constant-motive control elaborated. These tests exercise that exact family.

#[path = "support/catalog_or.rs"]
mod catalog_or;

use ken_elaborator::ElabEnv;

const FOK_SOURCE: &str =
    include_str!("../../../catalog/packages/Tooling/Verification/FoKripke.ken");

fn fok_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("base env");
    catalog_or::load_core_logic_or(&mut env);
    env.elaborate_file(FOK_SOURCE)
        .expect("FoKripke.ken must elaborate unchanged");
    env
}

#[test]
fn ac1_constant_motive_match_over_the_record_indexed_derivation_elaborates() {
    // AC-1: the EXACT reported record-index constant-motive match — invert
    // `FokDerivation (FokMkSequent gamma delta)` by matching all four
    // constructors, result `Nat`. Red before the fix (the specialized-at-
    // FokSequent J/Refl could not recover the `Eq` head after the record
    // equality reduced to a Sigma); green now.
    let mut env = fok_env();
    env.elaborate_decl(
        "fn fok_ac1_probe (gamma : List FokForm) (delta : List FokForm) \
           (d : FokDerivation (FokMkSequent gamma delta)) : Nat = \
         match d { \
           FokDerivInit g2 d2 left right g dd h1 h2 h3 ↦ left; \
           FokDerivImpRight g2 d2 right p q h child ↦ right; \
           FokDerivForallWorldRight g2 d2 right eigen body h1 h2 child ↦ eigen; \
           FokDerivForallObjRight g2 d2 right eigen body h1 h2 child ↦ eigen }",
    )
    .expect("the record-index constant-motive FokDerivation match must now elaborate");
}

#[test]
fn ac3_inversion_probe_reaches_the_arm_body_using_constructor_fields() {
    // AC-3 / AC-2: the D2b inversion shape — the arm bodies USE the
    // constructor's own fields/recursive evidence (a read of a helper term is
    // not enough). Each ForallRight arm returns the `right` looked-up index and
    // recurses need not close adequacy inside the predecessor; reaching and
    // kernel-checking the arm bodies over the unchanged `FokDerivation` is the
    // obligation.
    let mut env = fok_env();
    env.elaborate_decl(
        "fn fok_ac3_probe (gamma : List FokForm) (delta : List FokForm) \
           (d : FokDerivation (FokMkSequent gamma delta)) : Option FokForm = \
         match d { \
           FokDerivInit g2 d2 left right g dd h1 h2 h3 ↦ Some FokForm g; \
           FokDerivImpRight g2 d2 right p q h child ↦ Some FokForm (FokImp p q); \
           FokDerivForallWorldRight g2 d2 right eigen body h1 h2 child ↦ \
             Some FokForm (FokForallWorld body); \
           FokDerivForallObjRight g2 d2 right eigen body h1 h2 child ↦ \
             Some FokForm (FokForallObj body) }",
    )
    .expect("the inversion probe must reach and check its arm bodies");
}

#[test]
fn control_bare_variable_index_still_elaborates() {
    // The narrower control the report states: matching `FokDerivation s` at a
    // BARE variable index with a constant motive always elaborated, and must
    // still — this fix widens the supported set, it does not disturb the
    // already-working case.
    let mut env = fok_env();
    env.elaborate_decl(
        "fn fok_bare_probe (s : FokSequent) (d : FokDerivation s) : Nat = \
         match d { \
           FokDerivInit g2 d2 left right g dd h1 h2 h3 ↦ left; \
           FokDerivImpRight g2 d2 right p q h child ↦ right; \
           FokDerivForallWorldRight g2 d2 right eigen body h1 h2 child ↦ eigen; \
           FokDerivForallObjRight g2 d2 right eigen body h1 h2 child ↦ eigen }",
    )
    .expect("the bare-variable-index control must still elaborate");
}
