//! LANG-INSTANCE-SEARCH-SECOND-PATH acceptance for projection-purity lookup.

use ken_elaborator::{ElabEnv, ElabError};

fn projection_fixture() -> ElabEnv {
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_file(
        "class Quiet A {
             fn step : A -> A
         }
         fn quiet_bool (x : Bool) : Bool = x
         instance Quiet Bool { step = quiet_bool }
         const d : Quiet Bool = Quiet_instance_Bool

         class Effectful A {
             proc step : A ->[FS] A
         }
         proc step_bool (x : Bool) : Bool visits [FS] = x
         instance Effectful Bool { step = step_bool }",
    )
    .expect("projection-purity fixture must elaborate");
    env
}

/// Durable invariant: a sole explicitly named constraint still binds the
/// legacy `d` alias, even when a same-typed global named `d` is in scope.
#[test]
fn sole_explicit_constraint_retains_the_legacy_d_alias_row() {
    let mut env = projection_fixture();
    let result = env.elaborate_decl(
        "fn legacy_d_step (x : Bool) : Bool where (effect : Effectful Bool) =
             d.step x",
    );
    assert!(
        matches!(
            result,
            Err(ElabError::TypeMismatch { ref reason, .. })
                if reason.contains("false purity or effect escape")
                    && reason.contains("EffectEscapes")
                    && reason.contains("FS")
        ),
        "`d` must remain the sole constraint's effectful legacy alias: {result:?}"
    );
}

/// Durable invariant: without a local constraint binder, the previous
/// session's checked `d` stays selectable even after its flat alias is removed.
/// MEASURED: a new source declaration checks and retains that exact global ID.
/// CLAIMED: lexical shadowing does not erase the session binding itself.
/// THE GAP: the fixture must remove only the flat alias, not the session scope.
#[test]
fn previous_session_d_remains_selected_without_a_local_constraint() {
    let mut env = projection_fixture();
    let prior = env
        .globals
        .remove("d")
        .expect("fixture must own a checked d");
    let checked = env
        .elaborate_decl("const ordinary_d : Quiet Bool = d")
        .expect("prior-session d must resolve without a local constraint");
    let (_, body) = env
        .env
        .transparent_body(checked)
        .expect("ordinary_d must be transparent");
    assert!(
        matches!(body, ken_kernel::Term::Const { id, .. } if id == prior),
        "ordinary source must select the previous checked d, not a flat alias: {body:?}"
    );
}

/// Durable invariant: the explicitly spelled local-constraint binder remains
/// effectful independently of its `d` alias.
#[test]
fn explicit_constraint_binder_retains_its_projection_row() {
    let mut env = projection_fixture();
    let result = env.elaborate_decl(
        "fn bound_step (x : Bool) : Bool where (effect : Effectful Bool) =
             effect.step x",
    );
    assert!(
        matches!(
            result,
            Err(ElabError::TypeMismatch { ref reason, .. })
                if reason.contains("false purity or effect escape")
                    && reason.contains("EffectEscapes")
                    && reason.contains("FS")
        ),
        "the local Effectful binder must retain its FS projection row: {result:?}"
    );
}
