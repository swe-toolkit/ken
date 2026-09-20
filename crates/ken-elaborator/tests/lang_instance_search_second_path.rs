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
