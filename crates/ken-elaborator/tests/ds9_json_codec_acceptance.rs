//! DS-9 (`Json` codec) D1 acceptance.
//!
//! This increment binds AC-1 only: the ordinary nested-inductive `Json`
//! family elaborates, and all six public constructors are real globals owned
//! by that family. Codec, cursor, and proof acceptance belongs to later
//! increments.

use ken_elaborator::ElabEnv;
use ken_kernel::Term;

const JSON_KEN_MD: &str = include_str!("../../../catalog/packages/Data/Serialization/Json.ken.md");

#[test]
fn json_and_all_six_constructors_are_real_globals() {
    // Normative compatibility vector (AC-1): names, ownership, order, and the
    // complete field telescope are public. In particular, checking the field
    // terms prevents a non-recursive `List Bool` from masquerading as the
    // required `List Json` array carrier.
    let mut env = ElabEnv::empty().expect("prelude bootstrap");
    env.elaborate_ken_md_file(JSON_KEN_MD)
        .expect("Data/Serialization/Json.ken.md must elaborate");

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
}
