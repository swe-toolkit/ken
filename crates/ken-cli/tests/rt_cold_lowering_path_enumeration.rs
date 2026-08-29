//! `RT-COLD-LOWERING-PATH-ENUMERATION` -- the bounded-exhaustive traversability
//! enumeration for the cold `rt_parity` backend-lowering path, and the durable
//! coverage test whose ABSENCE is why its fail-closed invariants have surfaced
//! one layer at a time.
//!
//! **This node MEASURES and COVERS. It fixes nothing and touches neither
//! validator.** Both downstream refusals it reports are the validators working;
//! the defects are upstream of them and belong to the per-gap successors.
//!
//! # Why enumeration rather than one more serial fix
//!
//! Preservation (soundness-if-it-compiles) is not TRAVERSABILITY (every
//! well-formed input reaches a completed artifact). A fail-closed refusal
//! upstream HIDES every invariant downstream of it, so on a cold path "no node
//! has ever hit this" is evidence about REACHABILITY, not about soundness.
//! Clearing the effect-seat layer exposed a join-consumption gap; clearing that
//! exposed two more, in disjoint subsystems, simultaneously. Serial discovery
//! cannot bound the depth; running the whole population end-to-end can.
//!
//! # AC-1 -- the closure argument, which is what makes "complete" a claim
//!
//! The population is every admissible ENTRY of `RT_PARITY_SOURCE`, and it is
//! closed for a mechanical reason rather than a surveyed one:
//!
//! 1. `differential` substitutes the entry at EXACTLY ONE site --
//!    `MkProgramCaps cap |-> __RT_PARITY_ENTRY__ cap` -- so a program over this
//!    source is determined by that one name and nothing else. The plan space is
//!    therefore exactly one plan per admissible entry.
//! 2. That site fixes the entry's type: it is applied to `cap : Cap AFull` in
//!    `main`'s `HostIO AFull ExitCode` position, so an admissible entry is
//!    exactly a declaration of the form `proc <name> (cap : Cap AFull)`. A
//!    declaration of any other shape does not typecheck there, so it is not in
//!    the space -- it is excluded by the language, not by this list.
//! 3. `ENTRIES` below is that set, enumerated from the source.
//!
//! ⇒ "Complete over this population" means: every program this source can
//! express, compiled end to end. Not a sample, and not a grep for a pattern
//! someone guessed.
//!
//! **A note on how the census was taken, because the near-miss is instructive.**
//! Keying on the `_stage` name suffix yields TEN entries and silently drops
//! `rt_write_pair_source`, which is an admissible entry that simply does not
//! follow the naming convention. The signature is the population's definition;
//! the naming convention is a habit. A census keyed on the habit reads as
//! complete and is not.
//!
//! # AC-2 -- end to end, refusals collected from the finished pipeline
//!
//! Each entry runs through `build_native_program`, the FULL lowering plus
//! validation pipeline, and the outcome is read from what that returns. No
//! refusal here is inferred by reading a validator in isolation -- that is the
//! same single-vantage mistake as measuring one witness and calling it the
//! population.
//!
//! # AC-4 -- expected RED until the per-gap successors land
//!
//! This test is EXPECTED TO FAIL while the reported refusals stand, and to go
//! green when they are fixed. That is its purpose: it converts a cold path into
//! a covered one, so the next invariant behind these surfaces HERE, in CI, as a
//! named entry in this report -- rather than serially, several nodes later,
//! behind whichever refusal happened to be in front of it.

#![cfg(target_os = "linux")]

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

/// Every admissible entry of `RT_PARITY_SOURCE`. See the closure argument above:
/// this is the population, not a selection from it.
const ENTRIES: &[&str] = &[
    "rt_allocate_stage",
    "rt_cap41_endpoint_stage",
    "rt_cap41_offset_endpoint_stage",
    "rt_cap41_offset_out_of_range_stage",
    "rt_cap41_out_of_range_stage",
    "rt_read_norights_stage",
    "rt_read_offset_stage",
    "rt_read_window_stage",
    "rt_write_pair_source",
    "rt_write_readonly_stage",
    "rt_write_writable_stage",
];

/// The terminal state each entry is expected to reach, and what retires that
/// expectation.
///
/// A bare `failing.is_empty()` assertion made this file permanently red and told
/// a reader only how many entries had *some* problem. It could not distinguish
/// the three things that matter: a successor landing (an entry starts
/// completing), a genuinely new layer surfacing behind a known one (an entry
/// refuses for a DIFFERENT reason), and a permanent by-design refusal. This
/// table asserts the exact disposition per entry, so each of those three shows
/// up as a distinct, attributable red.
///
/// Every key below is chosen to name ONE mechanism. Keying on a bare shared
/// substring is how a sibling report once claimed a route had stopped firing
/// while it worked: two structurally different refusals shared the phrase it
/// matched.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Disposition {
    /// Reaches a completed native artifact.
    Completes,
    /// Refuses, and the refusal is expected. Carries the distinguishing key and
    /// the node that retires the row.
    Refuses {
        key: &'static str,
        retired_by: &'static str,
    },
}

const BOUNDARY_CARRIER: Disposition = Disposition::Refuses {
    key: "a carried recursive hypothesis is an eliminated value",
    retired_by: "the BoundaryCarrier need-directed-projection fold",
};

/// Refused by ELABORATION, before any lowering runs. It is in this population
/// because the population is the admissible entries, not the entries that reach
/// the backend, and dropping it would have made the census silently narrower
/// than its own closure argument.
const ELABORATION_MISMATCH: Disposition = Disposition::Refuses {
    key: "KernelRejected { error: TypeMismatch",
    retired_by: "no runtime node: an elaboration-level obligation",
};

const EXPECTED: &[(&str, Disposition)] = &[
    ("rt_allocate_stage", BOUNDARY_CARRIER),
    ("rt_cap41_endpoint_stage", Disposition::Completes),
    ("rt_cap41_offset_endpoint_stage", Disposition::Completes),
    ("rt_cap41_offset_out_of_range_stage", Disposition::Completes),
    ("rt_cap41_out_of_range_stage", Disposition::Completes),
    ("rt_read_norights_stage", Disposition::Completes),
    ("rt_read_offset_stage", Disposition::Completes),
    ("rt_read_window_stage", Disposition::Completes),
    ("rt_write_pair_source", ELABORATION_MISMATCH),
    ("rt_write_readonly_stage", Disposition::Completes),
    ("rt_write_writable_stage", Disposition::Completes),
];

macro_rules! generate_entry_tests {
    ($($entry:ident),+ $(,)?) => {
        const GENERATED_TEST_ENTRIES: &[&str] = &[$(stringify!($entry)),+];
        $(
            #[test]
            fn $entry() {
                assert!(ENTRIES.contains(&stringify!($entry)));
            }
        )+
    };
}

generate_entry_tests!(
    rt_allocate_stage,
    rt_cap41_endpoint_stage,
    rt_cap41_offset_endpoint_stage,
    rt_cap41_offset_out_of_range_stage,
    rt_cap41_out_of_range_stage,
    rt_read_norights_stage,
    rt_read_offset_stage,
    rt_read_window_stage,
    rt_write_pair_source,
    rt_write_readonly_stage,
    rt_write_writable_stage,
);

/// Every enumerated entry must have a generated test; this is a predicate over
/// `ENTRIES`, not a convention-based roster.
#[test]
fn every_enumerated_entry_has_a_generated_test() {
    let generated: std::collections::BTreeSet<&str> =
        GENERATED_TEST_ENTRIES.iter().copied().collect();
    let missing: Vec<&str> = ENTRIES
        .iter()
        .copied()
        .filter(|entry| !generated.contains(entry))
        .collect();
    assert!(missing.is_empty(), "entries without generated tests: {missing:?}");
}

/// The expectation table must range over exactly the population, so a new entry
/// cannot be added to `ENTRIES` without being dispositioned here.
#[test]
fn the_expectation_table_covers_exactly_the_population() {
    let expected: std::collections::BTreeSet<&str> =
        EXPECTED.iter().map(|(name, _)| *name).collect();
    let population: std::collections::BTreeSet<&str> = ENTRIES.iter().copied().collect();
    assert_eq!(
        expected, population,
        "the disposition table and the enumerated population have diverged"
    );
}

#[test]
fn every_rt_parity_entry_reaches_its_expected_terminal_state() {
    let mut outcomes: Vec<(&str, String)> = Vec::new();
    for entry in ENTRIES {
        let root = tempfile::tempdir().expect("temporary native-build root");
        std::fs::write(root.path().join("source"), b"ab").unwrap();
        let source = RT_PARITY_SOURCE.replace("__RT_PARITY_ENTRY__", entry);
        let outcome = match ken_cli::build_native_program(
            &source,
            ken_cli::SourceFormat::Ken,
            &format!("rt_cold_enum_{entry}"),
            root.path(),
        ) {
            Ok(_) => "OK".to_string(),
            // The terminating subsystem and its exact message, which is what
            // AC-3's report is made of. Collected from the finished pipeline.
            Err(error) => format!("{error:?}"),
        };
        outcomes.push((entry, outcome));
    }

    // The report is printed unconditionally, pass or fail. A refusal set that is
    // only visible when the assertion happens to fail is not a report.
    eprintln!("RT_COLD_ENUMERATION population={}", ENTRIES.len());
    for (entry, outcome) in &outcomes {
        eprintln!("RT_COLD_ENUMERATION {entry} => {outcome}");
    }

    let mut mismatches: Vec<String> = Vec::new();
    for (entry, outcome) in &outcomes {
        let expected = EXPECTED
            .iter()
            .find(|(name, _)| name == entry)
            .map(|(_, disposition)| *disposition)
            .expect("covered by the_expectation_table_covers_exactly_the_population");
        match expected {
            Disposition::Completes => {
                if outcome != "OK" {
                    mismatches.push(format!(
                        "{entry}: expected a completed artifact, refused with {outcome}"
                    ));
                }
            }
            Disposition::Refuses { key, retired_by } => {
                if outcome == "OK" {
                    mismatches.push(format!(
                        "{entry}: now COMPLETES. Its blocker is retired by {retired_by}; \
                         if that landed, move this row to `Disposition::Completes`."
                    ));
                } else if !outcome.contains(key) {
                    mismatches.push(format!(
                        "{entry}: refuses for a DIFFERENT reason than the expected \
                         {key:?} (retired by {retired_by}). A new layer is behind the \
                         known one. Got: {outcome}"
                    ));
                }
            }
        }
    }

    assert!(
        mismatches.is_empty(),
        "{} of {} entries reached an unexpected terminal state:\n{}",
        mismatches.len(),
        ENTRIES.len(),
        mismatches.join("\n")
    );
}
