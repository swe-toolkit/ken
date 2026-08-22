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
//! **The pair, and why it is not the previous nodes' shape.** This need has no
//! "REFUSE" half to assert: an out-of-range carried `Int` is not an error, it
//! returns `valid = 0` into the operation's EXISTING narrow-failure lane
//! (`InvalidOffset` / `InvalidBounds`) exactly as an out-of-range specialized
//! `Int` already did. So the two assertions are:
//!
//! - the `ExactIntU64` carried refusal is GONE -- if the `Avail` move or the
//!   reader were reverted, this inverts;
//! - the terminal is the NEXT distinct blocker and not something upstream -- if
//!   the seats were admitted without the paired reader, the compile would stop
//!   at `ClaimedEffectSeats::specialized`'s tripwire instead, which is the
//!   claim-admitted-read-refuses shape this pairing exists to prevent.
//!
//! **WHAT IS NOT OBSERVABLE YET, stated rather than left as a gap.** The
//! runtime discrimination the Architect specified -- in-range carried `Int`
//! advances, out-of-range yields `valid = 0` into the narrow-failure lane --
//! CANNOT be observed on this witness today, because the compile does not
//! complete: it now terminates at `FsReadAt` `Argument(2)`'s buffer reply-path
//! gate, which is `RT-FSREADAT-REPLY-BUFFER-GATE-REMOVAL`'s to remove. The
//! fixture for that half is a runtime observation and it becomes writable the
//! moment that node lands.
//!
//! RESTORATION HOME: `RT-FSREADAT-REPLY-BUFFER-GATE-REMOVAL`, tracked as an AC
//! there rather than left to this comment (Architect `evt_4wkc748vgfhhf`).
//! Removing the Arg(2) gate is what lets the compile complete, which is exactly
//! when the runtime half becomes observable end-to-end. Note what IS already
//! proven: `narrow_carried_int_u64` is the same decoder `BufferAllocate` `0`
//! ships and runs today, so what is missing is end-to-end coverage ON THIS
//! OPERATION, not an unproven mechanism.
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
fn the_positioned_int_seats_decode_carried_and_stop_at_the_next_distinct_blocker() {
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

    // The measured next distinct blocker, which is what keeps the absences
    // above non-vacuous: without it they would also hold on a compile that
    // failed for some unrelated earlier reason.
    //
    // REPOINTED by `RT-FSREADAT-REPLY-BUFFER-GATE-REMOVAL`. The old terminal was
    // the Arg(2) gate this very node removes -- named above as the next node's
    // to remove, and this candidate is that node -- so the witness advances past
    // its own former stop and the move is downstream by construction. The new
    // terminal is a different subsystem again: the checked-IH nullary force of
    // an ESCAPING functional IH, a deferred capability gap owned by
    // `RT-CHECKED-IH-FUNCTIONAL-REPRESENTATION`.
    assert!(
        error.contains("static worker expects 1 arguments but call provides 0"),
        "the expected terminal after this node is the checked-IH nullary force of an \
         escaping functional IH -- a different need and reader again, and a deferred \
         capability gap: {error}"
    );
}
