use super::*;

const SOURCE: &str = include_str!("directory_stream.ken");

fn run_directory(subject_kind: crate::SeedNodeKind, entries: &[ken_host::DirEntryV1]) -> CanonicalDifferentialRun {
    let mut profile = ken_runtime::boundary_resource_profile::starter_smoke_profile();
    profile.persistent.nodes = 8192;
    profile.persistent.words = 32768;
    profile.persistent.data_bytes = 131072;
    let mut initial_filesystem = vec![SeedNode { relative_path: b"subject".to_vec(), kind: subject_kind }];
    for entry in entries {
        let mut path = b"subject/".to_vec();
        path.extend_from_slice(&entry.name);
        initial_filesystem.push(SeedNode { relative_path: path, kind: crate::SeedNodeKind::File(Vec::new()) });
    }
    let scenario = Scenario {
        boundary_resource_profile: profile,
        process_input: RawProcessInput { arguments: Vec::new(), environment: Vec::new() },
        ambient: AmbientScript::default(),
        program_caps: ProgramCapsShape {
            fs_authority: AUTH_FULL,
            relative_root: Vec::new(),
            rights: RightSet::ALL,
            symlink: SymlinkPolicy::NoFollow,
        },
        entry: CheckedProgramEntry {
            catalog_roots: fixture_catalog_roots(),
            identity: "abi-s2-directory-stream".into(),
            package_name: "abi-s2-directory-stream".into(),
            source: SOURCE.into(),
        },
        initial_filesystem,
        expected_fs: Vec::new(),
    };
    // Each lane independently opens the same unmodified directory, with no
    // assumptions about whether distinct inodes enumerate in the same order.
    execute_scenario_with_roots(&scenario, Some(SharedScenarioRoot::DirectoryFixture))
        .expect("checked directory bracket runs on both lanes")
}

// Durable invariant: assert the oracle population separately, then compare
// exact batches and settlement; neither lane mints the oracle set.
#[test]
fn abi_s2_identical_nonempty_batches_and_end() {
    let entries: Vec<_> = (0..400).map(|n| ken_host::DirEntryV1 {
        name: format!("{n:0200}").into_bytes(), kind: ken_host::FsNodeKindV1::File,
    }).collect();
    let mut run = run_directory(crate::SeedNodeKind::Directory, &entries);
    for lane in [&run.interpreter, &run.native] {
        assert_eq!(lane.exit_status, 0, "terminal: {:?}", lane.terminal_error);
        assert!(lane.effect_trace.iter().filter(|event| matches!(event.outcome,
            CanonicalOutcomeV1::Success(CanonicalReplyV1::DirectoryEntries(_)))).count() >= 2);
    }
    run.compare_directory_stream(b"subject", &entries).unwrap();
    for op in [HostOpV1::FsOpenDirectory, HostOpV1::FsReadDirectoryBatch] {
        assert_eq!(confirm_native_tested_transition(op, NativeTestedEvidence::from_directory_stream_run(op, &run, b"subject", &entries)), Ok(HostOpAvailabilityV1::NativeTested));
    }
    let event = run.native.effect_trace.iter_mut().find(|e| matches!(e.outcome,
        CanonicalOutcomeV1::Success(CanonicalReplyV1::DirectoryEntries(_)))).unwrap();
    let CanonicalOutcomeV1::Success(CanonicalReplyV1::DirectoryEntries(batch)) = &mut event.outcome else { unreachable!() };
    batch.pop().unwrap();
    assert!(matches!(run.compare_directory_stream(b"subject", &entries),
        Err(FsDirectoryDifferentialError::Listing { lane: "native", .. })));
    assert!(confirm_native_tested_transition(HostOpV1::FsReadDirectoryBatch,
        NativeTestedEvidence::from_directory_stream_run(HostOpV1::FsReadDirectoryBatch, &run, b"subject", &entries)).is_err());
}

// Durable invariant: neither dot entry may appear, and EOF is a case, not
// a successful zero-length data batch.
#[test]
fn abi_s2_empty_directory_starts_at_end() {
    let run = run_directory(crate::SeedNodeKind::Directory, &[]);
    run.compare_directory_stream(b"subject", &[]).unwrap();
    for lane in [&run.interpreter, &run.native] {
        assert!(matches!(lane.effect_trace[1].outcome,
            CanonicalOutcomeV1::Success(CanonicalReplyV1::DirectoryEnd)));
    }
}

// Durable invariant: the acquire error is keyed on the operation and no
// resource, batch, or release reaches the host after a failed acquire.
#[test]
fn abi_s2_regular_file_refuses_directory_acquire() {
    let run = run_directory(crate::SeedNodeKind::File(Vec::new()), &[]);
    run.compare_exact().unwrap();
    for lane in [&run.interpreter, &run.native] {
        assert_eq!(lane.exit_status, 73);
        let [event] = lane.effect_trace.as_slice() else { panic!("expected only acquisition") };
        assert_eq!(event.operation, HostOpV1::FsOpenDirectory);
        assert_eq!(event.outcome, CanonicalOutcomeV1::Error(SemanticErrorV1::File(
            ken_host::FileErrorIdentityV1 {
                operation: HostOpV1::FsOpenDirectory,
                relative_path: b"subject".to_vec(),
                cause: FileErrorCauseV1::Io(IoErrorIdentityV1::NotDirectory),
            }
        )));
    }
}
