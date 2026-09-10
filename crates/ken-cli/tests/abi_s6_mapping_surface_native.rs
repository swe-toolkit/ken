//! ABI-S6 D5a-surface: checked window-direct Mapping programs.
//!
//! Promise class: normative compatibility vectors from `38 §1.9` and
//! `conformance/surface/ffi-io/seed-mapping.md`. The programs cross the real
//! checked-source, native-object, host-dispatch, and interpreter boundaries.
//! File-backed copy-on-write remains blocked on D5b; this target exercises only
//! the frozen anonymous allocate/read/write operation set plus release.

#![cfg(target_os = "linux")]

const SOURCE: &str = r#"program capabilities FS AFull
const body_ok_io : HostIO AFull (ResourceBodyResult Unit Unit) =
  Ret (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (ResourceBodyResult Unit Unit)
    (ResourceBodyOk Unit Unit MkUnit)

const body_error_io : HostIO AFull (ResourceBodyResult Unit Unit) =
  Ret (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (ResourceBodyResult Unit Unit)
    (ResourceBodyErr Unit Unit MkUnit)

fn finish (outcome : Result ResourceError (ResourceBracketResult Unit Unit))
  : HostIO AFull ExitCode =
  match outcome {
    Err error |-> host_exit AFull (Failure 91);
    Ok bracket |-> match bracket {
      ResourceBracketOk value |-> host_exit AFull Success;
      ResourceBracketBodyError error |-> host_exit AFull (Failure 92);
      ResourceBracketReleaseError error |-> host_exit AFull (Failure 93);
      ResourceBracketBodyAndReleaseError body_error release_error |->
        host_exit AFull (Failure 94)
    }
  }

fn expect_bytes (outcome : Result ResourceError Bytes)
  : HostIO AFull (ResourceBodyResult Unit Unit) =
  match outcome {
    Err error |-> body_error_io;
    Ok bytes |-> body_ok_io
  }

proc read_body (mapping : MappingHandle)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError Bytes) (ResourceBodyResult Unit Unit)
    (mapBytes AFull mapping (MkMappingWindow (2 : Int) (4 : Int)))
    (\outcome. expect_bytes outcome)

proc read_stage (_cap : Cap AFull)
  : HostIO AFull ExitCode visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError (ResourceBracketResult Unit Unit)) ExitCode
    (withMapping AFull Unit Unit (Anonymous (8 : Int)) ReadOnly read_body)
    (\outcome. finish outcome)

proc after_write (mapping : MappingHandle)
  (outcome : Result ResourceError Unit)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  match outcome {
    Err error |-> body_error_io;
    Ok unit |-> bind (Coproduct (FSOp AFull) AmbientOp)
      (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
      (Result ResourceError Bytes) (ResourceBodyResult Unit Unit)
      (mapBytes AFull mapping (MkMappingWindow (2 : Int) (4 : Int)))
      (\bytes. expect_bytes bytes)
  }

proc write_read_body (mapping : MappingHandle)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError Unit) (ResourceBodyResult Unit Unit)
    (mapWrite AFull mapping (MkMappingWindow (2 : Int) (4 : Int))
      (bytes_encode "ABCD"))
    (\outcome. after_write mapping outcome)

proc write_read_stage (_cap : Cap AFull)
  : HostIO AFull ExitCode visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError (ResourceBracketResult Unit Unit)) ExitCode
    (withMapping AFull Unit Unit (Anonymous (8 : Int)) ReadWrite write_read_body)
    (\outcome. finish outcome)

fn expect_invalid_bounds (outcome : Result ResourceError Bytes)
  : HostIO AFull (ResourceBodyResult Unit Unit) =
  match outcome {
    Err InvalidBounds |-> body_ok_io;
    Err error |-> body_error_io;
    Ok bytes |-> body_error_io
  }

proc out_of_range_body (mapping : MappingHandle)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError Bytes) (ResourceBodyResult Unit Unit)
    (mapBytes AFull mapping (MkMappingWindow (6 : Int) (4 : Int)))
    (\outcome. expect_invalid_bounds outcome)

proc out_of_range_stage (_cap : Cap AFull)
  : HostIO AFull ExitCode visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError (ResourceBracketResult Unit Unit)) ExitCode
    (withMapping AFull Unit Unit (Anonymous (8 : Int)) ReadOnly out_of_range_body)
    (\outcome. finish outcome)

proc negative_window_body (mapping : MappingHandle)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError Bytes) (ResourceBodyResult Unit Unit)
    (mapBytes AFull mapping
      (MkMappingWindow (sub_int (0 : Int) (1 : Int)) (1 : Int)))
    (\outcome. expect_invalid_bounds outcome)

proc negative_window_stage (_cap : Cap AFull)
  : HostIO AFull ExitCode visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError (ResourceBracketResult Unit Unit)) ExitCode
    (withMapping AFull Unit Unit (Anonymous (8 : Int)) ReadOnly negative_window_body)
    (\outcome. finish outcome)

fn expect_read_only_refusal (outcome : Result ResourceError Unit)
  : HostIO AFull (ResourceBodyResult Unit Unit) =
  match outcome {
    Err (RightNotHeld required held) |-> body_ok_io;
    Err error |-> body_error_io;
    Ok unit |-> body_error_io
  }

proc read_only_write_body (mapping : MappingHandle)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError Unit) (ResourceBodyResult Unit Unit)
    (mapWrite AFull mapping (MkMappingWindow (0 : Int) (1 : Int))
      (bytes_encode "X"))
    (\outcome. expect_read_only_refusal outcome)

proc read_only_write_stage (_cap : Cap AFull)
  : HostIO AFull ExitCode visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError (ResourceBracketResult Unit Unit)) ExitCode
    (withMapping AFull Unit Unit (Anonymous (8 : Int)) ReadOnly read_only_write_body)
    (\outcome. finish outcome)

proc main (_input : ProcessInput) (caps : ProgramCaps AFull)
  : HostIO AFull ExitCode visits [FS] =
  match caps {
    MkProgramCaps cap |-> __ENTRY__ cap
  }
"#;

struct Differential {
    interpreted: ken_runtime::EffectObservation,
    native: ken_runtime::EffectObservation,
}

fn differential(case: &str, entry: &str) -> Differential {
    let root = tempfile::Builder::new()
        .prefix(&format!("ken-abi-s6-surface-{case}-"))
        .tempdir()
        .unwrap();
    let source = SOURCE.replace("__ENTRY__", entry);
    let output = ken_cli::build_native_program(
        &source,
        ken_cli::SourceFormat::Ken,
        &format!("abi_s6_surface_{}", case.replace('-', "_")),
        root.path(),
    )
    .unwrap_or_else(|error| {
        panic!("{case}: window-direct source reaches native lowering: {error:?}")
    });
    let native = ken_runtime::run_bound_process_effect_observation(
        &output.artifact,
        &ken_runtime::NativeEffectRunOptionsV1 {
            arguments: Vec::new(),
            environment: Vec::new(),
            cwd: root.path().to_owned(),
            plan_hash: output.plan_transport_hash,
        },
    )
    .unwrap_or_else(|error| panic!("{case}: linked artifact runs: {error:?}"));
    let mut host = ken_interp::PosixHost::new_at(root.path());
    let interpreted = ken_cli::run_program_effect_observation(
        &source,
        ken_cli::SourceFormat::Ken,
        &[],
        &[],
        root.path().as_os_str().as_encoded_bytes(),
        &mut host,
    )
    .unwrap_or_else(|error| panic!("{case}: source runs in interpreter: {error:?}"));
    Differential {
        interpreted,
        native,
    }
}

fn non_release_events(
    observation: &ken_runtime::EffectObservation,
) -> Vec<ken_runtime::EffectEvent> {
    observation
        .effect_trace
        .iter()
        .filter(|event| event.operation != ken_runtime::HostOpV1::ResourceRelease)
        .cloned()
        .collect()
}

fn release_set(observation: &ken_runtime::EffectObservation) -> Vec<String> {
    let mut releases = observation
        .effect_trace
        .iter()
        .filter(|event| event.operation == ken_runtime::HostOpV1::ResourceRelease)
        .map(|event| {
            format!(
                "{:?}",
                (
                    event.resource_bindings.clone(),
                    event.request.clone(),
                    event.outcome.clone(),
                )
            )
        })
        .collect::<Vec<_>>();
    releases.sort();
    releases
}

fn operation_events(
    observation: &ken_runtime::EffectObservation,
    operation: ken_runtime::HostOpV1,
) -> Vec<ken_runtime::EffectEvent> {
    observation
        .effect_trace
        .iter()
        .filter(|event| event.operation == operation)
        .cloned()
        .collect()
}

fn assert_parity(case: &str, result: &Differential) {
    assert_eq!(
        (result.native.exit_status, result.interpreted.exit_status),
        (0, 0),
        "{case}: both engines must take the fixture's exact success branch"
    );
    assert_eq!(result.native.terminal_error, None, "{case}: native");
    assert_eq!(
        result.interpreted.terminal_error, None,
        "{case}: interpreter"
    );
    assert_eq!(
        non_release_events(&result.native),
        non_release_events(&result.interpreted),
        "{case}: ordered non-release effects, requests, outcomes, and bindings"
    );
    assert_eq!(
        release_set(&result.native),
        release_set(&result.interpreted),
        "{case}: release set parity; relative release order is excluded"
    );
    assert_eq!(
        result.native.terminal_exit, result.interpreted.terminal_exit,
        "{case}: terminal exit class"
    );
}

/// Promise class: durable behavioral invariant. MEASURED: an honest checked
/// `mapBytes mapping window` executes in both engines, returns four copied zero
/// bytes from exactly [2,6), and releases once. CLAIMED: the public window-direct
/// read composes over the frozen MappingReadView wire. THE GAP: file acquisition
/// is not exercised and remains D5b.
#[test]
fn window_direct_map_bytes_executes_natively_and_matches_the_interpreter() {
    let result = differential("read", "read_stage");
    assert_parity("read", &result);
    let reads = operation_events(&result.native, ken_runtime::HostOpV1::MappingReadView);
    assert_eq!(reads.len(), 1);
    assert_eq!(
        reads[0].request,
        ken_runtime::CanonicalRequestV1::MappingReadView {
            start: 2,
            length: 4,
        }
    );
    assert!(matches!(
        &reads[0].outcome,
        ken_runtime::CanonicalOutcomeV1::Success(ken_runtime::CanonicalReplyV1::Bytes(bytes))
            if bytes == &[0, 0, 0, 0]
    ));
    assert_eq!(release_set(&result.native).len(), 1);
}

/// Promise class: durable behavioral invariant. MEASURED: one checked write is
/// followed by one checked read of the same immutable window; both engines
/// observe `ABCD`, the ordered effect trace agrees, and release occurs once.
/// CLAIMED: sequential window-direct access composes without a Bytes-to-span
/// response transform. THE GAP: MAP_PRIVATE file isolation remains D5b.
#[test]
fn window_direct_map_write_then_read_executes_and_preserves_process_local_bytes() {
    let result = differential("write-read", "write_read_stage");
    assert_parity("write-read", &result);
    let writes = operation_events(&result.native, ken_runtime::HostOpV1::MappingWriteView);
    assert_eq!(writes.len(), 1);
    assert_eq!(
        writes[0].request,
        ken_runtime::CanonicalRequestV1::MappingWriteView {
            start: 2,
            bytes: b"ABCD".to_vec(),
        }
    );
    assert!(matches!(
        writes[0].outcome,
        ken_runtime::CanonicalOutcomeV1::Success(ken_runtime::CanonicalReplyV1::Unit)
    ));
    let reads = operation_events(&result.native, ken_runtime::HostOpV1::MappingReadView);
    assert_eq!(reads.len(), 1);
    assert!(matches!(
        &reads[0].outcome,
        ken_runtime::CanonicalOutcomeV1::Success(ken_runtime::CanonicalReplyV1::Bytes(bytes))
            if bytes.as_slice() == b"ABCD"
    ));
    assert_eq!(release_set(&result.native).len(), 1);
}

/// Promise class: normative compatibility vector. MEASURED: [6,10) against an
/// eight-byte mapping enters the real MappingReadView dispatch and returns the
/// exact InvalidBounds variant in both engines, never a clamped byte result.
/// CLAIMED: positive but out-of-range windows fail visibly. THE GAP: negative
/// scalar narrowing is pinned separately below.
#[test]
fn out_of_range_window_is_invalid_bounds_not_clamped() {
    let result = differential("out-of-range", "out_of_range_stage");
    assert_parity("out-of-range", &result);
    let reads = operation_events(&result.native, ken_runtime::HostOpV1::MappingReadView);
    assert_eq!(reads.len(), 1);
    assert!(matches!(
        reads[0].outcome,
        ken_runtime::CanonicalOutcomeV1::Error(ken_runtime::SemanticErrorV1::Resource(
            ken_runtime::ResourceErrorV1::InvalidBounds
        ))
    ));
    assert_eq!(release_set(&result.native).len(), 1);
}

/// Promise class: durable narrowing invariant. MEASURED: a negative window
/// offset reaches the checked helper but never host dispatch; both engines take
/// the exact InvalidBounds branch, record no MappingReadView event, and release
/// once. CLAIMED: malformed scalar windows are rejected before wire marshalling.
/// THE GAP: positive out-of-range bounds are pinned by the sibling above.
#[test]
fn negative_window_skips_mapping_read_dispatch_in_both_engines() {
    let result = differential("negative-window", "negative_window_stage");
    assert_parity("negative-window", &result);
    assert!(operation_events(&result.native, ken_runtime::HostOpV1::MappingReadView).is_empty());
    assert!(
        operation_events(&result.interpreted, ken_runtime::HostOpV1::MappingReadView).is_empty()
    );
    assert_eq!(release_set(&result.native).len(), 1);
}

/// Promise class: normative compatibility vector. MEASURED: a checked mapWrite
/// through a ReadOnly handle enters the real write operation and returns the
/// exact WRITE-vs-READ RightNotHeld masks in both engines. CLAIMED: ReadOnly
/// mappings refuse writes without mutating bytes. THE GAP: the unchanged byte
/// content is independently covered by the host resource-table test.
#[test]
fn read_only_mapping_refuses_window_write_with_exact_rights() {
    let result = differential("read-only-write", "read_only_write_stage");
    assert_parity("read-only-write", &result);
    let writes = operation_events(&result.native, ken_runtime::HostOpV1::MappingWriteView);
    assert_eq!(writes.len(), 1);
    assert!(matches!(
        writes[0].outcome,
        ken_runtime::CanonicalOutcomeV1::Error(ken_runtime::SemanticErrorV1::Resource(
            ken_runtime::ResourceErrorV1::RightNotHeld {
                required: 2,
                held: 1,
            }
        ))
    ));
    assert_eq!(release_set(&result.native).len(), 1);
}
