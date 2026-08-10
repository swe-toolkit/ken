use std::collections::BTreeSet;

use ken_elaborator::checked_core::{
    emit_checked_core_package, representative_checked_core_fixtures, AssumptionTrustKind,
    AssumptionTrustMetadata, CheckedCorePackage, LowerabilityStatus, ObligationMetadata,
    ObligationStatus, RecordSigmaKind, RecordSigmaMetadata, StableSymbol, SymbolNamespace,
};
use ken_elaborator::compiler_driver::{
    compile_ken_package_sources, CompilerManifest, CompilerSource, CompilerTargetKind,
    TargetSelector,
};
use ken_elaborator::erasure::{
    emit_proof_erasure_boundary_witness_for_targets, erase_checked_core_package_for_target,
};
use ken_elaborator::ElabEnv;
use ken_interp::{
    ken_check_proof_erasure_boundary_witness, KenProofErasureBoundaryCheckStage,
    NC9_PROOF_ERASURE_BOUNDARY_CHECKER_SOURCE,
};
use ken_runtime::{
    proof_erasure_boundary_facts_from_program,
    run_ken_checked_proof_erasure_example_with_interpreter_observation,
    InterpreterOracleObservation, KenProofErasureBoundaryChecker, NativeArtifactIdentity,
    NativeDifferentialStage, NativeDifferentialVerdict, NativeFidelity, NativeSeedEnvironment,
    ProofErasureBoundaryWitness, ProofErasureBoundaryWitnessStage, ProofErasureBoundaryWitnessTier,
    RuntimeArtifactIdentity, RuntimeGroundValue, RuntimeObservation, RuntimeProgram,
};

fn fixture_package() -> CheckedCorePackage {
    representative_checked_core_fixtures()
        .expect("fixture emits")
        .into_iter()
        .next()
        .expect("representative fixture exists")
        .package
}

fn reemit(mut package: CheckedCorePackage) -> CheckedCorePackage {
    package.header.dependency_semantic_hashes =
        package.artifact.semantic.dependency_semantic_hashes.clone();
    emit_checked_core_package(package.header, package.artifact).expect("package re-emits")
}

fn proof_erasure_record_package() -> (CheckedCorePackage, StableSymbol, StableSymbol, StableSymbol)
{
    let mut package = fixture_package();
    let target = StableSymbol::declaration("fixture", &["Proof"], "ErasureRecord");
    let obligation = StableSymbol::obligation("erasure-record.obligation");
    let assumption = StableSymbol::assumption(&target, "trusted-helper");
    let non_target_record = StableSymbol::declaration("fixture", &["Proof"], "NonTargetRecord");
    let non_target_unsupported = StableSymbol::new(
        SymbolNamespace::Unsupported,
        ["fixture".to_string(), "NonTarget".to_string()],
    );

    package.artifact.semantic.symbols.extend([
        target.clone(),
        non_target_record.clone(),
        obligation.clone(),
        assumption.clone(),
        non_target_unsupported.clone(),
    ]);
    package
        .artifact
        .semantic
        .declarations
        .insert(target.clone(), b"checked-decl:erasure-record".to_vec());
    package.artifact.semantic.declarations.insert(
        non_target_record.clone(),
        b"checked-decl:non-target-record".to_vec(),
    );
    package.artifact.semantic.record_sigma_metadata.insert(
        target.clone(),
        RecordSigmaMetadata {
            kind: RecordSigmaKind::Record,
            fields: vec![
                ken_elaborator::checked_core::FieldMetadata {
                    name: "runtime_payload".to_string(),
                    ty: target.clone(),
                    runtime: ken_elaborator::checked_core::RuntimeFieldStatus::Runtime,
                },
                ken_elaborator::checked_core::FieldMetadata {
                    name: "law_payload".to_string(),
                    ty: target.clone(),
                    runtime: ken_elaborator::checked_core::RuntimeFieldStatus::ErasedLaw,
                },
                ken_elaborator::checked_core::FieldMetadata {
                    name: "proof_payload".to_string(),
                    ty: target.clone(),
                    runtime: ken_elaborator::checked_core::RuntimeFieldStatus::ErasedProof,
                },
            ],
            lowerability: LowerabilityStatus::Supported,
        },
    );
    package.artifact.semantic.record_sigma_metadata.insert(
        non_target_record.clone(),
        RecordSigmaMetadata {
            kind: RecordSigmaKind::Record,
            fields: vec![ken_elaborator::checked_core::FieldMetadata {
                name: "non_target_proof".to_string(),
                ty: non_target_record.clone(),
                runtime: ken_elaborator::checked_core::RuntimeFieldStatus::ErasedProof,
            }],
            lowerability: LowerabilityStatus::Supported,
        },
    );
    package
        .artifact
        .semantic
        .lowerability
        .insert(target.clone(), LowerabilityStatus::Supported);
    package
        .artifact
        .semantic
        .lowerability
        .insert(non_target_record.clone(), LowerabilityStatus::Supported);
    package.artifact.semantic.obligations.insert(
        obligation.clone(),
        b"obligation survives the proof-erasure boundary".to_vec(),
    );
    package.artifact.semantic.obligation_metadata.insert(
        obligation.clone(),
        ObligationMetadata {
            status: ObligationStatus::Unknown,
            origin: target.clone(),
            affects_runtime_meaning: true,
        },
    );
    package.artifact.semantic.assumptions.insert(
        assumption.clone(),
        b"assumption survives the proof-erasure boundary".to_vec(),
    );
    package.artifact.semantic.assumption_trust_metadata.insert(
        assumption.clone(),
        AssumptionTrustMetadata {
            kind: AssumptionTrustKind::PrimitiveAssumption,
            target: target.clone(),
            affects_runtime_meaning: true,
        },
    );
    package.artifact.semantic.trusted_base_delta.insert(
        assumption.clone(),
        b"trusted-base delta survives the proof-erasure boundary".to_vec(),
    );
    package.artifact.semantic.lowerability.insert(
        non_target_unsupported.clone(),
        LowerabilityStatus::Unsupported {
            reason: "non-target unsupported lane remains auditable".to_string(),
        },
    );
    package
        .artifact
        .semantic
        .unsupported
        .insert(non_target_unsupported, b"non-target unsupported".to_vec());

    (reemit(package), target, obligation, assumption)
}

/// `RT-DYNAMIC-ARM-SCALAR-MERGE` `D1b-role-c1` — NC9's native row runs on a
/// **real** driver-compiled carrier.
///
/// The hand-built program it replaced carried no checked role record and no
/// pre-source trusted-base roster, so once package-backed compilation began
/// failing closed on those it could not reach the native backend, and the row
/// measures a native trust report. This carrier comes from the same
/// `compile_ken_package_sources -> erase_checked_core_package_for_target` path a
/// real package takes.
///
/// ⛔ The NC9 witness and checker are **unchanged**: the proof-erasure boundary
/// facts are still recomputed from the program and checked by `ken-interp`'s own
/// checker, so the migration swaps the carrier without touching the property.
const NC9_CARRIER_PKG: &str = "nc9_boundary_carrier";
const NC9_CARRIER_SOURCE: &str = "const addTwoThree : Nat = Suc (Suc (Suc Zero))";

fn real_scalar_carrier() -> RuntimeProgram {
    let out = compile_ken_package_sources(
        &CompilerManifest::new(NC9_CARRIER_PKG, Vec::new()),
        vec![CompilerSource::new("src/main.ken", NC9_CARRIER_SOURCE)],
        TargetSelector::StableSymbol {
            package_identity: StableSymbol::new(
                SymbolNamespace::Module,
                vec![NC9_CARRIER_PKG.to_string()],
            ),
            symbol: StableSymbol::declaration(NC9_CARRIER_PKG, &[], "addTwoThree"),
            kind: CompilerTargetKind::Executable,
        },
    )
    .expect("the NC9 carrier source emits a checked-core package");
    let closure = out.closures.first().expect("selected target closure");
    let program =
        erase_checked_core_package_for_target(&out.package, closure.reachable_declarations.iter())
            .expect("the NC9 carrier package erases");
    assert!(
        program
            .examples
            .iter()
            .any(|example| example.name == "closed-scalar-primitive"),
        "the carrier must carry the closed scalar example the row runs"
    );
    program
}

#[test]
fn ken_checker_source_kernel_checks_without_trust_delta() {
    let mut env = ElabEnv::new().expect("base env");
    let before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();

    env.elaborate_file(NC9_PROOF_ERASURE_BOUNDARY_CHECKER_SOURCE)
        .expect("NC9 Ken checker must elaborate/kernel-check");

    let after: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(
        before, after,
        "NC9 checker must not add hidden holes/postulates"
    );
}

#[test]
fn ken_checker_accepts_bounded_proof_erasure_witness() {
    let (package, target, obligation, assumption) = proof_erasure_record_package();
    let program =
        erase_checked_core_package_for_target(&package, [&target]).expect("erasure succeeds");
    let witness = emit_proof_erasure_boundary_witness_for_targets(&package, [&target], &program)
        .expect("witness emits");

    let report = ken_check_proof_erasure_boundary_witness(&program, &witness)
        .expect("Ken/Rust checker agreement accepts");

    assert_eq!(
        report.tier,
        ProofErasureBoundaryWitnessTier::Nc9BoundedProofErasureBoundary
    );
    assert_eq!(
        report.checker,
        KenProofErasureBoundaryChecker::Nc9KenLaneVerdictCheckerV1
    );
    assert_eq!(report.artifact.package_identity, program.package_identity);
    assert!(report
        .facts
        .obligation_metadata
        .contains_key(&obligation.to_string()));
    assert!(report
        .facts
        .assumption_trust_metadata
        .contains_key(&assumption.to_string()));
    assert!(!report.helper_assumptions.is_empty());
    assert!(report
        .evidence_source
        .contains("ProofErasureBoundaryChecker.ken"));
}

#[test]
fn ken_checker_rejects_stale_identity_with_named_lane() {
    let (package, target, _, _) = proof_erasure_record_package();
    let program =
        erase_checked_core_package_for_target(&package, [&target]).expect("erasure succeeds");
    let mut witness =
        emit_proof_erasure_boundary_witness_for_targets(&package, [&target], &program)
            .expect("witness emits");
    witness.artifact.artifact_hash += 1;

    let err = ken_check_proof_erasure_boundary_witness(&program, &witness)
        .expect_err("stale identity must reject");

    assert_eq!(
        err.stage,
        KenProofErasureBoundaryCheckStage::WitnessIdentity
    );
    assert_eq!(err.lane, "artifact_identity");
}

#[test]
fn ken_checker_rejects_dropped_metadata_with_named_lane() {
    let (package, target, _, _) = proof_erasure_record_package();
    let program =
        erase_checked_core_package_for_target(&package, [&target]).expect("erasure succeeds");
    let mut witness =
        emit_proof_erasure_boundary_witness_for_targets(&package, [&target], &program)
            .expect("witness emits");
    witness.facts.obligation_metadata.clear();

    let err = ken_check_proof_erasure_boundary_witness(&program, &witness)
        .expect_err("dropped obligation metadata must reject");

    assert_eq!(
        err.stage,
        KenProofErasureBoundaryCheckStage::WitnessMismatch
    );
    assert_eq!(err.lane, "obligation_metadata");
}

#[test]
fn ken_checker_rejects_witness_program_mismatch_with_named_lane() {
    let (package, target, _, _) = proof_erasure_record_package();
    let mut program =
        erase_checked_core_package_for_target(&package, [&target]).expect("erasure succeeds");
    let witness = emit_proof_erasure_boundary_witness_for_targets(&package, [&target], &program)
        .expect("witness emits");
    program.erased_core.metadata.assumptions.clear();

    let err = ken_check_proof_erasure_boundary_witness(&program, &witness)
        .expect_err("witness/program mismatch must reject");

    assert_eq!(
        err.stage,
        KenProofErasureBoundaryCheckStage::WitnessMismatch
    );
    assert_eq!(err.lane, "assumptions");
}

#[test]
fn native_trust_report_records_nc9_separately_from_nc8_and_f1() {
    let program = real_scalar_carrier();
    let witness = ProofErasureBoundaryWitness {
        artifact: RuntimeArtifactIdentity::from_program(&program),
        facts: proof_erasure_boundary_facts_from_program(&program),
    };
    let nc9_report = ken_check_proof_erasure_boundary_witness(&program, &witness)
        .expect("simple witness accepts");
    // Selected by NAME, not by position: the real carrier emits several
    // examples and their order is not a contract.
    let example = program
        .examples
        .iter()
        .find(|example| example.name == "closed-scalar-primitive")
        .expect("the carrier carries the closed scalar example")
        .clone();
    let oracle = InterpreterOracleObservation {
        artifact: NativeArtifactIdentity {
            package_identity: program.package_identity.clone(),
            core_semantic_hash: program.core_semantic_hash,
            runtime_artifact_hash: program.artifact_hash,
        },
        observation: RuntimeObservation::Returned(RuntimeGroundValue::Int((5).into())),
        evidence_source: "test oracle for exact NC9 RuntimeProgram identity".to_string(),
    };

    let report = run_ken_checked_proof_erasure_example_with_interpreter_observation(
        &program,
        &example,
        &NativeSeedEnvironment::empty(),
        oracle,
        nc9_report,
    )
    .expect("NC9 report identity matches");

    assert_eq!(
        report.verdict,
        NativeDifferentialVerdict::F1InterpreterAgreement {
            stage: NativeDifferentialStage::InterpreterNativeCompare,
        }
    );
    let native = report.native.expect("native side ran");
    assert_eq!(
        native.trust.fidelity,
        NativeFidelity::F1InterpreterDifferentialAgreement
    );
    assert!(
        native.trust.artifact_validation.is_none(),
        "NC9 must not backfill the NC8 F2 artifact-validation slot"
    );
    let nc9 = native
        .trust
        .ken_checked_proof_erasure_boundary
        .expect("NC9 fact is report-visible");
    assert_eq!(
        nc9.tier,
        ProofErasureBoundaryWitnessTier::Nc9BoundedProofErasureBoundary
    );
    assert!(nc9.evidence_source.contains("ken-interp"));
}

#[test]
fn native_trust_report_attachment_recheck_names_concrete_mismatch_lane() {
    let (package, target, _, _) = proof_erasure_record_package();
    let program =
        erase_checked_core_package_for_target(&package, [&target]).expect("erasure succeeds");
    let witness = emit_proof_erasure_boundary_witness_for_targets(&package, [&target], &program)
        .expect("witness emits");
    let mut nc9_report = ken_check_proof_erasure_boundary_witness(&program, &witness)
        .expect("Ken/Rust checker agreement accepts");
    nc9_report.facts.assumptions.clear();

    let example = program.examples[0].clone();
    let oracle = InterpreterOracleObservation {
        artifact: NativeArtifactIdentity {
            package_identity: program.package_identity.clone(),
            core_semantic_hash: program.core_semantic_hash,
            runtime_artifact_hash: program.artifact_hash,
        },
        observation: example.observation.clone(),
        evidence_source: "test oracle for exact NC9 RuntimeProgram identity".to_string(),
    };

    let err = run_ken_checked_proof_erasure_example_with_interpreter_observation(
        &program,
        &example,
        &NativeSeedEnvironment::empty(),
        oracle,
        nc9_report,
    )
    .expect_err("attached NC9 report with stale assumptions lane must reject");

    assert_eq!(err.stage, ProofErasureBoundaryWitnessStage::WitnessMismatch);
    assert_eq!(err.lane, "assumptions");
}
