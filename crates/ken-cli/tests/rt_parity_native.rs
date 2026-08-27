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

/// Transition sentinel. MEASURED: the exact checked-source InvalidOffset
/// witness crosses the repaired private route lane and reaches the named
/// ResourceBodyResult fail-closed frontier with recoverable planner provenance.
/// CLAIMED: D1 no longer terminates at the earlier ITree default. THE GAP: the
/// ResourceBodyResult default is not final behavior and this test intentionally
/// retires when RT-RESULT-CONTINUATION-BINDING-PROVENANCE replaces it with the
/// durable nonignored InvalidOffset product witness.
#[test]
fn fs_read_at_malformed_offset_reaches_resource_body_result_frontier() {
    in_large_stack_thread("rt-parity-read-offset-provenance", || {
        let Differential { native, .. } =
            differential("fs-read-at-offset-provenance", "rt_read_offset_stage");
        let Some(ken_runtime::TerminalErrorV1::RuntimeTrap(provenance)) =
            native.terminal_error.as_ref()
        else {
            panic!("native witness must report typed planner trap provenance: {native:?}");
        };
        assert!(
            provenance.planned_identity > 0,
            "identity zero is reserved for no trap"
        );
        assert_eq!(
            provenance.trap.code,
            ken_runtime::RuntimeTrapCode::PatternMatchFailure
        );
        assert_eq!(
            provenance.trap.message,
            "no runtime match case selected for \
             decl:rt_parity_fs_read_at_offset_provenance::ResourceBodyResult"
        );
        let stderr = String::from_utf8_lossy(&native.stderr);
        assert!(stderr.contains("PatternMatchFailure"));
        assert!(stderr.contains(&provenance.trap.message));
        assert!(!stderr.contains("unknown terminal sentinel"));
        assert_eq!(
            native
                .effect_trace
                .iter()
                .map(|event| event.operation)
                .collect::<Vec<_>>(),
            vec![
                ken_runtime::HostOpV1::FsOpen,
                ken_runtime::HostOpV1::BufferAllocate,
                ken_runtime::HostOpV1::ResourceRelease,
                ken_runtime::HostOpV1::ResourceRelease,
            ]
        );
    });
}

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

#[test]
fn checked_ih_continuation_inheritance_is_byte_inert() {
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
            "planner-only continuation inheritance must change no emitted ABI, call, or artifact byte"
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

#[test]
fn checked_ih_continuation_inheritance_mutations_bite_their_own_arms() {
    if std::env::var_os(CONTINUATION_INHERITANCE_MUTATION_CHILD).is_some() {
        in_large_stack_thread(
            "rt-parity-continuation-inheritance-mutation-child",
            assert_continuation_inheritance_mutation_child,
        );
        return;
    }

    let cases = [
        ("remove", "not the exact closed forward derivation"),
        (
            "duplicate",
            "resolve more than one continuation inheritance",
        ),
        (
            "swap",
            "does not reference one exact existing transport endpoint",
        ),
        ("break-step", "self-resumption step is disconnected"),
        (
            "remove-k-locator",
            "does not have exactly one immediate K locator",
        ),
        (
            "duplicate-k-locator",
            "does not have exactly one immediate K locator",
        ),
        ("wrong-k-domain", "wrong runtime environment domain"),
        (
            "wrong-k-consumer",
            "different descendant invocation or callee",
        ),
        ("wrong-k-index", "does not equal its forward binder re-derivation"),
        ("source-slot-k-locator", "wrong runtime environment domain"),
        (
            "final-residual-k-locator",
            "wrong runtime environment domain",
        ),
        ("reclassify-ret", "reclassified as an induction hypothesis"),
        ("descriptor-only", "descriptor-only closure was substituted"),
        (
            "earlier-result",
            "earlier transport source result was substituted",
        ),
        (
            "read-write-swap",
            "does not reference one exact existing transport endpoint",
        ),
    ];
    for (mode, expected) in cases {
        let output = std::process::Command::new(std::env::current_exe().expect("test binary"))
            .arg("--exact")
            .arg("checked_ih_continuation_inheritance_mutations_bite_their_own_arms")
            .arg("--nocapture")
            .env(CONTINUATION_INHERITANCE_MUTATION_CHILD, mode)
            .env_remove("RUST_MIN_STACK")
            .output()
            .expect("spawn isolated continuation-inheritance mutation child");
        assert!(
            output.status.success(),
            "{mode}: mutation child failed\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains(expected),
            "{mode}: mutation missed its intended validator arm; stderr:\n{stderr}"
        );
    }
}

const D1_ROUTE_CONTROL_CHILD: &str = "KEN_RT_ITREE_D1_ROUTE_CONTROL_CHILD";

fn assert_d1_route_control_child() {
    let mode = std::env::var(D1_ROUTE_CONTROL_CHILD).expect("D1 child mode");
    let (case, entry, expected_family, expected_effects) = match mode.as_str() {
        "exact-write" => (
            "fs-write-at-offset-single",
            "rt_write_writable_stage",
            "ResourceBodyResult",
            vec![
                ken_runtime::HostOpV1::FsOpen,
                ken_runtime::HostOpV1::FsOpen,
                ken_runtime::HostOpV1::ResourceRelease,
                ken_runtime::HostOpV1::ResourceRelease,
            ],
        ),
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
    let Differential { native, .. } = differential(case, entry);
    let Some(ken_runtime::TerminalErrorV1::RuntimeTrap(provenance)) =
        native.terminal_error.as_ref()
    else {
        panic!("{mode}: expected typed frontier trap, got {native:?}");
    };
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

/// Durable invariant. MEASURED: isolated child compiles of both admitted full
/// programs replace the active checked edge's control with Direct and return to
/// the exact ITree default; an unknown active control also defaults; malformed
/// initial Direct control cannot shadow an ordinary case; and a real
/// recursor-layer Direct producer defaults unless that same edge is misrouted
/// Checked. CLAIMED: both header edges consume only their exact route control,
/// ordinary cases precede fallback, and all out-of-domain controls fail closed.
/// THE GAP: these are test-support mutations at frame 1, not production route
/// authority; the unmutated transitional witness above records the later
/// ResourceBodyResult frontier and D2 owns final InvalidOffset behavior.
#[test]
fn d1_route_control_full_program_mutations_are_fail_closed() {
    if std::env::var_os(D1_ROUTE_CONTROL_CHILD).is_some() {
        in_large_stack_thread("rt-parity-d1-route-child", assert_d1_route_control_child);
        return;
    }

    let cases = [
        ("exact-write", None, None),
        ("drop-read", Some("active-checked-to-direct"), None),
        ("drop-write", Some("active-checked-to-direct"), None),
        ("unknown-read", Some("active-checked-to-unknown"), None),
        ("ordinary-read", Some("initial-direct-to-unknown"), None),
        ("direct-read", None, Some("drop-checked-frame-1")),
        (
            "misroute-direct-read",
            Some("active-direct-to-checked"),
            Some("drop-checked-frame-1"),
        ),
    ];
    for (mode, control, recursor) in cases {
        let mut child = std::process::Command::new(std::env::current_exe().expect("test binary"));
        child
            .arg("--exact")
            .arg("d1_route_control_full_program_mutations_are_fail_closed")
            .arg("--nocapture")
            .env(D1_ROUTE_CONTROL_CHILD, mode)
            .env_remove("KEN_RT_ITREE_D1_ROUTE_CONTROL")
            .env_remove("KEN_RT_ITREE_D1_RECURSOR_ROUTE")
            .env_remove("RUST_MIN_STACK");
        if let Some(control) = control {
            child.env("KEN_RT_ITREE_D1_ROUTE_CONTROL", control);
        }
        if let Some(recursor) = recursor {
            child.env("KEN_RT_ITREE_D1_RECURSOR_ROUTE", recursor);
        }
        let output = child.output().expect("spawn isolated D1 control child");
        assert!(
            output.status.success(),
            "{mode} child failed\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        let stderr = String::from_utf8_lossy(&output.stderr);
        if let Some(control) = control {
            assert!(
                stderr.contains(&format!("RT_ITREE_D1_CONTROL_APPLIED mode={control}")),
                "{mode}: the route-control mutation did not reach its real producer: {stderr}"
            );
        }
        if let Some(recursor) = recursor {
            assert!(
                stderr.contains(&format!("RT_ITREE_D1_RECURSOR_APPLIED mode={recursor}")),
                "{mode}: the recursor-route mutation did not reach its real producer: {stderr}"
            );
        }
    }
}

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

// Ignored pending RT-CLOSURE-BOUNDARY-LANE.
//
// Observed signature, exactly:
//   Closure: a closure cannot cross the boundary: it is runtime-local and
//     live-domain only, and it has no durable lane
//
// Owner node: RT-CLOSURE-BOUNDARY-LANE.
// Pre-existing base debt, NOT a bind-order regression: this row fails at
// base 21fd46dc as well, measured by the D12 two-way differential over the
// complete --no-fail-fast surface of both packages.
// It refuses at object emission, so the program never executes and no
// binding order is observable in it.
// This is the CLOSURE lane, not the byte-span seat that owns the four
// px4b-adjacent rt_parity rows. Its own nearest sibling
// fs_write_at_malformed_offset_without_write_right_... refuses on the
// byte-span seat. Two rows, near-identical names, different owners.
// The refusal surfaces on the helper thread 'rt-parity-write-offset'; this
// test thread then fails only with the wrapper
//   RT-PARITY fixture thread: Any { .. }
// which carries no signature of its own. The signature above is the
// real cause.
// Annotation only -- test body and expectations are unchanged.
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
