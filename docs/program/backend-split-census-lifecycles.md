# Backend split census: lifecycle, evidence, and closeout

Measurement SHA: `4de48651434dd6340f81ec9b1b7a5ac2ec8c0199`

This inventory records the authority-bearing lifecycle families in the
backend. It treats an authority as a value or ledger whose production licenses
a later operation and whose closeout detects missing, duplicate, foreign, or
unconsumed evidence. Plain identifiers without such a lifecycle stay in the
type-ownership inventory.

## Boundary and selector

The search domain is every Rust file below
`crates/ken-runtime/src/cranelift_backend/` plus
`crates/ken-runtime/src/boundary_value_clif.rs`. Candidate declarations came
from this identifier selector:

```text
(Authority|Ledger|Claim|Token|Identity|Owner|Seat|Closure|Evidence)
```

Candidate lifecycle operations came from:

```text
(mint|issue|record|consume|close|finish|finalize|validate|claim|settle|rebind|recognize)
```

Those selectors find names, not laws. They miss authority expressed through a
plainly named field, enum state, or type alias, and they over-enter observation
types whose names contain the same words. The ledger below is the source-read
classification of that candidate set; a zero name hit would not prove that a
lifecycle is absent.

## Lifecycle ledger

| authority or ledger | owner | production and evidence | consumer and closeout |
|---|---|---|---|
| `PlannedOccurrenceAuthority` and `PlannedOccurrenceChildAuthority` | `planning/static_transition.rs` | The planner records one source occurrence and its ordered child identities while constructing the static plan. | `validate_occurrence_authority_plan` checks the completed parent/child relation before publication; lowering reads the published occurrence relations. |
| `CaseProducerAuthority` | `planning/static_transition.rs` | Match planning records the producer occurrence and case-body ownership used to authorize case emission. | `validate_case_emission_plan` closes the case population before the plan becomes consumable. |
| `JoinPlanToken` | `planning/static_transition.rs` | Planning issues a token for a join result whose representation and predecessor population are fixed. | Lowering calls `consume_join_plan`; `validate_join_plan_consumption` and `finalize_join_disposition` reject missing, duplicate, or mismatched consumption. |
| `PlannedTrapIdentity` and `TrapExitAuthority` | planner plus `lowering/mod.rs` | Planning mints trap identities and lowering binds the selected identity to a trap exit. | The trap exit path consumes the identity; its validation rejects a wrong planned seat, wrong frame binding, or caller protocol. |
| `ContinuationEnvironmentClaimOver`, `ContinuationValueSourceAuthority`, and `ContinuationSourceSlotAuthority` | `planning/static_transition.rs` | Planning derives a closed source coordinate, environment claim, and exact continuation input slot. | `finalize_continuation_claim`, `finalize_continuation_availability`, and `validate_continuation_source_slot` close the draft; lowering must match on the published coordinate rather than infer an ABI position. |
| `ContinuationSpecializationCallToken` and `ContinuationConsumingOccurrence` | `planning/static_transition.rs` | The forward walk records the specialization call and exact consuming occurrence. | `validate_continuation_consuming_occurrences` and `validate_continuation_specialization_closure` reject missing, duplicate, or conflicting occurrences before lowering reads them. |
| `RequiredConsumerProjection` | `planning/static_transition.rs` | Planning derives the required consumer from the source relation and records a projection only when source and required occurrences differ. | `validate_required_consumer_projections` closes the set; lowering consumes the published projection at the carrier boundary. |
| `FusionRegionClaimLedger` and `FusionRegionClaim` | `planning/static_transition.rs` | The ledger records defined and redirected regions, then `claim` selects one opaque outstanding claim. | `consume` settles the selected claim exactly once; `close` refuses every still-outstanding, replayed, or inconsistent claim and returns the consumption count. |
| `StaticContinuationFusionPlan`, key, descriptor, and id | `planning/static_transition.rs` | The planner interns the fusion key, assigns its id, and publishes the exact descriptor and oriented plan relation. | `validate_static_worker_member_population` and the specialization/fusion validators close the plan; lowering reads the same key, descriptor, call, and projection relation. |
| `AbiContinuationInputAuthority` and continuation ABI descriptors | `planning/static_transition/abi.rs` | ABI derivation records the continuation owner, input provenance, capture layout, and frame/context identity. | ABI preflight and the static-transition validators reject a descriptor whose owner, input run, or capture provenance disagrees with the semantic plan. |
| `CheckedCallLedger` and `CheckedCallRecord` | `lowering/units.rs` | Unit construction records each declared call and the checked target/argument relation it authorizes. | Unit emission validates and consumes those records; the ledger closeout rejects undeclared, duplicate, or unconsumed calls. |
| `ContinuationClaimLedger` | `lowering/units.rs` | Unit emission records candidate continuation claims issued by the plan. | The lowering consumer settles the exact selected claim; closeout rejects claims that remain unconsumed after unit emission. |
| `ContinuationCandidateLedger` | `lowering/units.rs` | Candidate formation records each continuation identity and its disposition. | Selection changes the candidate from outstanding to accepted or declined; closeout rejects an unresolved or multiply settled candidate. |
| `FusionCompositionLedger` | `lowering/units.rs` | Composition records the exact fusion target class and layer used for a unit. | The emitted unit must consume the recorded composition; closeout rejects missing or extra composition evidence. |
| `StaticWorkerFieldLedger`, `StaticWorkerTransportId`, and `StaticWorkerRecognitionId` | `lowering/mod.rs` | `TransportIdIssuer::mint` names a transport; `RecognitionIdIssuer::mint` and `recognize` create a distinct field obligation; `rebind` transports it without changing the recognition identity. | Field consumption marks the recognition consumed; `StaticWorkerFieldLedger::close` rejects an unconsumed recognition and reports that its own transport never reached a consumer. |
| `AmbientBodyAuthority` and `CheckedFrameFunctionScope` | `lowering/core.rs` | Entering a checked frame records the body/function authority visible for that dynamic scope. | Their `close`/`finish` paths compare the observed frame and subcontinuation consumptions before the scope can return. |
| `AggregateAllocationLedger` and `AggregateRelationClosure` | `lowering/mod.rs` | Lowering records each governed aggregate allocation together with its planned producer, owner, root, and child relation. | `AggregateAllocationLedger::close` reconciles events with the plan; `AggregateRelationClosure` is the closed relation handed to later emission. |
| `EffectSeatLedger`, `EffectSeatGroupId`, and `EffectSeatClosure` | `lowering/mod.rs` | `EffectSeatGroupId::mint` opens a group and `claim_host_effect_seat` claims each planned host-effect seat once. | `close_host_effect_seat_group` commits one group; the ledger `close` rejects missing, duplicate, foreign, or still-open seats and yields `EffectSeatClosure`. |
| `OrientedControlLedgerEntry` | `lowering/mod.rs` | Lowering records each oriented control projection and the planned identity it follows. | `consume` marks the entry; `validate_open_control_obligations` rejects any entry left open. |
| `RootTerminalAnswerAuthority` and `TerminalAnswerAuthority` | `lowering/mod.rs` | Root lowering mints authority only after the terminal answer preconditions hold. | `emit_result` consumes the authority while emitting the root result; the private zero-sized token cannot be forged outside its owner. |
| checked recursive-IH marker lifecycle | `lowering/mod.rs` | `mint_checked_computational_ih_instance` creates the checked instance and exact marker after plan/slot/parent authority exists. | `consume_checked_ih_marker_at_static_worker_call`, `consume_checked_recursive_invocation_call`, and `finish_checked_computational_ih_marker` require one matching consumption and reject residue. |
| checked subcontinuation frame lifecycle | `lowering/mod.rs` and `lowering/core.rs` | The selected continuation installs the exact checked frame and records its outstanding identity. | `consume_checked_subcontinuation_frame` settles it; the enclosing checked-frame closeout rejects an unconsumed or foreign frame. |
| `NativeRunEvidence` and `NativeArtifactIdentity` | `surface.rs` and `artifact/api.rs` | The public artifact/run API records validation facts and artifact identity only after the native operation completes. | Callers compare or publish the immutable report; this is evidence publication rather than a mutable consume-once ledger. |

## Boundary of the classification

The table is exhaustive over candidate authority-bearing declarations selected
by the two patterns and the source-read plain-name lifecycles listed above. It
does not claim that every `Id`, `Identity`, or `Owner` is independently minted
or consumed: many are immutable coordinates inside one of these larger
lifecycles. Test-only mutation and observation ledgers are inventoried with the
test property surface rather than promoted into production authorities.
