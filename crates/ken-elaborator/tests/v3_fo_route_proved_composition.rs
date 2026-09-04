//! Acceptance for `V3-FO-ROUTE-PROVED-COMPOSITION`.
//!
//! Promise class: durable soundness invariants. Rust checker acceptance is
//! necessary but insufficient: only the kernel-checked checker-soundness then
//! embedding-adequacy composite may authorize `Verdict::Proved`.

#[path = "support/catalog_or.rs"]
mod catalog_or;

use std::collections::BTreeSet;

use ken_elaborator::{
    attempt_obligation_with_catalog_handles,
    fo_kripke::{
        check_cert, discover_and_quote_fo_with_catalog, embed, find_certificate,
        kernel_checked_fo_composite, negative_control_term, positive_control_term, quote_fo,
        FoCatalogHandles, FoSliceSignature,
    },
    prover::{attempt_fo_with_signature, Verdict},
    v2_extract, ElabEnv,
};
use ken_kernel::{check, Context, Decl, GlobalId, Level, Term};

const FOK_SOURCE: &str =
    include_str!("../../../catalog/packages/Tooling/Verification/FoKripke.ken");
const FO_WITHHELD_MARKER: &str = "theorem-home unapproved";
const ORDINARY_LABEL: &str = "prover unknown goal";

fn env_with_fok() -> ElabEnv {
    let mut env = ElabEnv::new().expect("base environment");
    catalog_or::load_core_logic_or(&mut env);
    env.elaborate_file(FOK_SOURCE)
        .expect("FoKripke catalog unit must elaborate");
    env
}

fn add_predicate(env: &mut ElabEnv) -> GlobalId {
    env.elaborate_decl(
        "fn d3_pred (x : Bool) : Omega = \
         match x { True ↦ Top; False ↦ Bottom }",
    )
    .expect("D3 predicate")
}

fn installed_signature(env: &ElabEnv, pred_p: GlobalId) -> FoSliceSignature {
    FoSliceSignature {
        sort_a: Term::indformer(env.globals["Bool"], vec![]),
        pred_p,
        or_id: env.globals["Core.Logic.Or.Or"],
        catalog: None,
    }
    .with_catalog_globals(&env.globals)
    .expect("complete FoKripke catalog installation")
}

fn hole_label(env: &ken_kernel::GlobalEnv, hole_id: GlobalId) -> String {
    match env.lookup(hole_id) {
        Some(Decl::Opaque { name, .. }) => name.clone(),
        other => panic!("expected opaque prover hole, got {other:?}"),
    }
}

/// `AC-1`/`AC-3`/`AC-6`: the exact native composition is returned, checks
/// against the independently constructed obligation, and adds no trust.
#[test]
fn accepted_certificate_returns_exact_kernel_checked_composite() {
    let mut env = env_with_fok();
    let pred_p = add_predicate(&mut env);
    let sig = installed_signature(&env, pred_p);
    let phi_closed = positive_control_term(&sig);
    let problem = quote_fo(&env.env, &sig, &phi_closed).expect("accepted FO quotation");
    let rust_cert = find_certificate(&problem.f).expect("slice certificate");
    assert!(
        check_cert(&embed(&problem.f), &rust_cert),
        "positive control must reach Rust checker acceptance"
    );

    let expected = kernel_checked_fo_composite(&env.env, &sig, &problem, &rust_cert, &phi_closed)
        .expect("checker-soundness then adequacy must compose for the obligation");
    let before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();

    let verdict = attempt_fo_with_signature(
        &mut env.env,
        &Context::new(),
        &phi_closed,
        &phi_closed,
        &sig,
    );
    let returned = match verdict {
        Verdict::Proved { cert } => cert,
        other => panic!("kernel-checkable accepted certificate must be Proved, got {other:?}"),
    };
    assert_eq!(
        returned, expected,
        "the route must return the native theorem composition, not a substitute"
    );
    check(&env.env, &Context::new(), &returned, &phi_closed)
        .expect("the returned certificate must kernel-check against independent phi_closed");
    assert_eq!(
        before,
        env.env.trusted_base().into_iter().collect(),
        "the proved route must add no trusted declaration"
    );
}

/// `AC-2`/`AC-4`: hold the accepted Rust problem and certificate fixed while
/// corrupting only the theorem composition. The final kernel guard must refuse
/// `Proved` and retain the accepted-but-withheld `Unknown` exit.
#[test]
fn checker_acceptance_without_a_kernel_check_stays_withheld_unknown() {
    let mut env = env_with_fok();
    let pred_p = add_predicate(&mut env);
    let sig = installed_signature(&env, pred_p);
    let phi_closed = positive_control_term(&sig);
    let problem = quote_fo(&env.env, &sig, &phi_closed).expect("accepted FO quotation");
    let rust_cert = find_certificate(&problem.f).expect("slice certificate");
    assert!(
        check_cert(&embed(&problem.f), &rust_cert),
        "negative row must share the checker-accepted problem and certificate"
    );

    let mut wrong_sig = sig.clone();
    let handles = wrong_sig
        .catalog
        .as_mut()
        .expect("installed catalog handles");
    handles.checker_soundness = handles.embedding_adequacy;
    assert!(
        kernel_checked_fo_composite(&env.env, &wrong_sig, &problem, &rust_cert, &phi_closed,)
            .is_none(),
        "deliberately mis-composed theorem handles must fail the final kernel check"
    );

    let verdict = attempt_fo_with_signature(
        &mut env.env,
        &Context::new(),
        &phi_closed,
        &phi_closed,
        &wrong_sig,
    );
    let hole_id = match verdict {
        Verdict::Unknown { hole_id } => hole_id,
        other => panic!(
            "check_cert acceptance without a checking composite must never be Proved, got {other:?}"
        ),
    };
    assert!(
        hole_label(&env.env, hole_id).contains(FO_WITHHELD_MARKER),
        "accepted-but-nonchecking composition must use the withheld Unknown exit"
    );
}

/// `AC-1`: exercise the real catalog-aware production entry rather than only
/// the direct signature seam, checking its certificate against `goal_closed`.
#[test]
fn public_catalog_route_returns_a_kernel_backed_proved_verdict() {
    let mut env = env_with_fok();
    let handles = FoCatalogHandles::resolve(&env.globals).expect("catalog handles");
    let route_sort_id = env
        .declare_postulate_raw("D3RouteSort", Term::Type(Level::zero()))
        .expect("discoverable route sort");
    let route_sort = Term::const_(route_sort_id, vec![]);
    env.declare_postulate_raw(
        "d3_route_pred",
        Term::pi(route_sort, Term::omega(Level::zero())),
    )
    .expect("discoverable route predicate");

    let elaborated = env
        .elaborate_decl_v1(
            "prove d3_public_route : \
             (x : D3RouteSort) -> \
             d3_route_pred x -> d3_route_pred x",
        )
        .expect("public-route obligation");
    let extracted = v2_extract(&elaborated);
    let triple = extracted.obligations.first().expect("one obligation");
    let (routed_sig, problem) =
        discover_and_quote_fo_with_catalog(&env.env, &triple.goal_closed, Some(&handles))
            .expect("the exact public obligation must reach catalog-aware FO discovery");
    let rust_cert = find_certificate(&problem.f).expect("public-route slice certificate");
    assert!(
        check_cert(&embed(&problem.f), &rust_cert),
        "public-route control must reach Rust checker acceptance"
    );
    let expected = kernel_checked_fo_composite(
        &env.env,
        &routed_sig,
        &problem,
        &rust_cert,
        &triple.goal_closed,
    )
    .expect("public-route composition must check against independent goal_closed");
    let before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();

    let result = attempt_obligation_with_catalog_handles(&mut env.env, &handles, triple);
    let cert = match result.verdict {
        Verdict::Proved { cert } => cert,
        other => panic!("catalog-backed public FO route must be Proved, got {other:?}"),
    };
    assert_eq!(
        cert, expected,
        "public entry must return the FO composition, not an IPC fallback certificate"
    );
    check(&env.env, &Context::new(), &cert, &triple.goal_closed)
        .expect("public-route certificate must prove the independent obligation");
    assert_eq!(
        before,
        env.env.trusted_base().into_iter().collect(),
        "public FO proof must add no trusted declaration"
    );
}

/// `AC-5`: quotation refusal and absence of a certificate retain the ordinary
/// IPC fallback rather than entering the accepted-but-withheld exit.
#[test]
fn quotation_refusal_and_no_certificate_keep_the_ipc_fallback() {
    let mut env = env_with_fok();
    let pred_p = add_predicate(&mut env);
    let sig = installed_signature(&env, pred_p);

    let no_cert = negative_control_term(&env.env, &sig);
    let no_cert_problem = quote_fo(&env.env, &sig, &no_cert).expect("negative control quotes");
    assert!(
        find_certificate(&no_cert_problem.f).is_none(),
        "negative control must reach the no-certificate path"
    );
    let no_cert_hole =
        match attempt_fo_with_signature(&mut env.env, &Context::new(), &no_cert, &no_cert, &sig) {
            Verdict::Unknown { hole_id } => hole_id,
            other => panic!("no-certificate path must preserve IPC fallback, got {other:?}"),
        };
    assert_eq!(hole_label(&env.env, no_cert_hole), ORDINARY_LABEL);

    let refused = Term::Type(Level::zero());
    assert!(
        quote_fo(&env.env, &sig, &refused).is_err(),
        "refusal control must not enter certificate search"
    );
    let refused_hole =
        match attempt_fo_with_signature(&mut env.env, &Context::new(), &refused, &refused, &sig) {
            Verdict::Unknown { hole_id } => hole_id,
            other => panic!("quotation refusal must preserve IPC fallback, got {other:?}"),
        };
    assert_eq!(hole_label(&env.env, refused_hole), ORDINARY_LABEL);
}
