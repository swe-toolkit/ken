//! Regression controls for carrier confirmation on every term-producing
//! instance-resolution return.

use ken_elaborator::{ElabEnv, ElabError};

fn setup_rebound_construction(with_old_pick: bool) -> ElabEnv {
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_file(
        "class Pick carrier { selected : Bool } \
         class Outer carrier { selected : Bool } \
         data Foo : Type where { MkOldFoo : Foo }",
    )
    .expect("classes and old Foo elaborate from surface source");
    if with_old_pick {
        env.elaborate_decl("instance Pick Foo { selected = True }")
            .expect("old Pick Foo instance elaborates from surface source");
    }
    env.elaborate_decl("instance Outer (List x) where Pick x { selected = d.selected }")
        .expect("generic Outer instance retains its Pick prerequisite");
    env
}

/// Promise class: durable invariant. MEASURED: a second package unit refuses
/// Foo and retains the old carrier and dictionary. CLAIMED: package duplicate
/// refusal does not silently turn into the interactive rebinding path. THE GAP:
/// the session rebinding guards below separately test the stale-ID checks.
#[test]
fn package_route_refuses_rebound_carrier_without_displacing_old_owner() {
    let mut env = setup_rebound_construction(true);
    let old_foo = env.globals["Foo"];
    let old_pick = env.class_env.instance_search("Pick", "Foo");
    let error = env
        .elaborate_decl("data Foo : Type where { MkNewFoo : Foo }")
        .expect_err("a package unit cannot rebind Foo");
    assert!(
        matches!(error, ElabError::DuplicateDefinition { ref name, .. } if name == "Foo"),
        "package duplicate must precede stale-dictionary resolution: {error:?}"
    );
    assert_eq!(
        env.globals["Foo"], old_foo,
        "old checked carrier remains bound"
    );
    assert_eq!(env.class_env.instance_search("Pick", "Foo"), old_pick);
}

#[test]
fn rebound_construction_prerequisite_refuses_before_emitting_a_dictionary_term() {
    let mut env = setup_rebound_construction(true);
    let old_foo = env.globals["Foo"];
    let old_pick = env
        .class_env
        .instance_search("Pick", "Foo")
        .expect("old Pick Foo is registered");

    env.elaborate_session_decl_results_v1("data Foo : Type where { MkNewFoo : Foo }")
        .expect("a later interactive declaration rebinds Foo");
    let new_foo = env.globals["Foo"];
    assert_ne!(old_foo, new_foo, "the carrier identities must differ");
    assert_eq!(
        env.class_env.instance_search("Pick", "Foo"),
        Some(old_pick),
        "the name-keyed registry still exposes the old candidate"
    );

    env.resolution_provenance.clear();
    let result = env.elaborate_decl("const rebound : Bool where Outer (List Foo) = d.selected");
    assert!(
        matches!(
            result,
            Err(ElabError::InstanceCarrierIdentityMismatch {
                ref class,
                ref spelling,
                ..
            }) if class == "Pick" && spelling == "Foo"
        ),
        "the nested old Pick dictionary must be refused against new Foo before it is \
         applied to the generic Outer dictionary: {result:?}"
    );
    assert!(
        !env.globals.contains_key("rebound"),
        "a refused substitution must not leave a checked rebound declaration"
    );
    assert!(
        env.resolution_provenance.is_empty(),
        "a failed carrier confirmation must precede successful provenance append"
    );
}

#[test]
fn rebound_declaration_entry_refuses_the_old_dictionary_identity() {
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_file(
        "class Pick carrier { selected : Bool } \
         data Foo : Type where { MkOldFoo : Foo } \
         instance Pick Foo { selected = True }",
    )
    .expect("old Foo and Pick Foo elaborate from surface source");
    let old_foo = env.globals["Foo"];
    env.elaborate_session_decl_results_v1("data Foo : Type where { MkNewFoo : Foo }")
        .expect("a later interactive declaration rebinds Foo");
    assert_ne!(
        env.globals["Foo"], old_foo,
        "the new carrier has its own checked ID"
    );

    env.resolution_provenance.clear();
    let result = env.elaborate_decl("const rebound_entry : Bool where Pick Foo = d.selected");
    assert!(
        matches!(
            result,
            Err(ElabError::InstanceCarrierIdentityMismatch {
                ref class,
                ref spelling,
                ..
            }) if class == "Pick" && spelling == "Foo"
        ),
        "the declaration-entry surface request must carry new Foo's independently \
         elaborated core carrier into common confirmation: {result:?}"
    );
    assert!(!env.globals.contains_key("rebound_entry"));
    assert!(env.resolution_provenance.is_empty());
}

#[test]
fn absent_old_prerequisite_remains_no_instance() {
    let mut env = setup_rebound_construction(false);
    let old_foo = env.globals["Foo"];
    env.elaborate_session_decl_results_v1("data Foo : Type where { MkNewFoo : Foo }")
        .expect("a later interactive declaration rebinds Foo");
    assert_ne!(
        env.globals["Foo"], old_foo,
        "the new carrier has its own checked ID"
    );

    let result = env.elaborate_decl("const rebound : Bool where Outer (List Foo) = d.selected");
    assert!(
        matches!(
            result,
            Err(ElabError::NoInstance { ref class, ref ty, .. })
                if class == "Pick" && ty == "Foo"
        ),
        "confirmation must not manufacture a candidate when old Pick Foo is absent: \
         {result:?}"
    );
}

#[test]
fn the_same_nested_program_checks_without_rebinding() {
    let mut env = setup_rebound_construction(true);
    let result = env.elaborate_decl("const ordinary : Bool where Outer (List Foo) = d.selected");
    assert!(
        result.is_ok(),
        "the correctly matched old Foo prerequisite must still check: {result:?}"
    );
}

#[test]
fn valid_parameterized_nested_prerequisites_still_check() {
    let mut env = ElabEnv::new().expect("base environment");
    let result = env.elaborate_file(
        "class Pick carrier { selected : carrier } \
         instance Pick Int { selected = 0 } \
         instance Pick Bool { selected = True } \
         instance Pick (Pair a b) where Pick a, Pick b { \
           selected = mk_pair a b da.selected db.selected \
         } \
         const selected_pair : Pair Int Bool \
           where Pick (Pair Int Bool) = d.selected",
    );
    assert!(
        result.is_ok(),
        "carrier confirmation must accept correctly instantiated recursive prerequisites: \
         {result:?}"
    );
}
