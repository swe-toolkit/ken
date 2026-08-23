//! LANG-MOD-PUB-ELIGIBILITY acceptance controls for the visibility grammar in
//! `spec/30-surface/32-grammar.md` §1 and `33-declarations.md` §4/§8.2.
//!
//! Promise class: durable invariants. Intended extensions may add new eligible
//! declaration kinds only by classifying the new `Decl` variant; structural and
//! anonymous forms remain surface errors when prefixed by `pub`.

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::env::Decl as KernelDecl;
use ken_kernel::Term;

fn assert_ineligible_pub(src: &str) {
    let ordinary = src
        .strip_prefix("pub ")
        .expect("fixture must be the one-token public mutation of an ordinary declaration");
    parse_decls(ordinary).expect("the same declaration without `pub` must parse");
    match parse_decls(src) {
        Err(ElabError::ParseError { .. }) => {}
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
    for src in [
        "pub import M",
        "pub export foo",
        "pub module M {}",
        "pub instance C Int { field = 0 }",
        "pub prove p : Top",
        "pub law L (x) { field : Top }",
        "pub foreign f : Int = \"f\" \"libf\" pure",
        "pub temporal t { event }",
        "pub record R { field : Int }",
        "pub program",
        "pub package",
        "pub pub const x : Int = 0",
    ] {
        assert_ineligible_pub(src);
    }
}

/// Durable invariant: the normative postfix derive form reaches the existing
/// generator and kernel-check path, while adding `pub` to the derive clause is
/// a surface-stage rejection.
#[test]
fn postfix_derive_accepts_ordinary_and_rejects_public_clause() {
    let mut env = ElabEnv::new().expect("base environment");
    let trust_before = env.env.trusted_base();
    env.elaborate_file("class DecEq a {} data T = MkT derive (DecEq)")
        .expect("ordinary postfix derive must elaborate through the real generator");

    let instance = env.globals["DecEq_instance_T"];
    assert!(
        matches!(
            env.env.lookup(instance),
            Some(KernelDecl::Transparent { .. })
        ),
        "the generated instance must be a kernel-checked transparent definition"
    );
    assert_eq!(
        env.env.trusted_base(),
        trust_before,
        "postfix derive must not add trust"
    );

    match parse_decls("data T = MkT pub derive (DecEq)") {
        Err(ElabError::ParseError { .. }) => {}
        Err(other) => panic!("public postfix derive must reject at the surface: {other:?}"),
        Ok(decls) => panic!("public postfix derive was accepted as {decls:?}"),
    }
}

/// Durable invariant: visibility is not a per-item modifier for either import
/// or facade export; the ordinary item forms remain live positive controls.
#[test]
fn per_item_pub_import_and_export_are_surface_errors() {
    for (ordinary, forbidden) in [
        ("import M (foo)", "import M (pub foo)"),
        ("export M (foo)", "export M (pub foo)"),
    ] {
        parse_decls(ordinary).expect("ordinary selected item must parse");
        match parse_decls(forbidden) {
            Err(ElabError::ParseError { .. }) => {}
            Err(other) => panic!("per-item pub must reject at the surface: {other:?}"),
            Ok(decls) => panic!("per-item pub was accepted as {decls:?}"),
        }
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
