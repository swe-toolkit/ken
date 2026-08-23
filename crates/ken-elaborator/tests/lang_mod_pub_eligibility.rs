//! LANG-MOD-PUB-ELIGIBILITY acceptance controls for the visibility grammar in
//! `spec/30-surface/32-grammar.md` §1 and `33-declarations.md` §4/§8.2.
//!
//! Promise class: durable invariants. Intended extensions may add new eligible
//! declaration kinds only by classifying the new `Decl` variant; structural and
//! anonymous forms remain surface errors when prefixed by `pub`.

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::Term;

fn assert_ineligible_pub(src: &str, expected_kind: &str) {
    match parse_decls(src) {
        Err(ElabError::ParseError { msg, .. }) => assert_eq!(
            msg,
            format!("`pub` is not permitted on {expected_kind}"),
            "the parsed declaration kind must select the pub-eligibility diagnostic"
        ),
        Err(other) => panic!("ineligible pub placement must be a surface ParseError: {other:?}"),
        Ok(decls) => panic!("ineligible pub placement was accepted as {decls:?}"),
    }
}

/// Durable invariant: the visibility gate discriminates rather than rejecting
/// every `pub`; an eligible definition is exported under its provider identity.
#[test]
fn eligible_pub_definition_is_accepted_and_exported() {
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_file(
        "module M { pub const api : Int = 0 } \
         import M (api) \
         const observed : Int = api",
    )
    .expect("eligible public definition and selective import elaborate");

    let provider = env.globals["M.api"];
    let (_, observed) = env
        .env
        .transparent_body(env.globals["observed"])
        .expect("consumer is transparent");
    assert!(
        matches!(observed, Term::Const { id, .. } if id == provider),
        "the exported binding must retain the provider's GlobalId"
    );
}

/// Durable invariant: structural, anonymous, and generated declaration forms
/// never accept and ignore a visibility marker.
#[test]
fn every_parsed_ineligible_pub_placement_is_a_surface_error() {
    for (src, kind) in [
        ("pub import M", "an `import` declaration"),
        ("pub export foo", "an `export` declaration"),
        ("pub module M {}", "a `module` declaration"),
        (
            "pub instance C Int { field = 0 }",
            "an `instance` declaration",
        ),
        ("pub derive C for D", "a `derive` declaration"),
        ("pub prove p : Top", "a `prove` obligation"),
        ("pub law L (x) { field : Top }", "a `law` declaration"),
        (
            "pub foreign f : Int = \"f\" \"libf\" pure",
            "a `foreign` declaration",
        ),
        ("pub temporal t { event }", "a `temporal` obligation"),
        ("pub record R { field : Int }", "a `record` declaration"),
        ("pub program", "a `program` header"),
        ("pub package", "a `package` header"),
        ("pub pub const x : Int = 0", "another `pub` marker"),
    ] {
        assert_ineligible_pub(src, kind);
    }
}

/// Durable invariant: fixity declarations are not yet a parsed `Decl` kind,
/// and therefore cannot bypass eligibility by following `pub`.
#[test]
fn pub_fixity_spelling_is_a_surface_error() {
    match parse_decls("pub infixl 5 plus") {
        Err(ElabError::ParseError { .. }) => {}
        Err(other) => panic!("pub fixity must reject at the surface: {other:?}"),
        Ok(decls) => panic!("pub fixity was accepted as {decls:?}"),
    }
}

/// Durable invariant: `pub proof` remains eligible only when its attached
/// subject is itself public (`33 §8.2`).
#[test]
fn public_attached_proof_still_rejects_a_private_subject() {
    let mut env = ElabEnv::new().expect("base environment");
    let err = env
        .elaborate_file(
            "module M { \
               fn id (x : Int) : Int = x \
               pub proof id_self for id (x : Int) \
                 : Equal Int (id x) x = Refl \
             }",
        )
        .expect_err("public proof with private subject must reject");

    match err {
        ElabError::UnboundName { name, .. } => assert_eq!(name, "id"),
        other => panic!("expected private-subject surface rejection, got {other:?}"),
    }
}
