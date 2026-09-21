//! PX8-TA public checked-bracket oriented-subcontinuation controls.

fn output_dir(name: &str) -> tempfile::TempDir {
    let prefix = format!("ken-px8ta-{name}-");
    tempfile::Builder::new().prefix(&prefix).tempdir().unwrap()
}

const NESTED_BRACKET_PROGRAM: &str = r#"program capabilities FS AFull
fn leaf_body (_resource : Resource FsHandle)
  : HostIO AFull (ResourceBodyResult Unit Unit) =
  Ret (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (ResourceBodyResult Unit Unit)
    (ResourceBodyOk Unit Unit MkUnit)

fn body_result
  (outcome : Result FileError (ResourceBracketResult Unit Unit))
  : HostIO AFull (ResourceBodyResult Unit Unit) =
  match outcome {
    Ok (ResourceBracketOk unit) |->
      Ret (Coproduct (FSOp AFull) AmbientOp)
        (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
        (ResourceBodyResult Unit Unit)
        (ResourceBodyOk Unit Unit MkUnit);
    Ok bracket |->
      Ret (Coproduct (FSOp AFull) AmbientOp)
        (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
        (ResourceBodyResult Unit Unit)
        (ResourceBodyErr Unit Unit MkUnit);
    Err error |->
      Ret (Coproduct (FSOp AFull) AmbientOp)
        (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
        (ResourceBodyResult Unit Unit)
        (ResourceBodyErr Unit Unit MkUnit)
  }

proc level_one_body
  (cap : Cap AFull) (_resource : Resource FsHandle)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result FileError (ResourceBracketResult Unit Unit))
    (ResourceBodyResult Unit Unit)
    (withResource AFull Unit Unit cap (bytes_encode "held-1.bin")
      ResourceMetadata leaf_body)
    (\outcome. body_result outcome)

proc level_two_body
  (cap : Cap AFull) (_resource : Resource FsHandle)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result FileError (ResourceBracketResult Unit Unit))
    (ResourceBodyResult Unit Unit)
    (withResource AFull Unit Unit cap (bytes_encode "held-1.bin")
      ResourceMetadata (\resource. level_one_body cap resource))
    (\outcome. body_result outcome)

fn after_root
  (outcome : Result FileError (ResourceBracketResult Unit Unit))
  : HostIO AFull ExitCode =
  match outcome {
    Ok (ResourceBracketOk unit) |-> host_exit AFull Success;
    Ok (ResourceBracketBodyError error) |-> host_exit AFull (Failure 81);
    Ok (ResourceBracketReleaseError error) |-> host_exit AFull (Failure 82);
    Ok (ResourceBracketBodyAndReleaseError body_error release_error) |->
      host_exit AFull (Failure 83);
    Err error |-> host_exit AFull (Failure 84)
  }

proc main (_input : ProcessInput) (caps : ProgramCaps AFull)
  : HostIO AFull ExitCode visits [FS] =
  match caps {
    MkProgramCaps cap |->
      bind (Coproduct (FSOp AFull) AmbientOp)
        (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
        (Result FileError (ResourceBracketResult Unit Unit)) ExitCode
        (withResource AFull Unit Unit cap (bytes_encode "held-0.bin")
          ResourceMetadata __ROOT_BODY__)
        (\outcome. after_root outcome)
  }
"#;

const PX8DS_SIBLING_RECURSION_PROGRAM: &str = r#"program capabilities FS APartial
proc countdown (fuel : Nat)
  : HostIO APartial (Result Int Bool) visits [Console] =
  match fuel {
    Zero |-> Ret (Coproduct (FSOp APartial) AmbientOp)
      (resp_coproduct (FSOp APartial) AmbientOp
        (fs_resp APartial) ambient_resp)
      (Result Int Bool) (Ok Int Bool True);
    Suc rest |-> bind (Coproduct (FSOp APartial) AmbientOp)
      (resp_coproduct (FSOp APartial) AmbientOp
        (fs_resp APartial) ambient_resp)
      Bool (Result Int Bool)
      (host_console APartial Bool (is_terminal Stdout))
      (\terminal. match terminal {
        False |-> countdown rest;
        True |-> countdown rest
      })
  }

fn after_countdown (_outcome : Result Int Bool)
  : HostIO APartial (ResourceBodyResult Unit Unit) =
  Ret (Coproduct (FSOp APartial) AmbientOp)
    (resp_coproduct (FSOp APartial) AmbientOp
      (fs_resp APartial) ambient_resp)
    (ResourceBodyResult Unit Unit) (ResourceBodyOk Unit Unit MkUnit)

proc after_first_countdown (_outcome : Result Int Bool)
  : HostIO APartial (ResourceBodyResult Unit Unit) visits [Console] =
  bind (Coproduct (FSOp APartial) AmbientOp)
    (resp_coproduct (FSOp APartial) AmbientOp
      (fs_resp APartial) ambient_resp)
    (Result Int Bool) (ResourceBodyResult Unit Unit)
    (countdown (Suc Zero))
    (\outcome. after_countdown outcome)

proc selected_buffer_body (terminal : Bool)
  : HostIO APartial (ResourceBodyResult Unit Unit) visits [Console] =
  match terminal {
    False |-> after_countdown (Ok Int Bool False);
    True |-> bind (Coproduct (FSOp APartial) AmbientOp)
      (resp_coproduct (FSOp APartial) AmbientOp
        (fs_resp APartial) ambient_resp)
      (Result Int Bool) (ResourceBodyResult Unit Unit)
      (countdown (Suc (Suc Zero)))
      (\outcome. after_first_countdown outcome)
  }

proc buffer_body (_buffer : BufferHandle)
  : HostIO APartial (ResourceBodyResult Unit Unit) visits [Console] =
  bind (Coproduct (FSOp APartial) AmbientOp)
    (resp_coproduct (FSOp APartial) AmbientOp
      (fs_resp APartial) ambient_resp)
    Bool (ResourceBodyResult Unit Unit)
    (host_console APartial Bool (is_terminal Stdout))
    (\terminal. selected_buffer_body terminal)

fn finish_buffer
  (outcome : Result ResourceError (ResourceBracketResult Unit Unit))
  : HostIO APartial ExitCode =
  match outcome {
    Err _ |-> host_exit APartial (Failure 91);
    Ok (ResourceBracketOk _) |-> host_exit APartial Success;
    Ok _ |-> host_exit APartial (Failure 92)
  }

proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode visits [FS, Console] =
  bind (Coproduct (FSOp APartial) AmbientOp)
    (resp_coproduct (FSOp APartial) AmbientOp
      (fs_resp APartial) ambient_resp)
    (Result ResourceError (ResourceBracketResult Unit Unit)) ExitCode
    (withBuffer APartial Unit Unit (2 : Int) buffer_body)
    (\outcome. finish_buffer outcome)
"#;

#[cfg(target_os = "linux")]
fn nested_bracket_source(depth: usize) -> String {
    let body = match depth {
        1 => "leaf_body",
        2 => {
            r#"(\resource.
          bind (Coproduct (FSOp AFull) AmbientOp)
            (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
            (Result FileError (ResourceBracketResult Unit Unit))
            (ResourceBodyResult Unit Unit)
            (withResource AFull Unit Unit cap (bytes_encode "held-1.bin")
              ResourceMetadata leaf_body)
            (\outcome. body_result outcome))"#
        }
        3 => {
            r#"(\resource.
          bind (Coproduct (FSOp AFull) AmbientOp)
            (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
            (Result FileError (ResourceBracketResult Unit Unit))
            (ResourceBodyResult Unit Unit)
            (withResource AFull Unit Unit cap (bytes_encode "held-1.bin")
              ResourceMetadata (\inner_resource.
                bind (Coproduct (FSOp AFull) AmbientOp)
                  (resp_coproduct (FSOp AFull) AmbientOp
                    (fs_resp AFull) ambient_resp)
                  (Result FileError (ResourceBracketResult Unit Unit))
                  (ResourceBodyResult Unit Unit)
                  (withResource AFull Unit Unit cap (bytes_encode "held-2.bin")
                    ResourceMetadata leaf_body)
                  (\inner_outcome. body_result inner_outcome)))
            (\outcome. body_result outcome))"#
        }
        _ => panic!("PX8-TA public control supports depths one through three"),
    };
    NESTED_BRACKET_PROGRAM.replace("__ROOT_BODY__", body)
}

#[cfg(target_os = "linux")]
fn nested_bracket_output_dir(depth: usize, label: &str) -> tempfile::TempDir {
    let dir = output_dir(&format!("depth-{depth}-{label}"));
    for index in 0..depth {
        std::fs::write(
            dir.path().join(format!("held-{index}.bin")),
            format!("held resource {index}"),
        )
        .unwrap();
    }
    dir
}

#[cfg(target_os = "linux")]
fn compile_nested_bracket(
    depth: usize,
    label: &str,
) -> (
    tempfile::TempDir,
    Result<
        ken_elaborator::compiler_driver::NativeProgramBuildOutput,
        ken_elaborator::compiler_driver::NativeProgramBuildError,
    >,
) {
    let source = nested_bracket_source(depth);
    let dir = nested_bracket_output_dir(depth, label);
    let result = ken_cli::build_native_program(
        &source,
        ken_cli::SourceFormat::Ken,
        &format!("px8ta-depth-{depth}-{label}"),
        dir.path(),
    );
    (dir, result)
}

#[cfg(target_os = "linux")]
fn compile_nested_bracket_with_diagnostics(
    depth: usize,
    label: &str,
) -> (
    tempfile::TempDir,
    Result<
        ken_elaborator::compiler_driver::NativeProgramBuildOutput,
        ken_elaborator::compiler_driver::NativeProgramBuildError,
    >,
    Vec<ken_runtime::StaticResponseFeasibilityDiagnostic>,
) {
    let source = nested_bracket_source(depth);
    let dir = nested_bracket_output_dir(depth, label);
    let (result, diagnostics) = ken_runtime::with_static_response_feasibility_diagnostics(|| {
        ken_cli::build_native_program(
            &source,
            ken_cli::SourceFormat::Ken,
            &format!("px8ta-depth-{depth}-{label}"),
            dir.path(),
        )
    });
    (dir, result, diagnostics)
}

#[cfg(target_os = "linux")]
fn run_depth(depth: usize) -> (ken_runtime::EffectObservation, usize) {
    let (dir, output) = compile_nested_bracket(depth, "run");
    let output = output.unwrap_or_else(|error| {
        panic!("depth {depth} checked nested bracket reaches native lowering: {error:?}")
    });
    let plan = output
        .runtime_program
        .erased_core
        .metadata
        .checked_core
        .metadata
        .values()
        .find(|bytes| bytes.starts_with(ken_runtime::ORIENTED_SUBCONTINUATION_PLAN_V1_HEADER))
        .and_then(|bytes| ken_runtime::OrientedSubcontinuationPlanV1::decode(bytes).ok())
        .expect("checked nested bracket transports its oriented answer plan");
    let observation = ken_runtime::run_bound_process_effect_observation(
        &output.artifact,
        &ken_runtime::NativeEffectRunOptionsV1 {
            arguments: Vec::new(),
            environment: Vec::new(),
            cwd: dir.path().to_owned(),
            plan_hash: output.plan_transport_hash,
        },
    )
    .expect("linked nested bracket emits its canonical observation");
    (observation, plan.frames.len())
}

#[cfg(target_os = "linux")]
fn nested_bracket_resource_order(
    observation: &ken_runtime::EffectObservation,
) -> (Vec<u64>, Vec<u64>) {
    let acquisitions = observation
        .effect_trace
        .iter()
        .filter(|event| event.operation == ken_runtime::HostOpV1::FsOpen)
        .map(|event| event.resource_bindings[0].1 .0)
        .collect::<Vec<_>>();
    let releases = observation
        .effect_trace
        .iter()
        .filter(|event| event.operation == ken_runtime::HostOpV1::ResourceRelease)
        .map(|event| event.resource_bindings[0].1 .0)
        .collect::<Vec<_>>();
    (acquisitions, releases)
}

#[cfg(target_os = "linux")]
// Readmitted under RT-IGNORED-PASSING-ROWS, row 4 of 11.
//
// This row carried THREE layers of annotation, each refuting the one above it
// and all three still present. They are struck together rather than appended to:
//
//   1. "Ignored pending RT-CARRIER-BYTESPAN-OBSERVE ... it refuses at object
//      emission, so the program never executes" -- refuted by layer 2, three
//      lines below it, and by the row: the program emits, executes, exits 0.
//   2. "RT-SITEOP-CARRIED-WITNESS D1a/D2 ... D5 byte-span observation was not
//      the blocker" -- the correction to layer 1, filed below it.
//   3. "RT-SITEOP-CARRIED-WITNESS D2: ... this row next refuses because a
//      carried recursive hypothesis is an eliminated value, not a callable" --
//      the live label. Does not reproduce: no refusal, and neither the
//      byte-span nor the eliminated-value signature appears in any run.
//
// READMITTED ON A MUTATION, NOT ON THE GREEN. Perturbing the ResourceRelease
// arm of ken-host effect_v1.rs dispatch (the Target binding pushed from
// pending.identity, +1000) reds the strict-LIFO assertion in
// assert_depth_finishes_and_releases_lifo -- releases against opens reversed --
// with the perturbation visible in it: left [ResourceTraceIdentityV1(1001)]
// against right [ResourceTraceIdentityV1(1)]. An earlier mutation of abi_v1.rs
// record_resource_settlements left the row green: that is the process-exit
// finalize-all path, and a bracket that releases explicitly never reaches it.
#[test]
fn public_one_level_bracket_finishes_and_releases() {
    assert_depth_finishes_and_releases_lifo(1);
}

#[cfg(target_os = "linux")]
#[test]
fn public_two_level_bracket_release_only_suffix_is_lifo() {
    std::thread::Builder::new()
        .name("px8ta-two-level-release-only".to_string())
        .stack_size(256 * 1024 * 1024)
        .spawn(|| {
            let ((observation, _planned_frames), diagnostics) =
                ken_runtime::with_static_response_feasibility_diagnostics(|| run_depth(2));
            assert_eq!(diagnostics.len(), 1);
            let bounded_releases = diagnostics[0]
                .static_response_deferred
                .iter()
                .filter(|row| {
                    row.operation == "ResourceRelease"
                        && row.sub_case == "UnconsumedTransportCaller"
                })
                .collect::<Vec<_>>();
            assert_eq!(
                bounded_releases
                    .iter()
                    .filter(|row| row.handler_owner.is_some())
                    .count(),
                1,
                "the inner release did not acquire one unique bounded handler owner: \
                 {bounded_releases:?}"
            );
            let (acquisitions, releases) = nested_bracket_resource_order(&observation);
            assert_eq!(acquisitions.len(), 2);
            assert_eq!(releases.len(), 2);
            assert_eq!(
                releases,
                acquisitions.iter().rev().copied().collect::<Vec<_>>(),
                "the release-only suffix must settle inner before outer"
            );
        })
        .expect("spawn two-level release-only bracket control")
        .join()
        .expect("two-level release-only bracket control thread");
}

#[cfg(target_os = "linux")]
#[test]
fn public_two_level_bracket_release_only_suffix_mutation_reddens() {
    std::thread::Builder::new()
        .name("px8ta-two-level-release-only-mutation".to_string())
        .stack_size(256 * 1024 * 1024)
        .spawn(|| {
            let ((observation, _planned_frames), applications) =
                ken_runtime::with_release_only_suffix_admission_suppressed(|| run_depth(2));
            assert!(applications > 0, "release-only suppression did not apply");
            let (acquisitions, releases) = nested_bracket_resource_order(&observation);
            assert_eq!(releases, acquisitions, "mutation must restore [r1,r2]");
            let red = std::panic::catch_unwind(|| {
                assert_eq!(
                    releases,
                    acquisitions.iter().rev().copied().collect::<Vec<_>>()
                );
            });
            assert!(red.is_err(), "release-only mutation did not redden D2a-B");
            assert!(
                ken_runtime::release_only_suffix_admission_suppressed_is_exact(),
                "release-only suppression mutation did not restore"
            );
        })
        .expect("spawn two-level release-only mutation control")
        .join()
        .expect("two-level release-only mutation control thread");
}

#[cfg(target_os = "linux")]
/// Promise class: durable invariant. Every nested bracket settles in reverse
/// acquisition order before its enclosing bracket resumes, at both reaching
/// depths represented by this public fixture.
///
/// MEASURED: each independently compiled depth emits one acquisition and one
/// release per bracket, exits successfully, and releases in exact reverse.
/// CLAIMED: native bracket settlement is lexical rather than an artifact of one
/// planner classification or one depth.
/// THE GAP: the three depth-3 replacement controls below independently break
/// the selected-caller edge, the typed-K-context edge, and Arm A's route choice.
#[test]
fn public_two_three_level_brackets_finish_and_release_lifo() {
    // Lowering nested checked brackets is stack-hungry, and libtest hands a
    // test a 2 MiB (2048 KiB) worker thread. Bisected minimum passing stack
    // for this test, measured on both commits at 64 KiB resolution:
    //
    //   70bd2c74   > 1984 KiB, <= 2048 KiB   -- cleared the default by < 64 KiB
    //   08633b3c   > 2112 KiB, <= 2176 KiB   -- does not fit; SIGABRT
    //
    // Production is unaffected: this test drives the lowering directly on the
    // libtest worker, while a real build runs on the main thread (8 MiB by
    // `ulimit -s`), so the product had ~3.7x the headroom this harness gave
    // itself. The wrapper below gives the harness **at least** the product's
    // headroom, exactly as
    // `px8ds_real_same_depth_path_runs_exact_edges` in
    // this file already does. ⚠ Not "matches the product": 256 MiB is 32x the
    // product's 8 MiB, and saying "matches" would understate by how much this
    // stops being able to observe a stack regression at all.
    //
    // ⛔ This is a harness fix and nothing more. It is NOT scaling evidence for
    // per-static-origin target functions, and it does NOT discharge the n=3..7
    // nesting-depth gate -- the fixture only defines depths 1/2/3 by hand, so
    // n >= 4 is not even expressible here. The thresholds above are recorded so
    // the next reader sees how little room there was instead of rediscovering
    // it from a red shard.
    std::thread::Builder::new()
        .name("px8ta-nested-brackets".to_string())
        .stack_size(256 * 1024 * 1024)
        .spawn(|| {
            for depth in 2..=3 {
                assert_depth_finishes_and_releases_lifo(depth);
            }
        })
        .expect("spawn large-stack nested-bracket control")
        .join()
        .expect("nested-bracket control thread");
}

#[cfg(target_os = "linux")]
const DEPTH_THREE_MISSING_SELECTED_CALLER_ERROR: &str = r#"Packaging(ObjectLinkerPackagingError { stage: ObjectEmission, field: "checked_process_object", reason: "Cranelift backend failure: module operation failed: a forward-declared response owner has no verified selected incoming call: owner=StaticResponseOwnerId(0), context=ContinuationContextId(2), preexisting=false, caller=ContinuationCallIdentity { token: ContinuationSpecializationCallToken { producer_owner: PredeclaredFunctionId(5), emission_owner: Specialization(ContinuationSpecializationId(3)), producer_result_origin: StaticOriginId(896), producer_construct_origin: StaticOriginId(515), producer_alternative: 1, call_site_sequence: 0, target: ContinuationSpecializationId(4), worker: ContinuationWorkerProvenance { parent_origin: StaticOriginId(12), producer_origin: StaticOriginId(515), sibling_position: 1, closure_origin: StaticOriginId(510), body_origin: StaticOriginId(502), declared_arity: 1, captures: [ContinuationWorkerCaptureProvenance { ordinal: 0, owner: PredeclaredFunctionId(5), closure_origin: StaticOriginId(510), source: Lexical(StaticOriginId(509)), lifetime: ActivationOwned }, ContinuationWorkerCaptureProvenance { ordinal: 1, owner: PredeclaredFunctionId(5), closure_origin: StaticOriginId(510), source: Lexical(StaticOriginId(508)), lifetime: ActivationOwned }, ContinuationWorkerCaptureProvenance { ordinal: 2, owner: PredeclaredFunctionId(5), closure_origin: StaticOriginId(510), source: Lexical(StaticOriginId(507)), lifetime: ActivationOwned }, ContinuationWorkerCaptureProvenance { ordinal: 3, owner: PredeclaredFunctionId(5), closure_origin: StaticOriginId(510), source: Lexical(StaticOriginId(506)), lifetime: ActivationOwned }, ContinuationWorkerCaptureProvenance { ordinal: 4, owner: PredeclaredFunctionId(5), closure_origin: StaticOriginId(510), source: Lexical(StaticOriginId(505)), lifetime: ActivationOwned }, ContinuationWorkerCaptureProvenance { ordinal: 5, owner: PredeclaredFunctionId(5), closure_origin: StaticOriginId(510), source: Lexical(StaticOriginId(504)), lifetime: ActivationOwned }, ContinuationWorkerCaptureProvenance { ordinal: 6, owner: PredeclaredFunctionId(5), closure_origin: StaticOriginId(510), source: Lexical(StaticOriginId(503)), lifetime: ActivationOwned }] } }, recursive_position: 1 }, disposition=Some(InlineNoCall)" })"#;

#[cfg(target_os = "linux")]
/// Promise class: durable invariant. Arm A's depth-3 route must enter the exact
/// selected response owner rather than restoring the former ordinary K target.
///
/// MEASURED: changing only the selected-caller target applies positively,
/// leaves the typed planner diagnostic byte-for-value unchanged, and reaches
/// the finished-artifact selected-incoming-call refusal.
/// CLAIMED: depth-3 success uses the selected-caller to response-owner edge.
/// THE GAP: the mutation acts at real target resolution and restoration is
/// checked by a fresh exact compile with the baseline typed diagnostic.
#[test]
fn public_depth_three_selected_caller_retarget_proves_replacement_edge() {
    std::thread::Builder::new()
        .name("px8ta-depth-three-selected-caller".to_string())
        .stack_size(256 * 1024 * 1024)
        .spawn(|| {
            let (_baseline_dir, baseline_result, baseline_diagnostics) =
                compile_nested_bracket_with_diagnostics(3, "selected-caller-baseline");
            baseline_result.expect("the exact depth-three replacement route compiles");
            assert_eq!(baseline_diagnostics.len(), 1);
            assert!(
                !baseline_diagnostics[0].static_response_owners.is_empty(),
                "the depth-three program supplies no selected response owner"
            );

            let ((_mutated_dir, mutated_result, mutated_diagnostics), applications) =
                ken_runtime::with_static_response_caller_retarget_mutation(
                    ken_runtime::StaticResponseCallerRetargetMutation::RestoreSelectedKTarget,
                    || {
                        compile_nested_bracket_with_diagnostics(
                            3,
                            "selected-caller-restored-k-target",
                        )
                    },
                );
            assert!(applications > 0, "selected-caller mutation did not apply");
            let error = mutated_result
                .expect_err("restoring the former K target must leave the owner unentered");
            let error = format!("{error:?}");
            assert_eq!(error, DEPTH_THREE_MISSING_SELECTED_CALLER_ERROR);
            assert_eq!(
                mutated_diagnostics, baseline_diagnostics,
                "selected-caller mutation changed the typed planner diagnostic"
            );
            assert!(
                ken_runtime::static_response_caller_retarget_mutation_is_exact(),
                "selected-caller mutation did not restore"
            );

            let (_restored_dir, restored_result, restored_diagnostics) =
                compile_nested_bracket_with_diagnostics(3, "selected-caller-restored");
            restored_result.expect("the selected-caller route must restore");
            assert_eq!(restored_diagnostics, baseline_diagnostics);
        })
        .expect("spawn depth-three selected-caller control")
        .join()
        .expect("depth-three selected-caller control thread");
}

#[cfg(target_os = "linux")]
/// Promise class: durable invariant. When the same selected caller is both
/// uncalled and compositionally consumed, the unchanged final detector reports
/// the complete missing-selected-caller diagnostic before overlap.
#[test]
fn public_depth_three_missing_caller_precedes_composed_overlap() {
    std::thread::Builder::new()
        .name("px8ta-depth-three-coverage-precedence".to_string())
        .stack_size(256 * 1024 * 1024)
        .spawn(|| {
            let (_baseline_dir, baseline_result, baseline_diagnostics) =
                compile_nested_bracket_with_diagnostics(3, "coverage-precedence-baseline");
            baseline_result.expect("the exact depth-three replacement route compiles");
            assert_eq!(baseline_diagnostics.len(), 1);

            let ((_mutated_dir, mutated_result, mutated_diagnostics), applications) =
                ken_runtime::with_static_response_caller_retarget_mutation(
                    ken_runtime::StaticResponseCallerRetargetMutation::RestoreSelectedKTargetWithComposedOverlap,
                    || {
                        compile_nested_bracket_with_diagnostics(
                            3,
                            "coverage-precedence-dual-defect",
                        )
                    },
                );
            assert!(applications > 0, "dual-defect mutation did not apply");
            let error = mutated_result
                .expect_err("the dual defect must refuse at final owner coverage");
            assert_eq!(
                format!("{error:?}"),
                DEPTH_THREE_MISSING_SELECTED_CALLER_ERROR,
            );
            assert_eq!(mutated_diagnostics, baseline_diagnostics);
            assert!(ken_runtime::static_response_caller_retarget_mutation_is_exact());

            let (_restored_dir, restored_result, restored_diagnostics) =
                compile_nested_bracket_with_diagnostics(3, "coverage-precedence-restored");
            restored_result.expect("final owner coverage precedence must restore");
            assert_eq!(restored_diagnostics, baseline_diagnostics);
        })
        .expect("spawn depth-three coverage precedence control")
        .join()
        .expect("depth-three coverage precedence control thread");
}

#[cfg(target_os = "linux")]
/// Promise class: durable invariant. A selected depth-3 response owner calls
/// its exact typed K context once before returning.
///
/// MEASURED: omitting only the K call applies positively, preserves the typed
/// planner diagnostic, and reaches the finished-owner zero-K-call refusal.
/// CLAIMED: route replacement retains the response-owner to typed-K edge.
/// THE GAP: the mutation changes emitted owner-body structure rather than the
/// classifier, and a fresh exact compile checks restoration.
#[test]
fn public_depth_three_owner_omit_k_call_proves_typed_context_edge() {
    std::thread::Builder::new()
        .name("px8ta-depth-three-owner-k".to_string())
        .stack_size(256 * 1024 * 1024)
        .spawn(|| {
            const REFUSAL: &str = "a response owner emitted 0 K calls instead of exactly one";

            let (_baseline_dir, baseline_result, baseline_diagnostics) =
                compile_nested_bracket_with_diagnostics(3, "owner-k-baseline");
            baseline_result.expect("the exact depth-three response-owner body compiles");
            assert_eq!(baseline_diagnostics.len(), 1);
            assert!(
                !baseline_diagnostics[0].static_response_owners.is_empty(),
                "the depth-three program supplies no response owner"
            );

            let ((_mutated_dir, mutated_result, mutated_diagnostics), applications) =
                ken_runtime::with_static_response_owner_body_mutation(
                    ken_runtime::StaticResponseOwnerBodyMutation::OmitKCall,
                    || compile_nested_bracket_with_diagnostics(3, "owner-k-omitted"),
                );
            assert!(applications > 0, "owner K-call mutation did not apply");
            let error = mutated_result
                .expect_err("a selected response owner without its K call must refuse");
            let error = format!("{error:?}");
            assert!(
                error.contains(REFUSAL),
                "owner K-call mutation reached the wrong refusal: {error}"
            );
            assert_eq!(
                error.matches(REFUSAL).count(),
                1,
                "owner K-call mutation must report one exact refusal: {error}"
            );
            assert_eq!(
                mutated_diagnostics, baseline_diagnostics,
                "owner K-call mutation changed the typed planner diagnostic"
            );
            assert!(
                ken_runtime::static_response_owner_body_mutation_is_exact(),
                "owner K-call mutation did not restore"
            );

            let (_restored_dir, restored_result, restored_diagnostics) =
                compile_nested_bracket_with_diagnostics(3, "owner-k-restored");
            restored_result.expect("the exact response-owner K call must restore");
            assert_eq!(restored_diagnostics, baseline_diagnostics);
        })
        .expect("spawn depth-three owner K-call control")
        .join()
        .expect("depth-three owner K-call control thread");
}

#[cfg(target_os = "linux")]
/// Promise class: transition sentinel. Removing Arm A's bounded authority seed
/// must restore the old undeclared-target refusal on the unchanged depth-3
/// program. That red proves route replacement; the refusal is not desired
/// product behavior and this sentinel does not authorize preserving it.
///
/// MEASURED: suppressing the whole single-exclusive authority arm applies
/// positively and restores the exact former refusal.
/// CLAIMED: Arm A eliminates the ordinary path that formed the invalid claim.
/// THE GAP: the production row above is the positive control, while a fresh
/// post-mutation compile proves thread-local restoration.
#[test]
fn public_depth_three_authority_seed_suppression_proves_route_replacement() {
    std::thread::Builder::new()
        .name("px8ta-depth-three-authority-seed".to_string())
        .stack_size(256 * 1024 * 1024)
        .spawn(|| {
            const REFUSAL: &str = "ContinuationSpecialization: the claimed continuation target \
                                   was not declared into this function";

            let (_baseline_dir, baseline_result, baseline_diagnostics) =
                compile_nested_bracket_with_diagnostics(3, "authority-seed-baseline");
            baseline_result.expect("Arm A's depth-three replacement route compiles");
            assert_eq!(baseline_diagnostics.len(), 1);

            let ((_mutated_dir, mutated_result, _mutated_diagnostics), applications) =
                ken_runtime::with_single_exclusive_plane_authority_suppressed(|| {
                    compile_nested_bracket_with_diagnostics(3, "authority-seed-suppressed")
                });
            assert!(applications > 0, "Arm A suppression did not apply");
            let error = mutated_result
                .expect_err("removing Arm A must restore the former ordinary-path refusal");
            let error = format!("{error:?}");
            assert!(
                error.contains(REFUSAL),
                "Arm A suppression reached the wrong refusal: {error}"
            );
            assert_eq!(
                error.matches(REFUSAL).count(),
                1,
                "Arm A suppression must report one exact refusal: {error}"
            );
            assert!(
                ken_runtime::single_exclusive_plane_authority_suppressed_is_exact(),
                "Arm A suppression did not restore"
            );

            let (_restored_dir, restored_result, restored_diagnostics) =
                compile_nested_bracket_with_diagnostics(3, "authority-seed-restored");
            restored_result.expect("Arm A's replacement route must restore");
            assert_eq!(restored_diagnostics, baseline_diagnostics);
        })
        .expect("spawn depth-three authority-seed control")
        .join()
        .expect("depth-three authority-seed control thread");
}

#[cfg(target_os = "linux")]
// Durable invariant: the test-only retired flat-order plan has no M4 crossing
// authority. It must remain on the ordinary fail-closed closure refusal rather
// than acquiring the production plan's captured-environment representation.
#[test]
fn px8ds_retired_flat_order_does_not_gain_m4_representation() {
    std::thread::Builder::new()
        .name("px8ds-retired-flat".to_string())
        .stack_size(256 * 1024 * 1024)
        .spawn(run_px8ds_retired_flat_control)
        .expect("spawn large-stack PX8-DS retired-flat control")
        .join()
        .expect("PX8-DS retired-flat control thread");
}

#[cfg(target_os = "linux")]
fn run_px8ds_retired_flat_control() {
    const REFUSAL: &str = "unsupported runtime-IR lowering: Closure: a closure cannot cross the boundary: it is runtime-local and live-domain only, and it has no durable lane";

    let retired_dir = output_dir("px8ds-retired-flat");
    let retired = ken_runtime::with_px8ds_retired_flat_order(|| {
        ken_cli::build_native_program(
            PX8DS_SIBLING_RECURSION_PROGRAM,
            ken_cli::SourceFormat::Ken,
            "px8ds-retired-flat",
            retired_dir.path(),
        )
    })
    .expect_err("the retired flat-order plan must retain the closure refusal");
    let retired = format!("{retired:?}");
    assert!(
        retired.contains(REFUSAL),
        "the retired plan must reach the exact closure refusal: {retired}"
    );
    assert_eq!(
        retired.matches(REFUSAL).count(),
        1,
        "the retired plan must report one exact closure refusal: {retired}"
    );
}

#[cfg(target_os = "linux")]
// Existing baseline provisioning for the ordinary px8ds row. This WP neither
// raises nor otherwise changes its stack budget.
//
// With ambient RUST_MIN_STACK absent, one freshly compiled temporary probe
// changed only this named ignored row: its Builder read the probed byte count
// before spawning, and its thread body called
// `observe_px8ds_real_same_depth_path` directly. The compiler reported the
// constant and assertion-bearing helper as unused, confirming that the probe
// body, rather than a cached ordinary body, was in the test binary. The same
// binary was bisected at 64 KiB resolution and run twice at each boundary:
//
//   1792 KiB  stack overflow twice
//   1856 KiB  completes the build-and-observe path twice
//
// The committed 256 MiB provision is therefore 260,288 KiB (254.1875 MiB)
// above the measured passing bound and 141.24 times that bound (140.24 times
// extra headroom). This is baseline fixture provisioning, not a regression
// repair.
const PX8DS_THREAD_STACK_BYTES: usize = 256 * 1024 * 1024;

#[cfg(target_os = "linux")]
// This focused native row compiles, links, and runs a large artifact on its
// separately provisioned thread. It is invoked explicitly by targeted Runtime
// validation rather than by the default package suite. The assertions pin a
// successful false branch with one Console observation and one release.
//
// Registered in .github/ignored-test-exemptions.toml as `policy-cost` under
// RT-IGNORED-PASSING-ROWS, the disposition the frame pre-classified for it
// (D-REGISTER). Its ignore is a standing cost decision, not a defect, so it
// should never have been a sweep finding in either direction.
#[test]
#[ignore = "focused native resource-cost row; run outside default suite"]
fn px8ds_real_same_depth_path_runs_exact_edges() {
    std::thread::Builder::new()
        .name("px8ds-real-siblings".to_string())
        .stack_size(PX8DS_THREAD_STACK_BYTES)
        .spawn(run_px8ds_real_same_depth_path)
        .expect("spawn large-stack PX8-DS ordinary-plan half")
        .join()
        .expect("PX8-DS ordinary-plan thread");
}

#[cfg(target_os = "linux")]
fn observe_px8ds_real_same_depth_path() -> ken_runtime::EffectObservation {
    let exact_dir = output_dir("px8ds-exact-edges");
    let exact = ken_cli::build_native_program(
        PX8DS_SIBLING_RECURSION_PROGRAM,
        ken_cli::SourceFormat::Ken,
        "px8ds-exact-edges",
        exact_dir.path(),
    )
    .expect("exact dynamic edges compile the same checked source");
    ken_runtime::run_bound_process_effect_observation(
        &exact.artifact,
        &ken_runtime::NativeEffectRunOptionsV1 {
            arguments: Vec::new(),
            environment: Vec::new(),
            cwd: exact_dir.path().to_owned(),
            plan_hash: exact.plan_transport_hash,
        },
    )
    .expect("the exact-edge linked artifact runs")
}

#[cfg(target_os = "linux")]
fn run_px8ds_real_same_depth_path() {
    let observation = observe_px8ds_real_same_depth_path();
    assert_eq!(observation.exit_status, 0, "{observation:?}");
    assert_eq!(observation.terminal_error, None);
    assert_eq!(
        observation
            .effect_trace
            .iter()
            .filter(|event| event.operation == ken_runtime::HostOpV1::ConsoleIsTerminal)
            .count(),
        1,
        "the live false branch must skip both recursive siblings"
    );
    assert_eq!(
        observation
            .effect_trace
            .iter()
            .map(|event| event.operation)
            .collect::<Vec<_>>(),
        vec![
            ken_runtime::HostOpV1::BufferAllocate,
            ken_runtime::HostOpV1::ConsoleIsTerminal,
            ken_runtime::HostOpV1::ResourceRelease,
        ]
    );
}

#[cfg(target_os = "linux")]
fn assert_depth_finishes_and_releases_lifo(depth: usize) {
    let (observation, planned_frames) = run_depth(depth);
    assert_eq!(observation.exit_status, 0, "depth {depth}: {observation:?}");
    assert_eq!(observation.terminal_error, None, "depth {depth}");
    assert!(
        planned_frames >= depth,
        "depth {depth} must retain every checked bracket continuation"
    );

    let opens = observation
        .effect_trace
        .iter()
        .filter(|event| event.operation == ken_runtime::HostOpV1::FsOpen)
        .map(|event| event.resource_bindings[0].1.clone())
        .collect::<Vec<_>>();
    let releases = observation
        .effect_trace
        .iter()
        .filter(|event| event.operation == ken_runtime::HostOpV1::ResourceRelease)
        .map(|event| event.resource_bindings[0].1.clone())
        .collect::<Vec<_>>();
    assert_eq!(opens.len(), depth, "depth {depth} acquisition count");
    assert_eq!(releases.len(), depth, "depth {depth} release count");
    assert_eq!(
        releases,
        opens.into_iter().rev().collect::<Vec<_>>(),
        "depth {depth} releases must be strict LIFO"
    );
}
