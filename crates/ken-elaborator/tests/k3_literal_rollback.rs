//! K3 rollback lifetime: a freed literal id cannot prove facts about a later
//! foreign String or override the interpreter's value of a later Int.
//! Promise class: durable invariants, with adjacent reaching controls.

use ken_elaborator::{ElabEnv, ElabError, NumericLitVal};
use ken_interp::eval::{eval, EvalStore, EvalVal};
use ken_kernel::{KernelError, Term};

fn rolled_back() -> ElabEnv {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_decl("const zz_lit : String = \"zz\"")
        .expect("initial checked literal");
    let error = env
        .elaborate_decl("fn bad (s : String) : String = bad \"zz\"")
        .expect_err("SCT rejects non-decreasing self-call");
    assert!(
        matches!(error, ElabError::KernelRejected { error: KernelError::NotTerminating(_), .. }),
        "the rollback fixture must reach the SCT gate: {error:?}"
    );
    env
}

fn assert_refl_refused(env: &mut ElabEnv, name: &str) {
    let src = format!(
        "theorem bogus_{name} : Equal (List Char) (string_to_list_char {name}) (string_to_list_char zz_lit) = Refl"
    );
    let err = env.elaborate_decl(&src).expect_err("false Refl must be rejected");
    assert!(
        matches!(&err, ElabError::TypeMismatch { reason, .. } if reason == "Refl: the two sides of the goal are not convertible"),
        "expected the Refl obligation, not an earlier error: {err:?}"
    );
}

fn make_store(env: &ElabEnv) -> EvalStore {
    let mut store = EvalStore::new();
    for (&id, value) in &env.num_values {
        let runtime = match value {
            NumericLitVal::Int(n) => EvalVal::from(n.clone()),
            NumericLitVal::Str(s) => EvalVal::Str(s.clone()),
            _ => continue,
        };
        store.num_values.insert(id, runtime);
    }
    store
}

fn eval_const(env: &ElabEnv, name: &str) -> EvalVal {
    let mut store = make_store(env);
    eval(
        &[],
        &Term::const_(env.globals[name], vec![]),
        &env.env,
        &mut store,
    )
}

#[test]
fn reused_foreign_id_cannot_inherit_a_checked_string_view() {
    let mut env = rolled_back();
    for name in ["p0", "p1", "p2"] {
        env.elaborate_decl(&format!(
            "foreign {name} : String = \"sym_{name}\" \"libc.so\" pure"
        ))
        .expect("foreign declarations inhabit neutral String");
    }
    assert!(env.num_values.get(&env.globals["p1"]).is_none());
    for name in ["p0", "p1", "p2"] {
        assert_refl_refused(&mut env, name);
    }
    env.elaborate_decl("theorem genuine_reflexive : Equal (List Char) (string_to_list_char p1) (string_to_list_char p1) = Refl")
        .expect("the neutral view is live and reflexivity is still valid");
}

#[test]
fn reused_int_id_observes_its_own_value_not_an_old_string() {
    let mut env = rolled_back();
    for (name, value) in [("i0", 6), ("i1", 7), ("i2", 8)] {
        env.elaborate_decl(&format!("const {name} : Int = {value}"))
            .expect("a later definition is checked independently");
        assert_eq!(eval_const(&env, name), EvalVal::Int(value));
    }
    assert!(env.num_values.get(&env.globals["i1"]).is_none());
}

#[test]
fn fresh_string_and_non_string_literals_after_rollback_keep_own_values() {
    let mut env = rolled_back();
    env.elaborate_decl("const fresh_after : String = \"az\"")
        .expect("fresh checked String");
    env.elaborate_decl("const equal_after : String = \"az\"")
        .expect("independent occurrence of same value");
    env.elaborate_decl("theorem fresh_view_good : Equal (List Char) (string_to_list_char fresh_after) (string_to_list_char equal_after) = Refl")
        .expect("the newly declared literal has its own checked view");
    let wrong = env.elaborate_decl("theorem wrong_fresh_view : Equal (List Char) (string_to_list_char fresh_after) (string_to_list_char zz_lit) = Refl").expect_err("different checked strings cannot prove equality");
    assert!(matches!(&wrong, ElabError::TypeMismatch { reason, .. } if reason == "Refl expects an `Eq`-shaped goal"), "unexpected fresh-view refusal: {wrong:?}");
    assert_eq!(eval_const(&env, "fresh_after"), EvalVal::Str("az".into()));
    env.elaborate_decl("const fresh_int : Int = 9")
        .expect("fresh non-String value");
    assert_eq!(eval_const(&env, "fresh_int"), EvalVal::Int(9));
}
