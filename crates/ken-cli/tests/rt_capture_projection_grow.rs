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

// ── STACK FLOOR, AND WHAT IT COSTS ──────────────────────────────────────────
//
// These tests drive the elaborator's recursive descent and had NO stack grant:
// they ran on libtest's ambient 2 MiB thread and passed on an ACCIDENTAL
// margin, which an unrelated elaborator change then consumed. That is the same
// shape as the guard deleted from `abi_s6_mapping_surface_native` — a test
// whose need sits just under the ambient default has no signal of its own
// until something else spends the remainder, and the arriving change wears the
// blame.
//
// MEASURED, both arms built and the binaries invoked directly, bisected on
// `RUST_MIN_STACK` at 64 KiB resolution:
//
//     without the change   1984 KiB     with it   2112 KiB     delta 128 KiB
//     libtest ambient      2048 KiB     -> 1984 < 2048 < 2112, so it tips
//
// The floor below is DERIVED RATHER THAN CHOSEN, and written as arithmetic so
// the spend is legible. The property the old 3.1% margin never had was that
// anybody could see what they were spending.
//
// THIS IS A STOPGAP AND ITS SUBJECT IS NOT THIS FILE. `1984 KiB` is consumed
// before any test-specific work begins and is IDENTICAL across three files in
// two crates — a shared dominating consumer. Raising floors buys time; it does
// not address a baseline that eats 97% of the ambient stack.
const MEASURED_NEED_KIB: usize = 2112;
/// The bisection could not resolve finer than this, so the true need may be
/// up to one step above what was observed.
const BISECTION_RESOLUTION_KIB: usize = 64;
/// One elaborator change of the size that tipped these cost 128 KiB here.
const OBSERVED_INCREMENT_KIB: usize = 128;
/// How many further such changes this floor is buying room for. State the
/// number rather than a round total: this is the quantity being spent.
const INCREMENTS_OF_HEADROOM: usize = 8;
const TEST_STACK_BYTES: usize = (MEASURED_NEED_KIB
    + BISECTION_RESOLUTION_KIB
    + INCREMENTS_OF_HEADROOM * OBSERVED_INCREMENT_KIB)
    * 1024;

/// Run a test body on a thread with the floor above.
///
/// The thread is NAMED: an overflow on an unnamed `Builder` reports
/// `thread '<unknown>'`, which cost this investigation a step when exactly
/// that happened elsewhere. A future overflow here names its own test.
fn with_stack_floor(name: &str, body: impl FnOnce() + Send + 'static) {
    let handle = std::thread::Builder::new()
        .name(name.to_string())
        .stack_size(TEST_STACK_BYTES)
        .spawn(body)
        .expect("spawning the floored test body must succeed");
    if let Err(payload) = handle.join() {
        std::panic::resume_unwind(payload);
    }
}

#[test]
fn the_conditional_join_grows_the_projection_and_records_every_deferral() {
    with_stack_floor(
        "the_conditional_join_grows_the_projection_and_records_every_deferral",
        the_conditional_join_grows_the_projection_and_records_every_deferral_body,
    );
}

fn the_conditional_join_grows_the_projection_and_records_every_deferral_body() {
    let root = tempfile::tempdir().expect("temporary native-build root");
    let (result, deferrals) = ken_runtime::with_worker_prefix_deferrals(|| {
        ken_cli::build_native_program(
            WITNESS_SOURCE,
            ken_cli::SourceFormat::Ken,
            "rt_capture_projection_grow",
            root.path(),
            ken_runtime::boundary_resource_profile::starter_smoke_profile(),
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

    // The conditional join must preserve the continuation. The non-empty,
    // exact deferral ledger above proves this mechanism ran; M6 now closes its
    // former terminal checked-IH refusal, so completion is the positive
    // non-vacuity anchor rather than another advancing-refusal sentinel.
    result.expect("the conditional capture-projection join and M6 representation must complete");
}
