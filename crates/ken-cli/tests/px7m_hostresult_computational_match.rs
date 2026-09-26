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

// C4: native counterexample to the owner's checked Ret ingress. Without
// bypass, the same checked program above exits normally; here the test-only
// owner sends a Vis-tagged carrier through its unchecked Result slot. The
// emitted C2 branch must terminate when that carrier reaches the Match.
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
#[test]
fn owner_ret_check_bypass_reaches_the_inner_vis_trap_natively() {
    use std::os::unix::process::ExitStatusExt;
    let dir = output_dir("owner-vis-trap");
    let ((output, applications), emissions) =
        ken_runtime::with_selected_pending_match_emissions(|| {
            ken_runtime::with_static_response_owner_body_mutation(
                ken_runtime::StaticResponseOwnerBodyMutation::BypassRetValidationAndReturnVis,
                || ken_cli::build_native_program(
                    OK_PROGRAM, ken_cli::SourceFormat::Ken,
                    "px7m-owner-vis-bypass", dir.path(),
                    ken_runtime::boundary_resource_profile::starter_smoke_profile(),
                ),
            )
        });
    assert_eq!(applications, 1, "one admitted owner must carry the test-only bypass");
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
#[ignore = "RT-SELECTED-PENDING-CALL-BUILD increment 2 refuses this selected pending route at admission E: the deferred Effect has free Var(1) but its selected operation supplies one field. No native dynamic-error execution is claimed; the separate admission test pins the exact refusal reason."]
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

// Transition sentinel: if response-owner environment extension plus J-a
// accounting later admits this route, this reason pin intentionally reddens.
// It does not infer native execution from checked-source planning.
#[test]
fn dynamic_err_pending_route_refuses_missing_effect_binding_before_join_accounting() {
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
    assert_eq!(relevant, [ken_runtime::PendingRefusal::RelocatedWorkMissingLoweringBinding],
        "the checked ERR source must reach one pending producer, refused first by E: {admissions:#?}");
    assert!(!admissions.iter().any(|row| matches!(
        row.outcome,
        ken_runtime::SelectedPendingCallOutcomeObservation::ValidatedResponseOwner { .. }
    )), "one refused producer must not gain an owner-validated route: {admissions:#?}");
    assert!(result.is_err(), "a refused pending route cannot emit an artifact");
}
