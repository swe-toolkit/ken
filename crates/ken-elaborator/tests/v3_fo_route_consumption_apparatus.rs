//! Acceptance for `V3-FO-ROUTE-CONSUMPTION-APPARATUS`.
//!
//! Promise class: durable invariants. The positive and negative rows share one
//! Rust certificate and differ only in the independently interpreted atom
//! environment, so the catalog checker prefix stays accepted while the final
//! kernel check against the original obligation discriminates faithfulness.

#[path = "support/catalog_or.rs"]
mod catalog_or;

use std::collections::BTreeSet;

use ken_elaborator::{
    attempt_obligation_with_catalog_globals,
    fo_kripke::{
        check_cert, embed, encode_fo_problem, find_certificate, kernel_checked_fo_composite,
        positive_control_term, quote_fo, AtomEnv, Carriers, Cert, FOProblem, FoCatalogHandles,
        FoSliceSignature, IForm, IVar,
    },
    prover::{attempt_fo_with_signature, Verdict},
    v2_extract, ElabEnv,
};
use ken_kernel::{check, infer, Context, Term};

const FOK_SOURCE: &str =
    include_str!("../../../catalog/packages/Tooling/Verification/FoKripke.ken");

fn env_with_fok() -> ElabEnv {
    let mut env = ElabEnv::new().expect("base environment");
    catalog_or::load_core_logic_or(&mut env);
    env.elaborate_file(FOK_SOURCE)
        .expect("FoKripke catalog unit must elaborate");
    env
}

fn add_predicates(env: &mut ElabEnv) -> (ken_kernel::GlobalId, ken_kernel::GlobalId) {
    let faithful = env
        .elaborate_decl(
            "fn apparatus_pred (x : Bool) : Omega = \
             match x { True ↦ Top; False ↦ Bottom }",
        )
        .expect("faithful predicate");
    let wrong = env
        .elaborate_decl("fn apparatus_wrong_pred (x : Bool) : Omega = Bottom")
        .expect("wrong predicate with the same carrier type");
    (faithful, wrong)
}

fn slice_signature(env: &ElabEnv, pred_p: ken_kernel::GlobalId) -> FoSliceSignature {
    FoSliceSignature {
        sort_a: Term::indformer(env.globals["Bool"], vec![]),
        pred_p,
        or_id: env.globals["Core.Logic.Or.Or"],
        catalog: None,
    }
}

fn accepted_problem(env: &ElabEnv, sig: &FoSliceSignature) -> (Term, FOProblem, Cert) {
    let phi_closed = positive_control_term(sig);
    let problem = quote_fo(&env.env, sig, &phi_closed).expect("accepted FO quotation");
    let cert = find_certificate(&problem.f).expect("slice certificate");
    assert!(
        check_cert(&embed(&problem.f), &cert),
        "positive control must reach Rust checker acceptance"
    );
    (phi_closed, problem, cert)
}

fn checker_validity_prefix(
    env: &ElabEnv,
    sig: &FoSliceSignature,
    problem: &FOProblem,
    cert: &Cert,
) -> Term {
    let encoded = encode_fo_problem(sig, problem, cert).expect("catalog encoding");
    let checker_soundness = sig
        .catalog
        .as_ref()
        .expect("installed catalog handles")
        .checker_soundness;
    Term::app(
        Term::app(
            Term::app(Term::const_(checker_soundness, vec![]), encoded.target_form),
            encoded.cert,
        ),
        Term::const_(env.env.tt_id(), vec![]),
    )
}

#[test]
fn handles_resolve_atomically_then_survive_source_name_removal() {
    let mut env = env_with_fok();
    let (pred_p, _) = add_predicates(&mut env);

    let mut missing_theorem = env.globals.clone();
    missing_theorem.remove("fok_checker_soundness");
    assert!(
        FoCatalogHandles::resolve(&missing_theorem).is_none(),
        "a partial theorem installation must fail closed"
    );
    let mut missing_encoder_constructor = env.globals.clone();
    missing_encoder_constructor.remove("FokScopedForall");
    assert!(
        FoCatalogHandles::resolve(&missing_encoder_constructor).is_none(),
        "a partial encoding vocabulary must fail closed"
    );

    let sig = slice_signature(&env, pred_p)
        .with_catalog_globals(&env.globals)
        .expect("complete catalog installation");
    let (phi_closed, problem, cert) = accepted_problem(&env, &sig);

    env.globals.remove("fok_checker_soundness");
    env.globals.remove("fok_embedding_adequacy");
    let composite = kernel_checked_fo_composite(&env.env, &sig, &problem, &cert, &phi_closed)
        .expect("carried GlobalIds must not be looked up by name in the prover");
    check(&env.env, &Context::new(), &composite, &phi_closed)
        .expect("carried theorem composition must remain kernel-checkable");
}

#[test]
fn faithful_and_wrong_encodings_form_a_kernel_guarded_pair() {
    let mut env = env_with_fok();
    let (pred_p, wrong_pred) = add_predicates(&mut env);
    let bare_sig = slice_signature(&env, pred_p);
    let (bare_phi, bare_problem, bare_cert) = accepted_problem(&env, &bare_sig);
    assert!(encode_fo_problem(&bare_sig, &bare_problem, &bare_cert).is_none());
    assert!(
        kernel_checked_fo_composite(&env.env, &bare_sig, &bare_problem, &bare_cert, &bare_phi,)
            .is_none(),
        "an uninstalled catalog must refuse rather than guess identities"
    );

    let sig = bare_sig
        .with_catalog_globals(&env.globals)
        .expect("catalog installation");
    let (phi_closed, problem, cert) = accepted_problem(&env, &sig);
    let before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();

    let encoded = encode_fo_problem(&sig, &problem, &cert).expect("faithful encoding");
    for term in [
        &encoded.signature,
        &encoded.carriers,
        &encoded.atom_env,
        &encoded.source_form,
        &encoded.scoped_form,
        &encoded.target_form,
        &encoded.cert,
    ] {
        infer(&env.env, &Context::new(), term)
            .unwrap_or_else(|error| panic!("encoded catalog term is ill-typed: {error:?}"));
    }

    let composite = kernel_checked_fo_composite(&env.env, &sig, &problem, &cert, &phi_closed)
        .expect("faithful checker-soundness then adequacy composition");
    check(&env.env, &Context::new(), &composite, &phi_closed)
        .expect("faithful composite must prove the independent obligation");
    assert_eq!(
        before,
        env.env.trusted_base().into_iter().collect(),
        "encoding and composite construction must add no trusted declaration"
    );

    let wrong_problem = FOProblem {
        carriers: Carriers {
            sort_a: problem.carriers.sort_a.clone(),
        },
        atoms: AtomEnv { pred_p: wrong_pred },
        f: problem.f.clone(),
    };
    assert!(
        check_cert(&embed(&wrong_problem.f), &cert),
        "negative shares the same Rust-checker-accepted form and certificate"
    );
    infer(
        &env.env,
        &Context::new(),
        &checker_validity_prefix(&env, &sig, &wrong_problem, &cert),
    )
    .expect("wrong atom interpretation must pass the catalog checker prefix");
    assert!(
        kernel_checked_fo_composite(&env.env, &sig, &wrong_problem, &cert, &phi_closed,).is_none(),
        "wrong denotation must fail the final kernel check against phi_closed"
    );
    assert_eq!(
        before,
        env.env.trusted_base().into_iter().collect(),
        "a rejected wrong encoding must add no trusted declaration"
    );
}

#[test]
fn encoder_covers_every_slice_source_constructor_and_route_stays_unknown() {
    let mut env = env_with_fok();
    let (pred_p, _) = add_predicates(&mut env);
    let sig = slice_signature(&env, pred_p)
        .with_catalog_globals(&env.globals)
        .expect("catalog installation");
    let (_, accepted, cert) = accepted_problem(&env, &sig);

    let all_source_constructors = IForm::Forall(Box::new(IForm::Forall(Box::new(IForm::Or(
        Box::new(IForm::Atom(IVar(1))),
        Box::new(IForm::Imp(
            Box::new(IForm::Bottom),
            Box::new(IForm::Atom(IVar(0))),
        )),
    )))));
    let full_problem = FOProblem {
        carriers: accepted.carriers.clone(),
        atoms: accepted.atoms.clone(),
        f: all_source_constructors,
    };
    let encoded = encode_fo_problem(&sig, &full_problem, &cert)
        .expect("every slice source constructor, including nonzero Fin");
    for term in [
        encoded.source_form,
        encoded.scoped_form,
        encoded.target_form,
        encoded.cert,
    ] {
        infer(&env.env, &Context::new(), &term)
            .expect("complete slice encoding must be kernel-well-formed");
    }

    let (phi_closed, problem, accepted_cert) = accepted_problem(&env, &sig);
    assert!(matches!(
        attempt_fo_with_signature(
            &mut env.env,
            &Context::new(),
            &phi_closed,
            &phi_closed,
            &sig,
        ),
        Verdict::Unknown { .. }
    ));

    let elaborated = env
        .elaborate_decl_v1(
            "prove apparatus_public_route : \
             (x : Bool) -> apparatus_pred x -> apparatus_pred x",
        )
        .expect("public-route obligation");
    let extracted = v2_extract(&elaborated);
    let triple = extracted.obligations.first().expect("one obligation");
    assert!(matches!(
        attempt_obligation_with_catalog_globals(&mut env.env, &env.globals, triple).verdict,
        Verdict::Unknown { .. }
    ));

    assert!(
        kernel_checked_fo_composite(&env.env, &sig, &problem, &accepted_cert, &phi_closed,)
            .is_some(),
        "the composite is ready while this prerequisite still withholds Proved"
    );
}
