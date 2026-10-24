//! RT-BRACKET-CONTROL-REGION-IR D0 equivalence refuter.
//!
//! This file pre-registers the exact comparison tuple before the first product
//! measurement. The tuple is intentionally stricter than the older release-set
//! comparisons: the host-effect trace remains ordered and resource identities
//! are normalized only by successful acquisition ordinal.

#![cfg(target_os = "linux")]
#![allow(dead_code)]

use std::collections::BTreeMap;

use ken_host::{
    CapabilityTraceIdentity, FsOpenModeV1, MappingProtectionV1, ResourceKindV1,
    ResourceSettlementOutcomeV1, ResourceTraceIdentityV1,
};
use ken_runtime::{
    CanonicalOutcomeV1, CanonicalReplyV1, CanonicalRequestV1, EffectEvent,
    EffectObservation, HostOpV1, IoErrorIdentityV1, ResourceBindingRole, ResourceErrorV1,
    RuntimeGroundValue, SemanticErrorV1, TerminalErrorV1, TerminalExitClass,
};

/// One lane-independent resource identity, numbered by the order in which its
/// successful acquisition appears in the ordered host-effect trace.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct AcquisitionOrdinal(u64);

#[derive(Clone, Debug, Eq, PartialEq)]
enum ExitStatusOrGroundResult {
    ExitStatus(i32),
    GroundResult(RuntimeGroundValue),
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct NormalizedAcquisition {
    ordinal: AcquisitionOrdinal,
    resource_kind: ResourceKindV1,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum HostOutcomeClass {
    Success,
    Error,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct HostResultOutcomeClass {
    terminal: TerminalExitClass,
    ordered_host_outcomes: Vec<HostOutcomeClass>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum NormalizedReply {
    Other(CanonicalReplyV1),
    ResourceAcquired {
        schema_version: u16,
        resource_kind: ResourceKindV1,
        identity: AcquisitionOrdinal,
    },
    ResourceSettlement {
        schema_version: u16,
        resource_kind: ResourceKindV1,
        identity: AcquisitionOrdinal,
        outcome: ResourceSettlementOutcomeV1,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum NormalizedError {
    Other(SemanticErrorV1),
    ReleaseFailed {
        schema_version: u16,
        resource_kind: ResourceKindV1,
        identity: AcquisitionOrdinal,
        io: IoErrorIdentityV1,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum NormalizedOutcome {
    Success(NormalizedReply),
    Error(NormalizedError),
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct NormalizedEffectEvent {
    sequence: u64,
    operation: HostOpV1,
    capability: Option<CapabilityTraceIdentity>,
    resource_bindings: Vec<(ResourceBindingRole, AcquisitionOrdinal)>,
    request: CanonicalRequestV1,
    outcome: NormalizedOutcome,
}

/// The exact D0 tuple registered before any control-region product run.
///
/// MEASURED: exact normalized interpreter/native observation tuple.
/// CLAIMED: explicit bracket control preserves result propagation and effect
/// order.
/// THE GAP: closed only for the exercised path plus region/edge closure, never
/// by call coverage.
#[derive(Clone, Debug, Eq, PartialEq)]
struct NormalizedObservationTuple {
    exit_status_or_ground_result: ExitStatusOrGroundResult,
    terminal_error: Option<TerminalErrorV1>,
    ordered_host_effect_trace: Vec<NormalizedEffectEvent>,
    acquire_relative_resource_identities: Vec<NormalizedAcquisition>,
    host_result_outcome_class: HostResultOutcomeClass,
}

fn acquisition_map(
    observation: &EffectObservation,
) -> Result<
    (
        BTreeMap<ResourceTraceIdentityV1, AcquisitionOrdinal>,
        Vec<NormalizedAcquisition>,
    ),
    String,
> {
    let mut identities = BTreeMap::new();
    let mut acquisitions = Vec::new();
    for event in &observation.effect_trace {
        let CanonicalOutcomeV1::Success(CanonicalReplyV1::ResourceAcquired {
            resource_kind,
            identity,
            ..
        }) = &event.outcome
        else {
            continue;
        };
        let ordinal = AcquisitionOrdinal(
            u64::try_from(acquisitions.len())
                .map_err(|_| "D0 acquisition ordinal exhausted".to_string())?
                .checked_add(1)
                .ok_or_else(|| "D0 acquisition ordinal exhausted".to_string())?,
        );
        if identities.insert(identity.clone(), ordinal).is_some() {
            return Err(format!(
                "one resource identity was acquired more than once: {identity:?}"
            ));
        }
        acquisitions.push(NormalizedAcquisition {
            ordinal,
            resource_kind: *resource_kind,
        });
    }
    Ok((identities, acquisitions))
}

fn ordinal_for(
    identities: &BTreeMap<ResourceTraceIdentityV1, AcquisitionOrdinal>,
    identity: &ResourceTraceIdentityV1,
) -> Result<AcquisitionOrdinal, String> {
    identities
        .get(identity)
        .copied()
        .ok_or_else(|| format!("resource identity has no successful acquisition: {identity:?}"))
}

fn normalize_outcome(
    outcome: &CanonicalOutcomeV1,
    identities: &BTreeMap<ResourceTraceIdentityV1, AcquisitionOrdinal>,
) -> Result<NormalizedOutcome, String> {
    Ok(match outcome {
        CanonicalOutcomeV1::Success(reply) => {
            NormalizedOutcome::Success(match reply {
                CanonicalReplyV1::ResourceAcquired {
                    schema_version,
                    resource_kind,
                    identity,
                } => NormalizedReply::ResourceAcquired {
                    schema_version: *schema_version,
                    resource_kind: *resource_kind,
                    identity: ordinal_for(identities, identity)?,
                },
                CanonicalReplyV1::ResourceSettlement(settlement) => {
                    NormalizedReply::ResourceSettlement {
                        schema_version: settlement.schema_version,
                        resource_kind: settlement.resource_kind,
                        identity: ordinal_for(identities, &settlement.identity)?,
                        outcome: settlement.outcome.clone(),
                    }
                }
                CanonicalReplyV1::Unit
                | CanonicalReplyV1::Bool(_)
                | CanonicalReplyV1::Bytes(_)
                | CanonicalReplyV1::ReadChunk(_)
                | CanonicalReplyV1::ReadEof
                | CanonicalReplyV1::Instant(_)
                | CanonicalReplyV1::FileMetadata(_)
                | CanonicalReplyV1::DirectoryEntries(_)
                | CanonicalReplyV1::ReadProgress(_)
                | CanonicalReplyV1::WriteProgress(_)
                | CanonicalReplyV1::MonotonicInstant(_)
                | CanonicalReplyV1::FilePosition(_)
                | CanonicalReplyV1::FdInheritancePolicy(_) => {
                    NormalizedReply::Other(reply.clone())
                }
            })
        }
        CanonicalOutcomeV1::Error(error) => NormalizedOutcome::Error(match error {
            SemanticErrorV1::Resource(ResourceErrorV1::ReleaseFailed {
                schema_version,
                resource_kind,
                identity,
                io,
            }) => NormalizedError::ReleaseFailed {
                schema_version: *schema_version,
                resource_kind: *resource_kind,
                identity: ordinal_for(identities, identity)?,
                io: *io,
            },
            SemanticErrorV1::Io(_)
            | SemanticErrorV1::File(_)
            | SemanticErrorV1::Capability(_)
            | SemanticErrorV1::Resource(_) => NormalizedError::Other(error.clone()),
        }),
    })
}

fn normalize_observation(
    observation: &EffectObservation,
) -> Result<NormalizedObservationTuple, String> {
    let (identities, acquisitions) = acquisition_map(observation)?;
    let ordered_host_effect_trace = observation
        .effect_trace
        .iter()
        .map(|event| {
            Ok(NormalizedEffectEvent {
                sequence: event.sequence,
                operation: event.operation,
                capability: event.capability.clone(),
                resource_bindings: event
                    .resource_bindings
                    .iter()
                    .map(|(role, identity)| Ok((*role, ordinal_for(&identities, identity)?)))
                    .collect::<Result<Vec<_>, String>>()?,
                request: event.request.clone(),
                outcome: normalize_outcome(&event.outcome, &identities)?,
            })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let ordered_host_outcomes = observation
        .effect_trace
        .iter()
        .map(|event| match event.outcome {
            CanonicalOutcomeV1::Success(_) => HostOutcomeClass::Success,
            CanonicalOutcomeV1::Error(_) => HostOutcomeClass::Error,
        })
        .collect();
    Ok(NormalizedObservationTuple {
        exit_status_or_ground_result: ExitStatusOrGroundResult::ExitStatus(
            observation.exit_status,
        ),
        terminal_error: observation.terminal_error.clone(),
        ordered_host_effect_trace,
        acquire_relative_resource_identities: acquisitions,
        host_result_outcome_class: HostResultOutcomeClass {
            terminal: observation.terminal_exit,
            ordered_host_outcomes,
        },
    })
}

#[test]
fn preregistered_tuple_normalizes_only_by_acquisition_ordinal() {
    fn observation(first: u64, second: u64) -> EffectObservation {
        let acquired = |sequence, kind, identity| EffectEvent {
            sequence,
            operation: match kind {
                ResourceKindV1::FsHandle => HostOpV1::FsOpen,
                ResourceKindV1::Buffer => HostOpV1::BufferAllocate,
                ResourceKindV1::Mapping => HostOpV1::MappingAllocate,
            },
            capability: None,
            resource_bindings: vec![(ResourceBindingRole::Target, identity)],
            request: match kind {
                ResourceKindV1::FsHandle => CanonicalRequestV1::FsOpen {
                    path: b"held".to_vec(),
                    mode: FsOpenModeV1::Metadata,
                },
                ResourceKindV1::Buffer => CanonicalRequestV1::BufferAllocate { capacity: 1 },
                ResourceKindV1::Mapping => CanonicalRequestV1::MappingAllocate {
                    length: 1,
                    protection: MappingProtectionV1::ReadOnly,
                },
            },
            outcome: CanonicalOutcomeV1::Success(CanonicalReplyV1::ResourceAcquired {
                schema_version: 1,
                resource_kind: kind,
                identity,
            }),
        };
        EffectObservation {
            stdout: Vec::new(),
            stderr: Vec::new(),
            filesystem_delta: Vec::new(),
            terminal_error: None,
            effect_trace: vec![
                acquired(
                    0,
                    ResourceKindV1::FsHandle,
                    ResourceTraceIdentityV1(first),
                ),
                acquired(
                    1,
                    ResourceKindV1::Buffer,
                    ResourceTraceIdentityV1(second),
                ),
            ],
            terminal_exit: TerminalExitClass::NormalReturn,
            exit_status: 0,
        }
    }

    let left = normalize_observation(&observation(91, 7)).unwrap();
    let right = normalize_observation(&observation(4, 83)).unwrap();
    assert_eq!(left, right);
    assert_eq!(
        left.acquire_relative_resource_identities,
        vec![
            NormalizedAcquisition {
                ordinal: AcquisitionOrdinal(1),
                resource_kind: ResourceKindV1::FsHandle,
            },
            NormalizedAcquisition {
                ordinal: AcquisitionOrdinal(2),
                resource_kind: ResourceKindV1::Buffer,
            },
        ]
    );
}

// D0 equivalence and mutation gates: RT-BRACKET-LOWERING-AND-D0-REFUTER §2,
// spec/40-runtime/42-evaluation.md §6, spec/40-runtime/45-native-backend.md §4.
// The c22f preregistered tuple above remains unchanged.
#[path = "support/rt_bracket_nested_source.rs"]
mod bracket_source;

const D0_BRANCHED_SOURCE: &str = include_str!("fixtures/d0_branched_scrutinee_source.ken");
const D0_MAPPING_SOURCE: &str = include_str!("fixtures/d0_abi_mapping_source.ken");
const D0_PARITY_SOURCE: &str = include_str!("fixtures/d0_rt_parity_source.ken");

// The complete D0 suite passed with a test-only 4,096-KiB Builder override
// and no ambient RUST_MIN_STACK; branched and depth-three rows also passed at
// 2,048 KiB. The retained 256-MiB px8ta baseline provides 258,048 KiB of
// headroom over the full-suite passing bound, not a depth or stack-limit pin.
// No previously green test was changed to mask a new stack regression.
const D0_WORKER_STACK_BYTES: usize = 256 * 1024 * 1024;

/// Promise class: durable invariant.
/// MEASURED: exact depth-two tuple, exit 81 and both releases after leaf error.
/// CLAIMED: native merge does not drop the body's Err edge.
/// THE GAP: a real backend ReleaseFailed has no shared deterministic fixture;
/// scratch body- and release-input mutations separately pin merge sensitivity.
#[test]
fn d0_body_error_outcome_exact_tuple() {
    std::thread::Builder::new()
        .name("d0-body-error".to_owned())
        .stack_size(D0_WORKER_STACK_BYTES)
        .spawn(|| {
            let root = tempfile::tempdir().expect("D0 body-error root");
            for index in 0..2 {
                std::fs::write(
                    root.path().join(format!("held-{index}.bin")),
                    format!("held resource {index}"),
                )
                .expect("D0 body-error acquired resource");
            }
            let source = bracket_source::nested_bracket_source(2);
            let leaf_ok = "    (ResourceBodyOk Unit Unit MkUnit)\n\nfn body_result";
            let leaf_count = source.matches(leaf_ok).count();
            assert_eq!(
                leaf_count, 1,
                "D0 body-error input has exactly one replaceable leaf-success arm"
            );
            let source = source.replacen(
                leaf_ok,
                "    (ResourceBodyErr Unit Unit MkUnit)\n\nfn body_result",
                1,
            );
            let output = ken_cli::build_native_program(
                &source,
                ken_cli::SourceFormat::Ken,
                "rt-bracket-d0-body-error",
                root.path(),
                ken_runtime::boundary_resource_profile::starter_smoke_profile(),
            )
            .expect("D0 body-error linked build");
            let native = ken_runtime::run_bound_process_effect_observation(
                &output.artifact,
                &ken_runtime::NativeEffectRunOptionsV1 {
                    arguments: Vec::new(),
                    environment: Vec::new(),
                    cwd: root.path().to_owned(),
                    plan_hash: output.plan_transport_hash,
                },
            )
            .expect("D0 body-error native observation");
            let mut host = ken_interp::PosixHost::new_at(root.path());
            let interpreted = ken_cli::run_program_effect_observation(
                &source,
                ken_cli::SourceFormat::Ken,
                &[],
                &[],
                root.path().as_os_str().as_encoded_bytes(),
                &mut host,
            )
            .expect("D0 body-error interpreter observation");
            let reference =
                normalize_observation(&interpreted).expect("D0 body-error interpreter tuple");
            let compiled = normalize_observation(&native).expect("D0 body-error native tuple");
            assert_eq!(reference, compiled, "D0 body-error exact tuple");
            assert_eq!(
                native.exit_status, 81,
                "ResourceBracketBodyError must choose after_root's body-error arm"
            );
            assert_eq!(
                native
                    .effect_trace
                    .iter()
                    .filter(|event| event.operation == HostOpV1::ResourceRelease)
                    .count(),
                2,
                "body error still releases both acquired resources"
            );
        })
        .expect("D0 body-error scratch thread")
        .join()
        .expect("D0 body-error scratch thread result");
}

/// Promise class: durable invariant.
/// MEASURED: one failed FsOpen, no release, exact tuple and exit 84.
/// CLAIMED: acquisition failure bypasses settlement without false release.
/// THE GAP: no successful acquisition or nested body is exercised here.
#[test]
fn d0_missing_resource_acquisition_exact_tuple() {
    std::thread::Builder::new()
        .name("d0-missing-resource".to_owned())
        .stack_size(D0_WORKER_STACK_BYTES)
        .spawn(|| {
            let root = tempfile::tempdir().expect("D0 missing-resource root");
            let source = bracket_source::nested_bracket_source(1);
            // Do not provision held-0.bin: the program must observe the
            // actual FsOpen acquisition error and its outer failure branch.
            let output = ken_cli::build_native_program(
                &source,
                ken_cli::SourceFormat::Ken,
                "rt-bracket-d0-missing-resource",
                root.path(),
                ken_runtime::boundary_resource_profile::starter_smoke_profile(),
            )
            .expect("D0 missing-resource linked build");
            let native = ken_runtime::run_bound_process_effect_observation(
                &output.artifact,
                &ken_runtime::NativeEffectRunOptionsV1 {
                    arguments: Vec::new(),
                    environment: Vec::new(),
                    cwd: root.path().to_owned(),
                    plan_hash: output.plan_transport_hash,
                },
            )
            .expect("D0 missing-resource native observation");
            let mut host = ken_interp::PosixHost::new_at(root.path());
            let interpreted = ken_cli::run_program_effect_observation(
                &source,
                ken_cli::SourceFormat::Ken,
                &[],
                &[],
                root.path().as_os_str().as_encoded_bytes(),
                &mut host,
            )
            .expect("D0 missing-resource interpreter observation");
            let reference =
                normalize_observation(&interpreted).expect("D0 missing-resource interpreter tuple");
            let compiled =
                normalize_observation(&native).expect("D0 missing-resource native tuple");
            assert_eq!(reference, compiled, "D0 missing-resource exact tuple");
            assert_eq!(
                native.exit_status, 84,
                "acquisition failure reaches after_root Err"
            );
            assert!(
                matches!(native.effect_trace.as_slice(), [open]
                if open.operation == HostOpV1::FsOpen
                    && matches!(open.outcome, CanonicalOutcomeV1::Error(_))),
                "one real failed FsOpen, no release: {:?}",
                native.effect_trace
            );
        })
        .expect("D0 missing-resource scratch thread")
        .join()
        .expect("D0 missing-resource scratch thread result");
}

/// Promise class: durable invariant.
/// MEASURED: exact normalized O0/read-offset and W0/read-window tuples.
/// CLAIMED: nested read continuations preserve host effects and result.
/// THE GAP: only these two checked read paths execute here.
#[test]
fn d0_read_offset_and_window_exact_tuples() {
    std::thread::Builder::new()
        .name("d0-read-offset-window".to_owned())
        .stack_size(D0_WORKER_STACK_BYTES)
        .spawn(|| {
            let cases = [
                ("O0-read-offset", "rt_read_offset_stage"),
                ("W0-read-window", "rt_read_window_stage"),
            ];
            let mut executed = 0;
            for (case, entry) in cases {
                let root = tempfile::tempdir().expect("D0 read root");
                std::fs::write(root.path().join("source"), b"ab").expect("D0 read source file");
                let source = D0_PARITY_SOURCE.replace("__RT_PARITY_ENTRY__", entry);
                let output = ken_cli::build_native_program(
                    &source,
                    ken_cli::SourceFormat::Ken,
                    &format!("rt-bracket-d0-read-{}", case.replace('-', "_")),
                    root.path(),
                    ken_runtime::boundary_resource_profile::starter_smoke_profile(),
                )
                .unwrap_or_else(|error| panic!("D0 {case} linked build: {error:?}"));
                let native = ken_runtime::run_bound_process_effect_observation(
                    &output.artifact,
                    &ken_runtime::NativeEffectRunOptionsV1 {
                        arguments: Vec::new(),
                        environment: Vec::new(),
                        cwd: root.path().to_owned(),
                        plan_hash: output.plan_transport_hash,
                    },
                )
                .unwrap_or_else(|error| panic!("D0 {case} native run: {error:?}"));
                let mut host = ken_interp::PosixHost::new_at(root.path());
                let interpreted = ken_cli::run_program_effect_observation(
                    &source,
                    ken_cli::SourceFormat::Ken,
                    &[],
                    &[],
                    root.path().as_os_str().as_encoded_bytes(),
                    &mut host,
                )
                .unwrap_or_else(|error| panic!("D0 {case} reference run: {error:?}"));
                let reference = normalize_observation(&interpreted)
                    .unwrap_or_else(|error| panic!("D0 {case} interpreter tuple: {error}"));
                let compiled = normalize_observation(&native)
                    .unwrap_or_else(|error| panic!("D0 {case} native tuple: {error}"));
                assert_eq!(reference, compiled, "D0 {case} exact tuple");
                executed += 1;
            }
            assert_eq!(executed, 2, "O0 and W0 each executed in both engines");
        })
        .expect("D0 read scratch thread")
        .join()
        .expect("D0 read scratch thread result");
}

/// Promise class: durable invariant.
/// MEASURED: exact tuples for four two-op and two alternating Mapping forms.
/// CLAIMED: mapped access and settlement order and outcomes match interpreter.
/// THE GAP: only these six operation sequences execute here.
#[test]
fn d0_mapping_six_exact_tuples() {
    std::thread::Builder::new()
        .name("d0-mapping-six".to_owned())
        .stack_size(D0_WORKER_STACK_BYTES)
        .spawn(|| {
            let cases = [
                ("read-read", "read_read_body"),
                ("read-write", "read_write_body"),
                ("write-read", "write_read_body"),
                ("write-write", "write_write_body"),
                ("read-write-read", "read_write_read_body"),
                ("write-read-write", "write_read_write_body"),
            ];
            let mut executed = 0;
            for (case, body) in cases {
                let root = tempfile::tempdir().expect("D0 Mapping root");
                let source = D0_MAPPING_SOURCE
                    .replace("__ENTRY__", "matrix_stage")
                    .replace("__MATRIX_BODY__", body);
                let output = ken_cli::build_native_program(
                    &source,
                    ken_cli::SourceFormat::Ken,
                    &format!("rt-bracket-d0-mapping-{}", case.replace('-', "_")),
                    root.path(),
                    ken_runtime::boundary_resource_profile::starter_smoke_profile(),
                )
                .unwrap_or_else(|error| panic!("D0 {case} linked build: {error:?}"));
                let native = ken_runtime::run_bound_process_effect_observation(
                    &output.artifact,
                    &ken_runtime::NativeEffectRunOptionsV1 {
                        arguments: Vec::new(),
                        environment: Vec::new(),
                        cwd: root.path().to_owned(),
                        plan_hash: output.plan_transport_hash,
                    },
                )
                .unwrap_or_else(|error| panic!("D0 {case} native run: {error:?}"));
                let mut host = ken_interp::PosixHost::new_at(root.path());
                let interpreted = ken_cli::run_program_effect_observation(
                    &source,
                    ken_cli::SourceFormat::Ken,
                    &[],
                    &[],
                    root.path().as_os_str().as_encoded_bytes(),
                    &mut host,
                )
                .unwrap_or_else(|error| panic!("D0 {case} reference run: {error:?}"));
                let reference = normalize_observation(&interpreted)
                    .unwrap_or_else(|error| panic!("D0 {case} interpreter tuple: {error}"));
                let compiled = normalize_observation(&native)
                    .unwrap_or_else(|error| panic!("D0 {case} native tuple: {error}"));
                assert_eq!(reference, compiled, "D0 Mapping {case} exact tuple");
                executed += 1;
            }
            assert_eq!(executed, 6, "all six Mapping matrix forms executed");
        })
        .expect("D0 Mapping scratch thread")
        .join()
        .expect("D0 Mapping scratch thread result");
}

/// Promise class: durable invariant.
/// MEASURED: out-of-bounds branched-scrutinee exact ordered tuple.
/// CLAIMED: body failure releases the nested resources in interpreter order.
/// THE GAP: this one failed branch does not cover every scrutinee arm.
#[test]
fn d0_branched_scrutinee_out_of_bounds_exact_tuple() {
    std::thread::Builder::new()
        .name("d0-branched-out-of-bounds".to_owned())
        .stack_size(D0_WORKER_STACK_BYTES)
        .spawn(|| {
            let root = tempfile::tempdir().expect("D0 out-of-bounds root");
            std::fs::write(root.path().join("source"), b"0123456789abcdef")
                .expect("D0 readable source file");
            let source = D0_BRANCHED_SOURCE;
            let output = ken_cli::build_native_program(
                source,
                ken_cli::SourceFormat::Ken,
                "rt-bracket-d0-branched-out-of-bounds",
                root.path(),
                ken_runtime::boundary_resource_profile::starter_smoke_profile(),
            )
            .expect("D0 branched linked native build");
            let native = ken_runtime::run_bound_process_effect_observation(
                &output.artifact,
                &ken_runtime::NativeEffectRunOptionsV1 {
                    arguments: Vec::new(),
                    environment: Vec::new(),
                    cwd: root.path().to_owned(),
                    plan_hash: output.plan_transport_hash,
                },
            )
            .expect("D0 branched native observation");
            let mut host = ken_interp::PosixHost::new_at(root.path());
            let interpreted = ken_cli::run_program_effect_observation(
                source,
                ken_cli::SourceFormat::Ken,
                &[],
                &[],
                root.path().as_os_str().as_encoded_bytes(),
                &mut host,
            )
            .expect("D0 branched interpreter observation");
            let reference = normalize_observation(&interpreted).expect("D0 interpreter tuple");
            let compiled = normalize_observation(&native).expect("D0 native tuple");
            assert_eq!(
                reference, compiled,
                "D0 branched out-of-bounds ordered tuple"
            );
        })
        .expect("D0 branched scratch thread")
        .join()
        .expect("D0 branched scratch thread result");
}

/// Promise class: normative compatibility vector for the fixed D0 row roster.
/// MEASURED: eight historical parity and two additional ABI depth-two tuples.
/// CLAIMED: these checked nested bracket routes retain full order and result.
/// THE GAP: these builds share a source family; no general route closure follows.
// Builds 0 and 2 of row 01 use the same checked read source but separate links.
#[test]
fn d0_measured_depth_two_members_exact_tuples() {
    std::thread::Builder::new()
        .name("d0-measured-depth-two".to_owned())
        .stack_size(D0_WORKER_STACK_BYTES)
        .spawn(|| {
            let cases = [
                ("parity-01-build-0", false, "rt_read_offset_stage"),
                ("parity-01-build-2", false, "rt_read_offset_stage"),
                ("parity-14-read", false, "rt_read_offset_stage"),
                ("parity-14-write", false, "rt_write_writable_stage"),
                ("parity-17-read", false, "rt_read_offset_stage"),
                ("parity-17-write", false, "rt_write_writable_stage"),
                ("parity-34-invalid", false, "rt_cap41_out_of_range_stage"),
                (
                    "parity-34-eof-witness",
                    false,
                    "rt_cap41_out_of_range_eof_stage",
                ),
                ("abi-extra-out-of-range", true, "read_then_out_of_range_stage"),
                ("abi-extra-read-only", true, "read_then_read_only_write_stage"),
            ];
            let mut executed = 0;
            for (case, mapping, entry) in cases {
                let root = tempfile::tempdir().expect("D0 measured-member root");
                std::fs::write(root.path().join("source"), b"ab").expect("D0 parity source file");
                let source = if mapping {
                    D0_MAPPING_SOURCE
                        .replace("__ENTRY__", entry)
                        .replace("__MATRIX_BODY__", "read_read_body")
                } else {
                    D0_PARITY_SOURCE.replace("__RT_PARITY_ENTRY__", entry)
                };
                let output = ken_cli::build_native_program(
                    &source,
                    ken_cli::SourceFormat::Ken,
                    &format!("d0-measured-{}", case),
                    root.path(),
                    ken_runtime::boundary_resource_profile::starter_smoke_profile(),
                )
                .unwrap_or_else(|error| panic!("D0 {case} linked build: {error:?}"));
                let native = ken_runtime::run_bound_process_effect_observation(
                    &output.artifact,
                    &ken_runtime::NativeEffectRunOptionsV1 {
                        arguments: Vec::new(),
                        environment: Vec::new(),
                        cwd: root.path().to_owned(),
                        plan_hash: output.plan_transport_hash,
                    },
                )
                .unwrap_or_else(|error| panic!("D0 {case} native run: {error:?}"));
                let mut host = ken_interp::PosixHost::new_at(root.path());
                let interpreted = ken_cli::run_program_effect_observation(
                    &source,
                    ken_cli::SourceFormat::Ken,
                    &[],
                    &[],
                    root.path().as_os_str().as_encoded_bytes(),
                    &mut host,
                )
                .unwrap_or_else(|error| panic!("D0 {case} interpreter run: {error:?}"));
                assert_eq!(
                    normalize_observation(&interpreted).expect("reference tuple"),
                    normalize_observation(&native).expect("native tuple"),
                    "D0 {case} exact ordered tuple"
                );
                executed += 1;
            }
            assert_eq!(executed, 10, "eight parity and two ABI cases executed");
        })
        .expect("D0 measured-member thread")
        .join()
        .expect("D0 measured-member thread result");
}

/// Promise class: durable invariant.
/// MEASURED: per-installed-row dual owner insertion refuses at exact plan
/// close. CLAIMED: BracketOwned and Specialized cannot both own one S.
/// THE GAP: this exercises post-install assignment; the derivation's marked
/// demands are structurally excluded from Phase B, not tested by this hook.
#[test]
fn d0_px8ta_dual_owner_plan_close_refuses_per_installed_row() {
    std::thread::Builder::new()
        .name("d0-dual-owner-plan-close".to_owned())
        .stack_size(D0_WORKER_STACK_BYTES)
        .spawn(|| {
            let mut executed = 0;
            for depth in [2, 3] {
                let root = tempfile::tempdir().expect("D0 dual-owner root");
                for index in 0..depth {
                    std::fs::write(root.path().join(format!("held-{index}.bin")),
                        format!("held resource {index}"))
                        .expect("D0 dual-owner resource input");
                }
                let source = bracket_source::nested_bracket_source(depth);
                let build = |suffix: &str| {
                    ken_runtime::with_static_response_feasibility_diagnostics(|| {
                        ken_cli::build_native_program(
                            &source, ken_cli::SourceFormat::Ken,
                            &format!("rt-bracket-d0-dual-{depth}-{suffix}"), root.path(),
                            ken_runtime::boundary_resource_profile::starter_smoke_profile(),
                        )
                    })
                };
                let (positive, baseline_diagnostics) = build("positive");
                positive.expect("unmutated D0 row must link");
                assert_eq!(baseline_diagnostics.len(), 1, "one installed plane per build");
                let installed = baseline_diagnostics[0].bracket_response_rows.len();
                assert!(installed >= 1, "D0 depth {depth} reaches an installed bracket row");
                let ((mutated, diagnostics), applications) =
                    ken_runtime::with_bracket_ordinary_owner_mutation(|| build("dual"));
                let error = mutated.expect_err("dual-owned bracket plane must refuse");
                let ken_elaborator::compiler_driver::NativeProgramBuildError::Packaging(
                    packaging,
                ) = error else {
                    panic!("D0 depth {depth} did not refuse at object emission: {error:?}");
                };
                assert_eq!(
                    packaging.stage,
                    ken_runtime::ObjectLinkerPackagingStage::ObjectEmission,
                    "D0 depth {depth}: plan-close refusal stage"
                );
                let prefix = "Cranelift backend failure: native static transition planner invariant failed; please report this compiler bug: ";
                assert_eq!(
                    packaging.reason.strip_prefix(prefix),
                    Some("the installed response context/continuation plane is not its exact closed derivation"),
                    "D0 depth {depth} reached a different refusal: {packaging:?}"
                );
                assert!(diagnostics.is_empty(),
                    "D0 depth {depth} must refuse before plan diagnostic publication");
                assert_eq!(applications, installed,
                    "D0 depth {depth} applies the hook to every installed bracket row");
                eprintln!("D0 depth {depth}: installed Continuation bracket rows={installed}, mutation applications={applications}");
                assert!(ken_runtime::bracket_ordinary_owner_mutation_is_exact(),
                    "D0 depth {depth} restores the owner mutation hook");
                executed += 1;
            }
            assert_eq!(executed, 2, "both nested D0 rows exercise dual ownership");
        })
        .expect("D0 dual-owner thread")
        .join()
        .expect("D0 dual-owner thread result");
}

/// Promise class: durable invariant.
/// MEASURED: depth-two exact tuple, ordered child then parent releases.
/// CLAIMED: explicit nested bracket control preserves order and result.
/// THE GAP: only this checked depth and root execute.
#[test]
fn d0_px8ta_depth_two_exact_tuple() {
    d0_px8ta_exact_tuple(2);
}

/// Promise class: durable invariant.
/// MEASURED: depth-three exact tuple, ordered three-resource LIFO.
/// CLAIMED: nested producer routing preserves result and effect order.
/// THE GAP: this finite depth does not prove arbitrary nesting.
#[test]
fn d0_px8ta_depth_three_exact_tuple() {
    d0_px8ta_exact_tuple(3);
}

fn d0_px8ta_exact_tuple(depth: usize) {
    std::thread::Builder::new()
        .name(format!("d0-px8ta-depth-{depth}"))
        .stack_size(D0_WORKER_STACK_BYTES)
        .spawn(move || {
            let root = tempfile::tempdir().expect("D0 scratch root");
            for index in 0..depth {
                std::fs::write(
                    root.path().join(format!("held-{index}.bin")),
                    format!("held resource {index}"),
                )
                .expect("D0 input");
            }
            let source = bracket_source::nested_bracket_source(depth);
            let output = ken_cli::build_native_program(
                &source,
                ken_cli::SourceFormat::Ken,
                &format!("rt-bracket-d0-depth-{depth}"),
                root.path(),
                ken_runtime::boundary_resource_profile::starter_smoke_profile(),
            )
            .expect("D0 linked native build");
            let native = ken_runtime::run_bound_process_effect_observation(
                &output.artifact,
                &ken_runtime::NativeEffectRunOptionsV1 {
                    arguments: Vec::new(),
                    environment: Vec::new(),
                    cwd: root.path().to_owned(),
                    plan_hash: output.plan_transport_hash,
                },
            )
            .expect("D0 native observation");
            let mut host = ken_interp::PosixHost::new_at(root.path());
            let interpreted = ken_cli::run_program_effect_observation(
                &source,
                ken_cli::SourceFormat::Ken,
                &[],
                &[],
                root.path().as_os_str().as_encoded_bytes(),
                &mut host,
            )
            .expect("D0 reference interpreter observation");
            let reference = normalize_observation(&interpreted).expect("D0 interpreter tuple");
            let compiled = normalize_observation(&native).expect("D0 native tuple");
            assert_eq!(
                reference, compiled,
                "depth {depth} same checked input D0 tuple"
            );
        })
        .expect("D0 scratch thread")
        .join()
        .expect("D0 scratch thread result");
}
