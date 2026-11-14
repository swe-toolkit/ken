//! LANG-DELETE I1b: both generic compiler-driver entries honor explicit roots.
//! The two checked clients select the real catalog Not. Their bare twins must
//! refuse at the missing binding, not pass through a pre-source global.
//! THE GAP: neither public result exposes the premise head's checked owner;
//! the paired bare refusal distinguishes Strict import selection, not an
//! independent read of the body's Not GlobalId.

use std::path::PathBuf;

use ken_elaborator::checked_core::{StableSymbol, SymbolNamespace};
use ken_elaborator::compiler_driver::{
    compile_checked_target_denotation, compile_ken_package_sources, CheckedTargetDenotationError,
    CompilerDriverError, CompilerManifest, CompilerSource, CompilerTargetKind, TargetSelector,
};
use ken_elaborator::ElabError;

const PKG: &str = "lang-delete-rooted-driver";
const CHECKED: &str = "import Core.Logic.Not (Not as CheckedNot)\n\
    fn guarded (n : Int) (d : Int) : Int requires CheckedNot (Equal Int d 0) = n\n\
    fn caller (value : Int) : Int = guarded 1 0";
const BARE: &str = "fn guarded (n : Int) (d : Int) : Int requires Not (Equal Int d 0) = n\n\
    fn caller (value : Int) : Int = guarded 1 0";

fn roots() -> Vec<PathBuf> {
    vec![PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")]
}

fn rooted(text: &str) -> CompilerSource {
    CompilerSource::new("rooted.ken", text).with_roots(roots())
}

fn rooted_literate(text: &str) -> CompilerSource {
    CompilerSource::new(
        "rooted.ken.md",
        format!("# Rooted client\n\n```ken\n{text}\n```\n"),
    )
    .with_roots(roots())
}

fn package(
    sources: Vec<CompilerSource>,
) -> Result<ken_elaborator::compiler_driver::CompilerDriverOutput, CompilerDriverError> {
    compile_ken_package_sources(
        &CompilerManifest::new(PKG, Vec::new()),
        sources,
        TargetSelector::StableSymbol {
            package_identity: StableSymbol::new(SymbolNamespace::Module, vec![PKG.to_string()]),
            symbol: StableSymbol::declaration(PKG, &[], "caller"),
            kind: CompilerTargetKind::Library,
        },
    )
}

#[test]
fn rooted_denotation_selects_checked_not_for_caller() {
    let denotation = compile_checked_target_denotation(PKG, rooted(CHECKED), "caller")
        .expect("rooted denotation must resolve imported checked Not");
    assert_eq!(denotation.target_name(), "caller");
    assert_eq!(
        denotation.target_symbol(),
        &StableSymbol::declaration(PKG, &[], "caller")
    );
}

#[test]
fn rooted_denotation_refuses_bare_not() {
    let error = compile_checked_target_denotation(PKG, rooted(BARE), "caller")
        .expect_err("the explicit root must not admit bare pre-source Not");
    assert!(
        matches!(error, CheckedTargetDenotationError::Driver(CompilerDriverError::Elaboration(
            ElabError::UnboundName { ref name, .. }
        )) if name == "Not"),
        "the bare source must refuse on Not: {error:?}"
    );
}

/// Promise class: durable invariant. MEASURED: a rooted literate source with
/// an explicit checked Not import selects the caller, while the same source
/// without the import refuses on bare Not. CLAIMED: the generic denotation
/// entry prepares a literate source in Strict mode with the selected roots.
/// THE GAP: the result exposes the caller, not the premise's checked owner;
/// the paired refusal distinguishes the import from ambient lookup.
#[test]
fn rooted_literate_denotation_selects_checked_not_for_caller() {
    let denotation = compile_checked_target_denotation(PKG, rooted_literate(CHECKED), "caller")
        .expect("rooted literate denotation must resolve imported checked Not");
    assert_eq!(denotation.target_name(), "caller");
    assert_eq!(
        denotation.target_symbol(),
        &StableSymbol::declaration(PKG, &[], "caller")
    );
}

#[test]
fn rooted_literate_denotation_refuses_bare_not() {
    let error = compile_checked_target_denotation(PKG, rooted_literate(BARE), "caller")
        .expect_err("rooted literate denotation must refuse bare pre-source Not");
    assert!(
        matches!(error, CheckedTargetDenotationError::Driver(CompilerDriverError::Elaboration(
            ElabError::UnboundName { ref name, .. }
        )) if name == "Not"),
        "literate bare source must refuse on Not: {error:?}"
    );
}

#[test]
fn rooted_package_selects_checked_not_for_caller() {
    let package =
        package(vec![rooted(CHECKED)]).expect("rooted package must resolve imported checked Not");
    assert_eq!(package.closures.len(), 1);
    assert_eq!(
        package.closures[0].report.target_symbol,
        StableSymbol::declaration(PKG, &[], "caller")
    );
}

#[test]
fn rooted_package_refuses_bare_not() {
    let error = package(vec![rooted(BARE)])
        .expect_err("the explicit root must not admit bare pre-source Not");
    assert!(
        matches!(error, CompilerDriverError::Elaboration(
            ElabError::UnboundName { ref name, .. }
        ) if name == "Not"),
        "the bare source must refuse on Not: {error:?}"
    );
}

/// Promise class: durable invariant. MEASURED: the generic package entry
/// accepts an imported checked Not in a rooted literate source and refuses a
/// bare Not at UnboundName. CLAIMED: the package path prepares `.ken.md`
/// sources with their selected roots before elaborating the checked fences.
/// THE GAP: the output exposes the caller symbol, not the premise owner; the
/// paired refusal distinguishes checked import from ambient fallback.
#[test]
fn rooted_literate_package_selects_checked_not_for_caller() {
    let output = package(vec![rooted_literate(CHECKED)])
        .expect("rooted literate package must resolve imported checked Not");
    assert_eq!(output.closures.len(), 1);
    assert_eq!(
        output.closures[0].report.target_symbol,
        StableSymbol::declaration(PKG, &[], "caller")
    );
}

#[test]
fn rooted_literate_package_refuses_bare_not() {
    let error = package(vec![rooted_literate(BARE)])
        .expect_err("rooted literate package must refuse bare pre-source Not");
    assert!(
        matches!(error, CompilerDriverError::Elaboration(
            ElabError::UnboundName { ref name, .. }
        ) if name == "Not"),
        "literate bare source must refuse on Not: {error:?}"
    );
}

#[test]
fn rooted_package_refuses_a_rootless_following_source() {
    let first = CompilerSource::new("first.ken", "const anchor : Nat = Zero").with_roots(roots());
    let second = CompilerSource::new("second.ken", "fn caller (value : Int) : Int = value");
    let error = package(vec![first, second])
        .expect_err("a rootless source must not inherit the previous source's roots");
    assert!(
        matches!(error, CompilerDriverError::Elaboration(
            ElabError::ParseError { ref msg, .. }
        ) if msg.contains("catalog roots cannot change")),
        "mixed root ownership must refuse at the boundary: {error:?}"
    );
}
