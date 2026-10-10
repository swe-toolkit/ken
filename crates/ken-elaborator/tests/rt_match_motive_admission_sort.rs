//! Elaborated-source controls for checked-core Match-motive admission.
//! `spec/40-runtime/46-checked-core-package.md` §1.2 defines the body-view
//! fail-closed boundary; `spec/10-kernel/14-inductive.md` §3 admits Ω motives.

use ken_elaborator::checked_core::{
    checked_core_declaration_body_view, emit_checked_core_package, CheckedCoreBodyTerm,
    CheckedCoreBodyViewError, CheckedCoreBodyViewSelection, StableSymbol, SymbolNamespace,
};
use ken_elaborator::compiler_driver::{
    compile_ken_package_sources, prepare_native_program_sources, CompilerDriverOutput,
    CompilerManifest, CompilerSource, CompilerTargetKind, TargetSelector,
};
use ken_elaborator::omega_erasure::OmegaErasurePlan;

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

fn selected_package(
    package_name: &str,
    source: &str,
    name: &str,
) -> (
    CompilerDriverOutput,
    CheckedCoreBodyViewSelection,
    StableSymbol,
) {
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
    (compiled, selection, owner)
}

fn selected_body(
    package_name: &str,
    source: &str,
    name: &str,
) -> Result<ken_elaborator::checked_core::CheckedCoreDeclarationBodyView, CheckedCoreBodyViewError>
{
    let (compiled, selection, owner) = selected_package(package_name, source, name);
    checked_core_declaration_body_view(&compiled.package, &selection, &owner)
}

/// Durable invariant. MEASURED: the host's input-dependent Bool match survives
/// production normalization; a proof-only Let is admitted with its checked Ω
/// plan but refused on the same package with a present empty plan, while a
/// dependent Type motive still reaches its separate body-view refusal arm.
/// CLAIMED: checked erasure, sort, dependence and owner govern admission, not
/// a host metadata marker. THE GAP: the empty-plan copy must reach the proof
/// motive arm rather than an earlier package-identity or plan-count refusal;
/// source checking alone is not the admission boundary.
#[test]
fn elaborated_match_sort_and_selected_delivery_boundary() {
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

            let (compiled, selection, owner) = selected_package(
                "rt_motive_computational",
                COMPUTATIONAL_PROOF,
                "computational_prop",
            );
            checked_core_declaration_body_view(&compiled.package, &selection, &owner)
                .expect("a checked proof Let is erased before runtime body admission");

            // Change only the plan for this declaration in the SAME checked
            // package. Re-emission binds the new plan to fresh semantic and
            // artifact hashes, so the empty-plan copy reaches the decoder's
            // proof-motive refusal rather than a stale-identity guard.
            let mut unplanned = compiled.package.clone();
            let previous = unplanned
                .artifact
                .semantic
                .omega_erasure_plans
                .insert(owner.clone(), OmegaErasurePlan::default())
                .expect("production supplied this declaration's plan");
            assert!(
                !previous.erased_subterms.is_empty(),
                "the positive half must actually erase a proof subterm"
            );
            unplanned = emit_checked_core_package(unplanned.header.clone(), unplanned.artifact)
                .expect("a present empty plan still admits this package");
            let mut unplanned_selection = selection.clone();
            unplanned_selection.package_core_semantic_hash = unplanned.core_semantic_hash;
            unplanned_selection.package_artifact_hash = unplanned.artifact_hash;
            let without_erasure =
                checked_core_declaration_body_view(&unplanned, &unplanned_selection, &owner)
                    .expect_err("the same proof match without erasure must refuse");
            assert!(matches!(
                without_erasure,
                CheckedCoreBodyViewError::UnsupportedProofOnlyMatch { symbol, .. }
                    if symbol == owner
            ));

            let dependent =
                selected_body("rt_motive_dependent", DEPENDENT_TYPE, "dependent_type").unwrap_err();
            assert!(matches!(
                dependent,
                CheckedCoreBodyViewError::UnsupportedDependentMotive { .. }
            ));

            // The selected Library's proof-valued owner is wholly erased:
            // the checked Ω plan makes the undelivered Top irrelevant at
            // runtime without changing its kernel-checked declaration.
            let proof = selected_body("rt_motive_proof", HOST_AND_PROOF, "proof_match")
                .expect("a proof-valued owner is erased before body-view admission");
            assert!(matches!(
                proof.body,
                CheckedCoreBodyTerm::ErasedOmegaSubterm
            ));
        })
        .expect("spawn stated-stack preparation worker")
        .join()
        .expect("preparation worker must complete");
}
