//! RT-ESCAPE linked-native discriminator: an escaped resource consumed by a
//! host operation whose `Result` match fans out must reach native execution
//! with the same observable bracket semantics as the interpreter.
//!
//! ## The defect (native-lowering completeness, M1)
//!
//! Constructing a closed-but-still-referenced resource requires escaping it from
//! its bracket; using that escaped resource through a host op (`readAt`,
//! `writeAll`, metadata, release) whose `Result` match fans out used to fail
//! native lowering with
//! `OrientedSubcontinuationPlanV1: checked Runtime frame marker was consumed
//! more than once`. Escaping a resource *unused* downstream, or escaping a
//! resource plus a plain value, both lowered fine (`escape_one_used`,
//! `escape_resource_plus_plain` below) — the trip is the *use* site.
//!
//! Classification is **M1** (one checked occurrence revisited, not two occurrences
//! aliasing a shared id): a match on a dynamic value lowers its shared post-match
//! continuation once per mutually-exclusive arm (`ok_block`/`err_block` off one
//! `brif`), so a checked subcontinuation frame in that shared continuation is a
//! *distinct lawful activation per arm*. The single per-lowering
//! `consumed_subcontinuation_frames` set conflated the two arms. The repair forks
//! that set per mutually-exclusive branch (snapshot → reset-per-arm → union at
//! rejoin) in every source-prefix fanout lowerer via `lower_forked_branch` — the
//! complete set (each instantiates one `source_prefix_template` per arm off a
//! single `brif`) is bounded-Nat (`Zero`/`Suc`), Bool, host-result, and the two
//! dynamic-constructor variants (nested + planned). The fork preserves
//! the within-a-single-path affine rejection (a real double-consume on one path
//! still rejects — proven by
//! `rt_escape_within_path_duplicate_frame_consume_still_rejects` in
//! `crates/ken-runtime/src/cranelift_backend/lowering/core/tests/control.rs`).
//!
//! Each case runs the identical source through the linked native artifact and the
//! reference interpreter and asserts the canonical observations agree, so the
//! guard is a semantic equivalence, not merely "it lowers".

#[cfg(target_os = "linux")]
struct Differential {
    interpreted: ken_runtime::EffectObservation,
    native: ken_runtime::EffectObservation,
    ret_key_applications: Vec<(u32, u32, ken_runtime::HostOpV1)>,
    ret_sink_assessments: Vec<(String, String)>,
    ret_sink_installs: Vec<ken_runtime::ComposedReturnRetSinkObservation>,
}

#[cfg(target_os = "linux")]
fn output_dir(name: &str) -> tempfile::TempDir {
    let prefix = format!("ken-rtescape-{name}-");
    tempfile::Builder::new().prefix(&prefix).tempdir().unwrap()
}

/// Compile `source` to a linked native artifact, run it, then run the identical
/// source through the reference interpreter against the same root, and return
/// both canonical observations. A `held.bin` seed file is present in the root.
#[cfg(target_os = "linux")]
fn differential(case: &str, source: &str) -> Differential {
    differential_with_profile(
        case,
        source,
        ken_runtime::boundary_resource_profile::starter_smoke_profile(),
    )
}

#[cfg(target_os = "linux")]
fn differential_with_profile(
    case: &str,
    source: &str,
    profile: ken_runtime::boundary_resource_profile::BoundaryResourceProfileV3,
) -> Differential {
    let root = output_dir(case);
    std::fs::write(root.path().join("held.bin"), b"held resource").unwrap();

    let (((output, applications), assessments), installs, _) =
        ken_runtime::with_composed_return_ret_sink_mutation(
            ken_runtime::ComposedReturnRetSinkMutation::Exact,
            || {
                ken_runtime::with_composed_return_ret_assessments(|| {
                    ken_runtime::with_pending_checked_ret_sink_applications(|| {
                        ken_cli::build_native_program(
                            source,
                            ken_cli::SourceFormat::Ken,
                            &format!("rt_escape_{}", case.replace('-', "_")),
                            root.path(),
                            profile,
                        )
                    })
                })
            },
        );
    let output =
        output.unwrap_or_else(|error| panic!("{case}: reaches linked native lowering: {error:?}"));
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
        source,
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
        ret_key_applications: applications,
        ret_sink_assessments: assessments,
        ret_sink_installs: installs,
    }
}

/// Native and interpreter must agree on exit, terminal class, and the exact
/// canonical effect-operation sequence.
#[cfg(target_os = "linux")]
fn assert_native_matches_interpreter(case: &str, diff: &Differential) {
    let Differential {
        interpreted,
        native,
        ret_key_applications,
        ..
    } = diff;
    assert_eq!(
        native.exit_status, interpreted.exit_status,
        "{case}: exit status must agree; native={native:?} interp={interpreted:?}"
    );
    assert_eq!(
        native.terminal_error, interpreted.terminal_error,
        "{case}: terminal error must agree"
    );
    assert_eq!(
        native.terminal_exit, interpreted.terminal_exit,
        "{case}: terminal exit class must agree"
    );
    let native_ops: Vec<_> = native
        .effect_trace
        .iter()
        .map(|event| event.operation)
        .collect();
    let interp_ops: Vec<_> = interpreted
        .effect_trace
        .iter()
        .map(|event| event.operation)
        .collect();
    assert_eq!(
        native_ops, interp_ops,
        "{case}: canonical effect-operation sequence must agree across executors"
    );
    // Check the population after the behavioral discriminator. A mutation
    // that breaks build, execution or effects must fail for that reason, not
    // because its changed key also empties this diagnostic observation.
    assert_eq!(
        !ret_key_applications.is_empty(),
        case == "escape-buffer-then-readat" || case.starts_with("nat-reached-"),
        "{case}: decisive checked-control Ret applications {ret_key_applications:?}"
    );
}

// (a) One escaped Resource, used once after its bracket settles. Always lowered
// (negative control): a single escaped-resource use consumes its checked frame
// exactly once, on one path.
#[cfg(target_os = "linux")]
const ESCAPE_ONE_USED: &str = r#"program capabilities FS AFull
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
    ResourceBracketBodyAndReleaseError body_error release_error |-> host_exit AFull (Failure 95)
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

// (b) A Resource plus a plain value escaped as one aggregate. Always lowered
// (negative control): the aggregate carries one Resource, whose checked frame is
// consumed once.
#[cfg(target_os = "linux")]
const ESCAPE_RESOURCE_PLUS_PLAIN: &str = r#"program capabilities FS AFull
proc after_b
  (outcome : Result FileError (ResourceBracketResult Unit (Prod (Resource ResourceKind.FsHandle) Unit)))
  : HostIO AFull ExitCode visits [FS] =
  match outcome {
    Err open_error |-> host_exit AFull (Failure 96);
    Ok bracket |-> match bracket {
      ResourceBracketOk pair |-> host_exit AFull Success;
      ResourceBracketBodyError error |-> host_exit AFull (Failure 93);
      ResourceBracketReleaseError error |-> host_exit AFull (Failure 94);
      ResourceBracketBodyAndReleaseError body_error release_error |-> host_exit AFull (Failure 95)
    }
  }

proc main (_input : ProcessInput) (caps : ProgramCaps AFull)
  : HostIO AFull ExitCode visits [FS] =
  match caps {
    MkProgramCaps cap |->
      bind (Coproduct (FSOp AFull) AmbientOp)
        (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
        (Result FileError (ResourceBracketResult Unit (Prod (Resource ResourceKind.FsHandle) Unit))) ExitCode
        (withResource AFull Unit (Prod (Resource ResourceKind.FsHandle) Unit)
          cap (bytes_encode "held.bin") ResourceMetadata
          (\resource. Ret (Coproduct (FSOp AFull) AmbientOp)
            (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
            (ResourceBodyResult Unit (Prod (Resource ResourceKind.FsHandle) Unit))
            (ResourceBodyOk Unit (Prod (Resource ResourceKind.FsHandle) Unit)
              (MkProd (Resource ResourceKind.FsHandle) Unit resource MkUnit))))
        (\outcome. after_b outcome)
  }
"#;

// (c) THE defect: escape the FILE out of its bracket, then `readAt` it (with a
// live buffer) after settlement. `readAt` returns `Result ResourceError
// ReadProgress`; its match fans out (Ok/Err), and the escaped file's checked
// frame lives in the shared post-match continuation. Pre-fix this failed native
// lowering with "checked Runtime frame marker was consumed more than once".
#[cfg(target_os = "linux")]
const ESCAPE_FILE_THEN_READAT: &str = r#"program capabilities FS AFull
proc read_body (file_closed : Resource ResourceKind.FsHandle) (buffer : BufferHandle)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError ReadProgress) (ResourceBodyResult Unit Unit)
    (readAt AFull file_closed (0 : Int) buffer (MkBufferWindow (0 : Int) (6 : Int)))
    (\outcome. Ret (Coproduct (FSOp AFull) AmbientOp)
      (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
      (ResourceBodyResult Unit Unit) (ResourceBodyOk Unit Unit MkUnit))

proc after_file_escape (file_closed : Resource ResourceKind.FsHandle)
  : HostIO AFull ExitCode visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError (ResourceBracketResult Unit Unit)) ExitCode
    (withBuffer AFull Unit Unit (6 : Int) (read_body file_closed))
    (\outcome. host_exit AFull Success)

proc handle_outer (outcome : Result FileError (ResourceBracketResult Unit (Resource ResourceKind.FsHandle)))
  : HostIO AFull ExitCode visits [FS] =
  match outcome {
    Err open_error |-> host_exit AFull (Failure 96);
    Ok bracket |-> match bracket {
      ResourceBracketOk file_closed |-> after_file_escape file_closed;
      ResourceBracketBodyError error |-> host_exit AFull (Failure 93);
      ResourceBracketReleaseError error |-> host_exit AFull (Failure 94);
      ResourceBracketBodyAndReleaseError body_error release_error |-> host_exit AFull (Failure 95)
    }
  }

proc main (_input : ProcessInput) (caps : ProgramCaps AFull)
  : HostIO AFull ExitCode visits [FS] =
  match caps {
    MkProgramCaps cap |->
      bind (Coproduct (FSOp AFull) AmbientOp)
        (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
        (Result FileError (ResourceBracketResult Unit (Resource ResourceKind.FsHandle))) ExitCode
        (withResource AFull Unit (Resource ResourceKind.FsHandle)
          cap (bytes_encode "held.bin") ResourceRead
          (\resource. Ret (Coproduct (FSOp AFull) AmbientOp)
            (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
            (ResourceBodyResult Unit (Resource ResourceKind.FsHandle))
            (ResourceBodyOk Unit (Resource ResourceKind.FsHandle) resource)))
        (\outcome. handle_outer outcome)
  }
"#;

// Closure across resource *kinds*: the mirror of (c) with the escaped resource
// being a `Buffer` instead of an `FsHandle`. Escape the BUFFER out of its
// bracket, then `readAt` it with a still-live file. Same fan-out lowering, other
// kind — pre-fix this tripped the identical "consumed more than once".
#[cfg(target_os = "linux")]
const ESCAPE_BUFFER_THEN_READAT: &str = r#"program capabilities FS AFull
fn escape_buffer (buffer : BufferHandle)
  : HostIO AFull (ResourceBodyResult Unit (BufferHandle)) =
  Ret (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (ResourceBodyResult Unit (BufferHandle))
    (ResourceBodyOk Unit (BufferHandle) buffer)

proc read_with_escaped_buffer (file : Resource ResourceKind.FsHandle) (buffer_closed : BufferHandle)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError ReadProgress) (ResourceBodyResult Unit Unit)
    (readAt AFull file (0 : Int) buffer_closed (MkBufferWindow (0 : Int) (6 : Int)))
    (\outcome. Ret (Coproduct (FSOp AFull) AmbientOp)
      (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
      (ResourceBodyResult Unit Unit) (ResourceBodyOk Unit Unit MkUnit))

proc after_buffer_escape
  (file : Resource ResourceKind.FsHandle)
  (inner : Result ResourceError (ResourceBracketResult Unit (BufferHandle)))
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  match inner {
    Err allocate_error |-> Ret (Coproduct (FSOp AFull) AmbientOp)
      (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
      (ResourceBodyResult Unit Unit) (ResourceBodyErr Unit Unit MkUnit);
    Ok bracket |-> match bracket {
      ResourceBracketOk buffer_closed |-> read_with_escaped_buffer file buffer_closed;
      ResourceBracketBodyError error |-> Ret (Coproduct (FSOp AFull) AmbientOp)
        (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
        (ResourceBodyResult Unit Unit) (ResourceBodyErr Unit Unit MkUnit);
      ResourceBracketReleaseError error |-> Ret (Coproduct (FSOp AFull) AmbientOp)
        (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
        (ResourceBodyResult Unit Unit) (ResourceBodyErr Unit Unit MkUnit);
      ResourceBracketBodyAndReleaseError body_error release_error |-> Ret (Coproduct (FSOp AFull) AmbientOp)
        (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
        (ResourceBodyResult Unit Unit) (ResourceBodyErr Unit Unit MkUnit)
    }
  }

proc file_body (file : Resource ResourceKind.FsHandle)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError (ResourceBracketResult Unit (BufferHandle)))
    (ResourceBodyResult Unit Unit)
    (withBuffer AFull Unit (BufferHandle) (6 : Int) escape_buffer)
    (\inner. after_buffer_escape file inner)

fn finish (outcome : Result FileError (ResourceBracketResult Unit Unit))
  : HostIO AFull ExitCode =
  match outcome {
    Err error |-> host_exit AFull (Failure 81);
    Ok bracket |-> match bracket {
      ResourceBracketOk value |-> host_exit AFull Success;
      ResourceBracketBodyError error |-> host_exit AFull (Failure 82);
      ResourceBracketReleaseError error |-> host_exit AFull (Failure 83);
      ResourceBracketBodyAndReleaseError body_error release_error |-> host_exit AFull (Failure 84)
    }
  }

proc main (_input : ProcessInput) (caps : ProgramCaps AFull)
  : HostIO AFull ExitCode visits [FS] =
  match caps {
    MkProgramCaps cap |->
      bind (Coproduct (FSOp AFull) AmbientOp)
        (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
        (Result FileError (ResourceBracketResult Unit Unit)) ExitCode
        (withResource AFull Unit Unit cap (bytes_encode "held.bin") ResourceRead file_body)
        (\outcome. finish outcome)
  }
"#;

// R2 reaching lane (AC-6): once two nested buffer resources compile, a
// `BufferSpan` obtained by reading into buffer_a (capacity 6, span length 6)
// applied to `freeze` on buffer_b (capacity 2) is the cross-buffer overlap
// fault. Statically predicted outcome: an `InvalidBounds` rejection (the span
// length exceeds buffer_b's capacity). The trace confirms `BufferFreeze` fails
// closed with `InvalidBounds` in both executors — the span length is bounded by
// the *target* buffer, so a span from a larger buffer cannot read a smaller one
// out of bounds. No distinct BufferFreeze defect; the obligation is discharged
// as a bounds rejection, not buried.
#[cfg(target_os = "linux")]
const R2_CROSS_BUFFER_FREEZE: &str = r#"program capabilities FS AFull
fn body_from_freeze (r : Result ResourceError Bytes) : ResourceBodyResult Unit Unit =
  match r {
    Ok bytes |-> ResourceBodyErr Unit Unit MkUnit;
    Err error |-> match error {
      InvalidBounds |-> ResourceBodyOk Unit Unit MkUnit;
      Closed |-> ResourceBodyErr Unit Unit MkUnit;
      InvalidOffset |-> ResourceBodyErr Unit Unit MkUnit;
      BufferLimit |-> ResourceBodyErr Unit Unit MkUnit;
      AllocationFailed |-> ResourceBodyErr Unit Unit MkUnit;
      NoProgress |-> ResourceBodyErr Unit Unit MkUnit;
      MappingLimit |-> ResourceBodyErr Unit Unit MkUnit;
      MalformedResource |-> ResourceBodyErr Unit Unit MkUnit;
      RightNotHeld required held |-> ResourceBodyErr Unit Unit MkUnit;
      ResourceHostIO io |-> ResourceBodyErr Unit Unit MkUnit;
      ReleaseFailed kind identity io |-> ResourceBodyErr Unit Unit MkUnit;
      ResourceKindMismatch expected actual |-> ResourceBodyErr Unit Unit MkUnit
    }
  }

fn body_from_bracket (bracket : ResourceBracketResult Unit Unit) : ResourceBodyResult Unit Unit =
  match bracket {
    ResourceBracketOk value |-> ResourceBodyOk Unit Unit MkUnit;
    ResourceBracketBodyError error |-> ResourceBodyErr Unit Unit MkUnit;
    ResourceBracketReleaseError error |-> ResourceBodyErr Unit Unit MkUnit;
    ResourceBracketBodyAndReleaseError body_error release_error |-> ResourceBodyErr Unit Unit MkUnit
  }

fn body_from_alloc (outcome : Result ResourceError (ResourceBracketResult Unit Unit))
  : ResourceBodyResult Unit Unit =
  match outcome {
    Err error |-> ResourceBodyErr Unit Unit MkUnit;
    Ok bracket |-> body_from_bracket bracket
  }

proc after_read (buffer_b : BufferHandle) (outcome : Result ResourceError ReadProgress)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  match outcome {
    Err error |-> Ret (Coproduct (FSOp AFull) AmbientOp)
      (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
      (ResourceBodyResult Unit Unit) (ResourceBodyErr Unit Unit MkUnit);
    Ok progress |-> match progress {
      ReadEof |-> Ret (Coproduct (FSOp AFull) AmbientOp)
        (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
        (ResourceBodyResult Unit Unit) (ResourceBodyErr Unit Unit MkUnit);
      ReadSome span_a count |->
        bind (Coproduct (FSOp AFull) AmbientOp)
          (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
          (Result ResourceError Bytes) (ResourceBodyResult Unit Unit)
          (freeze AFull buffer_b span_a)
          (\r. Ret (Coproduct (FSOp AFull) AmbientOp)
            (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
            (ResourceBodyResult Unit Unit) (body_from_freeze r))
    }
  }

proc buffer_b_body (file : Resource ResourceKind.FsHandle) (buffer_a : BufferHandle) (buffer_b : BufferHandle)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError ReadProgress) (ResourceBodyResult Unit Unit)
    (readAt AFull file (0 : Int) buffer_a (MkBufferWindow (0 : Int) (6 : Int)))
    (\outcome. after_read buffer_b outcome)

proc buffer_a_body (file : Resource ResourceKind.FsHandle) (buffer_a : BufferHandle)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError (ResourceBracketResult Unit Unit)) (ResourceBodyResult Unit Unit)
    (withBuffer AFull Unit Unit (2 : Int) (buffer_b_body file buffer_a))
    (\outcome. Ret (Coproduct (FSOp AFull) AmbientOp)
      (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
      (ResourceBodyResult Unit Unit) (body_from_alloc outcome))

proc file_body (file : Resource ResourceKind.FsHandle)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError (ResourceBracketResult Unit Unit)) (ResourceBodyResult Unit Unit)
    (withBuffer AFull Unit Unit (6 : Int) (buffer_a_body file))
    (\outcome. Ret (Coproduct (FSOp AFull) AmbientOp)
      (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
      (ResourceBodyResult Unit Unit) (body_from_alloc outcome))

fn finish (outcome : Result FileError (ResourceBracketResult Unit Unit)) : HostIO AFull ExitCode =
  match outcome {
    Err error |-> host_exit AFull (Failure 81);
    Ok bracket |-> match bracket {
      ResourceBracketOk value |-> host_exit AFull Success;
      ResourceBracketBodyError error |-> host_exit AFull (Failure 82);
      ResourceBracketReleaseError error |-> host_exit AFull (Failure 83);
      ResourceBracketBodyAndReleaseError body_error release_error |-> host_exit AFull (Failure 84)
    }
  }

proc main (_input : ProcessInput) (caps : ProgramCaps AFull)
  : HostIO AFull ExitCode visits [FS] =
  match caps {
    MkProgramCaps cap |->
      bind (Coproduct (FSOp AFull) AmbientOp)
        (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
        (Result FileError (ResourceBracketResult Unit Unit)) ExitCode
        (withResource AFull Unit Unit cap (bytes_encode "held.bin") ResourceRead file_body)
        (\outcome. finish outcome)
  }
"#;

// Closure across the bounded-Nat fanout lowerer (`lower_source_bounded_nat_match`,
// the fifth source-prefix fanout): escape the file, `readAt` it, then
// `match (buffer_span_budget span) { Zero; Suc }` whose SHARED continuation does
// a *second* `readAt` on the escaped file. The Nat match fans out (Zero/Suc off
// one `brif`) and the second read's checked frame lives in its shared tail, so
// pre-fix it tripped the identical "consumed more than once" on the Nat lane
// (verified by reverting only the Nat-lane fork). The current compiler retries
// unclaimed scalar joins, emits the native artifact, and matches interpreter
// stdout, full effects, and terminal observations in the active Nat row on its
// explicitly provisioned 256 MiB thread; no default-stack claim is made.
#[cfg(target_os = "linux")]
const NAT_FANOUT_ESCAPED_RESOURCE: &str = include_str!("rt_nat_fanout_escaped_resource.ken");

#[cfg(target_os = "linux")]
// Readmitted under RT-IGNORED-PASSING-ROWS, row 6 of 11.
//
// The struck label read: "RT-CARRIED-RESOURCE-SCALAR: the FsHandleMetadata seat
// cannot observe a carried word as a resource scalar; fails at base 21fd46dc",
// and the block above it claimed the row "refuses at object emission, so the
// program never executes". Refuted POSITIVELY, not just by absence: the native
// effect-operation sequence for this row is [FsOpen, ResourceRelease,
// FsHandleMetadata]. The seat the label says cannot observe a carried word
// appears in the trace, having observed one.
//
// READMITTED ON A MUTATION, NOT ON THE GREEN. Duplicating the live native
// dispatch event push in ken-host abi_v1.rs (the sole producer of the native
// effect trace, in ken_host_dispatch after dispatch_host_op_v1 returns) reds the
// canonical effect-operation-sequence assertion with the perturbation visible in
// it -- every native op appearing twice against the interpreter's single
// sequence. The interpreter side is untouched by that edit and is an independent
// producer, which is what makes the comparison an oracle rather than a
// self-check.
//
// TWO EARLIER MUTATIONS DID NOT ESTABLISH IT AND ARE RECORDED BECAUSE THEY ARE
// WHY THE THIRD IS TRUSTWORTHY. Duplicating the settlement event in
// record_resource_settlements left both rows GREEN: that is the process-exit
// finalize-all path, and these resources are released before termination, so it
// never reaches. Duplicating the live push without cloning the request FAILED TO
// COMPILE (E0382, CanonicalRequestV1 is not Copy) -- a red that is not a probe
// result, which is exactly the four-producer aggregate a colour reading cannot
// separate. Both were read from the output, not from the exit code.
#[test]
fn escape_one_used_matches_interpreter() {
    let diff = differential("escape-one-used", ESCAPE_ONE_USED);
    assert_eq!(diff.native.exit_status, 0, "{:?}", diff.native);
    assert_native_matches_interpreter("escape-one-used", &diff);
}

#[cfg(target_os = "linux")]
// Readmitted under RT-IGNORED-PASSING-ROWS, row 7 of 11.
//
// Three annotation layers struck: a byte-span block claiming the row "refuses at
// object emission, so the program never executes"; a D1a/D2 note below it saying
// the byte-span observation "was not the blocker"; and a live label claiming the
// row "next refuses because a carried recursive hypothesis is an eliminated
// value, not a callable". None reproduces. The program emits, executes, exits 0,
// and its native effect-operation sequence is [FsOpen, ResourceRelease],
// matching the interpreter's.
//
// READMITTED ON A MUTATION, NOT ON THE GREEN. Duplicating the live native
// dispatch event push in ken-host abi_v1.rs (the sole producer of the native
// effect trace, in ken_host_dispatch after dispatch_host_op_v1 returns) reds the
// canonical effect-operation-sequence assertion with the perturbation visible in
// it -- every native op appearing twice against the interpreter's single
// sequence. The interpreter side is untouched by that edit and is an independent
// producer, which is what makes the comparison an oracle rather than a
// self-check.
//
// TWO EARLIER MUTATIONS DID NOT ESTABLISH IT AND ARE RECORDED BECAUSE THEY ARE
// WHY THE THIRD IS TRUSTWORTHY. Duplicating the settlement event in
// record_resource_settlements left both rows GREEN: that is the process-exit
// finalize-all path, and these resources are released before termination, so it
// never reaches. Duplicating the live push without cloning the request FAILED TO
// COMPILE (E0382, CanonicalRequestV1 is not Copy) -- a red that is not a probe
// result, which is exactly the four-producer aggregate a colour reading cannot
// separate. Both were read from the output, not from the exit code.
#[test]
fn escape_resource_plus_plain_matches_interpreter() {
    let diff = differential("escape-res-plus-plain", ESCAPE_RESOURCE_PLUS_PLAIN);
    assert_eq!(diff.native.exit_status, 0, "{:?}", diff.native);
    assert_native_matches_interpreter("escape-res-plus-plain", &diff);
}

#[cfg(target_os = "linux")]
#[test]
// Promise class: durable native/interpreter differential. MEASURED: equal
// stdout and full effect events for the escaped FsHandle fanning source.
// CLAIMED: the linked native execution retains its observable host effects.
// THE GAP: one fixture cannot establish parity for every escaped-resource
// program; it guards this reached, effect-bearing source.
// The default libtest thread overflowed on landed main 79f44eecb with
// RUST_MIN_STACK removed. On the passing 256 MiB helper thread, a disposable
// mincore low-water probe measured 3028 KiB of touched stack (3028 KiB RSS,
// zero swap) after this row: the provision is 86.6 times that measured depth.
// This provisions the baseline, not a claim that the default stack is adequate.
fn escaped_resource_used_by_fanning_host_op_matches_interpreter() {
    in_large_stack_thread("rt-escape-fshandle-fanning", || {
        // Pre-fix: this panicked in `build_native_program` with
        // "checked Runtime frame marker was consumed more than once". The fork/union
        // of `consumed_subcontinuation_frames` per mutually-exclusive arm makes it
        // reach native execution; the assertion below pins interpreter equivalence.
        let diff = differential("escape-file-then-readat", ESCAPE_FILE_THEN_READAT);
        assert_native_matches_interpreter("escape-file-then-readat", &diff);
        assert!(
            !diff.interpreted.effect_trace.is_empty(),
            "escaped FsHandle must reach a host effect before comparing vectors"
        );
        assert_eq!(
            diff.native.stdout, diff.interpreted.stdout,
            "FsHandle stdout"
        );
        assert_eq!(
            diff.native.effect_trace, diff.interpreted.effect_trace,
            "FsHandle full effect events"
        );
    });
}

#[cfg(target_os = "linux")]
fn escape_file_then_two_reads() -> String {
    let first = "    (\\outcome. Ret (Coproduct (FSOp AFull) AmbientOp)";
    let second = r"    (\outcome. bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError ReadProgress) (ResourceBodyResult Unit Unit)
    (readAt AFull file_closed (0 : Int) buffer (MkBufferWindow (0 : Int) (6 : Int)))
    (\again. Ret (Coproduct (FSOp AFull) AmbientOp)";
    let end = "(ResourceBodyOk Unit Unit MkUnit))";
    assert_eq!(ESCAPE_FILE_THEN_READAT.matches(first).count(), 1);
    assert_eq!(ESCAPE_FILE_THEN_READAT.matches(end).count(), 1);
    let source = ESCAPE_FILE_THEN_READAT.replacen(first, second, 1).replacen(
        end,
        "(ResourceBodyOk Unit Unit MkUnit)))",
        1,
    );
    assert_eq!(source.matches("(readAt AFull file_closed").count(), 2);
    source
}

/// Promise class: normative compatibility vector for the declared invocation
/// node limit. MEASURED: this checked, effect-bearing escape/read source with
/// one extra read has a typed native Nodes(64→65) terminal and an interpreter
/// effect prefix; changing only the node limit to 65 restores full parity.
/// CLAIMED: generated allocation status reaches the owner-backed terminal.
/// THE GAP: one witness does not cover other resources or every generated edge.
#[cfg(target_os = "linux")]
#[test]
fn region_capacity_escape_second_read_reports_typed_invocation_nodes() {
    in_large_stack_thread("rt-region-capacity-fshandle", || {
        let source = escape_file_then_two_reads();
        let profile = ken_runtime::boundary_resource_profile::starter_smoke_profile();
        assert_eq!(profile.invocation.nodes, 64);
        let at_limit = differential_with_profile("region-node-64", &source, profile);
        assert_eq!(at_limit.interpreted.exit_status, 0);
        assert_eq!(
            at_limit.native.terminal_error,
            Some(ken_runtime::TerminalErrorV1::CapacityExhausted(
                ken_runtime::CapacityExhaustedV1 {
                    scope: ken_runtime::CapacityScopeV1::Invocation,
                    resource: ken_runtime::CapacityResourceV1::Nodes,
                    limit: 64,
                    requested: 65,
                }
            ))
        );
        assert!(!at_limit.native.effect_trace.is_empty());
        assert!(
            at_limit
                .interpreted
                .effect_trace
                .starts_with(&at_limit.native.effect_trace),
            "native effects up to the refusal must be the interpreter prefix"
        );

        let mut roomy = profile;
        roomy.invocation.nodes = 65;
        let at_65 = differential_with_profile("region-node-65", &source, roomy);
        assert_native_matches_interpreter("region-node-65", &at_65);
        assert_eq!(at_65.native.stdout, at_65.interpreted.stdout);
        assert_eq!(at_65.native.effect_trace, at_65.interpreted.effect_trace);
    });
}

#[cfg(target_os = "linux")]
fn region_capacity_string_lookalike() -> String {
    let source = include_str!("fixtures/rt_bracket_body_once_lookalike.ken");
    assert_eq!(source.matches("held-0.bin").count(), 1);
    source.replacen("held-0.bin", "held.bin", 1)
}

#[cfg(target_os = "linux")]
fn assert_linked_region_capacity(
    case: &str,
    source: &str,
    profile: ken_runtime::boundary_resource_profile::BoundaryResourceProfileV3,
    scope: ken_runtime::CapacityScopeV1,
    resource: ken_runtime::CapacityResourceV1,
    limit: u128,
) {
    let diff = differential_with_profile(case, source, profile);
    assert_eq!(
        diff.interpreted.exit_status, 0,
        "{case}: interpreter control"
    );
    match diff.native.terminal_error {
        Some(ken_runtime::TerminalErrorV1::CapacityExhausted(fault)) => {
            assert_eq!(fault.scope, scope, "{case}: capacity scope");
            assert_eq!(fault.resource, resource, "{case}: capacity resource");
            assert_eq!(fault.limit, limit, "{case}: declared limit");
            assert!(fault.requested > fault.limit, "{case}: actual exhaustion");
        }
        other => {
            panic!("{case}: expected emitted {scope:?}/{resource:?} exhaustion, got {other:?}")
        }
    }
}

/// Promise class: normative compatibility vector for invocation child words.
/// MEASURED: an unchanged checked two-read program with the words budget
/// tightened but nodes ample reports a typed Words fault from linked code.
/// CLAIMED: the alloc status distinguishes words from nodes on one owner.
/// THE GAP: it does not independently measure the CLIF word counter; that is
/// reconstructed by the recorder from the shared published region header.
#[cfg(target_os = "linux")]
#[test]
fn region_capacity_invocation_words_remains_distinct_from_nodes() {
    in_large_stack_thread("rt-region-words-fshandle", || {
        let mut profile = ken_runtime::boundary_resource_profile::starter_smoke_profile();
        profile.invocation.nodes = 128;
        profile.invocation.words = 1;
        assert_linked_region_capacity(
            "region-words-1",
            &escape_file_then_two_reads(),
            profile,
            ken_runtime::CapacityScopeV1::Invocation,
            ken_runtime::CapacityResourceV1::Words,
            1,
        );
    });
}

/// Promise class: normative compatibility vector for persistent node storage.
/// MEASURED: a checked, linked run reaches its persistent node limit and
/// reports the exact owner/resource without substituting invocation capacity.
/// CLAIMED: alloc routes the persistent-band owner to the existing issuer.
/// THE GAP: this fixture names one persistent allocation path, not all tags.
#[cfg(target_os = "linux")]
#[test]
fn region_capacity_persistent_nodes_are_owner_attributed() {
    in_large_stack_thread("rt-region-persistent-nodes", || {
        let mut profile = ken_runtime::boundary_resource_profile::starter_smoke_profile();
        profile.invocation.nodes = 128;
        profile.persistent.nodes = 1;
        assert_linked_region_capacity(
            "region-persistent-nodes-1",
            &escape_file_then_two_reads(),
            profile,
            ken_runtime::CapacityScopeV1::Persistent,
            ken_runtime::CapacityResourceV1::Nodes,
            1,
        );
    });
}

/// Promise class: normative compatibility vector for persistent child words.
/// MEASURED: a checked, linked run with ample persistent nodes exhausts its
/// child words and reports the persistent owner and declared word limit.
/// CLAIMED: Words and Nodes have distinct observed terminal identity.
/// THE GAP: the recorder re-reads the region header after CLIF returns -6.
#[cfg(target_os = "linux")]
#[test]
fn region_capacity_persistent_words_are_not_nodes() {
    in_large_stack_thread("rt-region-persistent-words", || {
        let mut profile = ken_runtime::boundary_resource_profile::starter_smoke_profile();
        profile.invocation.nodes = 128;
        profile.persistent.nodes = 128;
        profile.persistent.words = 1;
        assert_linked_region_capacity(
            "region-persistent-words-1",
            &region_capacity_string_lookalike(),
            profile,
            ken_runtime::CapacityScopeV1::Persistent,
            ken_runtime::CapacityResourceV1::Words,
            1,
        );
    });
}

/// Promise class: normative compatibility vector for persistent data bytes.
/// MEASURED: a checked, linked run with ample nodes/words reaches the byte
/// table capacity and reports its owner/resource/limit as a typed terminal.
/// CLAIMED: a stored literal's data status survives the generated call path.
/// THE GAP: this fixture does not reach copied ResponseBytes, the other site.
#[cfg(target_os = "linux")]
#[test]
fn region_capacity_persistent_bytes_reach_typed_terminal() {
    in_large_stack_thread("rt-region-persistent-bytes", || {
        let mut profile = ken_runtime::boundary_resource_profile::starter_smoke_profile();
        profile.invocation.nodes = 128;
        profile.persistent.nodes = 128;
        profile.persistent.words = 512;
        profile.persistent.data_bytes = 1;
        assert_linked_region_capacity(
            "region-persistent-bytes-1",
            &region_capacity_string_lookalike(),
            profile,
            ken_runtime::CapacityScopeV1::Persistent,
            ken_runtime::CapacityResourceV1::DataBytes,
            1,
        );
    });
}

#[cfg(target_os = "linux")]
fn escape_file_then_two_reads_with_wide_int() -> String {
    let source = escape_file_then_two_reads();
    let second_read = "    (\\outcome. bind (Coproduct (FSOp AFull) AmbientOp)";
    let with_wide = r"    (\outcome. let w : Int = add_int 18446744073709551615
      (match outcome {
        Ok (ReadSome span count) |-> transfer_count_int count;
        Ok ReadEof |-> (1 : Int);
        Err _ |-> (1 : Int)
      }) in
      bind (Coproduct (FSOp AFull) AmbientOp)";
    let result = "    (\\again. Ret (Coproduct (FSOp AFull) AmbientOp)";
    let with_result = r"    (\again. match eq_int w (18446744073709551621 : Int) {
      True |-> Ret (Coproduct (FSOp AFull) AmbientOp)";
    let terminal = "(ResourceBodyOk Unit Unit MkUnit)))";
    let with_terminal = r"(ResourceBodyOk Unit Unit MkUnit);
      False |-> Ret (Coproduct (FSOp AFull) AmbientOp)
        (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
        (ResourceBodyResult Unit Unit) (ResourceBodyErr Unit Unit MkUnit)
    }))";
    assert_eq!(source.matches(second_read).count(), 1);
    assert_eq!(source.matches(result).count(), 1);
    assert_eq!(source.matches(terminal).count(), 1);
    let source = source.replacen(second_read, with_wide, 1)
        .replacen(result, with_result, 1)
        .replacen(terminal, with_terminal, 1);
    assert_eq!(source.matches("(readAt AFull file_closed").count(), 2);
    source
}

/// WIP checked-program limb witness, NOT an acceptance pin yet. MEASURED:
/// deriving a wide Int from the first read's runtime transfer count before
/// the second read and comparing it afterward still returns terminal None
/// with a persistent limb limit of 1. CLAIMED: a live Big-tagged Int carried
/// across that continuation would exhaust the persistent limb table at 1→2.
/// THE GAP: the current source did not demonstrate a live Big carrier; a
/// scratch wide-arm sentinel was not reached, while Small was reached.
///
/// Invocation NativeIntLimbs is structurally unreachable by this emitted
/// path: the sole production `store_int_limbs` caller allocates a
/// `PersistentGround` node; the invocation native-Int arena is separate from
/// the boundary-region limb table, not charged to that profile cell.
#[cfg(target_os = "linux")]
#[test]
fn region_capacity_persistent_native_int_limbs_reach_typed_terminal() {
    in_large_stack_thread("rt-region-persistent-limbs", || {
        let source = escape_file_then_two_reads_with_wide_int();
        let mut profile = ken_runtime::boundary_resource_profile::starter_smoke_profile();
        profile.invocation.nodes = 128;
        profile.persistent.nodes = 128;
        profile.persistent.words = 512;
        profile.persistent.native_int_limbs = 1;
        let at_limit = differential_with_profile("region-limbs-1", &source, profile);
        assert_eq!(at_limit.interpreted.exit_status, 0, "interpreter control");
        assert_eq!(at_limit.native.terminal_error, Some(
            ken_runtime::TerminalErrorV1::CapacityExhausted(ken_runtime::CapacityExhaustedV1 {
                scope: ken_runtime::CapacityScopeV1::Persistent,
                resource: ken_runtime::CapacityResourceV1::NativeIntLimbs,
                limit: 1,
                requested: 2,
            })
        ));
        let mut roomy = profile;
        roomy.persistent.native_int_limbs = 64;
        let at_64 = differential_with_profile("region-limbs-64", &source, roomy);
        assert_native_matches_interpreter("region-limbs-64", &at_64);
        assert_eq!(at_64.native.stdout, at_64.interpreted.stdout);
        assert_eq!(at_64.native.effect_trace, at_64.interpreted.effect_trace);
    });
}

#[cfg(target_os = "linux")]
// Ignored pending RT-CARRIER-BYTESPAN-OBSERVE.
//
// Observed signature, exactly:
//   Effect: seat Argument(0) of FsReadFile needs BytesPointerLength,
//     which it cannot observe in CarriedWord
//
// Owner node: RT-CARRIER-BYTESPAN-OBSERVE.
// Pre-existing base debt, NOT a bind-order regression: this row fails at
// base 21fd46dc as well, measured by the D12 two-way differential over the
// complete --no-fail-fast surface of both packages.
// It refuses at object emission, so the program never executes and no
// binding order is observable in it.
// Its near-twin escaped_resource_used_by_fanning_host_op refuses with
// the CLOSURE-lane signature under a different owner. The names differ
// by one word; the causes differ entirely.
// Annotation only -- test body and expectations are unchanged.
#[test]
// RT-SITEOP-CARRIED-WITNESS D1a/D2: FsReadFile Argument(0) was site-bound:
// FileError SiteOperand(0) could not project its carried word. D5 byte-span
// observation was not the blocker; D2 supplies the exact emitted-helper port.
fn escaped_buffer_used_by_fanning_host_op_matches_interpreter() {
    // Promise class: durable differential for the escaped-buffer effects.
    // MEASURED: full native/interpreter parity and one read on this row.
    // CLAIMED: the checked-control Ret admission performs the effect before
    // releasing the bracket. M-admission-off makes this observation red.
    // GAP: PendingTopology FormedBasePath frames reach the base-path seat 0
    // times with a non-residual value in escape/parity/PX8; the call_tail form
    // is unpinned there and carried as unknown outside that measured corpus.
    // The escaped Buffer remains live through the nested checked Ret body.
    // Execute-then-resume must perform its read before releasing the bracket.
    let diff = differential("escape-buffer-then-readat", ESCAPE_BUFFER_THEN_READAT);
    assert_native_matches_interpreter("escape-buffer-then-readat", &diff);
    assert_eq!(diff.native.stdout, diff.interpreted.stdout, "buffer stdout");
    assert_eq!(
        diff.native.effect_trace, diff.interpreted.effect_trace,
        "buffer full events"
    );
    assert_eq!(
        diff.native
            .effect_trace
            .iter()
            .filter(|event| event.operation == ken_runtime::HostOpV1::FsReadAt)
            .count(),
        1,
        "escaped buffer must perform one FsReadAt"
    );
    // Promise class: durable emission invariant. MEASURED: a pre-filter
    // assessment recorder and the independent register-time sink recorder.
    // CLAIMED: non-Ready frames have no installed sink or D6a checked-answer
    // edge. THE GAP: the corpus never delivers an untagged checked-control
    // word to such a frame (the prior widened-install mutant had 14 reaches
    // with unchanged execution), so its runtime trap is not observed here.
    assert!(
        diff.ret_sink_assessments
            .iter()
            .any(|(_, status)| status == "PendingCheckedControl"),
        "buffer must assess a non-Ready checked-control Ret frame"
    );
    let ready = diff
        .ret_sink_assessments
        .iter()
        .filter(|(_, status)| status == "Ready")
        .map(|(origin, _)| origin.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    let installed = diff
        .ret_sink_installs
        .iter()
        .map(|sink| sink.active_frame_origin.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(
        installed, ready,
        "only Ready Ret frames install sinks: assessments={:?} installs={:?}",
        diff.ret_sink_assessments, diff.ret_sink_installs
    );
}

#[cfg(target_os = "linux")]
const NAT_FANOUT_REACHED_LIVE_RESOURCE: &str =
    include_str!("rt_nat_fanout_reached_live_resource.ken");

#[cfg(target_os = "linux")]
fn reached_nat_arm_variant(arm: &str) -> String {
    let original = match arm {
        "zero" => {
            r#"Zero |-> Ret (Coproduct (FSOp AFull) AmbientOp)
              (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
              (Result ResourceError ReadProgress) (Ok ResourceError ReadProgress ReadEof);"#
        }
        "suc" => {
            r#"Suc m |-> Ret (Coproduct (FSOp AFull) AmbientOp)
              (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
              (Result ResourceError ReadProgress) (Ok ResourceError ReadProgress ReadEof)"#
        }
        _ => panic!("unknown Nat arm: {arm}"),
    };
    let replacement = match arm {
        "zero" => {
            "Zero |-> readAt AFull file (0 : Int) buffer (MkBufferWindow (0 : Int) (6 : Int));"
        }
        "suc" => {
            "Suc m |-> readAt AFull file (0 : Int) buffer (MkBufferWindow (0 : Int) (6 : Int))"
        }
        _ => unreachable!(),
    };
    // Only the chosen arm changes. The shared continuation stays identical;
    // dropping the fanout cannot satisfy the read-count discriminator.
    let source = NAT_FANOUT_REACHED_LIVE_RESOURCE.replacen(original, replacement, 1);
    assert_ne!(
        source, NAT_FANOUT_REACHED_LIVE_RESOURCE,
        "the arm mutation must land"
    );
    source
}

#[cfg(target_os = "linux")]
// Promise class: durable native/interpreter differential. MEASURED: all three
// reached Nat fanout variants select their 2/2/3 read branches and compare
// exit, terminal class, stdout and full effect events across both executors.
// CLAIMED: a host-produced bounded Nat in a composed carried Match selects
// the structural Zero/Suc arm without losing or reordering host effects.
// THE GAP: these fixtures do not cover every future host-derived Nat source
// or scalar representation; non-Nat carried constructor families remain
// subject to their original class guard.
// Earlier baseline 52dd8640 and candidate 3cd909fd3 Nat builds overflowed
// the default test thread. This row keeps the existing 256 MiB helper; a
// disposable mincore probe after all three passing variants touched 3260 KiB
// (3260 KiB RSS, zero swap). The provision is 80.4 times that measured depth,
// not a claim of default-stack adequacy on the current candidate.
#[test]
fn nat_fanout_reached_live_resource_matches_interpreter() {
    in_large_stack_thread("rt-escape-nat-reached", || {
        for (case, source, expected_reads) in [
            (
                "nat-reached-base",
                NAT_FANOUT_REACHED_LIVE_RESOURCE.to_owned(),
                2,
            ),
            ("nat-reached-zero-extra", reached_nat_arm_variant("zero"), 2),
            ("nat-reached-suc-extra", reached_nat_arm_variant("suc"), 3),
        ] {
            let root = output_dir(case);
            std::fs::write(root.path().join("held.bin"), b"held resource").unwrap();
            let mut host = ken_interp::PosixHost::new_at(root.path());
            let interpreted = ken_cli::run_program_effect_observation(
                &source,
                ken_cli::SourceFormat::Ken,
                &[],
                &[],
                root.path().as_os_str().as_encoded_bytes(),
                &mut host,
            )
            .unwrap_or_else(|error| panic!("{case}: interpreter runs: {error:?}"));
            assert_eq!(
                interpreted
                    .effect_trace
                    .iter()
                    .filter(|event| event.operation == ken_runtime::HostOpV1::FsReadAt)
                    .count(),
                expected_reads,
                "{case}: selected Nat arm must be observable"
            );
            let diff = differential(case, &source);
            assert_native_matches_interpreter(case, &diff);
            assert_eq!(
                diff.native.stdout, diff.interpreted.stdout,
                "{case}: stdout"
            );
            assert_eq!(
                diff.native.effect_trace, diff.interpreted.effect_trace,
                "{case}: full trace"
            );
        }
    });
}

#[cfg(target_os = "linux")]
// Promise class: durable differential for the reached prefix only. The
// escaped file is already closed, so readAt returns Closed and the Nat match
// is never reached; this active row cannot claim bounded-Nat fanout parity.
// MEASURED: stdout, full effect trace, and terminal observation on both
// engines through the closed-file Err path. CLAIMED: this prefix preserves
// effects and terminal results. THE GAP: this closed-file row alone never
// observes the live-handle Nat Match; the active row above pins that route.
#[test]
fn nat_fanout_escaped_resource_matches_interpreter() {
    in_large_stack_thread("rt-escape-nat-fanout", || {
        let diff = differential("nat-fanout-escaped", NAT_FANOUT_ESCAPED_RESOURCE);
        assert_native_matches_interpreter("nat-fanout-escaped", &diff);
        assert_eq!(diff.native.stdout, diff.interpreted.stdout, "Nat stdout parity");
        assert_eq!(
            diff.native.effect_trace, diff.interpreted.effect_trace,
            "Nat complete effect-trace parity"
        );
    });
}

/// The nested three-resource R2 fixture needs a deep native stack, as the
/// oriented subcontinuation tests do.
#[cfg(target_os = "linux")]
fn in_large_stack_thread(name: &'static str, body: fn()) {
    std::thread::Builder::new()
        .name(name.to_string())
        .stack_size(256 * 1024 * 1024)
        .spawn(body)
        .unwrap()
        .join()
        .unwrap();
}

#[cfg(target_os = "linux")]
fn buffer_freeze_outcome(
    observation: &ken_runtime::EffectObservation,
) -> ken_runtime::CanonicalOutcomeV1 {
    observation
        .effect_trace
        .iter()
        .find(|event| event.operation == ken_runtime::HostOpV1::BufferFreeze)
        .map(|event| event.outcome.clone())
        .expect("the cross-buffer freeze must reach dispatch as a BufferFreeze")
}

#[cfg(target_os = "linux")]
// The protocol diagnostic and the unignored native row exercise the same
// r2 fixture. Raw relay provenance stays visible, but both marked successors
// are settlement-owned and leave the projected returned set before emission.
/// Promise class: transition sentinel for this r2 fixture's exact owner/Vis
/// members. MEASURED: raw owners 1276 and 1296 retain their exact two-member
/// provenance; successors 756 and 537 are authenticated settlements and neither
/// remains a projected returned Vis. CLAIMED: no owner emits a second returned
/// edge for a continuation consumed by its bracket. THE GAP: this plan-only
/// relation does not prove source drive; native parity separately observes it.
#[test]
fn r2_relay_owners_settle_authenticated_successors_without_returning_them() {
    in_large_stack_thread("rt-escape-r2-protocol-exclusion", || {
        let root = output_dir("r2-protocol-exclusion");
        let (compiled, diagnostics) = ken_runtime::with_static_response_feasibility_diagnostics(|| {
            ken_cli::build_native_program(
                R2_CROSS_BUFFER_FREEZE, ken_cli::SourceFormat::Ken,
                "rt_escape_r2_cross_buffer_freeze", root.path(),
                ken_runtime::boundary_resource_profile::starter_smoke_profile(),
            )
        });
        let _output = compiled.expect("r2 compiles to the current fail-closed artifact");
        assert_eq!(diagnostics.len(), 1, "one r2 response plan");
        let plan = &diagnostics[0];
        let expected = [(1276u32, 586u32, 756u32), (1296, 369, 537)];
        let observed_owners = plan.returned_vis_protocols.iter()
            .filter(|protocol| expected.iter().any(|(owner, _, _)| *owner == protocol.owner_origin))
            .map(|protocol| protocol.owner_origin)
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(observed_owners, [1276, 1296].into_iter().collect(),
            "r2's exact raw response-owner inventory must remain present");
        for (owner, relay, successor) in expected {
            let protocol = plan.returned_vis_protocols.iter()
                .find(|protocol| protocol.owner_origin == owner)
                .expect("one exact raw r2 owner");
            assert!(protocol.error.is_none() && protocol.excluded_by_relay,
                "the raw owner stays source-driven and free of return errors: {protocol:?}");
            assert_eq!(protocol.contexts.len(), 1, "one raw context per named r2 owner");
            assert_eq!(protocol.contexts[0].0, if owner == 1276 { 0 } else { 1 },
                "r2 owner {owner} moved its exact raw context");
            assert_eq!(protocol.contexts[0].1.len(), 2,
                "r2 owner {owner} must retain exactly two raw provenance members");
            let members = protocol.contexts.iter().flat_map(|(_, members)| members)
                .map(|member| (member.origin, member.relay, member.successor_id.is_none()))
                .collect::<std::collections::BTreeSet<_>>();
            assert_eq!(members, [(relay, true, true), (successor, false, true)]
                .into_iter().collect(),
                "raw r2 owner {owner} lost a named member or reintroduced a successor edge");
            assert!(plan.bracket_settlement_vis_origins.contains(&successor),
                "r2 successor {successor} must be an authenticated bracket settlement");
            assert!(plan.settled_returned_vis_protocols.iter()
                .all(|(projected_owner, _)| *projected_owner != owner),
                "settled owner {owner} retained a returned successor after consumption");
        }
        eprintln!("RT-OWNER-VIS SOURCE-DRIVEN R2 owners=[1276, 1296]");
    });
}

/// Promise class: durable invariant over the r2 fixture's planned Match children.
/// MEASURED: every persistent source-Match child with a scalar join plan under
/// nonempty process transports retains PersistentStore in its possible owners.
/// CLAIMED: a process-composed CarrierWord may still name a persistent referent
/// even if that Match's standalone join plan has no referent.
/// THE GAP: this checks the planner's allowance, not actual word ownership or
/// native execution; the positive-domain assertion prevents an empty fixture
/// from passing. Source-origin renumbering and added Match children stay green
/// when the planner preserves this relation.
#[test]
fn r2_process_carrier_domain_keeps_persistent_match_child_owners() {
    in_large_stack_thread("rt-escape-r2-process-carrier-owners", || {
        let root = output_dir("r2-process-carrier-owners");
        let (compiled, diagnostics) =
            ken_runtime::with_static_response_feasibility_diagnostics(|| {
                ken_cli::build_native_program(
                    R2_CROSS_BUFFER_FREEZE,
                    ken_cli::SourceFormat::Ken,
                    "rt_escape_r2_cross_buffer_freeze",
                    root.path(),
                    ken_runtime::boundary_resource_profile::starter_smoke_profile(),
                )
            });
        compiled.expect("r2 compiles to an artifact before inspecting its owner plan");
        assert_eq!(
            diagnostics.len(),
            1,
            "one completed r2 source ownership plan"
        );
        let plan = &diagnostics[0];
        assert!(
            !plan.pre_schema_transport_sources.is_empty(),
            "the r2 fixture must have a process-composed transport domain"
        );
        let possible_carried_children = plan
            .source_aggregate_children
            .iter()
            .filter(|child| {
                child.source_is_match
                    && child.join_is_native_scalar_pair
                    && child.child_is_persistent
            })
            .collect::<Vec<_>>();
        assert!(
            !possible_carried_children.is_empty(),
            "r2 must have a persistent scalar-planned source-Match aggregate child"
        );
        for child in possible_carried_children {
            assert!(
                child.owners.contains(
                    &ken_runtime::boundary_value::BoundaryReferentOwner::PersistentStore
                ),
                "a process-domain source-Match child must allow a persistent referent: parent={} position={} child={:?} owners={:?}",
                child.parent_origin, child.position, child.child_origin, child.owners
            );
        }
    });
}

/// Promise class: transition sentinel for the selected-call fixture shape.
/// MEASURED: the pre-schema transport source names the r2 S5→S6 call, and
/// exactly one plane owns it: the checked bracket settlement row at its
/// ResourceRelease Vis 537 (main owned it as installed owner 4, context 4).
/// Every pre-schema transport source has exactly one owner, an installed
/// owner or a bracket row, and preselection matches installed owners.
/// CLAIMED: the representation-independent source stratum retains the exact
/// call whose loss the subsequent residual-issuance increment must repair.
/// THE GAP: this does not show the call is emitted; the existing r2 execution
/// test is the separate emission gate.
#[test]
fn r2_pre_schema_response_selection_retains_selected_transport() {
    in_large_stack_thread("rt-escape-r2-pre-schema", || {
        let root = output_dir("r2-pre-schema");
        let (compiled, diagnostics) = ken_runtime::with_static_response_feasibility_diagnostics(|| {
            ken_cli::build_native_program(
                R2_CROSS_BUFFER_FREEZE, ken_cli::SourceFormat::Ken,
                "rt_escape_r2_pre_schema", root.path(),
                ken_runtime::boundary_resource_profile::starter_smoke_profile(),
            )
        });
        // I-1 does not change lowering. The existing object-emission refusal
        // remains lawful until I-2; no unrelated failure can pass this pin.
        if let Err(error) = compiled {
            assert!(format!("{error:?}").contains("no verified selected incoming call"),
                "an unrelated failure cannot prove pre-schema selection: {error:?}");
        }
        assert_eq!(diagnostics.len(), 1, "one planner must publish one stratum");
        let plan = &diagnostics[0];
        let selected_call = |identity: &str| [
            "emission_owner: Specialization(ContinuationSpecializationId(5))",
            "producer_construct_origin: StaticOriginId(537)",
            "producer_alternative: 1",
            "target: ContinuationSpecializationId(6)",
            "recursive_position: 1",
        ].iter().all(|component| identity.contains(component));
        assert!(!plan.static_response_owners.iter()
                .any(|owner| selected_call(&owner.selected_caller)),
            "the settled release must not also be an ordinary installed owner");
        let settled = plan.bracket_response_rows.iter()
            .filter(|row| selected_call(&row.k_identity)).collect::<Vec<_>>();
        let [row] = settled.as_slice() else {
            panic!("exactly one bracket row must own r2's selected call: {settled:#?}");
        };
        assert_eq!((row.vis_origin, row.k_context, row.operation.as_str()),
            (537, 4, "ResourceRelease"), "the selected call's settlement row moved");
        let selected = &row.k_identity;
        assert!(plan.pre_schema_transport_sources.contains(selected),
            "the pre-schema transport-source set lost the selected edge");
        let installed = plan.static_response_owners.iter().map(|owner| &owner.selected_caller)
            .collect::<std::collections::BTreeSet<_>>();
        let rows = plan.bracket_response_rows.iter().map(|row| &row.k_identity)
            .collect::<std::collections::BTreeSet<_>>();
        assert!(installed.is_disjoint(&rows), "one call has an installed owner and a bracket row");
        for source in &plan.pre_schema_transport_sources {
            assert!(installed.contains(source) || rows.contains(source),
                "pre-schema transport source has no owner: {source}");
        }
        let preselected = plan.preselected_response_callers.iter()
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(preselected, installed, "preselection must match installed owners by identity");
    });
}

/// Promise class: durable invariant. MEASURED: a same-buffer freeze of a
/// read span returns exactly the span's bytes natively and in the interpreter.
/// CLAIMED: the span length reaching BufferFreeze is the span's budget. THE
/// GAP: this fixture observes one six-byte span; it does not bound the
/// general conversion's cost or prove every possible span length.
#[test]
fn r2_same_buffer_freeze_returns_exact_span_bytes() {
    in_large_stack_thread("rt-escape-r2-same-buffer-freeze", || {
        let source = R2_CROSS_BUFFER_FREEZE
            .replace(
                "(\\outcome. after_read buffer_b outcome)",
                "(\\outcome. after_read buffer_a outcome)",
            )
            .replace(
                "Ok bytes |-> ResourceBodyErr Unit Unit MkUnit;",
                "Ok bytes |-> ResourceBodyOk Unit Unit MkUnit;",
            );
        assert!(
            source.contains("after_read buffer_a outcome"),
            "the control must freeze the span's own buffer"
        );
        let diff = differential("r2-same-buffer-freeze", &source);
        assert_native_matches_interpreter("r2-same-buffer-freeze", &diff);
        let expected = ken_runtime::CanonicalOutcomeV1::Success(
            ken_runtime::CanonicalReplyV1::Bytes(b"held r".to_vec()),
        );
        assert_eq!(buffer_freeze_outcome(&diff.native), expected);
        assert_eq!(buffer_freeze_outcome(&diff.interpreted), expected);
    });
}

#[test]
fn r2_cross_buffer_freeze_fails_closed_with_invalid_bounds() {
    in_large_stack_thread("rt-escape-r2", || {
        // R2 reaching lane: two nested buffer resources compile and run; a span
        // from buffer_a (length 6) applied to freeze buffer_b (capacity 2) is
        // rejected with InvalidBounds in both executors. The span length is
        // bounded by the target buffer, so this is the statically-predicted
        // bounds rejection, not a distinct BufferFreeze semantic defect.
        let diff = differential("r2-cross-buffer-freeze", R2_CROSS_BUFFER_FREEZE);
        assert_native_matches_interpreter("r2-cross-buffer-freeze", &diff);
        let expected = ken_runtime::CanonicalOutcomeV1::Error(
            ken_runtime::SemanticErrorV1::Resource(ken_runtime::ResourceErrorV1::InvalidBounds),
        );
        assert_eq!(
            buffer_freeze_outcome(&diff.native),
            expected,
            "native: cross-buffer freeze must fail closed with InvalidBounds"
        );
        assert_eq!(
            buffer_freeze_outcome(&diff.interpreted),
            expected,
            "interpreter: cross-buffer freeze must fail closed with InvalidBounds"
        );
    });
}
