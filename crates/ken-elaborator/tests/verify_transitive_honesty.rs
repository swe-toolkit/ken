//! `21 §5.4`: proof status follows the checked certificate's transitive
//! obligation-hole identities, not the proof's own hole alone or every axiom.

use std::collections::BTreeSet;

use ken_elaborator::{attempt_with_cert, ElabEnv, Verdict};
use ken_kernel::{check, Context, Decl, Term};

/// Promise class: durable invariant. MEASURED: one source `prove` hole passes
/// through two checked transparent theorems and demotes both the prover verdict
/// and the term-proof claim; discharging that same seed flips both to proved.
/// CLAIMED: reachable open obligations (not merely one's own hole) govern
/// status. THE GAP: the kernel-only unit independently covers structural term
/// variants; this exercises the real elaborator registration and prover seam.
#[test]
fn checked_two_level_proof_is_not_proved_until_reachable_seed_closes() {
    let mut env = ElabEnv::new().expect("prelude");
    let seed = env.elaborate_decl_v1("prove hole_seed : Top").unwrap();
    let seed_obligation = seed.obligations.first().expect("seed hole");
    let seed_id = seed_obligation.hole_id;
    let first = env
        .elaborate_decl_v1("theorem bridge_one : Top = hole_seed")
        .unwrap()
        .def_id;
    let second = env
        .elaborate_decl_v1("theorem bridge_two : Top = bridge_one")
        .unwrap()
        .def_id;
    let target = env.elaborate_decl_v1("prove claim_target : Top").unwrap();
    let target_obligation = target.obligations.first().expect("target hole");
    assert_ne!(seed_id, target_obligation.hole_id);
    assert_eq!(
        env.env.transparent_body(second).unwrap().1,
        Term::const_(first, vec![])
    );
    assert_eq!(
        env.env.transparent_body(first).unwrap().1,
        Term::const_(seed_id, vec![])
    );
    assert_eq!(
        env.obligation_holes
            .reached_open(&env.env, &Term::const_(second, vec![])),
        BTreeSet::from([seed_id])
    );
    assert!(
        !env.claim_proved(second),
        "term proof inherits the seed hole"
    );

    let cert = Term::const_(second, vec![]);
    check(
        &env.env,
        &Context::new(),
        &cert,
        &target_obligation.goal_closed,
    )
    .expect("the certificate is kernel-checked even while depending on a hole");
    let verdict = attempt_with_cert(
        &mut env.env,
        &mut env.obligation_holes,
        &target_obligation.goal_closed,
        cert.clone(),
    );
    let withheld = match verdict {
        Verdict::Unknown { hole_id } => hole_id,
        other => panic!("open transitive hole must demote the prover: {other:?}"),
    };
    assert_ne!(withheld, seed_id);
    assert_ne!(withheld, target_obligation.hole_id);
    assert!(env
        .obligation_holes
        .reached_open(&env.env, &Term::const_(withheld, vec![]))
        .contains(&withheld));
    assert!(env.is_open_hole(seed_id));
    assert!(!env.claim_proved(target_obligation.hole_id));

    let checked_tt = Term::const_(env.env.tt_id(), vec![]);
    assert!(env.discharge_hole(seed_obligation, checked_tt));
    assert!(!env.is_open_hole(seed_id));
    assert!(
        env.claim_proved(second),
        "term proof flips after the seed closes"
    );
    assert!(env
        .obligation_holes
        .reached_open(&env.env, &cert)
        .is_empty());
    let proved = attempt_with_cert(
        &mut env.env,
        &mut env.obligation_holes,
        &target_obligation.goal_closed,
        cert.clone(),
    );
    assert!(matches!(proved, Verdict::Proved { cert: ref got } if *got == cert));
    assert!(env.discharge_hole(target_obligation, cert));
    assert!(env.claim_proved(target_obligation.hole_id));
    assert!(
        env.is_open_hole(withheld),
        "earlier withheld verdict retains its distinct hole"
    );
}

/// Promise class: durable invariant. MEASURED: an independently admitted
/// audited axiom remains in the kernel trusted base but not the obligation
/// registry, and an actual term proof and prover certificate both stay proved.
/// CLAIMED: audited assumptions must remain visible without being mislabelled
/// open verification obligations. THE GAP: catalog-wide census is separate.
#[test]
fn audited_postulate_does_not_demote_an_independent_checked_proof() {
    let mut env = ElabEnv::new().unwrap();
    let top = Term::const_(env.env.top_id(), vec![]);
    let audit = env
        .declare_postulate_raw("audited_witness", top.clone())
        .unwrap();
    assert!(matches!(env.env.lookup(audit), Some(Decl::Opaque { .. })));
    assert!(env.env.trusted_base().contains(&audit));
    let proof = env
        .elaborate_decl_v1("theorem audited_bridge : Top = audited_witness")
        .unwrap()
        .def_id;
    let claim = env.elaborate_decl_v1("prove audited_claim : Top").unwrap();
    assert!(env
        .obligation_holes
        .reached_open(&env.env, &Term::const_(proof, vec![]))
        .is_empty());
    assert!(env.claim_proved(proof));
    assert!(matches!(
        attempt_with_cert(
            &mut env.env,
            &mut env.obligation_holes,
            &claim.obligations[0].goal_closed,
            Term::const_(proof, vec![])
        ),
        Verdict::Proved { .. }
    ));
    assert!(env.env.trusted_base().contains(&audit));
    assert!(
        !env.claim_proved(audit),
        "an opaque assumption is not itself proved"
    );
}
