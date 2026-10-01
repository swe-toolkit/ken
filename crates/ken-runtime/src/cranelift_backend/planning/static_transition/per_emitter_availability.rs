//! Planner-only census of where an interned continuation's recursive field
//! can be materialized, and what that point's own emitter can actually hold.
//! An unfinalizable claim is a recorded result, not a new program refusal.

use super::continuations::{
    self, ContinuationEmitterFrame, ContinuationProducerEnvironment,
    ContinuationSourceSlotAuthority, ContinuationValueSourceAuthority,
    ContinuationWorkerCaptureSource,
};
use super::*;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) enum MaterializationKind {
    ConstructEmission,
    CheckedIhTransportDestination,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) enum CaptureRun {
    Worker,
    Context,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) enum UnfinalizableReason {
    NoClaim,
    Ambiguous(usize),
    ForeignOwner,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) enum PerEmitterCaptureClaim {
    Finalized(ContinuationEnvironmentClaim),
    Unfinalizable {
        ordinal: u32,
        owner: ContinuationEmissionOwner,
        reason: UnfinalizableReason,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct PerEmitterCaptureResult {
    pub(in crate::cranelift_backend) run: CaptureRun,
    pub(in crate::cranelift_backend) ordinal: u32,
    pub(in crate::cranelift_backend) source: Option<ContinuationSourceCoordinate>,
    pub(in crate::cranelift_backend) claim: PerEmitterCaptureClaim,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct PerEmitterMaterialization {
    pub(in crate::cranelift_backend) specialization: ContinuationSpecializationId,
    pub(in crate::cranelift_backend) producer_construct_origin: StaticOriginId,
    pub(in crate::cranelift_backend) recursive_position: u32,
    pub(in crate::cranelift_backend) emission_origin: StaticOriginId,
    pub(in crate::cranelift_backend) owner: ContinuationEmissionOwner,
    pub(in crate::cranelift_backend) kind: MaterializationKind,
    pub(in crate::cranelift_backend) captures: Vec<PerEmitterCaptureResult>,
}

/// A field emitted at a classified construct, with no interned specialization
/// for this `(construct, field, owner)`. It cannot become a residual by guess.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) struct UnclassifiedMaterialization {
    pub(in crate::cranelift_backend) construct_origin: StaticOriginId,
    pub(in crate::cranelift_backend) field: u32,
    pub(in crate::cranelift_backend) owner: ContinuationEmissionOwner,
}

/// The source expression, not a physical `producer_env` index, identifies a
/// worker capture. Only a Var whose semantic source walk proves one complete
/// source slot can be projected into another emitter's frame. A seed, computed
/// value, Open source, or join of distinct sources has no such coordinate.
fn worker_source_slot(
    plan: &StaticTransitionPlan<'_>,
    capture: &continuations::ContinuationWorkerCaptureProvenance,
) -> Result<Result<ContinuationSourceSlotAuthority, UnfinalizableReason>, CraneliftBackendError> {
    let ContinuationWorkerCaptureSource::Lexical(origin) = capture.source() else {
        return Ok(Err(UnfinalizableReason::NoClaim));
    };
    let RuntimeExpr::Var(index) = plan.planned_occurrence_expr(origin)? else {
        return Ok(Err(UnfinalizableReason::NoClaim));
    };
    let root = continuations::continuation_owner_source_root(plan, capture.owner())?;
    let entry = continuations::continuation_owner_entry_sources(plan, capture.owner())?
        .into_iter()
        .map(ContinuationValueSourceAuthority::source)
        .collect::<Vec<_>>();
    let (_, reached) =
        continuations::walk_continuation_value_environment(plan, root, origin, &entry)?;
    let seat = reached.ok_or_else(|| {
        planner_error("a worker capture's lexical source is outside its declared owner")
    })?;
    Ok(match seat.get(*index as usize) {
        Some(ContinuationValueSourceAuthority::Closed(sources)) if sources.len() == 1 => {
            Ok(sources[0].clone())
        }
        Some(ContinuationValueSourceAuthority::Closed(sources)) if sources.len() > 1 => {
            Err(UnfinalizableReason::Ambiguous(sources.len()))
        }
        _ => Err(UnfinalizableReason::NoClaim),
    })
}

/// Classify only the two previously fail-closed membership sites. The
/// original interning path still uses the refusing projection unchanged;
/// unrelated planner errors in that projection or frame finalization propagate.
fn point_claim(
    plan: &StaticTransitionPlan<'_>,
    unit: &continuations::PlannedContinuationSpecialization,
    source: &ContinuationSourceSlotAuthority,
    emission_origin: StaticOriginId,
    frame: &ContinuationEmitterFrame<'_>,
) -> Result<Result<ContinuationEnvironmentClaim, UnfinalizableReason>, CraneliftBackendError> {
    match frame {
        ContinuationEmitterFrame::GeneratedContext {
            enclosing_inputs, ..
        } => {
            let count = enclosing_inputs
                .iter()
                .filter(|input| input.coordinate == source.coordinate)
                .count();
            if count == 0 {
                return Ok(Err(UnfinalizableReason::NoClaim));
            }
            if count != 1 {
                return Ok(Err(UnfinalizableReason::Ambiguous(count)));
            }
        }
        ContinuationEmitterFrame::Predeclared(p) => {
            if *p != unit.key.producer_owner {
                return Ok(Err(UnfinalizableReason::ForeignOwner));
            }
            if emission_origin != unit.key.producer_construct_origin {
                // The direct lexical claim belongs to the exact creation
                // seat, not to another occurrence reusing its source key.
                return Ok(Err(UnfinalizableReason::NoClaim));
            }
            let entry_count = continuations::continuation_owner_entry_sources(plan, *p)?
                .iter()
                .filter(|member| member.coordinate == source.coordinate)
                .count();
            if entry_count > 1 {
                return Ok(Err(UnfinalizableReason::Ambiguous(entry_count)));
            }
            let environment = ContinuationProducerEnvironment {
                producer_owner: unit.key.producer_owner,
                producer_result_origin: unit.key.producer_result_origin,
                producer_construct_origin: unit.key.producer_construct_origin,
                consumer_owner: unit.key.consumer_owner,
                inputs: Vec::new(),
            };
            let (_, seat) =
                continuations::continuation_emission_seat_environment(plan, &environment)?;
            // The interning path's exact alias rule returns a typed absence
            // here; capacity errors still propagate rather than being recast
            // as NoClaim.
            match continuations::probe_nearest_exact_alias(source, &seat)? {
                continuations::ExactAliasProbe::Found(_) => {}
                continuations::ExactAliasProbe::Ambiguous(count) => {
                    return Ok(Err(UnfinalizableReason::Ambiguous(count)));
                }
                continuations::ExactAliasProbe::Missing
                | continuations::ExactAliasProbe::ContractMismatch => {
                    return Ok(Err(UnfinalizableReason::NoClaim));
                }
            }
        }
    }
    let environment = ContinuationProducerEnvironment {
        producer_owner: unit.key.producer_owner,
        producer_result_origin: unit.key.producer_result_origin,
        producer_construct_origin: unit.key.producer_construct_origin,
        consumer_owner: unit.key.consumer_owner,
        inputs: vec![source.clone()],
    };
    let projected = continuations::exact_continuation_projection(
        plan,
        &environment,
        unit.key.ordinary_parameters,
        frame,
    )?;
    let draft = projected
        .into_iter()
        .next()
        .ok_or_else(|| {
            planner_error("one requested capture produced no exact continuation projection")
        })?
        .availability;
    let views =
        continuations::finalize_continuation_availability(&plan.continuation_contexts, draft)?;
    // The residual consumes the emitter's entry/context slot when available;
    // direct_emission is the sole fallback at that exact creator-local seat.
    let claim = views
        .context_capture
        .or(views.direct_emission)
        .ok_or_else(|| planner_error("an available source has neither emission nor frame claim"))?;
    Ok(Ok(claim))
}

fn capture_result(
    plan: &StaticTransitionPlan<'_>,
    unit: &continuations::PlannedContinuationSpecialization,
    run: CaptureRun,
    ordinal: u32,
    source: Option<&ContinuationSourceSlotAuthority>,
    missing_reason: UnfinalizableReason,
    owner: ContinuationEmissionOwner,
    emission_origin: StaticOriginId,
    frame: Option<&ContinuationEmitterFrame<'_>>,
) -> Result<PerEmitterCaptureResult, CraneliftBackendError> {
    let claim = match (source, frame) {
        (Some(source), Some(frame)) => {
            match point_claim(plan, unit, source, emission_origin, frame)? {
                Ok(claim) => PerEmitterCaptureClaim::Finalized(claim),
                Err(reason) => PerEmitterCaptureClaim::Unfinalizable {
                    ordinal,
                    owner,
                    reason,
                },
            }
        }
        (None, _) => PerEmitterCaptureClaim::Unfinalizable {
            ordinal,
            owner,
            reason: missing_reason,
        },
        (_, None) => PerEmitterCaptureClaim::Unfinalizable {
            ordinal,
            owner,
            reason: UnfinalizableReason::ForeignOwner,
        },
    };
    Ok(PerEmitterCaptureResult {
        run,
        ordinal,
        source: source.map(|source| source.coordinate),
        claim,
    })
}

pub(super) fn build_per_emitter_availability(
    plan: &StaticTransitionPlan<'_>,
) -> Result<
    (
        Vec<PerEmitterMaterialization>,
        Vec<UnclassifiedMaterialization>,
    ),
    CraneliftBackendError,
> {
    let mut points = BTreeSet::new();
    let mut classified = BTreeSet::new();
    let mut construct_owners = BTreeSet::new();
    for call in &plan.continuation_specialization_calls {
        let token = &call.token;
        let unit = plan
            .continuation_specializations
            .get(token.target.0 as usize)
            .ok_or_else(|| planner_error("a materialization names no interned specialization"))?;
        points.insert((
            token.target,
            token.producer_construct_origin,
            unit.key.recursive_position,
            token.producer_construct_origin,
            token.emission_owner,
            MaterializationKind::ConstructEmission,
        ));
        classified.insert((
            token.producer_construct_origin,
            unit.key.recursive_position,
            token.emission_owner,
        ));
        construct_owners.insert((token.producer_construct_origin, token.emission_owner));
    }
    for transport in &plan.checked_ih_environment_transports {
        let source = transport.source_specialization();
        let unit = plan
            .continuation_specializations
            .get(source.0 as usize)
            .ok_or_else(|| {
                planner_error("a checked-IH transport names no source specialization")
            })?;
        // The transport itself names its source specialization and exact
        // destination field. Neither is inferred from a coincident origin or
        // field number in the source specialization's own construct.
        points.insert((
            source,
            unit.key.producer_construct_origin,
            transport.recursive_position(),
            transport.destination_construct_origin(),
            transport.destination_owner(),
            MaterializationKind::CheckedIhTransportDestination,
        ));
        classified.insert((
            transport.destination_construct_origin(),
            transport.recursive_position(),
            transport.destination_owner(),
        ));
        construct_owners.insert((
            transport.destination_construct_origin(),
            transport.destination_owner(),
        ));
    }
    let mut unclassified = BTreeSet::new();
    for (construct_origin, owner) in construct_owners {
        let RuntimeExpr::Construct { args, .. } = plan.planned_occurrence_expr(construct_origin)?
        else {
            return Err(planner_error(
                "a materialization point is not a construct occurrence",
            ));
        };
        for field in 0..args.len() {
            let field = u32::try_from(field)
                .map_err(|_| planner_capacity_error("construct materialization field exhausted"))?;
            if !classified.contains(&(construct_origin, field, owner)) {
                unclassified.insert(UnclassifiedMaterialization {
                    construct_origin,
                    field,
                    owner,
                });
            }
        }
    }
    let mut results = Vec::with_capacity(points.len());
    for (
        specialization,
        producer_construct_origin,
        recursive_position,
        emission_origin,
        owner,
        kind,
    ) in points
    {
        let unit = &plan.continuation_specializations[specialization.0 as usize];
        let frame = match owner {
            ContinuationEmissionOwner::Fusion(_) => None,
            _ => Some(continuations::emitter_frame_for_owner(
                &plan.continuation_specializations,
                owner,
            )?),
        };
        let mut captures =
            Vec::with_capacity(unit.key.worker.captures.len() + unit.key.continuation_inputs.len());
        for capture in &unit.key.worker.captures {
            let source = worker_source_slot(plan, capture)?;
            let (source, missing_reason) = match source {
                Ok(source) => (Some(source), UnfinalizableReason::NoClaim),
                Err(reason) => (None, reason),
            };
            captures.push(capture_result(
                plan,
                unit,
                CaptureRun::Worker,
                capture.ordinal,
                source.as_ref(),
                missing_reason,
                owner,
                emission_origin,
                frame.as_ref(),
            )?);
        }
        for input in &unit.key.continuation_inputs {
            let source = ContinuationSourceSlotAuthority {
                coordinate: input.coordinate,
                carrier: input.carrier,
                ownership: input.ownership,
                storage_owner: input.storage_owner,
                referent_affinity: input.referent_affinity.clone(),
            };
            captures.push(capture_result(
                plan,
                unit,
                CaptureRun::Context,
                input.ordinal,
                Some(&source),
                UnfinalizableReason::NoClaim,
                owner,
                emission_origin,
                frame.as_ref(),
            )?);
        }
        results.push(PerEmitterMaterialization {
            specialization,
            producer_construct_origin,
            recursive_position,
            emission_origin,
            owner,
            kind,
            captures,
        });
    }
    Ok((results, unclassified.into_iter().collect()))
}

#[cfg(feature = "px8-ds-test-support")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PerEmitterOwnerDiagnostic {
    Predeclared(u32),
    Specialization(u32),
    Fusion(u32),
}

#[cfg(feature = "px8-ds-test-support")]
fn diagnostic_owner(owner: ContinuationEmissionOwner) -> PerEmitterOwnerDiagnostic {
    match owner {
        ContinuationEmissionOwner::Predeclared(id) => PerEmitterOwnerDiagnostic::Predeclared(id.0),
        ContinuationEmissionOwner::Specialization(id) => {
            PerEmitterOwnerDiagnostic::Specialization(id.0)
        }
        ContinuationEmissionOwner::Fusion(id) => PerEmitterOwnerDiagnostic::Fusion(id.0),
    }
}

#[cfg(feature = "px8-ds-test-support")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PerEmitterCaptureDiagnostic {
    pub run: String,
    pub ordinal: u32,
    pub source: Option<String>,
    pub result: String,
    /// Obtained from the capture's typed result, not from its enclosing point.
    pub unfinalizable_owner: Option<PerEmitterOwnerDiagnostic>,
}

#[cfg(feature = "px8-ds-test-support")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PerEmitterMaterializationDiagnostic {
    pub specialization: u32,
    pub producer_construct_origin: u32,
    pub recursive_position: u32,
    pub emission_origin: u32,
    pub owner: String,
    pub kind: String,
    pub captures: Vec<PerEmitterCaptureDiagnostic>,
}

#[cfg(feature = "px8-ds-test-support")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PerEmitterAvailabilityDiagnostic {
    pub materializations: Vec<PerEmitterMaterializationDiagnostic>,
    pub unclassified: Vec<(u32, u32, String)>,
    /// Exact debug serialization of every interned key, including its
    /// continuation-input availability drafts, for before/after comparison.
    pub interned_keys: Vec<String>,
}

#[cfg(feature = "px8-ds-test-support")]
thread_local! {
    static DIAGNOSTICS: std::cell::RefCell<Option<Vec<PerEmitterAvailabilityDiagnostic>>> =
        const { std::cell::RefCell::new(None) };
}

#[cfg(feature = "px8-ds-test-support")]
pub fn with_per_emitter_availability_diagnostics<T>(
    operation: impl FnOnce() -> T,
) -> (T, Vec<PerEmitterAvailabilityDiagnostic>) {
    DIAGNOSTICS.with(|slot| {
        assert!(slot.borrow().is_none(), "per-emitter windows cannot nest");
        *slot.borrow_mut() = Some(Vec::new());
    });
    let result = operation();
    let observations = DIAGNOSTICS.with(|slot| {
        slot.borrow_mut()
            .take()
            .expect("per-emitter observation window")
    });
    (result, observations)
}

#[cfg(feature = "px8-ds-test-support")]
pub(super) fn record_per_emitter_availability_diagnostic(plan: &StaticTransitionPlan<'_>) {
    DIAGNOSTICS.with(|slot| {
        let mut slot = slot.borrow_mut();
        let Some(rows) = slot.as_mut() else { return };
        rows.push(PerEmitterAvailabilityDiagnostic {
            materializations: plan
                .per_emitter_materializations()
                .iter()
                .map(|point| PerEmitterMaterializationDiagnostic {
                    specialization: point.specialization.0,
                    producer_construct_origin: point.producer_construct_origin.0,
                    recursive_position: point.recursive_position,
                    emission_origin: point.emission_origin.0,
                    owner: format!("{:?}", point.owner),
                    kind: format!("{:?}", point.kind),
                    captures: point
                        .captures
                        .iter()
                        .map(|capture| PerEmitterCaptureDiagnostic {
                            run: format!("{:?}", capture.run),
                            ordinal: capture.ordinal,
                            source: capture.source.map(|source| format!("{source:?}")),
                            result: format!("{:?}", capture.claim),
                            unfinalizable_owner: match capture.claim {
                                PerEmitterCaptureClaim::Finalized(_) => None,
                                PerEmitterCaptureClaim::Unfinalizable { owner, .. } => {
                                    Some(diagnostic_owner(owner))
                                }
                            },
                        })
                        .collect(),
                })
                .collect(),
            unclassified: plan
                .unclassified_materializations
                .iter()
                .map(|point| {
                    (
                        point.construct_origin.0,
                        point.field,
                        format!("{:?}", point.owner),
                    )
                })
                .collect(),
            interned_keys: plan
                .continuation_specializations
                .iter()
                .map(|unit| format!("{:?}", unit.key))
                .collect(),
        });
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unavailable_emitter_members_are_typed_not_planner_refusals() {
        let expr = Box::leak(Box::new(
            super::continuations::tests::contspec_multiple_worker_captures_fixture(),
        ));
        let plan = plan_static_transition_graph(expr, &BTreeMap::new()).expect("source plans");
        let unit = &plan.continuation_specializations[0];
        let input = &unit.key.continuation_inputs[0];
        let source = ContinuationSourceSlotAuthority {
            coordinate: input.coordinate,
            carrier: input.carrier,
            ownership: input.ownership,
            storage_owner: input.storage_owner,
            referent_affinity: input.referent_affinity.clone(),
        };
        let own = continuations::emitter_frame_for_owner(
            &plan.continuation_specializations,
            unit.key.emission_owner,
        )
        .expect("actual frame exists");
        assert!(
            matches!(
                point_claim(
                    &plan,
                    unit,
                    &source,
                    unit.key.producer_construct_origin,
                    &own
                ),
                Ok(Ok(_))
            ),
            "the fixture must also exercise a real available claim"
        );

        let foreign = ContinuationEmitterFrame::Predeclared(PredeclaredFunctionId(
            unit.key.producer_owner.0 + 1,
        ));
        assert_eq!(
            point_claim(
                &plan,
                unit,
                &source,
                unit.key.producer_construct_origin,
                &foreign
            )
            .unwrap(),
            Err(UnfinalizableReason::ForeignOwner)
        );
        let missing = ContinuationEmitterFrame::GeneratedContext {
            enclosing: unit.id,
            worker_body_origin: unit.key.worker.body_origin,
            context_parameters: 0,
            enclosing_inputs: &[],
        };
        assert_eq!(
            point_claim(
                &plan,
                unit,
                &source,
                unit.key.producer_construct_origin,
                &missing
            )
            .unwrap(),
            Err(UnfinalizableReason::NoClaim)
        );
        let twice = [input.clone(), input.clone()];
        let ambiguous = ContinuationEmitterFrame::GeneratedContext {
            enclosing: unit.id,
            worker_body_origin: unit.key.worker.body_origin,
            context_parameters: 0,
            enclosing_inputs: &twice,
        };
        assert_eq!(
            point_claim(
                &plan,
                unit,
                &source,
                unit.key.producer_construct_origin,
                &ambiguous
            )
            .unwrap(),
            Err(UnfinalizableReason::Ambiguous(2))
        );
    }
}

impl StaticTransitionPlan<'_> {
    /// Read-only planner result. No lowering consumer is installed by this WP.
    pub(in crate::cranelift_backend) fn per_emitter_materializations(
        &self,
    ) -> &[PerEmitterMaterialization] {
        &self.per_emitter_materializations
    }
}
