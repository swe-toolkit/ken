fn output_dir(name: &str) -> tempfile::TempDir {
    let prefix = format!("ken-px7m-{name}-");
    tempfile::Builder::new().prefix(&prefix).tempdir().unwrap()
}

const OK_PROGRAM: &str = r#"program capabilities FS APartial
proc two_step (label : String) : HostIO APartial Unit visits [Console] =
  bind (Coproduct (FSOp APartial) AmbientOp)
    (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
    Unit Unit
    (host_console APartial Unit (print_line label))
    (\_. bind (Coproduct (FSOp APartial) AmbientOp)
      (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
      (Result IOError Unit) Unit
      (host_console APartial (Result IOError Unit) (flush Stdout))
      (\_. Ret (Coproduct (FSOp APartial) AmbientOp)
        (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
        Unit MkUnit))

proc after_write (written : Result IOError Unit)
  : HostIO APartial Unit visits [Console] =
  match written {
    Err _ |-> two_step "unexpected-error" ;
    Ok unit |-> match unit { MkUnit |-> two_step "ok-payload" }
  }

proc inner : HostIO APartial Unit visits [Console] =
  bind (Coproduct (FSOp APartial) AmbientOp)
    (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
    (Result IOError Unit) Unit
    (host_console APartial (Result IOError Unit)
      (write Stdout (bytes_encode "probe:")))
    after_write

proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode visits [Console] =
  bind (Coproduct (FSOp APartial) AmbientOp)
    (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
    Unit ExitCode inner (\_. host_exit APartial Success)
"#;

const ERR_PROGRAM: &str = r#"program capabilities FS APartial
proc write_bytes_then_line (bytes : Bytes) (label : String)
  : HostIO APartial Unit visits [Console] =
  bind (Coproduct (FSOp APartial) AmbientOp)
    (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
    (Result IOError Unit) Unit
    (host_console APartial (Result IOError Unit) (write Stdout bytes))
    (\_. bind (Coproduct (FSOp APartial) AmbientOp)
      (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
      Unit Unit
      (host_console APartial Unit (print_line label))
      (\_. Ret (Coproduct (FSOp APartial) AmbientOp)
        (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
        Unit MkUnit))

fn failed_path (error : FileError) : Bytes =
  match error {
    MkFileError _operation path _kind |-> match path {
      None |-> bytes_encode "no-path" ;
      Some bytes |-> bytes
    }
  }

proc after_read (read : Result FileError Bytes)
  : HostIO APartial Unit visits [Console] =
  match read {
    Err error |-> write_bytes_then_line (failed_path error) "not-found" ;
    Ok bytes |-> write_bytes_then_line bytes "unexpected-ok"
  }

proc inner (cap : Cap APartial) : HostIO APartial Unit visits [FS, Console] =
  bind (Coproduct (FSOp APartial) AmbientOp)
    (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
    (Result FileError Bytes) Unit
    (inject_l (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp
      (Result FileError Bytes)
      (readFile APartial cap (bytes_encode "missing.bin")))
    after_read

proc main (_input : ProcessInput) (caps : ProgramCaps APartial)
  : HostIO APartial ExitCode visits [FS, Console] =
  match caps {
    MkProgramCaps cap |->
      bind (Coproduct (FSOp APartial) AmbientOp)
        (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
        Unit ExitCode (inner cap) (\_. host_exit APartial Success)
  }
"#;

fn assert_agreement(
    source: &str,
    name: &str,
    expected_stdout: &[u8],
    expected_operations: &[ken_runtime::HostOpV1],
) {
    let dir = output_dir(name);
    let ((output, admissions), match_emissions) =
        ken_runtime::with_selected_pending_match_emissions(|| {
            ken_runtime::with_selected_pending_call_admissions(|| {
                ken_cli::build_native_program(source, ken_cli::SourceFormat::Ken, name, dir.path(), ken_runtime::boundary_resource_profile::starter_smoke_profile())
            })
        });
    let output = output.expect("dynamic HostResult producer reaches the linked artifact");
    assert!(admissions.iter().any(|row| matches!(
        row.outcome,
        ken_runtime::SelectedPendingCallOutcomeObservation::ValidatedResponseOwner { .. }
    )), "native selected route needs a validated response-owner admission: {admissions:#?}");
    // P4a: the same Match origin has two distinct emitted populations. The
    // owner's Vis branch is a trap, whereas each nested ordinary continuation
    // lowers its Vis body; one aggregate count cannot prove the pairing.
    let trapped_origins = match_emissions.iter().filter(|row|
        row.kind == ken_runtime::SelectedPendingMatchEmissionKind::ValidatedOwnerVisTrap
            && row.emission_owner.starts_with("Predeclared(")
    ).map(|row| row.origin).collect::<std::collections::BTreeSet<_>>();
    assert_eq!(trapped_origins.len(), 1,
        "one owner-fed origin must emit the trap: {match_emissions:#?}");
    let trapped_origin = *trapped_origins.iter().next().unwrap();
    let ordinary_owners = match_emissions.iter().filter(|row|
        row.origin == trapped_origin
            && row.kind == ken_runtime::SelectedPendingMatchEmissionKind::OrdinaryVisBodyLowered
            && row.emission_owner.starts_with("Specialization(")
    ).map(|row| row.emission_owner.as_str()).collect::<std::collections::BTreeSet<_>>();
    assert_eq!(ordinary_owners.len(), 2,
        "both ordinary specialization copies must lower the Vis body for the trapped origin: {match_emissions:#?}");
    assert!(!match_emissions.iter().any(|row| row.origin == trapped_origin &&
        (row.kind == ken_runtime::SelectedPendingMatchEmissionKind::ValidatedOwnerVisTrap
            && row.emission_owner.starts_with("Specialization(")
         || row.kind == ken_runtime::SelectedPendingMatchEmissionKind::OrdinaryVisBodyLowered
            && row.emission_owner.starts_with("Predeclared("))),
        "the owner and ordinary outcomes must not swap: {match_emissions:#?}");
    let native = ken_runtime::run_bound_process_effect_observation(
        &output.artifact,
        &ken_runtime::NativeEffectRunOptionsV1 {
            arguments: Vec::new(),
            environment: Vec::new(),
            cwd: dir.path().to_owned(),
            plan_hash: output.plan_transport_hash,
        },
    )
    .expect("linked artifact returns its complete observation");

    let mut host = ken_interp::CaptureHost::new(Vec::new());
    let interpreted = ken_cli::run_program_effect_observation(
        source,
        ken_cli::SourceFormat::Ken,
        &[b"ken".to_vec()],
        &[],
        b"/",
        &mut host,
    )
    .expect("same checked source runs through the interpreter");
    assert_eq!(native, interpreted);
    assert_eq!(native.exit_status, 0);
    assert_eq!(native.stdout, expected_stdout);
    assert_eq!(
        native
            .effect_trace
            .iter()
            .map(|event| event.operation)
            .collect::<Vec<_>>(),
        expected_operations
    );
}

// Owner node: RT-CARRIED-RESIDUAL-IH-ARITY.
//
// Observed signature, exactly, re-measured 2026-09-17 at origin/main
// 3f4ae2d83 (NOT carried from the ledger):
//   unsupported runtime-IR lowering: BoundaryCarrier: a carried recursive
//   hypothesis is an eliminated value, not a callable, so it takes no
//   arguments, but the call provides 1
//
// SUPERSEDED OWNER, recorded because it is what this row was filed under and
// the old text asserted a signature this row no longer produces:
// RT-CARRIER-BYTESPAN-OBSERVE, whose signature was
//   Effect: seat Argument(0) of FsReadFile needs BytesPointerLength, which it
//   cannot observe in CarriedWord
// That refusal no longer reaches this row -- RT-SITEOP-CARRIED-WITNESS D2
// landed the carried SiteOperand port and the labels record it succeeding.
// The four px4b rows still carry RT-CARRIER-BYTESPAN-OBSERVE, with the
// OPPOSITE provenance: those were branch-introduced, this one predates the
// branch.
//
// Pre-existing base debt, NOT a bind-order regression: measured failing at
// the frozen base 21fd46dc by the D10 differential, before any
// RT-SRCBODY-BIND-ORDER commit.
// It refuses at object emission, so the program never executes and no
// binding order is observable in it.
// Annotation only -- test body and expectations are unchanged.
#[test]
// RT-SITEOP-CARRIED-WITNESS D1a/D2: FsReadFile Argument(0) was site-bound:
// FileError SiteOperand(0) could not project its carried word. D5 byte-span
// observation was not the blocker; D2 supplies the exact emitted-helper port.
// The selected return is validated Ret by its static response owner; no
// pending-call package is issued on this native path.
fn dynamic_ok_payload_selects_a_multistep_tree_across_real_executors() {
    assert_agreement(
        OK_PROGRAM,
        "px7m-ok",
        b"probe:ok-payload\n",
        &[
            ken_runtime::HostOpV1::ConsoleWrite,
            ken_runtime::HostOpV1::ConsoleWrite,
            ken_runtime::HostOpV1::ConsoleFlush,
        ],
    );
}

// This source-only flip changes which outer after_write arm names the observed
// label; it does not turn the inner Vis into a native owner return. Both
// executors must report the changed output with the same effect sequence.
#[test]
fn flipped_outer_arm_label_preserves_the_inner_owner_contract() {
    let flipped = OK_PROGRAM
        .replace("two_step \"unexpected-error\"", "two_step \"flipped-ok\"")
        .replace("two_step \"ok-payload\"", "two_step \"flipped-err\"");
    assert_agreement(
        &flipped,
        "px7m-flipped-outer-arm",
        b"probe:flipped-err\n",
        &[
            ken_runtime::HostOpV1::ConsoleWrite,
            ken_runtime::HostOpV1::ConsoleWrite,
            ken_runtime::HostOpV1::ConsoleFlush,
        ],
    );
}

// C4: a native counterexample to the owner-fed C2 Match. The first test-only
// hook admits a Vis past the entire finished-body layer. The second changes
// its member to an out-of-set discriminant and bypasses only the loop's
// unknown-member exit, depositing that non-member in the Result slot. C2 must
// trap at the inner Match rather than merely emit an unreachable trap branch.
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
#[test]
fn owner_nonmember_c2_ingress_reaches_inner_vis_trap_natively() {
    use std::os::unix::process::ExitStatusExt;
    let dir = output_dir("owner-vis-trap");
    let (((output, applications), emissions), nonmember_applications) =
        ken_runtime::with_pending_vis_nonmember_mutation(
            ken_runtime::PendingVisNonmemberMutation::BypassToC2, || {
                ken_runtime::with_selected_pending_match_emissions(|| {
                    ken_runtime::with_static_response_owner_body_mutation(
                        ken_runtime::StaticResponseOwnerBodyMutation::BypassRetValidationAndReturnVis,
                        || ken_cli::build_native_program(
                            OK_PROGRAM, ken_cli::SourceFormat::Ken,
                            "px7m-owner-vis-bypass", dir.path(),
                            ken_runtime::boundary_resource_profile::starter_smoke_profile(),
                        ),
                    )
                })
            },
        );
    assert_eq!(applications, 1, "one owner must bypass finished-body verification");
    assert_eq!(nonmember_applications, 1,
        "one protocol owner must admit the distinct out-of-set C2 ingress");
    let output = output.expect("the bypass builds a native artifact before it is run");
    assert!(emissions.iter().any(|row|
        row.kind == ken_runtime::SelectedPendingMatchEmissionKind::ValidatedOwnerVisTrap),
        "the object must emit the owner-fed inner Vis trap before native execution: {emissions:#?}");
    let native = std::process::Command::new(&output.artifact.executable_path)
        .current_dir(dir.path())
        .env_clear()
        .env("KEN_HOST_OBSERVATION_PATH", dir.path().join("bypass-trace"))
        .output().expect("the linked executable starts");
    assert_eq!(native.status.signal(), Some(4),
        "the bypassed owner must trap in the native executable, not refuse at admission or emission; status={:?} stdout={:?} stderr={:?}",
        native.status, native.stdout, native.stderr);
}

/// Promise class: durable invariant. The injected Vis is a lawful returned
/// member; bypassing finished-body verification alone must make the owner
/// drive the next host effect, not confuse an emitted C2 trap with a reached
/// one. The distinct fourth Flush is absent from the unmutated run.
#[cfg(target_os = "linux")]
#[test]
fn owner_vis_verifier_bypass_drives_a_returned_member() {
    let run = |label: &str, mutated: bool| {
        let dir = output_dir(label);
        let build = || ken_cli::build_native_program(
            OK_PROGRAM, ken_cli::SourceFormat::Ken, "px7m-owner-loop-member",
            dir.path(), ken_runtime::boundary_resource_profile::starter_smoke_profile(),
        );
        let output = if mutated {
            let (result, applications) = ken_runtime::with_static_response_owner_body_mutation(
                ken_runtime::StaticResponseOwnerBodyMutation::BypassRetValidationAndReturnVis,
                build,
            );
            assert_eq!(applications, 1, "the finished-body bypass must reach one owner");
            result
        } else { build() }.expect("both member configurations compile");
        ken_runtime::run_bound_process_effect_observation(
            &output.artifact, &ken_runtime::NativeEffectRunOptionsV1 {
                arguments: Vec::new(), environment: Vec::new(),
                cwd: dir.path().to_owned(), plan_hash: output.plan_transport_hash,
            },
        ).expect("the member is driven to a native terminal result")
    };
    let baseline = run("px7m-member-baseline", false);
    let injected = run("px7m-member-injected", true);
    let operations = |observation: &ken_runtime::EffectObservation| observation.effect_trace
        .iter().map(|event| event.operation).collect::<Vec<_>>();
    assert_eq!(operations(&baseline), [
        ken_runtime::HostOpV1::ConsoleWrite,
        ken_runtime::HostOpV1::ConsoleWrite,
        ken_runtime::HostOpV1::ConsoleFlush,
    ]);
    assert_eq!(operations(&injected), [
        ken_runtime::HostOpV1::ConsoleWrite,
        ken_runtime::HostOpV1::ConsoleWrite,
        ken_runtime::HostOpV1::ConsoleFlush,
        ken_runtime::HostOpV1::ConsoleFlush,
    ], "the injected member must cause a distinct owner-loop dispatch");
    assert_eq!(injected.stdout, baseline.stdout);
    assert_eq!(injected.exit_status, baseline.exit_status);
}

/// Promise class: durable invariant. A non-member discriminant cannot select
/// a successor or escape to C2 while the owner loop's unknown-member exit is
/// intact. This pairs the same invalid input with the C2-bypass test above.
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
#[test]
fn owner_nonmember_refuses_at_the_loop_before_c2() {
    use std::os::unix::process::ExitStatusExt;
    let dir = output_dir("px7m-unknown-member-refusal");
    let ((output, bypass_applications), nonmember_applications) =
        ken_runtime::with_pending_vis_nonmember_mutation(
            ken_runtime::PendingVisNonmemberMutation::Refuse, || {
                ken_runtime::with_static_response_owner_body_mutation(
                    ken_runtime::StaticResponseOwnerBodyMutation::BypassRetValidationAndReturnVis,
                    || ken_cli::build_native_program(
                        OK_PROGRAM, ken_cli::SourceFormat::Ken,
                        "px7m-unknown-member-refusal", dir.path(),
                        ken_runtime::boundary_resource_profile::starter_smoke_profile(),
                    ),
                )
            },
        );
    assert_eq!(bypass_applications, 1, "one owner must bypass the finished-body layer");
    assert_eq!(nonmember_applications, 1, "one owner must receive the invalid member");
    let output = output.expect("the non-member object builds before its native refusal");
    let native = std::process::Command::new(&output.artifact.executable_path)
        .current_dir(dir.path()).env_clear()
        .env("KEN_HOST_OBSERVATION_PATH", dir.path().join("nonmember-trace"))
        .output().expect("the native non-member executable starts");
    assert_eq!(native.status.code(), Some(1),
        "the unknown member must fail by status, not SIGILL: {native:?}");
    assert!(native.status.signal().is_none(),
        "the loop refused before the inner C2 trap: {native:?}");
    let error = ken_runtime::run_bound_process_effect_observation(
        &output.artifact, &ken_runtime::NativeEffectRunOptionsV1 {
            arguments: Vec::new(), environment: Vec::new(),
            cwd: dir.path().to_owned(), plan_hash: output.plan_transport_hash,
        },
    ).expect_err("the unknown member returns a native failure");
    assert!(format!("{error:?}").contains("UnclassifiedRuntimeTrap { terminal_value: -1 }"),
        "the unknown-member exit must propagate its exact -1: {error:?}");
}

// Owner node: RT-CARRIED-RESIDUAL-IH-ARITY.
//
// Observed signature, exactly, re-measured 2026-09-17 at origin/main
// 3f4ae2d83 (NOT carried from the ledger):
//   unsupported runtime-IR lowering: BoundaryCarrier: a carried recursive
//   hypothesis is an eliminated value, not a callable, so it takes no
//   arguments, but the call provides 1
//
// SUPERSEDED OWNER, recorded because it is what this row was filed under and
// the old text asserted a signature this row no longer produces:
// RT-CARRIER-BYTESPAN-OBSERVE, whose signature was
//   Effect: seat Argument(0) of FsReadFile needs BytesPointerLength, which it
//   cannot observe in CarriedWord
// That refusal no longer reaches this row -- RT-SITEOP-CARRIED-WITNESS D2
// landed the carried SiteOperand port and the labels record it succeeding.
// The four px4b rows still carry RT-CARRIER-BYTESPAN-OBSERVE, with the
// OPPOSITE provenance: those were branch-introduced, this one predates the
// branch.
//
// Pre-existing base debt, NOT a bind-order regression: measured failing at
// the frozen base 21fd46dc by the D10 differential, before any
// RT-SRCBODY-BIND-ORDER commit.
// It refuses at object emission, so the program never executes and no
// binding order is observable in it.
// Annotation only -- test body and expectations are unchanged.
#[test]
// RT-SITEOP-CARRIED-WITNESS D1a/D2: FsReadFile Argument(0) was site-bound:
// FileError SiteOperand(0) could not project its carried word. D5 byte-span
// observation was not the blocker; D2 supplies the exact emitted-helper port.
#[ignore = "RT-SELECTED-PENDING-CALL-BUILD: the selected ERR route refuses at admission J (SelectedPendingLeafRelocatesUnaccountedJoins). No native dynamic-error execution is claimed; the separate admission test pins the exact refusal reason."]
fn dynamic_err_payload_selects_a_multistep_tree_across_real_executors() {
    assert_agreement(
        ERR_PROGRAM,
        "px7m-err",
        b"missing.binnot-found\n",
        &[
            ken_runtime::HostOpV1::FsReadFile,
            ken_runtime::HostOpV1::ConsoleWrite,
            ken_runtime::HostOpV1::ConsoleWrite,
        ],
    );
}

// Transition sentinel: the selected ERR route still relocates unaccounted
// joins. If J-a accounting later admits it, this exact-refusal pin reddens;
// checked-source planning alone does not establish native execution.
#[test]
fn dynamic_err_pending_route_refuses_unaccounted_relocated_joins() {
    let dir = output_dir("err-admission");
    let (result, admissions) = ken_runtime::with_selected_pending_call_admissions(|| {
        ken_cli::build_native_program(
            ERR_PROGRAM,
            ken_cli::SourceFormat::Ken,
            "px7m-err-admission",
            dir.path(),
            ken_runtime::boundary_resource_profile::starter_smoke_profile(),
        )
    });
    let relevant: Vec<_> = admissions.iter().filter_map(|row| match row.outcome {
        ken_runtime::SelectedPendingCallOutcomeObservation::Refused(reason) => Some(reason),
        _ => None,
    }).collect();
    assert_eq!(relevant, [ken_runtime::PendingRefusal::SelectedPendingLeafRelocatesUnaccountedJoins],
        "the checked ERR source must reach J's unaccounted-join refusal: {admissions:#?}");
    assert!(!admissions.iter().any(|row| matches!(
        row.outcome,
        ken_runtime::SelectedPendingCallOutcomeObservation::ValidatedResponseOwner { .. }
    )), "one refused producer must not gain an owner-validated route: {admissions:#?}");
    let error = result.expect_err("a refused pending route cannot emit an artifact");
    let text = error.to_string();
    assert!(text.contains("unsupported runtime-IR lowering: PendingCallAdmission: refused pending call: SelectedPendingLeafRelocatesUnaccountedJoins"),
        "the user must see the admission-J reason rather than a compiler ICE: {text}");
    assert!(!text.contains("planner invariant") && !text.contains("compiler bug"),
        "a classified admission refusal is not a compiler ICE: {text}");
}
