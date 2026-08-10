//! DS-9 (`Json` codec) D1 acceptance.
//!
//! This increment binds AC-1 only: the ordinary nested-inductive `Json`
//! family elaborates, and all six public constructors are real globals owned
//! by that family. Codec, cursor, and proof acceptance belongs to later
//! increments.

use ken_elaborator::ElabEnv;

const JSON_KEN_MD: &str = include_str!("../../../catalog/packages/Data/Serialization/Json.ken.md");

#[test]
fn json_and_all_six_constructors_are_real_globals() {
    // Normative compatibility vector (AC-1): changing any public constructor
    // name makes its global lookup fail, while checking constructor ownership
    // prevents a pre-existing unrelated global from satisfying the assertion.
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

    for (expected_index, name) in [
        "JsonNull",
        "JsonBool",
        "JsonNumber",
        "JsonString",
        "JsonArray",
        "JsonObject",
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
    }
}
