//! RT-PARITY executable interpreter/native exact-variant differential.
//!
//! Each case is its own `#[test]` so that every constructible overlap reaches
//! independently: a composite driver aborts on the first failing arm and leaves
//! the later arms unexecuted, which cannot demonstrate a per-arm pre-fix ->
//! post-fix flip.
//!
//! Every *narrowing* case asserts two independent discriminators. The one
//! non-narrowing case asserts neither by design: the producer-closure case is a
//! source-scope check that runs no fixture.
//!
//! 1. **Exact variant.** The Ken fixture matches the one expected
//!    `ResourceError` constructor and exits `0`; every other error constructor
//!    takes a distinct non-zero exit. Both executors must exit `0`, so the
//!    assertion is on the exact public variant rather than on `is_err`.
//! 2. **Dispatch skip.** Narrowing now rejects at the consuming operation, so
//!    neither executor records a canonical effect event for that operation.
//!    Before the repair the interpreter substituted a sentinel and entered
//!    shared dispatch, recording an event native never had.
//!
//! Carrying both axes is what makes every *narrowing* case discriminating. On
//! the variant axis alone the `u64::MAX`-sentinel single-fault cases are
//! green-vs-green:
//! shared dispatch rejects a `u64::MAX` argument with the very same
//! `InvalidOffset`/`InvalidBounds` the repair produces, so no single-fault
//! input can separate the implementations for those consumers. The dispatch-
//! skip axis separates them anyway, because pre-fix the interpreter still
//! entered dispatch and recorded an event native never had.
//!
//! Measured pre-fix (this suite against `origin/main` interpreter production).
//! All six *narrowing* cases fail pre-fix; the one non-narrowing case is
//! deliberately fix-independent and is never cited as flip evidence:
//!
//! | Case | Pre-fix | Discriminating axis |
//! |---|---|---|
//! | `buffer_allocate_malformed_capacity` | FAILS | variant (`BufferLimit`) |
//! | `fs_read_at_malformed_offset_without_read_right` | FAILS | variant (`RightNotHeld`) |
//! | `fs_write_at_malformed_offset_without_write_right` | FAILS | variant (`RightNotHeld`) |
//! | `fs_read_at_malformed_offset` | FAILS | dispatch skip |
//! | `fs_read_at_malformed_window` | FAILS | dispatch skip |
//! | `fs_write_at_malformed_offset` | FAILS | dispatch skip |
//! | `buffer_freeze_malformed_span_is_unconstructible...` | passes | none -- source-scope pin, not interpreter behaviour |
//!
//! `BufferFreeze` has no *narrowing* case here because no malformed span is
//! constructible from checked source at the landed surface -- an empirical
//! finding, not a derived closure result, and not an omission. See
//! `buffer_freeze_malformed_span_is_unconstructible_at_the_landed_surface`
//! for exactly what that rests on and what it does not claim. Its narrowing
//! guards stay covered at the dispatch boundary in `ken-interp`.

#![cfg(target_os = "linux")]

fn output_dir(name: &str) -> tempfile::TempDir {
    let prefix = format!("ken-rt-parity-{name}-");
    tempfile::Builder::new().prefix(&prefix).tempdir().unwrap()
}

const RT_PARITY_SOURCE: &str = r#"program capabilities FS AFull
fn rt_body_ok (_buffer : BufferHandle)
  : HostIO AFull (ResourceBodyResult Unit Unit) =
  Ret (Coproduct (FSOp AFull) AmbientOp)
(resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
(ResourceBodyResult Unit Unit) (ResourceBodyOk Unit Unit MkUnit)

fn rt_expect_invalid_offset (outcome : Result ResourceError ReadProgress)
  : HostIO AFull (ResourceBodyResult Unit Unit) =
  match outcome {
Err InvalidOffset |-> Ret (Coproduct (FSOp AFull) AmbientOp)
  (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
  (ResourceBodyResult Unit Unit) (ResourceBodyOk Unit Unit MkUnit);
Err error |-> Ret (Coproduct (FSOp AFull) AmbientOp)
  (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
  (ResourceBodyResult Unit Unit) (ResourceBodyErr Unit Unit MkUnit);
Ok progress |-> Ret (Coproduct (FSOp AFull) AmbientOp)
  (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
  (ResourceBodyResult Unit Unit) (ResourceBodyErr Unit Unit MkUnit)
  }

fn rt_expect_invalid_bounds (outcome : Result ResourceError ReadProgress)
  : HostIO AFull (ResourceBodyResult Unit Unit) =
  match outcome {
Err InvalidBounds |-> Ret (Coproduct (FSOp AFull) AmbientOp)
  (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
  (ResourceBodyResult Unit Unit) (ResourceBodyOk Unit Unit MkUnit);
Err error |-> Ret (Coproduct (FSOp AFull) AmbientOp)
  (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
  (ResourceBodyResult Unit Unit) (ResourceBodyErr Unit Unit MkUnit);
Ok progress |-> Ret (Coproduct (FSOp AFull) AmbientOp)
  (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
  (ResourceBodyResult Unit Unit) (ResourceBodyErr Unit Unit MkUnit)
  }

fn rt_expect_write_invalid_offset (outcome : Result ResourceError WriteProgress)
  : HostIO AFull (ResourceBodyResult Unit Unit) =
  match outcome {
Err InvalidOffset |-> Ret (Coproduct (FSOp AFull) AmbientOp)
  (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
  (ResourceBodyResult Unit Unit) (ResourceBodyOk Unit Unit MkUnit);
Err error |-> Ret (Coproduct (FSOp AFull) AmbientOp)
  (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
  (ResourceBodyResult Unit Unit) (ResourceBodyErr Unit Unit MkUnit);
Ok progress |-> Ret (Coproduct (FSOp AFull) AmbientOp)
  (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
  (ResourceBodyResult Unit Unit) (ResourceBodyErr Unit Unit MkUnit)
  }

fn rt_bracket_done
  (outcome : Result FileError (ResourceBracketResult Unit Unit))
  : HostIO AFull ExitCode =
  match outcome {
Ok (ResourceBracketOk unit) |-> host_exit AFull Success;
Ok bracket |-> host_exit AFull (Failure 51);
Err error |-> host_exit AFull (Failure 52)
  }

fn rt_buffer_bracket_done
  (outcome : Result ResourceError (ResourceBracketResult Unit Unit))
  : HostIO AFull ExitCode =
  match outcome {
Ok (ResourceBracketOk unit) |-> host_exit AFull Success;
Ok bracket |-> host_exit AFull (Failure 53);
Err error |-> host_exit AFull (Failure 54)
  }

fn rt_inner_bracket_result
  (outcome : Result ResourceError (ResourceBracketResult Unit Unit))
  : HostIO AFull (ResourceBodyResult Unit Unit) =
  match outcome {
Ok (ResourceBracketOk unit) |-> Ret (Coproduct (FSOp AFull) AmbientOp)
  (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
  (ResourceBodyResult Unit Unit) (ResourceBodyOk Unit Unit MkUnit);
Ok bracket |-> Ret (Coproduct (FSOp AFull) AmbientOp)
  (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
  (ResourceBodyResult Unit Unit) (ResourceBodyErr Unit Unit MkUnit);
Err error |-> Ret (Coproduct (FSOp AFull) AmbientOp)
  (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
  (ResourceBodyResult Unit Unit) (ResourceBodyErr Unit Unit MkUnit)
  }

fn rt_allocate_done
  (outcome : Result ResourceError (ResourceBracketResult Unit Unit))
  : HostIO AFull ExitCode =
  match outcome {
Err InvalidBounds |-> host_exit AFull Success;
Err error |-> host_exit AFull (Failure 41);
Ok bracket |-> host_exit AFull (Failure 42)
  }

proc rt_allocate_stage (cap : Cap AFull)
  : HostIO AFull ExitCode visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
(resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
(Result ResourceError (ResourceBracketResult Unit Unit)) ExitCode
(withBuffer AFull Unit Unit (sub_int 0 1) rt_body_ok)
(\outcome. rt_allocate_done outcome)

proc rt_read_offset_body (file : Resource FsHandle) (buffer : BufferHandle)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
(resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
(Result ResourceError ReadProgress) (ResourceBodyResult Unit Unit)
(readAt AFull file (sub_int 0 1) buffer (MkBufferWindow (0 : Int) (1 : Int)))
(\outcome. rt_expect_invalid_offset outcome)

proc rt_read_offset_file (file : Resource FsHandle)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
(resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
(Result ResourceError (ResourceBracketResult Unit Unit))
(ResourceBodyResult Unit Unit)
(withBuffer AFull Unit Unit (1 : Int) (rt_read_offset_body file))
(\outcome. rt_inner_bracket_result outcome)

proc rt_read_offset_stage (cap : Cap AFull)
  : HostIO AFull ExitCode visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
(resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
(Result FileError (ResourceBracketResult Unit Unit)) ExitCode
(withResource AFull Unit Unit cap (bytes_encode "source")
  ResourceRead rt_read_offset_file)
(\outcome. rt_bracket_done outcome)

proc rt_read_window_body (file : Resource FsHandle) (buffer : BufferHandle)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
(resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
(Result ResourceError ReadProgress) (ResourceBodyResult Unit Unit)
(readAt AFull file (0 : Int) buffer
  (MkBufferWindow (sub_int 0 1) (1 : Int)))
(\outcome. rt_expect_invalid_bounds outcome)

proc rt_read_window_file (file : Resource FsHandle)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
(resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
(Result ResourceError (ResourceBracketResult Unit Unit))
(ResourceBodyResult Unit Unit)
(withBuffer AFull Unit Unit (1 : Int) (rt_read_window_body file))
(\outcome. rt_inner_bracket_result outcome)

proc rt_read_window_stage (cap : Cap AFull)
  : HostIO AFull ExitCode visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
(resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
(Result FileError (ResourceBracketResult Unit Unit)) ExitCode
(withResource AFull Unit Unit cap (bytes_encode "source")
  ResourceRead rt_read_window_file)
(\outcome. rt_bracket_done outcome)

proc rt_read_norights_stage (cap : Cap AFull)
  : HostIO AFull ExitCode visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
(resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
(Result FileError (ResourceBracketResult Unit Unit)) ExitCode
(withResource AFull Unit Unit cap (bytes_encode "sink")
  (ResourceWriteCreate CreateOrTruncate) rt_read_offset_file)
(\outcome. rt_bracket_done outcome)

proc rt_write_after_read
  (file : Resource FsHandle) (buffer : BufferHandle)
  (outcome : Result ResourceError ReadProgress)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  match outcome {
Ok (ReadSome span count) |-> bind (Coproduct (FSOp AFull) AmbientOp)
  (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
  (Result ResourceError WriteProgress) (ResourceBodyResult Unit Unit)
  (writeAt AFull file (sub_int 0 1) buffer span)
  (\written. rt_expect_write_invalid_offset written);
Ok ReadEof |-> Ret (Coproduct (FSOp AFull) AmbientOp)
  (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
  (ResourceBodyResult Unit Unit) (ResourceBodyErr Unit Unit MkUnit);
Err error |-> Ret (Coproduct (FSOp AFull) AmbientOp)
  (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
  (ResourceBodyResult Unit Unit) (ResourceBodyErr Unit Unit MkUnit)
  }

proc rt_write_body (file : Resource FsHandle) (buffer : BufferHandle)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
(resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
(Result ResourceError ReadProgress) (ResourceBodyResult Unit Unit)
(readAt AFull file (0 : Int) buffer (MkBufferWindow (0 : Int) (1 : Int)))
(\outcome. rt_write_after_read file buffer outcome)

proc rt_write_file (file : Resource FsHandle)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
(resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
(Result ResourceError (ResourceBracketResult Unit Unit))
(ResourceBodyResult Unit Unit)
(withBuffer AFull Unit Unit (1 : Int) (rt_write_body file))
(\outcome. rt_inner_bracket_result outcome)

proc rt_write_readonly_stage (cap : Cap AFull)
  : HostIO AFull ExitCode visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
(resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
(Result FileError (ResourceBracketResult Unit Unit)) ExitCode
(withResource AFull Unit Unit cap (bytes_encode "source")
  ResourceRead rt_write_file)
(\outcome. rt_bracket_done outcome)

fn rt_file_bracket_result
  (outcome : Result FileError (ResourceBracketResult Unit Unit))
  : HostIO AFull (ResourceBodyResult Unit Unit) =
  match outcome {
Ok (ResourceBracketOk unit) |-> Ret (Coproduct (FSOp AFull) AmbientOp)
  (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
  (ResourceBodyResult Unit Unit) (ResourceBodyOk Unit Unit MkUnit);
Ok bracket |-> Ret (Coproduct (FSOp AFull) AmbientOp)
  (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
  (ResourceBodyResult Unit Unit) (ResourceBodyErr Unit Unit MkUnit);
Err error |-> Ret (Coproduct (FSOp AFull) AmbientOp)
  (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
  (ResourceBodyResult Unit Unit) (ResourceBodyErr Unit Unit MkUnit)
  }

proc rt_write_pair_after
  (sink : Resource FsHandle) (buffer : BufferHandle)
  (outcome : Result ResourceError ReadProgress)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  match outcome {
Ok (ReadSome span count) |-> bind (Coproduct (FSOp AFull) AmbientOp)
  (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
  (Result ResourceError WriteProgress) (ResourceBodyResult Unit Unit)
  (writeAt AFull sink (sub_int 0 1) buffer span)
  (\written. rt_expect_write_invalid_offset written);
Ok ReadEof |-> Ret (Coproduct (FSOp AFull) AmbientOp)
  (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
  (ResourceBodyResult Unit Unit) (ResourceBodyErr Unit Unit MkUnit);
Err error |-> Ret (Coproduct (FSOp AFull) AmbientOp)
  (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
  (ResourceBodyResult Unit Unit) (ResourceBodyErr Unit Unit MkUnit)
  }

proc rt_write_pair_buffer
  (source : Resource FsHandle) (sink : Resource FsHandle)
  (buffer : BufferHandle)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
(resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
(Result ResourceError ReadProgress) (ResourceBodyResult Unit Unit)
(readAt AFull source (0 : Int) buffer (MkBufferWindow (0 : Int) (1 : Int)))
(\outcome. rt_write_pair_after sink buffer outcome)

proc rt_write_pair_sink
  (source : Resource FsHandle) (sink : Resource FsHandle)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
(resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
(Result ResourceError (ResourceBracketResult Unit Unit))
(ResourceBodyResult Unit Unit)
(withBuffer AFull Unit Unit (1 : Int) (rt_write_pair_buffer source sink))
(\outcome. rt_inner_bracket_result outcome)

proc rt_write_pair_source (cap : Cap AFull) (source : Resource FsHandle)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
(resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
(Result FileError (ResourceBracketResult Unit Unit))
(ResourceBodyResult Unit Unit)
(withResource AFull Unit Unit cap (bytes_encode "sink")
  (ResourceWriteCreate CreateOrTruncate) (rt_write_pair_sink source))
(\outcome. rt_file_bracket_result outcome)

proc rt_write_writable_stage (cap : Cap AFull)
  : HostIO AFull ExitCode visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
(resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
(Result FileError (ResourceBracketResult Unit Unit)) ExitCode
(withResource AFull Unit Unit cap (bytes_encode "source")
  ResourceRead (rt_write_pair_source cap))
(\outcome. rt_bracket_done outcome)

fn rt_cap41_expect_eof (outcome : Result ResourceError ReadProgress)
  : HostIO AFull (ResourceBodyResult Unit Unit) =
  match outcome {
  Ok ReadEof |-> Ret (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (ResourceBodyResult Unit Unit) (ResourceBodyOk Unit Unit MkUnit);
  Ok progress |-> Ret (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (ResourceBodyResult Unit Unit) (ResourceBodyErr Unit Unit MkUnit);
  Err error |-> Ret (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (ResourceBodyResult Unit Unit) (ResourceBodyErr Unit Unit MkUnit)
  }

proc rt_cap41_endpoint_buffer
  (file : Resource FsHandle) (buffer : BufferHandle)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError ReadProgress) (ResourceBodyResult Unit Unit)
    (readAt AFull file (0 : Int) buffer
      (MkBufferWindow (8 : Int) (4 : Int)))
    (\outcome. rt_cap41_expect_eof outcome)

proc rt_cap41_out_of_range_buffer
  (file : Resource FsHandle) (buffer : BufferHandle)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError ReadProgress) (ResourceBodyResult Unit Unit)
    (readAt AFull file (0 : Int) buffer
      (MkBufferWindow (9 : Int) (4 : Int)))
    (\outcome. rt_expect_invalid_bounds outcome)

proc rt_cap41_offset_endpoint_buffer
  (file : Resource FsHandle) (buffer : BufferHandle)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError ReadProgress) (ResourceBodyResult Unit Unit)
    (readAt AFull file (sub_int 0 1) buffer
      (MkBufferWindow (8 : Int) (4 : Int)))
    (\outcome. rt_expect_invalid_offset outcome)

proc rt_cap41_offset_out_of_range_buffer
  (file : Resource FsHandle) (buffer : BufferHandle)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError ReadProgress) (ResourceBodyResult Unit Unit)
    (readAt AFull file (sub_int 0 1) buffer
      (MkBufferWindow (9 : Int) (4 : Int)))
    (\outcome. rt_expect_invalid_offset outcome)

proc rt_cap41_endpoint_file (file : Resource FsHandle)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError (ResourceBracketResult Unit Unit))
    (ResourceBodyResult Unit Unit)
    (withBuffer AFull Unit Unit (8 : Int) (rt_cap41_endpoint_buffer file))
    (\outcome. rt_inner_bracket_result outcome)

proc rt_cap41_out_of_range_file (file : Resource FsHandle)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError (ResourceBracketResult Unit Unit))
    (ResourceBodyResult Unit Unit)
    (withBuffer AFull Unit Unit (8 : Int) (rt_cap41_out_of_range_buffer file))
    (\outcome. rt_inner_bracket_result outcome)

proc rt_cap41_offset_endpoint_file (file : Resource FsHandle)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError (ResourceBracketResult Unit Unit))
    (ResourceBodyResult Unit Unit)
    (withBuffer AFull Unit Unit (8 : Int) (rt_cap41_offset_endpoint_buffer file))
    (\outcome. rt_inner_bracket_result outcome)

proc rt_cap41_offset_out_of_range_file (file : Resource FsHandle)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError (ResourceBracketResult Unit Unit))
    (ResourceBodyResult Unit Unit)
    (withBuffer AFull Unit Unit (8 : Int)
      (rt_cap41_offset_out_of_range_buffer file))
    (\outcome. rt_inner_bracket_result outcome)

proc rt_cap41_endpoint_stage (cap : Cap AFull)
  : HostIO AFull ExitCode visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result FileError (ResourceBracketResult Unit Unit)) ExitCode
    (withResource AFull Unit Unit cap (bytes_encode "source")
      ResourceRead rt_cap41_endpoint_file)
    (\outcome. rt_bracket_done outcome)

proc rt_cap41_out_of_range_stage (cap : Cap AFull)
  : HostIO AFull ExitCode visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result FileError (ResourceBracketResult Unit Unit)) ExitCode
    (withResource AFull Unit Unit cap (bytes_encode "source")
      ResourceRead rt_cap41_out_of_range_file)
    (\outcome. rt_bracket_done outcome)

proc rt_cap41_offset_endpoint_stage (cap : Cap AFull)
  : HostIO AFull ExitCode visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result FileError (ResourceBracketResult Unit Unit)) ExitCode
    (withResource AFull Unit Unit cap (bytes_encode "source")
      ResourceRead rt_cap41_offset_endpoint_file)
    (\outcome. rt_bracket_done outcome)

proc rt_cap41_offset_out_of_range_stage (cap : Cap AFull)
  : HostIO AFull ExitCode visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result FileError (ResourceBracketResult Unit Unit)) ExitCode
    (withResource AFull Unit Unit cap (bytes_encode "source")
      ResourceRead rt_cap41_offset_out_of_range_file)
    (\outcome. rt_bracket_done outcome)

fn rt_uint64_checked_bounds_stage (_cap : Cap AFull)
  : HostIO AFull ExitCode =
  match intToUInt64 18446744073709551615 {
  None |-> host_exit AFull (Failure 61);
  Some bounded |-> match intToUInt64
      (add_int 18446744073709551615 1) {
    Some overflow |-> host_exit AFull (Failure 62);
    None |-> match intToUInt64 (sub_int 0 1) {
      Some negative |-> host_exit AFull (Failure 63);
      None |-> host_exit AFull Success
      }
    }
  }

proc main (_input : ProcessInput) (caps : ProgramCaps AFull)
  : HostIO AFull ExitCode visits [FS] =
  match caps {
MkProgramCaps cap |-> __RT_PARITY_ENTRY__ cap
  }
"#;

/// One case's differential outcome, used for the exact-variant assertions.
struct Differential {
    interpreted: ken_runtime::EffectObservation,
    native: ken_runtime::EffectObservation,
}

/// Compile the fixture at `entry` to a linked native artifact, run it, then run
/// the identical source through the reference interpreter against the same
/// root, and return both canonical observations.
fn differential(case: &str, entry: &str) -> Differential {
    let root = output_dir(case);
    std::fs::write(root.path().join("source"), b"ab").unwrap();
    let source = RT_PARITY_SOURCE.replace("__RT_PARITY_ENTRY__", entry);

    let output = ken_cli::build_native_program(
        &source,
        ken_cli::SourceFormat::Ken,
        &format!("rt_parity_{}", case.replace('-', "_")),
        root.path(),
    )
    .unwrap_or_else(|error| panic!("{case}: reaches linked native lowering: {error:?}"));
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

fn operation_events(
    observation: &ken_runtime::EffectObservation,
    operation: ken_runtime::HostOpV1,
) -> Vec<(
    ken_runtime::CanonicalRequestV1,
    ken_runtime::CanonicalOutcomeV1,
)> {
    observation
        .effect_trace
        .iter()
        .filter(|event| event.operation == operation)
        .map(|event| (event.request.clone(), event.outcome.clone()))
        .collect()
}

/// Assert both discriminators for one narrowing case.
///
/// `operation` is the consuming host operation whose narrowing rejects the
/// malformed argument; after the repair neither executor dispatches it.
fn assert_narrowed_alike(
    case: &str,
    entry: &str,
    operation: ken_runtime::HostOpV1,
    expected_variant: &str,
) {
    let Differential {
        interpreted,
        native,
    } = differential(case, entry);

    // Axis 1 -- exact public variant. The fixture exits 0 only on
    // `expected_variant`; any other `ResourceError` constructor exits non-zero.
    assert_eq!(
        interpreted.exit_status, 0,
        "{case}: interpreter must observe exactly {expected_variant}; got {interpreted:?}"
    );
    assert_eq!(
        native.exit_status, 0,
        "{case}: native must observe exactly {expected_variant}; got {native:?}"
    );
    assert_eq!(interpreted.terminal_error, None, "{case}: interpreter");
    assert_eq!(native.terminal_error, None, "{case}: native");
    assert_eq!(
        native.effect_trace, interpreted.effect_trace,
        "{case}: complete ordered effects, requests, outcomes, and resource provenance must agree",
    );
    assert_eq!(
        interpreted.terminal_exit, native.terminal_exit,
        "{case}: terminal exit class must agree across executors"
    );

    // Axis 2 -- dispatch skip. Narrowing rejects at the consuming operation, so
    // the malformed request never reaches shared dispatch in either executor.
    let interpreted_events = operation_events(&interpreted, operation);
    let native_events = operation_events(&native, operation);
    assert_eq!(
        interpreted_events, native_events,
        "{case}: canonical {operation:?} events must agree across executors"
    );
    assert!(
        interpreted_events.is_empty(),
        "{case}: a narrowed {operation:?} must not enter shared dispatch; got {interpreted_events:?}"
    );
}

fn in_large_stack_thread(name: &'static str, body: fn()) {
    std::thread::Builder::new()
        .name(name.to_string())
        .stack_size(256 * 1024 * 1024)
        .spawn(body)
        .expect("spawn large-stack RT-PARITY fixture")
        .join()
        .expect("RT-PARITY fixture thread");
}

// Measured on the complete seven-test generated-entry suite at exact blocked
// candidate 85b0d624: 2 MiB passes every row, while the total-population row
// aborts from stack overflow at 1 MiB. The conservative measured peak is
// therefore 2 MiB. Retaining this file's pre-existing 256 MiB provision applies
// 254 MiB of explicit headroom; this respin states the budget and does not raise
// it. Builder::stack_size makes the provision local rather than ambient.
const GENERATED_ENTRY_STACK_MEASURED_PEAK_BYTES: usize = 2 * 1024 * 1024;
const GENERATED_ENTRY_STACK_HEADROOM_BYTES: usize = 254 * 1024 * 1024;
const GENERATED_ENTRY_STACK_BYTES: usize =
    GENERATED_ENTRY_STACK_MEASURED_PEAK_BYTES + GENERATED_ENTRY_STACK_HEADROOM_BYTES;

fn in_generated_entry_stack_thread(name: &'static str, body: fn()) {
    std::thread::Builder::new()
        .name(name.to_string())
        .stack_size(GENERATED_ENTRY_STACK_BYTES)
        .spawn(body)
        .expect("spawn stated-stack generated-entry fixture")
        .join()
        .expect("generated-entry fixture thread");
}

#[test]
fn uint64_checked_wrapper_admits_max_and_rejects_both_neighbors() {
    in_large_stack_thread("uint64-checked-bounds", || {
        let Differential {
            interpreted,
            native,
        } = differential("uint64-checked-bounds", "rt_uint64_checked_bounds_stage");
        assert_eq!(
            interpreted.exit_status, 0,
            "interpreter must admit UInt64::MAX and reject both neighbors: {interpreted:?}"
        );
        assert_eq!(
            native.exit_status, 0,
            "native must admit UInt64::MAX and reject both neighbors: {native:?}"
        );
        assert_eq!(interpreted.terminal_error, None);
        assert_eq!(native.terminal_error, None);
        assert_eq!(interpreted.terminal_exit, native.terminal_exit);
    });
}

// -- BufferAllocate ------------------------------------------------------
//
// Single fault only. `BufferAllocate` consumes no resource, so no
// liveness/rights fault can coincide with the malformed capacity; the
// overlapping-fault obligation is structurally unreachable and is reported
// rather than silently dropped (frame AC-5).
//
// This consumer's pre-repair sentinel was `0`, not `u64::MAX`: a lawful
// capacity. It failed closed only because the resource table rejects a
// zero-capacity request as `BufferLimit` -- the wrong public variant, but not a
// silent success (frame AC-4).

// Ignored pending RT-CARRIER-BYTESPAN-OBSERVE.
//
// Observed signature, exactly:
//   Effect: seat Argument(0) of FsReadFile needs BytesPointerLength, which it cannot observe in CarriedWord
//
// Owner node: RT-CARRIER-BYTESPAN-OBSERVE.
// Pre-existing base debt, NOT a bind-order regression: measured failing at
// the frozen base 21fd46dc by the D10 differential, before any
// RT-SRCBODY-BIND-ORDER commit.
// It refuses at object emission, so the program never executes and no
// binding order is observable in it.
// The four px4b rows carry this same owner with the OPPOSITE provenance:
// those were branch-introduced, this one predates the branch.
// Annotation only -- test body and expectations are unchanged.
#[test]
// RT-SITEOP-CARRIED-WITNESS D1a/D2: FsReadFile Argument(0) was site-bound:
// FileError SiteOperand(0) could not project its carried word. D5 byte-span
// observation was not the blocker; D2 supplies the exact emitted-helper port.
#[ignore = "RT-SITEOP-CARRIED-WITNESS D2: the carried SiteOperand port succeeds; this row next refuses because a carried recursive hypothesis is an eliminated value, not a callable, but the call provides 1"]
fn buffer_allocate_malformed_capacity_narrows_to_invalid_bounds() {
    in_large_stack_thread("rt-parity-allocate", || {
        assert_narrowed_alike(
            "buffer-allocate-single",
            "rt_allocate_stage",
            ken_runtime::HostOpV1::BufferAllocate,
            "InvalidBounds",
        )
    });
}

// -- FsReadAt ------------------------------------------------------------

#[test]
fn checked_ih_continuation_inheritance_derives_read_and_write_independently() {
    in_large_stack_thread("rt-parity-continuation-inheritance", || {
        let (read_result, read) =
            ken_runtime::with_checked_ih_continuation_inheritance_observations(|| {
                differential("fs-read-at-offset-single", "rt_read_offset_stage")
            });
        let (write_result, write) =
            ken_runtime::with_checked_ih_continuation_inheritance_observations(|| {
                differential("fs-write-at-offset-single", "rt_write_writable_stage")
            });
        let (shifted_read_result, shifted_read) =
            ken_runtime::with_checked_ih_continuation_inheritance_mutation(
                ken_runtime::CheckedIhContinuationInheritanceMutation::InsertInterveningBinder,
                || {
                    ken_runtime::with_checked_ih_continuation_inheritance_observations(|| {
                        differential("fs-read-at-offset-single", "rt_read_offset_stage")
                    })
                },
            );
        assert!(ken_runtime::checked_ih_continuation_inheritance_mutation_is_exact());

        let select = |rows: &[ken_runtime::CheckedIhContinuationInheritanceObservation],
                      source_specialization,
                      destination_specialization,
                      active_frame_origin| {
            let found = rows
                .iter()
                .filter(|row| {
                    row.source_specialization == source_specialization
                        && row.destination_specialization == destination_specialization
                        && row.active_frame_origin == active_frame_origin
                        && row.recursive_position == 1
                })
                .collect::<Vec<_>>();
            assert_eq!(
                found.len(),
                1,
                "one exact transport/call identity must inherit to one descendant coordinate: {found:?}"
            );
            found[0].clone()
        };
        let read_target = select(&read, 1, 2, 301);
        assert_eq!(read_target.active_frame_lineage, vec![470, 301]);
        assert_eq!(read_target.destination_construct_origin, 476);
        assert_eq!(read_target.recursive_child_origin, 474);
        assert_eq!(read_target.selected_case_body_origin, 308);
        assert_eq!(read_target.invocation_origin, 305);
        assert_eq!(read_target.call_origin, 304);
        assert_eq!(read_target.callee_origin, 303);
        assert_eq!(read_target.immediate_k_locator_count, 1);
        assert_eq!(read_target.immediate_k_locator_invocation_origin, 305);
        assert_eq!(read_target.immediate_k_locator_callee_origin, 303);
        assert_eq!(
            read_target.immediate_k_locator_domain,
            "ImmediateInvocationEnvironment"
        );
        assert_eq!(read_target.immediate_k_environment_index, 0);
        assert_eq!(read_target.immediate_k_preceding_environment_provenance, None);
        assert_eq!(read_target.immediate_k_lineage_environment_indices, vec![0, 0]);
        assert_eq!(read_target.ret_case_body_origin, 465);
        assert_eq!(read_target.closure_origin, 460);
        assert_eq!(read_target.capture_ordinal, 0);
        assert_eq!(read_target.capture_occurrence, 459);
        assert_eq!(read_target.closure_body_origin, 452);
        assert_eq!(read_target.body_capture_reads, vec![450]);
        assert_eq!(read_target.closure_parameter_count, 1);
        assert!(!read_target.fresh_destination_mentions_source_result);
        assert!(read_target.ordinary_non_governed_exclusion_count > 0);

        let write_target = select(&write, 3, 5, 314);
        assert_eq!(write_target.active_frame_lineage, vec![483, 314]);
        assert_eq!(write_target.destination_construct_origin, 489);
        assert_eq!(write_target.recursive_child_origin, 487);
        assert_eq!(write_target.selected_case_body_origin, 321);
        assert_eq!(write_target.invocation_origin, 318);
        assert_eq!(write_target.call_origin, 317);
        assert_eq!(write_target.callee_origin, 316);
        assert_eq!(write_target.immediate_k_locator_count, 1);
        assert_eq!(write_target.immediate_k_locator_invocation_origin, 318);
        assert_eq!(write_target.immediate_k_locator_callee_origin, 316);
        assert_eq!(
            write_target.immediate_k_locator_domain,
            "ImmediateInvocationEnvironment"
        );
        assert_eq!(write_target.immediate_k_environment_index, 0);
        assert_eq!(write_target.immediate_k_preceding_environment_provenance, None);
        assert_eq!(write_target.immediate_k_lineage_environment_indices, vec![0, 0]);
        assert_eq!(write_target.ret_case_body_origin, 478);
        assert_eq!(write_target.closure_origin, 473);
        assert_eq!(write_target.capture_ordinal, 0);
        assert_eq!(write_target.capture_occurrence, 472);
        assert_eq!(write_target.closure_body_origin, 465);
        assert_eq!(write_target.body_capture_reads, vec![463]);
        assert_eq!(write_target.closure_parameter_count, 1);
        assert!(!write_target.fresh_destination_mentions_source_result);
        assert!(write_target.ordinary_non_governed_exclusion_count > 0);
        assert!(write_target.descriptor_only_exclusion_count > 0);

        let shifted_read_target = select(&shifted_read, 1, 2, 301);
        assert_eq!(
            shifted_read_target.source_call_identity,
            read_target.source_call_identity,
            "inserting a binder must not change transport or call identity"
        );
        assert_eq!(
            (
                shifted_read_target.destination_specialization,
                shifted_read_target.destination_body_origin,
                shifted_read_target.active_frame_origin,
                shifted_read_target.recursive_position,
                shifted_read_target.selected_case_body_origin,
                shifted_read_target.invocation_origin,
                shifted_read_target.call_origin,
                shifted_read_target.callee_origin,
                shifted_read_target.immediate_k_locator_invocation_origin,
                shifted_read_target.immediate_k_locator_callee_origin,
                shifted_read_target.immediate_k_locator_domain.as_str(),
            ),
            (
                read_target.destination_specialization,
                read_target.destination_body_origin,
                read_target.active_frame_origin,
                read_target.recursive_position,
                read_target.selected_case_body_origin,
                read_target.invocation_origin,
                read_target.call_origin,
                read_target.callee_origin,
                read_target.immediate_k_locator_invocation_origin,
                read_target.immediate_k_locator_callee_origin,
                read_target.immediate_k_locator_domain.as_str(),
            ),
            "inserting a binder must preserve semantic K identity and consumer identity"
        );
        assert_eq!(
            shifted_read_target.immediate_k_environment_index,
            read_target.immediate_k_environment_index + 1,
            "the immediate locator must re-derive past the inserted binder"
        );
        assert_eq!(
            shifted_read_target.immediate_k_preceding_environment_provenance.as_deref(),
            Some("Ordinary"),
            "the pre-shift slot must be the inserted ordinary binder, not K"
        );
        let mut expected_shifted_lineage =
            read_target.immediate_k_lineage_environment_indices.clone();
        *expected_shifted_lineage
            .last_mut()
            .expect("the governed inheritance has a final arrival") += 1;
        assert_eq!(
            shifted_read_target.immediate_k_lineage_environment_indices,
            expected_shifted_lineage,
            "only the arrival below the inserted binder may shift"
        );

        assert_ne!(
            read_target.source_call_identity, write_target.source_call_identity,
            "read and write must retain their independently issued K authority"
        );
        for rows in [&read, &write] {
            let keys = rows
                .iter()
                .map(|row| {
                    (
                        row.source_call_identity.as_str(),
                        row.destination_specialization,
                        row.active_frame_origin,
                        row.recursive_position,
                    )
                })
                .collect::<std::collections::BTreeSet<_>>();
            assert_eq!(keys.len(), rows.len(), "inheritance keys must be injective");
        }
        let write_depths = write
            .iter()
            .map(|row| row.active_frame_lineage.len())
            .collect::<std::collections::BTreeSet<_>>();
        assert!(
            write_depths.contains(&1) && write_depths.contains(&2),
            "the same typed self-resumption rule must close at varied depths: {write_depths:?}"
        );

        for (label, result, forbidden_operation) in [
            ("read", read_result, ken_runtime::HostOpV1::FsReadAt),
            (
                "shifted read",
                shifted_read_result,
                ken_runtime::HostOpV1::FsReadAt,
            ),
            ("write", write_result, ken_runtime::HostOpV1::FsWriteAt),
        ] {
            let Some(ken_runtime::TerminalErrorV1::RuntimeTrap(provenance)) =
                result.native.terminal_error.as_ref()
            else {
                panic!("{label}: planner-only relation must preserve the fail-closed product");
            };
            assert!(provenance.trap.message.ends_with("::ResourceBodyResult"));
            assert!(result
                .native
                .effect_trace
                .iter()
                .all(|event| event.operation != forbidden_operation));
        }
    });
}

/// **Promise class: transition sentinel.** A reviewed change to the fixed
/// read/write planner graph may replace its dense coordinates, but must replace
/// this witness while preserving quotient membership and exact-capsule reach.
///
/// **MEASURED:** the two fixed products' complete governed certificate classes,
/// installation state, typed direct/tail fresh-result routes, and successful
/// terminal validation observations.
/// **CLAIMED:** W0/W1 share one typed projection while context-sharing siblings
/// remain separate, every governed key is reached, and both route variants
/// retain their exact typed source, intermediate edge, and sink coordinates.
/// **THE GAP:** the fixed coordinate table is independent of the certificate
/// builder and is paired with population-side disagreement mutations below;
/// numeric origins remain transition witnesses rather than durable authority.
#[test]
fn checked_ih_generated_entry_confluence_reaches_exact_capsules() {
    in_generated_entry_stack_thread("rt-parity-generated-entry-confluence", || {
        let (read_result, read) = ken_runtime::with_checked_ih_generated_entry_observations(|| {
            differential("fs-read-at-offset-single", "rt_read_offset_stage")
        });
        let (write_result, write) =
            ken_runtime::with_checked_ih_generated_entry_observations(|| {
                differential("fs-write-at-offset-single", "rt_write_writable_stage")
            });

        assert_eq!(read.len(), 2, "read has two distinct entry coordinates");
        assert_eq!(write.len(), 3, "write has three distinct entry coordinates");
        assert_eq!(
            read.iter().map(|row| row.context).collect::<std::collections::BTreeSet<_>>().len(),
            1,
            "read context sharing must not quotient distinct coordinates"
        );
        assert_eq!(
            write.iter().map(|row| row.context).collect::<std::collections::BTreeSet<_>>().len(),
            2,
            "write context count remains unchanged"
        );
        assert_eq!(read.iter().map(|row| row.members.len()).sum::<usize>(), 2);
        assert_eq!(write.iter().map(|row| row.members.len()).sum::<usize>(), 4);

        let collision = write
            .iter()
            .find(|row| row.binding_frame_origin == 737 && row.invocation_origin == 741)
            .expect("the real W0/W1 coordinate");
        assert_eq!(collision.members.len(), 2);
        assert_ne!(collision.members[0], collision.members[1]);
        assert_eq!(collision.call_origin, 740);
        assert_eq!(collision.callee_origin, 739);
        assert_eq!(collision.locator_index, 0);
        assert_eq!(collision.locator_domain, "ImmediateInvocationEnvironment");
        assert!(
            collision
                .fresh_result_route
                .starts_with("DirectInvocationReturn"),
            "the exact body-refined invocation-return edge is the direct route: {collision:?}"
        );
        for coordinate in [
            "invocation_origin: StaticOriginId(741)",
            "call_origin: StaticOriginId(740)",
            "callee_origin: StaticOriginId(739)",
            "binding: CheckedIhBinding { frame_origin: StaticOriginId(737), recursive_position: 1 }",
        ] {
            assert!(
                collision.fresh_result_route.contains(coordinate),
                "the direct route must retain {coordinate}: {collision:?}"
            );
        }
        assert!(
            collision.reached_count > 0,
            "the real collision certificate is reused by at least one arrival"
        );

        let write_singleton = write
            .iter()
            .find(|row| row.context == collision.context && row.invocation_origin == 529)
            .expect("W2 stays separate despite sharing the context");
        assert_eq!(write_singleton.members.len(), 1);
        assert_ne!(write_singleton.callee_origin, collision.callee_origin);
        assert!(
            write_singleton
                .fresh_result_route
                .starts_with("TailProducerToRet"),
            "the Tail case must name the governed producer-to-Ret route: {write_singleton:?}"
        );
        for coordinate in [
            "invocation_origin: StaticOriginId(700)",
            "call_origin: StaticOriginId(699)",
            "callee_origin: StaticOriginId(698)",
            "active_frame_origin: StaticOriginId(696)",
            "direction: Forward",
            "ret_case_body_origin: StaticOriginId(731)",
            "ret_input_binder: ConstructorChild { frame_origin: StaticOriginId(696), field_position: 0 }",
            "ret_input_delivery: ProducerResultDirect",
        ] {
            assert!(
                write_singleton.fresh_result_route.contains(coordinate),
                "the Tail producer-to-Ret route must retain {coordinate}: {write_singleton:?}"
            );
        }
        let all_rows = read.iter().chain(&write).collect::<Vec<_>>();
        let tail_rows = all_rows
            .iter()
            .copied()
            .filter(|row| row.fresh_result_route.starts_with("TailProducerToRet"))
            .collect::<Vec<_>>();
        let direct_rows = all_rows
            .iter()
            .copied()
            .filter(|row| row.fresh_result_route.starts_with("DirectInvocationReturn"))
            .collect::<Vec<_>>();
        assert_eq!(
            tail_rows.len(),
            4,
            "the fixed products have four tail routes"
        );
        assert_eq!(
            direct_rows.len(),
            1,
            "the fixed products have one direct route"
        );
        assert_eq!(
            tail_rows
                .iter()
                .flat_map(|row| row.forward_ret_coordinates.iter())
                .map(|coordinate| &coordinate.active_frame_origin)
                .collect::<std::collections::BTreeSet<_>>()
                .len(),
            tail_rows.len(),
            "each fixed-product tail route has a distinct producer active frame"
        );
        for row in &tail_rows {
            assert_eq!(row.forward_ret_coordinates.len(), row.members.len());
            for coordinate in &row.forward_ret_coordinates {
                assert_eq!(
                    coordinate.entry_invocation_origin,
                    format!("StaticOriginId({})", row.invocation_origin)
                );
                assert_eq!(
                    coordinate.entry_call_origin,
                    format!("StaticOriginId({})", row.call_origin)
                );
                assert_eq!(
                    coordinate.entry_callee_origin,
                    format!("StaticOriginId({})", row.callee_origin)
                );
                assert_eq!(
                    coordinate.entry_binding,
                    format!(
                        "CheckedIhBinding {{ frame_origin: StaticOriginId({}), recursive_position: {} }}",
                        row.binding_frame_origin, row.binding_recursive_position
                    )
                );
                assert_ne!(
                    (
                        &coordinate.entry_binding,
                        &coordinate.entry_invocation_origin,
                        &coordinate.entry_call_origin,
                        &coordinate.entry_callee_origin,
                    ),
                    (
                        &coordinate.binding,
                        &coordinate.invocation_origin,
                        &coordinate.call_origin,
                        &coordinate.callee_origin,
                    ),
                    "Tail generated-entry E and producer source S must remain distinct"
                );
                for route_coordinate in [
                    format!("invocation_origin: {}", coordinate.invocation_origin),
                    format!("call_origin: {}", coordinate.call_origin),
                    format!("callee_origin: {}", coordinate.callee_origin),
                    format!("active_frame_origin: {}", coordinate.active_frame_origin),
                    "direction: Forward".to_string(),
                    format!("ret_input_binder: {}", coordinate.ret_input_binder),
                    "ret_input_delivery: ProducerResultDirect".to_string(),
                ] {
                    assert!(
                        row.fresh_result_route.contains(&route_coordinate),
                        "each real Tail route must retain {route_coordinate}: {row:?}"
                    );
                }
            }
        }
        for row in all_rows {
            assert!(row.installed, "every certificate key is installed: {row:?}");
            assert!(
                row.reached_count > 0,
                "every installed governed key is validated on at least one arrival: {row:?}"
            );
            assert!(row.reached_exact_capsule, "the exact recursor arm is required");
            assert!(row.reached_carried_residual, "K's residual remains Carried");
            assert_eq!(row.destination_body_origin, row.worker_body_origin);
            assert_eq!(row.invocation_origin, row.locator_invocation_origin);
            assert_eq!(row.callee_origin, row.locator_callee_origin);
        }

        for (label, result) in [("read", read_result), ("write", write_result)] {
            let Some(ken_runtime::TerminalErrorV1::RuntimeTrap(provenance)) =
                result.native.terminal_error.as_ref()
            else {
                panic!("{label}: predecessor must preserve the fail-closed product");
            };
            assert!(provenance.trap.message.ends_with("::ResourceBodyResult"));
        }
    });
}

/// **Promise class: transition sentinel.** A reviewed graph change may replace
/// these dense coordinates, but must replace the complete response-producer
/// ledger and retain one fully validated context demand per singleton K row.
///
/// **MEASURED:** the read product has four response rows and the write product
/// has seven. Their earlier `FsReadAt` rows require new contexts while the later
/// `BufferAllocate` rows reuse causal context zero; every row has one exact
/// owner ABI and selected caller.
/// **CLAIMED:** absence from the old-caller context population is a demand, not
/// SSA infeasibility. Every fixed row has one explicit capture/input schema and
/// resolves by `(ContinuationSpecializationId, worker body)` after old context
/// identities are preserved as the union prefix.
/// **THE GAP:** this fixture observes planning and ABI identity. Finished owner
/// body, exact context-call/Ret closure, and runtime InvalidOffset behavior are
/// independently pinned below. Runtime closure remains unselected.
#[test]
fn static_response_context_demand_ledger_closes_fixed_products() {
    in_generated_entry_stack_thread("rt-parity-static-response-demand", || {
        let compile = |label: &str, entry: &str| {
            let source = RT_PARITY_SOURCE.replace("__RT_PARITY_ENTRY__", entry);
            let root = output_dir(&format!("static-response-demand-{label}"));
            let (result, diagnostics) =
                ken_runtime::with_static_response_feasibility_diagnostics(|| {
                    ken_cli::build_native_program(
                        &source,
                        ken_cli::SourceFormat::Ken,
                        &format!("rt_parity_static_response_demand_{label}"),
                        root.path(),
                    )
                });
            result.expect("the fixed response-demand product must compile");
            assert_eq!(diagnostics.len(), 1, "one compile publishes one plan");
            let diagnostic = diagnostics.into_iter().next().unwrap();
            assert_eq!(diagnostic.static_response_infeasible, None);
            assert_eq!(diagnostic.all_static_response_infeasible, None);
            // RECUT 2 HS6 #2 (A)-refined (Architect evt_27hj9nxevvjyr): TOTALITY +
            // sub-case oracle. The BufferAllocate response is in EXACTLY ONE column.
            // The read product specializes it (owner + ABI, pinned below); the write
            // product's BufferAllocate K is the closure-boundary transport source, so
            // the recut correctly DEFERS it. The sub-case UnconsumedTransportCaller is
            // phase B's own correctness label (assigned ONLY to a transport source),
            // so it is the classification's correctness oracle over the fixture-known
            // population -- not a value derived from what the code emitted. The
            // Deferred member's context id is deliberately NOT pinned: it is the one
            // native value that would otherwise be an ungrounded guess.
            let specialized_buffer = &diagnostic.static_response_rows;
            let deferred_buffer = diagnostic
                .static_response_deferred
                .iter()
                .filter(|row| row.operation == "BufferAllocate")
                .collect::<Vec<_>>();
            assert_eq!(
                specialized_buffer.len() + deferred_buffer.len(),
                1,
                "exactly one BufferAllocate response, in exactly one column \
                 (Specialized={specialized_buffer:?}, Deferred={deferred_buffer:?})",
            );
            if let Some(buffer) = specialized_buffer.first() {
                assert_eq!(buffer.operation, "BufferAllocate");
                assert_eq!(
                    diagnostic
                        .all_static_response_rows
                        .iter()
                        .filter(|row| row.operation == "BufferAllocate")
                        .collect::<Vec<_>>(),
                    vec![buffer],
                    "the filtered row must be the same authority as the all-producer row"
                );
            } else {
                assert_eq!(
                    deferred_buffer[0].sub_case, "UnconsumedTransportCaller",
                    "write-BufferAllocate's K is a checked-IH environment transport source, so \
                     the recut correctly Defers it (the (a) correctness oracle)"
                );
            }
            assert_eq!(
                diagnostic.static_response_owners.len(),
                diagnostic.all_static_response_rows.len(),
                "each feasible response row owns one forward declaration"
            );
            for (ordinal, (owner, row)) in diagnostic
                .static_response_owners
                .iter()
                .zip(&diagnostic.all_static_response_rows)
                .enumerate()
            {
                assert_eq!(owner.owner as usize, ordinal);
                assert_eq!(owner.response as usize, ordinal);
                assert_eq!(owner.base_owner, row.base_owner);
                assert_eq!(owner.selected_caller, row.k_identity);
                assert_eq!(owner.k_context, row.k_context);
                assert_eq!(owner.context_was_preexisting, row.context_was_preexisting);
                assert_eq!(owner.parameters as usize, 1 + row.captures.len());
                assert_eq!(owner.captures as usize, row.continuation_inputs.len());
                assert_eq!(
                    owner.slots.len(),
                    owner.parameters as usize + owner.captures as usize + 4
                );
                for (position, (kind, slot_ordinal)) in owner.slots.iter().enumerate() {
                    let expected = if position < owner.parameters as usize {
                        ("Parameter", position as u32)
                    } else if position < (owner.parameters + owner.captures) as usize {
                        ("Capture", position as u32 - owner.parameters)
                    } else {
                        (
                            ["Result", "Control", "Trap", "Store"]
                                [position - (owner.parameters + owner.captures) as usize],
                            0,
                        )
                    };
                    assert_eq!((kind.as_str(), *slot_ordinal), expected);
                }
                assert!(owner.frame_bytes > 0);
            }
            diagnostic
        };

        let read = compile("read", "rt_read_offset_stage");
        let write = compile("write", "rt_write_writable_stage");
        fn summary(
            diagnostic: &ken_runtime::StaticResponseFeasibilityDiagnostic,
        ) -> Vec<(&str, u32, u32, u32, u32, u32, u32, u32, bool, usize, usize)> {
            diagnostic
                .all_static_response_rows
                .iter()
                .map(|row| {
                    (
                        row.operation.as_str(),
                        row.producer_call_origin,
                        row.response_origin,
                        row.vis_origin,
                        row.k_specialization,
                        row.k_closure_origin,
                        row.k_body_origin,
                        row.k_context,
                        row.context_was_preexisting,
                        row.captures.len(),
                        row.continuation_inputs.len(),
                    )
                })
                .collect::<Vec<_>>()
        }
        assert_eq!(
            summary(&read),
            vec![
                ("FsReadAt", 126, 124, 798, 0, 776, 766, 1, false, 9, 7),
                ("BufferAllocate", 138, 136, 951, 2, 947, 941, 0, true, 5, 4),
                (
                    "ResourceRelease",
                    146,
                    144,
                    465,
                    3,
                    460,
                    452,
                    2,
                    false,
                    7,
                    2
                ),
                (
                    "ResourceRelease",
                    146,
                    144,
                    676,
                    1,
                    671,
                    662,
                    3,
                    false,
                    8,
                    6
                ),
            ]
        );
        assert_eq!(
            summary(&write),
            vec![
                // RECUT 2 HS6 #2 (A)-refined: write-BufferAllocate is correctly
                // Deferred (its K is the closure-boundary transport source), so it is
                // NO LONGER a Specialized row here -- asserted in the Deferred column
                // via its UnconsumedTransportCaller sub-case above. The remaining
                // Specialized rows are unchanged: phase A mints the same has-K demand
                // population, and BufferAllocate reused causal context 0 (preexisting),
                // so deferring it removes only its owner, not any appended context id.
                ("FsWriteAt", 126, 124, 1043, 1, 993, 979, 2, false, 13, 9),
                ("FsReadAt", 139, 137, 1107, 0, 1087, 1075, 3, false, 11, 9),
                (
                    "ResourceRelease",
                    159,
                    157,
                    478,
                    6,
                    473,
                    465,
                    4,
                    false,
                    7,
                    2
                ),
                (
                    "ResourceRelease",
                    159,
                    157,
                    691,
                    4,
                    686,
                    676,
                    5,
                    false,
                    9,
                    6
                ),
                (
                    "ResourceRelease",
                    159,
                    157,
                    904,
                    2,
                    899,
                    888,
                    6,
                    false,
                    10,
                    8
                ),
                ("FsOpen", 175, 173, 1273, 5, 1265, 1259, 1, true, 5, 4),
            ]
        );

        for diagnostic in [&read, &write] {
            let mut contexts = std::collections::BTreeMap::new();
            for row in &diagnostic.all_static_response_rows {
                assert_eq!(
                    row.base_owner,
                    format!(
                        "Specialization(ContinuationSpecializationId({}))",
                        row.k_specialization
                    )
                );
                for exact in [
                    format!(
                        "producer_construct_origin: StaticOriginId({})",
                        row.vis_origin
                    ),
                    format!(
                        "target: ContinuationSpecializationId({})",
                        row.k_specialization
                    ),
                    format!("closure_origin: StaticOriginId({})", row.k_closure_origin),
                    format!("body_origin: StaticOriginId({})", row.k_body_origin),
                ] {
                    assert!(
                        row.k_identity.contains(&exact),
                        "K identity omitted {exact}"
                    );
                }
                assert_eq!(
                    contexts.insert((row.k_specialization, row.k_body_origin), row.k_context),
                    None,
                    "these fixed products contain no duplicate K demand key"
                );
                for (ordinal, capture) in row.captures.iter().enumerate() {
                    assert_eq!(capture.ordinal as usize, ordinal);
                    assert_eq!(capture.origin, row.k_closure_origin - 1 - ordinal as u32);
                    assert_eq!(capture.producer_abi_slot, 1 + ordinal as u32);
                    assert!(
                        capture.source.starts_with("ProducerLocal {")
                            || capture.source.starts_with("EntryAbi {")
                    );
                }
                for (ordinal, source, slot) in &row.continuation_inputs {
                    assert_eq!(*ordinal as usize + 1 + row.captures.len(), *slot as usize);
                    assert!(
                        source.starts_with("ProducerLocal {") || source.starts_with("EntryAbi {")
                    );
                }
            }
        }
    });
}

/// The demand mutation changes the population the union interner receives.
/// Every negative reaches once and fails before emission; exact duplication is
/// idempotent and leaves both the ledger and assigned context identities equal
/// to the unmutated compile.
#[test]
fn static_response_context_demand_controls_reach_and_restore() {
    use ken_runtime::StaticResponseContextDemandMutation as Mutation;

    in_generated_entry_stack_thread("rt-parity-static-response-demand-controls", || {
        let source = RT_PARITY_SOURCE.replace("__RT_PARITY_ENTRY__", "rt_read_offset_stage");
        let compile = |label: &str| {
            let root = output_dir(&format!("static-response-demand-control-{label}"));
            let observed = ken_runtime::with_static_response_feasibility_diagnostics(|| {
                ken_cli::build_native_program(
                    &source,
                    ken_cli::SourceFormat::Ken,
                    "rt_parity_static_response_demand_control",
                    root.path(),
                )
            });
            (root, observed)
        };
        let (baseline_root, (baseline_result, baseline)) = compile("baseline");
        let baseline_result = baseline_result.expect("baseline response-demand compile");
        let baseline_hash = baseline_result.artifact.executable_hash;
        let baseline_bytes = std::fs::read(&baseline_result.artifact.executable_path)
            .expect("baseline response-demand executable bytes");
        assert_eq!(baseline.len(), 1);

        for (label, mutation, expected) in [
            (
                "delete",
                Mutation::DeleteResponseOnlyDemand,
                "does not cover every derived response row",
            ),
            (
                "vary-k",
                Mutation::VaryKSpecialization,
                "disagrees with its fully validated response row",
            ),
            (
                "vary-body",
                Mutation::VaryKBody,
                "disagrees with its fully validated response row",
            ),
            (
                "vary-capture",
                Mutation::VaryCaptureSource,
                "disagrees with its fully validated response row",
            ),
            (
                "vary-input",
                Mutation::VaryContinuationInputSource,
                "disagrees with its K worker or input schema",
            ),
        ] {
            let ((_mutated_root, (result, diagnostics)), applications) =
                ken_runtime::with_static_response_context_demand_mutation(mutation, || {
                    compile(label)
                });
            assert_eq!(applications, 1, "{label} did not reach its demand");
            let error = result.expect_err("the reaching demand mutation must red");
            assert!(
                format!("{error:?}").contains(expected),
                "{label} failed for a different reason: {error:?}"
            );
            assert!(
                diagnostics.is_empty(),
                "a red plan must not publish a ledger"
            );
            assert!(ken_runtime::static_response_context_demand_mutation_is_exact());
            let (restored_root, (restored_result, restored)) =
                compile(&format!("{label}-restored"));
            let restored_result =
                restored_result.expect("the exact response demand must restore");
            assert_eq!(restored, baseline);
            assert_eq!(restored_result.artifact.executable_hash, baseline_hash);
            assert_eq!(
                std::fs::read(&restored_result.artifact.executable_path)
                    .expect("per-control restored response-demand bytes"),
                baseline_bytes,
                "{label}: exact byte restoration failed"
            );
            drop(restored_root);
        }

        let ((duplicate_root, (duplicate_result, duplicate)), applications) =
            ken_runtime::with_static_response_context_demand_mutation(
                Mutation::DuplicateResponseOnlyDemand,
                || compile("duplicate"),
            );
        let duplicate_result =
            duplicate_result.expect("an exact duplicate demand must reuse one context");
        assert_eq!(applications, 1);
        assert_eq!(duplicate, baseline);
        assert_eq!(duplicate_result.artifact.executable_hash, baseline_hash);
        assert_eq!(
            std::fs::read(&duplicate_result.artifact.executable_path)
                .expect("idempotent duplicate executable bytes"),
            baseline_bytes
        );
        assert!(ken_runtime::static_response_context_demand_mutation_is_exact());
        drop((duplicate_root, baseline_root));
    });
}

/// **Promise class: durable invariant.** The complete producer/K relation is
/// affine by response-row identity, preserves distinct K keys for a shared
/// producer, and retains every capture/input coordinate in exact order.
///
/// **MEASURED:** row drop/duplicate/vary, K-key merge, three response-authority
/// substitutions, and independent drop/permute/vary of every fixed READ and
/// WRITE capture/input each reach the closed demand validator and red. The
/// fixed ledgers also contain shared producers whose distinct K callers select
/// distinct owners.
/// **CLAIMED:** context interning cannot launder a malformed producer/K row or
/// an incomplete explicit input run.
/// **THE GAP:** exact duplicate context demand reuse is separately pinned by
/// `static_response_context_demand_controls_reach_and_restore`; this test moves
/// producer rows and each element of their typed population instead.
#[test]
fn static_response_full_demand_population_controls_reach_red_and_restore() {
    use ken_runtime::StaticResponseContextDemandMutation as Mutation;

    in_generated_entry_stack_thread("rt-parity-static-response-full-grid", || {
        let compile = |entry: &str, label: &str| {
            let source = RT_PARITY_SOURCE.replace("__RT_PARITY_ENTRY__", entry);
            let root = output_dir(&format!("static-response-full-grid-{entry}-{label}"));
            let observed = ken_runtime::with_static_response_feasibility_diagnostics(|| {
                ken_cli::build_native_program(
                    &source,
                    ken_cli::SourceFormat::Ken,
                    &format!("rt_parity_static_response_full_grid_{entry}"),
                    root.path(),
                )
            });
            (root, observed)
        };
        let (read_root, (read_result, read)) = compile("rt_read_offset_stage", "baseline");
        let read_result = read_result.expect("the exact READ response population compiles");
        let read_bytes = std::fs::read(&read_result.artifact.executable_path)
            .expect("READ response-grid executable bytes");
        let read_hash = read_result.artifact.executable_hash;
        let (write_root, (write_result, write)) =
            compile("rt_write_writable_stage", "baseline");
        let write_result = write_result.expect("the exact WRITE response population compiles");
        let write_bytes = std::fs::read(&write_result.artifact.executable_path)
            .expect("WRITE response-grid executable bytes");
        let write_hash = write_result.artifact.executable_hash;
        assert_eq!(read.len(), 1);
        assert_eq!(write.len(), 1);

        for (diagnostic, producer, expected_members) in [
            (&read[0], 146, 2usize),
            (&write[0], 159, 3usize),
        ] {
            let rows = diagnostic
                .all_static_response_rows
                .iter()
                .filter(|row| row.producer_call_origin == producer)
                .collect::<Vec<_>>();
            assert_eq!(rows.len(), expected_members);
            assert_eq!(
                rows.iter()
                    .map(|row| (row.k_specialization, row.k_body_origin))
                    .collect::<std::collections::BTreeSet<_>>()
                    .len(),
                expected_members,
                "shared producer {producer} merged distinct K keys"
            );
            assert_eq!(
                rows.iter()
                    .map(|row| &row.k_identity)
                    .collect::<std::collections::BTreeSet<_>>()
                    .len(),
                expected_members,
                "shared producer {producer} merged distinct incoming callers"
            );
            for row in rows {
                assert_eq!(
                    diagnostic
                        .static_response_owners
                        .iter()
                        .filter(|owner| owner.selected_caller == row.k_identity)
                        .count(),
                    1,
                    "shared producer {producer} did not split this K into one owner"
                );
            }
        }

        for (label, mutation, expected) in [
            (
                "drop-row",
                Mutation::DropProducerKRow,
                "does not cover every derived response row",
            ),
            (
                "duplicate-row",
                Mutation::DuplicateProducerKRow,
                "names no derived response row",
            ),
            (
                "vary-row",
                Mutation::VaryProducerKRow,
                "disagrees with its fully validated response row",
            ),
            (
                "merge-k",
                Mutation::MergeTwoKKeys,
                "disagrees with its fully validated response row",
            ),
            (
                "response-operation",
                Mutation::SubstituteResponseWithOperation,
                "disagrees with its fully validated response row",
            ),
            (
                "response-prior",
                Mutation::SubstituteResponseWithPriorResponse,
                "disagrees with its fully validated response row",
            ),
            (
                "response-app-env",
                Mutation::SubstituteResponseWithApplicationEnvironment,
                "disagrees with its fully validated response row",
            ),
            (
                "causal-prefix",
                Mutation::VaryCausalContextPrefix,
                "appending response context demands changed a causal context identity or schema",
            ),
        ] {
            let ((_root, (result, diagnostics)), applications) =
                ken_runtime::with_static_response_context_demand_mutation(mutation, || {
                    compile("rt_read_offset_stage", label)
                });
            assert_eq!(applications, 1, "{label}: mutation did not reach");
            let error = result.expect_err("a malformed producer/K row must red");
            assert!(
                format!("{error:?}").contains(expected),
                "{label}: wrong refusal: {error:?}"
            );
            assert!(diagnostics.is_empty(), "{label}: red plan published rows");
            assert!(ken_runtime::static_response_context_demand_mutation_is_exact());
            let (restored_root, (restored, restored_rows)) =
                compile("rt_read_offset_stage", &format!("{label}-restored"));
            let restored = restored.expect("the exact READ demand population must restore");
            assert_eq!(restored_rows, read, "{label}: restored rows changed");
            assert_eq!(restored.artifact.executable_hash, read_hash);
            assert_eq!(
                std::fs::read(&restored.artifact.executable_path)
                    .expect("per-control restored READ executable bytes"),
                read_bytes,
                "{label}: exact byte restoration failed"
            );
            drop(restored_root);
        }

        for (entry, population, cases) in [
            (
                "rt_read_offset_stage",
                (29usize, 19usize),
                [
                    ("drop-capture", Mutation::DropEveryCapture, 29usize),
                    ("permute-capture", Mutation::PermuteEveryCapture, 29usize),
                    ("vary-capture", Mutation::VaryEveryCapture, 29usize),
                    ("drop-input", Mutation::DropEveryContinuationInput, 19usize),
                    ("permute-input", Mutation::PermuteEveryContinuationInput, 19usize),
                    ("vary-input", Mutation::VaryEveryContinuationInput, 19usize),
                ],
            ),
            (
                "rt_write_writable_stage",
                (62usize, 44usize),
                [
                    ("drop-capture", Mutation::DropEveryCapture, 62usize),
                    ("permute-capture", Mutation::PermuteEveryCapture, 62usize),
                    ("vary-capture", Mutation::VaryEveryCapture, 62usize),
                    ("drop-input", Mutation::DropEveryContinuationInput, 44usize),
                    ("permute-input", Mutation::PermuteEveryContinuationInput, 44usize),
                    ("vary-input", Mutation::VaryEveryContinuationInput, 44usize),
                ],
            ),
        ] {
            assert!(population.0 > 0 && population.1 > 0);
            for (label, mutation, expected_applications) in cases {
                let ((_root, (result, diagnostics)), applications) =
                    ken_runtime::with_static_response_context_demand_mutation(mutation, || {
                        compile(entry, label)
                    });
                assert_eq!(
                    applications, expected_applications,
                    "{entry}/{label}: not every element reached independently"
                );
                let error = result.expect_err("a malformed explicit input run must red");
                assert!(
                    format!("{error:?}")
                        .contains("disagrees with its fully validated response row"),
                    "{entry}/{label}: wrong refusal: {error:?}"
                );
                assert!(diagnostics.is_empty(), "{entry}/{label}: red plan published rows");
                assert!(ken_runtime::static_response_context_demand_mutation_is_exact());
                let (restored_root, (restored, restored_rows)) =
                    compile(entry, &format!("{label}-restored"));
                let restored = restored.expect("the exact input population must restore");
                let (baseline_rows, baseline_hash, baseline_bytes) =
                    if entry == "rt_read_offset_stage" {
                        (&read, read_hash, &read_bytes)
                    } else {
                        (&write, write_hash, &write_bytes)
                    };
                assert_eq!(&restored_rows, baseline_rows);
                assert_eq!(restored.artifact.executable_hash, baseline_hash);
                assert_eq!(
                    &std::fs::read(&restored.artifact.executable_path)
                        .expect("per-control restored input-grid executable bytes"),
                    baseline_bytes,
                    "{entry}/{label}: exact byte restoration failed"
                );
                drop(restored_root);
            }
        }

        assert!(ken_runtime::static_response_context_demand_mutation_is_exact());
        drop((write_root, read_root));
    });
}

/// **Promise class: durable invariant.** Every specialized response owner has
/// exactly one planner-selected incoming identity and at least one decoded
/// finished-CLIF caller targeting that owner.
///
/// **MEASURED:** removing the incoming edge, restoring its unspecialized K
/// target, or retargeting it to a different response owner each reaches and
/// leaves the exact owner unentered.
/// **CLAIMED:** neither declaration nor another owner's call can discharge the
/// selected incoming-caller relation.
/// **THE GAP:** the controls mutate the real resolution population and the
/// unchanged whole-artifact coverage ledger supplies the refusal.
#[test]
fn static_response_selected_caller_retarget_reaches_and_restores() {
    use ken_runtime::StaticResponseCallerRetargetMutation as Mutation;

    in_generated_entry_stack_thread("rt-parity-static-response-retarget", || {
        let source =
            RT_PARITY_SOURCE.replace("__RT_PARITY_ENTRY__", "rt_read_offset_stage");
        let compile = |label: &str| {
            let root = output_dir(&format!("static-response-retarget-{label}"));
            let observed = ken_runtime::with_static_response_feasibility_diagnostics(|| {
                ken_cli::build_native_program(
                    &source,
                    ken_cli::SourceFormat::Ken,
                    "rt_parity_static_response_retarget",
                    root.path(),
                )
            });
            (root, observed)
        };

        let (baseline_root, (baseline_result, baseline)) = compile("baseline");
        let baseline_result =
            baseline_result.expect("the exact response-owner retarget must compile");
        let baseline_hash = baseline_result.artifact.executable_hash;
        let baseline_bytes = std::fs::read(&baseline_result.artifact.executable_path)
            .expect("baseline response-retarget executable bytes");
        assert_eq!(baseline.len(), 1);
        assert!(!baseline[0].static_response_owners.is_empty());

        for (label, mutation, expected) in [
            (
                "restore-k",
                Mutation::RestoreSelectedKTarget,
                "a forward-declared response owner has no verified selected incoming call",
            ),
            (
                "remove",
                Mutation::RemoveSelectedCaller,
                "removed one selected incoming caller",
            ),
            (
                "retarget-owner",
                Mutation::RetargetToDifferentResponseOwner,
                "a forward-declared response owner has no verified selected incoming call",
            ),
        ] {
            let ((_mutated_root, (mutated_result, mutated_diagnostic)), applications) =
                ken_runtime::with_static_response_caller_retarget_mutation(mutation, || {
                    compile(label)
                });
            assert!(applications > 0, "{label}: caller mutation did not reach");
            let error = mutated_result.expect_err("a caller mutation must leave an owner unentered");
            assert!(
                format!("{error:?}").contains(expected),
                "{label}: caller mutation failed for a different reason: {error:?}"
            );
            assert_eq!(
                mutated_diagnostic, baseline,
                "{label}: caller mutation altered the typed planner ledger"
            );
            assert!(ken_runtime::static_response_caller_retarget_mutation_is_exact());
            let (restored_root, (restored_result, restored)) =
                compile(&format!("{label}-restored"));
            let restored_result =
                restored_result.expect("the response-owner retarget must restore");
            assert_eq!(restored, baseline);
            assert_eq!(restored_result.artifact.executable_hash, baseline_hash);
            assert_eq!(
                std::fs::read(&restored_result.artifact.executable_path)
                    .expect("per-control restored response-retarget bytes"),
                baseline_bytes,
                "{label}: exact byte restoration failed"
            );
            drop(restored_root);
        }
        drop(baseline_root);
    });
}

/// **Promise class: durable invariant.** A specialized response-owner body
/// emits exactly one finished-CLIF call to its exact K context, after host
/// validation and before exact Ret collapse, with status and Trap checked before
/// Result. Only that K-call Result reaches the owner Result slot.
///
/// **MEASURED:** context-zero/raw-worker substitution, three wrong response
/// authorities, raw HostResult escape, K-call omission/duplication/reordering,
/// Trap bypass, and Ret variation each alter one real emitted owner and red in
/// the unchanged finished-function verifier. Exact rebuilds restore diagnostics,
/// hashes, and executable bytes for both fixed products.
/// **CLAIMED:** declaration shape cannot stand in for the emitted owner/context/
/// call/Ret relation, and no response/control/environment value can escape in
/// place of the Trap-checked K result.
/// **THE GAP:** the verifier decodes direct callees and reads instruction/value
/// order from finalized CLIF; response provenance itself remains the typed
/// owner-local construction that the three substitution controls vary.
#[test]
fn static_response_owner_body_controls_reach_red_and_restore() {
    use ken_runtime::StaticResponseOwnerBodyMutation as Mutation;

    in_generated_entry_stack_thread("rt-parity-static-response-owner-grid", || {
        let compile = |entry: &str, label: &str| {
            let source = RT_PARITY_SOURCE.replace("__RT_PARITY_ENTRY__", entry);
            let root = output_dir(&format!("static-response-owner-grid-{entry}-{label}"));
            let observed = ken_runtime::with_static_response_feasibility_diagnostics(|| {
                ken_cli::build_native_program(
                    &source,
                    ken_cli::SourceFormat::Ken,
                    &format!("rt_parity_static_response_owner_grid_{entry}"),
                    root.path(),
                )
            });
            (root, observed)
        };
        let (read_root, (read_result, read_rows)) =
            compile("rt_read_offset_stage", "baseline");
        let read_result = read_result.expect("the exact READ response owners compile");
        let read_bytes = std::fs::read(&read_result.artifact.executable_path)
            .expect("READ response-owner executable bytes");
        let read_hashes = (
            read_result.plan_transport_hash,
            read_result.runtime_program.core_semantic_hash,
            read_result.runtime_program.artifact_hash,
            read_result.artifact.executable_hash,
        );
        let (write_root, (write_result, write_rows)) =
            compile("rt_write_writable_stage", "baseline");
        let write_result = write_result.expect("the exact WRITE response owners compile");
        let write_bytes = std::fs::read(&write_result.artifact.executable_path)
            .expect("WRITE response-owner executable bytes");
        let write_hashes = (
            write_result.plan_transport_hash,
            write_result.runtime_program.core_semantic_hash,
            write_result.runtime_program.artifact_hash,
            write_result.artifact.executable_hash,
        );
        let app486 = write_rows[0]
            .all_static_response_rows
            .iter()
            .filter(|row| row.operation == "BufferAllocate")
            .collect::<Vec<_>>();
        assert_eq!(app486.len(), 1);
        assert_eq!(
            (app486[0].k_closure_origin, app486[0].k_body_origin),
            (1246, 1238),
            "the application-environment control must target app486's exact K"
        );

        for (label, entry, mutation, expected) in [
            (
                "context-zero",
                "rt_read_offset_stage",
                Mutation::SubstituteContextZero,
                "called a context or raw worker other than its exact K context",
            ),
            (
                "response-operation",
                "rt_read_offset_stage",
                Mutation::ResponseWithOperation,
                "substituted operation, prior-response, or application-environment authority",
            ),
            (
                "response-prior",
                "rt_read_offset_stage",
                Mutation::ResponseWithPriorResponse,
                "substituted operation, prior-response, or application-environment authority",
            ),
            (
                "response-app486-environment",
                "rt_write_writable_stage",
                Mutation::ResponseWithApplicationEnvironment,
                "substituted operation, prior-response, or application-environment authority",
            ),
            (
                "raw-host-result",
                "rt_read_offset_stage",
                Mutation::RawHostResultEscape,
                "raw HostResult or non-K value escape",
            ),
            (
                "raw-worker",
                "rt_read_offset_stage",
                Mutation::CallRawWorker,
                "called a context or raw worker other than its exact K context",
            ),
            (
                "omit-k-call",
                "rt_read_offset_stage",
                Mutation::OmitKCall,
                "emitted 0 K calls instead of exactly one",
            ),
            (
                "duplicate-k-call",
                "rt_read_offset_stage",
                Mutation::DuplicateKCall,
                "emitted 2 K calls instead of exactly one",
            ),
            (
                "before-host-validation",
                "rt_read_offset_stage",
                Mutation::CallBeforeHostValidation,
                "called K before host response validation completed",
            ),
            (
                "after-answer-collapse",
                "rt_read_offset_stage",
                Mutation::CallAfterAnswerCollapse,
                "called K after its answer was already collapsed",
            ),
            (
                "trap-bypass",
                "rt_read_offset_stage",
                Mutation::BypassTrapBeforeResult,
                "without the status then Trap-before-Result branches",
            ),
            (
                "vary-ret",
                "rt_read_offset_stage",
                Mutation::VaryRet,
                "validated a Ret identity other than its exact K Ret",
            ),
            (
                "omit-owner-definition",
                "rt_read_offset_stage",
                Mutation::OmitOwnerDefinition,
                "the response-owner body population is incomplete",
            ),
        ] {
            let ((_root, (result, diagnostics)), applications) =
                ken_runtime::with_static_response_owner_body_mutation(mutation, || {
                    compile(entry, label)
                });
            assert_eq!(applications, 1, "{label}: owner mutation did not reach");
            let error = result.expect_err("a malformed response-owner body must red");
            assert!(
                format!("{error:?}").contains(expected),
                "{label}: wrong finished-body refusal: {error:?}"
            );
            let baseline = if entry == "rt_read_offset_stage" {
                &read_rows
            } else {
                &write_rows
            };
            assert_eq!(
                &diagnostics, baseline,
                "{label}: body mutation changed the typed planner population"
            );
            assert!(ken_runtime::static_response_owner_body_mutation_is_exact());
            let (restored_root, (restored, restored_rows)) =
                compile(entry, &format!("{label}-restored"));
            let restored = restored.expect("the exact response-owner body must restore");
            let (baseline_rows, baseline_hashes, baseline_bytes) =
                if entry == "rt_read_offset_stage" {
                    (&read_rows, read_hashes, &read_bytes)
                } else {
                    (&write_rows, write_hashes, &write_bytes)
                };
            assert_eq!(&restored_rows, baseline_rows);
            assert_eq!(
                (
                    restored.plan_transport_hash,
                    restored.runtime_program.core_semantic_hash,
                    restored.runtime_program.artifact_hash,
                    restored.artifact.executable_hash,
                ),
                baseline_hashes,
                "{label}: restored owner artifact hashes changed"
            );
            assert_eq!(
                &std::fs::read(&restored.artifact.executable_path)
                    .expect("per-control restored response-owner bytes"),
                baseline_bytes,
                "{label}: exact byte restoration failed"
            );
            drop(restored_root);
        }

        drop((write_root, read_root));
    });
}

/// **Promise class: durable invariant.** Intended planner growth may add Direct
/// arrivals, but every such arrival must retain one source-keyed declared call
/// and use that call's Trap-checked result rather than its capture environment.
///
/// **MEASURED:** the write fixture's exact governed Direct application records
/// its invocation/call/callee provenance, a non-degenerate planner-ordered
/// capture run, one
/// emitted call and a call-derived result; the Tail-only read fixture records no
/// Direct application.
/// **CLAIMED:** Direct applies its carried environment exactly once while Tail
/// cannot enter the Direct lookup and app486 remains an environment-only
/// zero-call materializer.
/// **THE GAP:** the observation is emitted at the actual declared-call seam.
/// The population mutations below independently vary every joining operand and
/// compare restored executable bytes.
#[test]
fn checked_ih_direct_application_pairs_one_declared_call_result() {
    in_generated_entry_stack_thread("rt-parity-direct-application-pairing", || {
        let compile = |label: &str, entry: &str| {
            let source = RT_PARITY_SOURCE.replace("__RT_PARITY_ENTRY__", entry);
            let root = output_dir(&format!("direct-application-pairing-{label}"));
            let (result, observations, applications) =
                ken_runtime::with_checked_ih_direct_application_mutation(
                    ken_runtime::CheckedIhDirectApplicationMutation::Exact,
                    || {
                        ken_cli::build_native_program(
                            &source,
                            ken_cli::SourceFormat::Ken,
                            &format!("rt_parity_direct_application_pairing_{label}"),
                            root.path(),
                        )
                    },
                );
            result.expect("the exact Direct application fixture must compile");
            (observations, applications)
        };

        let (read, read_applications) = compile("read", "rt_read_offset_stage");
        let (write, write_applications) = compile("write", "rt_write_writable_stage");
        assert!(
            read.is_empty() && read_applications == 0,
            "the Tail-only read fixture must not enter the Direct application lookup: {read:#?}"
        );
        assert!(
            !write.is_empty(),
            "the write fixture must reach its governed Direct application"
        );
        assert_eq!(write_applications, write.len());
        for row in &write {
            assert_eq!(row.invocation_origin, "StaticOriginId(741)");
            assert_eq!(row.application_origin, "StaticOriginId(740)");
            assert_eq!(row.callee_origin, "StaticOriginId(739)");
            assert!(
                row.capture_count > 1,
                "the capture-order control requires a non-degenerate Direct population: {row:#?}"
            );
            assert_eq!(row.emitted_call_count, 1);
            assert!(row.emitted_call.is_some());
            assert!(row.application_result_from_call);
            assert!(
                !row.application_origin.contains("486"),
                "app486 remains environment-only and cannot be the Direct call seat"
            );
        }
        assert!(ken_runtime::checked_ih_direct_application_mutation_is_exact());
    });
}

/// **Promise class: durable invariant.** Every joining operand of the Direct
/// application is population-controlled at the production seam.
///
/// **MEASURED:** call removal, a neighboring declared transport identity,
/// capture permutation, capture removal and environment-for-result substitution
/// each reach one real Direct arrival and return their exact refusal.
/// **CLAIMED:** one validated Direct arrival can only assemble the planner-
/// ordered capture run, select its transport's declared call once and use the
/// emitted call's Result.
/// **THE GAP:** each mutation moves the population-side operand named by the
/// claim, leaves the detector fixed, proves application provenance, then an
/// independent exact rebuild restores hashes and executable bytes to the
/// baseline.
#[test]
fn checked_ih_direct_application_population_controls_refuse_and_restore() {
    in_generated_entry_stack_thread("rt-parity-direct-application-controls", || {
        use ken_runtime::CheckedIhDirectApplicationMutation as Mutation;

        let source = RT_PARITY_SOURCE.replace("__RT_PARITY_ENTRY__", "rt_write_writable_stage");
        let build = |label: &str, mutation: Mutation| {
            let root = output_dir(&format!("direct-application-control-{label}"));
            let (result, observations, applications) =
                ken_runtime::with_checked_ih_direct_application_mutation(mutation, || {
                    ken_cli::build_native_program(
                        &source,
                        ken_cli::SourceFormat::Ken,
                        "rt_parity_direct_application_control",
                        root.path(),
                    )
                });
            (root, result, observations, applications)
        };

        let (baseline_root, baseline, baseline_rows, baseline_applications) =
            build("baseline", Mutation::Exact);
        let baseline = baseline.expect("the exact Direct application must compile");
        assert!(!baseline_rows.is_empty());
        assert_eq!(baseline_applications, baseline_rows.len());
        let baseline_bytes = std::fs::read(&baseline.artifact.executable_path)
            .expect("baseline Direct executable bytes");
        let baseline_hashes = (
            baseline.plan_transport_hash,
            baseline.runtime_program.core_semantic_hash,
            baseline.runtime_program.artifact_hash,
            baseline.artifact.executable_hash,
        );

        for (label, mutation, expected, expected_calls) in [
            (
                "drop-call",
                Mutation::DropCall,
                "Direct application emitted zero calls for one governed arrival",
                0,
            ),
            (
                "vary-transport",
                Mutation::VaryTransportIdentity,
                "Direct declared-call lookup varied away from the selected transport identity",
                0,
            ),
            (
                "permute-captures",
                Mutation::PermuteCaptures,
                "continuation call's independently assembled WorkerCapture suffix",
                0,
            ),
            (
                "drop-capture",
                Mutation::DropCapture,
                "Direct captured environment has a missing planner-ordered capture",
                0,
            ),
            (
                "environment-for-result",
                Mutation::EnvironmentForResult,
                "substituted its captured environment for the emitted call Result",
                1,
            ),
        ] {
            let (_mutated_root, mutated, rows, applications) = build(label, mutation);
            let error = mutated.expect_err("a malformed Direct relation must refuse");
            let rendered = format!("{error:?}");
            assert!(
                rendered.contains(expected),
                "{label}: wrong refusal arm; error={rendered}; rows={rows:#?}"
            );
            assert_eq!(applications, 1, "{label}: mutation missed Direct");
            assert_eq!(rows.len(), 1, "{label}: missing application provenance");
            assert_eq!(rows[0].invocation_origin, "StaticOriginId(741)");
            assert_eq!(rows[0].application_origin, "StaticOriginId(740)");
            assert_eq!(rows[0].callee_origin, "StaticOriginId(739)");
            assert_eq!(rows[0].emitted_call_count, expected_calls);
            assert_eq!(rows[0].emitted_call.is_some(), expected_calls == 1);
            assert!(!rows[0].application_result_from_call);
            assert!(ken_runtime::checked_ih_direct_application_mutation_is_exact());

            let (restored_root, restored, restored_rows, restored_applications) =
                build(&format!("{label}-restored"), Mutation::Exact);
            let restored = restored.expect("the exact Direct relation must restore");
            assert_eq!(restored_rows, baseline_rows);
            assert_eq!(restored_applications, baseline_applications);
            assert_eq!(
                (
                    restored.plan_transport_hash,
                    restored.runtime_program.core_semantic_hash,
                    restored.runtime_program.artifact_hash,
                    restored.artifact.executable_hash,
                ),
                baseline_hashes,
                "{label}: exact restoration changed an artifact hash"
            );
            assert_eq!(
                std::fs::read(&restored.artifact.executable_path)
                    .expect("restored Direct executable bytes"),
                baseline_bytes,
                "{label}: exact restoration changed executable bytes"
            );
            drop(restored_root);
        }
        drop(baseline_root);
    });
}

/// **Promise class: transition sentinel.** The observation is stated over the
/// still-emitted pre-D3 edge pairing rather than fixed Cranelift block/value
/// numbers; D3 retires this observer when it activates the new forward edge.
///
/// **MEASURED:** one selected governed tail route records the result delivered
/// to its unambiguous source-machine resumption seat, that same value on the
/// active self-resumption jump, the target header's input parameter, and that
/// same header input directly installed in the exact Ret body environment, in
/// forward emission order.
/// **CLAIMED:** the pre-D3 emitted tail route is a directed value-flow edge
/// rather than four co-emitted endpoints.
/// **THE GAP:** Cranelift identities are diagnostic only. The feature-only
/// observation coordinate intentionally preserves the old emitted-path
/// controls without becoming a sibling production plan; D2's move-only proof
/// separately owns the future source/sink authority and makes no live-edge
/// claim. `CoEmissionOnly` preserves its landed aggregate control;
/// the substantive leg inventory derives one identity-only arm per predicate
/// leg, so no conjunct borrows another conjunct's negative observation.
#[test]
fn checked_ih_fresh_result_route_observation_is_forward_and_paired() {
    in_generated_entry_stack_thread("rt-parity-fresh-result-route-pairing", || {
        use ken_runtime::CheckedIhFreshResultRouteObservationMutation as Mutation;

        let (exact_result, exact) =
            ken_runtime::with_checked_ih_fresh_result_route_emission_observations(
                Mutation::Exact,
                || differential("fs-write-at-offset-single", "rt_write_writable_stage"),
            );
        assert!(
            !exact.is_empty(),
            "the governed tail-route population must emit"
        );
        let paired = |row: &ken_runtime::CheckedIhFreshResultRouteEmissionObservation| {
            row.is_forward_and_paired()
        };
        assert!(
            exact.iter().all(paired),
            "every governed tail route must be value-paired in forward order: {exact:#?}"
        );

        let (coemitted_result, coemitted) =
            ken_runtime::with_checked_ih_fresh_result_route_emission_observations(
                Mutation::CoEmissionOnly,
                || differential("fs-write-at-offset-single", "rt_write_writable_stage"),
            );
        assert_eq!(
            coemitted.len(),
            exact.len(),
            "the control must preserve the selected route population"
        );
        assert!(
            coemitted.iter().all(|row| {
                row.source_emitted
                    && row.active_edge_value.is_some()
                    && row.header_input_value.is_some()
                    && row.ret_input_value.is_some()
                    && !paired(row)
            }),
            "co-emitting every seat without value identity must not satisfy pairing: {coemitted:#?}"
        );
        assert_eq!(
            exact_result.native.effect_trace, coemitted_result.native.effect_trace,
            "observer-only pairing suppression must change no emitted behavior"
        );
        assert_eq!(
            exact_result.native.terminal_error, coemitted_result.native.terminal_error,
            "observer-only pairing suppression must preserve the terminal frontier"
        );

        for leg in ken_runtime::CheckedIhFreshResultRoutePairingLeg::ALL
            .iter()
            .copied()
        {
            let (controlled_result, controlled) =
                ken_runtime::with_checked_ih_fresh_result_route_emission_observations(
                    Mutation::PairingLegOnly(leg),
                    || differential("fs-write-at-offset-single", "rt_write_writable_stage"),
                );
            assert_eq!(
                controlled.len(),
                exact.len(),
                "the {leg:?} control must preserve the selected route population"
            );
            assert!(
                controlled
                    .iter()
                    .all(|row| row.pairing_seats_are_coemitted()),
                "the {leg:?} control must preserve every optional seat: {controlled:#?}"
            );
            assert!(
                controlled.iter().all(|row| !row.pairing_leg_holds(leg)),
                "the {leg:?} control must break its named identity: {controlled:#?}"
            );
            assert!(
                controlled.iter().all(|row| {
                    ken_runtime::CheckedIhFreshResultRoutePairingLeg::ALL
                        .iter()
                        .copied()
                        .filter(|other| *other != leg)
                        .all(|other| row.pairing_leg_holds(other))
                }),
                "the {leg:?} control must preserve every sibling identity: {controlled:#?}"
            );
            assert!(
                controlled.iter().all(|row| !paired(row)),
                "deleting the {leg:?} conjunct must expose this control: {controlled:#?}"
            );
            assert_eq!(
                exact_result.native.effect_trace, controlled_result.native.effect_trace,
                "the {leg:?} observer control must change no emitted behavior"
            );
            assert_eq!(
                exact_result.native.terminal_error, controlled_result.native.terminal_error,
                "the {leg:?} observer control must preserve the terminal frontier"
            );
        }
    });
}

/// **Promise class: transition sentinel.** A reviewed change to the fixed
/// read/write fixture's static occurrence graph must replace this exact P/G/N
/// witness while preserving totality, disjointness, and the per-arrival laws.
///
/// **MEASURED:** the complete planner-derived admission-key sets for both fixed
/// product witnesses, including every explicit `NonGoverned` row.
/// **CLAIMED:** the sanitized map is total over P rather than a governed sample.
/// **THE GAP:** expected rows come from the independently fixed witness table,
/// not from re-projecting the map under test.
#[test]
fn checked_ih_generated_entry_admission_population_is_total() {
    in_generated_entry_stack_thread("rt-parity-generated-entry-admissions", || {
        let (read_result, read) =
            ken_runtime::with_checked_ih_generated_entry_admission_observations(|| {
                differential("fs-read-at-offset-single", "rt_read_offset_stage")
            });
        let (write_result, write) =
            ken_runtime::with_checked_ih_generated_entry_admission_observations(|| {
                differential("fs-write-at-offset-single", "rt_write_writable_stage")
            });
        let keys = |rows: &[ken_runtime::CheckedIhGeneratedEntryAdmissionObservation]| {
            rows.iter()
                .map(|row| {
                    (
                        row.enclosing_specialization,
                        row.worker_body_origin,
                        row.binding_frame_origin,
                        row.binding_recursive_position,
                        row.invocation_origin,
                        row.call_origin,
                        row.callee_origin,
                        row.governed,
                    )
                })
                .collect::<std::collections::BTreeSet<_>>()
        };
        let expected_read = std::collections::BTreeSet::from([
            (2, 941, 301, 1, 305, 304, 303, true),
            (2, 941, 470, 1, 474, 473, 472, false),
            (2, 941, 511, 1, 515, 514, 513, true),
            (2, 941, 681, 1, 685, 684, 683, false),
        ]);
        let expected_write = std::collections::BTreeSet::from([
            (3, 1238, 525, 1, 529, 528, 527, true),
            (3, 1238, 696, 1, 700, 699, 698, false),
            (3, 1238, 737, 1, 741, 740, 739, true),
            (3, 1238, 909, 1, 913, 912, 911, false),
            (5, 1259, 314, 1, 318, 317, 316, true),
            (5, 1259, 483, 1, 487, 486, 485, false),
            (5, 1259, 525, 1, 529, 528, 527, false),
            (5, 1259, 696, 1, 700, 699, 698, false),
            (5, 1259, 737, 1, 741, 740, 739, false),
            (5, 1259, 909, 1, 913, 912, 911, false),
        ]);
        assert_eq!(keys(&read), expected_read, "the read admission population P is closed");
        assert_eq!(keys(&write), expected_write, "the write admission population P is closed");
        for row in read.iter().chain(&write) {
            assert!(row.installed);
            assert_eq!(row.installation_count, 1);
            assert_eq!(
                row.raw_arrival_count, row.admission_outcome_count,
                "every raw arrival performs exactly one total-map lookup: {row:?}"
            );
            if row.governed {
                assert!(row.raw_arrival_count > 0, "every governed key is reached: {row:?}");
                assert_eq!(
                    row.raw_arrival_count, row.governed_validation_count,
                    "every governed arrival completes one full validation: {row:?}"
                );
                assert_eq!(row.ordinary_continuation_count, 0);
            } else {
                assert_eq!(row.governed_validation_count, 0);
                assert_eq!(
                    row.raw_arrival_count, row.ordinary_continuation_count,
                    "every NonGoverned arrival continues ordinary dispatch once: {row:?}"
                );
            }
        }
        for result in [read_result, write_result] {
            assert!(matches!(
                result.native.terminal_error,
                Some(ken_runtime::TerminalErrorV1::RuntimeTrap(_))
            ));
        }
    });
}

/// **Promise class: durable invariant.**
///
/// **MEASURED:** substituting the Tail producer source for the final
/// generated-entry arrival at `checked_ih_generated_entry_row` preserves the
/// closed call-coordinate population but complements every governed bit in the
/// non-degenerate read fixture.
/// **CLAIMED:** producer source `S` cannot replace generated-entry arrival `E`
/// as the P/G/N access discriminator.
/// **THE GAP:** the mutation moves the production-side row key and arrival,
/// while the exact comparison comes from a separately compiled unmutated plan;
/// requiring identical key populations and opposite classifications proves the
/// control reached this regression rather than an unrelated build failure.
#[test]
fn checked_ih_generated_entry_arrival_cannot_be_replaced_by_producer_source() {
    in_generated_entry_stack_thread("rt-parity-generated-entry-e-from-s", || {
        use ken_runtime::CheckedIhGeneratedEntryConfluenceMutation as Mutation;

        let observe = |label: &str, mutation: Mutation| {
            let source = RT_PARITY_SOURCE.replace("__RT_PARITY_ENTRY__", "rt_read_offset_stage");
            let root = output_dir(&format!("generated-entry-e-from-s-{label}"));
            let (result, rows) =
                ken_runtime::with_checked_ih_generated_entry_confluence_mutation(mutation, || {
                    ken_runtime::with_checked_ih_generated_entry_admission_observations(|| {
                        ken_cli::build_native_program(
                            &source,
                            ken_cli::SourceFormat::Ken,
                            &format!("rt_parity_generated_entry_e_from_s_{label}"),
                            root.path(),
                        )
                    })
                });
            result.expect("the compile-preserving E-from-S control must build");
            rows.into_iter()
                .map(|row| {
                    (
                        (
                            row.enclosing_specialization,
                            row.worker_body_origin,
                            row.binding_frame_origin,
                            row.binding_recursive_position,
                            row.invocation_origin,
                            row.call_origin,
                            row.callee_origin,
                        ),
                        row.governed,
                    )
                })
                .collect::<std::collections::BTreeMap<_, _>>()
        };

        let exact = observe("exact", Mutation::Exact);
        let substituted = observe("substituted", Mutation::EntryFromRouteSource);
        assert!(
            exact.len() > 1,
            "the control fixture must be non-degenerate"
        );
        assert_eq!(
            exact.keys().collect::<Vec<_>>(),
            substituted.keys().collect::<Vec<_>>(),
            "E-from-S must preserve P while changing its governed partition"
        );
        assert!(
            exact
                .iter()
                .all(|(key, governed)| substituted.get(key) == Some(&!*governed)),
            "E-from-S did not reproduce the admission-classification inversion"
        );
        assert!(ken_runtime::checked_ih_generated_entry_confluence_mutation_is_exact());
    });
}

const GENERATED_ENTRY_ARRIVAL_MUTATION_CHILD: &str =
    "KEN_RT_CHECKED_IH_GENERATED_ENTRY_ARRIVAL_MUTATION_CHILD";

fn assert_generated_entry_arrival_mutation_child() {
    use ken_runtime::CheckedIhGeneratedEntryArrivalMutation as Mutation;

    let mode = std::env::var(GENERATED_ENTRY_ARRIVAL_MUTATION_CHILD)
        .expect("generated-entry arrival mutation child mode");
    let mutation = match mode.as_str() {
        "duplicate-lookup" => Mutation::DuplicateLookup,
        "skip-lookup" => Mutation::SkipLookup,
        "duplicate-validation" => Mutation::DuplicateGovernedValidation,
        "skip-validation" => Mutation::SkipGovernedValidation,
        "governed-through-non-governed" => Mutation::GovernedThroughNonGoverned,
        "non-governed-through-governed" => Mutation::NonGovernedThroughGoverned,
        other => panic!("unknown generated-entry arrival mutation: {other}"),
    };
    let root = output_dir(&format!("generated-entry-arrival-{mode}"));
    let source = RT_PARITY_SOURCE.replace("__RT_PARITY_ENTRY__", "rt_read_offset_stage");
    let (build, rows) = ken_runtime::with_checked_ih_generated_entry_admission_observations(|| {
        ken_runtime::with_checked_ih_generated_entry_arrival_mutation(mutation, || {
            ken_cli::build_native_program(
                &source,
                ken_cli::SourceFormat::Ken,
                &format!("rt_parity_generated_entry_arrival_{}", mode.replace('-', "_")),
                root.path(),
            )
        })
    });
    assert!(
        rows.iter().any(|row| row.installed && row.raw_arrival_count > 0),
        "{mode}: mutation never reached the production arrival seam"
    );
    let mismatch = rows.iter().any(|row| match mode.as_str() {
        "duplicate-lookup" => row.admission_outcome_count > row.raw_arrival_count,
        "skip-lookup" => row.admission_outcome_count < row.raw_arrival_count,
        "duplicate-validation" => {
            row.governed && row.governed_validation_count > row.raw_arrival_count
        }
        "skip-validation" => {
            row.governed && row.governed_validation_count < row.raw_arrival_count
        }
        "governed-through-non-governed" => {
            row.governed
                && row.ordinary_continuation_count > 0
                && row.governed_validation_count < row.raw_arrival_count
        }
        "non-governed-through-governed" => {
            !row.governed && row.ordinary_continuation_count < row.raw_arrival_count
        }
        _ => unreachable!(),
    });
    assert!(
        mismatch,
        "{mode}: operation-path mutation did not break the per-key equality; build_ok={} rows={rows:#?}",
        build.is_ok()
    );
    assert!(
        ken_runtime::checked_ih_generated_entry_arrival_mutation_is_exact(),
        "{mode}: scoped arrival mutation state did not restore"
    );
}

/// **Promise class: durable invariant.** Intended extensions may change how
/// many times a static call arrives, but must keep one lookup and one sealed
/// continuation action paired with every arrival.
///
/// **MEASURED:** independently incremented raw-arrival, map-lookup outcome,
/// governed-validation, and ordinary-continuation counts per installed key.
/// **CLAIMED:** operation count, not today's arrival magnitude, is exactly one
/// per arrival for both sealed variants.
/// **THE GAP:** each control therefore mutates the real operation/control path,
/// never the observation counters, and must make the corresponding relation
/// unequal.

macro_rules! generated_entry_case {
    ($name:ident, $env:ident, $runner:ident, $child:ident, $mode:literal) => {
        #[test]
        fn $name() {
            if std::env::var_os($env).is_some() {
                $runner(concat!("rt-parity-", stringify!($name), "-child"), $child);
                return;
            }
            let output = std::process::Command::new(std::env::current_exe().expect("test binary"))
                .arg("--exact").arg(stringify!($name)).arg("--nocapture")
                .env($env, $mode).env_remove("RUST_MIN_STACK").output()
                .expect("spawn isolated mutation child");
            assert!(output.status.success(), "{}: mutation child failed\nstdout:\n{}\nstderr:\n{}", $mode, String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr));
        }
    };
}

macro_rules! generated_entry_checked_case {
    ($name:ident, $env:ident, $runner:ident, $child:ident, $mode:literal, $expected:literal) => {
        #[test]
        fn $name() {
            if std::env::var_os($env).is_some() {
                $runner(concat!("rt-parity-", stringify!($name), "-child"), $child);
                return;
            }
            let output = std::process::Command::new(std::env::current_exe().expect("test binary"))
                .arg("--exact").arg(stringify!($name)).arg("--nocapture")
                .env($env, $mode).env_remove("RUST_MIN_STACK").output()
                .expect("spawn isolated mutation child");
            assert!(output.status.success(), "{}: mutation child failed\nstdout:\n{}\nstderr:\n{}", $mode, String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr));
            let stderr = String::from_utf8_lossy(&output.stderr);
            assert!(stderr.contains($expected), "{}: mutation missed intended arm; stderr:\n{}", $mode, stderr);
        }
    };
}

macro_rules! generated_entry_split_checked_case {
    ($name:ident, $mode:literal, $expected:literal, $reached:literal, $target:literal) => {
        #[test]
        fn $name() {
            if std::env::var_os(GENERATED_ENTRY_CAPSULE_MUTATION_CHILD).is_some() {
                in_generated_entry_stack_thread(
                    concat!("rt-parity-", stringify!($name), "-child"),
                    assert_generated_entry_capsule_mutation_child,
                );
                return;
            }
            let output = std::process::Command::new(
                std::env::current_exe().expect("test binary"),
            )
            .arg("--exact")
            .arg(stringify!($name))
            .arg("--nocapture")
            .env(GENERATED_ENTRY_CAPSULE_MUTATION_CHILD, $mode)
            .env_remove("RUST_MIN_STACK")
            .output()
            .expect("spawn isolated projection-control child");
            assert!(
                output.status.success(),
                "{}: mutation child failed\nstdout:\n{}\nstderr:\n{}",
                $mode,
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
            let stderr = String::from_utf8_lossy(&output.stderr);
            assert!(
                stderr.contains($expected),
                "{}: mutation missed intended arm; stderr:\n{}",
                $mode,
                stderr
            );
            assert!(
                stderr.contains($reached),
                "{}: mutation did not reach its selected semantic layer; stderr:\n{}",
                $mode,
                stderr
            );
            let restored = format!(
                "RT_CHECKED_IH_PUBLISHED_PROJECTION_CONTROL_RESTORED_GREEN target={} mode={}",
                $target, $mode
            );
            assert!(
                stderr.contains(&restored),
                "{}: restored target did not report GREEN; stderr:\n{}",
                $mode,
                stderr
            );
        }
    };
}

macro_rules! d1_route_case {
    ($name:ident, $mode:literal, $control:expr, $recursor:expr) => {
        #[test]
        fn $name() {
            if std::env::var_os(D1_ROUTE_CONTROL_CHILD).is_some() {
                in_large_stack_thread(concat!("rt-parity-", stringify!($name), "-child"), assert_d1_route_control_child);
                return;
            }
            let mut child = std::process::Command::new(std::env::current_exe().expect("test binary"));
            child.arg("--exact").arg(stringify!($name)).arg("--nocapture")
                .env(D1_ROUTE_CONTROL_CHILD, $mode)
                .env_remove("KEN_RT_ITREE_D1_ROUTE_CONTROL")
                .env_remove("KEN_RT_ITREE_D1_RECURSOR_ROUTE")
                .env_remove("RUST_MIN_STACK");
            let control: Option<&str> = $control;
            let recursor: Option<&str> = $recursor;
            if let Some(control) = control { child.env("KEN_RT_ITREE_D1_ROUTE_CONTROL", control); }
            if let Some(recursor) = recursor { child.env("KEN_RT_ITREE_D1_RECURSOR_ROUTE", recursor); }
            let output = child.output().expect("spawn isolated D1 control child");
            assert!(output.status.success(), "{} child failed\nstdout:\n{}\nstderr:\n{}", $mode, String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr));
            let stderr = String::from_utf8_lossy(&output.stderr);
            if let Some(control) = control { assert!(stderr.contains(&format!("RT_ITREE_D1_CONTROL_APPLIED mode={control}")), "{}: the route-control mutation did not reach its real producer: {}", $mode, stderr); }
            if let Some(recursor) = recursor { assert!(stderr.contains(&format!("RT_ITREE_D1_RECURSOR_APPLIED mode={recursor}")), "{}: the recursor-route mutation did not reach its real producer: {}", $mode, stderr); }
        }
    };
}

generated_entry_case!(generated_entry_arrival_duplicate_lookup, GENERATED_ENTRY_ARRIVAL_MUTATION_CHILD, in_generated_entry_stack_thread, assert_generated_entry_arrival_mutation_child, "duplicate-lookup");
generated_entry_case!(generated_entry_arrival_skip_lookup, GENERATED_ENTRY_ARRIVAL_MUTATION_CHILD, in_generated_entry_stack_thread, assert_generated_entry_arrival_mutation_child, "skip-lookup");
generated_entry_case!(generated_entry_arrival_duplicate_validation, GENERATED_ENTRY_ARRIVAL_MUTATION_CHILD, in_generated_entry_stack_thread, assert_generated_entry_arrival_mutation_child, "duplicate-validation");
generated_entry_case!(generated_entry_arrival_skip_validation, GENERATED_ENTRY_ARRIVAL_MUTATION_CHILD, in_generated_entry_stack_thread, assert_generated_entry_arrival_mutation_child, "skip-validation");
generated_entry_case!(generated_entry_arrival_governed_through_non_governed, GENERATED_ENTRY_ARRIVAL_MUTATION_CHILD, in_generated_entry_stack_thread, assert_generated_entry_arrival_mutation_child, "governed-through-non-governed");
generated_entry_case!(generated_entry_arrival_non_governed_through_governed, GENERATED_ENTRY_ARRIVAL_MUTATION_CHILD, in_generated_entry_stack_thread, assert_generated_entry_arrival_mutation_child, "non-governed-through-governed");

const GENERATED_ENTRY_ADMISSION_MUTATION_CHILD: &str =
    "KEN_RT_CHECKED_IH_GENERATED_ENTRY_ADMISSION_MUTATION_CHILD";

fn assert_generated_entry_admission_mutation_child() {
    use ken_runtime::CheckedIhGeneratedEntryAdmissionMutation as Mutation;

    let mode = std::env::var(GENERATED_ENTRY_ADMISSION_MUTATION_CHILD)
        .expect("generated-entry admission mutation child mode");
    let mutation = match mode.as_str() {
        "drop-governed" => Mutation::DropGoverned,
        "drop-non-governed" => Mutation::DropNonGoverned,
        "duplicate-governed" => Mutation::DuplicateGoverned,
        "duplicate-non-governed" => Mutation::DuplicateNonGoverned,
        "governed-to-non-governed" => Mutation::GovernedToNonGoverned,
        "non-governed-to-governed" => Mutation::NonGovernedToGoverned,
        "governed-key-collision" => Mutation::GovernedProjectedCollision,
        "non-governed-key-collision" => Mutation::NonGovernedProjectedCollision,
        other => panic!("unknown generated-entry admission mutation: {other}"),
    };
    let red = ken_runtime::with_checked_ih_generated_entry_admission_mutation(mutation, || {
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            differential("fs-write-at-offset-single", "rt_write_writable_stage")
        }))
    });
    assert!(red.is_err(), "{mode}: admission population mutation did not redden");
    assert!(
        ken_runtime::checked_ih_generated_entry_admission_mutation_is_exact(),
        "{mode}: scoped admission mutation state did not restore"
    );
}

// **Promise class: durable invariant.** Intended planner extensions may grow
// P, but every row must still have one explicit, disjoint Governed/NonGoverned
// classification and governed call-key projection must remain functional.
//
// **MEASURED:** drop, duplicate, cross-variant reclassification, and projected
// collision mutations applied to the real admission population.
// **CLAIMED:** the six closed-partition laws reject both variants at their
// named planner-validation arms.
// **THE GAP:** the child asserts the exact error text for each arm, so an
// earlier unrelated rejection cannot masquerade as admission validation.
generated_entry_checked_case!(generated_entry_admission_drop_governed, GENERATED_ENTRY_ADMISSION_MUTATION_CHILD, in_generated_entry_stack_thread, assert_generated_entry_admission_mutation_child, "drop-governed", "total generated-entry admission keys are not equal to the closed call population");
generated_entry_checked_case!(generated_entry_admission_drop_non_governed, GENERATED_ENTRY_ADMISSION_MUTATION_CHILD, in_generated_entry_stack_thread, assert_generated_entry_admission_mutation_child, "drop-non-governed", "total generated-entry admission keys are not equal to the closed call population");
generated_entry_checked_case!(generated_entry_admission_duplicate_governed, GENERATED_ENTRY_ADMISSION_MUTATION_CHILD, in_generated_entry_stack_thread, assert_generated_entry_admission_mutation_child, "duplicate-governed", "one Governed generated-entry admission key was inserted twice");
generated_entry_checked_case!(generated_entry_admission_duplicate_non_governed, GENERATED_ENTRY_ADMISSION_MUTATION_CHILD, in_generated_entry_stack_thread, assert_generated_entry_admission_mutation_child, "duplicate-non-governed", "one NonGoverned generated-entry admission key was inserted twice");
generated_entry_checked_case!(generated_entry_admission_governed_to_non_governed, GENERATED_ENTRY_ADMISSION_MUTATION_CHILD, in_generated_entry_stack_thread, assert_generated_entry_admission_mutation_child, "governed-to-non-governed", "governed generated-entry admission keys are not equal to the projected governed set");
generated_entry_checked_case!(generated_entry_admission_non_governed_to_governed, GENERATED_ENTRY_ADMISSION_MUTATION_CHILD, in_generated_entry_stack_thread, assert_generated_entry_admission_mutation_child, "non-governed-to-governed", "governed generated-entry admission keys are not equal to the projected governed set");
generated_entry_checked_case!(generated_entry_admission_governed_key_collision, GENERATED_ENTRY_ADMISSION_MUTATION_CHILD, in_generated_entry_stack_thread, assert_generated_entry_admission_mutation_child, "governed-key-collision", "two governed coordinates project one call key to different typed projections");
generated_entry_checked_case!(generated_entry_admission_non_governed_key_collision, GENERATED_ENTRY_ADMISSION_MUTATION_CHILD, in_generated_entry_stack_thread, assert_generated_entry_admission_mutation_child, "non-governed-key-collision", "one NonGoverned generated-entry admission key was inserted twice or overlapped Governed");

const GENERATED_ENTRY_MUTATION_CHILD: &str =
    "KEN_RT_CHECKED_IH_GENERATED_ENTRY_MUTATION_CHILD";

fn assert_generated_entry_mutation_child() {
    use ken_runtime::CheckedIhGeneratedEntryConfluenceMutation as Mutation;

    let mode = std::env::var(GENERATED_ENTRY_MUTATION_CHILD)
        .expect("generated-entry mutation child mode");
    let mutation = match mode.as_str() {
        "context-key" => Mutation::ContextOnlyKey,
        "identity-key" => Mutation::SourceIdentityInKey,
        "projection-key" => Mutation::ProjectionInKey,
        "destination-owner" => Mutation::DestinationOwner,
        "destination-body" => Mutation::DestinationBody,
        "binding-frame" => Mutation::BindingFrame,
        "binding-position" => Mutation::BindingPosition,
        "locator-invocation" => Mutation::LocatorInvocation,
        "locator-callee" => Mutation::LocatorCallee,
        "locator-domain" => Mutation::LocatorDomain,
        "locator-index" => Mutation::LocatorIndex,
        "fresh-active-frame" => Mutation::FreshActiveFrame,
        "fresh-ret-body" => Mutation::FreshRetBody,
        "fresh-constructor-role" => Mutation::FreshConstructorRole,
        "fresh-constructor-coordinate" => Mutation::FreshConstructorCoordinate,
        "fresh-closure-record" => Mutation::FreshClosureRecord,
        "fresh-closure-origin" => Mutation::FreshClosureOrigin,
        "fresh-closure-body" => Mutation::FreshClosureBody,
        "fresh-closure-parameters" => Mutation::FreshClosureParameterCount,
        "fresh-capture-ordinal" => Mutation::FreshCaptureOrdinal,
        "fresh-capture-occurrence" => Mutation::FreshCaptureOccurrence,
        "fresh-body-reads" => Mutation::FreshBodyReadMembership,
        "route-removal" => Mutation::RouteRemoval,
        "route-duplication" => Mutation::RouteDuplication,
        "route-cross-variant" => Mutation::RouteCrossVariant,
        "route-wrong-active-frame" => Mutation::RouteWrongActiveFrame,
        "route-wrong-selected-case" => Mutation::RouteWrongSelectedCase,
        "route-wrong-direct-edge" => Mutation::RouteWrongDirectEdge,
        "route-wrong-ret-input-body" => Mutation::RouteWrongRetInputBody,
        "route-wrong-ret-input-binder" => Mutation::RouteWrongRetInputBinder,
        "route-wrong-governed-key" => Mutation::RouteWrongGovernedKey,
        "route-wrong-delivery" => Mutation::RouteWrongDelivery,
        "route-reversed" => Mutation::RouteReversed,
        "route-disagreement" => Mutation::RouteDisagreement,
        "remove-member" => Mutation::RemoveFirstMember,
        "duplicate-member" => Mutation::DuplicateFirstMember,
        "filter-member" => Mutation::FilterCollidingMember,
        other => panic!("unknown generated-entry mutation: {other}"),
    };
    let red = ken_runtime::with_checked_ih_generated_entry_confluence_mutation(mutation, || {
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            differential("fs-write-at-offset-single", "rt_write_writable_stage")
        }))
    });
    assert!(red.is_err(), "{mode}: mutation did not redden");
    assert!(
        ken_runtime::checked_ih_generated_entry_confluence_mutation_is_exact(),
        "{mode}: scoped mutation state did not restore"
    );
}

// **Promise class: durable invariant.** Every quotient-key weakening,
// projection disagreement, route corruption, and set-membership corruption
// must reject before a certificate can be published.
//
// **MEASURED:** each population-side mutation changes one real route relation,
// membership operation, declared-body transport selector, or legal neighboring
// case/frame/binder/key and reaches its named planner refusal.
// **CLAIMED:** the governed projection carries exactly one directed route; its
// Direct arm preserves the body-refined transport, and its Tail arm composes
// source, selected case, active frame, and producer-result-direct Ret sink.
// **THE GAP:** both same-variant positives live in
// `checked_ih_generated_entry_confluence_reaches_exact_capsules`; these
// mutation children establish rejection, not positive reach. Neighboring
// bodies, frames, binders, and keys are drawn from validated planner rows.
generated_entry_checked_case!(generated_entry_confluence_context_key, GENERATED_ENTRY_MUTATION_CHILD, in_generated_entry_stack_thread, assert_generated_entry_mutation_child, "context-key", "disagree on their typed consumer projection");
generated_entry_checked_case!(generated_entry_confluence_identity_key, GENERATED_ENTRY_MUTATION_CHILD, in_generated_entry_stack_thread, assert_generated_entry_mutation_child, "identity-key", "not equal as sets");
generated_entry_checked_case!(generated_entry_confluence_projection_key, GENERATED_ENTRY_MUTATION_CHILD, in_generated_entry_stack_thread, assert_generated_entry_mutation_child, "projection-key", "not equal as sets");
generated_entry_checked_case!(generated_entry_confluence_destination_owner, GENERATED_ENTRY_MUTATION_CHILD, in_generated_entry_stack_thread, assert_generated_entry_mutation_child, "destination-owner", "disagree on their typed consumer projection");
generated_entry_checked_case!(generated_entry_confluence_destination_body, GENERATED_ENTRY_MUTATION_CHILD, in_generated_entry_stack_thread, assert_generated_entry_mutation_child, "destination-body", "disagree on their typed consumer projection");
generated_entry_checked_case!(generated_entry_confluence_binding_frame, GENERATED_ENTRY_MUTATION_CHILD, in_generated_entry_stack_thread, assert_generated_entry_mutation_child, "binding-frame", "disagree on their typed consumer projection");
generated_entry_checked_case!(generated_entry_confluence_binding_position, GENERATED_ENTRY_MUTATION_CHILD, in_generated_entry_stack_thread, assert_generated_entry_mutation_child, "binding-position", "disagree on their typed consumer projection");
generated_entry_checked_case!(generated_entry_confluence_locator_invocation, GENERATED_ENTRY_MUTATION_CHILD, in_generated_entry_stack_thread, assert_generated_entry_mutation_child, "locator-invocation", "disagree on their typed consumer projection");
generated_entry_checked_case!(generated_entry_confluence_locator_callee, GENERATED_ENTRY_MUTATION_CHILD, in_generated_entry_stack_thread, assert_generated_entry_mutation_child, "locator-callee", "disagree on their typed consumer projection");
generated_entry_checked_case!(generated_entry_confluence_locator_domain, GENERATED_ENTRY_MUTATION_CHILD, in_generated_entry_stack_thread, assert_generated_entry_mutation_child, "locator-domain", "disagree on their typed consumer projection");
generated_entry_checked_case!(generated_entry_confluence_locator_index, GENERATED_ENTRY_MUTATION_CHILD, in_generated_entry_stack_thread, assert_generated_entry_mutation_child, "locator-index", "disagree on their typed consumer projection");
generated_entry_checked_case!(generated_entry_confluence_fresh_active_frame, GENERATED_ENTRY_MUTATION_CHILD, in_generated_entry_stack_thread, assert_generated_entry_mutation_child, "fresh-active-frame", "disagree on their typed consumer projection");
generated_entry_checked_case!(generated_entry_confluence_fresh_ret_body, GENERATED_ENTRY_MUTATION_CHILD, in_generated_entry_stack_thread, assert_generated_entry_mutation_child, "fresh-ret-body", "disagree on their typed consumer projection");
generated_entry_checked_case!(generated_entry_confluence_fresh_constructor_role, GENERATED_ENTRY_MUTATION_CHILD, in_generated_entry_stack_thread, assert_generated_entry_mutation_child, "fresh-constructor-role", "disagree on their typed consumer projection");
generated_entry_checked_case!(generated_entry_confluence_fresh_constructor_coordinate, GENERATED_ENTRY_MUTATION_CHILD, in_generated_entry_stack_thread, assert_generated_entry_mutation_child, "fresh-constructor-coordinate", "disagree on their typed consumer projection");
generated_entry_checked_case!(generated_entry_confluence_fresh_closure_record, GENERATED_ENTRY_MUTATION_CHILD, in_generated_entry_stack_thread, assert_generated_entry_mutation_child, "fresh-closure-record", "disagree on their typed consumer projection");
generated_entry_checked_case!(generated_entry_confluence_fresh_closure_origin, GENERATED_ENTRY_MUTATION_CHILD, in_generated_entry_stack_thread, assert_generated_entry_mutation_child, "fresh-closure-origin", "disagree on their typed consumer projection");
generated_entry_checked_case!(generated_entry_confluence_fresh_closure_body, GENERATED_ENTRY_MUTATION_CHILD, in_generated_entry_stack_thread, assert_generated_entry_mutation_child, "fresh-closure-body", "disagree on their typed consumer projection");
generated_entry_checked_case!(generated_entry_confluence_fresh_closure_parameters, GENERATED_ENTRY_MUTATION_CHILD, in_generated_entry_stack_thread, assert_generated_entry_mutation_child, "fresh-closure-parameters", "disagree on their typed consumer projection");
generated_entry_checked_case!(generated_entry_confluence_fresh_capture_ordinal, GENERATED_ENTRY_MUTATION_CHILD, in_generated_entry_stack_thread, assert_generated_entry_mutation_child, "fresh-capture-ordinal", "disagree on their typed consumer projection");
generated_entry_checked_case!(generated_entry_confluence_fresh_capture_occurrence, GENERATED_ENTRY_MUTATION_CHILD, in_generated_entry_stack_thread, assert_generated_entry_mutation_child, "fresh-capture-occurrence", "disagree on their typed consumer projection");
generated_entry_checked_case!(generated_entry_confluence_fresh_body_reads, GENERATED_ENTRY_MUTATION_CHILD, in_generated_entry_stack_thread, assert_generated_entry_mutation_child, "fresh-body-reads", "disagree on their typed consumer projection");
generated_entry_checked_case!(generated_entry_confluence_route_removal, GENERATED_ENTRY_MUTATION_CHILD, in_generated_entry_stack_thread, assert_generated_entry_mutation_child, "route-removal", "governed fresh-result route population is absent");
generated_entry_checked_case!(generated_entry_confluence_route_duplication, GENERATED_ENTRY_MUTATION_CHILD, in_generated_entry_stack_thread, assert_generated_entry_mutation_child, "route-duplication", "governed fresh-result route population is ambiguous");
generated_entry_checked_case!(generated_entry_confluence_route_cross_variant, GENERATED_ENTRY_MUTATION_CHILD, in_generated_entry_stack_thread, assert_generated_entry_mutation_child, "route-cross-variant", "route variant contradicts its exact direct-transport partition");
generated_entry_checked_case!(generated_entry_confluence_route_wrong_active_frame, GENERATED_ENTRY_MUTATION_CHILD, in_generated_entry_stack_thread, assert_generated_entry_mutation_child, "route-wrong-active-frame", "route active frame is not the exact governed frame");
generated_entry_checked_case!(generated_entry_confluence_route_wrong_selected_case, GENERATED_ENTRY_MUTATION_CHILD, in_generated_entry_stack_thread, assert_generated_entry_mutation_child, "route-wrong-selected-case", "disconnected from its selected recursive case");
generated_entry_checked_case!(generated_entry_confluence_route_wrong_direct_edge, GENERATED_ENTRY_MUTATION_CHILD, in_generated_entry_stack_thread, assert_generated_entry_mutation_child, "route-wrong-direct-edge", "direct fresh-result route's declared recursive-unit body has no exact typed invocation transport");
generated_entry_checked_case!(generated_entry_confluence_route_wrong_ret_input_body, GENERATED_ENTRY_MUTATION_CHILD, in_generated_entry_stack_thread, assert_generated_entry_mutation_child, "route-wrong-ret-input-body", "route does not name the exact Ret-input body");
generated_entry_checked_case!(generated_entry_confluence_route_wrong_ret_input_binder, GENERATED_ENTRY_MUTATION_CHILD, in_generated_entry_stack_thread, assert_generated_entry_mutation_child, "route-wrong-ret-input-binder", "route does not name the exact logical Ret-input binder");
generated_entry_checked_case!(generated_entry_confluence_route_wrong_governed_key, GENERATED_ENTRY_MUTATION_CHILD, in_generated_entry_stack_thread, assert_generated_entry_mutation_child, "route-wrong-governed-key", "route does not name its governed call key");
generated_entry_checked_case!(generated_entry_confluence_route_wrong_delivery, GENERATED_ENTRY_MUTATION_CHILD, in_generated_entry_stack_thread, assert_generated_entry_mutation_child, "route-wrong-delivery", "does not deliver the selected producer result directly");
generated_entry_checked_case!(generated_entry_confluence_route_reversed, GENERATED_ENTRY_MUTATION_CHILD, in_generated_entry_stack_thread, assert_generated_entry_mutation_child, "route-reversed", "reverses the governed source and Ret-input sink");
generated_entry_checked_case!(generated_entry_confluence_route_disagreement, GENERATED_ENTRY_MUTATION_CHILD, in_generated_entry_stack_thread, assert_generated_entry_mutation_child, "route-disagreement", "disagree on their typed consumer projection");
generated_entry_checked_case!(generated_entry_confluence_remove_member, GENERATED_ENTRY_MUTATION_CHILD, in_generated_entry_stack_thread, assert_generated_entry_mutation_child, "remove-member", "not equal as sets");
generated_entry_checked_case!(generated_entry_confluence_duplicate_member, GENERATED_ENTRY_MUTATION_CHILD, in_generated_entry_stack_thread, assert_generated_entry_mutation_child, "duplicate-member", "inserted twice");
generated_entry_checked_case!(generated_entry_confluence_filter_member, GENERATED_ENTRY_MUTATION_CHILD, in_generated_entry_stack_thread, assert_generated_entry_mutation_child, "filter-member", "not equal as sets");

const GENERATED_ENTRY_CAPSULE_MUTATION_CHILD: &str =
    "KEN_RT_CHECKED_IH_GENERATED_ENTRY_CAPSULE_MUTATION_CHILD";

fn assert_generated_entry_capsule_mutation_child() {
    use ken_runtime::CheckedIhGeneratedEntryCapsuleMutation as Mutation;
    let mode = std::env::var(GENERATED_ENTRY_CAPSULE_MUTATION_CHILD)
        .expect("generated-entry capsule mutation child mode");
    let mutation = match mode.as_str() {
        "outer-carried" => Mutation::OuterCarried,
        "specialized-sibling" => Mutation::SpecializedSibling,
        "static-worker" => Mutation::StaticWorker,
        "wrong-frame" => Mutation::WrongFrame,
        "wrong-slot" => Mutation::WrongSlot,
        "wrong-invocation" => Mutation::WrongInvocation,
        "non-carried-residual" => Mutation::NonCarriedResidual,
        "provenance-index" => Mutation::ProvenanceIndex,
        "wrong-destination-owner" => Mutation::WrongDestinationOwner,
        "wrong-destination-body" => Mutation::WrongDestinationBody,
        "wrong-binding" => Mutation::WrongBinding,
        "wrong-locator-invocation" => Mutation::WrongLocatorInvocation,
        "wrong-locator-callee" => Mutation::WrongLocatorCallee,
        "wrong-locator-domain" => Mutation::WrongLocatorDomain,
        "wrong-locator-index" => Mutation::WrongLocatorIndex,
        "retained-access-wrong-destination-owner" => Mutation::RetainedAccessWrongDestinationOwner,
        "retained-access-wrong-destination-body" => Mutation::RetainedAccessWrongDestinationBody,
        "retained-access-wrong-binding" => Mutation::RetainedAccessWrongBinding,
        "retained-access-wrong-locator-invocation" => {
            Mutation::RetainedAccessWrongLocatorInvocation
        }
        "retained-access-wrong-locator-callee" => Mutation::RetainedAccessWrongLocatorCallee,
        "retained-access-wrong-locator-domain" => Mutation::RetainedAccessWrongLocatorDomain,
        "retained-access-wrong-locator-index" => Mutation::RetainedAccessWrongLocatorIndex,
        other => panic!("unknown generated-entry capsule mutation: {other}"),
    };
    let direct_control = matches!(
        mutation,
        Mutation::WrongDestinationOwner
            | Mutation::WrongDestinationBody
            | Mutation::WrongBinding
            | Mutation::WrongLocatorInvocation
            | Mutation::WrongLocatorCallee
            | Mutation::WrongLocatorDomain
            | Mutation::WrongLocatorIndex
    );
    let retained_access_control = matches!(
        mutation,
        Mutation::RetainedAccessWrongDestinationOwner
            | Mutation::RetainedAccessWrongDestinationBody
            | Mutation::RetainedAccessWrongBinding
            | Mutation::RetainedAccessWrongLocatorInvocation
            | Mutation::RetainedAccessWrongLocatorCallee
            | Mutation::RetainedAccessWrongLocatorDomain
            | Mutation::RetainedAccessWrongLocatorIndex
    );
    let (target, stage) = if direct_control {
        ("fs-write-at-offset-single", "rt_write_writable_stage")
    } else {
        ("fs-read-at-offset-single", "rt_read_offset_stage")
    };
    let red = ken_runtime::with_checked_ih_generated_entry_capsule_mutation(mutation, || {
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| differential(target, stage)))
    });
    assert!(red.is_err(), "{mode}: capsule mutation did not redden");
    assert!(
        ken_runtime::checked_ih_generated_entry_capsule_mutation_is_exact(),
        "{mode}: scoped capsule mutation state did not restore"
    );
    if direct_control || retained_access_control {
        differential(target, stage);
        let target = if direct_control { "write" } else { "read" };
        eprintln!(
            "RT_CHECKED_IH_PUBLISHED_PROJECTION_CONTROL_RESTORED_GREEN \
             target={target} mode={mode}"
        );
    }
}

// **Promise class: durable invariant.** The D2 post-selection authority must
// refuse any selected Tail entry whose published sanitized access projection
// differs from its retained confluence projection.
//
// **MEASURED:** each control mutates one named projection field only on the
// semantic Tail route population, observes the exact mutated entry selected by
// the real read fixture, asserts the retained whole-projection refusal, and
// reruns the same unmutated read target GREEN.
// **CLAIMED:** no `(C,I,E,S)` authority forms from a Tail certificate whose E
// access projection disagrees with its retained confluence class.
// **THE GAP:** the paired Direct/write controls below independently prove the
// older terminal consumer-boundary checks; neither layer borrows the other's
// refusal.

generated_entry_split_checked_case!(generated_entry_forward_ret_access_destination_owner_disagreement, "retained-access-wrong-destination-owner", "the retained forward Ret confluence projection disagrees with the published access projection", "RT_CHECKED_IH_PUBLISHED_PROJECTION_CONTROL_SELECTED layer=Tail mutation=DestinationOwner direct_applied=false tail_applied=true", "read");
generated_entry_split_checked_case!(generated_entry_forward_ret_access_destination_body_disagreement, "retained-access-wrong-destination-body", "the retained forward Ret confluence projection disagrees with the published access projection", "RT_CHECKED_IH_PUBLISHED_PROJECTION_CONTROL_SELECTED layer=Tail mutation=DestinationBody direct_applied=false tail_applied=true", "read");
generated_entry_split_checked_case!(generated_entry_forward_ret_access_binding_disagreement, "retained-access-wrong-binding", "the retained forward Ret confluence projection disagrees with the published access projection", "RT_CHECKED_IH_PUBLISHED_PROJECTION_CONTROL_SELECTED layer=Tail mutation=BindingFrame direct_applied=false tail_applied=true", "read");
generated_entry_split_checked_case!(generated_entry_forward_ret_access_locator_invocation_disagreement, "retained-access-wrong-locator-invocation", "the retained forward Ret confluence projection disagrees with the published access projection", "RT_CHECKED_IH_PUBLISHED_PROJECTION_CONTROL_SELECTED layer=Tail mutation=LocatorInvocation direct_applied=false tail_applied=true", "read");
generated_entry_split_checked_case!(generated_entry_forward_ret_access_locator_callee_disagreement, "retained-access-wrong-locator-callee", "the retained forward Ret confluence projection disagrees with the published access projection", "RT_CHECKED_IH_PUBLISHED_PROJECTION_CONTROL_SELECTED layer=Tail mutation=LocatorCallee direct_applied=false tail_applied=true", "read");
generated_entry_split_checked_case!(generated_entry_forward_ret_access_locator_domain_disagreement, "retained-access-wrong-locator-domain", "the retained forward Ret confluence projection disagrees with the published access projection", "RT_CHECKED_IH_PUBLISHED_PROJECTION_CONTROL_SELECTED layer=Tail mutation=LocatorDomain direct_applied=false tail_applied=true", "read");
generated_entry_split_checked_case!(generated_entry_forward_ret_access_locator_index_disagreement, "retained-access-wrong-locator-index", "the retained forward Ret confluence projection disagrees with the published access projection", "RT_CHECKED_IH_PUBLISHED_PROJECTION_CONTROL_SELECTED layer=Tail mutation=LocatorIndex direct_applied=false tail_applied=true", "read");

// **Promise class: durable invariant.** Only the exact computational-recursor
// capsule satisfying every governed fact may pass the pre-dispatch guard; each
// independently varied sibling/fact must reject at its named arm.
//
// **MEASURED:** the seven published-projection controls select only the
// GovernedDirect route population on the real write fixture, observe both
// mutation application and entry into terminal validation, assert that
// consumer's exact refusal, and rerun the same unmutated write target GREEN.
// The remaining capsule controls mutate the real forwarding value consumed by
// their named guard on the read fixture.
// **CLAIMED:** every terminal projection conjunct, including locator domain
// and index, is independently load-bearing at the consumer seat.
// **THE GAP:** the Tail/read controls above separately prove retained D2
// access/confluence equality and prove that these Direct controls mutate zero
// Tail projections.
generated_entry_checked_case!(generated_entry_capsule_outer_carried, GENERATED_ENTRY_CAPSULE_MUTATION_CHILD, in_generated_entry_stack_thread, assert_generated_entry_capsule_mutation_child, "outer-carried", "does not name a specialized computational-recursor capsule");
generated_entry_checked_case!(generated_entry_capsule_specialized_sibling, GENERATED_ENTRY_CAPSULE_MUTATION_CHILD, in_generated_entry_stack_thread, assert_generated_entry_capsule_mutation_child, "specialized-sibling", "is not a computational-recursor capsule");
generated_entry_checked_case!(generated_entry_capsule_static_worker, GENERATED_ENTRY_CAPSULE_MUTATION_CHILD, in_generated_entry_stack_thread, assert_generated_entry_capsule_mutation_child, "static-worker", "StaticWorkerBinding: a source-machine Var in value position is a value-producing position");
generated_entry_checked_case!(generated_entry_capsule_wrong_frame, GENERATED_ENTRY_CAPSULE_MUTATION_CHILD, in_generated_entry_stack_thread, assert_generated_entry_capsule_mutation_child, "wrong-frame", "checked frame, slot, call template, or residual phase");
generated_entry_checked_case!(generated_entry_capsule_wrong_slot, GENERATED_ENTRY_CAPSULE_MUTATION_CHILD, in_generated_entry_stack_thread, assert_generated_entry_capsule_mutation_child, "wrong-slot", "checked frame, slot, call template, or residual phase");
generated_entry_checked_case!(generated_entry_capsule_wrong_invocation, GENERATED_ENTRY_CAPSULE_MUTATION_CHILD, in_generated_entry_stack_thread, assert_generated_entry_capsule_mutation_child, "wrong-invocation", "projection disagrees with its current function, binding, or call coordinate");
generated_entry_checked_case!(generated_entry_capsule_non_carried_residual, GENERATED_ENTRY_CAPSULE_MUTATION_CHILD, in_generated_entry_stack_thread, assert_generated_entry_capsule_mutation_child, "non-carried-residual", "checked frame, slot, call template, or residual phase");
generated_entry_checked_case!(generated_entry_capsule_provenance_index, GENERATED_ENTRY_CAPSULE_MUTATION_CHILD, in_generated_entry_stack_thread, assert_generated_entry_capsule_mutation_child, "provenance-index", "callee Var disagrees with the immediate K locator index");
generated_entry_split_checked_case!(generated_entry_capsule_wrong_destination_owner, "wrong-destination-owner", "a governed generated-entry projection disagrees with its current function, binding, or call coordinate", "RT_CHECKED_IH_PUBLISHED_PROJECTION_CONTROL_VALIDATION layer=Direct mutation=DestinationOwner direct_applied=true tail_applied=false", "write");
generated_entry_split_checked_case!(generated_entry_capsule_wrong_destination_body, "wrong-destination-body", "a governed generated-entry projection disagrees with its current function, binding, or call coordinate", "RT_CHECKED_IH_PUBLISHED_PROJECTION_CONTROL_VALIDATION layer=Direct mutation=DestinationBody direct_applied=true tail_applied=false", "write");
generated_entry_split_checked_case!(generated_entry_capsule_wrong_binding, "wrong-binding", "a governed generated-entry projection disagrees with its current function, binding, or call coordinate", "RT_CHECKED_IH_PUBLISHED_PROJECTION_CONTROL_VALIDATION layer=Direct mutation=BindingFrame direct_applied=true tail_applied=false", "write");
generated_entry_split_checked_case!(generated_entry_capsule_wrong_locator_invocation, "wrong-locator-invocation", "a governed generated-entry projection disagrees with its current function, binding, or call coordinate", "RT_CHECKED_IH_PUBLISHED_PROJECTION_CONTROL_VALIDATION layer=Direct mutation=LocatorInvocation direct_applied=true tail_applied=false", "write");
generated_entry_split_checked_case!(generated_entry_capsule_wrong_locator_callee, "wrong-locator-callee", "a governed generated-entry projection disagrees with its current function, binding, or call coordinate", "RT_CHECKED_IH_PUBLISHED_PROJECTION_CONTROL_VALIDATION layer=Direct mutation=LocatorCallee direct_applied=true tail_applied=false", "write");
generated_entry_split_checked_case!(generated_entry_capsule_wrong_locator_domain, "wrong-locator-domain", "the governed immediate K locator has the wrong domain or is outside the current environment", "RT_CHECKED_IH_PUBLISHED_PROJECTION_CONTROL_VALIDATION layer=Direct mutation=LocatorDomain direct_applied=true tail_applied=false", "write");
generated_entry_split_checked_case!(generated_entry_capsule_wrong_locator_index, "wrong-locator-index", "the governed immediate K locator has the wrong domain or is outside the current environment", "RT_CHECKED_IH_PUBLISHED_PROJECTION_CONTROL_VALIDATION layer=Direct mutation=LocatorIndex direct_applied=true tail_applied=false", "write");

/// **Promise class: durable invariant.** Dense numbering may move, but planner
/// iteration and context-interning order must not change class/member/caller
/// association or the product observation.
#[test]
fn checked_ih_generated_entry_confluence_is_interning_and_inheritance_order_independent() {
    in_generated_entry_stack_thread("rt-parity-generated-entry-permute", || {
        let (exact_result, exact) =
            ken_runtime::with_checked_ih_generated_entry_observations(|| {
                differential("fs-write-at-offset-single", "rt_write_writable_stage")
            });
        let (permuted_result, permuted) =
            ken_runtime::with_checked_ih_generated_entry_confluence_mutation(
                ken_runtime::CheckedIhGeneratedEntryConfluenceMutation::PermuteInheritanceOrder,
                || {
                    ken_runtime::with_checked_ih_generated_entry_observations(|| {
                        differential("fs-write-at-offset-single", "rt_write_writable_stage")
                    })
                },
            );
        let (context_result, context_permuted) =
            ken_runtime::with_checked_ih_generated_entry_confluence_mutation(
                ken_runtime::CheckedIhGeneratedEntryConfluenceMutation::PermuteContextInterningOrder,
                || {
                    ken_runtime::with_checked_ih_generated_entry_observations(|| {
                        differential("fs-write-at-offset-single", "rt_write_writable_stage")
                    })
                },
            );
        assert_eq!(exact, permuted, "association must not depend on row order");
        let normalize = |mut rows: Vec<ken_runtime::CheckedIhGeneratedEntryObservation>| {
            for row in &mut rows {
                row.context = 0;
            }
            rows.sort_by_key(|row| {
                (
                    row.enclosing_specialization,
                    row.worker_body_origin,
                    row.binding_frame_origin,
                    row.invocation_origin,
                    row.call_origin,
                    row.callee_origin,
                )
            });
            rows
        };
        assert_eq!(
            normalize(exact.clone()),
            normalize(context_permuted),
            "dense context numbering may move, but key/member/caller association must not"
        );
        assert_eq!(exact_result.native.effect_trace, permuted_result.native.effect_trace);
        assert_eq!(exact_result.native.effect_trace, context_result.native.effect_trace);
        assert_eq!(exact_result.native.terminal_error, permuted_result.native.terminal_error);
        assert_eq!(exact_result.native.terminal_error, context_result.native.terminal_error);
        assert!(ken_runtime::checked_ih_generated_entry_confluence_mutation_is_exact());
    });
}

/// **Promise class: transition sentinel.** A reviewed change to the fixed
/// read/write static occurrence graphs may replace these coordinates, but must
/// preserve a non-degenerate sink population and per-entry uniqueness.
///
/// **MEASURED:** every strict-`Ret` block created while compiling the two fixed
/// products installs one compiler-only sink and completes one exact lookup.
/// **CLAIMED:** D1 covers the real carried strict-`Ret` population and each
/// emitted function's active stack entry owns one sink.
/// **THE GAP:** the fixed products instantiate multiple frames independently;
/// the mutation controls below vary the installation and lookup operations at
/// the production block-creation seam.
#[test]
fn composed_return_ret_sink_population_is_unique() {
    in_large_stack_thread("rt-parity-composed-return-ret-sink-population", || {
        let compile = |case: &str, entry: &str, label: &str| {
            let source = RT_PARITY_SOURCE.replace("__RT_PARITY_ENTRY__", entry);
            let root = output_dir(&format!("composed-return-ret-sink-{label}"));
            let (artifact, observations, applications) =
                ken_runtime::with_composed_return_ret_sink_mutation(
                    ken_runtime::ComposedReturnRetSinkMutation::Exact,
                    || {
                        ken_cli::build_native_program(
                            &source,
                            ken_cli::SourceFormat::Ken,
                            &format!("rt_parity_composed_return_ret_sink_{label}"),
                            root.path(),
                        )
                    },
                );
            artifact.expect("the exact D1 sink population compiles");
            assert_eq!(
                applications,
                observations.len(),
                "{case}: every reached strict-Ret seam installs one sink"
            );
            assert!(
                observations.len() > 1,
                "{case}: the fixture must instantiate more than one sink: {observations:#?}"
            );
            observations
        };

        let read = compile("fs-read-at-offset-single", "rt_read_offset_stage", "read");
        let write = compile(
            "fs-write-at-offset-single",
            "rt_write_writable_stage",
            "write",
        );
        for (label, observations, expected_count, expected_semantic_coordinates) in [
            (
                "read",
                read,
                35,
                std::collections::BTreeSet::from([
                    ("StaticOriginId(12)", "StaticOriginId(294)", 0),
                    ("StaticOriginId(301)", "StaticOriginId(465)", 0),
                    ("StaticOriginId(470)", "StaticOriginId(505)", 0),
                    ("StaticOriginId(511)", "StaticOriginId(676)", 0),
                    ("StaticOriginId(681)", "StaticOriginId(744)", 0),
                ]),
            ),
            (
                "write",
                write,
                26,
                std::collections::BTreeSet::from([
                    ("StaticOriginId(25)", "StaticOriginId(307)", 0),
                    ("StaticOriginId(314)", "StaticOriginId(478)", 0),
                    ("StaticOriginId(483)", "StaticOriginId(518)", 0),
                    ("StaticOriginId(525)", "StaticOriginId(691)", 0),
                    ("StaticOriginId(696)", "StaticOriginId(731)", 0),
                    ("StaticOriginId(737)", "StaticOriginId(904)", 0),
                    ("StaticOriginId(909)", "StaticOriginId(1053)", 0),
                ]),
            ),
        ] {
            assert_eq!(
                observations.len(),
                expected_count,
                "{label}: the fixed emitted sink population changed"
            );
            let semantic_coordinates = observations
                .iter()
                .map(|row| {
                    (
                        row.active_frame_origin.as_str(),
                        row.ret_case_body_origin.as_str(),
                        row.ret_input_field_position,
                    )
                })
                .collect::<std::collections::BTreeSet<_>>();
            assert_eq!(
                semantic_coordinates, expected_semantic_coordinates,
                "{label}: the fixed semantic sink population changed"
            );
            let function_local_keys = observations
                .iter()
                .map(|row| {
                    (
                        row.defining_function,
                        row.active_frame_origin.as_str(),
                        row.header_block.as_str(),
                        row.ret_case_body_origin.as_str(),
                        row.ret_input_field_position,
                        row.return_body_block.as_str(),
                    )
                })
                .collect::<std::collections::BTreeSet<_>>();
            assert_eq!(
                function_local_keys.len(),
                observations.len(),
                "{label}: each function-local active stack entry must own one sink: {observations:#?}"
            );
            assert!(observations.iter().all(|row| {
                row.ret_input_field_position == 0
                    && row.installation_count == 1
                    && row.exact_lookup_count == 1
            }));
        }
        assert!(ken_runtime::composed_return_ret_sink_mutation_is_exact());
    });
}

/// **Promise class: durable invariant.** Extensions may add strict-`Ret`
/// frames, but missing, duplicate, and mismatched coordinates must still refuse
/// at the D1 seam before any result consumer exists.
///
/// **MEASURED:** each control mutates the real installation or exact lookup at
/// shared-block creation and the build returns the named sink-seam error.
/// **CLAIMED:** absence, duplication, wrong active frame, wrong body, and wrong
/// binder cannot yield a usable compiler-local block.
/// **THE GAP:** mutation application count proves production reach; the exact
/// error text distinguishes the intended arm from unrelated build failure.
#[test]
fn composed_return_ret_sink_lookup_controls_refuse() {
    in_large_stack_thread("rt-parity-composed-return-ret-sink-controls", || {
        use ken_runtime::ComposedReturnRetSinkMutation as Mutation;

        let source = RT_PARITY_SOURCE.replace("__RT_PARITY_ENTRY__", "rt_write_writable_stage");
        for (label, mutation, expected) in [
            (
                "missing",
                Mutation::Missing,
                "active carried frame has no installed strict Ret sink",
            ),
            (
                "duplicate-installation",
                Mutation::DuplicateInstallation,
                "received more than one strict Ret sink",
            ),
            (
                "duplicate-active-frame",
                Mutation::DuplicateActiveFrame,
                "lookup found more than one active carried frame",
            ),
            (
                "wrong-frame",
                Mutation::WrongActiveFrame,
                "lookup names a different active carried frame",
            ),
            (
                "wrong-body",
                Mutation::WrongRetBody,
                "belongs to a different Ret case body",
            ),
            (
                "wrong-binder",
                Mutation::WrongBinder,
                "belongs to a different Ret input binder",
            ),
        ] {
            let root = output_dir(&format!("composed-return-ret-sink-control-{label}"));
            let (result, observations, applications) =
                ken_runtime::with_composed_return_ret_sink_mutation(mutation, || {
                    ken_cli::build_native_program(
                        &source,
                        ken_cli::SourceFormat::Ken,
                        &format!(
                            "rt_parity_composed_return_ret_sink_control_{}",
                            label.replace('-', "_")
                        ),
                        root.path(),
                    )
                });
            assert!(
                applications > 0,
                "{label}: control missed the production seam"
            );
            let error = result.expect_err("a malformed D1 sink operation must refuse");
            let rendered = format!("{error:?}");
            assert!(
                rendered.contains(expected),
                "{label}: wrong refusal arm; error={rendered}; observations={observations:#?}"
            );
            assert!(ken_runtime::composed_return_ret_sink_mutation_is_exact());
        }
    });
}

/// **Promise class: durable invariant.**
///
/// **MEASURED:** exact D1 sink installation and complete seam suppression emit
/// identical semantic hashes, executable hashes, and executable bytes.
/// **CLAIMED:** the compiler-only sink record changes no ABI, call, value,
/// result route, or emitted behavior before D3 activates a consumer.
/// **THE GAP:** suppression removes both the install and its internal lookup;
/// D2 authority formation is suppressed in both arms so this differential
/// still isolates D1. The population and refusal controls above independently
/// establish that the exact seam is present and fail-closed.
#[test]
fn composed_return_ret_sink_is_byte_inert() {
    in_large_stack_thread("rt-parity-composed-return-ret-sink-inert", || {
        let source = RT_PARITY_SOURCE.replace("__RT_PARITY_ENTRY__", "rt_read_offset_stage");
        let exact_root = output_dir("composed-return-ret-sink-inert-exact");
        let suppressed_root = output_dir("composed-return-ret-sink-inert-suppressed");
        let (exact, exact_rows, exact_applications) =
            ken_runtime::with_composed_return_ret_sink_mutation(
                ken_runtime::ComposedReturnRetSinkMutation::Exact,
                || {
                    let (artifact, _d2_rows, _d2_applications) =
                        ken_runtime::with_composed_return_forward_ret_authority_mutation(
                            ken_runtime::ComposedReturnForwardRetAuthorityMutation::SuppressForInertness,
                            || {
                                ken_cli::build_native_program(
                                    &source,
                                    ken_cli::SourceFormat::Ken,
                                    "rt_parity_composed_return_ret_sink_inert",
                                    exact_root.path(),
                                )
                            },
                        );
                    artifact
                },
            );
        let exact = exact.expect("exact composed-return Ret-sink artifact");
        let (suppressed, suppressed_rows, suppressed_applications) =
            ken_runtime::with_composed_return_ret_sink_mutation(
                ken_runtime::ComposedReturnRetSinkMutation::SuppressForInertness,
                || {
                    let (artifact, _d2_rows, _d2_applications) =
                        ken_runtime::with_composed_return_forward_ret_authority_mutation(
                            ken_runtime::ComposedReturnForwardRetAuthorityMutation::SuppressForInertness,
                            || {
                                ken_cli::build_native_program(
                                    &source,
                                    ken_cli::SourceFormat::Ken,
                                    "rt_parity_composed_return_ret_sink_inert",
                                    suppressed_root.path(),
                                )
                            },
                        );
                    artifact
                },
            );
        let suppressed = suppressed.expect("suppressed composed-return Ret-sink artifact");

        assert!(!exact_rows.is_empty());
        assert!(suppressed_rows.is_empty());
        assert_eq!(exact_applications, exact_rows.len());
        assert_eq!(suppressed_applications, exact_applications);
        assert_eq!(exact.plan_transport_hash, suppressed.plan_transport_hash);
        assert_eq!(
            exact.runtime_program.core_semantic_hash,
            suppressed.runtime_program.core_semantic_hash
        );
        assert_eq!(
            exact.runtime_program.artifact_hash,
            suppressed.runtime_program.artifact_hash
        );
        assert_eq!(
            exact.artifact.executable_hash,
            suppressed.artifact.executable_hash
        );
        assert_eq!(
            std::fs::read(&exact.artifact.executable_path).expect("exact executable bytes"),
            std::fs::read(&suppressed.artifact.executable_path)
                .expect("suppressed executable bytes"),
            "the D1 sink seam must change no emitted byte"
        );
        assert!(ken_runtime::composed_return_ret_sink_mutation_is_exact());
    });
}

/// **Promise class: durable invariant.**
///
/// **MEASURED:** exact post-selection D2 authority formation and complete
/// suppression emit identical semantic hashes, executable hashes, and bytes;
/// every exact observation names forward, producer-result-direct delivery to
/// field zero of one compiler-local Ret block.
/// **CLAIMED:** the move-only authority join changes no call, result route,
/// ABI, runtime carrier, or artifact before D3 activates a consumer.
/// **THE GAP:** the suppression arm moves the authority operation while leaving
/// the same new planner route in place; the plan-shape positive and the five
/// natural-site refusal arms independently cover what this differential does
/// not.
#[test]
fn composed_return_forward_ret_authority_is_byte_inert() {
    in_large_stack_thread("rt-parity-forward-ret-authority-inert", || {
        use ken_runtime::ComposedReturnForwardRetAuthorityMutation as Mutation;

        let source = RT_PARITY_SOURCE.replace("__RT_PARITY_ENTRY__", "rt_read_offset_stage");
        let exact_root = output_dir("forward-ret-authority-inert-exact");
        let suppressed_root = output_dir("forward-ret-authority-inert-suppressed");
        let (exact, exact_rows, exact_applications) =
            ken_runtime::with_composed_return_forward_ret_authority_mutation(
                Mutation::Exact,
                || {
                    ken_cli::build_native_program(
                        &source,
                        ken_cli::SourceFormat::Ken,
                        "rt_parity_forward_ret_authority_inert",
                        exact_root.path(),
                    )
                },
            );
        let exact = exact.expect("exact D2 forward-Ret authority artifact");
        let (suppressed, suppressed_rows, suppressed_applications) =
            ken_runtime::with_composed_return_forward_ret_authority_mutation(
                Mutation::SuppressForInertness,
                || {
                    ken_cli::build_native_program(
                        &source,
                        ken_cli::SourceFormat::Ken,
                        "rt_parity_forward_ret_authority_inert",
                        suppressed_root.path(),
                    )
                },
            );
        let suppressed = suppressed.expect("suppressed D2 forward-Ret authority artifact");

        assert!(
            !exact_rows.is_empty(),
            "the real Tail authority population must reach"
        );
        assert!(suppressed_rows.is_empty());
        assert_eq!(exact_applications, exact_rows.len());
        assert_eq!(suppressed_applications, exact_applications);
        assert!(exact_rows.iter().all(|row| {
            row.coordinate
                .ret_input_binder
                .ends_with("field_position: 0 }")
                && row.coordinate.direction == "Forward"
                && row.coordinate.delivery == "ProducerResultDirect"
                && !row.coordinate.source_call_identity.is_empty()
                && !row.return_body_block.is_empty()
        }));
        assert_eq!(exact.plan_transport_hash, suppressed.plan_transport_hash);
        assert_eq!(
            exact.runtime_program.core_semantic_hash,
            suppressed.runtime_program.core_semantic_hash
        );
        assert_eq!(
            exact.runtime_program.artifact_hash,
            suppressed.runtime_program.artifact_hash
        );
        assert_eq!(
            exact.artifact.executable_hash,
            suppressed.artifact.executable_hash
        );
        assert_eq!(
            std::fs::read(&exact.artifact.executable_path).expect("exact executable bytes"),
            std::fs::read(&suppressed.artifact.executable_path)
                .expect("suppressed executable bytes"),
            "D2 authority formation must change no emitted byte"
        );
        assert!(ken_runtime::composed_return_forward_ret_authority_mutation_is_exact());
    });
}

/// **Promise class: durable invariant.**
///
/// **MEASURED:** at the actual consumer call, every formed authority pairs the
/// current C/admission and selected I with the same `(I,E,S)` coordinate that
/// the independent planner and authority observers report; E's existing
/// capsule row is installed and reached, and at least one real row has C != E.
/// **CLAIMED:** current consumer call C and certificate entry E have distinct
/// roles, while exact whole-transport selection I causally joins C to the real
/// planned and reached `(E,S)` certificate without storing I in access.
/// **THE GAP:** the role witness is observation-only and cannot license
/// authority; existing production equality/member/source/projection/sink gates
/// remain the enforcement, while the paired witness prevents their coordinates
/// from being misdescribed as one call again.
#[test]
fn composed_return_forward_ret_role_witness_pairs_c_and_certificate() {
    in_large_stack_thread("rt-parity-forward-ret-role-witness", || {
        use ken_runtime::ComposedReturnForwardRetAuthorityMutation as Mutation;

        let compile = |label: &str, entry: &str| {
            let source = RT_PARITY_SOURCE.replace("__RT_PARITY_ENTRY__", entry);
            let root = output_dir(&format!("forward-ret-role-witness-{label}"));
            let (((result, plan_rows), role_rows), authorities, applications) =
                ken_runtime::with_composed_return_forward_ret_authority_mutation(
                    Mutation::Exact,
                    || {
                        ken_runtime::with_composed_return_forward_ret_role_witnesses(|| {
                            ken_runtime::with_checked_ih_generated_entry_observations(|| {
                                ken_cli::build_native_program(
                                    &source,
                                    ken_cli::SourceFormat::Ken,
                                    &format!("rt_parity_forward_ret_role_witness_{label}"),
                                    root.path(),
                                )
                            })
                        })
                    },
                );
            result.expect("the exact C/I/E/S role witness fixture must compile");
            assert!(!authorities.is_empty(), "{label}: no authority formed");
            assert_eq!(applications, authorities.len());

            let planned = plan_rows
                .iter()
                .flat_map(|row| row.forward_ret_coordinates.iter().cloned())
                .collect::<std::collections::BTreeSet<_>>();
            let formed = authorities
                .iter()
                .map(|row| row.coordinate.clone())
                .collect::<std::collections::BTreeSet<_>>();
            let paired = role_rows
                .iter()
                .filter(|row| row.outcome == "Formed")
                .map(|row| {
                    let coordinate = row
                        .formed_coordinate
                        .clone()
                        .expect("a Formed role witness must carry E/S proof coordinates");
                    assert_eq!(
                        row.selected_source_call_identity, coordinate.source_call_identity,
                        "{label}: paired row's selected I differs from its proof I"
                    );
                    assert!(
                        plan_rows.iter().any(|plan| {
                            plan.installed
                                && plan.reached_count > 0
                                && plan.forward_ret_coordinates.contains(&coordinate)
                        }),
                        "{label}: formed E has no installed and reached capsule row: {coordinate:#?}"
                    );
                    coordinate
                })
                .collect::<std::collections::BTreeSet<_>>();
            assert_eq!(planned, formed, "{label}: planned/formed Tail set changed");
            assert_eq!(
                formed, paired,
                "{label}: formed authority lacks one C/I/E/S row"
            );
            assert_eq!(
                role_rows
                    .iter()
                    .filter(|row| row.outcome == "Formed")
                    .count(),
                authorities.len(),
                "{label}: paired rows and formed authorities differ in multiplicity"
            );
            assert!(
                role_rows.iter().any(|row| {
                    row.outcome == "Formed"
                        && row.formed_coordinate.as_ref().is_some_and(|coordinate| {
                            (
                                &row.current_invocation_origin,
                                &row.current_call_origin,
                                &row.current_callee_origin,
                            ) != (
                                &coordinate.entry_invocation_origin,
                                &coordinate.entry_call_origin,
                                &coordinate.entry_callee_origin,
                            )
                        })
                }),
                "{label}: fixture did not prove a real current C distinct from certificate E"
            );
            assert!(
                role_rows.iter().all(|row| {
                    !row.current_admission.is_empty()
                        && !row.selected_source_call_identity.is_empty()
                        && (row.outcome != "Formed" || row.formed_coordinate.is_some())
                }),
                "{label}: a role-witness row is incomplete"
            );
            assert!(ken_runtime::composed_return_forward_ret_authority_mutation_is_exact());
        };

        compile("read", "rt_read_offset_stage");
        compile("write", "rt_write_writable_stage");
    });
}

/// **Promise class: durable invariant.**
///
/// **MEASURED:** the complete set of real planned Tail coordinates equals the
/// unique set of formed post-selection authorities across disjoint read and
/// write fixtures; removing each Tail member or duplicating one makes the
/// unchanged source consumer refuse before call emission.
/// **CLAIMED:** every validated Tail producer-to-Ret route forms exactly one
/// move-only authority, while Direct and non-governed routes form none.
/// **THE GAP:** plan and authority observations come from independent sides of
/// the consumer join; exact set equality closes pairing, per-source removal
/// proves no member may fail open, and duplication separately closes
/// multiplicity rather than relying on set equality.
#[test]
fn composed_return_forward_ret_authority_population_is_exact() {
    in_large_stack_thread("rt-parity-forward-ret-authority-population", || {
        use ken_runtime::ComposedReturnForwardRetAuthorityMutation as Mutation;

        let compile = |label: &str, mutation: Mutation, entry: &str| {
            let source = RT_PARITY_SOURCE.replace("__RT_PARITY_ENTRY__", entry);
            let root = output_dir(&format!("forward-ret-authority-population-{label}"));
            let ((result, plan_rows), formed, applications) =
                ken_runtime::with_composed_return_forward_ret_authority_mutation(mutation, || {
                    ken_runtime::with_checked_ih_generated_entry_observations(|| {
                        ken_cli::build_native_program(
                            &source,
                            ken_cli::SourceFormat::Ken,
                            &format!("rt_parity_forward_ret_authority_population_{label}"),
                            root.path(),
                        )
                    })
                });
            let expected = plan_rows
                .into_iter()
                .flat_map(|row| row.forward_ret_coordinates)
                .collect::<Vec<_>>();
            let actual = formed
                .iter()
                .map(|row| row.coordinate.clone())
                .collect::<Vec<_>>();
            (result, expected, actual, applications)
        };

        let (exact, expected, actual, exact_applications) =
            compile("read-exact", Mutation::Exact, "rt_read_offset_stage");
        exact.expect("the exact Tail authority population must compile");
        let expected_set = expected
            .iter()
            .cloned()
            .collect::<std::collections::BTreeSet<_>>();
        let actual_set = actual
            .iter()
            .cloned()
            .collect::<std::collections::BTreeSet<_>>();
        assert!(
            expected_set.len() > 1,
            "the fixture must instantiate a non-degenerate Tail population"
        );
        assert_eq!(
            expected.len(),
            expected_set.len(),
            "the real planned Tail population must be unique"
        );
        assert_eq!(expected_set, actual_set);
        assert_eq!(exact_applications, actual.len());
        assert!(
            expected.iter().all(|coordinate| {
                (
                    &coordinate.entry_binding,
                    &coordinate.entry_invocation_origin,
                    &coordinate.entry_call_origin,
                    &coordinate.entry_callee_origin,
                ) != (
                    &coordinate.binding,
                    &coordinate.invocation_origin,
                    &coordinate.call_origin,
                    &coordinate.callee_origin,
                ) && !coordinate.entry_immediate_k_locator.is_empty()
            }),
            "every planned Tail member must pair distinct generated-entry E and producer-source S coordinates"
        );

        for target in 0..expected.len() {
            let label = format!("remove-{target}");
            let (result, removed_expected, removed_actual, applications) = compile(
                &format!("read-{label}"),
                Mutation::RemoveTailAuthorityAt(target),
                "rt_read_offset_stage",
            );
            let error = result.expect_err("one missing Tail authority must refuse");
            assert!(
                format!("{error:?}").contains(
                    "validated Tail producer-to-Ret route has no exact post-selection authority"
                ),
                "removal {target} reached the wrong refusal: {error:?}"
            );
            assert_eq!(removed_expected, expected);
            assert_eq!(applications, removed_actual.len() + 1);
            let removed_set = removed_actual
                .iter()
                .cloned()
                .collect::<std::collections::BTreeSet<_>>();
            assert!(
                removed_set.is_subset(&expected_set) && removed_set != expected_set,
                "removal {target} did not remove one real planned coordinate"
            );
            assert!(
                ken_runtime::composed_return_forward_ret_authority_mutation_is_exact(),
                "removal {target} did not restore its scoped mutation"
            );
        }

        let (write_exact, write_expected, write_actual, write_applications) =
            compile("write-exact", Mutation::Exact, "rt_write_writable_stage");
        write_exact.expect("the exact write-side Tail authority population must compile");
        let write_expected_set = write_expected
            .iter()
            .cloned()
            .collect::<std::collections::BTreeSet<_>>();
        let write_actual_set = write_actual
            .iter()
            .cloned()
            .collect::<std::collections::BTreeSet<_>>();
        assert!(write_expected_set.len() > 1);
        assert_eq!(write_expected.len(), write_expected_set.len());
        assert_eq!(write_expected_set, write_actual_set);
        assert_eq!(write_applications, write_actual.len());
        assert_eq!(
            expected_set.union(&write_expected_set).count(),
            expected_set.len() + write_expected_set.len(),
            "read and write fixtures must cover disjoint Tail coordinates"
        );

        for target in 0..write_expected.len() {
            let (result, removed_expected, removed_actual, applications) = compile(
                &format!("write-remove-{target}"),
                Mutation::RemoveTailAuthorityAt(target),
                "rt_write_writable_stage",
            );
            let error = result.expect_err("one missing write-side Tail authority must refuse");
            assert!(
                format!("{error:?}").contains(
                    "validated Tail producer-to-Ret route has no exact post-selection authority"
                ),
                "write removal {target} reached the wrong refusal: {error:?}"
            );
            assert_eq!(removed_expected, write_expected);
            assert_eq!(applications, removed_actual.len() + 1);
            let removed_set = removed_actual
                .iter()
                .cloned()
                .collect::<std::collections::BTreeSet<_>>();
            assert!(
                removed_set.is_subset(&write_expected_set) && removed_set != write_expected_set,
                "write removal {target} did not remove one real planned coordinate"
            );
            assert!(ken_runtime::composed_return_forward_ret_authority_mutation_is_exact());
        }

        let duplicate_target = expected.len() / 2;
        let (result, duplicate_expected, duplicated, applications) = compile(
            "read-duplicate",
            Mutation::DuplicateTailAuthorityAt(duplicate_target),
            "rt_read_offset_stage",
        );
        let error = result.expect_err("one duplicated Tail authority must refuse");
        assert!(
            format!("{error:?}").contains(
                "validated Tail producer-to-Ret route formed more than one post-selection authority"
            ),
            "duplication reached the wrong refusal: {error:?}"
        );
        assert_eq!(duplicate_expected, expected);
        assert_eq!(applications, duplicated.len());
        let [.., penultimate, last] = duplicated.as_slice() else {
            panic!("the duplication control formed fewer than two authorities");
        };
        assert_eq!(
            penultimate, last,
            "the duplication control did not duplicate one real authority coordinate"
        );
        assert!(ken_runtime::composed_return_forward_ret_authority_mutation_is_exact());
    });
}

/// **Promise class: durable invariant.**
///
/// **MEASURED:** each control changes one operand after exact generated-entry
/// validation and transport selection, reaches the D2 join, and returns its
/// specific member, projection, selected identity, producer source, or sink
/// refusal.
/// **CLAIMED:** no other confluence member, projection, source identity, or
/// function-local Ret sink can yield usable forward authority.
/// **THE GAP:** applications prove reach and exact messages distinguish the
/// intended arms; the exact byte-inert positive above supplies the passing
/// configuration each negative control needs.
#[test]
fn composed_return_forward_ret_authority_controls_refuse() {
    in_large_stack_thread("rt-parity-forward-ret-authority-controls", || {
        use ken_runtime::ComposedReturnForwardRetAuthorityMutation as Mutation;

        let source = RT_PARITY_SOURCE.replace("__RT_PARITY_ENTRY__", "rt_write_writable_stage");
        for (label, mutation, expected) in [
            (
                "wrong-member",
                Mutation::WrongMember,
                "source-call identity is not a member of the exact forward Ret confluence class",
            ),
            (
                "projection-disagreement",
                Mutation::ProjectionDisagreement,
                "projection disagrees with its exact access-coordinate projection",
            ),
            (
                "wrong-source",
                Mutation::WrongSource,
                "proof source is not the selected transport's own source-call identity",
            ),
            (
                "producer-source-from-entry",
                Mutation::ProducerSourceFromEntry,
                "Tail producer source disagrees with the selected member's planner-derived producer step",
            ),
            (
                "wrong-sink",
                Mutation::WrongSink,
                "does not match the unique emission sink",
            ),
        ] {
            let root = output_dir(&format!("forward-ret-authority-control-{label}"));
            let (result, observations, applications) =
                ken_runtime::with_composed_return_forward_ret_authority_mutation(mutation, || {
                    ken_cli::build_native_program(
                        &source,
                        ken_cli::SourceFormat::Ken,
                        &format!(
                            "rt_parity_forward_ret_authority_control_{}",
                            label.replace('-', "_")
                        ),
                        root.path(),
                    )
                });
            assert!(
                applications > 0,
                "{label}: control missed the post-selection D2 join"
            );
            let error = result.expect_err("a mismatched D2 authority operand must refuse");
            let rendered = format!("{error:?}");
            assert!(
                rendered.contains(expected),
                "{label}: wrong refusal arm; error={rendered}; observations={observations:#?}"
            );
            assert!(
                ken_runtime::composed_return_forward_ret_authority_mutation_is_exact(),
                "{label}: scoped D2 authority mutation did not restore"
            );
        }
    });
}

/// **Promise class: durable invariant.**
///
/// **MEASURED:** the exact inheritance-to-producer planner chain and the
/// upstream-suppressed chain emit identical semantic hashes, executable hashes,
/// and executable bytes.
/// **CLAIMED:** adding the fresh-result route proof changes no ABI, call, or
/// emitted behavior.
/// **THE GAP:** suppression removes the route with its upstream inheritance;
/// the production-use review separately establishes that the route's only
/// production consumer is the pre-dispatch validation guard.
#[test]
fn checked_ih_inheritance_and_fresh_result_route_are_byte_inert() {
    in_large_stack_thread("rt-parity-continuation-inheritance-inert", || {
        let source = RT_PARITY_SOURCE.replace("__RT_PARITY_ENTRY__", "rt_read_offset_stage");
        let exact_root = output_dir("continuation-inheritance-inert-exact");
        let suppressed_root = output_dir("continuation-inheritance-inert-suppressed");
        let exact = ken_cli::build_native_program(
            &source,
            ken_cli::SourceFormat::Ken,
            "rt_parity_continuation_inheritance_inert",
            exact_root.path(),
        )
        .expect("exact continuation-inheritance artifact");
        let suppressed = ken_runtime::with_checked_ih_continuation_inheritance_mutation(
            ken_runtime::CheckedIhContinuationInheritanceMutation::SuppressForInertness,
            || {
                ken_cli::build_native_program(
                    &source,
                    ken_cli::SourceFormat::Ken,
                    "rt_parity_continuation_inheritance_inert",
                    suppressed_root.path(),
                )
            },
        )
        .expect("suppressed continuation-inheritance artifact");

        assert_eq!(exact.plan_transport_hash, suppressed.plan_transport_hash);
        assert_eq!(
            exact.runtime_program.core_semantic_hash,
            suppressed.runtime_program.core_semantic_hash
        );
        assert_eq!(
            exact.runtime_program.artifact_hash,
            suppressed.runtime_program.artifact_hash
        );
        assert_eq!(
            exact.artifact.executable_hash,
            suppressed.artifact.executable_hash
        );
        assert_eq!(
            std::fs::read(&exact.artifact.executable_path).expect("exact executable bytes"),
            std::fs::read(&suppressed.artifact.executable_path)
                .expect("suppressed executable bytes"),
            "planner-only inheritance and fresh-result route proofs must change no emitted ABI, call, or artifact byte"
        );
        assert!(ken_runtime::checked_ih_continuation_inheritance_mutation_is_exact());
    });
}

const CONTINUATION_INHERITANCE_MUTATION_CHILD: &str =
    "KEN_RT_ITREE_CONTINUATION_INHERITANCE_MUTATION_CHILD";

fn assert_continuation_inheritance_mutation_child() {
    use ken_runtime::CheckedIhContinuationInheritanceMutation as Mutation;

    let mode = std::env::var(CONTINUATION_INHERITANCE_MUTATION_CHILD)
        .expect("continuation-inheritance mutation child mode");
    let mutation = match mode.as_str() {
        "remove" => Mutation::RemoveInheritedCapability,
        "duplicate" => Mutation::DuplicateInheritedCapability,
        "swap" => Mutation::SwapInheritedEndpoints,
        "break-step" => Mutation::BreakSelfResumptionStep,
        "remove-k-locator" => Mutation::RemoveImmediateKLocator,
        "duplicate-k-locator" => Mutation::DuplicateImmediateKLocator,
        "wrong-k-domain" => Mutation::SubstituteWrongKLocatorDomain,
        "wrong-k-consumer" => Mutation::SubstituteWrongKLocatorConsumer,
        "wrong-k-index" => Mutation::SubstituteWrongKLocatorIndex,
        "source-slot-k-locator" => Mutation::SubstituteSourceRecursiveSlotLocator,
        "final-residual-k-locator" => Mutation::SubstituteFinalRecursorResidualLocator,
        "reclassify-ret" => Mutation::ReclassifyRetChildAsIh,
        "descriptor-only" => Mutation::SubstituteDescriptorOnlyAuthority,
        "earlier-result" => Mutation::SubstituteEarlierResult,
        "read-write-swap" => Mutation::SwapReadWriteEndpoints,
        other => panic!("unknown continuation-inheritance mutation: {other}"),
    };
    let red = ken_runtime::with_checked_ih_continuation_inheritance_mutation(mutation, || {
        if mutation == Mutation::SwapReadWriteEndpoints {
            differential("fs-read-at-offset-single", "rt_read_offset_stage");
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                differential("fs-write-at-offset-single", "rt_write_writable_stage")
            }))
        } else {
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                differential("fs-read-at-offset-single", "rt_read_offset_stage")
            }))
        }
    });
    assert!(
        red.is_err(),
        "{mode}: mutation did not redden its validator arm"
    );
    assert!(
        ken_runtime::checked_ih_continuation_inheritance_mutation_is_exact(),
        "{mode}: scoped mutation state did not restore exactly"
    );
}

generated_entry_checked_case!(continuation_inheritance_remove, CONTINUATION_INHERITANCE_MUTATION_CHILD, in_large_stack_thread, assert_continuation_inheritance_mutation_child, "remove", "not the exact closed forward derivation");
generated_entry_checked_case!(continuation_inheritance_duplicate, CONTINUATION_INHERITANCE_MUTATION_CHILD, in_large_stack_thread, assert_continuation_inheritance_mutation_child, "duplicate", "resolve more than one continuation inheritance");
generated_entry_checked_case!(continuation_inheritance_swap, CONTINUATION_INHERITANCE_MUTATION_CHILD, in_large_stack_thread, assert_continuation_inheritance_mutation_child, "swap", "does not reference one exact existing transport endpoint");
generated_entry_checked_case!(continuation_inheritance_break_step, CONTINUATION_INHERITANCE_MUTATION_CHILD, in_large_stack_thread, assert_continuation_inheritance_mutation_child, "break-step", "self-resumption step is disconnected");
generated_entry_checked_case!(continuation_inheritance_remove_k_locator, CONTINUATION_INHERITANCE_MUTATION_CHILD, in_large_stack_thread, assert_continuation_inheritance_mutation_child, "remove-k-locator", "does not have exactly one immediate K locator");
generated_entry_checked_case!(continuation_inheritance_duplicate_k_locator, CONTINUATION_INHERITANCE_MUTATION_CHILD, in_large_stack_thread, assert_continuation_inheritance_mutation_child, "duplicate-k-locator", "does not have exactly one immediate K locator");
generated_entry_checked_case!(continuation_inheritance_wrong_k_domain, CONTINUATION_INHERITANCE_MUTATION_CHILD, in_large_stack_thread, assert_continuation_inheritance_mutation_child, "wrong-k-domain", "wrong runtime environment domain");
generated_entry_checked_case!(continuation_inheritance_wrong_k_consumer, CONTINUATION_INHERITANCE_MUTATION_CHILD, in_large_stack_thread, assert_continuation_inheritance_mutation_child, "wrong-k-consumer", "different descendant invocation or callee");
generated_entry_checked_case!(continuation_inheritance_wrong_k_index, CONTINUATION_INHERITANCE_MUTATION_CHILD, in_large_stack_thread, assert_continuation_inheritance_mutation_child, "wrong-k-index", "does not equal its forward binder re-derivation");
generated_entry_checked_case!(continuation_inheritance_source_slot_k_locator, CONTINUATION_INHERITANCE_MUTATION_CHILD, in_large_stack_thread, assert_continuation_inheritance_mutation_child, "source-slot-k-locator", "wrong runtime environment domain");
generated_entry_checked_case!(continuation_inheritance_final_residual_k_locator, CONTINUATION_INHERITANCE_MUTATION_CHILD, in_large_stack_thread, assert_continuation_inheritance_mutation_child, "final-residual-k-locator", "wrong runtime environment domain");
generated_entry_checked_case!(continuation_inheritance_reclassify_ret, CONTINUATION_INHERITANCE_MUTATION_CHILD, in_large_stack_thread, assert_continuation_inheritance_mutation_child, "reclassify-ret", "reclassified as an induction hypothesis");
generated_entry_checked_case!(continuation_inheritance_descriptor_only, CONTINUATION_INHERITANCE_MUTATION_CHILD, in_large_stack_thread, assert_continuation_inheritance_mutation_child, "descriptor-only", "descriptor-only closure was substituted");
generated_entry_checked_case!(continuation_inheritance_earlier_result, CONTINUATION_INHERITANCE_MUTATION_CHILD, in_large_stack_thread, assert_continuation_inheritance_mutation_child, "earlier-result", "earlier transport source result was substituted");
generated_entry_checked_case!(continuation_inheritance_read_write_swap, CONTINUATION_INHERITANCE_MUTATION_CHILD, in_large_stack_thread, assert_continuation_inheritance_mutation_child, "read-write-swap", "does not reference one exact existing transport endpoint");

const D1_ROUTE_CONTROL_CHILD: &str = "KEN_RT_ITREE_D1_ROUTE_CONTROL_CHILD";

fn assert_d1_route_control_child() {
    let mode = std::env::var(D1_ROUTE_CONTROL_CHILD).expect("D1 child mode");
    let (case, entry, expected_family, expected_effects) = match mode.as_str() {
        "drop-read" | "unknown-read" | "direct-read" => (
            "fs-read-at-offset-single",
            "rt_read_offset_stage",
            "ITree",
            vec![
                ken_runtime::HostOpV1::FsOpen,
                ken_runtime::HostOpV1::BufferAllocate,
                ken_runtime::HostOpV1::ResourceRelease,
                ken_runtime::HostOpV1::ResourceRelease,
            ],
        ),
        "drop-write" => (
            "fs-write-at-offset-single",
            "rt_write_writable_stage",
            "ITree",
            vec![
                ken_runtime::HostOpV1::FsOpen,
                ken_runtime::HostOpV1::FsOpen,
                ken_runtime::HostOpV1::ResourceRelease,
                ken_runtime::HostOpV1::ResourceRelease,
            ],
        ),
        "ordinary-read" | "misroute-direct-read" => (
            "fs-read-at-offset-single",
            "rt_read_offset_stage",
            "ResourceBodyResult",
            vec![
                ken_runtime::HostOpV1::FsOpen,
                ken_runtime::HostOpV1::BufferAllocate,
                ken_runtime::HostOpV1::ResourceRelease,
                ken_runtime::HostOpV1::ResourceRelease,
            ],
        ),
        other => panic!("unknown D1 route-control child mode: {other}"),
    };
    let Differential { native, interpreted } = differential(case, entry);
    // Relocation (Architect re-rule evt_2427xbynt1d2e, Q2). SSA response
    // specialization statically supersedes the runtime route control for the
    // routes it specializes, so a runtime route-control perturbation of a
    // specialized route is INERT rather than reaching the ITree frontier. The
    // property therefore relocates onto native's actual outcome versus the
    // interpreter oracle, per-mode, with no assertion of an outcome that mode no
    // longer produces:
    //   - native traps  => the runtime route-control guard is still live for this
    //     (unspecialized) route; assert the exact fail-closed frontier family and
    //     pre-dispatch effect prefix, unchanged.
    //   - native returns => the route is statically specialized, so the
    //     perturbation is inert; native must then agree with the interpreter
    //     oracle EXACTLY. A native NormalReturn that diverged from a trapping
    //     interpreter would be arm (a) fail-open and fails here.
    match native.terminal_error.as_ref() {
        Some(ken_runtime::TerminalErrorV1::RuntimeTrap(provenance)) => {
            assert_eq!(
                provenance.trap.code,
                ken_runtime::RuntimeTrapCode::PatternMatchFailure
            );
            assert!(
                provenance
                    .trap
                    .message
                    .ends_with(&format!("::{expected_family}")),
                "{mode}: expected {expected_family} frontier, got {}",
                provenance.trap.message
            );
            assert_eq!(
                native
                    .effect_trace
                    .iter()
                    .map(|event| event.operation)
                    .collect::<Vec<_>>(),
                expected_effects,
                "{mode}: route perturbation must preserve the pre-dispatch effect prefix"
            );
        }
        _ => {
            assert_eq!(
                native.terminal_error, interpreted.terminal_error,
                "{mode}: a specialized route's inert perturbation must match the \
                 interpreter oracle's terminal outcome (a divergent NormalReturn is \
                 arm (a) fail-open)"
            );
            assert_eq!(
                native.exit_status, interpreted.exit_status,
                "{mode}: specialized-route native/interpreter exit parity"
            );
            assert_eq!(
                native.effect_trace, interpreted.effect_trace,
                "{mode}: specialized-route native/interpreter complete-effect parity"
            );
        }
    }
}

// Durable invariant. MEASURED: isolated mutation children replace each named
// active checked edge's control with Direct and return to
// the exact ITree default; an unknown active control also defaults; malformed
// initial Direct control cannot shadow an ordinary case; and a real
// recursor-layer Direct producer defaults unless that same edge is misrouted
// Checked. CLAIMED: both header edges consume only their exact route control,
// ordinary cases precede fallback, and all out-of-domain controls fail closed.
// THE GAP: these are test-support mutations at frame 1, not production route
// authority; the durable read/write InvalidOffset products below own the
// unmutated behavior.
d1_route_case!(d1_route_control_drop_read, "drop-read", Some("active-checked-to-direct"), None);
d1_route_case!(d1_route_control_drop_write, "drop-write", Some("active-checked-to-direct"), None);
d1_route_case!(d1_route_control_unknown_read, "unknown-read", Some("active-checked-to-unknown"), None);
d1_route_case!(d1_route_control_ordinary_read, "ordinary-read", Some("initial-direct-to-unknown"), None);
d1_route_case!(d1_route_control_direct_read, "direct-read", None, Some("drop-checked-frame-1"));
d1_route_case!(d1_route_control_misroute_direct_read, "misroute-direct-read", Some("active-direct-to-checked"), Some("drop-checked-frame-1"));

/// Durable invariant: a statically specialized read response preserves the
/// complete ordered effect/provenance trace and exposes exact InvalidOffset
/// without dispatching the malformed FsReadAt request.
#[test]
#[ignore = "post-M6 runtime parity debt: native construction completes, but execution traps on a malformed ExitCode::Failure payload instead of observing InvalidOffset"]
fn fs_read_at_malformed_offset_narrows_to_invalid_offset() {
    in_large_stack_thread("rt-parity-read-offset", || {
        assert_narrowed_alike(
            "fs-read-at-offset-single",
            "rt_read_offset_stage",
            ken_runtime::HostOpV1::FsReadAt,
            "InvalidOffset",
        )
    });
}

#[test]
#[ignore = "post-M6 runtime parity debt: native construction completes, but execution traps on a malformed ExitCode::Failure payload instead of observing InvalidBounds"]
fn fs_read_at_malformed_window_narrows_to_invalid_bounds() {
    in_large_stack_thread("rt-parity-read-window", || {
        assert_narrowed_alike(
            "fs-read-at-window-single",
            "rt_read_window_stage",
            ken_runtime::HostOpV1::FsReadAt,
            "InvalidBounds",
        )
    });
}

/// Overlapping fault: the same malformed offset as
/// `fs_read_at_malformed_offset_narrows_to_invalid_offset`, against a handle
/// opened write-only so the *read* right is not held. The two cases are a
/// non-degenerate pair -- identical program and identical malformed offset,
/// differing only in whether the coincident resource fault exists -- so a
/// narrowing that ran in the wrong order would fail exactly one of them.
///
/// Before the repair the sentinel entered dispatch and rights won, surfacing
/// `RightNotHeld`; native synthesised `InvalidOffset`.
///
/// The coincident fault here is a *rights* fault rather than a liveness one
/// because the liveness shape is not compilable: constructing a closed-but-
/// referenced resource requires escaping it from its bracket, and escaping a
/// second `Resource` through a bracket currently fails native lowering with
/// `OrientedSubcontinuationPlanV1: checked Runtime frame marker was consumed
/// more than once`. That is a pre-existing native lowering limitation, not an
/// RT-PARITY regression, and is reported rather than worked around; the
/// rights fault discriminates the same narrowing-order property.
#[test]
#[ignore = "post-M6 runtime parity debt: native construction completes, but execution traps on a malformed ExitCode::Failure payload instead of observing InvalidOffset"]
fn fs_read_at_malformed_offset_without_read_right_narrows_to_invalid_offset() {
    in_large_stack_thread("rt-parity-read-norights", || {
        assert_narrowed_alike(
            "fs-read-at-offset-overlap",
            "rt_read_norights_stage",
            ken_runtime::HostOpV1::FsReadAt,
            "InvalidOffset",
        )
    });
}

// -- FsWriteAt -----------------------------------------------------------
//
// Only `file_offset` is source-controllable. `writeAt` takes a `BufferSpan`,
// and no malformed span is constructible from checked source **at the landed
// surface**, so the `buffer_start`/`length` narrowings are not reachable here.
//
// That rests on the same empirical seal as the `BufferFreeze` case below --
// **not** on the privacy of `BufferSpan`'s constructor, which does not by
// itself establish that no public producer exists. The same qualifications
// carry over: the landed oracle's evidence is bounded and known
// enumeration-incomplete, `SEAL-2` owns the durable producer-enumeration gate,
// and if the seal or its future gate fails these narrowings owe executable
// coverage too. See
// `buffer_freeze_malformed_span_is_unconstructible_at_the_landed_surface`
// for the full statement of what that evidence does and does not support.
//
// Their coverage is the interpreter-level dispatch test, not this differential.

/// Durable invariant: a statically specialized write response preserves the
/// complete ordered effect/provenance trace and exposes exact InvalidOffset
/// without dispatching the malformed FsWriteAt request.
#[test]
#[ignore = "RT-CLOSURE-BOUNDARY-LANE: a runtime-local closure has no durable lane across the boundary; fails at base 21fd46dc"]
fn fs_write_at_malformed_offset_narrows_to_invalid_offset() {
    in_large_stack_thread("rt-parity-write-offset", || {
        assert_narrowed_alike(
            "fs-write-at-offset-single",
            "rt_write_writable_stage",
            ken_runtime::HostOpV1::FsWriteAt,
            "InvalidOffset",
        )
    });
}

/// Overlapping fault: the same malformed offset against a file opened
/// read-only, so the write right is not held. Before the repair the sentinel
/// entered dispatch and rights won, surfacing `RightNotHeld`; native
/// synthesised `InvalidOffset`.
#[test]
#[ignore = "post-M6 runtime parity debt: native construction completes, but execution traps on a malformed ExitCode::Failure payload instead of observing InvalidOffset"]
fn fs_write_at_malformed_offset_without_write_right_narrows_to_invalid_offset() {
    in_large_stack_thread("rt-parity-write-readonly", || {
        assert_narrowed_alike(
            "fs-write-at-offset-overlap",
            "rt_write_readonly_stage",
            ken_runtime::HostOpV1::FsWriteAt,
            "InvalidOffset",
        )
    });
}

// -- BufferFreeze --------------------------------------------------------

const SPAN_PROBE: &str = r#"program capabilities FS AFull
const rt_probe_span : BufferSpan = __RT_PARITY_SPAN__

proc main (_input : ProcessInput) (caps : ProgramCaps AFull)
  : HostIO AFull ExitCode visits [FS] =
  match caps {
MkProgramCaps cap |-> host_exit AFull Success
  }
"#;

fn elaborates(span_expression: &str, result_type: &str) -> Result<(), ken_cli::RunError> {
    let source = SPAN_PROBE
        .replace("__RT_PARITY_SPAN__", span_expression)
        .replace(": BufferSpan", &format!(": {result_type}"));
    ken_cli::run_program_effect_observation(
        &source,
        ken_cli::SourceFormat::Ken,
        &[],
        &[],
        b".",
        &mut ken_interp::CaptureHost::new(Vec::new()),
    )
    .map(|_| ())
}

/// `BufferFreeze` has no executable *narrowing* case, because no malformed span
/// is constructible from checked source **at the landed surface**.
///
/// That is an **empirical finding about the code as it stands**, not a derived
/// closure result. The distinction is load-bearing, and it is spelled out
/// because two earlier revisions of this comment claimed more than the evidence
/// supports: the first inferred it from constructor-name privacy, and the
/// second inferred it from an empty oracle result. Both were blocked.
///
/// **What is established.** Source-level span forgery is rejected today:
/// `PrivateBufferSpan` and the now-sealed `write_all_advance_span` are both
/// unnameable from checked source, pinned below. An independent adversary sweep
/// (SPAN-SEAL) separately found the seal holds, including a wrapped-inclusive
/// search and direct forgery attempts.
///
/// **What the landed oracle does and does not give.** `px8f_buffer_io_surface`
/// asserts that the set of public globals whose result type is `BufferSpan` is
/// empty, along three axes: modulo definitional equality
/// (`buffer_span_producer_closure_reduces_transparent_type_aliases`); over
/// declarations **and** constructors
/// (`buffer_span_producer_closure_resolves_public_constructors`); and with a
/// loud failure for any public id in neither category
/// (`buffer_span_producer_closure_rejects_unknown_public_ids`).
///
/// That evidence is **bounded and known enumeration-incomplete**. The walk is
/// head-only; it considers only ids already present in `env.globals`; and it
/// loads only the prelude plus the `Buffer` and `IO` catalog packages. It
/// therefore does not see wrapped result positions, class fields -- which are
/// source-reachable producers outside `env.globals` -- or producers in other
/// catalog packages, and its loud-failure arm totalizes classification only
/// *within* that partial enumeration.
///
/// **So an empty result from that oracle does not entail that every span
/// reaching `freeze` is host-minted, and nothing here makes that inference.**
/// Labelling the oracle "test-derived rather than proof" would not repair the
/// implication; it would only describe an insufficient test. The oracle is
/// corroborating evidence over the fragment it covers, and no more.
///
/// **`SEAL-2` owns the durable producer-enumeration gate.** It is deliberately
/// not built here.
///
/// **If this empirical seal breaks, or its future gate fails, `BufferFreeze`
/// owes executable single-fault and overlap differential coverage.** The
/// narrowing guards remain correct defense-in-depth and stay covered at the
/// dispatch boundary by
/// `eval::px5b_effect_observation_tests::rt_parity_buffer_freeze_*` and
/// `rt_parity_malformed_freeze_bounds_precede_closed_resource`.
///
/// **On the `TransferCount` pin below -- verified but ungated.** `TransferCount`
/// has no public producer empirically at the landed surface: every public
/// declaration mentioning it consumes one, and `PrivateTransferCount` is
/// sealed. That is a grep-verified fact with **no oracle behind it** -- the
/// landed oracle covers `BufferSpan` only, so nothing would catch a future
/// public `TransferCount` producer. That gap is also `SEAL-2`'s, and the pin is
/// retained here as defense in depth rather than as a load-bearing premise.
#[test]
fn buffer_freeze_malformed_span_is_unconstructible_at_the_landed_surface() {
    // Both private span producers are unnameable from checked source. This pins
    // the empirical seal at the differential layer; it does not enumerate the
    // producer surface, which is SEAL-2's job.
    for forged in [
        "PrivateBufferSpan (sub_int 0 1) Zero",
        "PrivateBufferSpan (0 : Int) Zero",
    ] {
        let error = elaborates(forged, "BufferSpan")
            .err()
            .unwrap_or_else(|| panic!("a source-forged BufferSpan must not elaborate: {forged}"));
        assert!(
            matches!(error, ken_cli::RunError::Elaboration(_)),
            "PrivateBufferSpan must be absent from source scope; got {error:?}"
        );
    }
    let error = elaborates(
        "write_all_advance_span rt_seed_span rt_seed_count",
        "BufferSpan",
    )
    .err()
    .expect("the sealed span transform must not elaborate");
    assert!(
        matches!(error, ken_cli::RunError::Elaboration(_)),
        "write_all_advance_span must stay sealed: it was the public transform \
         that defeated the earlier privacy argument; got {error:?}"
    );
    // Defense in depth, verified but ungated -- see SEAL-2 in the doc comment.
    let error = elaborates("PrivateTransferCount Zero Zero", "TransferCount")
        .err()
        .expect("a source-forged TransferCount must not elaborate");
    assert!(
        matches!(error, ken_cli::RunError::Elaboration(_)),
        "PrivateTransferCount must be absent from source scope; got {error:?}"
    );

    // Control: a public constructor of the same shape does elaborate, so the
    // rejections above are about scope and not about the probe's own form.
    elaborates("MkBufferWindow (sub_int 0 1) (1 : Int)", "BufferWindow")
        .expect("control: the public window constructor elaborates from source");
}

#[cfg(target_os = "linux")]
/// Promise class: durable invariant. Recut option-2 coverage
/// (Architect evt_55jt2yydg0661). The concrete "real-but-unrelated owner
/// substituted" arm of the SubstituteUnrelatedOwnerRoot mutation lost its only
/// exerciser under the recut: its sole prior host (the px8f writeAll fixture)
/// degraded to the empty-traversal 0-owner case (writeAll's responses are now all
/// Deferred), so every writeAll unrelated-owner-root run duplicates
/// suppress-graph-claims and the distinct real-owner substitution went uncovered.
/// This drives the SAME existing mutation against a program that carries real
/// specialization owners (the rt_parity READ stage), so a concrete unrelated legal
/// static-body owner is substituted as the retained-unit traversal root and the
/// retained body reaches its "has no graph-derived call target in this unit"
/// rejection from a genuine unrelated root, not the degenerate empty one.
#[cfg(target_os = "linux")]
const OPTION2_UNRELATED_OWNER_CHILD: &str = "KEN_RT_OPTION2_UNRELATED_OWNER_CHILD";

#[cfg(target_os = "linux")]
fn assert_option2_unrelated_owner_child() {
    let source = RT_PARITY_SOURCE.replace("__RT_PARITY_ENTRY__", "rt_read_offset_stage");
    let root = output_dir("option2-unrelated-owner-root");
    let result = ken_runtime::with_retained_unit_call_target_mutation(
        ken_runtime::RetainedUnitCallTargetMutation::SubstituteUnrelatedOwnerRoot,
        || {
            ken_cli::build_native_program(
                &source,
                ken_cli::SourceFormat::Ken,
                "rt_parity_option2_unrelated_owner",
                root.path(),
            )
        },
    );
    let error = result.expect_err(
        "substituting an unrelated owner root on a specialized-owner program must not compile",
    );
    let rendered = format!("{error:?}");
    assert!(
        rendered.contains("has no graph-derived call target in this unit"),
        "option-2: the unrelated-owner substitution must red the retained-body rejection; \
         got:\n{rendered}"
    );
}

#[cfg(target_os = "linux")]
/// Promise class: durable invariant. Recut option-2 coverage (Architect
/// evt_55jt2yydg0661; QA re-check evt_6tw7cdk4kmwt0). Restores an independent
/// positive control for the CONCRETE "real-but-unrelated owner substituted" arm of
/// SubstituteUnrelatedOwnerRoot, which lost its only exerciser when the writeAll
/// fixture degraded to the 0-owner empty-traversal case under the recut.
///
/// The 0-owner degenerate (empty-traversal) and the 1-owner concrete substitution
/// reach the IDENTICAL downstream "has no graph-derived call target in this unit"
/// rejection, so asserting only that string cannot tell the real substitution from
/// a silent fallback (QA's finding). The distinguishing signal is the provenance
/// line `substitute_unrelated_owner_roots` emits: the concrete branch prints "...
/// with unrelated legal static-body owner {id}", the degenerate branch prints "...
/// with an empty traversal ...". So the CHILD (spawned with the mutation active on
/// the rt_parity READ stage, which carries real specialization owners) asserts the
/// rejection, and the PARENT captures the child's stderr and asserts the CONCRETE
/// provenance -- proving the real 1-owner substitution was exercised.
#[test]
fn substitute_unrelated_owner_root_reds_on_a_specialized_owner_program() {
    if std::env::var_os(OPTION2_UNRELATED_OWNER_CHILD).is_some() {
        in_generated_entry_stack_thread(
            "rt-parity-option2-unrelated-owner-child",
            assert_option2_unrelated_owner_child,
        );
        return;
    }
    let output = std::process::Command::new(std::env::current_exe().expect("test binary"))
        .args([
            "--exact",
            "substitute_unrelated_owner_root_reds_on_a_specialized_owner_program",
            "--nocapture",
        ])
        .env(OPTION2_UNRELATED_OWNER_CHILD, "1")
        .env_remove("RUST_MIN_STACK")
        .output()
        .expect("spawn option-2 unrelated-owner child");
    assert!(
        output.status.success(),
        "option-2 child failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("with unrelated legal static-body owner"),
        "option-2: the child must exercise the CONCRETE 1-owner substitution, not the \
         0-owner empty-traversal degenerate (which prints 'with an empty traversal'); \
         stderr:\n{stderr}"
    );
}
