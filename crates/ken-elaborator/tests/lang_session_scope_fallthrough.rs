//! A checked session reference must not depend on its legacy globals spelling.

use ken_elaborator::ElabEnv;
use ken_kernel::{Level, Term};

/// Promise class: durable invariant. MEASURED: a checked raw postulate is
/// usable as a type after its flat spelling is removed. CLAIMED: source-unit
/// resolution uses session identity. THE GAP: the matching pre-session base
/// must reject this exact test after the same removal, not merely stay green.
#[test]
fn raw_type_survives_disabled_flat_fallthrough() {
    let mut env = ElabEnv::new().expect("prelude");
    let ty = env
        .declare_postulate_raw("SessionRawType", Term::ty(Level::Zero))
        .expect("checked raw type");
    assert_eq!(env.globals.remove("SessionRawType"), Some(ty));
    let use_id = env
        .elaborate_decl("fn use_session_raw (x : SessionRawType) : SessionRawType = x")
        .expect("type reference must use the session identity");
    let (_, checked_ty) = env.env.const_type(use_id).expect("checked type");
    assert_eq!(
        checked_ty,
        Term::pi(Term::const_(ty, vec![]), Term::const_(ty, vec![]))
    );
}

/// Promise class: durable invariant. MEASURED: a prior incremental source
/// declaration remains readable to the next unit without its flat globals key.
#[test]
fn prior_source_local_survives_disabled_flat_fallthrough() {
    let mut env = ElabEnv::new().expect("prelude");
    let first = env
        .elaborate_decl("const session_source : Bool = True")
        .expect("checked earlier source unit");
    assert_eq!(env.globals.remove("session_source"), Some(first));
    let second = env
        .elaborate_decl("const second_session : Bool = session_source")
        .expect("later unit must select the earlier checked identity");
    let (_, body) = env.env.transparent_body(second).expect("checked body");
    assert_eq!(body, Term::const_(first, vec![]));
}

/// Promise class: durable invariant. MEASURED: the REPL expression API reads
/// a checked raw postulate's identity after its flat spelling is removed.
#[test]
fn repl_expression_survives_disabled_flat_fallthrough() {
    let mut env = ElabEnv::new().expect("prelude");
    let ty = env
        .declare_postulate_raw("SessionRawType", Term::ty(Level::Zero))
        .expect("checked raw type");
    let value = env
        .declare_postulate_raw("session_raw_value", Term::const_(ty, vec![]))
        .expect("checked raw value");
    assert_eq!(env.globals.remove("session_raw_value"), Some(value));
    let (term, actual_ty) = env
        .elaborate_expr("session REPL probe", "session_raw_value")
        .expect("standalone expression must select the session identity");
    assert_eq!(term, Term::const_(value, vec![]));
    assert_eq!(actual_ty, Term::const_(ty, vec![]));
}
