//! AC-9: checked transparent bodies cannot outlive their allocation prefix.

use ken_elaborator::ElabEnv;
use ken_interp::{eval, EvalStore, EvalVal};
use ken_kernel::Term;

#[test]
fn erased_body_cache_does_not_survive_checked_rollback_and_id_reuse() {
    // Promise class: durable invariant (42 §3.2, 46 §4, AC-9).
    // MEASURED: the same store evaluates a checked constant before and after
    // a kernel-admitted rollback that reuses its GlobalId for a new body.
    // CLAIMED: no erased body cached in the earlier allocation survives reuse.
    // THE GAP: this observes one Int constant and public eval entry; unwind
    // cleanup and nested evaluation share the scoped-cache implementation.
    let mut env = ElabEnv::new().expect("prelude admits");
    let mut store = EvalStore::new();
    let mark = ken_kernel::env_mark(&env.env);
    env.elaborate_decl("const seven_or_eight : Int = 7")
        .expect("first declaration admits");
    let first = env.globals["seven_or_eight"];
    assert_eq!(
        eval(&[], &Term::const_(first, vec![]), &env.env, &mut store),
        EvalVal::Int(7)
    );

    ken_kernel::rollback_to_mark(&mut env.env, mark).expect("supported rollback admits");
    env.globals.remove("seven_or_eight");
    env.elaborate_decl("const seven_or_eight : Int = 8")
        .expect("second declaration admits");
    let second = env.globals["seven_or_eight"];
    assert_eq!(first, second, "the fixture must reuse the GlobalId");
    assert_eq!(
        eval(&[], &Term::const_(second, vec![]), &env.env, &mut store),
        EvalVal::Int(8),
        "a reused GlobalId must not retain the erased body of its predecessor"
    );
}
