//! `V3-FO-KRIPKE-SLICE` acceptance: the first route-(a) vertical slice of
//! the FO Kripke embedding (`23-prover.md §4.5`), through the public
//! `ken_elaborator::fo_kripke`/`ken_elaborator::prover` APIs.
//!
//! Mirrors `v3_acceptance.rs`'s own structure (kernel-level term
//! construction, no elaborator surface syntax) and REPLACES that file's
//! `kripke_embedding_cert_rechecks_fo_placeholder` (`[placeholder — reifies
//! in V4]`) with a real exercise of the slice this node builds.

use ken_elaborator::{
    extract::{ObligationId, ObligationTriple, ProvKind, Provenance},
    fo_kripke::{
        check_cert, declare_fo_slice_signature, embed, find_certificate,
        negative_control_term, positive_control_term, quote_fo, FoBoundary,
    },
    prover::{attempt_fo_with_signature, Verdict},
    error::Span,
};
use ken_kernel::{Context, GlobalEnv, Level, Term};

fn closed_triple(env: &mut GlobalEnv, id: &str, phi: Term) -> ObligationTriple {
    let placeholder_hole = env.fresh_id();
    ObligationTriple {
        id: ObligationId(id.to_owned()),
        hole_id: placeholder_hole,
        context: vec![],
        phi: phi.clone(),
        goal_closed: phi,
        provenance: Provenance { kind: ProvKind::Prove, span: Span::zero() },
    }
}

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
/// certificate -- demonstrated by running the search.
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

/// `D5`/`AC-5`/`AC-6`: `attempt_fo`'s core wiring, exercised through
/// `attempt_fo_with_signature` (the caller-supplied-signature variant --
/// `attempt_fo` itself mints a fresh, unpredictable signature per call, so
/// this is how the boundary behavior is verified from outside `prover.rs`).
/// Even though the positive control's certificate is genuinely accepted by
/// `check_cert`, the verdict must be `Unknown`, never `Proved` -- the
/// theorem-home placement decision (`23 §4.4`) has not been made.
#[test]
fn attempt_fo_returns_unknown_never_proved_for_accepted_slice_certificate() {
    let mut env = GlobalEnv::new();
    let sig = declare_fo_slice_signature(&mut env);
    let positive = positive_control_term(&sig);

    // Sanity: this obligation's certificate really is accepted by the
    // slice's own check_cert (otherwise this test would trivially pass for
    // the wrong reason -- the fallback path, not the D5 boundary).
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

/// `D5`: the negative control (no certificate at all) also returns
/// `Unknown` via the IPC fallback, never `Proved` -- unaffected by whether
/// the classical-only reasoning that would forge a certificate is present.
#[test]
fn attempt_fo_returns_unknown_for_the_negative_control() {
    let mut env = GlobalEnv::new();
    let sig = declare_fo_slice_signature(&mut env);
    let negative = negative_control_term(&env, &sig);
    let triple = closed_triple(&mut env, "fo-negative-control", negative.clone());

    let verdict =
        attempt_fo_with_signature(&mut env, &Context::new(), &triple.phi, &triple.goal_closed, &sig);
    assert!(
        matches!(verdict, Verdict::Unknown { .. }),
        "the negative control must never be Proved, got {verdict:?}"
    );
}
