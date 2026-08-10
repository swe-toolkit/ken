//! Certificate, preflight, differential and outward-runner subject tests
//! (RT-SPLIT §10.1/§10.2).
//!
//! Moved from the residual facade in slice 6, alongside `artifact::api`.
//! Ruled test module: imports are permitted here (AC-8 class 2).

use super::*;

use std::collections::BTreeSet;

// RT-SPLIT slice 7, rule 8: `total_primitive` moved from facade file scope to
// its lawful facade-LCA home. Import-only edit in a ruled test module.
use crate::cranelift_backend::test_support::total_primitive;
use crate::{
    evaluate_runtime_ir_example, nc5_seed_examples, ErasedExecutableCore,
    RuntimeArtifactValidationStage, RuntimeArtifactValidationTier, RuntimeAssumptionTrustKind,
    RuntimeAssumptionTrustMetadata, RuntimeDeclaration, RuntimeEffectsForeignAuditMetadata,
    RuntimeFieldStatus, RuntimeGroundValue, RuntimeIrSeedEnvironment, RuntimeMatchCase,
    RuntimeMetadata, RuntimeObservation, RuntimePartiality, RuntimePrimitive,
    RuntimeSymbolMetadata, RuntimeTrap, RuntimeTrapCode, UnsupportedLowering,
};

fn seed_program_with_lowerability(status: Option<RuntimeLowerabilityStatus>) -> RuntimeProgram {
    let symbol = "decl:fixture::Main::main".to_string();
    let mut metadata = RuntimeMetadata::default();
    if let Some(status) = status.clone() {
        metadata.lowerability.insert(symbol.clone(), status);
    }
    RuntimeProgram {
        package_identity: "module:fixture::nc6".to_string(),
        core_semantic_hash: 1,
        artifact_hash: 2,
        erased_core: ErasedExecutableCore {
            symbols: BTreeSet::from([symbol.clone()]),
            metadata,
        },
        declarations: vec![RuntimeDeclaration {
            symbol,
            kind: RuntimeDeclarationKind::Record {
                fields: vec![crate::RuntimeField {
                    name: "value".to_string(),
                    status: RuntimeFieldStatus::Runtime,
                }],
            },
            metadata: RuntimeSymbolMetadata {
                lowerability: status,
                ..RuntimeSymbolMetadata::empty()
            },
        }],
        examples: nc5_seed_examples(),
    }
}
fn nc22_program_with_body(body: RuntimeExpr, observation: RuntimeObservation) -> RuntimeProgram {
    let symbol = "decl:fixture::Main::main".to_string();
    let mut metadata = RuntimeMetadata::default();
    metadata
        .lowerability
        .insert(symbol.clone(), RuntimeLowerabilityStatus::Supported);
    RuntimeProgram {
        package_identity: "module:fixture::nc22".to_string(),
        core_semantic_hash: 22,
        artifact_hash: 2200,
        erased_core: ErasedExecutableCore {
            symbols: BTreeSet::from([symbol.clone()]),
            metadata,
        },
        declarations: vec![RuntimeDeclaration {
            symbol: symbol.clone(),
            kind: RuntimeDeclarationKind::Transparent { body },
            metadata: RuntimeSymbolMetadata {
                lowerability: Some(RuntimeLowerabilityStatus::Supported),
                ..RuntimeSymbolMetadata::empty()
            },
        }],
        examples: vec![RuntimeExample {
            name: "main-entrypoint".to_string(),
            checked_core_shape: "compiler-produced declaration ref".to_string(),
            ir: RuntimeExpr::DeclarationRef { symbol },
            observation,
        }],
    }
}
#[test]
fn program_runner_preflights_metadata_before_backend_lowering() {
    let program = seed_program_with_lowerability(Some(RuntimeLowerabilityStatus::Supported));

    let reports = run_nc6_seed_examples(&program).expect("seed program runs");

    assert_eq!(reports.len(), 5);
    assert!(reports
        .iter()
        .all(|report| report.trust.fidelity == NativeFidelity::F1SeedObservationAgreement));
}
#[test]
#[ignore = "RT-FNUNIT-RESULT-TOKEN: fails with `native result token 265 is not in the result table`. RT-SEED-CALL-PORT D3 retired SeedClosureCall, which made this shape newly reachable on the FunctionizedUnits lane; the wall was already there and D3 is NOT the cause. Measured: flipping this fixture's callee from Closure to LexicalClosure -- an arm live since RT-DECL-CLOSURE-PORT and untouched by D2/D3 -- reproduces the identical error. RT-FNUNIT-RESULT-TOKEN owns un-skipping this row, which means running it green on the functionized lane, not tidying the skip"]
fn nc22_cranelift_agrees_with_runtime_ir_report_for_broad_starter_shapes() {
    let body = RuntimeExpr::Let {
        value: Box::new(total_primitive(
            "add_int",
            vec![
                RuntimeExpr::Value(RuntimeValue::Int((2).into())),
                RuntimeExpr::Value(RuntimeValue::Int((3).into())),
            ],
        )),
        body: Box::new(RuntimeExpr::Call {
            callee: Box::new(RuntimeExpr::Closure {
                captures: Vec::new(),
                params: vec!["x".to_string()],
                body: Box::new(RuntimeExpr::Match {
                    scrutinee: Box::new(RuntimeExpr::Construct {
                        constructor: "ctor:fixture::Box::Box".to_string(),
                        args: vec![RuntimeExpr::Var(0)],
                    }),
                    cases: vec![RuntimeMatchCase {
                        constructor: "ctor:fixture::Box::Box".to_string(),
                        binders: 1,
                        body: RuntimeExpr::Record {
                            fields: vec![
                                (
                                    "ok".to_string(),
                                    RuntimeExpr::If {
                                        scrutinee: Box::new(total_primitive(
                                            "eq_int",
                                            vec![
                                                RuntimeExpr::Var(0),
                                                RuntimeExpr::Value(RuntimeValue::Int((5).into())),
                                            ],
                                        )),
                                        then_expr: Box::new(RuntimeExpr::Value(
                                            RuntimeValue::Bool(true),
                                        )),
                                        else_expr: Box::new(RuntimeExpr::Value(
                                            RuntimeValue::Bool(false),
                                        )),
                                    },
                                ),
                                (
                                    "value".to_string(),
                                    total_primitive(
                                        "sub_int",
                                        vec![
                                            total_primitive(
                                                "mul_int",
                                                vec![
                                                    RuntimeExpr::Var(0),
                                                    RuntimeExpr::Value(RuntimeValue::Int(
                                                        (2).into(),
                                                    )),
                                                ],
                                            ),
                                            RuntimeExpr::Value(RuntimeValue::Int((3).into())),
                                        ],
                                    ),
                                ),
                            ],
                        },
                    }],
                    default: RuntimeTrap {
                        code: RuntimeTrapCode::PatternMatchFailure,
                        message: "unexpected constructor".to_string(),
                    },
                }),
            }),
            args: vec![RuntimeExpr::Var(0)],
        }),
    };
    let observation = RuntimeObservation::Returned(RuntimeGroundValue::Record {
        fields: vec![
            ("ok".to_string(), RuntimeGroundValue::Bool(true)),
            ("value".to_string(), RuntimeGroundValue::Int((7).into())),
        ],
    });
    let program = nc22_program_with_body(body, observation.clone());
    let run_report = evaluate_runtime_ir_example(
        &program,
        &program.examples[0],
        &RuntimeIrSeedEnvironment::empty(),
    )
    .expect("runtime-IR evaluator runs the compiler-produced artifact");

    let report = run_synthetic_runtime_ir_report_with_cranelift(
        &program,
        run_report,
        &NativeSeedEnvironment::empty(),
        &crate::native_process_authority::synthetic_test_legacy_authority(),
    );

    assert_eq!(
        report.verdict,
        NativeRuntimeIrComparisonVerdict::RuntimeIrNativeAgreement {
            stage: NativeDifferentialStage::RuntimeIrNativeCompare,
        }
    );
    let native = report.native.expect("native side ran");
    assert_eq!(native.observation, observation);
    assert_eq!(
        native.trust.fidelity,
        NativeFidelity::F1RuntimeIrEvaluatorAgreement
    );
    assert_eq!(
        native.trust.evidence.runtime_artifact_hash,
        Some(program.artifact_hash)
    );
}
#[test]
fn nc22_imported_dependency_lowers_as_stable_unsupported_native_lane() {
    let symbol = "decl:fixture::Main::main".to_string();
    let dependency = "dep:fixture".to_string();
    let imported = "decl:dep::value".to_string();
    let dependency_hash = "hash:dep".to_string();
    let mut program = nc22_program_with_body(
        RuntimeExpr::ImportedDeclarationRef {
            symbol: imported.clone(),
            dependency: dependency.clone(),
            dependency_semantic_hash: dependency_hash.clone(),
        },
        RuntimeObservation::Returned(RuntimeGroundValue::Int((9).into())),
    );
    program.declarations[0].symbol = symbol.clone();
    program.erased_core.symbols.insert(imported.clone());
    program
        .erased_core
        .metadata
        .lowerability
        .insert(imported.clone(), RuntimeLowerabilityStatus::Supported);
    program
        .erased_core
        .metadata
        .dependency_semantic_hashes
        .insert(dependency.clone(), dependency_hash.clone());
    let mut runtime_env = RuntimeIrSeedEnvironment::empty();
    runtime_env.insert_imported_declaration(
        imported,
        dependency,
        dependency_hash,
        RuntimeGroundValue::Int((9).into()),
    );
    let run_report = evaluate_runtime_ir_example(&program, &program.examples[0], &runtime_env)
        .expect("runtime-IR evaluator can use an exact imported seed binding");

    let report = run_synthetic_runtime_ir_report_with_cranelift(
        &program,
        run_report,
        &NativeSeedEnvironment::empty(),
        &crate::native_process_authority::synthetic_test_legacy_authority(),
    );

    assert!(matches!(
        report.verdict,
        NativeRuntimeIrComparisonVerdict::Unsupported {
            stage: NativeDifferentialStage::NativeLoweringOrExecution,
            construct: "ImportedDeclarationRef",
            ..
        }
    ));
    assert!(report.native.is_none());
}
#[test]
fn nc22_runtime_ir_report_identity_mismatch_rejects_before_native_lowering() {
    let program = nc22_program_with_body(
        RuntimeExpr::Value(RuntimeValue::Int((1).into())),
        RuntimeObservation::Returned(RuntimeGroundValue::Int((1).into())),
    );
    let mut run_report = evaluate_runtime_ir_example(
        &program,
        &program.examples[0],
        &RuntimeIrSeedEnvironment::empty(),
    )
    .expect("runtime-IR evaluator runs");
    run_report.evidence.runtime_artifact_hash = 0xdead_beef;

    let report = run_synthetic_runtime_ir_report_with_cranelift(
        &program,
        run_report,
        &NativeSeedEnvironment::empty(),
        &crate::native_process_authority::synthetic_test_legacy_authority(),
    );

    assert!(matches!(
        report.verdict,
        NativeRuntimeIrComparisonVerdict::Unsupported {
            stage: NativeDifferentialStage::BoundaryPreflight,
            construct: "RuntimeIrRunReport",
            ..
        }
    ));
    assert!(report.native.is_none());
}
#[test]
fn nc22_ambiguous_runtime_ir_report_target_rejects_before_native_lowering() {
    let mut program = nc22_program_with_body(
        RuntimeExpr::Value(RuntimeValue::Int((1).into())),
        RuntimeObservation::Returned(RuntimeGroundValue::Int((1).into())),
    );
    program.examples.push(program.examples[0].clone());
    let mut run_report = evaluate_runtime_ir_example(
        &nc22_program_with_body(
            RuntimeExpr::Value(RuntimeValue::Int((1).into())),
            RuntimeObservation::Returned(RuntimeGroundValue::Int((1).into())),
        ),
        &program.examples[0],
        &RuntimeIrSeedEnvironment::empty(),
    )
    .expect("runtime-IR evaluator runs");
    run_report.artifact = RuntimeArtifactIdentity::from_program(&program);
    run_report.observation.artifact = RuntimeArtifactIdentity::from_program(&program);
    run_report.evidence.package_identity = program.package_identity.clone();
    run_report.evidence.core_semantic_hash = program.core_semantic_hash;
    run_report.evidence.runtime_artifact_hash = program.artifact_hash;

    let report = run_synthetic_runtime_ir_report_with_cranelift(
        &program,
        run_report,
        &NativeSeedEnvironment::empty(),
        &crate::native_process_authority::synthetic_test_legacy_authority(),
    );

    assert!(matches!(
        report.verdict,
        NativeRuntimeIrComparisonVerdict::Unsupported {
            stage: NativeDifferentialStage::BoundaryPreflight,
            construct: "RuntimeIrRunReport",
            ..
        }
    ));
    assert!(report.native.is_none());
}
#[test]
fn nc8_valid_certificate_records_f2_validation_separate_from_f1() {
    let example = nc5_seed_examples()
        .into_iter()
        .find(|example| example.name == "closed-scalar-primitive")
        .expect("seed exists");
    let mut program = seed_program_with_lowerability(Some(RuntimeLowerabilityStatus::Supported));
    program.examples = vec![example.clone()];
    let certificate = RuntimeArtifactCertificate::supported_runtime_artifact_for(&program);
    let oracle = InterpreterOracleObservation {
        artifact: NativeArtifactIdentity::from_program(&program),
        observation: example.observation.clone(),
        evidence_source: "test oracle over matching RuntimeProgram identity".to_string(),
    };

    let report = run_synthetic_validated_example_with_interpreter_observation(
        &program,
        &example,
        &NativeSeedEnvironment::empty(),
        oracle,
        &certificate,
        &crate::native_process_authority::synthetic_test_legacy_authority(),
    )
    .expect("certificate validates");

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
    let validation = native
        .trust
        .artifact_validation
        .expect("validated artifact fact is report-visible");
    assert_eq!(
        validation.tier,
        RuntimeArtifactValidationTier::F2BoundedRuntimeArtifactValidation
    );
    assert_eq!(
        validation.artifact.package_identity,
        program.package_identity
    );
    assert_eq!(
        validation.artifact.core_semantic_hash,
        program.core_semantic_hash
    );
    assert_eq!(validation.artifact.artifact_hash, program.artifact_hash);
    assert!(validation
        .evidence_source
        .contains("recomputed supported-subset facts"));
}
#[test]
fn nc8_certificate_wrong_identity_rejects_before_native_run() {
    let program = seed_program_with_lowerability(Some(RuntimeLowerabilityStatus::Supported));
    let mut certificate = RuntimeArtifactCertificate::supported_runtime_artifact_for(&program);
    certificate.artifact_hash = Some(0xdead_beef);

    let err = validate_supported_runtime_artifact_certificate(&program, &certificate)
        .expect_err("wrong artifact identity rejects");

    assert_eq!(err.stage, RuntimeArtifactValidationStage::ArtifactIdentity);
    assert_eq!(err.fact, "runtime_artifact_identity");
}
#[test]
fn nc8_certificate_missing_fields_rejects_loudly() {
    let program = seed_program_with_lowerability(Some(RuntimeLowerabilityStatus::Supported));
    let mut certificate = RuntimeArtifactCertificate::supported_runtime_artifact_for(&program);
    certificate.core_semantic_hash = None;

    let err = validate_supported_runtime_artifact_certificate(&program, &certificate)
        .expect_err("missing identity field rejects");

    assert_eq!(
        err.stage,
        RuntimeArtifactValidationStage::MalformedCertificate
    );
    assert_eq!(err.fact, "core_semantic_hash");

    let mut certificate = RuntimeArtifactCertificate::supported_runtime_artifact_for(&program);
    certificate.claim.as_mut().expect("claim exists").facts = None;
    let err = validate_supported_runtime_artifact_certificate(&program, &certificate)
        .expect_err("missing facts reject");

    assert_eq!(
        err.stage,
        RuntimeArtifactValidationStage::MalformedCertificate
    );
    assert_eq!(err.fact, "facts");
}
#[test]
fn nc8_certificate_contradictory_claim_rejects() {
    let mut program = seed_program_with_lowerability(Some(RuntimeLowerabilityStatus::Supported));
    program.examples = vec![nc5_seed_examples()
        .into_iter()
        .find(|example| example.name == "closed-scalar-primitive")
        .expect("seed exists")];
    let mut certificate = RuntimeArtifactCertificate::supported_runtime_artifact_for(&program);
    certificate
        .claim
        .as_mut()
        .expect("claim exists")
        .facts
        .as_mut()
        .expect("facts exist")
        .declaration_count = Some(program.declarations.len() + 1);

    let err = validate_supported_runtime_artifact_certificate(&program, &certificate)
        .expect_err("contradictory count rejects");

    assert_eq!(err.stage, RuntimeArtifactValidationStage::ClaimMismatch);
    assert_eq!(err.fact, "declaration_count");
}
#[test]
fn nc8_certificate_false_supported_claim_rejects_by_recomputation() {
    let mut program =
        seed_program_with_lowerability(Some(RuntimeLowerabilityStatus::Unsupported {
            reason: "not lowerable".to_string(),
        }));
    let symbol = program.declarations[0].symbol.clone();
    program.declarations[0].metadata.lowerability = Some(RuntimeLowerabilityStatus::Unsupported {
        reason: "not lowerable".to_string(),
    });
    program
        .erased_core
        .metadata
        .unsupported
        .insert(symbol, b"hidden blocker".to_vec());
    let certificate = RuntimeArtifactCertificate::supported_runtime_artifact_for(&program);

    let err = validate_supported_runtime_artifact_certificate(&program, &certificate)
        .expect_err("false supported-subset claim rejects");

    assert_eq!(err.stage, RuntimeArtifactValidationStage::ClaimRecompute);
    assert!(matches!(
        err.fact,
        "no_reachable_unsupported_entries" | "all_reachable_lowerability_supported"
    ));
}
#[test]
fn nc8_certificate_rejects_unknown_runtime_value_by_recomputation() {
    let mut program = seed_program_with_lowerability(Some(RuntimeLowerabilityStatus::Supported));
    program.examples = vec![RuntimeExample {
        name: "unknown-runtime-value".to_string(),
        checked_core_shape: "diagnostic label only".to_string(),
        ir: RuntimeExpr::Value(RuntimeValue::Unknown),
        observation: RuntimeObservation::Returned(RuntimeGroundValue::Int((0).into())),
    }];
    let certificate = RuntimeArtifactCertificate::supported_runtime_artifact_for(&program);

    let err = validate_supported_runtime_artifact_certificate(&program, &certificate)
        .expect_err("unknown runtime values are outside the supported subset");

    assert_eq!(err.stage, RuntimeArtifactValidationStage::ClaimRecompute);
    assert_eq!(err.fact, "all_runtime_values_supported");
    assert!(err.reason.contains("unknown runtime data"));
}
#[test]
fn nc8_certificate_rejects_let_expression_in_validated_example() {
    let mut program = seed_program_with_lowerability(Some(RuntimeLowerabilityStatus::Supported));
    program.examples = vec![RuntimeExample {
        name: "let-outside-supported-subset".to_string(),
        checked_core_shape: "diagnostic label only".to_string(),
        ir: RuntimeExpr::Let {
            value: Box::new(RuntimeExpr::Value(RuntimeValue::Int((1).into()))),
            body: Box::new(RuntimeExpr::Var(0)),
        },
        observation: RuntimeObservation::Returned(RuntimeGroundValue::Int((1).into())),
    }];
    let certificate = RuntimeArtifactCertificate::supported_runtime_artifact_for(&program);

    let err = validate_supported_runtime_artifact_certificate(&program, &certificate)
        .expect_err("let expressions are outside the NC6 supported subset");

    assert_eq!(err.stage, RuntimeArtifactValidationStage::ClaimRecompute);
    assert_eq!(err.fact, "all_runtime_expressions_supported");
    assert!(err.reason.contains("Let"));
}
#[test]
fn nc8_certificate_rejects_if_expression_in_reachable_transparent_declaration() {
    let mut program = seed_program_with_lowerability(Some(RuntimeLowerabilityStatus::Supported));
    program.declarations[0].kind = RuntimeDeclarationKind::Transparent {
        body: RuntimeExpr::If {
            scrutinee: Box::new(RuntimeExpr::Value(RuntimeValue::Bool(true))),
            then_expr: Box::new(RuntimeExpr::Value(RuntimeValue::Int((1).into()))),
            else_expr: Box::new(RuntimeExpr::Value(RuntimeValue::Int((0).into()))),
        },
    };
    let certificate = RuntimeArtifactCertificate::supported_runtime_artifact_for(&program);

    let err = validate_supported_runtime_artifact_certificate(&program, &certificate)
        .expect_err("if expressions are outside the NC6 supported subset");

    assert_eq!(err.stage, RuntimeArtifactValidationStage::ClaimRecompute);
    assert_eq!(err.fact, "all_runtime_expressions_supported");
    assert!(err.reason.contains("If"));
}
#[test]
fn nc8_certificate_rejects_unsupported_total_primitive_in_validated_example() {
    let mut program = seed_program_with_lowerability(Some(RuntimeLowerabilityStatus::Supported));
    program.examples = vec![RuntimeExample {
        name: "unsupported-total-primitive".to_string(),
        checked_core_shape: "diagnostic label only".to_string(),
        ir: RuntimeExpr::PrimitiveCall {
            primitive: RuntimePrimitive {
                symbol: "sub_int".to_string(),
                partiality: RuntimePartiality::Total,
            },
            args: vec![
                RuntimeExpr::Value(RuntimeValue::Int((2).into())),
                RuntimeExpr::Value(RuntimeValue::Int((1).into())),
            ],
        },
        observation: RuntimeObservation::Returned(RuntimeGroundValue::Int((1).into())),
    }];
    let certificate = RuntimeArtifactCertificate::supported_runtime_artifact_for(&program);

    let err = validate_supported_runtime_artifact_certificate(&program, &certificate)
        .expect_err("unsupported total primitives are outside the NC6 supported subset");

    assert_eq!(err.stage, RuntimeArtifactValidationStage::ClaimRecompute);
    assert_eq!(err.fact, "all_runtime_primitives_supported");
    assert!(err.reason.contains("sub_int"));
}
#[test]
fn nc8_certificate_rejects_add_int_wrong_arity_in_reachable_transparent_declaration() {
    let mut program = seed_program_with_lowerability(Some(RuntimeLowerabilityStatus::Supported));
    program.declarations[0].kind = RuntimeDeclarationKind::Transparent {
        body: RuntimeExpr::PrimitiveCall {
            primitive: RuntimePrimitive {
                symbol: "add_int".to_string(),
                partiality: RuntimePartiality::Total,
            },
            args: vec![RuntimeExpr::Value(RuntimeValue::Int((1).into()))],
        },
    };
    let certificate = RuntimeArtifactCertificate::supported_runtime_artifact_for(&program);

    let err = validate_supported_runtime_artifact_certificate(&program, &certificate)
        .expect_err("add_int arity mismatch is outside the NC6 supported subset");

    assert_eq!(err.stage, RuntimeArtifactValidationStage::ClaimRecompute);
    assert_eq!(err.fact, "all_runtime_primitives_supported");
    assert!(err.reason.contains("arity 1"));
}
#[test]
fn nc8_certificate_rejects_add_int_non_literal_int_operand_shape() {
    let mut program = seed_program_with_lowerability(Some(RuntimeLowerabilityStatus::Supported));
    program.examples = vec![RuntimeExample {
        name: "add-int-non-int-operand".to_string(),
        checked_core_shape: "diagnostic label only".to_string(),
        ir: RuntimeExpr::PrimitiveCall {
            primitive: RuntimePrimitive {
                symbol: "add_int".to_string(),
                partiality: RuntimePartiality::Total,
            },
            args: vec![
                RuntimeExpr::Value(RuntimeValue::Bool(true)),
                RuntimeExpr::Value(RuntimeValue::Int((1).into())),
            ],
        },
        observation: RuntimeObservation::Returned(RuntimeGroundValue::Int((2).into())),
    }];
    let certificate = RuntimeArtifactCertificate::supported_runtime_artifact_for(&program);

    let err = validate_supported_runtime_artifact_certificate(&program, &certificate)
        .expect_err("add_int non-literal-Int operands are outside the NC8 subset");

    assert_eq!(err.stage, RuntimeArtifactValidationStage::ClaimRecompute);
    assert_eq!(err.fact, "all_runtime_primitives_supported");
    assert!(err.reason.contains("non-literal-Int operand"));
}
#[test]
fn nc8_certificate_rejects_add_int_var_bound_to_bool_payload() {
    let mut program = seed_program_with_lowerability(Some(RuntimeLowerabilityStatus::Supported));
    program.examples = vec![RuntimeExample {
        name: "add-int-var-bound-to-bool".to_string(),
        checked_core_shape: "diagnostic label only".to_string(),
        ir: RuntimeExpr::Match {
            scrutinee: Box::new(RuntimeExpr::Construct {
                constructor: "ctor:fixture::BoolBox::Box".to_string(),
                args: vec![RuntimeExpr::Value(RuntimeValue::Bool(true))],
            }),
            cases: vec![RuntimeMatchCase {
                constructor: "ctor:fixture::BoolBox::Box".to_string(),
                binders: 1,
                body: RuntimeExpr::PrimitiveCall {
                    primitive: RuntimePrimitive {
                        symbol: "add_int".to_string(),
                        partiality: RuntimePartiality::Total,
                    },
                    args: vec![
                        RuntimeExpr::Var(0),
                        RuntimeExpr::Value(RuntimeValue::Int((1).into())),
                    ],
                },
            }],
            default: RuntimeTrap {
                code: RuntimeTrapCode::PatternMatchFailure,
                message: "unused default".to_string(),
            },
        },
        observation: RuntimeObservation::Returned(RuntimeGroundValue::Int((2).into())),
    }];
    let certificate = RuntimeArtifactCertificate::supported_runtime_artifact_for(&program);

    let err = validate_supported_runtime_artifact_certificate(&program, &certificate)
        .expect_err("add_int variable operands are outside the first NC8 validator");

    assert_eq!(err.stage, RuntimeArtifactValidationStage::ClaimRecompute);
    assert_eq!(err.fact, "all_runtime_expressions_supported");
    assert!(err.reason.contains("Match"));
}
#[test]
fn nc8_certificate_rejects_top_level_var_example() {
    let mut program = seed_program_with_lowerability(Some(RuntimeLowerabilityStatus::Supported));
    program.examples = vec![RuntimeExample {
        name: "top-level-var".to_string(),
        checked_core_shape: "diagnostic label only".to_string(),
        ir: RuntimeExpr::Var(0),
        observation: RuntimeObservation::Returned(RuntimeGroundValue::Int((0).into())),
    }];
    let certificate = RuntimeArtifactCertificate::supported_runtime_artifact_for(&program);

    let err = validate_supported_runtime_artifact_certificate(&program, &certificate)
        .expect_err("unbound var is outside the first NC8 validator");

    assert_eq!(err.stage, RuntimeArtifactValidationStage::ClaimRecompute);
    assert_eq!(err.fact, "all_runtime_expressions_supported");
    assert!(err.reason.contains("Var"));
}
#[test]
fn nc8_certificate_rejects_project_from_non_record_example() {
    let mut program = seed_program_with_lowerability(Some(RuntimeLowerabilityStatus::Supported));
    program.examples = vec![RuntimeExample {
        name: "project-from-int".to_string(),
        checked_core_shape: "diagnostic label only".to_string(),
        ir: RuntimeExpr::Project {
            record: Box::new(RuntimeExpr::Value(RuntimeValue::Int((1).into()))),
            field: "x".to_string(),
        },
        observation: RuntimeObservation::Returned(RuntimeGroundValue::Int((1).into())),
    }];
    let certificate = RuntimeArtifactCertificate::supported_runtime_artifact_for(&program);

    let err = validate_supported_runtime_artifact_certificate(&program, &certificate)
        .expect_err("project is outside the first NC8 validator");

    assert_eq!(err.stage, RuntimeArtifactValidationStage::ClaimRecompute);
    assert_eq!(err.fact, "all_runtime_expressions_supported");
    assert!(err.reason.contains("Project"));
}
#[test]
fn nc8_certificate_rejects_top_level_observable_closure() {
    let mut program = seed_program_with_lowerability(Some(RuntimeLowerabilityStatus::Supported));
    program.examples = vec![RuntimeExample {
        name: "top-level-closure".to_string(),
        checked_core_shape: "diagnostic label only".to_string(),
        ir: RuntimeExpr::Closure {
            captures: Vec::new(),
            params: Vec::new(),
            body: Box::new(RuntimeExpr::Value(RuntimeValue::Int((1).into()))),
        },
        observation: RuntimeObservation::Returned(RuntimeGroundValue::Int((1).into())),
    }];
    let certificate = RuntimeArtifactCertificate::supported_runtime_artifact_for(&program);

    let err = validate_supported_runtime_artifact_certificate(&program, &certificate)
        .expect_err("closure is outside the first NC8 validator");

    assert_eq!(err.stage, RuntimeArtifactValidationStage::ClaimRecompute);
    assert_eq!(err.fact, "all_runtime_expressions_supported");
    assert!(err.reason.contains("Closure"));
}
#[test]
fn nc8_certificate_rejects_var_in_reachable_transparent_declaration() {
    let mut program = seed_program_with_lowerability(Some(RuntimeLowerabilityStatus::Supported));
    program.declarations[0].kind = RuntimeDeclarationKind::Transparent {
        body: RuntimeExpr::Var(0),
    };
    let certificate = RuntimeArtifactCertificate::supported_runtime_artifact_for(&program);

    let err = validate_supported_runtime_artifact_certificate(&program, &certificate)
        .expect_err("transparent declaration var is outside the first NC8 validator");

    assert_eq!(err.stage, RuntimeArtifactValidationStage::ClaimRecompute);
    assert_eq!(err.fact, "all_runtime_expressions_supported");
    assert!(err.reason.contains("Var"));
}
#[test]
fn missing_lowerability_metadata_rejects_before_backend_lowering() {
    let program = seed_program_with_lowerability(None);

    let err = run_nc6_seed_examples(&program).expect_err("missing metadata rejects");

    assert!(matches!(
        err,
        CraneliftBackendError::Unsupported(UnsupportedLowering {
            construct: "RuntimeProgram",
            ..
        })
    ));
}
#[test]
fn reachable_unsupported_metadata_rejects_before_backend_lowering() {
    let mut program = seed_program_with_lowerability(Some(RuntimeLowerabilityStatus::Supported));
    let symbol = program.declarations[0].symbol.clone();
    program
        .erased_core
        .metadata
        .unsupported
        .insert(symbol, b"unsupported target".to_vec());

    let err = run_nc6_seed_examples(&program).expect_err("unsupported metadata rejects");

    assert!(matches!(
        err,
        CraneliftBackendError::Unsupported(UnsupportedLowering {
            construct: "RuntimeProgram",
            ..
        })
    ));
}
#[test]
fn reachable_declaration_effect_metadata_rejects_before_backend_lowering() {
    for lane in [
        "effects",
        "capabilities",
        "runtime_checks",
        "assumptions",
        "assumption_trust_metadata",
        "trusted_base_delta",
    ] {
        let mut program =
            seed_program_with_lowerability(Some(RuntimeLowerabilityStatus::Supported));
        let target = program.declarations[0].symbol.clone();
        match lane {
            "effects" => {
                program.declarations[0]
                    .metadata
                    .effects
                    .insert("Console".to_string());
            }
            "capabilities" => {
                program.declarations[0]
                    .metadata
                    .capabilities
                    .insert("cap:Console".to_string());
            }
            "runtime_checks" => {
                program.declarations[0]
                    .metadata
                    .runtime_checks
                    .insert("check:Console".to_string());
            }
            "assumptions" => {
                program.declarations[0]
                    .metadata
                    .assumptions
                    .insert("assume:Console".to_string());
            }
            "assumption_trust_metadata" => {
                program.declarations[0]
                    .metadata
                    .assumption_trust_metadata
                    .insert(
                        "assume:Console".to_string(),
                        RuntimeAssumptionTrustMetadata {
                            kind: RuntimeAssumptionTrustKind::Declassify,
                            target,
                            affects_runtime_meaning: true,
                        },
                    );
            }
            "trusted_base_delta" => {
                program.declarations[0]
                    .metadata
                    .trusted_base_delta
                    .insert("assume:Console".to_string());
            }
            _ => unreachable!("test lanes are exhaustive"),
        }

        let err = match run_nc6_seed_examples(&program) {
            Ok(_) => panic!("expected {lane} metadata to reject"),
            Err(err) => err,
        };

        assert!(matches!(
            err,
            CraneliftBackendError::Unsupported(UnsupportedLowering {
                construct: "RuntimeProgram",
                ..
            })
        ));
    }
}
#[test]
fn reachable_package_effect_metadata_rejects_before_backend_lowering() {
    for lane in ["effects", "capabilities", "runtime_checks"] {
        let mut program =
            seed_program_with_lowerability(Some(RuntimeLowerabilityStatus::Supported));
        match lane {
            "effects" => {
                program
                    .erased_core
                    .metadata
                    .effects
                    .insert("Console".to_string());
            }
            "capabilities" => {
                program
                    .erased_core
                    .metadata
                    .capabilities
                    .insert("cap:Console".to_string());
            }
            "runtime_checks" => {
                program
                    .erased_core
                    .metadata
                    .runtime_checks
                    .insert("check:Console".to_string());
            }
            _ => unreachable!("test lanes are exhaustive"),
        }

        let err = match run_nc6_seed_examples(&program) {
            Ok(_) => panic!("expected package {lane} metadata to reject"),
            Err(err) => err,
        };

        assert!(matches!(
            err,
            CraneliftBackendError::Unsupported(UnsupportedLowering {
                construct: "RuntimeProgram",
                ..
            })
        ));
    }
}
#[test]
fn reachable_effectful_checked_core_metadata_rejects_before_backend_lowering() {
    let mut program = seed_program_with_lowerability(Some(RuntimeLowerabilityStatus::Supported));
    let symbol = program.declarations[0].symbol.clone();
    program
        .erased_core
        .metadata
        .checked_core
        .effects_foreign_metadata
        .insert(
            symbol,
            RuntimeEffectsForeignAuditMetadata {
                declared_effects: BTreeSet::from(["Console".to_string()]),
                capabilities: BTreeSet::from(["cap:Console".to_string()]),
                foreign_symbol: None,
                boundary: RuntimeEffectBoundary::Effectful,
                runtime_checks: BTreeSet::from(["check:Console".to_string()]),
                lowerability: RuntimeLowerabilityStatus::Supported,
            },
        );

    let err =
        run_nc6_seed_examples(&program).expect_err("effectful checked-core metadata must reject");

    assert!(matches!(
        err,
        CraneliftBackendError::Unsupported(UnsupportedLowering {
            construct: "RuntimeProgram",
            ..
        })
    ));
}
#[test]
fn reachable_foreign_checked_core_metadata_rejects_before_backend_lowering() {
    let mut program = seed_program_with_lowerability(Some(RuntimeLowerabilityStatus::Supported));
    let symbol = program.declarations[0].symbol.clone();
    program
        .erased_core
        .metadata
        .checked_core
        .effects_foreign_metadata
        .insert(
            symbol,
            RuntimeEffectsForeignAuditMetadata {
                declared_effects: BTreeSet::new(),
                capabilities: BTreeSet::new(),
                foreign_symbol: Some("host.fixture.foreign".to_string()),
                boundary: RuntimeEffectBoundary::Foreign,
                runtime_checks: BTreeSet::new(),
                lowerability: RuntimeLowerabilityStatus::Supported,
            },
        );

    let err =
        run_nc6_seed_examples(&program).expect_err("foreign checked-core metadata must reject");

    assert!(matches!(
        err,
        CraneliftBackendError::Unsupported(UnsupportedLowering {
            construct: "RuntimeProgram",
            ..
        })
    ));
}

// ── `RT-DYNAMIC-ARM-SCALAR-MERGE` `D1b-role-c1` controls ─────────────────────
//
// The boundary this node lands is **structural in the call graph**: a
// package-backed compile resolves its authority through the fail-closed lane,
// and a synthetic hand-built program reaches the same lowering only by naming
// its authority at the call. These controls check the two halves that a test
// profile can see. The other two halves — that the synthetic entrypoint does
// not exist in a production build, and that omitting its authority argument
// fails rather than defaulting — are **compilation** facts, checked by the
// compilation-boundary controls in `scripts/` and recorded in the handback,
// because a `cargo test` run is structurally blind to both.

/// A body that constructs a Peano `Nat` out of the legacy spellings, so the
/// lowering's constructor dispatch actually consults `nat_zero` / `nat_suc`.
///
/// The control below turns on that consultation: an authority naming a
/// different package must change what is emitted.
fn nat_construct_body() -> RuntimeExpr {
    RuntimeExpr::Construct {
        constructor: "ctor:prelude::Nat::Suc".to_string(),
        args: vec![RuntimeExpr::Construct {
            constructor: "ctor:prelude::Nat::Zero".to_string(),
            args: Vec::new(),
        }],
    }
}

/// Control (a): a package-backed program carrying no role record is refused
/// **before the inner lowering is entered**, not merely refused.
///
/// The refusal itself is the weak half — an error can be produced anywhere.
/// The discriminating half is the sentinel: `arm_lowering_entry_sentinel`
/// survives the call only if `compile_expr_into_module_with_root_projection`
/// was never reached, and planning is invoked from inside that function.
#[test]
fn a_package_backed_program_without_a_role_record_refuses_before_lowering() {
    let observation = RuntimeObservation::Returned(RuntimeGroundValue::Int((5).into()));
    let body = total_primitive(
        "add_int",
        vec![
            RuntimeExpr::Value(RuntimeValue::Int((2).into())),
            RuntimeExpr::Value(RuntimeValue::Int((3).into())),
        ],
    );
    let program = nc22_program_with_body(body, observation);
    let run_report = evaluate_runtime_ir_example(
        &program,
        &program.examples[0],
        &RuntimeIrSeedEnvironment::empty(),
    )
    .expect("the runtime-IR evaluator runs the fixture");

    // This program carries no `checked_core.runtime_symbols` record, which is
    // exactly the state a package-backed compile must refuse.
    assert!(
        program
            .erased_core
            .metadata
            .checked_core
            .runtime_symbols
            .is_none(),
        "the fixture must carry NO role record, or this control is about something else"
    );

    // ── NEGATIVE HALF: the production entrypoint refuses before lowering ─────
    crate::cranelift_backend::lowering::core::arm_lowering_entry_sentinel();
    let refused = run_runtime_ir_report_with_cranelift(
        &program,
        run_report.clone(),
        &NativeSeedEnvironment::empty(),
    );
    assert!(
        matches!(
            refused.verdict,
            NativeRuntimeIrComparisonVerdict::Unsupported {
                construct: "checked-role-authority",
                ..
            }
        ),
        "a record-less package-backed program must be refused by the authority lane, got {:?}",
        refused.verdict
    );
    assert!(
        !crate::cranelift_backend::lowering::core::lowering_was_entered(),
        "the refusal happened only AFTER the inner lowering was entered, so it did not precede \
         planning -- the gate is in the wrong place"
    );
    assert!(refused.native.is_none(), "no native side may have run");

    // ── POSITIVE CONTROL ON THE SENTINEL ────────────────────────────────────
    // Without this, `!lowering_was_entered()` above would also hold if the
    // sentinel simply never flipped -- for instance if the epoch reset moved,
    // or if the fixture failed for an unrelated reason before any compile. The
    // same program, same report, differing only in HOW the authority is
    // supplied, must reach lowering.
    crate::cranelift_backend::lowering::core::arm_lowering_entry_sentinel();
    let ran = run_synthetic_runtime_ir_report_with_cranelift(
        &program,
        run_report,
        &NativeSeedEnvironment::empty(),
        &crate::native_process_authority::synthetic_test_legacy_authority(),
    );
    assert_eq!(
        ran.verdict,
        NativeRuntimeIrComparisonVerdict::RuntimeIrNativeAgreement {
            stage: NativeDifferentialStage::RuntimeIrNativeCompare,
        },
        "the synthetic entrypoint must compile the same program once authority is named"
    );
    assert!(
        crate::cranelift_backend::lowering::core::lowering_was_entered(),
        "the sentinel never flipped even on a successful compile, so the negative half above \
         asserted nothing"
    );
}

/// Control (b): the synthetic entrypoint **consumes** the authority it is
/// handed, rather than accepting it and lowering against something else.
///
/// This is the control the previous attempt at this node lacked. Threading an
/// authority argument through five fixture builders left all 38 tests red and
/// produced five `unused variable` warnings, because the value was accepted and
/// never read. A required parameter proves a caller *supplied* something; only
/// a behavioural difference proves the callee *used* it.
#[test]
fn the_synthetic_entrypoint_consumes_the_authority_it_is_given() {
    let observation = RuntimeObservation::Returned(RuntimeGroundValue::Int((0).into()));
    let program = nc22_program_with_body(nat_construct_body(), observation);
    let run_report = evaluate_runtime_ir_example(
        &program,
        &program.examples[0],
        &RuntimeIrSeedEnvironment::empty(),
    )
    .expect("the runtime-IR evaluator runs the Nat fixture");

    let legacy = crate::native_process_authority::synthetic_test_legacy_authority();
    // Same authority in every role EXCEPT the Peano pair, which now names a
    // different package. Nothing else differs between the two emissions.
    let mut foreign = legacy.clone();
    foreign.nat_zero = "ctor:other_package::Nat::Zero".to_string();
    foreign.nat_suc = "ctor:other_package::Nat::Suc".to_string();
    assert_ne!(
        legacy.nat_zero, foreign.nat_zero,
        "the two authorities must actually differ, or this control compares a thing with itself"
    );

    let with_legacy = emit_synthetic_runtime_ir_object_with_cranelift(
        &program,
        &run_report,
        &NativeSeedEnvironment::empty(),
        "ken_d1b_role_c1_authority_probe",
        &legacy,
    );
    let with_foreign = emit_synthetic_runtime_ir_object_with_cranelift(
        &program,
        &run_report,
        &NativeSeedEnvironment::empty(),
        "ken_d1b_role_c1_authority_probe",
        &foreign,
    );

    // POSITIVE CONTROL: the fixture must genuinely compile under the authority
    // it was written for, or "the two differ" would hold for the boring reason
    // that neither ever worked.
    let legacy_artifact = with_legacy
        .as_ref()
        .expect("the Nat fixture compiles under the authority whose spellings it uses");

    let differs = match &with_foreign {
        Ok(foreign_artifact) => foreign_artifact.object_hash != legacy_artifact.object_hash,
        // A refusal is also consumption: the authority reached the lowering and
        // changed the outcome.
        Err(_) => true,
    };
    assert!(
        differs,
        "emitting the SAME program under two DIFFERENT authorities produced byte-identical \
         output, so the authority argument was ignored -- exactly the failure the previous \
         builder-parameter threading had"
    );
}

/// Control (c): the admitted trust reaches BOTH emitted reports, and the same
/// program with an empty admitted set does not carry it.
///
/// ⛔ MEASURED: an admitted-trust set supplied at the compilation seam appears
/// in `CraneliftRunReport.trust.assumptions` and in
/// `CraneliftObjectArtifact.assumptions`, and is absent when the set is empty.
/// CLAIMED: the assumptions a package was admitted on are stated by the
/// artifacts that rest on them. THE GAP: this measures the seam, not a
/// production program -- see the vacuity note below, which is why it must.
///
/// **Why the set is supplied rather than obtained from an admitted program.**
/// `reject_program_blockers` refuses any package whose `assumptions` are
/// non-empty, and the admitted set *is* those assumption keys. So on every path
/// that produces a report today the set is necessarily empty and the union is
/// vacuous. A control built on a real admitted package would therefore assert
/// `∅ ⊆ ∅` and pass no matter what the seam did. The A/B below is the only
/// shape that can fail for the intended reason.
///
/// PROMISE CLASS: durable invariant on the seam (a set relation, no literal
/// pinned), paired with a transition sentinel recorded in the elaborator
/// provenance suite that measures the refusal making production paths empty.
/// It is retired -- becomes non-vacuous end to end -- when the native supported
/// subset admits packages carrying trusted-base assumptions.
#[test]
fn the_admitted_trust_reaches_the_run_report_and_the_object_artifact() {
    // The same body control (a) uses for its positive half, so this control is
    // known to reach lowering and produce a native side to inspect.
    let observation = RuntimeObservation::Returned(RuntimeGroundValue::Int((5).into()));
    let body = total_primitive(
        "add_int",
        vec![
            RuntimeExpr::Value(RuntimeValue::Int((2).into())),
            RuntimeExpr::Value(RuntimeValue::Int((3).into())),
        ],
    );
    let program = nc22_program_with_body(body, observation);
    let run_report = evaluate_runtime_ir_example(
        &program,
        &program.examples[0],
        &RuntimeIrSeedEnvironment::empty(),
    )
    .expect("the runtime-IR evaluator runs the fixture");

    let authority = crate::native_process_authority::synthetic_test_legacy_authority();
    let admitted: BTreeSet<crate::RuntimeSymbol> = BTreeSet::from([
        "assume:c1_fixture::alpha::trusted_base".to_string(),
        "assume:c1_fixture::beta::trusted_base".to_string(),
    ]);
    let none: BTreeSet<crate::RuntimeSymbol> = BTreeSet::new();

    // ── RUN REPORT ──────────────────────────────────────────────────────────
    let with_trust = run_synthetic_runtime_ir_report_with_admitted_trust(
        &program,
        run_report.clone(),
        &NativeSeedEnvironment::empty(),
        &authority,
        &admitted,
    );
    let native = with_trust
        .native
        .as_ref()
        .expect("the fixture compiles, so a native side exists to carry the assumptions");
    let carried = &native.trust.assumptions;
    assert!(
        admitted.iter().all(|entry| carried.contains(entry)),
        "the admitted trust did not reach the run report: admitted={admitted:?} carried={carried:?}"
    );

    // POSITIVE CONTROL ON THE DISCRIMINATOR: the same program with an EMPTY
    // admitted set must not carry these identities. Without this, the assertion
    // above would also pass if the lowering happened to mint them itself.
    let without_trust = run_synthetic_runtime_ir_report_with_admitted_trust(
        &program,
        run_report.clone(),
        &NativeSeedEnvironment::empty(),
        &authority,
        &none,
    );
    let bare = &without_trust
        .native
        .as_ref()
        .expect("the same fixture compiles with an empty admitted set")
        .trust
        .assumptions;
    assert!(
        admitted.iter().all(|entry| !bare.contains(entry)),
        "the run report carried the admitted identities even though NONE were admitted, so the \
         positive assertion above did not measure propagation: {bare:?}"
    );
    // And the two differ in EXACTLY the admitted set: the lowering's own
    // assumptions are untouched by the propagation.
    assert_eq!(
        carried.difference(bare).cloned().collect::<BTreeSet<_>>(),
        admitted,
        "propagation changed the report's assumptions by something other than the admitted set"
    );

    // ── OBJECT ARTIFACT ─────────────────────────────────────────────────────
    let object_with = emit_synthetic_runtime_ir_object_with_admitted_trust(
        &program,
        &run_report,
        &NativeSeedEnvironment::empty(),
        "ken_d1b_role_c1_propagation_probe",
        &authority,
        &admitted,
    )
    .expect("the fixture emits an object under its own authority");
    let object_without = emit_synthetic_runtime_ir_object_with_admitted_trust(
        &program,
        &run_report,
        &NativeSeedEnvironment::empty(),
        "ken_d1b_role_c1_propagation_probe",
        &authority,
        &none,
    )
    .expect("the same fixture emits an object with an empty admitted set");
    assert_eq!(
        object_with
            .assumptions
            .difference(&object_without.assumptions)
            .cloned()
            .collect::<BTreeSet<_>>(),
        admitted,
        "the object artifact's assumptions do not differ from the empty-admission emission by \
         exactly the admitted set"
    );
}
