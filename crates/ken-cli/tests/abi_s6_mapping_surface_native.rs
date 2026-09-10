//! ABI-S6 D5a-surface: checked read-window/write-payload Mapping programs.
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

fn expect_unit (outcome : Result ResourceError Unit)
  : HostIO AFull (ResourceBodyResult Unit Unit) =
  match outcome {
    Err error |-> body_error_io;
    Ok unit |-> body_ok_io
  }

proc map_bytes_window (mapping : MappingHandle) (start : Int) (length : Int)
  : HostIO AFull (Result ResourceError Bytes) visits [FS] =
  mapBytes AFull mapping (MkMappingWindow start length)

proc read_body (mapping : MappingHandle)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError Bytes) (ResourceBodyResult Unit Unit)
    (map_bytes_window mapping (2 : Int) (4 : Int))
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
    (mapWrite AFull mapping (2 : Int) (bytes_encode "ABCD"))
    (\outcome. after_write mapping outcome)

proc write_read_stage (_cap : Cap AFull)
  : HostIO AFull ExitCode visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError (ResourceBracketResult Unit Unit)) ExitCode
    (withMapping AFull Unit Unit (Anonymous (8 : Int)) ReadWrite write_read_body)
    (\outcome. finish outcome)

proc after_read (mapping : MappingHandle)
  (outcome : Result ResourceError Bytes)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  match outcome {
    Err error |-> body_error_io;
    Ok bytes |-> bind (Coproduct (FSOp AFull) AmbientOp)
      (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
      (Result ResourceError Unit) (ResourceBodyResult Unit Unit)
      (mapWrite AFull mapping (2 : Int) (bytes_encode "ABCD"))
      (\written. expect_unit written)
  }

proc read_write_body (mapping : MappingHandle)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError Bytes) (ResourceBodyResult Unit Unit)
    (map_bytes_window mapping (2 : Int) (4 : Int))
    (\outcome. after_read mapping outcome)

proc read_write_stage (_cap : Cap AFull)
  : HostIO AFull ExitCode visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError (ResourceBracketResult Unit Unit)) ExitCode
    (withMapping AFull Unit Unit (Anonymous (8 : Int)) ReadWrite read_write_body)
    (\outcome. finish outcome)

proc after_read_to_read (mapping : MappingHandle)
  (outcome : Result ResourceError Bytes)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  match outcome {
    Err error |-> body_error_io;
    Ok bytes |-> bind (Coproduct (FSOp AFull) AmbientOp)
      (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
      (Result ResourceError Bytes) (ResourceBodyResult Unit Unit)
      (mapBytes AFull mapping (MkMappingWindow (2 : Int) (4 : Int)))
      (\read. expect_bytes read)
  }

proc read_read_body (mapping : MappingHandle)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError Bytes) (ResourceBodyResult Unit Unit)
    (map_bytes_window mapping (2 : Int) (4 : Int))
    (\outcome. after_read_to_read mapping outcome)

proc after_write_to_write (mapping : MappingHandle)
  (outcome : Result ResourceError Unit)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  match outcome {
    Err error |-> body_error_io;
    Ok unit |-> bind (Coproduct (FSOp AFull) AmbientOp)
      (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
      (Result ResourceError Unit) (ResourceBodyResult Unit Unit)
      (mapWrite AFull mapping (2 : Int) (bytes_encode "EFGH"))
      (\written. expect_unit written)
  }

proc write_write_body (mapping : MappingHandle)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError Unit) (ResourceBodyResult Unit Unit)
    (mapWrite AFull mapping (2 : Int) (bytes_encode "ABCD"))
    (\outcome. after_write_to_write mapping outcome)

proc after_second_write_to_read (mapping : MappingHandle)
  (outcome : Result ResourceError Unit)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  match outcome {
    Err error |-> body_error_io;
    Ok unit |-> bind (Coproduct (FSOp AFull) AmbientOp)
      (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
      (Result ResourceError Bytes) (ResourceBodyResult Unit Unit)
      (mapBytes AFull mapping (MkMappingWindow (2 : Int) (4 : Int)))
      (\read. expect_bytes read)
  }

proc after_read_to_write_read (mapping : MappingHandle)
  (outcome : Result ResourceError Bytes)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  match outcome {
    Err error |-> body_error_io;
    Ok bytes |-> bind (Coproduct (FSOp AFull) AmbientOp)
      (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
      (Result ResourceError Unit) (ResourceBodyResult Unit Unit)
      (mapWrite AFull mapping (2 : Int) (bytes_encode "ABCD"))
      (\written. after_second_write_to_read mapping written)
  }

proc read_write_read_body (mapping : MappingHandle)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError Bytes) (ResourceBodyResult Unit Unit)
    (map_bytes_window mapping (2 : Int) (4 : Int))
    (\outcome. after_read_to_write_read mapping outcome)

proc after_second_read_to_write (mapping : MappingHandle)
  (outcome : Result ResourceError Bytes)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  match outcome {
    Err error |-> body_error_io;
    Ok bytes |-> bind (Coproduct (FSOp AFull) AmbientOp)
      (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
      (Result ResourceError Unit) (ResourceBodyResult Unit Unit)
      (mapWrite AFull mapping (2 : Int) (bytes_encode "EFGH"))
      (\written. expect_unit written)
  }

proc after_write_to_read_write (mapping : MappingHandle)
  (outcome : Result ResourceError Unit)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  match outcome {
    Err error |-> body_error_io;
    Ok unit |-> bind (Coproduct (FSOp AFull) AmbientOp)
      (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
      (Result ResourceError Bytes) (ResourceBodyResult Unit Unit)
      (mapBytes AFull mapping (MkMappingWindow (2 : Int) (4 : Int)))
      (\read. after_second_read_to_write mapping read)
  }

proc write_read_write_body (mapping : MappingHandle)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError Unit) (ResourceBodyResult Unit Unit)
    (mapWrite AFull mapping (2 : Int) (bytes_encode "ABCD"))
    (\outcome. after_write_to_read_write mapping outcome)

proc matrix_stage (_cap : Cap AFull)
  : HostIO AFull ExitCode visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError (ResourceBracketResult Unit Unit)) ExitCode
    (withMapping AFull Unit Unit (Anonymous (8 : Int)) ReadWrite __MATRIX_BODY__)
    (\outcome. finish outcome)

fn expect_invalid_bounds (outcome : Result ResourceError Bytes)
  : HostIO AFull (ResourceBodyResult Unit Unit) =
  match outcome {
    Err InvalidBounds |-> body_ok_io;
    Err error |-> body_error_io;
    Ok bytes |-> body_error_io
  }

fn expect_invalid_bounds_unit (outcome : Result ResourceError Unit)
  : HostIO AFull (ResourceBodyResult Unit Unit) =
  match outcome {
    Err InvalidBounds |-> body_ok_io;
    Err error |-> body_error_io;
    Ok unit |-> body_error_io
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

proc out_of_range_write_body (mapping : MappingHandle)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError Unit) (ResourceBodyResult Unit Unit)
    (mapWrite AFull mapping (6 : Int) (bytes_encode "ABCD"))
    (\outcome. expect_invalid_bounds_unit outcome)

proc out_of_range_write_stage (_cap : Cap AFull)
  : HostIO AFull ExitCode visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError (ResourceBracketResult Unit Unit)) ExitCode
    (withMapping AFull Unit Unit (Anonymous (8 : Int)) ReadWrite
      out_of_range_write_body)
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

proc after_read_to_read_only_write (mapping : MappingHandle)
  (outcome : Result ResourceError Bytes)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  match outcome {
    Err error |-> body_error_io;
    Ok bytes |-> bind (Coproduct (FSOp AFull) AmbientOp)
      (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
      (Result ResourceError Unit) (ResourceBodyResult Unit Unit)
      (mapWrite AFull mapping (0 : Int) (bytes_encode "X"))
      (\written. expect_read_only_refusal written)
  }

proc read_then_read_only_write_body (mapping : MappingHandle)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError Bytes) (ResourceBodyResult Unit Unit)
    (mapBytes AFull mapping (MkMappingWindow (0 : Int) (1 : Int)))
    (\outcome. after_read_to_read_only_write mapping outcome)

proc read_then_read_only_write_stage (_cap : Cap AFull)
  : HostIO AFull ExitCode visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError (ResourceBracketResult Unit Unit)) ExitCode
    (withMapping AFull Unit Unit (Anonymous (8 : Int)) ReadOnly
      read_then_read_only_write_body)
    (\outcome. finish outcome)

proc after_read_to_out_of_range (mapping : MappingHandle)
  (outcome : Result ResourceError Bytes)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  match outcome {
    Err error |-> body_error_io;
    Ok bytes |-> bind (Coproduct (FSOp AFull) AmbientOp)
      (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
      (Result ResourceError Bytes) (ResourceBodyResult Unit Unit)
      (mapBytes AFull mapping (MkMappingWindow (6 : Int) (4 : Int)))
      (\read. expect_invalid_bounds read)
  }

proc read_then_out_of_range_body (mapping : MappingHandle)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError Bytes) (ResourceBodyResult Unit Unit)
    (mapBytes AFull mapping (MkMappingWindow (0 : Int) (1 : Int)))
    (\outcome. after_read_to_out_of_range mapping outcome)

proc read_then_out_of_range_stage (_cap : Cap AFull)
  : HostIO AFull ExitCode visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError (ResourceBracketResult Unit Unit)) ExitCode
    (withMapping AFull Unit Unit (Anonymous (8 : Int)) ReadOnly
      read_then_out_of_range_body)
    (\outcome. finish outcome)

proc read_only_write_body (mapping : MappingHandle)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError Unit) (ResourceBodyResult Unit Unit)
    (mapWrite AFull mapping (0 : Int) (bytes_encode "X"))
    (\outcome. expect_read_only_refusal outcome)

proc read_only_write_stage (_cap : Cap AFull)
  : HostIO AFull ExitCode visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError (ResourceBracketResult Unit Unit)) ExitCode
    (withMapping AFull Unit Unit (Anonymous (8 : Int)) ReadOnly read_only_write_body)
    (\outcome. finish outcome)

proc accounting_body (_mapping : MappingHandle)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  body_ok_io

proc one_byte_accounting_stage (_cap : Cap AFull)
  : HostIO AFull ExitCode visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError (ResourceBracketResult Unit Unit)) ExitCode
    (withMapping AFull Unit Unit (Anonymous (1 : Int)) ReadOnly accounting_body)
    (\outcome. finish outcome)

proc over_page_accounting_stage (_cap : Cap AFull)
  : HostIO AFull ExitCode visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError (ResourceBracketResult Unit Unit)) ExitCode
    (withMapping AFull Unit Unit (Anonymous (4097 : Int)) ReadOnly accounting_body)
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

fn try_differential(case: &str, entry: &str, matrix_body: &str) -> Result<Differential, String> {
    let root = tempfile::Builder::new()
        .prefix(&format!("ken-abi-s6-surface-{case}-"))
        .tempdir()
        .map_err(|error| format!("{case}: creates temporary root: {error:?}"))?;
    let source = SOURCE
        .replace("__ENTRY__", entry)
        .replace("__MATRIX_BODY__", matrix_body);
    let output = ken_cli::build_native_program(
        &source,
        ken_cli::SourceFormat::Ken,
        &format!("abi_s6_surface_{}", case.replace('-', "_")),
        root.path(),
    )
    .map_err(|error| format!("{case}: native lowering: {error:?}"))?;
    let native = ken_runtime::run_bound_process_effect_observation(
        &output.artifact,
        &ken_runtime::NativeEffectRunOptionsV1 {
            arguments: Vec::new(),
            environment: Vec::new(),
            cwd: root.path().to_owned(),
            plan_hash: output.plan_transport_hash,
        },
    )
    .map_err(|error| format!("{case}: native execution: {error:?}"))?;
    let mut host = ken_interp::PosixHost::new_at(root.path());
    let interpreted = ken_cli::run_program_effect_observation(
        &source,
        ken_cli::SourceFormat::Ken,
        &[],
        &[],
        root.path().as_os_str().as_encoded_bytes(),
        &mut host,
    )
    .map_err(|error| format!("{case}: interpreter execution: {error:?}"))?;
    Ok(Differential {
        interpreted,
        native,
    })
}

fn differential(case: &str, entry: &str) -> Differential {
    try_differential(case, entry, "read_body").unwrap_or_else(|error| panic!("{error}"))
}

fn differential_matrix_body(case: &str, body: &str) -> Result<Differential, String> {
    try_differential(case, "matrix_stage", body)
}

fn differential_matrix_body_with_suppressed_local_drive(
    case: &str,
    body: &str,
) -> (Result<Differential, String>, usize, bool) {
    let owned_case = case.to_owned();
    let body = body.to_owned();
    std::thread::Builder::new()
        .name(format!("abi-s6-{case}-suppressed-local-drive"))
        .spawn(move || {
            let (result, applications) = ken_runtime::with_handler_owned_deferred_response_mutation(
                ken_runtime::HandlerOwnedDeferredResponseMutation::SuppressLocalContinuationDrive,
                || try_differential(&owned_case, "matrix_stage", &body),
            );
            (
                result,
                applications,
                ken_runtime::handler_owned_deferred_response_mutation_is_exact(),
            )
        })
        .expect("starts the suppressed local-drive worker")
        .join()
        .expect("the suppressed local-drive worker completes")
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

fn parity_difference(case: &str, result: &Differential) -> Option<String> {
    if (result.native.exit_status, result.interpreted.exit_status) != (0, 0) {
        return Some(format!(
            "{case}: exit statuses native={} interpreter={}",
            result.native.exit_status, result.interpreted.exit_status
        ));
    }
    if result.native.terminal_error.is_some() || result.interpreted.terminal_error.is_some() {
        return Some(format!(
            "{case}: terminal errors native={:?} interpreter={:?}",
            result.native.terminal_error, result.interpreted.terminal_error
        ));
    }
    if non_release_events(&result.native) != non_release_events(&result.interpreted) {
        return Some(format!("{case}: ordered non-release traces differ"));
    }
    if release_set(&result.native) != release_set(&result.interpreted) {
        return Some(format!("{case}: release sets differ"));
    }
    if result.native.terminal_exit != result.interpreted.terminal_exit {
        return Some(format!("{case}: terminal exit classes differ"));
    }
    None
}

/// Promise class: durable behavioral invariant. CONTROL: the complete ordered
/// two-operation Mapping access matrix plus both alternating three-operation
/// forms must execute through checked source in both engines. Every row requires
/// exact operation-order, request/outcome/binding parity and one terminal
/// release. CLAIMED: the D1 carried-transport frontier is closed for every
/// Mapping access kind that can follow either sibling. THE GAP: file-backed
/// acquisition and MAP_PRIVATE COW remain D5b.
#[test]
fn complete_carried_mapping_access_matrix_matches_the_interpreter() {
    use ken_runtime::HostOpV1::{MappingAllocate, MappingReadView, MappingWriteView};

    let cases = [
        (
            "read-read",
            "read_read_body",
            vec![MappingAllocate, MappingReadView, MappingReadView],
        ),
        (
            "read-write",
            "read_write_body",
            vec![MappingAllocate, MappingReadView, MappingWriteView],
        ),
        (
            "write-read",
            "write_read_body",
            vec![MappingAllocate, MappingWriteView, MappingReadView],
        ),
        (
            "write-write",
            "write_write_body",
            vec![MappingAllocate, MappingWriteView, MappingWriteView],
        ),
        (
            "read-write-read",
            "read_write_read_body",
            vec![
                MappingAllocate,
                MappingReadView,
                MappingWriteView,
                MappingReadView,
            ],
        ),
        (
            "write-read-write",
            "write_read_write_body",
            vec![
                MappingAllocate,
                MappingWriteView,
                MappingReadView,
                MappingWriteView,
            ],
        ),
    ];
    let mut failures = Vec::new();
    for (case, body, expected_operations) in cases {
        match differential_matrix_body(case, body) {
            Err(error) => failures.push(error),
            Ok(result) => {
                if let Some(error) = parity_difference(case, &result) {
                    failures.push(error);
                    continue;
                }
                let actual_operations = non_release_events(&result.native)
                    .iter()
                    .map(|event| event.operation)
                    .collect::<Vec<_>>();
                if actual_operations != expected_operations {
                    failures.push(format!(
                        "{case}: operations {actual_operations:?}, expected {expected_operations:?}"
                    ));
                }
                if release_set(&result.native).len() != 1 {
                    failures.push(format!("{case}: expected exactly one release"));
                }
            }
        }
    }
    assert!(
        failures.is_empty(),
        "the carried Mapping access frontier must have no refusal or trap:\n{}",
        failures.join("\n")
    );
}

/// Promise class: durable behavioral invariant.
///
/// MEASURED: the heterogeneous read-write-read program executes with full
/// native/interpreter parity, while suppressing only the handler-owned local
/// continuation drive reaches that production seat and restores the exact
/// pending-`Vis` native trap.
/// CLAIMED: the statically bounded response-owner interpreter, rather than an
/// unrelated carried-seat or host change, dispatches effects two and three.
/// THE GAP: planner shared-body selection and generated-entry closure have their
/// own compile-time invariants; this control isolates the lowering consumption
/// seat after those invariants have admitted the same unchanged source.
#[test]
fn suppressing_bounded_response_owner_drive_restores_the_pending_vis_trap() {
    let baseline =
        differential_matrix_body("read-write-read-drive-baseline", "read_write_read_body")
            .expect("the exact bounded response-owner path executes");
    assert_parity("read-write-read-drive-baseline", &baseline);

    let (mutated, applications, restored) = differential_matrix_body_with_suppressed_local_drive(
        "read-write-read-drive-suppressed",
        "read_write_read_body",
    );
    assert!(applications > 0, "the local-drive mutation did not reach");
    let error = match mutated {
        Err(error) => error,
        Ok(_) => panic!("suppressing local drive did not restore the native trap"),
    };
    assert!(
        error.contains("UnclassifiedRuntimeTrap { terminal_value: -1 }"),
        "the local-drive suppression failed at a different boundary: {error}"
    );
    assert!(restored, "the local-drive mutation leaked past its worker");
}

/// Promise class: durable behavioral invariant. MEASURED: an honest checked
/// `mapBytes mapping window` executes in both engines after both window `Int`s
/// cross declared proc ABI slots, returns four copied zero bytes from exactly
/// [2,6), and releases once. CLAIMED: both carried read-window seats compose
/// over the frozen MappingReadView wire. THE GAP: file acquisition is not
/// exercised and remains D5b.
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

/// Promise class: durable behavioral invariant. CONTROL: one checked write at
/// offset 2 is followed by one checked read whose window `Int`s cross declared
/// proc ABI slots; both engines must observe `ABCD`, agree on the ordered effect
/// trace, and release once. CLAIMED: payload-derived write extent composes with
/// window-direct read access without a response transform. THE GAP: MAP_PRIVATE
/// file isolation remains D5b.
#[test]
fn offset_direct_map_write_then_window_read_preserves_process_local_bytes() {
    let result = differential("write-read", "write_read_stage");
    assert_parity("write-read", &result);
    assert_eq!(
        non_release_events(&result.native)
            .iter()
            .map(|event| event.operation)
            .collect::<Vec<_>>(),
        vec![
            ken_runtime::HostOpV1::MappingAllocate,
            ken_runtime::HostOpV1::MappingWriteView,
            ken_runtime::HostOpV1::MappingReadView,
        ],
        "write-read: the native trace preserves source order"
    );
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

/// Promise class: durable behavioral invariant. CONTROL: one checked read of
/// [2,6) is followed by one checked write at offset 2 after the read-window and
/// write-offset `Int` seats cross declared proc ABI slots; both engines must
/// preserve the exact read-before-write trace, copied zero bytes, written bytes,
/// terminal result, and one release. CLAIMED: read-window then payload-derived
/// write access composes independently of sequential direction. THE GAP: later
/// in-mapping readback and MAP_PRIVATE file isolation are covered by the
/// write-then-read sibling and remain D5b respectively.
#[test]
fn window_read_then_offset_direct_map_write_executes_in_source_order() {
    let result = differential("read-write", "read_write_stage");
    assert_parity("read-write", &result);
    let events = non_release_events(&result.native);
    assert_eq!(
        events
            .iter()
            .map(|event| event.operation)
            .collect::<Vec<_>>(),
        vec![
            ken_runtime::HostOpV1::MappingAllocate,
            ken_runtime::HostOpV1::MappingReadView,
            ken_runtime::HostOpV1::MappingWriteView,
        ],
        "read-write: the native trace preserves source order"
    );
    assert_eq!(
        events[1].request,
        ken_runtime::CanonicalRequestV1::MappingReadView {
            start: 2,
            length: 4,
        }
    );
    assert!(matches!(
        &events[1].outcome,
        ken_runtime::CanonicalOutcomeV1::Success(ken_runtime::CanonicalReplyV1::Bytes(bytes))
            if bytes == &[0, 0, 0, 0]
    ));
    assert_eq!(
        events[2].request,
        ken_runtime::CanonicalRequestV1::MappingWriteView {
            start: 2,
            bytes: b"ABCD".to_vec(),
        }
    );
    assert!(matches!(
        events[2].outcome,
        ken_runtime::CanonicalOutcomeV1::Success(ken_runtime::CanonicalReplyV1::Unit)
    ));
    assert_eq!(release_set(&result.native).len(), 1);
}

/// Promise class: normative compatibility vector. MEASURED: checked
/// `withMapping` acquisitions at the two seed boundary lengths reach the real
/// MappingAllocate operation in both engines with identical requests, a
/// successful resource acquisition, terminal result, and one release. The shared
/// accounting seam independently pins those requests to exact canonical charges
/// in `abi_s6_d5a_mapping_accounting_is_fixed_4kib_and_buffer_is_byte_granular`.
/// CLAIMED: 1 byte and 4097 bytes exercise the native/interpreter paths whose
/// common accounting rule charges exactly 4096 and 8192 bytes. THE GAP: the
/// effect-observation schema intentionally exposes no live-capacity counter;
/// exact charge is observed at the shared ResourceTable boundary, not inferred
/// from this trace.
#[test]
fn fixed_accounting_boundary_lengths_reach_both_engines() {
    for (case, entry, length) in [
        ("accounting-one-byte", "one_byte_accounting_stage", 1),
        ("accounting-over-page", "over_page_accounting_stage", 4097),
    ] {
        let result = differential(case, entry);
        assert_parity(case, &result);
        let events = non_release_events(&result.native);
        assert_eq!(events.len(), 1, "{case}: allocate is the only body effect");
        assert_eq!(events[0].operation, ken_runtime::HostOpV1::MappingAllocate);
        assert!(matches!(
            events[0].request,
            ken_runtime::CanonicalRequestV1::MappingAllocate {
                length: actual,
                ..
            } if actual == length
        ));
        assert!(matches!(
            events[0].outcome,
            ken_runtime::CanonicalOutcomeV1::Success(
                ken_runtime::CanonicalReplyV1::ResourceAcquired { .. }
            )
        ));
        assert_eq!(release_set(&result.native).len(), 1);
    }
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

/// Promise class: normative compatibility vector. MEASURED: a checked write of
/// four payload bytes at offset 6 derives [6,10), reaches the real
/// MappingWriteView operation, and returns exact InvalidBounds in both engines.
/// CLAIMED: mapWrite bounds-checks the payload-derived range and never clamps or
/// exposes a partial write. THE GAP: negative read-window scalar narrowing is
/// pinned independently below; this row exercises a positive write overrun.
#[test]
fn out_of_range_map_write_is_invalid_bounds_not_clamped() {
    let result = differential("out-of-range-write", "out_of_range_write_stage");
    assert_parity("out-of-range-write", &result);
    let writes = operation_events(&result.native, ken_runtime::HostOpV1::MappingWriteView);
    assert_eq!(writes.len(), 1);
    assert_eq!(
        writes[0].request,
        ken_runtime::CanonicalRequestV1::MappingWriteView {
            start: 6,
            bytes: b"ABCD".to_vec(),
        }
    );
    assert!(matches!(
        writes[0].outcome,
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

/// Promise class: durable behavioral invariant. MEASURED: a valid first read
/// is followed by a write through the same ReadOnly mapping; effect two reaches
/// the real host admission and returns the exact WRITE-vs-READ masks in both
/// engines. CLAIMED: response-owner threading never bypasses per-effect rights
/// admission on effects two through N. THE GAP: revocation during an anonymous
/// mapping bracket is not source-constructible; file-source revocation remains
/// D5b with MappingAcquireFile.
#[test]
fn second_mapping_effect_rechecks_read_only_rights() {
    let result = differential(
        "read-then-read-only-write",
        "read_then_read_only_write_stage",
    );
    assert_parity("read-then-read-only-write", &result);
    let events = non_release_events(&result.native);
    assert_eq!(
        events
            .iter()
            .map(|event| event.operation)
            .collect::<Vec<_>>(),
        vec![
            ken_runtime::HostOpV1::MappingAllocate,
            ken_runtime::HostOpV1::MappingReadView,
            ken_runtime::HostOpV1::MappingWriteView,
        ]
    );
    assert!(matches!(
        events[2].outcome,
        ken_runtime::CanonicalOutcomeV1::Error(ken_runtime::SemanticErrorV1::Resource(
            ken_runtime::ResourceErrorV1::RightNotHeld {
                required: 2,
                held: 1,
            }
        ))
    ));
    assert_eq!(release_set(&result.native).len(), 1);
}

/// Promise class: durable behavioral invariant. MEASURED: a valid first read
/// is followed by [6,10) against the same eight-byte mapping; effect two reaches
/// MappingReadView and returns exact InvalidBounds in both engines. CLAIMED:
/// every statically unrolled access re-runs bounds admission and never clamps.
/// THE GAP: negative scalar narrowing is independently pinned by the single-op
/// sibling because its refusal occurs before host dispatch.
#[test]
fn second_mapping_effect_rechecks_bounds() {
    let result = differential("read-then-out-of-range", "read_then_out_of_range_stage");
    assert_parity("read-then-out-of-range", &result);
    let reads = operation_events(&result.native, ken_runtime::HostOpV1::MappingReadView);
    assert_eq!(reads.len(), 2);
    assert!(matches!(
        reads[1].outcome,
        ken_runtime::CanonicalOutcomeV1::Error(ken_runtime::SemanticErrorV1::Resource(
            ken_runtime::ResourceErrorV1::InvalidBounds
        ))
    ));
    assert_eq!(release_set(&result.native).len(), 1);
}

/// Promise class: normative compatibility vector. MEASURED: a checked mapWrite
/// through a ReadOnly handle enters the real write operation and returns the
/// exact WRITE-vs-READ RightNotHeld masks in both engines. CLAIMED: ReadOnly
/// mappings refuse payload-derived writes without mutating bytes. THE GAP: the
/// unchanged byte content is independently covered by the host resource-table
/// test.
#[test]
fn read_only_mapping_refuses_payload_write_with_exact_rights() {
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
