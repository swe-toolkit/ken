//! PX7-F public checked-Ken linked-native discriminators.

fn output_dir(name: &str) -> tempfile::TempDir {
    let prefix = format!("ken-px7f-{name}-");
    tempfile::Builder::new().prefix(&prefix).tempdir().unwrap()
}

fn run(name: &str, source: &str) -> ken_runtime::EffectObservation {
    let dir = output_dir(name);
    std::fs::write(dir.path().join("held.bin"), b"held resource").unwrap();
    let output = ken_cli::build_native_program(
        source,
        ken_cli::SourceFormat::Ken,
        name,
        dir.path(),
        ken_runtime::boundary_resource_profile::starter_smoke_profile(),
    )
        .expect("PX7-F checked program reaches the native resource lane");
    let oriented = output
        .runtime_program
        .erased_core
        .metadata
        .checked_core
        .metadata
        .values()
        .find(|bytes| bytes.starts_with(ken_runtime::ORIENTED_SUBCONTINUATION_PLAN_V1_HEADER))
        .and_then(|bytes| ken_runtime::OrientedSubcontinuationPlanV1::decode(bytes).ok())
        .expect("resource bracket carries its checked oriented plan");
    assert!(
        !oriented.frames.is_empty(),
        "the reaching resource bracket must retain checked answer interfaces"
    );
    let observation = ken_runtime::run_bound_process_effect_observation(
        &output.artifact,
        &ken_runtime::NativeEffectRunOptionsV1 {
            arguments: Vec::new(),
            environment: Vec::new(),
            cwd: dir.path().to_owned(),
            plan_hash: output.plan_transport_hash,
        },
    )
    .expect("linked PX7-F child emits its canonical observation");
    observation
}

const ESCAPE_CLOSED: &str = r#"program capabilities FS AFull
fn escape_body (resource : Resource ResourceKind.FsHandle)
  : HostIO AFull (ResourceBodyResult Unit (Resource ResourceKind.FsHandle)) =
  Ret (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (ResourceBodyResult Unit (Resource ResourceKind.FsHandle))
    (ResourceBodyOk Unit (Resource ResourceKind.FsHandle) resource)

proc after_escape (bracket : ResourceBracketResult Unit (Resource ResourceKind.FsHandle))
  : HostIO AFull ExitCode visits [FS] =
  match bracket {
    ResourceBracketOk resource |->
      bind (Coproduct (FSOp AFull) AmbientOp)
        (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
        (Result ResourceError FileMetadata) ExitCode
        (resourceMetadata AFull resource)
        (\used. match used {
          Err Closed |-> host_exit AFull Success;
          Err error |-> host_exit AFull (Failure 91);
          Ok metadata |-> host_exit AFull (Failure 92)
        });
    ResourceBracketBodyError error |-> host_exit AFull (Failure 93);
    ResourceBracketReleaseError error |-> host_exit AFull (Failure 94);
    ResourceBracketBodyAndReleaseError body_error release_error |->
      host_exit AFull (Failure 95)
  }

proc after_outer
  (outcome : Result FileError (ResourceBracketResult Unit (Resource ResourceKind.FsHandle)))
  : HostIO AFull ExitCode visits [FS] =
  match outcome {
    Err open_error |-> host_exit AFull (Failure 96);
    Ok bracket |-> after_escape bracket
  }

proc main (_input : ProcessInput) (caps : ProgramCaps AFull)
  : HostIO AFull ExitCode visits [FS] =
  match caps {
    MkProgramCaps cap |->
      bind (Coproduct (FSOp AFull) AmbientOp)
        (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
        (Result FileError (ResourceBracketResult Unit (Resource ResourceKind.FsHandle))) ExitCode
        (withResource AFull Unit (Resource ResourceKind.FsHandle)
          cap (bytes_encode "held.bin") ResourceMetadata
          (\resource. Ret (Coproduct (FSOp AFull) AmbientOp)
            (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
            (ResourceBodyResult Unit (Resource ResourceKind.FsHandle))
            (ResourceBodyOk Unit (Resource ResourceKind.FsHandle) resource)))
        (\outcome. after_outer outcome)
  }
"#;

const RIGHT_NOT_HELD: &str = r#"program capabilities FS AFull
fn metadata_after (outcome : Result ResourceError FileMetadata)
  : HostIO AFull (ResourceBodyResult ResourceError Unit) =
  match outcome {
    Err error |-> Ret (Coproduct (FSOp AFull) AmbientOp)
      (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
      (ResourceBodyResult ResourceError Unit)
      (ResourceBodyErr ResourceError Unit error);
    Ok metadata |-> Ret (Coproduct (FSOp AFull) AmbientOp)
      (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
      (ResourceBodyResult ResourceError Unit)
      (ResourceBodyOk ResourceError Unit MkUnit)
  }

proc metadata_body (resource : Resource ResourceKind.FsHandle)
  : HostIO AFull (ResourceBodyResult ResourceError Unit) visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError FileMetadata) (ResourceBodyResult ResourceError Unit)
    (resourceMetadata AFull resource) (\outcome. metadata_after outcome)

fn right_masks (error : ResourceError) : Bool =
  match error {
    ResourceHostIO io |-> False;
    Closed |-> False;
    MalformedResource |-> False;
    RightNotHeld required held |->
      match eq_int required 32 {
        True |-> eq_int held 1;
        False |-> False
      };
    ReleaseFailed kind identity io |-> False;
    ResourceKindMismatch expected actual |-> False;
    BufferLimit |-> False;
    AllocationFailed |-> False;
    InvalidOffset |-> False;
    InvalidBounds |-> False;
    NoProgress |-> False;
    MappingLimit |-> False
  }

fn bracket_has_right_denial (bracket : ResourceBracketResult ResourceError Unit) : Bool =
  match bracket {
    ResourceBracketOk unit |-> False;
    ResourceBracketBodyError error |-> right_masks error;
    ResourceBracketReleaseError error |-> False;
    ResourceBracketBodyAndReleaseError body_error release_error |-> False
  }

fn after_right_outer
  (outcome : Result FileError (ResourceBracketResult ResourceError Unit))
  : HostIO AFull ExitCode =
  match outcome {
    Err open_error |-> host_exit AFull (Failure 81);
    Ok bracket |-> match bracket_has_right_denial bracket {
      True |-> host_exit AFull Success;
      False |-> host_exit AFull (Failure 82)
    }
  }

proc main (_input : ProcessInput) (caps : ProgramCaps AFull)
  : HostIO AFull ExitCode visits [FS] =
  match caps {
    MkProgramCaps cap |->
      bind (Coproduct (FSOp AFull) AmbientOp)
        (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
        (Result FileError (ResourceBracketResult ResourceError Unit)) ExitCode
        (withResource AFull ResourceError Unit
          cap (bytes_encode "held.bin") ResourceRead metadata_body)
        (\outcome. after_right_outer outcome)
  }
"#;

const DOUBLE_RELEASE: &str = r#"program capabilities FS AFull
fn double_release_unexpected (error : ResourceError)
  : HostIO AFull (ResourceBodyResult ResourceError Unit) =
  Ret (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (ResourceBodyResult ResourceError Unit)
    (ResourceBodyErr ResourceError Unit error)

fn double_release_second_error (error : ResourceError)
  : HostIO AFull (ResourceBodyResult ResourceError Unit) =
  match error {
    Closed |-> Ret (Coproduct (FSOp AFull) AmbientOp)
      (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
      (ResourceBodyResult ResourceError Unit)
      (ResourceBodyOk ResourceError Unit MkUnit);
    ResourceHostIO io |-> Ret (Coproduct (FSOp AFull) AmbientOp)
      (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
      (ResourceBodyResult ResourceError Unit)
      (ResourceBodyErr ResourceError Unit (ResourceHostIO io));
    MalformedResource |-> Ret (Coproduct (FSOp AFull) AmbientOp)
      (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
      (ResourceBodyResult ResourceError Unit)
      (ResourceBodyErr ResourceError Unit MalformedResource);
    RightNotHeld required held |-> Ret (Coproduct (FSOp AFull) AmbientOp)
      (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
      (ResourceBodyResult ResourceError Unit)
      (ResourceBodyErr ResourceError Unit (RightNotHeld required held));
    ReleaseFailed kind identity io |-> Ret (Coproduct (FSOp AFull) AmbientOp)
      (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
      (ResourceBodyResult ResourceError Unit)
      (ResourceBodyErr ResourceError Unit (ReleaseFailed kind identity io));
    ResourceKindMismatch expected actual |->
      double_release_unexpected (ResourceKindMismatch expected actual);
    BufferLimit |-> double_release_unexpected BufferLimit;
    AllocationFailed |-> double_release_unexpected AllocationFailed;
    InvalidOffset |-> double_release_unexpected InvalidOffset;
    InvalidBounds |-> double_release_unexpected InvalidBounds;
    NoProgress |-> double_release_unexpected NoProgress;
    MappingLimit |-> double_release_unexpected MappingLimit
  }

fn double_release_after_second (outcome : Result ResourceError Unit)
  : HostIO AFull (ResourceBodyResult ResourceError Unit) =
  match outcome {
    Err error |-> double_release_second_error error;
    Ok unit |-> Ret (Coproduct (FSOp AFull) AmbientOp)
      (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
      (ResourceBodyResult ResourceError Unit)
      (ResourceBodyErr ResourceError Unit MalformedResource)
  }

proc double_release_after_first
  (resource : Resource ResourceKind.FsHandle) (first : Result ResourceError Unit)
  : HostIO AFull (ResourceBodyResult ResourceError Unit) visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError Unit) (ResourceBodyResult ResourceError Unit)
    (release AFull resource) (\second. double_release_after_second second)

proc double_release_body (resource : Resource ResourceKind.FsHandle)
  : HostIO AFull (ResourceBodyResult ResourceError Unit) visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError Unit) (ResourceBodyResult ResourceError Unit)
    (release AFull resource) (\first. double_release_after_first resource first)

fn double_release_done
  (outcome : Result FileError (ResourceBracketResult ResourceError Unit))
  : HostIO AFull ExitCode =
  match outcome {
    Ok (ResourceBracketOk unit) |-> host_exit AFull Success;
    Ok bracket |-> host_exit AFull (Failure 74);
    Err error |-> host_exit AFull (Failure 75)
  }

proc main (_input : ProcessInput) (caps : ProgramCaps AFull)
  : HostIO AFull ExitCode visits [FS] =
  match caps {
    MkProgramCaps cap |->
      bind (Coproduct (FSOp AFull) AmbientOp)
        (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
        (Result FileError (ResourceBracketResult ResourceError Unit)) ExitCode
        (withResource AFull ResourceError Unit cap (bytes_encode "held.bin")
          ResourceMetadata double_release_body)
        (\outcome. double_release_done outcome)
  }
"#;

#[cfg(target_os = "linux")]
// Readmitted under RT-IGNORED-PASSING-ROWS, row 1 of 11.
//
// The prior label read: "RT-CARRIED-RESOURCE-SCALAR: the FsHandleMetadata seat
// cannot observe a carried word as a resource scalar; fails at base 21fd46dc",
// on the signature "Effect: seat Argument(0) of FsHandleMetadata needs
// ResourceScalar, which it cannot observe in CarriedWord", and asserted that the
// row "refuses at object emission, so the program never executes". None of that
// still holds: the program emits, executes, and the row passes un-ignored.
//
// The readmission does NOT rest on that green. Mutating the stale-generation arm
// of ken-host effect_v1.rs fn lookup (Err(Closed) -> Err(RightNotHeld{0,0})) reds
// the row with the perturbation visible AT the assertion -- event 2 outcome
// Error(Resource(RightNotHeld{0,0})) where Closed is asserted, exit_status 91
// rather than 0. Two earlier mutations of resolve_fs_handle's state-match arms
// left it green because lookup's generation check returns before that match is
// reached; a green under mutation is "vacuous row" OR "mutation did not reach",
// and only the third mutation separated them.
#[test]
fn linked_public_escape_is_exact_closed() {
    let observation = run("escape-closed", ESCAPE_CLOSED);
    assert_eq!(observation.exit_status, 0, "{observation:?}");
    assert_eq!(
        observation
            .effect_trace
            .iter()
            .map(|event| event.operation)
            .collect::<Vec<_>>(),
        vec![
            ken_runtime::HostOpV1::FsOpen,
            ken_runtime::HostOpV1::ResourceRelease,
            ken_runtime::HostOpV1::FsHandleMetadata,
        ]
    );
    assert!(matches!(
        observation.effect_trace[2].outcome,
        ken_runtime::CanonicalOutcomeV1::Error(ken_runtime::SemanticErrorV1::Resource(
            ken_runtime::ResourceErrorV1::Closed
        ))
    ));
}

#[cfg(target_os = "linux")]
#[test]
fn bounded_epoch_refuses_before_the_checked_program_issues_an_effect() {
    use ken_runtime::boundary_resource_profile::starter_smoke_profile;
    for (limit, expected_trace_len) in [(0u64, 0usize), (1u64, 3usize)] {
        // Each executable launches in its own OS process. The positive limit
        // admits its first epoch; zero refuses the same checked program before
        // generated code can execute. The same wire decoder serves both.
        let dir = output_dir(&format!("epoch-profile-{limit}"));
        std::fs::write(dir.path().join("held.bin"), b"held resource").unwrap();
        let mut profile = starter_smoke_profile();
        profile.runtime.invocation_epochs = limit;
        let output = ken_cli::build_native_program(
            ESCAPE_CLOSED,
            ken_cli::SourceFormat::Ken,
            "epoch-profile",
            dir.path(),
            profile,
        )
        .expect("the explicit epoch profile packages with the checked source");
        let observation = ken_runtime::run_bound_process_effect_observation(
            &output.artifact,
            &ken_runtime::NativeEffectRunOptionsV1 {
                arguments: Vec::new(),
                environment: Vec::new(),
                cwd: dir.path().to_owned(),
                plan_hash: output.plan_transport_hash,
            },
        )
        .expect("the linked trace decodes");
        assert_eq!(observation.effect_trace.len(), expected_trace_len);
        if limit == 0 {
            assert_eq!(observation.exit_status, 1);
            let exact_failure = ken_runtime::CapacityExhaustedV1 {
                scope: ken_runtime::CapacityScopeV1::Runtime,
                resource: ken_runtime::CapacityResourceV1::InvocationEpochs,
                limit: 0,
                requested: 1,
            };
            assert_eq!(
                observation.terminal_error,
                Some(ken_runtime::TerminalErrorV1::CapacityExhausted(
                    exact_failure
                ),)
            );
            // The REAL linked path above is the positive control. A forged
            // linked trace with the same plan but a different negative token
            // or resource ceiling must not pass the profile-bound observer.
            use std::os::unix::fs::PermissionsExt;
            let fixture_path = dir.path().join("forged-trace");
            let script_path = dir.path().join("forged-starter.sh");
            std::fs::write(
                &script_path,
                format!(
                    "#!/bin/sh\ncp '{}' \"$KEN_HOST_OBSERVATION_PATH\"\nexit 1\n",
                    fixture_path.display(),
                ),
            )
            .unwrap();
            let mut perms = std::fs::metadata(&script_path).unwrap().permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&script_path, perms).unwrap();
            let mut forged = output.artifact.clone();
            forged.executable_path = script_path;
            for (token, fault) in [
                (-8i64, Some(exact_failure)),
                (
                    -7,
                    Some(ken_runtime::CapacityExhaustedV1 {
                        limit: 1,
                        requested: 2,
                        ..exact_failure
                    }),
                ),
                (
                    -7,
                    Some(ken_runtime::CapacityExhaustedV1 {
                        requested: 2,
                        ..exact_failure
                    }),
                ),
                (-7, None),
            ] {
                let trace = ken_runtime::LinkedEffectTrace {
                    plan_hash: output.plan_transport_hash,
                    target_abi_hash: ken_runtime::TARGET_ABI_MANIFEST_HASH,
                    host_effect_abi_hash: ken_runtime::HOST_EFFECT_ABI_V1_HASH,
                    terminal_value: -7,
                    terminal_error: fault.map(ken_runtime::TerminalErrorV1::CapacityExhausted),
                    effect_trace: Vec::new(),
                    terminal_exit: ken_runtime::TerminalExitClass::ControlledTrap,
                };
                let mut bytes = ken_runtime::encode_linked_effect_trace(&trace).unwrap();
                if token != -7 {
                    // Wire-format compatibility vector: magic(8), plan(8),
                    // target hash(32), host hash(32), then terminal i64.
                    bytes[80..88].copy_from_slice(&token.to_le_bytes());
                }
                std::fs::write(&fixture_path, bytes).unwrap();
                let refusal = ken_runtime::run_bound_process_effect_observation(
                    &forged,
                    &ken_runtime::NativeEffectRunOptionsV1 {
                        arguments: Vec::new(),
                        environment: Vec::new(),
                        cwd: dir.path().to_owned(),
                        plan_hash: output.plan_transport_hash,
                    },
                )
                .expect_err("forged terminal cannot borrow capacity authority");
                if fault.is_none() {
                    assert!(
                        matches!(
                            refusal,
                            ken_runtime::NativeEffectRunErrorV1::UnclassifiedRuntimeTrap {
                                terminal_value: -7
                            }
                        ),
                        "{refusal:?}"
                    );
                } else {
                    assert!(
                        matches!(refusal, ken_runtime::NativeEffectRunErrorV1::MalformedTrace),
                        "{refusal:?}"
                    );
                }
            }
        } else {
            assert_eq!(observation.exit_status, 0, "{observation:?}");
            assert!(observation.terminal_error.is_none());
            assert_eq!(
                observation.effect_trace[2].operation,
                ken_runtime::HostOpV1::FsHandleMetadata
            );
        }
    }
}

#[cfg(target_os = "linux")]
#[test]
// RT-SITEOP-CARRIED-WITNESS D1a/D2: FsReadFile Argument(0) was site-bound:
// FileError SiteOperand(0) could not project its carried word. D5 byte-span
// observation was not the blocker; D2 supplies the exact emitted-helper port.
#[ignore = "RT-PX7F-LINKED-PUBLIC-ROWS: require_i64(ret_tag, expected_ret) in define_static_response_owner_bodies rejects the response-K carrier because the planner expects the immediate Ret identity while the conforming grafted continuation returns Vis. This refusal is terminal here: RT-PLANNER-KRET-GRAFTED-SPINE is parked at structural stop 16 because preserving that Vis and its lexical K across the generated boundary has no lawful existing representation; no live node owns the next step and this node established that."]
fn linked_public_right_denial_preserves_exact_masks() {
    let observation = run("right-denial", RIGHT_NOT_HELD);
    assert_eq!(observation.exit_status, 0, "{observation:?}");
    assert!(observation.effect_trace.iter().any(|event| matches!(
        event.outcome,
        ken_runtime::CanonicalOutcomeV1::Error(ken_runtime::SemanticErrorV1::Resource(
            ken_runtime::ResourceErrorV1::RightNotHeld {
                required: 32,
                held: 1
            }
        ))
    )));
}

#[cfg(target_os = "linux")]
#[test]
// RT-SITEOP-CARRIED-WITNESS D1a/D2: FsReadFile Argument(0) was site-bound:
// FileError SiteOperand(0) could not project its carried word. D5 byte-span
// observation was not the blocker; D2 supplies the exact emitted-helper port.
#[ignore = "RT-PX7F-LINKED-PUBLIC-ROWS: require_i64(ret_tag, expected_ret) in define_static_response_owner_bodies rejects the response-K carrier because the planner expects the immediate Ret identity while the conforming grafted continuation returns Vis. This refusal is terminal here: RT-PLANNER-KRET-GRAFTED-SPINE is parked at structural stop 16 because preserving that Vis and its lexical K across the generated boundary has no lawful existing representation; no live node owns the next step and this node established that."]
fn linked_public_second_release_is_closed_and_the_handle_closes_once() {
    let observation = run("double-release", DOUBLE_RELEASE);
    assert_eq!(observation.exit_status, 0, "{observation:?}");
    assert_eq!(observation.terminal_error, None);
    let releases = observation
        .effect_trace
        .iter()
        .filter(|event| event.operation == ken_runtime::HostOpV1::ResourceRelease)
        .collect::<Vec<_>>();
    assert_eq!(
        releases.len(),
        3,
        "two public calls plus bracket settlement"
    );
    assert!(matches!(
        releases[0].outcome,
        ken_runtime::CanonicalOutcomeV1::Success(
            ken_runtime::CanonicalReplyV1::ResourceSettlement(_)
        )
    ));
    assert!(matches!(
        releases[1].outcome,
        ken_runtime::CanonicalOutcomeV1::Error(ken_runtime::SemanticErrorV1::Resource(
            ken_runtime::ResourceErrorV1::Closed
        ))
    ));
    assert_eq!(
        releases
            .iter()
            .filter(|event| matches!(
                event.outcome,
                ken_runtime::CanonicalOutcomeV1::Success(
                    ken_runtime::CanonicalReplyV1::ResourceSettlement(_)
                )
            ))
            .count(),
        1,
        "the owned descriptor is actually closed exactly once"
    );
}
