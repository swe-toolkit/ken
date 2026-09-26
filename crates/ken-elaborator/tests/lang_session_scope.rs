//! Session-local checked identities for incremental source and REPL expressions.

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{GlobalId, Level, Term};

fn checked_body(env: &ElabEnv, name: &str) -> Term {
    env.env
        .transparent_body(env.globals[name])
        .expect("checked definition")
        .1
}

/// Promise class: durable invariant. MEASURED: raw postulates select their
/// exact checked IDs in standalone expression and source type/term positions,
/// and an earlier checked unit remains unchanged after a same-spelling rebind.
/// CLAIMED: session scope is a checked-identity authority, not a view of mutable
/// globals. THE GAP: the separate fall-through-disable probe tests whether
/// selection, rather than the still-live flat map, provides the identity.
#[test]
fn raw_postulate_session_bindings_preserve_id_and_earlier_units() {
    let mut env = ElabEnv::new().expect("prelude");
    let first_type = env
        .declare_postulate_raw("SessionType", Term::ty(Level::Zero))
        .expect("first checked type");
    let first_value = env
        .declare_postulate_raw("session_value", Term::const_(first_type, vec![]))
        .expect("checked witness");
    let (standalone, standalone_ty) = env
        .elaborate_expr("session expression", "session_value")
        .expect("standalone expression reads the session identity");
    assert_eq!(standalone, Term::const_(first_value, vec![]));
    assert_eq!(standalone_ty, Term::const_(first_type, vec![]));
    let earlier = env
        .elaborate_decl("const earlier_session : SessionType = session_value")
        .expect("source term and type refer to the checked identities");
    let (_, earlier_ty) = env.env.const_type(earlier).expect("earlier checked type");
    assert_eq!(earlier_ty, Term::const_(first_type, vec![]));
    assert_eq!(
        checked_body(&env, "earlier_session"),
        Term::const_(first_value, vec![])
    );

    let second_type = env
        .declare_postulate_raw("SessionType", Term::ty(Level::Zero))
        .expect("same-spelling later checked type");
    let second_value = env
        .declare_postulate_raw("session_value", Term::const_(second_type, vec![]))
        .expect("same-spelling later checked witness");
    assert_ne!(first_type, second_type);
    assert_ne!(first_value, second_value);
    let later = env
        .elaborate_decl("const later_session : SessionType = session_value")
        .expect("the later unit sees the new identities");
    let (_, later_ty) = env.env.const_type(later).expect("later checked type");
    assert_eq!(later_ty, Term::const_(second_type, vec![]));
    assert_eq!(
        checked_body(&env, "later_session"),
        Term::const_(second_value, vec![])
    );
    assert_eq!(
        checked_body(&env, "earlier_session"),
        Term::const_(first_value, vec![])
    );
    let (_, earlier_ty_after) = env.env.const_type(earlier).expect("earlier type unchanged");
    assert_eq!(earlier_ty_after, Term::const_(first_type, vec![]));
}

/// Promise class: durable invariant. MEASURED: a session alias of one checked
/// constructor selects it in expressions and pattern coverage; rebinding the
/// alias changes the new unit but cannot rewrite the earlier checked term.
/// CLAIMED: constructor identity follows the session binding in both positions.
/// THE GAP: the fall-through-disable differential independently excludes a
/// coincidentally matching mutable globals spelling.
#[test]
fn checked_constructor_alias_selects_expression_and_pattern_by_id() {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_decl("data SessionChoice = LeftChoice | RightChoice")
        .expect("checked family");
    let left: GlobalId = env.globals["LeftChoice"];
    let right: GlobalId = env.globals["RightChoice"];
    env.bind_session_name("SessionAlias", left)
        .expect("alias checked constructor");
    let first = env
        .elaborate_decl("const first_choice : SessionChoice = SessionAlias")
        .expect("expression selects left");
    assert_eq!(
        checked_body(&env, "first_choice"),
        Term::constructor(left, vec![])
    );
    env.elaborate_decl(
        "fn inspect_choice (x : SessionChoice) : Bool = \
         match x { SessionAlias |-> True; RightChoice |-> False }",
    )
    .expect("alias selects left branch in an exhaustive checked pattern");
    let earlier_match = checked_body(&env, "inspect_choice");

    env.bind_session_name("SessionAlias", right)
        .expect("rebind alias to distinct constructor");
    let second = env
        .elaborate_decl("const second_choice : SessionChoice = SessionAlias")
        .expect("later expression selects right");
    assert_eq!(
        checked_body(&env, "second_choice"),
        Term::constructor(right, vec![])
    );
    assert_eq!(
        checked_body(&env, "first_choice"),
        Term::constructor(left, vec![])
    );
    assert_eq!(checked_body(&env, "inspect_choice"), earlier_match);
    assert_ne!(first, second);
    let error = env
        .elaborate_decl(
            "fn duplicate_choice (x : SessionChoice) : Bool = \
             match x { SessionAlias |-> True; RightChoice |-> False }",
        )
        .expect_err("the second constructor alias cannot cover the first branch");
    assert!(
        matches!(
            error,
            ElabError::ExhaustivenessError { .. } | ElabError::ReachabilityError { .. }
        ),
        "the checked pattern must reject the duplicate RightChoice: {error:?}"
    );
}

/// Promise class: durable invariant. MEASURED: a public session-binding call
/// cannot make a hidden prelude identity source-visible, whereas a public
/// checked constructor alias is admitted on the same entry path.
#[test]
fn session_alias_does_not_grant_private_prelude_identity() {
    let mut env = ElabEnv::new().expect("prelude");
    let public = env.globals["True"];
    env.bind_session_name("SessionTrue", public)
        .expect("public constructor alias");
    let private = env.prelude_env.buffer_handle_resource_id;
    assert_ne!(public, private);
    let error = env
        .bind_session_name("SessionHidden", private)
        .expect_err("private prelude identity cannot be granted by the public API");
    assert!(
        matches!(error, ElabError::Internal(_)),
        "unexpected error: {error:?}"
    );
    assert!(!env.globals.contains_key("SessionHidden"));
}
