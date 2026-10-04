//! `17 §2` Unit-η through the real checked prelude, not a kernel-only fixture.
//! This positive pin fails if prelude registration is removed.

use ken_elaborator::ElabEnv;
use ken_kernel::{convert, infer, Context, Term};

#[test]
fn prelude_unit_registers_its_checked_family_for_eta() {
    let env = ElabEnv::new().expect("prelude");
    let unit = env.globals["Unit"];
    let unit_ty = Term::indformer(unit, vec![]);
    let mut ctx = Context::new();
    ctx.push(unit_ty.clone());
    ctx.push(unit_ty.clone());
    assert_eq!(infer(&env.env, &ctx, &Term::var(1)), Ok(unit_ty.clone()));
    assert_eq!(infer(&env.env, &ctx, &Term::var(0)), Ok(unit_ty.clone()));
    assert!(
        convert(&env.env, &ctx, &unit_ty, &Term::var(1), &Term::var(0)),
        "distinct prelude Unit variables must convert by registered Unit-η"
    );
    assert_eq!(env.env.unit_type(), Some(unit));
}
