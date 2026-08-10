//! DS-9 (`Json` codec) D1-D2 acceptance.
//!
//! AC-1 binds the ordinary nested-inductive `Json` signature. AC-2 binds the
//! structural `List Char` cursor dictionary, its concrete behavior, its four
//! transparent proof witnesses, and a zero-`trusted_base()` delta.

use std::collections::BTreeSet;

use ken_elaborator::{ElabEnv, NumericLitVal};
use ken_interp::eval::{eval, EvalStore, EvalVal, ListCharIds};
use ken_kernel::{Decl, GlobalId, Term};

const TRANSPORT_KEN_MD: &str =
    include_str!("../../../catalog/packages/Core/Logic/Transport.ken.md");
const COLLECTIONS_KEN_MD: &str =
    include_str!("../../../catalog/packages/Data/Collections/Derived.ken.md");
const LAWFUL_CLASSES_KEN_MD: &str =
    include_str!("../../../catalog/packages/Core/Classes/LawfulClasses.ken.md");
const DIAGNOSTIC_KEN_MD: &str =
    include_str!("../../../catalog/packages/Capability/Diagnostics/Core.ken.md");
const CURSOR_KEN_MD: &str =
    include_str!("../../../catalog/packages/Capability/Parsing/Cursor.ken.md");
const JSON_KEN_MD: &str = include_str!("../../../catalog/packages/Data/Serialization/Json.ken.md");

fn dependency_env() -> ElabEnv {
    let mut env = ElabEnv::empty().expect("prelude bootstrap");
    for (name, source) in [
        ("Core.Logic.Transport", TRANSPORT_KEN_MD),
        ("Data.Collections.Derived", COLLECTIONS_KEN_MD),
        ("Core.Classes.LawfulClasses", LAWFUL_CLASSES_KEN_MD),
        ("Capability.Diagnostics.Core", DIAGNOSTIC_KEN_MD),
        ("Capability.Parsing.Cursor", CURSOR_KEN_MD),
    ] {
        env.elaborate_ken_md_file(source)
            .unwrap_or_else(|error| panic!("{name} dependency must elaborate: {error:?}"));
    }
    env
}

fn assert_transparent_global(env: &ElabEnv, name: &str) {
    let id = *env
        .globals
        .get(name)
        .unwrap_or_else(|| panic!("missing `{name}` global"));
    assert!(
        matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
        "`{name}` must be a real transparent, kernel-checked global"
    );
}

fn lit_to_eval(value: &NumericLitVal, mkdecimalpair_id: GlobalId) -> EvalVal {
    match value {
        NumericLitVal::Int(n) => EvalVal::from(*n),
        NumericLitVal::Float(f) => EvalVal::Float(*f),
        NumericLitVal::Float32(f) => EvalVal::Float32(*f),
        NumericLitVal::Decimal { coeff, exp } => {
            ken_interp::decimal_value(mkdecimalpair_id, *coeff, *exp)
        }
        NumericLitVal::Str(s) => EvalVal::Str(s.clone()),
    }
}

fn make_store(env: &ElabEnv) -> EvalStore {
    let mut store = EvalStore::new();
    let mkdecimalpair_id = env.prelude_env.mkdecimalpair_id;
    for (id, value) in &env.num_values {
        store
            .num_values
            .insert(*id, lit_to_eval(value, mkdecimalpair_id));
    }
    store.list_char_ids = Some(ListCharIds {
        nil_id: env.prelude_env.nil_id,
        cons_id: env.prelude_env.cons_id,
    });
    store
}

fn eval_global(env: &ElabEnv, store: &mut EvalStore, name: &str) -> EvalVal {
    let id = env.globals[name];
    match env.env.lookup(id) {
        Some(Decl::Transparent { body, .. }) => eval(&[], body, &env.env, store),
        other => panic!("`{name}` must be transparent, got {other:?}"),
    }
}

fn nat_count(env: &ElabEnv, value: &EvalVal) -> u64 {
    match value {
        EvalVal::Ctor { id, args, .. } if *id == env.prelude_env.zero_id && args.is_empty() => 0,
        EvalVal::Ctor { id, args, .. } if *id == env.prelude_env.suc_id && args.len() == 1 => {
            1 + nat_count(env, &args[0])
        }
        other => panic!("expected Nat, got {other:?}"),
    }
}

fn list_char_codepoints(env: &ElabEnv, value: &EvalVal) -> Vec<u32> {
    let mut out = Vec::new();
    let mut current = value.clone();
    loop {
        match &current {
            EvalVal::Ctor { id, .. } if *id == env.prelude_env.nil_id => return out,
            EvalVal::Ctor { id, args, .. } if *id == env.prelude_env.cons_id => {
                match &args[1] {
                    EvalVal::Int(n) => out.push(*n as u32),
                    other => panic!("expected Int-typed Char, got {other:?}"),
                }
                current = args[2].clone();
            }
            other => panic!("expected List Char, got {other:?}"),
        }
    }
}

#[test]
fn json_and_all_six_constructors_are_real_globals() {
    // Normative compatibility vector (AC-1): names, ownership, order, and the
    // complete field telescope are public. In particular, checking the field
    // terms prevents a non-recursive `List Bool` from masquerading as the
    // required `List Json` array carrier.
    let mut env = dependency_env();
    let trusted_before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    env.elaborate_ken_md_file(JSON_KEN_MD)
        .expect("Data/Serialization/Json.ken.md must elaborate");
    let trusted_after: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(
        trusted_after, trusted_before,
        "the Json carrier, character cursor, and cursor laws must add zero trusted declarations"
    );

    let json_id = *env.globals.get("Json").expect("missing `Json` global");
    let declaration = env
        .env
        .inductive(json_id)
        .expect("`Json` must be an inductive family");
    assert_eq!(
        declaration.constructors.len(),
        6,
        "`Json` must have exactly six constructors"
    );

    let global = |name: &str| {
        *env.globals
            .get(name)
            .unwrap_or_else(|| panic!("missing `{name}` prelude global"))
    };
    let bool_type = Term::indformer(global("Bool"), vec![]);
    let int_type = Term::const_(global("Int"), vec![]);
    let string_type = Term::const_(global("String"), vec![]);
    let json_type = Term::indformer(json_id, vec![]);
    let list_type = Term::indformer(global("List"), vec![]);
    let pair_type = Term::const_(global("Pair"), vec![]);
    let list_json = Term::app(list_type.clone(), json_type.clone());
    let pair_string_json = Term::app(Term::app(pair_type, string_type.clone()), json_type.clone());
    let list_pair_string_json = Term::app(list_type, pair_string_json);

    for (expected_index, (name, signature, expected_args)) in [
        ("JsonNull", "JsonNull : Json", vec![]),
        ("JsonBool", "JsonBool : Bool → Json", vec![bool_type]),
        ("JsonNumber", "JsonNumber : Int → Json", vec![int_type]),
        (
            "JsonString",
            "JsonString : String → Json",
            vec![string_type],
        ),
        ("JsonArray", "JsonArray : List Json → Json", vec![list_json]),
        (
            "JsonObject",
            "JsonObject : List (Pair String Json) → Json",
            vec![list_pair_string_json],
        ),
    ]
    .into_iter()
    .enumerate()
    {
        let constructor_id = *env
            .globals
            .get(name)
            .unwrap_or_else(|| panic!("missing `{name}` constructor global"));
        let (owner, actual_index) = env
            .env
            .constructor(constructor_id)
            .unwrap_or_else(|| panic!("`{name}` must be a registered constructor"));
        assert_eq!(owner.id, json_id, "`{name}` must belong to `Json`");
        assert_eq!(
            actual_index, expected_index,
            "`{name}` must retain its declared constructor position"
        );
        assert_eq!(
            owner.constructors[actual_index].args, expected_args,
            "`{name}` must retain the field shape `{signature}`"
        );
    }

    // Durable invariant (AC-2): the dictionary and each component proof are
    // transparent checked terms. A proof replaced by `Axiom` changes the
    // trusted-base set above; an unresolved hole prevents package elaboration.
    for name in [
        "char_cursor_remaining",
        "char_cursor_peek",
        "char_cursor_advance",
        "char_cursor_locate",
        "char_cursor_ops",
        "char_cursor_lt_suc",
        "char_cursor_peek_has_remaining",
        "char_cursor_advance_progress",
        "char_cursor_end_valid",
        "char_cursor_laws",
    ] {
        assert_transparent_global(&env, name);
    }

    // Durable invariant (AC-2): re-check every law witness at its literal
    // public type, independently of its declaration annotation.
    for declaration in [
        r#"theorem ds9_cursor_peek_law
             : CursorPeekHasRemaining (List Char) Char Nat char_cursor_ops =
           char_cursor_peek_has_remaining"#,
        r#"theorem ds9_cursor_advance_law
             : CursorAdvanceProgress (List Char) Char Nat char_cursor_ops =
           char_cursor_advance_progress"#,
        r#"theorem ds9_cursor_end_law
             : CursorEndValid (List Char) Char Nat char_cursor_ops =
           char_cursor_end_valid"#,
        r#"theorem ds9_cursor_laws
             : CursorLaws (List Char) Char Nat char_cursor_ops =
           char_cursor_laws"#,
    ] {
        env.elaborate_decl(declaration)
            .unwrap_or_else(|error| panic!("cursor acceptance probe must elaborate: {error:?}"));
    }

    // Durable invariant (AC-2): exercise the actual dictionary on non-empty
    // and empty input. These probes call the generic selectors, not the
    // implementation helpers directly.
    env.elaborate_file(
        r#"
        const ds9_cursor_input : List Char =
          Cons Char (65 : Int) (Cons Char (66 : Int) (Nil Char))
        const ds9_cursor_remaining_result : Nat =
          cursor_remaining (List Char) Char Nat char_cursor_ops ds9_cursor_input
        const ds9_cursor_location_result : Nat =
          cursor_locate (List Char) Char Nat char_cursor_ops ds9_cursor_input
        const ds9_cursor_peek_result : Option Char =
          cursor_peek (List Char) Char Nat char_cursor_ops ds9_cursor_input
        const ds9_cursor_advance_result : List Char =
          cursor_advance (List Char) Char Nat char_cursor_ops ds9_cursor_input
        const ds9_cursor_empty : List Char = Nil Char
        const ds9_cursor_empty_remaining_result : Nat =
          cursor_remaining (List Char) Char Nat char_cursor_ops ds9_cursor_empty
        const ds9_cursor_empty_location_result : Nat =
          cursor_locate (List Char) Char Nat char_cursor_ops ds9_cursor_empty
        const ds9_cursor_empty_peek_result : Option Char =
          cursor_peek (List Char) Char Nat char_cursor_ops ds9_cursor_empty
        const ds9_cursor_empty_advance_result : List Char =
          cursor_advance (List Char) Char Nat char_cursor_ops ds9_cursor_empty
        "#,
    )
    .expect("cursor behavior fixtures must elaborate");

    let mut store = make_store(&env);
    assert_eq!(
        nat_count(
            &env,
            &eval_global(&env, &mut store, "ds9_cursor_remaining_result")
        ),
        2
    );
    assert_eq!(
        nat_count(
            &env,
            &eval_global(&env, &mut store, "ds9_cursor_location_result")
        ),
        2
    );

    let peek = eval_global(&env, &mut store, "ds9_cursor_peek_result");
    match peek {
        EvalVal::Ctor { id, args, .. } if id == env.globals["Some"] => {
            assert!(matches!(args[1], EvalVal::Int(65)));
        }
        other => panic!("non-empty cursor must peek `Some 65`, got {other:?}"),
    }
    assert_eq!(
        list_char_codepoints(
            &env,
            &eval_global(&env, &mut store, "ds9_cursor_advance_result")
        ),
        vec![66]
    );
    assert_eq!(
        nat_count(
            &env,
            &eval_global(&env, &mut store, "ds9_cursor_empty_remaining_result")
        ),
        0
    );
    assert_eq!(
        nat_count(
            &env,
            &eval_global(&env, &mut store, "ds9_cursor_empty_location_result")
        ),
        0
    );
    let empty_peek = eval_global(&env, &mut store, "ds9_cursor_empty_peek_result");
    assert!(
        matches!(empty_peek, EvalVal::Ctor { id, .. } if id == env.globals["None"]),
        "empty cursor must peek `None`, got {empty_peek:?}"
    );
    assert_eq!(
        list_char_codepoints(
            &env,
            &eval_global(&env, &mut store, "ds9_cursor_empty_advance_result")
        ),
        Vec::<u32>::new()
    );
}
