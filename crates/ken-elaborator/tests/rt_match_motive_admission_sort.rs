//! Elaborated-source controls for checked-core Match-motive admission.
//! These assert the body-view boundary rather than checking repository text.

use ken_elaborator::checked_core::{
    checked_core_declaration_body_view, CheckedCoreBodyViewError, CheckedCoreBodyViewSelection,
    StableSymbol, SymbolNamespace,
};
use ken_elaborator::compiler_driver::{
    compile_ken_package_sources, prepare_native_program_sources, CompilerManifest, CompilerSource,
    CompilerTargetKind, TargetSelector,
};

const HOST_AND_PROOF: &str = r#"program capabilities FS APartial

theorem proof_match (b : Bool) : Top =
  match b { True |-> Proved; False |-> Proved }

fn exit_for (b : Bool) : ExitCode =
  match b { True |-> Success; False |-> Failure 7 }

fn input_flag (input : ProcessInput) : Bool =
  match input {
    MkProcessInput arguments _environment _cwd |->
      match arguments { Nil |-> True; Cons _ _ |-> False }
  }

proc main (input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode visits [FS] =
  host_exit APartial (exit_for (input_flag input))
"#;

const COMPUTATIONAL_PROOF: &str = r#"program capabilities FS APartial

fn computational_prop (b : Bool) : Nat =
  let p : Top = match b { True |-> Proved; False |-> Proved } in Zero

proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode visits [FS] =
  host_exit APartial (match computational_prop True {
    Zero |-> Success; Suc _ |-> Failure 7
  })
"#;

const DEPENDENT_TYPE: &str = r#"program capabilities FS APartial

fn result_type (b : Bool) : Type =
  match b { True |-> Nat; False |-> Bool }

fn dependent_type (b : Bool) : result_type b =
  match b { True |-> Zero; False |-> False }

proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode visits [FS] =
  host_exit APartial Success
"#;

fn selected_body(
    package_name: &str,
    source: &str,
    name: &str,
) -> Result<ken_elaborator::checked_core::CheckedCoreDeclarationBodyView, CheckedCoreBodyViewError>
{
    let owner = StableSymbol::declaration(package_name, &[], name);
    let compiled = compile_ken_package_sources(
        &CompilerManifest::new(package_name, Vec::new()),
        vec![CompilerSource::new("src/main.ken", source)],
        TargetSelector::StableSymbol {
            package_identity: StableSymbol::new(
                SymbolNamespace::Module,
                vec![package_name.to_string()],
            ),
            symbol: owner.clone(),
            kind: CompilerTargetKind::Library,
        },
    )
    .expect("source elaboration and target selection must succeed before body-view admission");
    let closure = compiled.closures.first().expect("selected target closure");
    let selection = CheckedCoreBodyViewSelection {
        package_identity: closure.report.package_identity.clone(),
        package_core_semantic_hash: closure.report.package_core_semantic_hash,
        package_artifact_hash: closure.report.package_artifact_hash,
        target_symbol: closure.report.target_symbol.clone(),
        reachable_declarations: closure.reachable_declarations.clone(),
        external_symbols: closure.external_symbols.clone(),
        dependency_semantic_hashes: closure.report.dependency_semantic_hashes.clone(),
    };
    checked_core_declaration_body_view(&compiled.package, &selection, &owner)
}

/// Durable invariant. Measured: the host's input-dependent Bool match survives
/// production normalization; computational proof-only and dependent Type
/// motives reach distinct raw body-view refusal arms. Claimed: admission
/// follows sort, dependence and owner, not a host metadata marker. The gap is
/// reachability: the ascript-first and owner/dependence mutants must redden
/// these assertions; source checking alone is not the admission boundary.
/// A separately selected proof owner is refused because the Library selection
/// omits its `Top` declaration; the host preparation delivers that declaration.
#[test]
fn elaborated_match_sort_and_owner_admission() {
    // Baseline provisioning, not a stack regression repair: comparable RT
    // preparation measured 3,936 KiB peak on a 256 MiB Builder stack, leaving
    // 258,208 KiB numeric headroom. The stack is local, not RUST_MIN_STACK.
    std::thread::Builder::new()
        .stack_size(256 * 1024 * 1024)
        .spawn(|| {
            let preparation = prepare_native_program_sources(
                "rt_motive_host",
                vec![CompilerSource::new("src/main.ken", HOST_AND_PROOF)],
            )
            .expect("normalized non-dependent Type match admits this host");
            assert!(
                !preparation.executable_closure().is_empty(),
                "the selected host must not be a zero-declaration preparation"
            );

            let computational = selected_body(
                "rt_motive_computational",
                COMPUTATIONAL_PROOF,
                "computational_prop",
            )
            .unwrap_err();
            assert!(matches!(
                computational,
                CheckedCoreBodyViewError::UnsupportedProofOnlyMatch { .. }
            ));

            let dependent =
                selected_body("rt_motive_dependent", DEPENDENT_TYPE, "dependent_type").unwrap_err();
            assert!(matches!(
                dependent,
                CheckedCoreBodyViewError::UnsupportedDependentMotive { .. }
            ));

            let proof =
                selected_body("rt_motive_proof", HOST_AND_PROOF, "proof_match").unwrap_err();
            assert!(
                matches!(
                    proof,
                    CheckedCoreBodyViewError::UnsupportedProofOnlyMatch { .. }
                ),
                "an undelivered Top in a selected Library package fails closed"
            );
        })
        .expect("spawn stated-stack preparation worker")
        .join()
        .expect("preparation worker must complete");
}
