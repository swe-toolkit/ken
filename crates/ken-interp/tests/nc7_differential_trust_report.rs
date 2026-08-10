use ken_elaborator::checked_core::{StableSymbol, SymbolNamespace};
use ken_elaborator::compiler_driver::{
    compile_ken_package_sources, CompilerManifest, CompilerSource, CompilerTargetKind,
    TargetSelector,
};
use ken_elaborator::erasure::erase_checked_core_package_for_target;
use ken_interp::{eval, EvalStore, EvalVal};
use ken_kernel::{declare_primitive, GlobalEnv, Level, PrimReduction, Term};
use ken_runtime::{
    InterpreterOracleObservation, NativeArtifactIdentity, NativeDifferentialStage,
    NativeDifferentialVerdict, NativeEvidenceFact, NativeFidelity, RuntimeExpr, RuntimeGroundValue,
    RuntimeObservation, RuntimeProgram, RuntimeValue,
};

struct OracleFixture {
    globals: GlobalEnv,
    store: EvalStore,
    term: Term,
}

fn interpreter_add_2_3_fixture() -> OracleFixture {
    let mut globals = GlobalEnv::new();
    let int_id = declare_primitive(
        &mut globals,
        vec![],
        Term::Type(Level::zero()),
        PrimReduction::OpaqueType,
    )
    .expect("Int primitive type");
    let int_ty = Term::Const {
        id: int_id,
        level_args: vec![],
    };
    let add_ty = Term::pi(int_ty.clone(), Term::pi(int_ty.clone(), int_ty.clone()));
    let add_id = declare_primitive(
        &mut globals,
        vec![],
        add_ty,
        PrimReduction::Op { symbol: "add_int" },
    )
    .expect("add_int primitive");
    let lit_2 = declare_primitive(&mut globals, vec![], int_ty.clone(), PrimReduction::Literal)
        .expect("literal 2");
    let lit_3 =
        declare_primitive(&mut globals, vec![], int_ty, PrimReduction::Literal).expect("literal 3");

    let mut store = EvalStore::new();
    store.num_values.insert(lit_2, EvalVal::Int(2));
    store.num_values.insert(lit_3, EvalVal::Int(3));

    let add = Term::Const {
        id: add_id,
        level_args: vec![],
    };
    let two = Term::Const {
        id: lit_2,
        level_args: vec![],
    };
    let three = Term::Const {
        id: lit_3,
        level_args: vec![],
    };
    let term = Term::app(Term::app(add, two), three);

    OracleFixture {
        globals,
        store,
        term,
    }
}

/// `RT-DYNAMIC-ARM-SCALAR-MERGE` `D1b-role-c1` — NC7 runs on a **real**
/// driver-compiled carrier, not a hand-built `RuntimeProgram`.
///
/// The hand-built fixture carried no checked role record and no pre-source
/// trusted-base roster, so once package-backed compilation began failing closed
/// on those it could not reach the native backend at all — and a differential
/// report is exactly what it exists to measure. The carrier below is produced by
/// the same `compile_ken_package_sources -> erase_checked_core_package_for_target`
/// path a real package takes, so its role record, roster and trust tuples are
/// the compiler's rather than this test's.
///
/// ⛔ The **oracle stays `ken-interp`'s**. That is the whole point of NC7: the
/// native side is compared against a real interpreter evaluation of a closed
/// core `Term`, not against the example's recorded observation. Migrating the
/// carrier must not quietly turn this into a seed-observation comparison, so
/// `oracle_observation` below is untouched.
const CARRIER_PKG: &str = "nc7_differential_carrier";
const CARRIER_SOURCE: &str = "const addTwoThree : Nat = Suc (Suc (Suc Zero))";

fn real_carrier() -> RuntimeProgram {
    let out = compile_ken_package_sources(
        &CompilerManifest::new(CARRIER_PKG, Vec::new()),
        vec![CompilerSource::new("src/main.ken", CARRIER_SOURCE)],
        TargetSelector::StableSymbol {
            package_identity: StableSymbol::new(
                SymbolNamespace::Module,
                vec![CARRIER_PKG.to_string()],
            ),
            symbol: StableSymbol::declaration(CARRIER_PKG, &[], "addTwoThree"),
            kind: CompilerTargetKind::Executable,
        },
    )
    .expect("the NC7 carrier source emits a checked-core package");
    let closure = out.closures.first().expect("selected target closure");
    erase_checked_core_package_for_target(&out.package, closure.reachable_declarations.iter())
        .expect("the NC7 carrier package erases")
}

/// The carrier's own identity. Asserting against this rather than against
/// hardcoded hashes keeps the rows about the differential lane instead of about
/// a fixture's literals.
fn carrier_identity(program: &RuntimeProgram) -> NativeArtifactIdentity {
    NativeArtifactIdentity {
        package_identity: program.package_identity.clone(),
        core_semantic_hash: program.core_semantic_hash,
        runtime_artifact_hash: program.artifact_hash,
    }
}

/// The `ken-interp` oracle. ⛔ UNCHANGED by the carrier migration: NC7 exists
/// to compare the native side against a real interpreter evaluation of a closed
/// core `Term`, and swapping this for the example's recorded observation would
/// delete the property while leaving the test green.
fn oracle_observation(artifact: NativeArtifactIdentity) -> InterpreterOracleObservation {
    let OracleFixture {
        globals,
        mut store,
        term,
    } = interpreter_add_2_3_fixture();
    let value = eval(&[], &term, &globals, &mut store);
    let observation = match value {
        EvalVal::Int(value) => RuntimeObservation::Returned(RuntimeGroundValue::Int((value).into())),
        other => panic!("NC7 oracle fixture must return Int, got {other:?}"),
    };
    InterpreterOracleObservation {
        artifact,
        observation,
        evidence_source: "ken-interp eval over GlobalEnv + closed core Term: add_int 2 3"
            .to_string(),
    }
}

/// The carrier's closed scalar example -- the one whose native run is compared
/// against the `ken-interp` oracle.
fn scalar_example(program: &RuntimeProgram) -> ken_runtime::RuntimeExample {
    program
        .examples
        .iter()
        .find(|example| example.name == "closed-scalar-primitive")
        .expect("the erased carrier carries the closed scalar example")
        .clone()
}

#[test]
fn interpreter_backed_f1_report_uses_real_oracle_not_seed_observation() {
    let program = real_carrier();
    let example = scalar_example(&program);
    let artifact = carrier_identity(&program);

    let report = ken_runtime::run_example_with_interpreter_observation(
        &program,
        &example,
        &ken_runtime::NativeSeedEnvironment::empty(),
        oracle_observation(artifact.clone()),
    );

    assert_eq!(report.oracle.artifact, artifact);
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
    // The report's identity is the carrier's own, asserted as a relation.
    assert_eq!(report.artifact, artifact);
    assert!(matches!(
        native.trust.toolchain.cranelift,
        NativeEvidenceFact::Unavailable { .. }
    ));
    assert!(matches!(
        native.trust.toolchain.linker,
        NativeEvidenceFact::Unavailable { .. }
    ));
    assert!(matches!(
        native.trust.toolchain.runtime,
        NativeEvidenceFact::Available { .. }
    ));
    // ⛔ THE ORACLE IS STILL `ken-interp`'s, not the example's observation.
    assert!(report
        .oracle
        .evidence_source
        .contains("ken-interp eval over GlobalEnv"));
    assert_eq!(
        native.observation,
        RuntimeObservation::Returned(RuntimeGroundValue::Int((5).into()))
    );

    // `D1b-role-c1`: and because this is a real carrier, the trust report now
    // states the assumptions admission proved. On the hand-built fixture this
    // set was necessarily empty, so it could assert nothing.
    let admitted = ken_runtime::native_program_admission(&program)
        .expect("the carrier is admitted")
        .admitted_trust()
        .clone();
    assert!(
        !admitted.is_empty(),
        "the carrier admits no trust, so the containment below would be vacuous"
    );
    let missing: Vec<_> = admitted.difference(&native.trust.assumptions).collect();
    assert!(
        missing.is_empty(),
        "the differential trust report dropped admitted trust: missing {missing:?}"
    );
}

#[test]
fn mismatch_report_names_compare_stage_after_both_sides_run() {
    let program = real_carrier();
    let mut example = scalar_example(&program);
    example.name = "mismatched-runtime-ir".to_string();
    example.ir = RuntimeExpr::Value(RuntimeValue::Int((4).into()));
    let artifact = carrier_identity(&program);

    let report = ken_runtime::run_example_with_interpreter_observation(
        &program,
        &example,
        &ken_runtime::NativeSeedEnvironment::empty(),
        oracle_observation(artifact),
    );

    assert!(report.native.is_some(), "native side must have run");
    assert!(matches!(
        report.verdict,
        NativeDifferentialVerdict::Mismatch {
            stage: NativeDifferentialStage::InterpreterNativeCompare,
            interpreter: RuntimeObservation::Returned(RuntimeGroundValue::Int(
                ken_runtime::RuntimeIntV1::Small(5),
            )),
            native: RuntimeObservation::Returned(RuntimeGroundValue::Int(
                ken_runtime::RuntimeIntV1::Small(4),
            )),
        }
    ));
}

/// An unsupported program emits no differential claim -- and the refusal is
/// still the **residual subset** blocker's, not a coarser earlier one.
///
/// ⛔ The effect is placed on a **declaration**, deliberately. Package-level
/// effect metadata is refused by admission, which now runs first, so using it
/// would move the diagnosis to the authority stage and this row would stop
/// discriminating what it names. Declaration-level effects are outside what
/// admission reconciles, so they are refused by the residual blocker -- which is
/// exactly the clause this row is about, and proves that blocker still fires
/// after admission rather than having been deleted with the coarse trust rule.
#[test]
fn unsupported_preflight_report_emits_no_differential_claim() {
    let mut program = real_carrier();
    let example = scalar_example(&program);
    let artifact = carrier_identity(&program);
    program
        .declarations
        .first_mut()
        .expect("the carrier has a declaration")
        .metadata
        .effects
        .insert("Console".to_string());

    // The carrier is still admitted: this is a residual-blocker refusal, not an
    // authority one, and saying so is what makes the row discriminating.
    ken_runtime::native_program_admission(&program)
        .expect("a declaration-level effect does not defeat admission");

    let report = ken_runtime::run_example_with_interpreter_observation(
        &program,
        &example,
        &ken_runtime::NativeSeedEnvironment::empty(),
        oracle_observation(artifact),
    );

    assert!(report.native.is_none());
    assert!(matches!(
        report.verdict,
        NativeDifferentialVerdict::Unsupported {
            stage: NativeDifferentialStage::BoundaryPreflight,
            construct: "RuntimeProgram",
            ..
        }
    ));
}

#[test]
fn oracle_identity_mismatch_emits_no_f1_and_does_not_run_native() {
    let program = real_carrier();
    let example = scalar_example(&program);
    let correct = carrier_identity(&program);
    let wrong_artifact = NativeArtifactIdentity {
        runtime_artifact_hash: correct.runtime_artifact_hash ^ 0x7777,
        ..correct.clone()
    };
    assert_ne!(
        correct, wrong_artifact,
        "the two identities must differ, or this row compares a thing with itself"
    );

    let report = ken_runtime::run_example_with_interpreter_observation(
        &program,
        &example,
        &ken_runtime::NativeSeedEnvironment::empty(),
        oracle_observation(wrong_artifact.clone()),
    );

    assert!(report.native.is_none());
    assert_eq!(report.artifact, correct);
    assert_eq!(report.oracle.artifact, wrong_artifact);
    assert!(matches!(
        report.verdict,
        NativeDifferentialVerdict::Unsupported {
            stage: NativeDifferentialStage::BoundaryPreflight,
            construct: "InterpreterOracleObservation",
            ..
        }
    ));
}
