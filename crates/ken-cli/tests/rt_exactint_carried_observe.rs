//! `RT-EXACTINT-CARRIED-OBSERVE` `AC-3` -- the discriminating control for the
//! carried exact-`Int` decode.
//!
//! MEASURED: with the positioned arm's exact-`Int` seats classified
//! `carried_exact_int` and read through `narrow_positioned_int_seat`, a witness
//! whose offsets and lengths arrive as CARRIED words decodes all three live
//! seats (`FsReadAt` `Argument(1)`, `(3)`, `(4)`) instead of refusing.
//! CLAIMED: the `Avail` move is paired with a reader that is TOTAL over both
//! phases, so no seat is admitted in a phase this cannot decode.
//!
//! **The pair after M6 closure.** This need has no "REFUSE" half to assert: an
//! out-of-range carried `Int` is not an error, it returns `valid = 0` into the
//! operation's EXISTING narrow-failure lane (`InvalidOffset` /
//! `InvalidBounds`) exactly as an out-of-range specialized `Int` already did.
//! The fixture therefore asserts:
//!
//! - the `ExactIntU64` carried refusal is GONE -- if the `Avail` move or the
//!   reader were reverted, this inverts;
//! - native-program construction COMPLETES -- the positive non-vacuity anchor
//!   proving the shared witness traverses this reader and the M6 representation.
//!
//! **WHAT THIS FIXTURE DOES NOT CLAIM.** `build_native_program` constructs the
//! executable but does not run it, so it does not independently observe the
//! in-range/out-of-range runtime split on `FsReadAt`. The shipped reader remains
//! `narrow_carried_int_u64`, the same decoder used by the executing
//! `BufferAllocate` path; this fixture pins reader admission plus completed
//! lowering, not runtime result values.
//!
//! **On the representation fail-close** (a word at the seat that is not a
//! decodable `Int` at all): NOT expressible in well-typed Ken -- what may reach
//! an `ExactIntU64` seat is fixed by the operation's contract and the
//! elaborator's typing -- so that path is DEFENSIVE-UNREACHABLE for source
//! programs. It is the `require_i64` on the viewed decode's status, and it
//! defends against a boundary/representation defect rather than a source-level
//! one. A reasoned claim about the type system, marked as reasoned, not a
//! measurement -- and deliberately not manufactured into a compile-time refusal
//! this need does not have.

#![cfg(target_os = "linux")]

const WITNESS_SOURCE: &str = r#"program capabilities FS AFull
fn rt_branched_body (_buffer : BufferHandle)
  : HostIO AFull (ResourceBodyResult Unit Unit) =
  Ret (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (ResourceBodyResult Unit Unit) (ResourceBodyOk Unit Unit MkUnit)

proc rt_branched_endpoint_buffer
  (file : Resource FsHandle) (buffer : BufferHandle)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError ReadProgress) (ResourceBodyResult Unit Unit)
    (readAt AFull file (0 : Int) buffer
      (MkBufferWindow (8 : Int) (4 : Int)))
    (\outcome. match outcome {
      Err error |-> Ret (Coproduct (FSOp AFull) AmbientOp)
        (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
        (ResourceBodyResult Unit Unit) (ResourceBodyErr Unit Unit MkUnit);
      Ok progress |-> Ret (Coproduct (FSOp AFull) AmbientOp)
        (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
        (ResourceBodyResult Unit Unit) (ResourceBodyOk Unit Unit MkUnit)
    })

proc rt_branched_after_buffer
  (outcome : Result ResourceError (ResourceBracketResult Unit Unit))
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  match outcome {
    Err error |-> Ret (Coproduct (FSOp AFull) AmbientOp)
      (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
      (ResourceBodyResult Unit Unit) (ResourceBodyErr Unit Unit MkUnit);
    Ok bracket |-> Ret (Coproduct (FSOp AFull) AmbientOp)
      (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
      (ResourceBodyResult Unit Unit) (ResourceBodyErr Unit Unit MkUnit)
  }

proc rt_branched_file (file : Resource FsHandle)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError (ResourceBracketResult Unit Unit))
    (ResourceBodyResult Unit Unit)
    (withBuffer AFull Unit Unit (8 : Int) (rt_branched_endpoint_buffer file))
    (\outcome. rt_branched_after_buffer outcome)

proc rt_branched_done
  (outcome : Result FileError (ResourceBracketResult Unit Unit))
  : HostIO AFull ExitCode visits [FS] =
  match outcome {
    Err error |-> host_exit AFull (Failure 71);
    Ok bracket |-> host_exit AFull (Failure 72)
  }

proc rt_branched_stage (cap : Cap AFull)
  : HostIO AFull ExitCode visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result FileError (ResourceBracketResult Unit Unit)) ExitCode
    (withResource AFull Unit Unit cap (bytes_encode "source")
      ResourceRead rt_branched_file)
    (\outcome. rt_branched_done outcome)

proc main (_input : ProcessInput) (caps : ProgramCaps AFull)
  : HostIO AFull ExitCode visits [FS] =
  match caps {
    MkProgramCaps cap |-> rt_branched_stage cap
  }
"#;

#[test]
fn the_positioned_int_seats_decode_carried_and_complete_native_construction() {
    let root = tempfile::tempdir().expect("temporary native-build root");
    let result = ken_cli::build_native_program(
        WITNESS_SOURCE,
        ken_cli::SourceFormat::Ken,
        "rt_exactint_carried_observe",
        root.path(),
    );
    let error = format!("{result:?}");

    // The Avail move AND its reader are both in. Reverting either inverts this.
    //
    // KEYED ON THE CLAIM-GATE WORDING, not the bare need name -- the same fix
    // this PR made to the predecessor's control, applied here before it can bite
    // (Architect `evt_31zjs18gm7egk`). A need name is exactly the kind of word
    // that several structurally different gates all mention: `needs ExactIntU64`
    // appears both in this membership refusal and in
    // `ClaimedEffectSeats::specialized`'s reader tripwire asserted below, so the
    // bare form would stop discriminating the moment the terminal moved -- which
    // on this chain is every node.
    assert!(
        !error.contains("needs ExactIntU64, which it cannot observe in CarriedWord"),
        "the positioned exact-Int seats must decode in the carried phase; a surviving \
         claim-gate membership refusal in that exact wording means the Avail move or its \
         reader is missing: {error}"
    );

    // The seats were admitted WITH a reader that can decode them. If the Avail
    // move had landed without the paired reader, the compile would stop at
    // `ClaimedEffectSeats::specialized`'s tripwire for an ExactIntU64 seat --
    // the claim-admitted-read-refuses shape -- rather than reaching the next
    // distinct blocker below.
    assert!(
        !error.contains("of FsReadAt needs ExactIntU64, which this release can observe only"),
        "an ExactIntU64 seat must not be admitted in a phase its reader cannot decode: {error}"
    );

    // Completion is now the positive non-vacuity anchor for both reader
    // assertions above: M6 retired the terminal checked-IH refusal without
    // weakening either exact-Int observation boundary.
    result.expect("the carried exact-Int readers and M6 representation must complete");
}
