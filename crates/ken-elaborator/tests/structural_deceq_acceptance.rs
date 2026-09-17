//! Structural `DecEq` lifting acceptance: the real catalog package registers
//! proof-carrying `Pair` and `List` instances, computes on concrete values,
//! and keeps its neutral proof paths dictionary-directed.

#[path = "support/catalog_or.rs"]
mod catalog_or;

use ken_elaborator::{ElabEnv, trusted_base_delta};
use ken_kernel::env::Decl;

fn mk_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("base env construction failed");
    catalog_or::load_core_logic_compare(&mut env);
    catalog_or::expose_core_logic_transport(&mut env);
    catalog_or::load_derived_fixture(&mut env);
    env
}

fn assert_bool_reduces(env: &mut ElabEnv, name: &str, expression: &str, expected: &str) {
    env.elaborate_decl(&format!("const {name} : Bool = {expression}"))
        .unwrap_or_else(|e| panic!("{name} must elaborate: {e}"));
    env.elaborate_decl(&format!(
        "theorem {name}_reduces : Equal Bool {name} {expected} = Proved"
    ))
    .unwrap_or_else(|e| panic!("{name} must reduce to {expected}: {e}"));
}

#[test]
fn structural_instances_are_checked_transparent_and_zero_delta() {
    let env = mk_env();
    for name in ["DecEq_instance_Pair", "DecEq_instance_List"] {
        let id = env.globals[name];
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{name} must be a checked transparent instance"
        );
        let mut delta = trusted_base_delta(&env.env, id);
        delta.remove(&env.class_env.record_nil_val_id);
        assert!(
            delta.is_empty(),
            "{name} must add no trusted base entries: {delta:?}"
        );
    }
}

#[test]
fn structural_instances_compute_positive_and_negative_bool_examples() {
    let mut env = mk_env();
    let pair_same = "(DecEq_instance_Pair Bool Bool DecEq_instance_Bool DecEq_instance_Bool).eq (mk_pair Bool Bool True False) (mk_pair Bool Bool True False)";
    let pair_distinct = "(DecEq_instance_Pair Bool Bool DecEq_instance_Bool DecEq_instance_Bool).eq (mk_pair Bool Bool True False) (mk_pair Bool Bool False False)";
    let list_same = "(DecEq_instance_List Bool DecEq_instance_Bool).eq (Cons Bool True (Cons Bool False (Nil Bool))) (Cons Bool True (Cons Bool False (Nil Bool)))";
    let list_distinct = "(DecEq_instance_List Bool DecEq_instance_Bool).eq (Cons Bool True (Cons Bool False (Nil Bool))) (Cons Bool False (Cons Bool False (Nil Bool)))";

    assert_bool_reduces(&mut env, "pair_same", pair_same, "True");
    assert_bool_reduces(&mut env, "pair_distinct", pair_distinct, "False");
    assert_bool_reduces(&mut env, "list_same", list_same, "True");
    assert_bool_reduces(&mut env, "list_distinct", list_distinct, "False");
}
