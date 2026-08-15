//! `V3-FO-OBLIGATION-SIGNATURE-DISCOVERY` `D1`-`D4` acceptance: the public
//! `attempt_obligation`/`attempt_fo` route reaches the Kripke slice boundary
//! on a REAL, externally-constructed obligation -- one built from
//! independently-declared postulates, never from
//! `fo_kripke::declare_fo_slice_signature`, which is exactly the mechanism
//! `V3-FO-KRIPKE-SLICE`'s own inert-probe defect used and which this node
//! replaces for the public route.
//!
//! Test structure mirrors `v3_acceptance.rs`/`v3_fo_kripke_slice_acceptance.rs`:
//! kernel-level term construction, no elaborator surface syntax.
//!
//! **The oracle used throughout:** `V3-FO-OBLIGATION-SIGNATURE-DISCOVERY`
//! `D5`'s two distinct `trusted_base()` labels. An `Unknown` verdict whose
//! hole carries the FO-specific "... theorem-home unapproved ..." label
//! reached `attempt_fo_with_signature`'s accepted-certificate branch --
//! i.e. discovery succeeded, quotation succeeded, preservation was
//! established, and a certificate was found and accepted. An `Unknown`
//! verdict whose hole carries the ordinary "prover unknown goal" label
//! reached the IPC fallback instead -- i.e. discovery or quotation refused
//! (or no certificate was found). This is exactly the instrument `D5` was
//! built to be, now exercised by a route that can actually reach it.

use ken_elaborator::{
    extract::{ObligationId, ObligationTriple, ProvKind, Provenance},
    fo_kripke::{check_cert, discover_and_quote_fo, embed, find_certificate},
    prover::Route,
    attempt_obligation, classify,
    error::Span,
    prover::Verdict,
};
use ken_kernel::{declare_postulate, subst::shift, Decl, GlobalEnv, GlobalId, Level, Term};

const FO_WITHHELD_MARKER: &str = "theorem-home unapproved";
const ORDINARY_LABEL: &str = "prover unknown goal";

/// Declare a sort `A : Type 0` and predicate `P : A -> Omega 0` as ordinary,
/// independent postulates -- simulating a real program's own declarations,
/// never `fo_kripke::declare_fo_slice_signature`'s prover-owned ones.
fn declare_real_program_signature(env: &mut GlobalEnv) -> (Term, GlobalId) {
    let a_id = declare_postulate(env, "user sort A".to_string(), vec![], Term::Type(Level::zero()))
        .expect("declare sort A");
    let sort_a = Term::const_(a_id, vec![]);
    let pred_id = declare_postulate(
        env,
        "user predicate P".to_string(),
        vec![],
        Term::pi(sort_a.clone(), Term::Omega(Level::zero())),
    )
    .expect("declare predicate P");
    (sort_a, pred_id)
}

/// `forall x : A. P x => P x` -- the same shape as
/// `fo_kripke::positive_control_term`, built here from independent,
/// externally-declared `GlobalId`s instead.
fn real_positive_obligation(sort_a: &Term, pred_id: GlobalId) -> Term {
    let px = Term::app(Term::const_(pred_id, vec![]), Term::Var(0));
    let px_at_codomain = shift(&px, 1, 0);
    Term::pi(sort_a.clone(), Term::pi(px, px_at_codomain))
}

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

fn hole_label(env: &GlobalEnv, hole_id: GlobalId) -> String {
    match env.lookup(hole_id) {
        Some(Decl::Opaque { name, .. }) => name.clone(),
        other => panic!("expected Decl::Opaque for a prover hole, got {other:?}"),
    }
}

/// `D2`: the public route quotes an externally-constructed positive
/// control, finds its certificate, and arrives at the `23 §4.4` boundary --
/// demonstrated by the FO-specific label, not the ordinary one, appearing
/// on the resulting `Unknown` hole.
#[test]
fn real_positive_obligation_reaches_fo_boundary_through_public_route() {
    let mut env = GlobalEnv::new();
    let (sort_a, pred_id) = declare_real_program_signature(&mut env);
    let phi = real_positive_obligation(&sort_a, pred_id);

    // classify must route this through FO for the public route to exercise
    // `attempt_fo` at all.
    assert_eq!(classify(&env, &phi), Route::FO, "classify must route this obligation to FO");

    let triple = closed_triple(&mut env, "d2.positive", phi);
    let result = attempt_obligation(&mut env, &triple);

    let hole_id = match result.verdict {
        Verdict::Unknown { hole_id } => hole_id,
        other => panic!("D2 obligation must yield Unknown (23 §4.4 still forbids Proved), got {other:?}"),
    };
    let label = hole_label(&env, hole_id);
    assert!(
        label.contains(FO_WITHHELD_MARKER),
        "the hole must carry the FO-withheld label -- proof the public route \
         reached the 23 §4.4 boundary via discovery, not the IPC fallback; got {label:?}"
    );
}

/// `D3`/`AC-5`: the fail-safe demonstrated THROUGH the public `attempt_fo`
/// route -- an obligation whose certificate `check_cert` genuinely accepts
/// still yields `Unknown`, never `Proved`. Acceptance is asserted as a
/// PRECONDITION, independently of the public route, so this test cannot
/// pass by accident via a route that never reached a real certificate.
#[test]
fn accepted_certificate_through_public_route_yields_unknown_never_proved() {
    let mut env = GlobalEnv::new();
    let (sort_a, pred_id) = declare_real_program_signature(&mut env);
    let phi = real_positive_obligation(&sort_a, pred_id);

    // Precondition, not decoration: independently establish that discovery
    // succeeds, quotation succeeds, and the slice's own search finds and
    // accepts a genuine certificate for this exact obligation -- BEFORE
    // routing it through the public entry point.
    let (sig, problem) =
        discover_and_quote_fo(&env, &phi).expect("discovery must succeed for this obligation");
    let cert = find_certificate(&problem.f).expect("a certificate must be found");
    let target = embed(&problem.f);
    assert!(
        check_cert(&target, &cert),
        "precondition failed: the certificate must compute True for this \
         test to be exercising the D3 boundary at all"
    );
    let _ = sig; // discovered signature reconfirmed only for the precondition above

    let triple = closed_triple(&mut env, "d3.accepted", phi);
    let result = attempt_obligation(&mut env, &triple);

    match result.verdict {
        Verdict::Unknown { hole_id } => {
            let label = hole_label(&env, hole_id);
            assert!(
                label.contains(FO_WITHHELD_MARKER),
                "an accepted certificate reached through the public route must \
                 land on the FO-withheld exit specifically, got {label:?}"
            );
        }
        other => panic!(
            "an accepted slice certificate must still yield Unknown through the \
             public route, never Proved, until the theorem-home placement \
             decision is made (23 §4.4) -- got {other:?}"
        ),
    }
}

/// `D0` conjunct 1 / `AC-1`: an obligation using TWO distinct declared
/// sorts has no unambiguous role assignment. Discovery must REFUSE, not
/// guess one -- demonstrated both directly (`discover_and_quote_fo`
/// returns `None`) and through the public route (falls through to the
/// ordinary IPC-fallback label, never the FO-specific one).
#[test]
fn ambiguous_two_sort_obligation_is_refused_by_discovery_not_guessed() {
    let mut env = GlobalEnv::new();
    let (sort_a, pred_id) = declare_real_program_signature(&mut env);
    let b_id = declare_postulate(&mut env, "user sort B".to_string(), vec![], Term::Type(Level::zero()))
        .expect("declare sort B");
    let sort_b = Term::const_(b_id, vec![]);

    // `forall x:A. forall y:B. P x => P x` -- two distinct sort candidates.
    let px = Term::app(Term::const_(pred_id, vec![]), Term::Var(1));
    let px_at_codomain = shift(&px, 1, 0);
    let phi = Term::pi(
        sort_a.clone(),
        Term::pi(sort_b, Term::pi(px, px_at_codomain)),
    );

    assert!(
        discover_and_quote_fo(&env, &phi).is_none(),
        "discovery must refuse an obligation with two ambiguous sort candidates"
    );

    let triple = closed_triple(&mut env, "ac1.ambiguous", phi);
    let result = attempt_obligation(&mut env, &triple);
    let hole_id = match result.verdict {
        Verdict::Unknown { hole_id } => hole_id,
        other => panic!("ambiguous obligation must not be Proved, got {other:?}"),
    };
    let label = hole_label(&env, hole_id);
    assert_eq!(
        label, ORDINARY_LABEL,
        "an ambiguous signature must be refused all the way through to the \
         ordinary IPC-fallback label, not guessed into the FO boundary"
    );
}

/// `D4`: an obligation is refused, not widened. `P` applied to something
/// other than a bare in-scope object `Var` (`f x`, not `x`) is outside
/// `quote_iform`'s own atom grammar -- unchanged by this node -- and
/// discovery must not create a new way past it.
#[test]
fn predicate_applied_to_non_var_argument_is_refused_not_widened() {
    let mut env = GlobalEnv::new();
    let (sort_a, pred_id) = declare_real_program_signature(&mut env);
    let f_id = declare_postulate(
        &mut env,
        "user function f".to_string(),
        vec![],
        Term::pi(sort_a.clone(), sort_a.clone()),
    )
    .expect("declare f : A -> A");

    // `forall x:A. P (f x)` -- the atom's argument is `f x`, not a bare Var.
    let fx = Term::app(Term::const_(f_id, vec![]), Term::Var(0));
    let p_fx = Term::app(Term::const_(pred_id, vec![]), fx);
    let phi = Term::pi(sort_a, p_fx);

    assert!(
        discover_and_quote_fo(&env, &phi).is_none(),
        "an atom applied to a non-Var argument must be refused, not accepted \
         through a discovered signature"
    );

    let triple = closed_triple(&mut env, "d4.non_var_atom", phi);
    let result = attempt_obligation(&mut env, &triple);
    match result.verdict {
        Verdict::Unknown { hole_id } => {
            let label = hole_label(&env, hole_id);
            assert_eq!(
                label, ORDINARY_LABEL,
                "an out-of-grammar obligation must land on the ordinary label, \
                 never the FO-withheld one"
            );
        }
        other => panic!("an out-of-slice obligation must never be Proved, got {other:?}"),
    }
}
