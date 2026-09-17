//! `V3-FO-SORTED-EIGENPARAMETER-DERIVATION` `AC-1`: the historical refuting
//! certificate, reconciled to the corrected parameter-only representation.
//!
//! Provenance. The refutation that motivated the FO soundness repair
//! (`evt_2yh515wg0mczy`, base `ef91b8225`) was a certificate for the embed image
//! of `forall x : A. forall y : A. P x -> P y` -- a source form that is NOT
//! valid -- built by the old, unsorted `ForallRight`. Its recorded certificate
//! hash was `db1bf51e9434307d587fbf9cd565af1343cbd877831ff2477f857d5a740779a8`.
//! The recorded 14-step tree instantiated the two OBJECT quantifiers with BOUND
//! references (`ForallR Bound5`, `ForallR Bound3`) into outer WORLD binders --
//! the exact "invent an object-sort inhabitant" exploit.
//!
//! Reconciliation (language-leader ruling `evt_k7x6nmxy4ydy`/`evt_6t3hrfekx34dw`).
//! Under the released representation the exploit is rejected AT CONSTRUCTION, a
//! strictly stronger closure than a checker `False`: the eigen is a parameter
//! INDEX, so a bound object eigen has NO constructor encoding on either surface.
//! The recorded hash is retained above as historical provenance only, not as a
//! reproducibility claim -- the object it hashed cannot be built in the
//! corrected calculus, and no certificate-hash mechanism is in-tree.
//!
//! Promise class: durable invariant. This pins (1) the genuine `fok_embed`
//! image of the exact source form, (2) corrected-search NONEXISTENCE of any
//! certificate for it, and (3) constructor-level unrepresentability of a bound
//! eigen on both surfaces.

#[path = "support/catalog_or.rs"]
mod catalog_or;

use ken_elaborator::fo_kripke::{declare_fo_slice_signature, embed, find_certificate, Form, IForm, IVar};
use ken_elaborator::ElabEnv;
use ken_kernel::GlobalEnv;

const FOK_SOURCE: &str =
    include_str!("../../../catalog/packages/Tooling/Verification/FoKripke.ken");

/// The exact refuting source form: `forall x : A. forall y : A. P x -> P y`.
/// Inside both object binders, `x = IVar(1)` (outer) and `y = IVar(0)` (inner).
fn refuting_source_form() -> IForm {
    IForm::Forall(Box::new(IForm::Forall(Box::new(IForm::Imp(
        Box::new(IForm::Atom(IVar(1))),
        Box::new(IForm::Atom(IVar(0))),
    )))))
}

/// The planted comparable witness: `forall x : A. forall y : A. P x -> P x`.
/// Identical quantifier/implication SHAPE to the refuting form (two object
/// binders, one implication between predicate atoms), but VALID -- `P x -> P x`
/// holds, so its embed image IS derivable with a lawful fresh-parameter eigen.
/// `AC-1-POWER`: the corrected search must FIND this, or the nonexistence result
/// above is vacuous (a search inert on every input proves nothing).
fn comparable_valid_form() -> IForm {
    IForm::Forall(Box::new(IForm::Forall(Box::new(IForm::Imp(
        Box::new(IForm::Atom(IVar(1))),
        Box::new(IForm::Atom(IVar(1))),
    )))))
}

#[test]
fn corrected_search_finds_no_certificate_for_the_refuting_embed_image() {
    let mut env = GlobalEnv::new();
    let _sig = declare_fo_slice_signature(&mut env);
    let f = refuting_source_form();

    // (1) The target is a genuine `fok_embed` image: `embed` produces the
    // canonical `Imp(K(Sigma), forall w : World. w |= f)` shape, not a
    // hand-built term outside embed's range.
    let target = embed(&f);
    match &target {
        Form::Imp(_k_sigma, consequent) => assert!(
            matches!(**consequent, Form::ForallWorld(_)),
            "embed's consequent must be `forall w : World. ...`, got {consequent:?}"
        ),
        other => panic!("embed must produce `Imp(K(Sigma), ...)`, got {other:?}"),
    }

    // (2) The corrected search finds NO certificate for that embed image --
    // demonstrated by RUNNING the decision procedure, not by argument. The old
    // search accepted the 14-step bound-eigen tree here; the corrected search
    // (whose ForallRight only ever emits a fresh PARAMETER eigen) cannot.
    assert!(
        find_certificate(&f).is_none(),
        "the corrected search must find no certificate for the refuting source form"
    );
}

#[test]
fn corrected_search_finds_a_planted_comparable_witness() {
    // AC-1-POWER (Rust). The nonexistence above is only meaningful if the search
    // can find SOMETHING: a search inert on every input satisfies it vacuously.
    // A valid form of identical shape -- `forall x. forall y. P x -> P x` -- is
    // derivable, and the corrected search (whose ForallRight emits a fresh
    // PARAMETER eigen) finds a certificate for it.
    let mut env = GlobalEnv::new();
    let _sig = declare_fo_slice_signature(&mut env);
    let g = comparable_valid_form();
    match &embed(&g) {
        Form::Imp(_k_sigma, consequent) => assert!(
            matches!(**consequent, Form::ForallWorld(_)),
            "the witness target must also be a genuine embed image"
        ),
        other => panic!("embed must produce `Imp(K(Sigma), ...)`, got {other:?}"),
    }
    assert!(
        find_certificate(&g).is_some(),
        "the corrected search must FIND the planted comparable valid witness -- \
         otherwise the exploit's nonexistence result is vacuous"
    );
}

fn fok_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("base env");
    catalog_or::load_core_logic_or(&mut env);
    env.elaborate_file(FOK_SOURCE)
        .expect("FoKripke.ken must elaborate");
    env
}

#[test]
fn bound_object_eigen_step_has_no_constructor_encoding_on_the_ken_surface() {
    // (3) Ken surface: `FokForallRight`'s eigen is a parameter INDEX (`Nat`).
    // The historical `ForallR Bound5`/`ForallR Bound3` steps -- a `FokQTerm`
    // (here a `FokQBound`/`FokQParameter`) in the eigen slot -- therefore have
    // no constructor encoding: supplying one is a type error, not a checker
    // `False`. Positive control: the parameter-index form elaborates.
    let mut env = fok_env();
    env.elaborate_decl("const fok_ac1_param_eigen_ok : FokRule = FokForallRight Zero (Suc Zero)")
        .expect("a parameter-index eigen is the representable form and must elaborate");

    let mut env2 = fok_env();
    assert!(
        env2.elaborate_decl(
            "const fok_ac1_bound_eigen : FokRule = FokForallRight Zero (FokQBound (Suc Zero))"
        )
        .is_err(),
        "a bound (FokQTerm) eigen must NOT elaborate as a FokForallRight -- the eigen slot is Nat"
    );

    let mut env3 = fok_env();
    assert!(
        env3.elaborate_decl(
            "const fok_ac1_qparam_eigen : FokRule = FokForallRight Zero (FokQParameter Zero)"
        )
        .is_err(),
        "even a FokQParameter term must NOT elaborate in the eigen slot -- it holds a Nat index, \
         so no QTerm (bound or otherwise) is a well-typed eigen"
    );
}

/// True iff `Equal Bool (fok_check_tree sequent cert) expected` is provable by
/// `Proved` -- i.e. the Ken checker returns `expected` on this certificate.
fn ken_verdict_is(env: &mut ElabEnv, name: &str, sequent: &str, cert: &str, expected: &str) -> bool {
    env.elaborate_decl(&format!(
        "theorem {name} : Equal Bool (fok_check_tree ({sequent}) ({cert})) {expected} = Proved"
    ))
    .is_ok()
}

/// A representable `FokForallObj`-right certificate over a Bound-0-free,
/// well-sorted object body (`FokQParameter 0` in the object slot, `1` in the
/// world slot). This is the LAWFUL analog of the exploit's object step:
/// `ForallR Bound5`/`Bound3` used a bound eigen, this uses a parameter INDEX.
/// The single child closes by `Init` on the trailing `FokBottom`, so it matches
/// under any eigen (`subst0` is a no-op on this body) -- leaving the checker's
/// own guards as the only discriminator.
fn ken_forall_obj_cert(eigen_ix: &str) -> (String, String) {
    let gamma = "Cons FokForm FokBottom (Nil FokForm)";
    let body = "FokForcingP (FokQParameter (Suc Zero)) (FokQParameter Zero)";
    let delta = format!("Cons FokForm (FokForallObj ({body})) (Cons FokForm FokBottom (Nil FokForm))");
    let sequent = format!("FokMkSequent ({gamma}) ({delta})");
    let child_delta =
        format!("fok_list_form_set_nth ({delta}) Zero (fok_subst0_form ({body}) (FokQParameter ({eigen_ix})))");
    let child_cert =
        format!("FokMkCert (FokMkSequent ({gamma}) ({child_delta})) (FokInit Zero (Suc Zero)) (Nil FokCert)");
    let cert = format!(
        "FokMkCert ({sequent}) (FokForallRight Zero ({eigen_ix})) (Cons FokCert ({child_cert}) (Nil FokCert))"
    );
    (sequent, cert)
}

#[test]
fn ken_checker_accepts_a_planted_comparable_cert_and_refuses_a_near_miss() {
    // AC-1-POWER (Ken). Constructor-unrepresentability alone cannot distinguish
    // "the exploit is unconstructible" from "nothing constructible checks". So
    // the Ken surface must exhibit a REPRESENTABLE comparable certificate that
    // the checker ACCEPTS (power), and a representable near-miss it REFUSES
    // (discrimination) -- both on this surface, no Rust result standing in.

    // Power: a lawful FokForallObj step with a FRESH parameter (`2`) eigen.
    let (accept_seq, accept_cert) = ken_forall_obj_cert("Suc (Suc Zero)");
    let mut env = fok_env();
    // Steward ruling: a representable comparable certificate that fails to check
    // is a HARD STOP to Steward+Architect (the corrected relation would reject
    // something it should admit) -- NOT a reason to weaken this assertion.
    assert!(
        ken_verdict_is(&mut env, "ken_ac1_power_accept", &accept_seq, &accept_cert, "True"),
        "HARD-STOP signal: the planted representable comparable certificate must be \
         ACCEPTED by fok_check_tree; if this reds, escalate to Steward + Architect"
    );

    // Discrimination: the same shape with a NON-FRESH parameter (`0`, already in
    // the body) is representable and REFUSED -- so the acceptance above is not
    // vacuous (the checker is not accepting everything).
    let (reject_seq, reject_cert) = ken_forall_obj_cert("Zero");
    let mut env2 = fok_env();
    assert!(
        ken_verdict_is(&mut env2, "ken_ac1_near_miss_reject", &reject_seq, &reject_cert, "False"),
        "the representable non-fresh near-miss must be REFUSED downstream"
    );
    let mut env3 = fok_env();
    assert!(
        !ken_verdict_is(&mut env3, "ken_ac1_near_miss_not_accept", &reject_seq, &reject_cert, "True"),
        "the near-miss must not also be accepted"
    );
}
