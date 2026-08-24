//! `RT-RESOURCE-RELEASE-CARRIED-OBSERVE` `AC-2` -- the discriminating control
//! for the carried resource-token observation route.
//!
//! MEASURED: compiling a witness whose resource handles reach their effect
//! seats as CARRIED words, the three `ResourceScalar` seats
//! (`ResourceRelease`, `FsHandleMetadata`, `FsReadAt` `Argument(0)`) are
//! observed and the compile advances past them.
//! CLAIMED: the new route proves observability for the `ResourceScalar` need in
//! the carried phase without relaxing the seat's `Need`-subset-`Avail`
//! membership. The ledger still derives that key structurally; this fixture no
//! longer has a different-need refusal with which to re-prove the keying.
//!
//! **The pair after the sibling readers and M6 closure:**
//!
//! - the `ResourceScalar` refusal is GONE -- if the route stopped firing, this
//!   inverts;
//! - native-program construction COMPLETES -- the positive non-vacuity anchor
//!   proving this shared witness traverses the resource reader, the exact-Int
//!   sibling reader, and the M6 representation.
//!
//! **The phase axis is not tested here and the reason is structural, not an
//! omission.** The route's condition requires `CarriedWord`, and in the
//! specialized phase `avail.admits` is true so `Direct` takes the claim
//! unchanged -- the route is the exact complement of where `Direct` serves. The
//! specialized path is exercised, unchanged, by every existing green
//! resource-seat row (`BufferFreeze`, and the specialized `ResourceRelease`
//! rows). A fixture asserting "the specialized phase still works" would be
//! green before and after the route existed, so it would discriminate nothing.
//!
//! **On the guard-failure negative half, stated rather than left implicit
//! (Architect `evt_3dnd21pjg193g`, Steward `evt_5xq3hw23kamrd`):** a carried
//! word failing the `InvocationBorrowed` / `BorrowedOpaque` guards takes a
//! RUNTIME failure return -- the guards runtime-dominate `emit_carrier_scalar`,
//! so the scalar is never read from a word not proven a borrowed-opaque handle.
//! A compile-time refusal is impossible here by construction, and manufacturing
//! one would defeat the route's purpose. **A malformed-carried-word negative is
//! not expressible in well-typed Ken**: what may reach a `ResourceScalar` seat
//! is fixed by the operation's contract and the elaborator's typing, so the
//! guard-failure path is DEFENSIVE-UNREACHABLE for source programs -- it defends
//! against a boundary/representation defect, not a source-level one. That is a
//! reasoned claim about the type system, not a measurement.
//!
//! **WHAT THE GUARDS DO AND DO NOT PROVE, stated precisely because the
//! difference is the soundness grounding** (Architect `evt_3q6v5gc6ta6qs`):
//!
//! - The guards prove **"a borrowed-opaque invocation handle"** -- NOT
//!   "specifically a resource token". `CapabilityToken` and the borrowed
//!   native/option variants share `InvocationBorrowed` / `BorrowedOpaque`.
//! - **The PRIMARY discriminator is the seat's `ResourceScalar` contract plus
//!   Ken's typing**, which is what restricts each of these three seats' carried
//!   population to resource tokens. Resource-token-ness rests on THAT.
//! - The guards are the **precedent's inherited runtime re-check on top of it**,
//!   never the membership proof.
//!
//! This is why the widening adds no assumption: it is the property
//! `lower_buffer_freeze_resource_seat` already relies on, at a resource seat, on
//! the same basis -- inherited unchanged across three seats of the SAME kind
//! (all `ResourceScalar` resource-handle argument positions), not introduced
//! here. The route reads through that SHARED function precisely so the
//! observation is the one already proven in production rather than a new one.
//!
//! And the uniformity the widening rests on is STRUCTURAL, not sampled: a
//! `Lowered`'s boundary tag/class is chosen by one `match` on its
//! `LoweredVariant` with no consuming operation in scope, so these seats do not
//! happen to agree -- they must.

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
fn the_carried_resource_route_observes_resource_scalar_and_completes_native_construction() {
    let root = tempfile::tempdir().expect("temporary native-build root");
    let result = ken_cli::build_native_program(
        WITNESS_SOURCE,
        ken_cli::SourceFormat::Ken,
        "rt_resource_release_carried_observe",
        root.path(),
    );
    let error = format!("{result:?}");

    // The route FIRED. Every `ResourceScalar` seat on this path -- across
    // `ResourceRelease`, `FsHandleMetadata` and `FsReadAt` `Argument(0)` -- is
    // now observed in the carried phase rather than refused.
    // KEYED ON THE CLAIM-GATE WORDING, not on the bare need name. The original
    // form asserted `!contains("needs ResourceScalar")`, and that substring does
    // NOT discriminate: it appears in TWO structurally different refusals -- the
    // claim-gate membership one this route is about ("...which it cannot observe
    // in CarriedWord") and `ClaimedEffectSeats::specialized`'s READER tripwire
    // ("...which this release can observe only in a specialized template").
    //
    // It cost a false signal to find out. When `RT-EXACTINT-CARRIED-OBSERVE`
    // advanced this witness to the Arg(2) reader tripwire, this row failed
    // printing "the route stopped firing" while the route was working perfectly
    // -- a message that would have sent the next reader hunting the wrong
    // mechanism. A control must key on the refusal it is ABOUT, not on a word
    // the refusal happens to contain.
    assert!(
        !error.contains("needs ResourceScalar, which it cannot observe in CarriedWord"),
        "the carried resource-token route must observe the ResourceScalar need at the CLAIM \
         GATE; a surviving membership refusal in that exact wording means the route stopped \
         firing: {error}"
    );

    // Completion is now the positive non-vacuity anchor for the ResourceScalar
    // assertion above: the witness traverses the carried resource reader and
    // the exact-Int sibling reader before M6 closes the former terminal refusal.
    result.expect("the carried resource-scalar reader and M6 representation must complete");
}
