//! Checked-core package closure for a later source reading an earlier source's
//! checked-but-not-shipped example fence (spec 46 §1.1, §5).

use ken_elaborator::ElabEnv;
use ken_elaborator::checked_core::{StableSymbol, SymbolNamespace};
use ken_elaborator::compiler_driver::{
    CompilerDriverError, CompilerManifest, CompilerSource, CompilerTargetKind, TargetSelector,
    compile_ken_package_sources,
};
use ken_elaborator::erasure::erase_checked_core_package_for_target;

const PKG: &str = "zz_adv_pkgex";
const DECLARATION_SOURCE: &str = "# Package\n\n```ken\nconst base : Bool = True\n```\n\n```ken example\nconst zz_ex : Bool = base\n```\n";

fn decl(name: &str) -> StableSymbol {
    StableSymbol::declaration(PKG, &[], name)
}

fn source_a(text: &str) -> CompilerSource {
    CompilerSource::new("a.ken.md", text)
}

fn source_b(text: &str) -> CompilerSource {
    CompilerSource::new("b.ken", text)
}

fn compile(
    first: &str,
    later: &str,
    kind: CompilerTargetKind,
) -> Result<ken_elaborator::compiler_driver::CompilerDriverOutput, CompilerDriverError> {
    compile_ken_package_sources(
        &CompilerManifest::new(PKG, Vec::new()),
        vec![source_a(first), source_b(later)],
        TargetSelector::StableSymbol {
            package_identity: StableSymbol::new(SymbolNamespace::Module, vec![PKG.to_string()]),
            symbol: decl("main"),
            kind,
        },
    )
}

fn outside_reference(error: CompilerDriverError, referenced: &StableSymbol) {
    let message = error.to_string();
    match error {
        CompilerDriverError::PackageReferenceOutsidePackage {
            declaration,
            referenced: actual,
        } => {
            assert_eq!(declaration, decl("main"), "wrong owner: {message}");
            assert_eq!(&actual, referenced, "wrong reference: {message}");
            assert!(message.contains(&declaration.to_string()));
            assert!(message.contains(&referenced.to_string()));
        }
        other => panic!("expected a typed package-closure refusal, got {other:?}"),
    }
}

/// Promise class: durable invariant. MEASURED: all three target selections
/// refuse before returning any package when the admitted main names the
/// unshipped example. CLAIMED: target kind cannot launder a dangling package
/// reference. THE GAP: the complementary ordinary-binding control below
/// verifies this is not unconditional rejection.
#[test]
fn later_source_example_reference_refuses_before_ok_for_all_target_kinds() {
    for kind in [
        CompilerTargetKind::NonRuntime,
        CompilerTargetKind::Library,
        CompilerTargetKind::Executable,
    ] {
        outside_reference(
            compile(
                DECLARATION_SOURCE,
                "const main : Bool = zz_ex",
                kind.clone(),
            )
            .expect_err("the checked package must be closed before target selection"),
            &decl("zz_ex"),
        );
    }
}

/// Promise class: durable invariant. MEASURED: a source references a binding
/// admitted from the same preceding file, is emitted and erases; a same-source
/// example is still checked and then excluded from declaration admission.
/// CLAIMED: the closure check refuses dangling references, not valid sharing
/// or example execution. THE GAP: fixed package/denotation hash vectors stay
/// pinned by the predecessor's existing, unchanged two-test suite.
#[test]
fn ordinary_predecessor_reference_still_erases_and_example_still_checks() {
    let mut env = ElabEnv::new().expect("prelude");
    let results = env
        .elaborate_ken_md_file_v1(DECLARATION_SOURCE)
        .expect("the example fence must execute and elaborate successfully");
    assert!(results.iter().any(|result| result.name == "zz_ex"));

    let out = compile(
        DECLARATION_SOURCE,
        "const main : Bool = base",
        CompilerTargetKind::NonRuntime,
    )
    .expect("the previous source's admitted base is visible");
    let semantic = &out.package.artifact.semantic;
    assert!(semantic.declarations.contains_key(&decl("base")));
    assert!(semantic.declarations.contains_key(&decl("main")));
    assert!(!semantic.declarations.contains_key(&decl("zz_ex")));
    assert!(!semantic.symbols.contains(&decl("zz_ex")));
    let closure = out.closures.first().expect("selected main closure");
    assert!(closure.reachable_declarations.contains(&decl("base")));
    erase_checked_core_package_for_target(&out.package, closure.reachable_declarations.iter())
        .expect("an admitted predecessor reference must erase");
}

/// Promise class: durable invariant. MEASURED: adding an example-only axiom
/// leaves the emitted semantic input and hash unchanged, even though executing
/// the example changes the live environment's trusted base. CLAIMED: env-wide
/// trust projection excludes example-only roots. THE GAP: this isolates one
/// axiom, not every kind of generated postulate.
#[test]
fn example_only_axiom_does_not_add_trust_to_the_package() {
    let plain = "```ken\nconst base : Bool = True\n```\n";
    let with_axiom =
        "```ken\nconst base : Bool = True\n```\n```ken example\naxiom zz_ex : Top\n```\n";
    let later = "const main : Bool = base";
    let expected = compile(plain, later, CompilerTargetKind::NonRuntime)
        .expect("ordinary package emits")
        .package;
    let actual = compile(with_axiom, later, CompilerTargetKind::NonRuntime)
        .expect("an example axiom checks but is not shipped")
        .package;
    assert_eq!(actual.artifact.semantic, expected.artifact.semantic);
    assert_eq!(actual.core_semantic_hash, expected.core_semantic_hash);
}

/// Promise class: durable invariant. MEASURED: both a former-only use and a
/// constructor elimination from an example family fail with a typed missing
/// package reference. CLAIMED: an example-only family and its constructors
/// cannot escape into admitted declarations. THE GAP: the constructor match
/// encodes the `Elim` family before its scrutinee, so this control observes
/// the absent family, not an independent constructor-only missing-symbol arm.
#[test]
fn later_source_example_family_and_constructor_refs_refuse() {
    const DATA: &str = "# Package\n\n```ken\nconst base : Bool = True\n```\n\n```ken example\ndata Hidden = MkHidden\n```\n";
    outside_reference(
        compile(
            DATA,
            "const main : Type = Hidden",
            CompilerTargetKind::NonRuntime,
        )
        .expect_err("example-declared data type must be unavailable to the package"),
        &decl("Hidden"),
    );
    outside_reference(
        compile(
            DATA,
            "const main : Bool = match MkHidden { MkHidden |-> True }",
            CompilerTargetKind::NonRuntime,
        )
        .expect_err("example-declared constructor must be unavailable to the package"),
        &decl("Hidden"),
    );
}

/// Promise class: durable invariant. MEASURED: module-class dictionary
/// synthesis reaches an example-only instance even when the later source
/// never names the instance, and package emission refuses its absent symbol.
/// CLAIMED: closure covers the class registry's implicit resolution path, not
/// only explicit reference spelling. THE GAP: the same instance placed in an
/// admitted fence must compile; that positive control is paired below.
#[test]
fn later_source_implicit_example_instance_refuses() {
    let class = "module Choices { pub class Select a { selected : a } }\nimport Choices (Select)\n";
    let instance = "instance Select Bool { selected = True }\n";
    let later = "const main : Bool where Select Bool = d.selected";
    let admitted_source = format!("```ken\n{class}{instance}```\n");
    compile(&admitted_source, later, CompilerTargetKind::NonRuntime)
        .expect("the module class and its admitted instance must resolve");

    let example_source = format!("```ken\n{class}```\n```ken example\n{instance}```\n");
    let error = compile(&example_source, later, CompilerTargetKind::NonRuntime)
        .expect_err("an implicit example dictionary must not escape package closure");
    let message = error.to_string();
    match error {
        CompilerDriverError::PackageReferenceOutsidePackage {
            declaration,
            referenced,
        } => {
            assert_eq!(declaration, decl("main"), "wrong owner: {message}");
            assert_ne!(referenced, decl("Choices.Select"));
            assert!(
                referenced.to_string().contains("instance"),
                "refusal must identify the resolved dictionary: {referenced}"
            );
            assert!(message.contains(&referenced.to_string()));
        }
        other => panic!("expected implicit dictionary closure refusal, got {other:?}"),
    }
}
