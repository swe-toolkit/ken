//! Structural `DecEq` lifting acceptance: the real catalog package registers
//! proof-carrying `Pair` and `List` instances, computes on concrete values,
//! and keeps its neutral proof paths dictionary-directed.

#[path = "support/catalog_or.rs"]
mod catalog_or;

use ken_elaborator::{ElabEnv, trusted_base_delta};
use ken_kernel::{env::Decl, GlobalId};

fn mk_env() -> (ElabEnv, Vec<GlobalId>) {
    let mut env = ElabEnv::new().expect("base env construction failed");
    let transport_owned = catalog_or::load_core_logic_compare(&mut env);
    catalog_or::expose_core_logic_transport(&mut env, &transport_owned);
    let lawful_owned = env
        .elaborate_module_from_roots(&[catalog_or::catalog_root()], "Core.Classes.LawfulClasses")
        .expect("the class owner must roots-load in this environment");
    catalog_or::load_derived_fixture(&mut env);
    (env, lawful_owned)
}

fn assert_bool_reduces(env: &mut ElabEnv, name: &str, expression: &str, expected: &str) {
    env.elaborate_decl(&format!("const {name} : Bool = {expression}"))
        .unwrap_or_else(|e| panic!("{name} must elaborate: {e}"));
    env.elaborate_decl(&format!(
        "theorem {name}_reduces : Equal Bool {name} {expected} = Proved"
    ))
    .unwrap_or_else(|e| panic!("{name} must reduce to {expected}: {e}"));
}

/// Promise class: durable checked-identity invariant. Both owner-checked
/// structural dictionaries remain transparent and add no trust.
#[test]
fn structural_instances_are_checked_transparent_and_zero_delta() {
    let (env, lawful_owned) = mk_env();
    let dec_eq_class = env
        .class_env
        .class("DecEq")
        .expect("DecEq must remain a checked class")
        .projection
        .type_id;
    assert!(lawful_owned.contains(&dec_eq_class), "the class owner must own DecEq");
    for head in ["Pair", "List"] {
        let id = env
            .class_env
            .instance_search("DecEq", head)
            .unwrap_or_else(|| panic!("DecEq {head} must remain registered"));
        assert!(lawful_owned.contains(&id), "DecEq {head} must be owner-checked");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "DecEq {head} must be a checked transparent instance"
        );
        let mut delta = trusted_base_delta(&env.env, id);
        delta.remove(&env.class_env.record_nil_val_id);
        assert!(
            delta.is_empty(),
            "DecEq {head} must add no trusted base entries: {delta:?}"
        );
    }
}

/// Promise class: durable behavior invariant. Public DecEq resolution reaches
/// the owned Pair/List dictionaries and separates equal from unequal values.
#[test]
fn structural_instances_compute_positive_and_negative_bool_examples() {
    let (mut env, lawful_owned) = mk_env();
    let dec_eq_class = env
        .class_env
        .class("DecEq")
        .expect("DecEq must remain a checked class")
        .projection
        .type_id;
    assert!(lawful_owned.contains(&dec_eq_class), "the class owner must own DecEq");
    for head in ["Bool", "Pair", "List"] {
        let id = env
            .class_env
            .instance_search("DecEq", head)
            .unwrap_or_else(|| panic!("DecEq {head} must remain registered"));
        assert!(lawful_owned.contains(&id), "DecEq {head} must be owner-checked");
    }
    env.elaborate_file(
        "import Core.Classes.LawfulClasses (DecEq)\n\
         fn structural_pair_eq (x : Pair Bool Bool) (y : Pair Bool Bool) : Bool \
           where DecEq (Pair Bool Bool) = d.eq x y\n\
         fn structural_list_eq (x : List Bool) (y : List Bool) : Bool \
           where DecEq (List Bool) = d.eq x y",
    )
    .expect("public DecEq import must resolve the structural dictionaries");
    let pair_same = "structural_pair_eq (mk_pair Bool Bool True False) (mk_pair Bool Bool True False)";
    let pair_distinct = "structural_pair_eq (mk_pair Bool Bool True False) (mk_pair Bool Bool False False)";
    let list_same = "structural_list_eq (Cons Bool True (Cons Bool False (Nil Bool))) (Cons Bool True (Cons Bool False (Nil Bool)))";
    let list_distinct = "structural_list_eq (Cons Bool True (Cons Bool False (Nil Bool))) (Cons Bool False (Cons Bool False (Nil Bool)))";

    assert_bool_reduces(&mut env, "pair_same", pair_same, "True");
    assert_bool_reduces(&mut env, "pair_distinct", pair_distinct, "False");
    assert_bool_reduces(&mut env, "list_same", list_same, "True");
    assert_bool_reduces(&mut env, "list_distinct", list_distinct, "False");

    for (case, head) in [
        ("pair_same", "Pair"),
        ("pair_distinct", "Pair"),
        ("list_same", "List"),
        ("list_distinct", "List"),
    ] {
        let instance = env.class_env.instance_search("DecEq", head).unwrap();
        let wrapper = env.globals[if head == "Pair" {
            "structural_pair_eq"
        } else {
            "structural_list_eq"
        }];
        let wrapper_references = catalog_or::declaration_references(
            env.env.lookup(wrapper).expect("the public-client wrapper must exist"),
        );
        let case_references = catalog_or::declaration_references(
            env.env.lookup(env.globals[case]).expect("checked observation must exist"),
        );
        assert!(
            wrapper_references.contains(&instance) && case_references.contains(&wrapper),
            "{case} must call the checked wrapper that resolves the owned DecEq {head} dictionary"
        );
    }
}
