//! A public obligation id cannot launder kernel vocabulary into a discharged
//! assumption; a genuine checked postulate can still be retired (spec 18 §5).

use ken_elaborator::{ElabEnv, Obligation, ObligationKind, Span};
use ken_kernel::{Context, GlobalId, Level, Term};

fn obligation(hole_id: GlobalId, goal_closed: Term) -> Obligation {
    Obligation {
        id: 0,
        hole_id,
        goal_closed,
        span: Span::zero(),
        kind: ObligationKind::Prove,
    }
}

#[test]
fn forged_prelude_id_passes_goal_check_but_cannot_be_discharged() {
    let mut env = ElabEnv::new().expect("checked prelude");
    let bottom = env.env.bottom_id();
    let top = env.env.top_id();
    let cert = Term::const_(top, vec![]);
    let goal = Term::Omega(Level::zero());
    assert!(
        ken_kernel::check(&env.env, &Context::new(), &cert, &goal).is_ok(),
        "the caller's goal check must pass so the kernel origin guard is reached"
    );
    let forged = obligation(bottom, goal);
    let before = env.env.clone();
    let trusted_before = env.env.trusted_base();
    assert!(!env.discharge_hole(&forged, cert));
    assert_eq!(env.env, before);
    assert_eq!(env.env.trusted_base(), trusted_before);
    assert!(!trusted_before.contains(&bottom));
    assert!(!trusted_before.contains(&top));
}

#[test]
fn checked_postulate_hole_discharge_retires_only_its_assumption() {
    let mut env = ElabEnv::new().expect("checked prelude");
    let goal = Term::const_(env.env.top_id(), vec![]);
    let hole = env
        .declare_postulate_raw("actual_hole", goal.clone())
        .expect("checked assumption");
    let other = env
        .declare_postulate_raw("other", goal.clone())
        .expect("independent checked assumption");
    let before = env.env.trusted_base();
    assert!(before.contains(&hole) && before.contains(&other));
    let cert = Term::const_(env.env.tt_id(), vec![]);
    assert!(env.discharge_hole(&obligation(hole, goal), cert));
    let mut expected = before;
    expected.retain(|id| *id != hole);
    assert_eq!(env.env.trusted_base(), expected);
}
