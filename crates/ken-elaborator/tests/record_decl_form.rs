//! Acceptance for the named-field record declaration form.

use ken_elaborator::{ElabEnv, ElabError};
use std::collections::BTreeSet;

fn assert_unresolved_owner(error: ElabError, expected: &str) {
    match error {
        ElabError::UnresolvedCon { name, .. } => assert_eq!(name, expected),
        other => panic!("expected UnresolvedCon for {expected}, got {other:?}"),
    }
}

fn assert_no_instance(error: ElabError, expected_class: &str, expected_type: &str) {
    match error {
        ElabError::NoInstance { class, ty, .. } => {
            assert_eq!(class, expected_class);
            assert_eq!(ty, expected_type);
        }
        other => panic!("expected NoInstance for {expected_class} {expected_type}, got {other:?}"),
    }
}

/// Durable invariant: a record's stored type identity reaches the same borrowed
/// projection view used by `RProj`, while remaining outside class-only storage.
#[test]
fn point_named_projections_are_typed_through_the_shared_owner_registry() {
    let mut env = ElabEnv::empty().expect("prelude");
    env.elaborate_file(
        "record Point { x : Int, y : Int }\n\
         view pointX (p : Point) : Int = p.x\n\
         view pointY (p : Point) : Int = p.y",
    )
    .expect("Point declaration and named projections elaborate");

    let point_id = env.globals["Point"];
    let projection = env
        .class_env
        .projection_by_type_id(point_id)
        .expect("record type identity must own projection metadata");
    assert_eq!(projection.owner_name, "Point");
    assert_eq!(projection.type_id, point_id);
    assert_eq!(projection.head_param, None);
    assert_eq!(projection.field_names, ["x", "y"]);
    assert!(
        env.class_env.class("Point").is_none(),
        "records must not enter class-only instance metadata"
    );
}

/// Durable invariant: named lookup preserves the right-nested field position,
/// and later field types may depend on earlier field values.
#[test]
fn right_nested_third_and_dependent_record_fields_elaborate() {
    let mut env = ElabEnv::empty().expect("prelude");
    env.elaborate_file(
        "record Triple { first : Bool, second : Int, third : Bytes }\n\
         view third (p : Triple) : Bytes = p.third\n\
         record Dependent { carrier : Type, value : carrier }",
    )
    .expect("right-nested and dependent record fields elaborate");
}

/// Durable invariant: adding a record owner does not change class registration,
/// instance construction, or class-field projection.
#[test]
fn class_instance_projection_is_unchanged_in_a_record_program() {
    let mut env = ElabEnv::empty().expect("prelude");
    env.elaborate_file(
        "record Point { x : Int, y : Int }\n\
         class Pick A { select : A }\n\
         instance Pick Bool { select = True }\n\
         view selected (d : Pick Bool) : Bool = d.select",
    )
    .expect("record and existing class/instance projection coexist");

    assert!(env.class_env.class("Pick").is_some());
    assert!(env.class_env.class("Point").is_none());
}

/// Durable invariant: a class and a record may use the same field label while
/// retaining distinct type identities and projection owners.
///
/// MEASURED: both projections type-check and retain distinct owner names and
/// type IDs despite the identical `value` label. CLAIMED: lookup authority is
/// per named-field owner, not per field label. THE GAP: registration and lookup
/// must use the atomic owner entry rather than a field-keyed side structure.
#[test]
fn shared_field_labels_keep_class_and_record_owners_distinct() {
    let mut env = ElabEnv::empty().expect("prelude");
    env.elaborate_file(
        "record Stored { value : Int }\n\
         class Selected a { value : a }\n\
         instance Selected Bool { value = True }\n\
         view storedValue (s : Stored) : Int = s.value\n\
         view selectedValue (d : Selected Bool) : Bool = d.value",
    )
    .expect("shared field labels retain distinct owners");

    let record_id = env.globals["Stored"];
    let record_projection = env
        .class_env
        .projection_by_type_id(record_id)
        .expect("record projection owner");
    let class_projection = env
        .class_env
        .class("Selected")
        .expect("class owner")
        .projection;

    assert_eq!(record_projection.owner_name, "Stored");
    assert_eq!(class_projection.owner_name, "Selected");
    assert_eq!(record_projection.field_names, ["value"]);
    assert_eq!(class_projection.field_names, ["value"]);
    assert_ne!(record_projection.type_id, class_projection.type_id);
    assert_eq!(
        env.class_env.instance_search("Selected", "Bool"),
        env.globals.get("Selected_instance_Bool").copied()
    );

    let class_owners = env
        .class_env
        .class_entries()
        .map(|class| class.projection.owner_name)
        .collect::<BTreeSet<_>>();
    assert!(class_owners.contains("Selected"));
    assert!(!class_owners.contains("Stored"));
}

/// Durable invariant: record owners cannot be reinterpreted by any class-only
/// declaration path, while the corresponding genuine class paths remain live.
///
/// MEASURED: genuine instance, constraint, and derive forms accept, while the
/// same three forms refuse a record owner at their exact error boundaries.
/// CLAIMED: records are structurally absent from class-only consumers. THE GAP:
/// every class consumer must match the closed owner kind before using class
/// metadata or registering an instance.
#[test]
fn records_are_unavailable_to_instance_constraint_and_derive_paths() {
    let mut env = ElabEnv::empty().expect("prelude");
    env.elaborate_file(
        "record Point { x : Int, y : Int }\n\
         class Pick a { select : a }\n\
         instance Pick Bool { select = True }\n\
         fn picked (x : Bool) : Bool where Pick Bool = d.select\n\
         data Flag = Raised\n\
         class Marker a { }\n\
         derive Marker for Flag",
    )
    .expect("positive class instance, constraint, and derive controls elaborate");

    let instance_error = env
        .elaborate_decl("instance Point Bool { x = 0 ; y = 0 }")
        .expect_err("record must not be accepted as an instance class");
    assert_unresolved_owner(instance_error, "Point");

    let constraint_error = env
        .elaborate_decl("fn constrained (x : Int) : Int where Point Int = x")
        .expect_err("record must not be accepted as a class constraint");
    assert_no_instance(constraint_error, "Point", "Int");

    let derive_error = env
        .elaborate_decl("derive Point for Flag")
        .expect_err("record must not be accepted as a derive class");
    assert_unresolved_owner(derive_error, "Point");
}

/// Durable invariant: unknown named fields preserve the exact span-bearing
/// refusal, while positional Sigma projection remains a separate live path.
#[test]
fn unknown_field_refuses_at_exact_span_and_positional_projection_survives() {
    let mut env = ElabEnv::empty().expect("prelude");
    env.elaborate_decl("record Point { x : Int, y : Int }")
        .expect("Point elaborates");

    let bad = "view bad (p : Point) : Int = p.missing";
    let start = bad.find("p.missing").expect("fixture contains projection");
    let error = env
        .elaborate_decl(bad)
        .expect_err("unknown record field must be rejected");
    match error {
        ElabError::UnresolvedCon { name, span } => {
            assert_eq!(name, "missing");
            assert_eq!((span.start, span.end), (start, start + "p.missing".len()));
        }
        other => panic!("expected span-bearing UnresolvedCon, got {other:?}"),
    }

    env.elaborate_file(
        "view pairFirst (p : ((first : Int) × Bool)) : Int = p.1\n\
         view pairSecond (p : ((first : Int) × Bool)) : Bool = p.2",
    )
    .expect("positional .1/.2 projection remains live");
}

/// Durable invariant: reserving `record` does not steal or redirect neighboring
/// declaration keywords or refinement parsing.
#[test]
fn record_keyword_preserves_declaration_neighbours() {
    let mut env = ElabEnv::empty().expect("prelude");
    env.elaborate_file(
        "record Point { x : Int, y : Int }\n\
         class Pick A { select : A }\n\
         instance Pick Bool { select = True }\n\
         module M { const flag : Bool = True }\n\
         def SelfInt = { n : Int | Equal Int n n }",
    )
    .expect("record, class, instance, module, and refinement neighbours elaborate");
}
