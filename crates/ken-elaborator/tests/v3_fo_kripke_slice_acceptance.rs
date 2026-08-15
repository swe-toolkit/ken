//! `V3-FO-KRIPKE-SLICE` acceptance: the first route-(a) vertical slice of
//! the FO Kripke embedding (`23-prover.md §4.5`), through the public
//! `ken_elaborator::fo_kripke`/`ken_elaborator::prover` APIs.
//!
//! Mirrors `v3_acceptance.rs`'s own structure (kernel-level term
//! construction, no elaborator surface syntax) and REPLACES that file's
//! `kripke_embedding_cert_rechecks_fo_placeholder` (`[placeholder — reifies
//! in V4]`) with a real exercise of the slice this node builds.
//!
//! **Scope, stated so it is not lost (Steward disposition on the QA block of
//! `e0474679c`, `D5`/`AC-5`/`AC-1`):** every claim in this file about the
//! Kripke-slice boundary (`D5`/`AC-5`/`AC-6`, and the negative control's
//! `AC-1`) is a claim about `prover::attempt_fo_with_signature`, the
//! caller-supplied-signature entry point -- NEVER about the public
//! `prover::attempt_fo`/`attempt_obligation` route. `attempt_fo` mints a
//! fresh `FoSliceSignature` on every call, so no externally-constructed
//! obligation can ever quote against it; every real obligation refuses
//! quotation and falls straight through to the unchanged IPC fallback, and
//! a test run through that route would measure the fallback, not this
//! slice, while printing the same `Unknown` word either way. Route FO is
//! built and checked here, but currently unreachable in production. No
//! behavioral change to the public prover route is claimed anywhere in this
//! file.

use ken_elaborator::{
    fo_kripke::{
        check_cert, declare_fo_slice_signature, embed, find_certificate,
        negative_control_term, positive_control_term, quote_fo, FoBoundary,
    },
    prover::{attempt_fo_with_signature, Verdict},
};
use ken_kernel::{Context, GlobalEnv, Level, Term};

/// `D1`/`AC-3`: both controls quote; a form outside the slice is refused
/// by construction.
#[test]
fn quote_fo_accepts_both_controls_refuses_outside_the_slice() {
    let mut env = GlobalEnv::new();
    let sig = declare_fo_slice_signature(&mut env);

    let positive = positive_control_term(&sig);
    let negative = negative_control_term(&env, &sig);

    assert!(quote_fo(&env, &sig, &positive).is_ok(), "positive control must quote");
    assert!(quote_fo(&env, &sig, &negative).is_ok(), "negative control must quote");

    let out_of_slice = Term::Type(Level::zero());
    assert_eq!(
        quote_fo(&env, &sig, &out_of_slice).err(),
        Some(FoBoundary::UnsupportedTermShape),
        "a form outside the slice must be refused by construction"
    );
}

/// `D3`/`AC-2`: the positive certificate computes to `True` under
/// `check_cert` -- executed, not merely typed.
#[test]
fn positive_control_certificate_computes_true() {
    let mut env = GlobalEnv::new();
    let sig = declare_fo_slice_signature(&mut env);
    let positive = positive_control_term(&sig);
    let problem = quote_fo(&env, &sig, &positive).expect("positive control quotes");
    let cert = find_certificate(&problem.f).expect("a positive certificate must be found");
    let target = embed(&problem.f);
    assert!(check_cert(&target, &cert), "check_cert must compute True for the positive cert");
}

/// `D4`/`AC-1`: the negative control does NOT obtain an accepted
/// certificate -- demonstrated by running the search directly against
/// `quote_fo`/`find_certificate` with an explicit signature, per the
/// Steward's `D5`/`AC-1` disposition on `e0474679c`: a negative run through
/// `attempt_fo`/`attempt_fo_with_signature` would be inert (the IPC
/// fallback also fails to prove this goal, for its own unrelated reason --
/// it does not handle `or` at all -- so an `Unknown` from that route would
/// not discriminate this slice's own behavior). This is the canonical
/// `AC-1` evidence.
#[test]
fn negative_control_obtains_no_certificate() {
    let mut env = GlobalEnv::new();
    let sig = declare_fo_slice_signature(&mut env);
    let negative = negative_control_term(&env, &sig);
    let problem = quote_fo(&env, &sig, &negative).expect("negative control quotes");
    assert!(
        find_certificate(&problem.f).is_none(),
        "the negative control must not obtain a certificate from init/imp-right/forall-right"
    );
}

/// `D2`/`AC-4`: `K(Sigma)` sits inside `embed`'s target, not emitted as an
/// external frame/forcing premise.
#[test]
fn embed_places_k_sigma_inside_the_target() {
    let mut env = GlobalEnv::new();
    let sig = declare_fo_slice_signature(&mut env);
    let positive = positive_control_term(&sig);
    let problem = quote_fo(&env, &sig, &positive).expect("positive control quotes");
    let target = embed(&problem.f);
    let repr = format!("{target:?}");
    assert!(
        repr.starts_with("Imp("),
        "embed's target must be Imp(K(Sigma), forall w. w|=f), got {repr}"
    );
}

/// `D5`/`AC-5`/`AC-6`: the fail-safe, exercised through
/// `attempt_fo_with_signature` -- a claim about THAT function, never about
/// the public `attempt_fo` (see this file's own header and
/// `attempt_fo_with_signature`'s doc comment in `prover.rs` for why a claim
/// about the public route would be inert). The explicit `check_cert`
/// precondition below is load-bearing, not decoration: without it, this
/// test would pass identically via the IPC fallback and prove nothing about
/// the boundary it names (exactly the failure mode QA's block on
/// `e0474679c` caught one level up, at the public route).
#[test]
fn attempt_fo_with_signature_returns_unknown_never_proved_for_accepted_slice_certificate() {
    let mut env = GlobalEnv::new();
    let sig = declare_fo_slice_signature(&mut env);
    let positive = positive_control_term(&sig);

    // Precondition, not decoration: this obligation's certificate really is
    // accepted by the slice's own check_cert. Without this assertion, the
    // test below could pass via the IPC fallback instead of the D5
    // boundary, and nothing would distinguish the two.
    let problem = quote_fo(&env, &sig, &positive).expect("positive control quotes");
    let cert = find_certificate(&problem.f).expect("a positive certificate must be found");
    let target = embed(&problem.f);
    assert!(
        check_cert(&target, &cert),
        "precondition failed: the positive certificate must compute True \
         for this test to be exercising the D5 boundary at all"
    );

    let verdict = attempt_fo_with_signature(&mut env, &Context::new(), &positive, &positive, &sig);
    assert!(
        matches!(verdict, Verdict::Unknown { .. }),
        "an accepted slice certificate must still yield Unknown, never \
         Proved, until the theorem-home placement decision is made \
         (23 §4.4) -- got {verdict:?}"
    );
}
