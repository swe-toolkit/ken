//! `21 §5.4`: an open obligation minted inside a space is not a proved claim.

use std::collections::BTreeSet;

use ken_elaborator::{ElabEnv, ObligationKind};
use ken_kernel::{check, declare_def, postulates_reachable, Context, Decl, GlobalId, Term};

const SOURCE: &str = r#"
fn d (x : Int) (y : Int) : Int requires Not (Equal Int x y) = x
const k : Int = d 0 0
space S {
  mut cell : Int = 0
  proc call () : Int visits [S] = d 0 0
}
theorem ts1 : Top = let u = S.call in Proved
space T {
  mut cell : Int = d 0 0
}
"#;

fn reached(env: &ElabEnv, root: GlobalId) -> BTreeSet<GlobalId> {
    postulates_reachable(&env.env, &Term::const_(root, vec![]))
}

/// Promise class: durable invariant. MEASURED: the kernel independently
/// reaches the open S.call hole through the checked operation and theorem;
/// the registry demotes both, while the ordinary k control remains demoted.
/// CLAIMED: a space body call-site is subject to transitive honesty. THE GAP:
/// independently checked reachability is compared with the reported hole,
/// rather than inferring a dependency from the registry alone.
#[test]
fn space_call_site_demotes_operation_and_transitive_theorem() {
    let mut env = ElabEnv::new().expect("prelude");
    let results = env
        .elaborate_file_v1(SOURCE)
        .expect("space and proof fixture");
    let result = |name: &str| {
        results
            .iter()
            .find(|result| result.name == name)
            .unwrap_or_else(|| panic!("missing result {name}"))
    };
    let trusted: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    let control = result("k");
    let k_hole = control
        .obligations
        .first()
        .expect("control call-site hole")
        .hole_id;
    assert!(trusted.contains(&k_hole));
    assert!(matches!(
        control.obligations[0].kind,
        ObligationKind::Requires
    ));
    assert_eq!(reached(&env, control.def_id), BTreeSet::from([k_hole]));
    assert!(
        !env.claim_proved(control.def_id),
        "ordinary declaration control"
    );

    let space_call = result("S.call");
    let call_hole = space_call
        .obligations
        .first()
        .expect("space call-site hole")
        .hole_id;
    assert!(trusted.contains(&call_hole));
    assert!(matches!(
        space_call.obligations[0].kind,
        ObligationKind::Requires
    ));
    assert_eq!(
        reached(&env, space_call.def_id),
        BTreeSet::from([call_hole])
    );
    assert_eq!(
        env.obligation_holes
            .reached_open(&env.env, &Term::const_(space_call.def_id, vec![])),
        BTreeSet::from([call_hole])
    );
    assert!(
        !env.claim_proved(space_call.def_id),
        "the open space operation"
    );

    let theorem = result("ts1");
    assert!(matches!(
        env.env.lookup(theorem.def_id),
        Some(Decl::Transparent { .. })
    ));
    assert_eq!(reached(&env, theorem.def_id), BTreeSet::from([call_hole]));
    assert_eq!(
        env.obligation_holes
            .reached_open(&env.env, &Term::const_(theorem.def_id, vec![])),
        BTreeSet::from([call_hole])
    );
    assert!(!env.claim_proved(theorem.def_id), "theorem through S.call");
    assert_ne!(k_hole, call_hole);
}

/// Promise class: durable invariant. MEASURED: the generated initial state
/// carries a checked cell initializer hole reached by a checked proof that
/// reads the state, and the registry demotes that proof. CLAIMED: space-cell
/// initializer obligations cannot make transitive claims read proved. THE
/// GAP: the generated state ID is read from the space's checked metadata;
/// the proof term is checked independently of the registry.
#[test]
fn space_cell_initializer_demotes_a_checked_state_reader() {
    let mut env = ElabEnv::new().expect("prelude");
    let results = env
        .elaborate_file_v1(SOURCE)
        .expect("space and proof fixture");
    let result = |name: &str| {
        results
            .iter()
            .find(|result| result.name == name)
            .unwrap_or_else(|| panic!("missing result {name}"))
    };
    let trusted: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    let space_state = result("T");
    let cell_hole = space_state
        .obligations
        .first()
        .expect("cell initializer hole")
        .hole_id;
    let initial_id = env
        .space_initial_state("T")
        .expect("generated initial state");
    assert!(trusted.contains(&cell_hole));
    assert!(matches!(
        space_state.obligations[0].kind,
        ObligationKind::Requires
    ));
    assert_eq!(reached(&env, initial_id), BTreeSet::from([cell_hole]));
    let state_type = Term::const_(space_state.def_id, vec![]);
    let top = Term::const_(env.env.top_id(), vec![]);
    let witness = Term::lam(state_type.clone(), Term::const_(env.env.tt_id(), vec![]));
    let witness_id = declare_def(
        &mut env.env,
        vec![],
        Term::pi(state_type, top.clone()),
        witness,
    )
    .expect("checked consumer of the state type");
    let claim_body = Term::app(
        Term::const_(witness_id, vec![]),
        Term::const_(initial_id, vec![]),
    );
    check(&env.env, &Context::new(), &claim_body, &top)
        .expect("proof consuming the generated single-cell initial state checks");
    let claim_id = declare_def(&mut env.env, vec![], top, claim_body)
        .expect("checked proof consuming T's cell");
    assert_eq!(reached(&env, claim_id), BTreeSet::from([cell_hole]));
    assert_eq!(
        env.obligation_holes
            .reached_open(&env.env, &Term::const_(claim_id, vec![])),
        BTreeSet::from([cell_hole])
    );
    assert!(
        !env.claim_proved(claim_id),
        "checked claim reading T's cell"
    );
    assert_ne!(result("k").obligations[0].hole_id, cell_hole);
}

/// Promise class: durable invariant. MEASURED: a space operation's ensures
/// hole is an opaque checked postulate and a registered open obligation.
/// CLAIMED: registration covers holes minted after the operation body as well
/// as cell initializers and body call sites. THE GAP: the ensures hole is not
/// necessarily reachable from the operation term; measure its own identity.
#[test]
fn space_ensures_hole_is_registered_even_without_body_reachability() {
    let mut env = ElabEnv::new().expect("prelude");
    let results = env
        .elaborate_file_v1(
            "space U { mut cell : Int = 0 \
         proc contract () : Int ensures Equal Int result 0 visits [U] = 1 }",
        )
        .expect("checked space contract");
    let operation = results
        .iter()
        .find(|r| r.name == "U.contract")
        .expect("operation");
    assert_eq!(operation.obligations.len(), 1);
    assert!(matches!(
        operation.obligations[0].kind,
        ObligationKind::Ensures
    ));
    let hole = operation.obligations[0].hole_id;
    assert!(env.env.trusted_base().contains(&hole));
    assert_eq!(reached(&env, hole), BTreeSet::from([hole]));
    assert_eq!(
        env.obligation_holes
            .reached_open(&env.env, &Term::const_(hole, vec![])),
        BTreeSet::from([hole])
    );
}
