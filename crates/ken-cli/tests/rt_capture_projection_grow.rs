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

    // ⛔ And the expected terminal state, which is what makes the assertion
    // above non-vacuous: D1 mints claims but does not seat them, so this
    // witness still refuses at the UNCHANGED carried-residual guard. If this
    // ever reports success, D2 has landed and this expectation is what should
    // be revisited — not deleted silently.
    assert!(
        error.contains("a carried recursive hypothesis is an eliminated value, not a callable"),
        "D1 alone greens no witness by design; the expected terminal state is the \
         unchanged BoundaryCarrier refusal: {error}"
    );
}
