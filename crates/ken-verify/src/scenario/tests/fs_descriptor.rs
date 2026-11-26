use super::*;
use ken_host::{FsSyncModeV1, ResourceBindingRole, ResourceErrorV1};

const SOURCE: &str = include_str!("fs_descriptor.ken");
const BEFORE: &[u8] = b"0123456789";
const AFTER: &[u8] = b"0123\0\0\0\0\0\0\0\0";
const READ: &str = "ResourceOpenMode.ResourceRead";
const WRITE: &str = "(ResourceOpenMode.ResourceWriteCreate CreatePolicy.CreateOrKeep)";

fn run_descriptor(mode: &str, body: &str) -> CanonicalDifferentialRun {
    let scenario = descriptor_scenario(mode, body, "(sub_int 0 1)");
    execute_scenario(&scenario).expect("descriptor fixture executes in both real roots")
}

fn descriptor_scenario(mode: &str, body: &str, position: &str) -> Scenario {
    // These bodies sequence their own file brackets.
    let entry = if matches!(body, "resize_sequence" | "max_length_then_shrink") {
        format!("{body} cap")
    } else {
        format!("host_bind AFull (Result FileError (ResourceBracketResult Unit Unit)) ExitCode \
                  (withResource AFull Unit Unit cap (bytes_encode \"subject\") {mode} {body}) final_exit")
    };
    let mut profile = ken_runtime::boundary_resource_profile::starter_smoke_profile();
    // The resize sequence crosses four file brackets and two buffer brackets.
    // It exceeded the 64-node invocation starter profile during its third
    // file bracket; this fixture tests descriptor I/O, not arena exhaustion.
    profile.invocation.nodes = 512;
    profile.invocation.words = 2048;
    profile.invocation.data_bytes = 4096;
    profile.persistent.nodes = 512;
    profile.persistent.words = 2048;
    profile.persistent.data_bytes = 4096;
    Scenario {
        boundary_resource_profile: profile,
        process_input: RawProcessInput::default(),
        ambient: AmbientScript::default(),
        program_caps: ProgramCapsShape::default(),
        entry: CheckedProgramEntry {
            catalog_roots: fixture_catalog_roots(),
            identity: format!("fs-descriptor-{body}"),
            package_name: "fs-descriptor".into(),
            source: SOURCE
                .replace("DESCRIPTOR_ENTRY", &entry)
                .replace("DESCRIPTOR_POSITION", position),
        },
        initial_filesystem: vec![
            SeedNode {
                relative_path: b"subject".to_vec(),
                kind: crate::SeedNodeKind::File(BEFORE.to_vec()),
            },
            SeedNode {
                relative_path: b"source".to_vec(),
                kind: crate::SeedNodeKind::File(b"Z".to_vec()),
            },
        ],
        expected_fs: Vec::new(),
    }
}

fn subject(run: &CanonicalDifferentialRun) -> [Vec<u8>; 2] {
    [run.roots.interpreter(), run.roots.native()]
        .map(|root| std::fs::read(root.join("subject")).unwrap())
}

fn assert_exit_success(run: &CanonicalDifferentialRun) {
    for lane in [&run.interpreter, &run.native] {
        assert_eq!(
            lane.exit_status, 0,
            "terminal: {:?}\ntrace: {:#?}",
            lane.terminal_error, lane.effect_trace
        );
    }
}

// Durable invariant. MEASURED: exact held-resource resize requests and Unit
// replies, a 10-byte read window yielding four bytes, zero-filled extension,
// and both roots' bytes. CLAIMED: truncation/extension parity. THE GAP: two
// sizes on a regular file, not all filesystems or all lengths.
#[test]
fn set_length_confirms_only_exact_trace_and_real_content() {
    use crate::StatusTransitionError::ExternalObservationMismatch;
    let mut run = run_descriptor(WRITE, "resize_sequence");
    assert_exit_success(&run);
    assert_eq!(subject(&run), [AFTER, AFTER]);
    let operation = HostOpV1::FsSetLength;
    let confirm = |run: &CanonicalDifferentialRun| {
        confirm_native_tested_transition(
            operation,
            NativeTestedEvidence::from_fs_set_length_run(run, "subject", BEFORE, [4, 12]),
        )
    };
    assert_eq!(confirm(&run), Ok(HostOpAvailabilityV1::NativeTested));

    let index = run
        .native
        .effect_trace
        .iter()
        .position(|event| event.operation == operation)
        .unwrap();
    let original = std::mem::replace(
        &mut run.native.effect_trace[index].request,
        CanonicalRequestV1::FsSetLength { length: 5 },
    );
    assert_eq!(confirm(&run), Err(ExternalObservationMismatch(operation)));
    run.native.effect_trace[index].request = original;
    assert_eq!(confirm(&run), Ok(HostOpAvailabilityV1::NativeTested));

    let path = run.roots.native().join("subject");
    std::fs::write(&path, BEFORE).unwrap();
    // The cached traces and observations still match. Only the real-root
    // oracle distinguishes this control from the unmodified run.
    run.compare_exact().unwrap();
    assert_eq!(confirm(&run), Err(ExternalObservationMismatch(operation)));
    std::fs::write(path, AFTER).unwrap();
    assert_eq!(confirm(&run), Ok(HostOpAvailabilityV1::NativeTested));
}

// Durable invariant. MEASURED: a write and full-sync Unit reply on one held
// file; distinguish mode drift and absence. CLAIMED: exact request and
// observation parity. THE GAP: equal traces do not prove fsync or durability;
// the separate pipe EINVAL witness excludes a backend no-op.
#[test]
fn sync_after_write_confirms_full_mode_and_requires_observation() {
    use crate::StatusTransitionError::{ExternalObservationMismatch, OperationNotObserved};
    let mut run = run_descriptor(WRITE, "(sync_after_write cap)");
    assert_exit_success(&run);
    let operation = HostOpV1::FsSync;
    for lane in [&run.interpreter, &run.native] {
        let writes: Vec<_> = lane
            .effect_trace
            .iter()
            .filter(|e| e.operation == HostOpV1::FsWriteAt)
            .collect();
        let syncs: Vec<_> = lane
            .effect_trace
            .iter()
            .filter(|e| e.operation == operation)
            .collect();
        let ([write], [sync]) = (writes.as_slice(), syncs.as_slice()) else {
            panic!("one write and one sync")
        };
        let CanonicalOutcomeV1::Success(CanonicalReplyV1::WriteProgress(
            ken_host::WriteProgressV1::Wrote(n),
        )) = write.outcome
        else {
            panic!("expected successful write")
        };
        assert_eq!(n.get(), 1);
        let file = write
            .resource_bindings
            .iter()
            .find(|(role, _)| *role == ResourceBindingRole::File)
            .unwrap()
            .1;
        assert!(write.sequence < sync.sequence);
        assert_eq!(
            sync.request,
            CanonicalRequestV1::FsSync {
                mode: FsSyncModeV1::SyncFull
            }
        );
        assert_eq!(
            sync.outcome,
            CanonicalOutcomeV1::Success(CanonicalReplyV1::Unit)
        );
        assert_eq!(
            sync.resource_bindings,
            [(ResourceBindingRole::Target, file)]
        );
        assert_eq!(sync.capability, None);
    }
    let confirm = |run: &CanonicalDifferentialRun| {
        confirm_native_tested_transition(operation, NativeTestedEvidence::from_run(operation, run))
    };
    assert_eq!(confirm(&run), Ok(HostOpAvailabilityV1::NativeTested));
    let sync = run
        .native
        .effect_trace
        .iter_mut()
        .find(|e| e.operation == operation)
        .unwrap();
    let original = std::mem::replace(
        &mut sync.request,
        CanonicalRequestV1::FsSync {
            mode: FsSyncModeV1::SyncData,
        },
    );
    assert_eq!(confirm(&run), Err(ExternalObservationMismatch(operation)));
    run.native
        .effect_trace
        .iter_mut()
        .find(|e| e.operation == operation)
        .unwrap()
        .request = original;
    assert_eq!(confirm(&run), Ok(HostOpAvailabilityV1::NativeTested));

    // Delete symmetrically to isolate missing observation from trace inequality,
    // which is a different earlier gate.
    let mut removed = Vec::new();
    for lane in [&mut run.interpreter, &mut run.native] {
        let index = lane
            .effect_trace
            .iter()
            .position(|e| e.operation == operation)
            .unwrap();
        removed.push((index, lane.effect_trace.remove(index)));
        for event in &mut lane.effect_trace[index..] {
            event.sequence -= 1;
        }
    }
    run.compare_exact().unwrap();
    assert_eq!(confirm(&run), Err(OperationNotObserved(operation)));
    for (lane, (index, event)) in [&mut run.interpreter, &mut run.native]
        .into_iter()
        .zip(removed)
    {
        for successor in &mut lane.effect_trace[index..] {
            successor.sequence += 1;
        }
        lane.effect_trace.insert(index, event);
    }
    assert_eq!(confirm(&run), Ok(HostOpAvailabilityV1::NativeTested));
}

// Durable invariant. MEASURED: checked Result selects RightNotHeld, the
// recorded denial has no admitted Target, and the real roots are unchanged.
// CLAIMED: admission refuses before backend I/O; THE GAP: sync has no state
// witness, so this row checks exact denial, not physical persistence.
#[test]
fn readonly_descriptor_operations_record_exact_denials() {
    for (operation, request, body) in [
        (
            HostOpV1::FsSetLength,
            CanonicalRequestV1::FsSetLength { length: 4 },
            "denied_length",
        ),
        (
            HostOpV1::FsSync,
            CanonicalRequestV1::FsSync {
                mode: FsSyncModeV1::SyncFull,
            },
            "denied_sync",
        ),
    ] {
        let run = run_descriptor(READ, body);
        assert_exit_success(&run);
        run.compare_exact().unwrap();
        for lane in [&run.interpreter, &run.native] {
            let events: Vec<_> = lane
                .effect_trace
                .iter()
                .filter(|e| e.operation == operation)
                .collect();
            let [event] = events.as_slice() else {
                panic!("exactly one {operation:?} denial")
            };
            assert_eq!(event.request, request);
            assert_eq!(
                event.outcome,
                CanonicalOutcomeV1::Error(SemanticErrorV1::Resource(
                    ResourceErrorV1::RightNotHeld {
                        required: 2,
                        held: 1
                    }
                ))
            );
            assert!(event.resource_bindings.is_empty());
            assert_eq!(event.capability, None);
        }
        if operation == HostOpV1::FsSetLength {
            assert_eq!(subject(&run), [BEFORE, BEFORE]);
        }
    }
}

// Durable invariant. MEASURED: a live WRITE handle's negative length yields
// the checked InvalidOffset branch and emits no descriptor event. CLAIMED:
// signed admission precedes dispatch. THE GAP: this row uses exactly −1.
#[test]
fn negative_length_refuses_before_dispatch() {
    let run = run_descriptor(WRITE, "invalid_length");
    assert_exit_success(&run);
    run.compare_exact().unwrap();
    for lane in [&run.interpreter, &run.native] {
        assert!(!lane
            .effect_trace
            .iter()
            .any(|event| event.operation == HostOpV1::FsSetLength));
    }
    assert_eq!(subject(&run), [BEFORE, BEFORE]);
}

// Durable invariant. MEASURED: both lanes select InvalidOffset for UInt64::MAX
// + 1, with identical envelopes, no FsSetLength dispatch and unchanged bytes.
// CLAIMED: native/interpreter parity on refusal before dispatch. THE GAP: the
// interpreter observation does not distinguish the checked guard from its
// defense-in-depth check.
#[test]
fn positive_u64_overflow_refuses_before_dispatch() {
    let scenario = descriptor_scenario(WRITE, "invalid_length", "18446744073709551616");
    let run = execute_scenario(&scenario).expect("descriptor overflow executes in both real roots");
    // Only Err InvalidOffset followed by successful release yields exit 0.
    assert_exit_success(&run);
    run.compare_exact().unwrap();
    for lane in [&run.interpreter, &run.native] {
        let operations: Vec<_> = lane
            .effect_trace
            .iter()
            .map(|event| event.operation)
            .collect();
        assert_eq!(operations, [HostOpV1::FsOpen, HostOpV1::ResourceRelease]);
    }
    assert_eq!(subject(&run), [BEFORE, BEFORE]);
}

// Keep the oracle independent of the production bound.
const HOST_MAX: u64 = 9_223_372_036_854_775_807;
const OVER_HOST_MAX: u64 = 9_223_372_036_854_775_808;
const INVALID_OFFSET: CanonicalOutcomeV1 =
    CanonicalOutcomeV1::Error(SemanticErrorV1::Resource(ResourceErrorV1::InvalidOffset));

fn run_position(mode: &str, body: &str, position: u64) -> CanonicalDifferentialRun {
    execute_scenario(&descriptor_scenario(mode, body, &position.to_string()))
        .expect("position fixture executes in both real roots")
}

fn width_events(
    run: &CanonicalDifferentialRun,
    operation: HostOpV1,
) -> [Vec<(&CanonicalRequestV1, &CanonicalOutcomeV1)>; 2] {
    // Keep both lanes in the assertion, so a symmetric failure is visible
    // rather than stopping at the first bad lane or relying on parity alone.
    [&run.interpreter, &run.native].map(|lane| {
        lane.effect_trace
            .iter()
            .filter(|event| event.operation == operation)
            .map(|event| (&event.request, &event.outcome))
            .collect()
    })
}

fn assert_position_result(run: &CanonicalDifferentialRun, final_bytes: &[u8]) {
    assert_exit_success(run);
    run.compare_exact().unwrap();
    assert_eq!(subject(run), [final_bytes, final_bytes]);
}

fn assert_length_refused_before_dispatch(length: u64) {
    let run = run_position(WRITE, "invalid_length", length);
    assert_eq!(
        width_events(&run, HostOpV1::FsSetLength),
        [vec![], vec![]],
        "setLength admission events (interpreter, native)"
    );
    // Only Err InvalidOffset yields exit 0 in this body.
    assert_position_result(&run, BEFORE);
}

// Durable invariant (S1/S2). MEASURED: checked InvalidOffset with no
// setLength event and unchanged real roots for two u64 values above the
// host bound. CLAIMED: width refusal at lane admission. THE GAP: these
// sample the endpoints of the u64-only range, not every interior value.
#[test]
fn set_length_one_past_host_width_refuses_before_dispatch() {
    assert_length_refused_before_dispatch(OVER_HOST_MAX);
}

#[test]
fn set_length_u64_max_refuses_before_dispatch() {
    assert_length_refused_before_dispatch(u64::MAX);
}

// Durable invariant (S3). MEASURED: max host length dispatch, then an
// unconditional successful shrink, before either root is read. CLAIMED:
// the admission maximum is inclusive. THE GAP: the first host outcome is
// filesystem-dependent; only a width refusal is excluded.
#[test]
fn set_length_host_max_dispatches_before_small_shrink() {
    let run = run_position(WRITE, "max_length_then_shrink", HOST_MAX);
    let events = width_events(&run, HostOpV1::FsSetLength);
    for (lane, events) in ["interpreter", "native"].into_iter().zip(events) {
        let [(first_request, first_outcome), (second_request, second_outcome)] = events.as_slice()
        else {
            panic!("{lane}: expected two resize events, got {events:?}");
        };
        assert_eq!(
            **first_request,
            CanonicalRequestV1::FsSetLength { length: HOST_MAX }
        );
        assert_ne!(**first_outcome, INVALID_OFFSET);
        assert_eq!(
            **second_request,
            CanonicalRequestV1::FsSetLength { length: 4 }
        );
        assert_eq!(
            **second_outcome,
            CanonicalOutcomeV1::Success(CanonicalReplyV1::Unit)
        );
    }
    assert_position_result(&run, b"0123");
}

fn assert_read_position(body: &str, offset: u64, outcome: Option<CanonicalOutcomeV1>) {
    // `body` accepts exactly one returned class (see the fixture), so the
    // exit-0 check below pins the value as well as the trace.
    let run = run_position(READ, body, offset);
    let request = CanonicalRequestV1::FsReadAt {
        file_offset: offset,
        buffer_start: 0,
        length: 1,
    };
    let expected: Vec<_> = outcome
        .as_ref()
        .map(|outcome| (&request, outcome))
        .into_iter()
        .collect();
    assert_eq!(
        width_events(&run, HostOpV1::FsReadAt),
        [expected.clone(), expected],
        "readAt events (interpreter, native)"
    );
    assert_position_result(&run, BEFORE);
}

// Durable invariant (R1–R3). MEASURED: a returned InvalidOffset without an
// event, a dispatched InvalidOffset, and a returned EOF event at the inclusive
// end bound; each body exits 0 only on its one expected class. The offset and
// the accepting body vary. CLAIMED: width is checked at the seat and the effective end
// before host I/O. THE GAP: one-byte windows; no claim about larger reads.
#[test]
fn read_at_one_past_host_width_refuses_before_dispatch() {
    assert_read_position("width_read_refused", OVER_HOST_MAX, None);
}

#[test]
fn read_at_end_past_host_width_refuses_at_dispatch() {
    assert_read_position("width_read_refused", HOST_MAX, Some(INVALID_OFFSET));
}

#[test]
fn read_at_end_at_host_max_dispatches_to_eof() {
    assert_read_position(
        "width_read_eof",
        HOST_MAX - 1,
        Some(CanonicalOutcomeV1::Success(CanonicalReplyV1::ReadProgress(
            ken_host::ReadProgressV1::ReadEof,
        ))),
    );
}

fn assert_write_position(offset: u64, outcome: Option<CanonicalOutcomeV1>) {
    let run = run_position(WRITE, "(width_write cap)", offset);
    let request = CanonicalRequestV1::FsWriteAt {
        file_offset: offset,
        buffer_start: 0,
        length: 1,
    };
    let expected: Vec<_> = outcome
        .as_ref()
        .map(|outcome| (&request, outcome))
        .into_iter()
        .collect();
    assert_eq!(
        width_events(&run, HostOpV1::FsWriteAt),
        [expected.clone(), expected],
        "writeAt events (interpreter, native)"
    );
    // Both lanes made a one-byte span from a separate, live source; a
    // missing write is not evidence if the preceding read never succeeded.
    for reads in width_events(&run, HostOpV1::FsReadAt) {
        assert!(
            matches!(
                reads.as_slice(),
                [(
                    CanonicalRequestV1::FsReadAt { file_offset: 0, buffer_start: 0, length: 1 },
                    CanonicalOutcomeV1::Success(CanonicalReplyV1::ReadProgress(
                        ken_host::ReadProgressV1::ReadSome { transferred, .. }
                    ))
                )] if transferred.get() == 1
            ),
            "span producer: {reads:?}"
        );
    }
    assert_position_result(&run, BEFORE);
}

// Durable invariant (W1/W2). MEASURED: a valid one-byte span, refusal without
// an event at the next offset, and one InvalidOffset event at the maximum.
// CLAIMED: the seat and effective-end checks are distinct. THE GAP: no
// high successful write, which could leave an enormous sparse file.
#[test]
fn write_at_one_past_host_width_refuses_before_dispatch() {
    assert_write_position(OVER_HOST_MAX, None);
}

#[test]
fn write_at_end_past_host_width_refuses_at_dispatch() {
    assert_write_position(HOST_MAX, Some(INVALID_OFFSET));
}
