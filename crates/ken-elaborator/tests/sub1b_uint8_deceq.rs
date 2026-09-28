//! SUB-1b acceptance: lawful `DecEq UInt8` and `DecEq Bytes` from one
//! conversion-layer retraction postulate.

#[path = "support/catalog_or.rs"]
mod catalog_or;

use std::collections::BTreeSet;

use ken_elaborator::{ElabEnv, ElabError};
use ken_interp::eval::{apply, eval, EvalStore, EvalVal, ListCharIds};
use ken_kernel::{Decl, GlobalId};
const BYTES_KEYS: &str =
    include_str!("../../../catalog/packages/Data/Binary/BytesKeys.ken.md");

fn dependency_env() -> (ElabEnv, Vec<GlobalId>) {
    let mut env = ElabEnv::new().expect("base env");
    let transport_owned = catalog_or::load_core_logic_compare(&mut env);
    catalog_or::expose_core_logic_transport(&mut env, &transport_owned);
    let lawful_owned = env
        .elaborate_module_from_roots(&[catalog_or::catalog_root()], "Core.Classes.LawfulClasses")
        .expect("LawfulClasses provider must roots-load");
    catalog_or::expose_module(&mut env, "Core.Classes.LawfulClasses");
    (env, lawful_owned)
}

fn env_with_bytes_keys() -> (ElabEnv, Vec<GlobalId>) {
    let (mut env, lawful_owned) = dependency_env();
    env.elaborate_ken_md_file(BYTES_KEYS)
        .expect("BytesKeys package");
    (env, lawful_owned)
}

fn lawful_id(env: &ElabEnv, lawful_owned: &[GlobalId], name: &str) -> GlobalId {
    catalog_or::provider_owned_id(env, lawful_owned, "Core.Classes.LawfulClasses", name)
        .unwrap_or_else(|error| panic!("{name} must belong to the loaded class owner: {error}"))
}

fn assert_transparent_globals(env: &ElabEnv, lawful_owned: &[GlobalId], names: &[&str]) {
    let dec_eq_class = env
        .class_env
        .class("DecEq")
        .expect("DecEq must remain a checked class")
        .projection
        .type_id;
    assert!(
        lawful_owned.contains(&dec_eq_class),
        "DecEq must remain a checked class owned by LawfulClasses"
    );
    for name in names {
        let id = if let Some(head) = name.strip_prefix("DecEq_instance_") {
            let instance = env
                .class_env
                .instance_search("DecEq", head)
                .unwrap_or_else(|| panic!("DecEq {head} must remain registered"));
            assert!(
                lawful_owned.contains(&instance),
                "DecEq {head} must resolve to a LawfulClasses-owned dictionary"
            );
            instance
        } else {
            lawful_id(env, lawful_owned, name)
        };
        assert!(
            env.env.transparent_body(id).is_some(),
            "{name} must be a real kernel-checked transparent declaration"
        );
        assert!(
            !env.env.trusted_base().contains(&id),
            "{name} must not itself enter trusted_base()"
        );
    }
}

fn make_store(env: &ElabEnv) -> EvalStore {
    let mut store = EvalStore::new();
    store.list_char_ids = Some(ListCharIds {
        nil_id: env.prelude_env.nil_id,
        cons_id: env.prelude_env.cons_id,
    });
    store
}

fn eval_bytes_deceq(
    env: &ElabEnv,
    store: &mut EvalStore,
    lawful_owned: &[GlobalId],
    left: &[u8],
    right: &[u8],
) -> bool {
    let id = lawful_id(env, lawful_owned, "bytes_deceq_eq");
    let function = match env.env.lookup(id) {
        Some(Decl::Transparent { body, .. }) => eval(&[], body, &env.env, store),
        other => panic!("expected transparent bytes_deceq_eq, got {other:?}"),
    };
    let applied_left = apply(function, EvalVal::Bytes(left.to_vec()), &env.env, store);
    let value = apply(
        applied_left,
        EvalVal::Bytes(right.to_vec()),
        &env.env,
        store,
    );
    match value {
        EvalVal::Bool(value) => value,
        EvalVal::Ctor { id, .. } if id == env.globals["True"] => true,
        EvalVal::Ctor { id, .. } if id == env.globals["False"] => false,
        other => panic!("expected Bool decision, got {other:?}"),
    }
}

#[test]
fn ac1_trusted_base_delta_is_exactly_uint8_int_retract() {
    let env = ElabEnv::new().expect("base env");
    let id = env.numeric_env.uint8_int_retract_id;
    assert_eq!(env.globals["uint8_int_retract"], id);
    assert!(matches!(env.env.lookup(id), Some(Decl::Opaque { .. })));
    let expected = BTreeSet::from([id]);
    let actual: BTreeSet<_> = env
        .numeric_env
        .uint8_retract_trusted_delta
        .iter()
        .copied()
        .collect();
    assert_eq!(actual, expected, "SUB-1b must add exactly one named entry");
    assert!(
        !env.globals.contains_key("eq_uint8"),
        "Route A's eq_uint8 primitive must remain absent"
    );
}

/// Promise class: durable checked-identity invariant. The real BytesKeys
/// client reaches LawfulClasses-owned checked dictionaries with no new trust.
#[test]
fn ac2_ac3_catalog_derivation_adds_zero_trust_and_has_real_laws() {
    let extracted =
        ken_elaborator::extract_ken_md(BYTES_KEYS).expect("BytesKeys.ken.md must extract");
    assert!(
        !extracted.source.contains("Axiom"),
        "BytesKeys checked Ken must contain zero Axiom literals"
    );

    let (mut env, lawful_owned) = dependency_env();
    let before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    env.elaborate_ken_md_file(BYTES_KEYS)
        .expect("BytesKeys package");
    let after: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(
        after, before,
        "the DecEq UInt8/List UInt8/Bytes chain must add zero further trust"
    );
    assert_transparent_globals(
        &env,
        &lawful_owned,
        &[
            "uint8_to_int_injective",
            "uint8_deceq_eq",
            "uint8_deceq_sound",
            "uint8_deceq_complete",
            "DecEq_instance_UInt8",
            "bytes_to_list_injective",
            "bytes_deceq_eq",
            "bytes_deceq_eq::sound",
            "bytes_deceq_eq::complete",
            "DecEq_instance_Bytes",
        ],
    );
}

/// Promise class: durable behavior invariant. Owner-resolved byte equality
/// distinguishes equal, unequal, prefix, and invalid byte sequences.
#[test]
fn ac4_deceq_bytes_decides_raw_bytes_non_vacuously() {
    let (env, lawful_owned) = env_with_bytes_keys();
    let mut store = make_store(&env);
    for (name, left, right, expected) in [
        (
            "sub1b_equal_invalid",
            &[0xff, 0x00][..],
            &[0xff, 0x00][..],
            true,
        ),
        (
            "sub1b_last_diff",
            &[0xff, 0x01][..],
            &[0xff, 0x02][..],
            false,
        ),
        ("sub1b_empty", &[][..], &[][..], true),
        ("sub1b_prefix", &[0xff][..], &[0xff, 0x00][..], false),
        ("sub1b_invalid_pair", &[0xff][..], &[0xfe][..], false),
    ] {
        let actual = eval_bytes_deceq(&env, &mut store, &lawful_owned, left, right);
        assert_eq!(actual, expected, "{name} produced the wrong decision");
    }
}

#[test]
fn ac5_postulate_is_usable_but_refl_remains_rejected() {
    let mut env = ElabEnv::new().expect("base env");
    catalog_or::load_core_logic_compare(&mut env);
    env.elaborate_decl(
        "theorem use_uint8_retract (x : UInt8) : \
         Equal UInt8 (int_to_uint8_raw (uint8_to_int x)) x = \
         uint8_int_retract x",
    )
    .expect("the registered propositional retraction must be usable");

    let result = env.elaborate_decl(
        "theorem false_refl_uint8_retract (x : UInt8) : \
         Equal UInt8 (int_to_uint8_raw (uint8_to_int x)) x = Refl",
    );
    assert!(
        matches!(
            result,
            Err(ElabError::TypeMismatch { .. }) | Err(ElabError::KernelRejected { .. })
        ),
        "conversion-opaque primitive operations must reject Refl, got {result:?}"
    );
}
