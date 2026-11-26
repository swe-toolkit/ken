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

fn descriptor_scenario(mode: &str, body: &str, invalid_length: &str) -> Scenario {
    let entry = if body == "resize_sequence" {
        "resize_sequence cap".to_owned()
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
                .replace("INVALID_DESCRIPTOR_LENGTH", invalid_length),
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
        let operations: Vec<_> = lane.effect_trace.iter().map(|event| event.operation).collect();
        assert_eq!(operations, [HostOpV1::FsOpen, HostOpV1::ResourceRelease]);
    }
    assert_eq!(subject(&run), [BEFORE, BEFORE]);
}
