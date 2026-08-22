//! `RT-CAPTURE-PROJECTION-GROW` `D1` — the AC-2 discriminating control.
//!
//! MEASURED: with the worker-prefix deferral ledger installed, compiling a
//! witness-shaped checked program records (a) the grown capture projection and
//! (b) every edge whose worker prefix was DEFERRED rather than joined.
//! CLAIMED: the conditional join grows the projection where the demand fits the
//! continuation's environment, and defers — never silently drops — the edges
//! whose workers reference values outside it.
//!
//! Why this control exists: without it, D1's effectiveness lives only in a
//! handback message and the ledger's mere existence. A deferral that stopped
//! being recorded, or a join that silently stopped growing, would both stay
//! green.

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
fn the_conditional_join_grows_the_projection_and_records_every_deferral() {
    let root = tempfile::tempdir().expect("temporary native-build root");
    let (result, deferrals) = ken_runtime::with_worker_prefix_deferrals(|| {
        ken_cli::build_native_program(
            WITNESS_SOURCE,
            ken_cli::SourceFormat::Ken,
            "rt_capture_projection_grow",
            root.path(),
        )
    });
    eprintln!("RT_CAPTURE_PROJECTION_GROW_DEFERRALS {deferrals:?}");

    // ── The deferral half: recorded, never silently dropped ──────────────────
    assert!(
        !deferrals.is_empty(),
        "the ledger must record the deferred edges; an empty ledger means either \
         the join stopped running or deferrals stopped being recorded, and both \
         would leave every assertion below vacuous"
    );

    // ⛔ The DEFINING invariant of a deferral, asserted over every row rather
    // than on a representative: a row is deferred exactly because its demand
    // exceeds the environment. A row with `demand <= reached` in this ledger
    // would mean the conditional joined the wrong way.
    for row in &deferrals {
        assert!(
            row.demand > row.reached,
            "a deferred edge must be one whose demand exceeds `reached`: {row:?}"
        );
    }

    // The measured numbers for this witness shape. `depth = 1` is the producer
    // `Match` arm's binder — the whole reason the prefix is measured at depth
    // rather than at zero.
    for row in &deferrals {
        assert_eq!(row.depth, 1, "producer arm binder depth: {row:?}");
        assert_eq!(row.demand, 6, "worker prefix demand: {row:?}");
        assert_eq!(row.reached, 4, "continuation environment: {row:?}");
    }

    // ⚠ The construct ORIGIN is deliberately not frozen to a literal. It is a
    // planner-assigned ordinal, and adding an unrelated binding upstream
    // renumbers the occurrence table without changing anything this test is
    // about — the same renumbering `AbiDescriptor` already documents for
    // `planned_node`. What IS asserted is that the rows agree on one origin, so
    // a ledger that started mixing edges would still fail.
    //
    // ⚠ Compared through `edge()`, an opaque identity: `StaticOriginId` is
    // crate-private to `ken-runtime`, so a consumer cannot name the type — and
    // the opacity is the point, since it commits this test to no id VALUE.
    let first = deferrals[0].edge();
    assert!(
        deferrals.iter().all(|row| row.edge() == first),
        "every deferral here belongs to the one over-demanding edge: {deferrals:?}"
    );

    // ── The conditional half: the join must never veto the continuation ──────
    //
    // ⭐ This is the assertion that would have caught the unconditional form.
    // Joining an over-demanding edge forces `required_input_count` above
    // `reached` and refuses the WHOLE continuation, taking down sibling edges
    // that were coverable. That failure is a planner error, textually distinct
    // from the lowering refusal this population is expected to reach.
    let error = format!("{result:?}");
    assert!(
        !error.contains("lacks its complete semantic value environment"),
        "the conditional join must never force `required_input_count` above \
         `reached`; this is the planner veto the unconditional form produced: {error}"
    );

    // The terminal state, AMENDED by `RT-CAPTURE-CONTEXT-FRAME-EMIT` `D2`.
    //
    // This clause used to read "the expected terminal state is the unchanged
    // BoundaryCarrier refusal", and it said in its own comment that if that
    // ever stopped holding, the expectation was to be **revisited, not deleted
    // silently**. `D2` is that event, and this is the revision.
    //
    // `D2` constructs the generated context's frame at the creation site and
    // supplies it at the carried-invocation retarget. That clears **two**
    // refusals this witness used to stop at, and the two are asserted
    // separately because they are two distinct claims about what `D2` did:
    //
    // 1. the carried-residual guard no longer fires, because the
    //    recursive-position body now resolves to a declared callable;
    // 2. the generated-context capture gather no longer fires, because the
    //    context's `Capture` run is supplied from the producer's live
    //    environment rather than looked for in an ABI operand run that
    //    structurally cannot hold it.
    //
    // Asserted as ABSENCES of two exact refusals rather than as the presence
    // of one new string. An absence is what "`D2` cleared this stop" actually
    // means, and it stays true however the deeper boundary is later resolved.
    assert!(
        !error.contains("a carried recursive hypothesis is an eliminated value, not a callable"),
        "`D2` resolves this witness's recursive position to a declared callable, so the \
         carried-residual guard must no longer be the stop: {error}"
    );
    assert!(
        !error.contains("carries no context-capture availability claim"),
        "`D2` supplies the context's Capture run from the producer environment, so the \
         context capture gather must no longer be the stop: {error}"
    );

    // **TRANSITION SENTINEL, and named for its boundary rather than its
    // text.** `D2` supplies the callee's frame; it says nothing about the phase
    // of the call's RESULT, which crosses a function boundary and therefore
    // carries only the word. This witness now advances into host-effect seat
    // dispatch and stops where a seat needs structure a carried word does not
    // have. That is a different subsystem and a different law.
    //
    // **This assertion is designed to go red when that boundary is addressed**,
    // and the red is its purpose: it is the event that says the witness got
    // further still, and the deliverable then is to re-measure what the new
    // terminal state is — not to widen this clause. Do not satisfy it by
    // deleting it.
    assert!(
        !result.is_ok(),
        "`D2` seats the context frame; it does not green this witness. A success here \
         means the effect-seat boundary below was closed too, and this sentinel is the \
         thing to re-measure rather than remove: {error}"
    );
    // REPOINTED AGAIN by `RT-EXACTINT-CARRIED-OBSERVE`. Every repoint has moved
    // this pin STRICTLY DOWNSTREAM: ConstructorTag (dead arm, trapped) ->
    // ResourceRelease/ResourceScalar (carried observation, routed) ->
    // ExactIntU64 (Avail move + carried decode) -> the FsReadAt Arg(2) reply-path
    // gate. Direction is the whole safety argument, so it is recorded rather
    // than left to be re-derived at each move.
    //
    // Repointed, not widened and not deleted — the sentinel's own instruction.
    // The new terminal is strictly DOWNSTREAM of the old one, which is what
    // keeps the two absences above non-vacuous: the program lowered even
    // further, so the guarded path is still exercised. A repoint to something
    // UPSTREAM of the guarded feature would be the accommodation this forbids.
    assert!(
        error.contains(
            "seat Argument(2) of FsReadAt needs ResourceScalar, which this release can \
             observe only in a specialized template"
        ),
        "the expected terminal state after `RT-RESOURCE-RELEASE-CARRIED-OBSERVE` is the \
         `FsReadAt` Arg(2) buffer reply-path gate — a different READER, owned by \
         `RT-FSREADAT-REPLY-BUFFER-GATE-REMOVAL` — which is what makes the two absences above \
         non-vacuous: without it they would also hold on a compile that failed for some \
         unrelated earlier reason: {error}"
    );
}
