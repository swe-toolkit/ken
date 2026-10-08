use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{KernelError, Level};

fn env() -> ElabEnv {
    ElabEnv::new().expect("base environment must build")
}

#[test]
fn undefined_uppercase_constructor_argument_is_unresolved() {
    let mut env = env();
    let error = env
        .elaborate_decl("data D = C Missing")
        .expect_err("an undefined uppercase type must reject");
    assert!(
        matches!(error, ElabError::UnresolvedCon { ref name, .. } if name == "Missing"),
        "expected the unresolved uppercase type, got {error:?}"
    );
    assert!(!env.globals.contains_key("D"));
    assert!(!env.globals.contains_key("C"));
}

#[test]
fn unbound_lowercase_constructor_argument_is_unresolved() {
    let error = env()
        .elaborate_decl("data D = C missing")
        .expect_err("an unbound lowercase type must reject after resolver fallback");
    assert!(
        matches!(error, ElabError::UnresolvedCon { ref name, .. } if name == "missing"),
        "expected the unresolved lowercase type, got {error:?}"
    );
}

/// Promise class: durable invariant (spec 33 §§1, 8.4).
/// MEASURED: a constructor's argument can name a checked data family in
/// either textual order, and both constructors belong to their own families.
/// CLAIMED: dependency order admits forward data references without losing
/// the identities of the two checked families. THE GAP: cycles and missing
/// names are rejected by separate forward-reference acceptance rows.
#[test]
fn data_type_references_follow_dependency_order() {
    let mut later_env = env();
    let ids = later_env
        .elaborate_file("data D = C Later\ndata Later = MkLater")
        .expect("a later data family must be checked before its earlier dependent");
    assert_eq!(ids.len(), 2, "both declarations must elaborate");
    assert_eq!(
        later_env
            .env
            .constructor(later_env.globals["C"])
            .expect("checked C")
            .0
            .id,
        later_env.globals["D"]
    );
    assert_eq!(
        later_env
            .env
            .constructor(later_env.globals["MkLater"])
            .expect("checked MkLater")
            .0
            .id,
        later_env.globals["Later"]
    );

    let mut earlier_env = env();
    let ids = earlier_env
        .elaborate_file("data Earlier = MkEarlier\ndata D = C Earlier")
        .expect("an earlier type declaration must remain available");
    assert_eq!(ids.len(), 2, "both ordered declarations must elaborate");
    assert_eq!(
        earlier_env
            .env
            .constructor(earlier_env.globals["MkEarlier"])
            .expect("checked MkEarlier")
            .0
            .id,
        earlier_env.globals["Earlier"]
    );
    assert_eq!(
        earlier_env
            .env
            .constructor(earlier_env.globals["C"])
            .expect("checked C")
            .0
            .id,
        earlier_env.globals["D"]
    );
}

#[test]
fn ordinary_legacy_type_zero_data_still_elaborates() {
    let id = env()
        .elaborate_decl("data D = C Int")
        .expect("a Type-0 constructor argument must remain valid");
    assert!(id.0 > 0);
}

#[test]
fn legacy_universe_overflow_is_actionable_and_source_attributed() {
    let error = env()
        .elaborate_decl("data D = C { safe : Int, payload : Type }")
        .expect_err("a Type-valued field cannot inhabit a Type-0 family");
    assert!(
        matches!(
            &error,
            ElabError::ConstructorUniverseViolation {
                data,
                constructor,
                argument_name: Some(argument_name),
                argument_index: 1,
                argument_level,
                family_level,
                span,
            } if data == "D"
                && constructor == "C"
                && argument_name == "payload"
                && *argument_level == Level::zero().suc()
                && *family_level == Level::zero()
                && span.start < span.end
        ),
        "expected an attributed surface universe error, got {error:?}"
    );
    let diagnostic = error.to_string();
    assert!(diagnostic.contains("argument 'payload' of 'C'"));
    assert!(diagnostic.contains("universe suc 0"));
    assert!(diagnostic.contains("family 'D' universe 0"));
    assert!(diagnostic.contains("data D : Type n where"));
}

#[test]
fn explicit_type_one_family_accepts_type_payload() {
    env()
        .elaborate_decl("data D : Type 1 where { C : (s : Type) -> D }")
        .expect("the existing explicit Type-1 escape must remain valid");
}

#[test]
fn failed_localization_preserves_the_unattributed_kernel_error() {
    let error = env()
        .elaborate_decl(
            "data D (x : Int Int) : Type where { \
             C : (safe : Int) -> (payload : Type) -> D x }",
        )
        .expect_err("the malformed family must reject without invented attribution");
    assert!(
        matches!(
            &error,
            ElabError::KernelRejected {
                error: KernelError::ConstructorUniverseViolation {
                    argument,
                    family,
                },
                ..
            } if *argument == Level::zero().suc() && *family == Level::zero()
        ),
        "expected the honest unattributed kernel error, got {error:?}"
    );
    let diagnostic = error.to_string();
    for invented in ["<unknown>", "#1", "C.safe"] {
        assert!(
            !diagnostic.contains(invented),
            "diagnostic fabricated source attribution '{invented}': {diagnostic}"
        );
    }
}
