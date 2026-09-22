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
