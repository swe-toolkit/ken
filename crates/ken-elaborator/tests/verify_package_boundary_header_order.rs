//! A package has one admission-boundary file identity, regardless of source
//! order. Contract: spec 33 §3.2.1; VERIFY-PACKAGE-BOUNDARY-HEADER-ORDER.

use ken_elaborator::checked_core::{StableSymbol, SymbolNamespace};
use ken_elaborator::compiler_driver::{
    compile_ken_package_sources, prepare_native_program_sources, CompilerDriverError,
    CompilerDriverOutput, CompilerManifest, CompilerSource, CompilerTargetKind,
    NativeProgramBuildError, NativeProgramPreparationV1, TargetSelector,
};

const PKG: &str = "verify_boundary_header_order";
const FULL: &str = "program capabilities FS AFull\n\
    fn main (_input : ProcessInput) (_caps : ProgramCaps AFull) : HostIO AFull ExitCode = host_exit AFull Success";
const NONE: &str = "program capabilities FS ANone\nconst zz_other : Nat = Zero";
const SAME: &str = "program capabilities FS AFull\nconst zz_other : Nat = Zero";
const PACKAGE: &str = "package\nconst zz_other : Nat = Zero";
const BARE: &str = "const zz_other : Nat = Zero";
const ONLY_EXAMPLE_HEADER: &str =
    "```ken example\nprogram capabilities FS ANone\nconst zz_example : Nat = Zero\n```\n";

fn full() -> CompilerSource {
    CompilerSource::new("full.ken", FULL)
}

fn other(name: &str, text: &str) -> CompilerSource {
    CompilerSource::new(name, text)
}

fn package(sources: Vec<CompilerSource>) -> Result<CompilerDriverOutput, CompilerDriverError> {
    compile_ken_package_sources(
        &CompilerManifest::new(PKG, Vec::new()),
        sources,
        TargetSelector::StableSymbol {
            package_identity: StableSymbol::new(SymbolNamespace::Module, vec![PKG.to_string()]),
            symbol: StableSymbol::declaration(PKG, &[], "main"),
            kind: CompilerTargetKind::Executable,
        },
    )
}

fn native(
    sources: Vec<CompilerSource>,
) -> Result<NativeProgramPreparationV1, NativeProgramBuildError> {
    prepare_native_program_sources(PKG, sources)
}

fn assert_native_refusal(first: CompilerSource, second: CompilerSource) {
    let name_first = first.name.clone();
    let name_second = second.name.clone();
    let native_result = native(vec![first, second]);
    assert!(
        matches!(
            &native_result,
            Err(NativeProgramBuildError::Driver(
                CompilerDriverError::DuplicateBoundaryHeader {
                    first_source,
                    second_source,
                }
            )) if first_source == &name_first && second_source == &name_second
        ),
        "native must refuse the second header before admission: {native_result:?}"
    );
}

fn assert_refusal_in_both_routes(first: CompilerSource, second: CompilerSource) {
    let name_first = first.name.clone();
    let name_second = second.name.clone();
    let package_result = package(vec![first.clone(), second.clone()]);
    assert!(
        matches!(
            &package_result,
            Err(CompilerDriverError::DuplicateBoundaryHeader {
                first_source,
                second_source,
            }) if first_source == &name_first && second_source == &name_second
        ),
        "package must refuse a second header before target selection: {package_result:?}"
    );
    assert_native_refusal(first, second);
}

/// Promise class: durable invariant. MEASURED: the two exact base inputs,
/// reversed, each yield a typed duplicate-boundary refusal naming both source
/// paths before package target selection or native main admission. CLAIMED:
/// source order cannot grant AFull to a package that also claims ANone.
/// THE GAP: the agreeing and mixed-kind controls below test other header pairs.
#[test]
fn conflicting_program_boundaries_refused_in_both_orders_and_routes() {
    let none = other("none.ken", NONE);
    assert_refusal_in_both_routes(none.clone(), full());
    assert_refusal_in_both_routes(full(), none);
}

/// Promise class: durable invariant. MEASURED: native refusal for both orders
/// independent of package-target selection. CLAIMED: an unguarded overwrite
/// cannot escape the admission gate in the ANone-then-AFull order. THE GAP:
/// the paired package test verifies the separate generic package loop.
#[test]
fn native_conflicting_boundaries_refuse_independently_of_package_selection() {
    let none = other("none.ken", NONE);
    assert_native_refusal(none.clone(), full());
    assert_native_refusal(full(), none);
}

/// Promise class: durable invariant. MEASURED: matching FS declarations and
/// mixed package/program kinds also refuse with exact two-source provenance.
/// CLAIMED: every second boundary is refused, not just conflicting authority.
/// THE GAP: no literal enumerates all possible authority or admits spellings.
#[test]
fn even_agreeing_or_mixed_kind_boundaries_refuse() {
    for second in [other("same.ken", SAME), other("package.ken", PACKAGE)] {
        assert_refusal_in_both_routes(second.clone(), full());
        assert_refusal_in_both_routes(full(), second);
    }
}

/// Promise class: durable invariant. MEASURED: one headed source alone and
/// with an unrelated headerless source in both orders prepares a native
/// program with unchanged authority/main symbol and builds the checked package.
/// CLAIMED: the new guard preserves a single boundary across package order.
/// THE GAP: the checked-example-only header needs the separate row below.
#[test]
fn headerless_source_never_erases_the_only_boundary() {
    let baseline = native(vec![full()]).expect("one headed source is admitted");
    let baseline_main = baseline.plan().main().clone();
    let bare = other("bare.ken", BARE);
    for sources in [vec![bare.clone(), full()], vec![full(), bare.clone()]] {
        package(sources.clone()).expect("package with one headed source builds");
        let prepared = native(sources).expect("native with one headed source prepares");
        assert_eq!(prepared.plan().authority_name(), "AFull");
        assert_eq!(prepared.plan().main(), &baseline_main);
    }
}

/// Promise class: durable invariant. MEASURED: a header inside a checked
/// `ken example` is not a second source boundary, in either source order;
/// native still prepares and generic package compilation reaches the target.
/// CLAIMED: package refusal does not reject lawful checked-fence headers.
/// THE GAP: an independent CLI boundary suite covers other fence roles.
#[test]
fn example_only_header_is_not_a_second_package_boundary() {
    let example = other("examples.ken.md", ONLY_EXAMPLE_HEADER);
    for sources in [vec![example.clone(), full()], vec![full(), example.clone()]] {
        package(sources.clone()).expect("checked example must not refuse package boundary");
        let prepared = native(sources).expect("checked example must not refuse native boundary");
        assert_eq!(prepared.plan().authority_name(), "AFull");
    }
}
