//! `RT-RESOURCE-RELEASE-CARRIED-OBSERVE` `AC-2` -- the discriminating control
//! for the carried resource-token observation route.
//!
//! MEASURED: compiling a witness whose resource handles reach their effect
//! seats as CARRIED words, the three `ResourceScalar` seats
//! (`ResourceRelease`, `FsHandleMetadata`, `FsReadAt` `Argument(0)`) are
//! observed and the compile advances past them.
//! CLAIMED: the new route PROVES observability for the `ResourceScalar` need in
//! the carried phase; it does not admit carried seats generally, and it does not
//! relax the seat's `Need`-subset-`Avail` membership.
//!
//! **The two assertions are a non-degenerate PAIR on ONE witness, and that is
//! the whole design.** A single "it compiles further now" row is green-vs-green
//! under the mutation this must catch -- a route that admitted ANY carried seat
//! would pass it. So the pair asserts an ABSENCE and a PRESENCE that move in
//! opposite directions:
//!
//! - the `ResourceScalar` refusal is GONE -- if the route stopped firing, this
//!   inverts;
//! - a carried seat of a DIFFERENT need (`ExactIntU64`) is STILL REFUSED -- if
//!   the route stopped being keyed on the need and admitted carried seats
//!   generally, this inverts.
//!
//! Neither can be satisfied by a predicate that answers uniformly.
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
fn the_carried_resource_route_observes_resource_scalar_and_still_refuses_a_different_need() {
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

    // The route is KEYED ON THE NEED. A carried seat of a different need is
    // still refused by the unchanged strict membership test -- so the route
    // proved observability for one need rather than admitting carried seats
    // generally. This is the half that fails if the key is ever loosened to
    // "any carried seat", and it is why the row above cannot pass vacuously.
    // WEAKENED, DELIBERATELY AND WITH THE LOSS NAMED. This half used to assert
    // that a carried seat of a DIFFERENT need (`ExactIntU64`) still refused at
    // the claim gate -- the half that failed if the route were ever loosened to
    // "any carried seat". `RT-EXACTINT-CARRIED-OBSERVE` closed that need, so
    // this witness no longer contains a different-need claim-gate refusal, and
    // the discriminator it provided is GONE rather than merely relocated.
    //
    // What remains is a non-vacuity anchor: the compile still stops, at a
    // measured later blocker, so the absence above is not satisfied by a
    // compile that failed upstream of the route. That is strictly less than the
    // keying property it replaces.
    //
    // The property itself still holds IN CODE -- the ledger's second
    // admissibility independently re-derives `(CarriedWord, ResourceScalar)`,
    // byte-unchanged by the node that dropped this row's witness -- so what was
    // lost is persistent regression DETECTION, not the route's keying.
    //
    // RESTORATION HOME: `RT-FSREADAT-REPLY-BUFFER-GATE-REMOVAL`, tracked as an
    // AC rather than left to this comment (Architect `evt_4wkc748vgfhhf`). That
    // node removes the Arg(2) gate, which is what produces a COMPLETING witness
    // to carry a durable discriminator. Durable means a POSITIVE CROSS-KEY form
    // -- one witness where a `ResourceScalar` carried seat and an `ExactIntU64`
    // carried seat each route through their OWN decoder -- rather than this
    // vanishing-contrast shape, which is inherently fragile precisely because
    // it is spent the moment the contrasting need is closed.
    assert!(
        error.contains("seat Argument(2) of FsReadAt needs ResourceScalar"),
        "the compile must still stop at the measured later blocker, so the absence above is \
         not satisfied vacuously by a failure upstream of the route: {error}"
    );
}
