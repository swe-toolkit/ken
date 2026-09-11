---
id: ABI-S6
title: "ordinary anonymous and file-backed mappings as opaque runtime-owned regions and bounded byte views"
status: active
owner: runtime
size: L
gate: none
tier: T1
depends_on: [ABI-S1]
blocks: []
github: null
origin: "docs/program/10-linux-abi-completion.md §4 Track S (the ABI-completion program), row ABI-S6. Node filed by the Steward 2026-07-25; framed and released 2026-09-09 on the operator's standing 'keep L1 on ABI/compiler work' direction after ABI-S1 (descriptor completion) merged. runtime-leader named ABI-S6 as the next ABI-B entry (evt_6sd89wq68prby): the explicit ABI-S1 successor, now unblocked, and the opaque-region + bounded-byte-view substrate that later MMIO builds on — with the steer to frame the lifetime/bounds/refusal boundary rather than assume an API shape."
---

# D5b NATIVE HARD-STOP 10 — OBSERVATION-ONLY, AMENDED IN PLACE 2026-09-11 (Steward). READ FIRST.

> # D5b native-lowering track, HARD STOP 10 — OBSERVATION-ONLY (no repair yet).
> # Architect classification + observation ruling evt_50ynvegxvydvm
> # (thr_7wy5wy45p7abm), grounded byte-clean fee133142 (full attempt 46bf387e…
> # applied cleanly; HS9 repair-only diff a29bca8b…; decisive log 1d8f7aa7…). The
> # HS9 root-parent repair is VALIDATED: `RejectExactRoot` applied once + restored
> # the HS9 refusal, and restored `Exact` alone advanced BEYOND HS9 to this new
> # ObjectEmission refusal — "a forward-declared response owner has no verified
> # selected incoming call: owner=StaticResponseOwnerId(0),
> # context=ContinuationContextId(1), preexisting=true, …,
> # disposition=Some(InlineNoCall)". HS10 is NOT evidence against HS9.
> #
> # CLASSIFICATION: exactly HS8's already-defined diagnostic FORK 2 — response-owner
> # classification and authoritative inline realization DISAGREE about whether one
> # complete caller identity is a call. A planning/lowering realization-domain
> # contradiction; NOT a missing response call inferred from absence, NOT a reason
> # to weaken closeout. The join is EXACT: owner StaticResponseOwnerId(0) names the
> # SAME complete identity as HS7 (producer/emission owner Predeclared(6), result
> # 832, construct 825, alternative 1, sequence 0, target Specialization(3),
> # recursive position 1, worker parent 12, closure 819, body 813, arity 1, lexical
> # captures 818–814). The prior HS8 stream already records it selected_caller=true,
> # transport_source=false, required_consumer=false, no direct/composed/transport
> # emission, disposition=Some(InlineNoCall); HS7 established the bridge settles this
> # identity once at successful bridge exit.
> #
> # THREE production mechanisms explain the refusal (no speculation): (1)
> # `responses.rs::static_response_phase_b_split` defers only a transport-source
> # residual — for this non-transport demand the 2405-2410 condition is false and
> # `specialized.push(demand)` (2424-2430) is unconditional; `static_response_owner_
> # specializations` creates an owner on ordinary-call population membership, NOT
> # realized-call disposition. (2) `core.rs`'s immediate-binder bridge settles every
> # still-unconsumed member `InlineNoCall` (7076-7088) after successful inline
> # completion — the ledger's positive authority, which `ContinuationCandidateLedger
> # ::close` deliberately excludes from DirectCall∪ComposedCall. (3)
> # `ContinuationClaimLedger::validate_response_owner_call_coverage`
> # CORRECTLY refuses because no verified response-owner call exists for a
> # forward-declared Specialized owner — THAT GUARD MUST REMAIN (tolerating an
> # unused owner would suppress ordinary host-effect lowering with no caller to
> # execute the owner).
> #
> # FENCES ON INTERPRETATION: do NOT reinterpret `InlineNoCall` as discharge, force
> # or fabricate a claim, remove this identity from candidate totality, accept an
> # unused Specialized owner, weaken `validate_response_owner_call_coverage`, or make
> # response-owner selection override the successful bridge. Preserve HS7's exact
> # settlement and the HS8/HS9 routes.
> #
> # NEXT STEP — ONE OBSERVATION-ONLY PASS (do NOT repair yet). On the exact
> # reapplied attempt + HS8/HS9 repairs, add a cfg test-support, streamed,
> # explicitly flushed observer with five structurally joined row kinds: (1) at
> # `static_response_phase_b_split`, one row per demand (response/Vis/op-root/
> # effect/producer origins, complete k_identity, transport_source, producer-group
> # ownership flags, requires_execute_then_resume, has_unitless_response, final
> # Specialized/Deferred branch); (2) at immediate-binder bridge selection + exit,
> # one row per bypassed identity (defining unit/emission owner, construct +
> # computational frame origin, alternative, recursive position, selected field,
> # consumer kind, checked_ih_slots_operation_bridge, both bridge predicates,
> # completion, disposition before/after, direct/pending-composed/verified-composed/
> # transport evidence); (3) at each generated Function's finished-CLIF verification
> # boundary, row the identity ONLY if it is a response selected caller
> # (defn/emission owner, direct + checked-IH transport + verified-composed emission
> # counts, resolved target, selected response-owner target, whether
> # verified_response_owner_calls records it — counts accompany the full identity,
> # never pair rows); (4) at artifact close, the complete response-owner matrix
> # joined by owner.selected_caller() (owner id, response row, context/preexisting,
> # full identity, final disposition, exact response-owner-call evidence — report
> # EVERY owner whose selected caller is not DirectCall or has no verified physical
> # response-owner call; do not stop after owner 0); (5) for owner 0's exact
> # op-root/effect only, observe whether ordinary lowering produced
> # StaticResponseDeferred outside an owner and whether any actual host-effect seat
> # was emitted in that same Function (observation, NOT permission to carry/decode
> # the marker). ALSO attempt an observer-only PRE-phase-B re-derivation of the
> # bridge-bypassed complete-identity set from existing source/plan records — it
> # must reproduce the bridge's full selector (construct, computational frame,
> # alternative, recursive position) and its exact structural predicate, with NO
> # origin arithmetic, order, counts, nearest match, or candidate disposition; if
> # the fact cannot be derived without lowering-local eliminator state, REPORT THAT
> # as the result rather than adding a plan field or duplicating a heuristic.
> #
> # VALID RUN: the one unchanged COW positive ending at the SAME HS10 production
> # refusal, with exactly one complete owner-0 identity joining the phase-B
> # Specialized row to a successful bridge InlineNoCall row and zero physical
> # response-owner calls. Preserve all rows, exact census, observation-only diff,
> # full run, and SHA-256; restore byte-clean fee133142 and return.
> #
> # THE NEXT RULING BRANCHES ON the joined result: (a) same identity + successful
> # bridge + no physical owner call + deferred op/effect = response-plan
> # OVERPROMOTION (next repair classifies that exact structural no-call BEFORE
> # owner creation, leaving the row on an existing ordinary/deferred effect path);
> # (b) finished CLIF contains the exact response-owner call but the global map
> # lacks it = evidence-collection/recording loss (repair the collector, not
> # classification); (c) different identities at phase B vs bridge = a structural
> # join defect; (d) no successful bridge or no ordinary effect realization = a
> # deeper lifecycle/loss defect (demotion NOT authorized); (e) failure to derive
> # the bridge class pre-phase-B is itself a HARD DESIGN BOUNDARY — stop rather
> # than infer from later InlineNoCall or build a speculative owner.
> #
> # STEWARD PACKAGING: observation-only, cfg test-support, the coverage guard and
> # HS7/HS8/HS9 routes all preserved => NO TCB delta, NO ban-lift => amend-in-place,
> # no operator sign-off (as at HS8's observation passes). Count remains 10; a new
> # refusal is HARD STOP 11 (no Research until 12). Runtime stays byte-clean at
> # fee133142 until this amendment lands and runtime-leader explicitly re-kicks the
> # observation pass. The Architect symptom-inventory record for HS10 follows
> # immediately below.

# D5b HARD-STOP 10 — RESPONSE-CALLER REALIZATION MISMATCH

Architect inventory, 2026-09-11.

> Runtime's exact HS9 causality run advanced beyond HS9 and reached a new
> ObjectEmission refusal at artifact closeout:
>
> `a forward-declared response owner has no verified selected incoming call:
> owner=StaticResponseOwnerId(0), context=ContinuationContextId(1),
> preexisting=true, ... disposition=Some(InlineNoCall)`.
>
> This is consecutive HARD STOP 10 on the same D5b checked-IH/native chain.
> Exact report: `evt_5w8e94hcn0gjw`; decisive log SHA-256
> `1d8f7aa75a86edd35a26becb9e99cef14f0b9d6c68587c20e3e0917dc2bfefe8`;
> HS9 repair-only diff SHA-256
> `a29bca8bb32ed59f5e80c4e7251fa89b776f56bf66529dfa85b5499e047ef4ad`;
> full attempt SHA-256
> `46bf387ec4f52da29a6c6da01061a774220f88ca5a6c8e018a950fe94c3eb82e`.
> `RejectExactRoot` applied exactly once and restored the exact HS9 refusal;
> restored `Exact` alone reached HS10. The unit control, feature check, and
> feature staticlib build passed. Runtime restored byte-clean `fee133142`; the
> lockfile is unchanged. This is not evidence against HS9.
>
> The refused caller is the complete HS7 identity: producer/emission owner
> `Predeclared(6)`, result origin 832, construct origin 825, alternative 1,
> sequence 0, target `Specialization(3)`, recursive position 1, worker parent 12,
> closure 819, body 813, arity 1, and five lexical captures 818 through 814.
> Prior observation already proved this exact identity settles once as
> `InlineNoCall`, with no direct emission, pending or verified composed call,
> checked-IH transport source/emission, or required consumer. The response plan
> nevertheless selects it as owner 0's incoming caller.
>
> CLASSIFICATION HOLD: this is the earlier HS8 diagnostic fork 2 — response-owner
> classification and authoritative inline realization disagree about whether an
> incoming call exists. It is not a missing call inferred from absence: the
> candidate ledger positively records `InlineNoCall`. The response-call coverage
> gate is correct to refuse because a Specialized response suppresses ordinary
> host-effect lowering outside its owner. Do not weaken that gate, reinterpret
> `InlineNoCall` as discharge, force a claim, or tolerate an unused Specialized
> owner. No Research pull is due until hard stop 12.

# D5b NATIVE HARD-STOP 9 — ROOT-PARENT REPAIR RULING, AMENDED IN PLACE 2026-09-11 (Steward). Superseded by the HS10 observation ruling above; retained as the root-parent repair record that advanced the witness to HS10.

> # D5b native-lowering track, HARD STOP 9 — ROOT-PARENT REPAIR. Architect ruling
> # evt_3c1nve6zjmsza (thr_7wy5wy45p7abm), grounded byte-clean fee133142 (all four
> # HS9 hashes rechecked, the exact ae422b30… attempt applied). The HS8 fork-3
> # ingress repair is validated (it advanced past HS8); the positive then reached
> # the new ObjectEmission refusal "OrientedSubcontinuationPlanV1: an external
> # source parent is not a non-root checked invocation" — consecutive HARD STOP 9.
> #
> # §1a MANDATORY RESEARCH (third pull; prior HS3 evt_1w0grnt5xrseq, HS6
> # evt_1r9vwnfpr7caq): advisory evt_7b67s21nrf1ds. Its core distinction holds — an
> # external parent is an explicit root/delimiter OR a concrete non-root call;
> # missing call provenance alone NEVER manufactures root ("source-less" = no
> # checked-call source, never no control authority). One advisory anchor
> # corrected: the root normalizer is `make_computational_recursor` (not
> # `instantiate_checked_invocation_segment`); the architectural conclusion is
> # unchanged. §1b ENTRIES 7–9: NO single structural predicate joins all three —
> # "a downstream consumer is too narrow" is too general to name one replaceable
> # mechanism. Entry 7 = a post-production consumer re-demanding a call after
> # authoritative `InlineNoCall` settlement; entry 8 = a pre-lifecycle ingress that
> # never entered its exact transport route; entry 9 = parent-kind admission after
> # source/mint/edge validation. Settlement, ingress, and parent role are distinct
> # authorities. Count remains 9; Research next re-triggers at 12.
> #
> # CLASSIFICATION: a ONE-CONSUMER DOMAIN MISMATCH over an EXISTING canonical root
> # class — NOT duplicate dispatcher entry, NOT lost non-root provenance, NOT a
> # missing representation. The exact attempt proves: (1) `external_source_parent:
> # Some` has one production origin (the source machine, after minting + exact
> # call-parent frame/site comparison + `validate_source_dynamic_splice_parent`);
> # all three non-source composer callers pass `None`, so outer `Some` is positive
> # current source-open provenance. (2) Reaching the HS9 string means
> # `parent.nonroot_invocation()? == Ok(None)` and `parent.id == Some(frame)` — the
> # helper admitted exactly `(Some(frame), None, None, 0)` or `(Some(frame),
> # Some(0), None, 0)`; partial tuples already refuse. (3) `make_computational_
> # recursor` maps those roots to frame-retained / invocation+source-absent /
> # depth-zero; non-roots retain the complete tuple. (4) Minting selects
> # distinguished parent `0`, allocates a fresh child+edge, inserts once, puts the
> # edge handle on the child; edge take removes the exact handle before
> # composition (replay/sibling theft refuse). (5) The composer's LATER tree
> # already admits external parent invocation `0` keyed with its frame — only the
> # EARLIER external-parent gate rejects that same root by requiring `Some(nonroot)`.
> # (6) HS8 is single-entry structurally (`lower_expr` returns before ordinary
> # occurrence entry; the transport-positive `InvocationReturn` dispatcher enters
> # once and does not fall back). (7) The source machine binds checked root
> # authority to `active.cursor` and restores it only through that cursor before
> # resume; composition consumes the child edge, not root authority.
> #
> # RULED REPAIR — change ONLY the early `external_source_parent_key`
> # classification in `compose_oriented_subcontinuation`. Reuse the existing
> # two-level sum; add NO enum, carrier, plan field, token, edge kind, or inference:
> #   let frame_id = parent.id.ok_or_else(|| {
> #       unsupported("OrientedSubcontinuationPlanV1",
> #           "an external source parent has no checked frame identity")
> #   })?;
> #   let invocation_id = match parent.nonroot_invocation()? {
> #       Some((invocation_id, _, _)) => invocation_id,
> #       None => 0,
> #   };
> #   let key = (invocation_id, frame_id);
> # Keep exact-one incoming-edge matching unchanged. An explicit canonical root is
> # key `(0, frame)`; non-root behavior is BYTE-IDENTICAL. Outer `None` stays
> # distinct from `Some(canonical root)`; never derive root from absent metadata
> # outside that outer `Some`. Keep the later id-0 tree law, child order,
> # frame/site/interface checks, edge removal, affine ledgers, installation, and
> # source return byte-identical. Do NOT alter `nonroot_invocation`,
> # `make_computational_recursor`, minting, source-parent validation, edge take,
> # `external_children`, root authority, cursor restoration, or the HS8 ingress.
> #
> # CAUSALITY CONTROL — cfg(any(test, feature = "px8-ds-test-support")): RAII
> # `D5bHs9ExternalRootMutation::{Exact, RejectExactRoot}` +
> # `with_d5b_hs9_external_root_mutation<T>(...) -> (T, usize)`. It may apply ONLY
> # after outer `Some`, `nonroot_invocation() == Ok(None)`, frame present, and
> # exactly one matching `(0, frame)` edge. `RejectExactRoot` increments once and
> # returns the exact HS9 refusal; it must NOT fire for no parent, non-root,
> # partial tuple, frame mismatch, nonzero edge parent, or duplicate edge. Extend
> # `oriented_external_source_parent_requires_the_exact_invocation_frame_pair`:
> # both frame-bearing root spellings with exact edge parent `0` install the
> # child-only segment; the existing non-root row stays green; root-frame mismatch,
> # nonzero edge parent, partial root tuple, and two matching root edges refuse at
> # their named gates; child qualification stays the child's nonzero invocation,
> # never `0`. On the unchanged COW witness with the exact attempt, `RejectExactRoot`
> # must apply exactly once and reproduce HS9; after RAII restoration, `Exact` must
> # advance. Then run the full battery (non-root external-parent; edge
> # deletion/duplication/replay/sibling/cycle; root authority affine/cursor;
> # frame-sequence + source-return; HS3–HS8 causality incl. HS6 inside-inner
> # overflow + HS8 bypass; scoped checks, staticlib, COW, ELF census). If it
> # completes, require native/interpreter value, ordered effect trace, file
> # preservation, and unchanged-stack parity.
> #
> # FENCES: no partial-parent acceptance; no id/source/depth inference; no
> # root-from-absence outside outer `Some`; no identity/direction/replay change; no
> # root consumed as child; no dispatcher re-entry; no carrier, continuation, plan,
> # owner, response, settlement, join, ABI/wire, stack, source-semantic,
> # generated-code, or TCB change.
> #
> # DECISIVE TURN. This closes the diagnosed D5b native-lowering chain IFF the
> # repaired positive advances past HS9 with no new stop. A NEW structural refusal
> # is HARD STOP 10: stop, preserve/revert, return it — no Research until 12.
> # STEWARD PACKAGING: fully fenced (no TCB delta, no ban-lift) => Steward
> # amend-in-place, no operator sign-off (as at HS3–HS8). Runtime stays byte-clean
> # at fee133142 until this amendment lands and runtime-leader explicitly re-kicks
> # the repair build.

# D5b NATIVE HARD-STOP 8 — FORK-3 REPAIR RULING, AMENDED IN PLACE 2026-09-11 (Steward). Superseded by the HS9 root-parent repair ruling above; retained as the fork-3 ingress-repair record that advanced the witness to HS9.

> # D5b native-lowering track, HARD STOP 8 — FORK-3 PRODUCTION REPAIR (the
> # observation loop is DISCHARGED; a real repair is now authorized). Architect
> # fork-3 production repair ruling evt_3pq668qnpt0p2 (thr_7wy5wy45p7abm), grounded
> # at byte-clean fee133142. The valid third observation is independently verified
> # (witness e32f9561…; trace 47215748…; census 6acabeac…; observation 9f474ad2…;
> # facts fd91ae09…). Runtime restored byte-clean fee133142; `Cargo.lock`
> # unchanged.
> #
> # CLASSIFICATION — FORK 3: an ordinary-Construct ingress bypass of the existing
> # transport-aware producer lifecycle. The exact result-bound route is
> # `TransferRepresentedBoundaryValue` on source `Constructor`/Vis origin 749; its
> # position-0 specialized child is structurally joined through aggregate
> # occurrence 182, producer `Source(749)`, to operation-root origin 748 and
> # exactly response row 4. Current emission owner `Specialization(1)`; the
> # selected caller's target is `Specialization(2)` and exact response owner is 4.
> # At refusal the leaf is selected and a checked-IH transport source, but
> # disposition is `None` and transport emissions are zero. The direct
> # represented-boundary guard is therefore CORRECT: the producer reached a carrier
> # join before the call lifecycle that consumes its transport could run. Fix the
> # INGRESS, not the join or the boundary.
> #
> # No new planner relation is needed. `checked_ih_environment_transport_at(
> # destination_owner, destination_construct_origin)` is the exact unique lookup.
> # `lower_computational_producer_expr_once` already treats `InvocationReturn` plus
> # that per-producer transport as the exception to ordinary lowering, calls
> # `call_checked_ih_environment_transport`, and replaces the constructor by the
> # returned value; `resolved_continuation_call_target` retargets the complete
> # caller to owner 4 and `call_declared_unit_target` writes the wordless input as
> # the exact inert zero slot the owner never loads. The missing step is REACHING
> # this path.
> #
> # REPAIR (`lowering/core.rs::lower_expr`). Immediately after destructuring
> # `SourceOccurrence`, and strictly BEFORE `enter_source_occurrence_plan(
> # static_origin)`, detect only this existing structural fact:
> #   let transport_destination = matches!(expr, RuntimeExpr::Construct { .. })
> #       && match self.defining_emission_owner {
> #           Some(owner) => self.static_transition_plan
> #               .checked_ih_environment_transport_at(owner, static_origin)?
> #               .is_some(),
> #           None => false,
> #       };
> #   if transport_destination {
> #       return self.lower_computational_producer_expr(builder,
> #           SourceOccurrence { expr, static_origin }, env,
> #           &[EliminatorFrame::InvocationReturn]);
> #   }
> #   self.enter_source_occurrence_plan(static_origin)?;
> # The placement before `enter_source_occurrence_plan` is MANDATORY: the producer
> # dispatcher enters the same source occurrence itself, so redirecting after the
> # ordinary entry would double-enter one occurrence and replace a lifecycle fix
> # with a second authority. The selector is exactly the `(current emission owner,
> # destination Construct origin)` transport lookup. Do NOT select on
> # `StaticResponseDeferred`, constructor name, HostOp, response-row/owner ordinal,
> # adjacency, counts, `D9OperandIdentity`, SSA equality, or frame values. Do NOT
> # make it response-only — every exact checked-IH transport destination arriving
> # at ordinary Construct ingress is already assigned to the transport-aware
> # producer path; response row 4 is merely the witness that exposed the generic
> # bypass. The producer dispatcher stays the SOLE consumer: do not pass a token,
> # call the transport from `lower_expr`, copy its assembly, or settle at the
> # redirect. Non-Construct, no-owner, and lookup-`None` fall through unchanged;
> # multiplicity remains the existing accessor's refusal.
> #
> # CAUSALITY CONTROL — under `cfg(any(test, feature = "px8-ds-test-support"))`:
> #   pub enum D5bHs8TransportIngressMutation { Exact, BypassExactTransport }
> #   pub fn with_d5b_hs8_transport_ingress_mutation<T>(
> #       mutation: D5bHs8TransportIngressMutation, body: impl FnOnce() -> T,
> #   ) -> (T, usize);
> # RAII restores `Exact`. `BypassExactTransport` may act only AFTER the exact
> # per-owner/per-Construct lookup returned `Some`; it increments its application
> # count and takes the old ordinary-Construct fallthrough without changing the
> # plan or value. On the unchanged COW witness: exactly one application and
> # restoration of the exact HS8 `StaticResponseDeferred` boundary refusal; then
> # restore `Exact` and the positive must advance beyond HS8. Also extend
> # `invocation_return_transport_selection_is_per_producer_in_production` with an
> # independent ordinary-ingress arm (fresh bare compiler: `lower_expr` on the
> # fixture's exact transport destination reaches the same transport-aware producer
> # decision and stops only at the deliberately absent function-local force target;
> # retain the same-owner transport-free producer row and both
> # `has_transport`/`!has_transport` observations) — proving the ingress uses the
> # existing per-producer discriminator, not a plan-wide transport-presence test.
> # After advancement, existing ledgers (not a new counter) must establish that row
> # 4 emits ONE transport call to its resolved response owner, records that owner
> # call, and receives one final disposition without generically transferring root
> # 748. Then run the existing response-retarget / execute-then-resume / transport /
> # disposition / call-slot controls, the owed HS3–HS7 mutations, the HS6 overflow
> # control, scoped Runtime checks, staticlib, unchanged-stack COW native/
> # interpreter/parity, and ELF census. STOP IMMEDIATELY ON HS9.
> #
> # FENCES: no change to `carried_join_arm`, join planning/representation,
> # `transfer_represented_boundary_value`, either boundary admissibility walk,
> # aggregate allocation, placeholder production/classification,
> # `checked_ih_environment_transport_at`, transport construction, call operand
> # assembly, caller retargeting, response owner selection/body, candidate
> # settlement sites/order, claim/discharge equality, required-consumer
> # realization, either inert call-slot case, carriers, ABI/wire/runtime tags,
> # generated native representation, source semantics, stack behavior, or TCB. The
> # ONLY production change: an exact transport-destination Construct can no longer
> # bypass the already-landed transport-aware `InvocationReturn` producer
> # dispatcher when it enters through ordinary `lower_expr`.
> #
> # DECISIVE TURN. This closes the diagnosed HS8 mechanism IFF the positive
> # advances. If the repaired witness reaches a NEW structural refusal, that is
> # HARD STOP 9: preserve/revert and report it; the Architect then holds before
> # ruling and triggers mandatory §1a Research. Stop count stays 8 and no Research
> # fires now. STEWARD PACKAGING: the repair grows NO TCB (fully fenced above) and
> # needs NO ban-lift => Steward amend-in-place, no operator sign-off (as at
> # HS3–HS7). Runtime stays byte-clean at fee133142 until this amendment lands and
> # runtime-leader explicitly re-kicks the repair build.

# D5b NATIVE HARD-STOP 8 — OBSERVATION-ONLY (THIRD PASS), AMENDED IN PLACE 2026-09-11 (Steward). Superseded by the fork-3 repair ruling above; retained as the diagnostic history that located the ingress.

> # D5b native-lowering track, HARD STOP 8 — OBSERVATION-ONLY (no repair yet).
> # Architect classification + observation ruling evt_14wxhxwwf403y
> # (thr_7wy5wy45p7abm), grounded at byte-clean fee133142. Nine-path attempt diff
> # SHA-256 e83f1a20…; run log SHA-256 12b9563c…. The HS7 exact-`InlineNoCall`
> # repair AND its causal control are ACCEPTED as far as this stop
> # (`IgnoreInlineNoCall` applied once and restored the HS7 non-constructor
> # refusal; after RAII restoration the same witness advanced PAST HS7). HS8 is a
> # NEW, later stop, NOT evidence against the HS7 repair.
> #
> # CLASSIFICATION: a CORRECT FAIL-CLOSED refusal over an UNCLASSIFIED
> # compiler-control escape. It is NOT evidence that `StaticResponseDeferred`
> # needs a boundary representation, and does NOT authorize weakening its
> # exact-owner law. At fee133142, `Lowered::boundary_transfer_admissibility`
> # refuses `StaticResponseDeferred` before generic carrier allocation — the
> # correct side of the boundary. The only lawful crossings are the two exact
> # call-slot cases (`lowering/mod.rs::carry_call_input` +
> # `lowering/calls.rs::call_declared_unit_target` write an inert zero only
> # because the selected response owner never loads parameter zero);
> # `lowering/units.rs::define_static_response_owner_bodies` sets
> # `static_response_owner`, reconstructs the effect from the validated
> # static-response row, materializes the current `HostResult`, and calls the
> # row's exact K context. A generic carrier would publish compiler control as a
> # runtime value — forbidden. The present diagnostic CANNOT decide why the guard
> # was reached (bypass vs stale classification): it carries no placeholder
> # production origin, enclosing lowered-value path, boundary screen site,
> # defining emission owner, response-owner id, full selected
> # `ContinuationCallIdentity`, or final disposition. Inferring any from
> # adjacency, counts, origin arithmetic, `D9OperandIdentity`, SSA equality, or
> # frame values is FORBIDDEN. NO production repair is authorized yet.
> #
> # OBSERVATION HISTORY — TWO topology corrections, still HS8 (count firmly 8, no
> # entry 9, no Research). PASS 1 (Architect evt_1825eynb0dg24): the observer's
> # row-UNIQUENESS premise was too strong — it halted on a response-row
> # multiplicity at effect origin 190 before reaching the production refusal.
> # Effect origin is a shared response-CLASS coordinate
> # (`responses.rs::response_disposition_at_effect` classifies existentially with
> # `.any(...)`; `static_response_owner_specializations` requires uniqueness of the
> # complete selected `ContinuationCallIdentity`, not `effect_origin`); the row
> # coordinate is row id + complete caller identity. PASS 2 (Architect
> # evt_4hzvfhvxgb2hs): corrected to candidate sets, the run genuinely reached the
> # production refusal (16 producer headers, 19 complete candidates, no absent
> # lookup) but emitted NO refusal row — the observer hooked only 2 of the 8 direct
> # `boundary_transfer_admissibility` ingress sites. Both were DIAGNOSTIC-topology
> # defects, never a new production refusal.
> #
> # THE EIGHT DIRECT INGRESS SITES (exact fee133142; grep-confirmed complete
> # population outside boundary.rs' own definition/recursive arms/tests):
> # `core.rs::transfer_constructor_operands` + `mod.rs::transfer_into_carrier`
> # (covered by pass 2), plus six in `lowering/aggregates.rs` blob
> # 8d117792edd3dba424e033a6e001209fb014b2fa at lines 1060/1086/1143/1179/3983/4113
> # — represented-boundary leaf fallback, bind-continuation unrecognized-closure
> # fallback, bind-continuation leaf fallback, the retired-flat-order control,
> # checked-IH captured-environment child preflight, and boundary-closure-
> # environment child preflight. The pass-2 diff never touched aggregates.rs, so a
> # refusal originating at an aggregate-specific preflight could not be observed.
> #
> # OBSERVATION PROTOCOL — THIRD PASS (supersedes the pass-2 protocol; Steward
> # re-kicks Runtime for THIS bounded pass only). Reapply the byte-identical
> # nine-path attempt (e83f1a20…) AND the final corrected producer/candidate
> # observer (9b545637…) to clean fee133142. Run only the same unchanged positive
> # COW witness under `px8-ds-test-support`, `--nocapture --test-threads=1`.
> #
> # Add ONE test-support-only helper that returns `()` and NEVER replaces the
> # result it observes:
> #   fn d5b_hs8_observe_admissibility_result(&self, layer: &'static str,
> #       site: &'static str, boundary_root_origin: Option<StaticOriginId>,
> #       parent: Option<String>, value: &Lowered,
> #       result: &Result<(), CraneliftBackendError>)
> # On `Ok` it may return silently. On `Err` it emits+flushes one
> # `HS8-ADMISSIBILITY` header (sequence, `layer={DirectGuard,EnclosingRoute}`,
> # exact site, root origin, root lowered variant, parent identity, full error,
> # current defining/response-owner fields), then runs the EXISTING structural
> # path collector on the exact `value` and emits the existing row-id-ordered
> # `HS8-REFUSE-CANDIDATE` records for every path (a header even if no path; on a
> # collector error, stream `path=INVALID` + the error and PRESERVE the original
> # result — observation never masks or substitutes the production error).
> #
> # At every direct production call OUTSIDE boundary.rs, bind the result before
> # `?`/`return`, pass that same local by reference to the helper, then propagate
> # unchanged. Use exactly these EIGHT direct-site labels (the complete grep
> # population): `RepresentedBoundaryLeaf`, `BindContinuationUnrecognizedClosure`,
> # `BindContinuationLeaf`, `RetiredFlatOrder`, `CheckedIhCapturedEnvironmentChild`,
> # `BoundaryClosureEnvironmentChild`, `ConstructorPreAllocationChild`,
> # `TransferIntoCarrier`. Do NOT call admissibility twice; preserve every guard +
> # order. For the two synthesized-environment child preflights, destructure the
> # existing `SynthesizedArgument::WorkerCaptureOperand { seat, ordinal, origin,
> # value }` and record the exact parent (owner, seat, aggregate record id, capture
> # ordinal, child origin) — never from vector proximity or counts.
> #
> # ALSO bind + observe the enclosing returned result with `layer=EnclosingRoute`
> # at the three origin-bearing represented routes:
> # `transfer_bind_continuation_boundary_value` (its `origin`),
> # `transfer_represented_boundary_value` (its `origin`), and
> # `transfer_constructor_operands`' represented-child branch (planner-derived child
> # origin + exact parent constructor/argument position). These supply the
> # structural root-to-leaf origin when an internal leaf fallback has `origin=None`;
> # they observe the same returned `Result` and add/suppress/retry/reinterpret
> # nothing.
> #
> # VALID only if the run ends at the original production message ("a deferred host
> # response is compiler control and can only enter its exact response owner") AND
> # has at least one `HS8-ADMISSIBILITY` error row whose `result` contains that
> # exact `StaticResponseDeferred` refusal. Report ALL error rows. A direct row
> # identifies the exact guard ingress; its enclosing-route row (or synthesized-
> # environment parent row) must structurally recover each deferred leaf and JOIN
> # it to the complete response candidate set at refusal time. If an internal
> # fallback has `origin=None`, do NOT fill it from the nearest producer — use only
> # the enclosing origin-bearing route. Multiple candidates remain an AMBIGUITY,
> # all reported; no first/last/nearest selection.
> #
> # THE FIVE FORKS the result-bound refused leaf selects among (diagnostic
> # outcomes, NOT pre-authorized repairs): (1) `DirectCall` caller with a
> # different/absent actual owner = an existing call-funnel/retarget bypass;
> # (2) exact `InlineNoCall` caller = response-owner classification and inline
> # realization disagree about whether a call exists; (3) `None`/unsettled =
> # lifecycle/order defect, no owner exemption may be inferred; (4) a leaf whose
> # exact origin is `Deferred` or has no response row = the placeholder
> # producer/classifier is wrong; (5) a placeholder produced while already inside
> # its exact response owner = the producer's owner guard is wrong.
> #
> # FENCES: no active-call stack, TLS context, backtrace, `track_caller`, new
> # planner/row field, carrier, tag, owner selector, or production accessor — the
> # helper observes the exact local `Result` already returned by each call. No
> # change to either placeholder producer, response classification,
> # `boundary_transfer_admissibility`, represented/bind semantics, aggregate
> # allocation, owner selection/body, caller retargeting, continuation settlement,
> # required-consumer realization, the inert call-slot cases, ABI/wire/runtime
> # tags, generated native code, source semantics, stack behavior, or TCB. No
> # mutations or remaining D5b controls. Preserve the complete streamed log, the
> # exact call-site census, and the observation-only diff with SHA-256, then
> # restore the attempt + every observation edit byte-clean at fee133142;
> # `Cargo.lock` unchanged. Observation-only — NO TCB delta, NO ban-lift => Steward
> # amend-in-place, no operator sign-off (as at HS2–HS7).
> #
> # Durable: Architect inventory entry 8 committed d8879411 (issue blob 2984805f),
> # recorded below; the pass-1/2/3 corrections add NO entry 9 (Architect rulings).
> # Stop count is FIRMLY 8; no §1a Research trigger — a real stop 9 is not reached.
> # Runtime STAYS byte-clean at fee133142 until the Steward's explicit third
> # observation-only re-kick; the result-bound refused leaf + final candidate/
> # evidence vector return to the Architect for the HS8 production ruling.

# D5b NATIVE HARD-STOP 7 — CLOSED, AMENDED IN PLACE 2026-09-11 (Steward).

> # D5b native-lowering track, HARD STOP 7 — CLOSED. Architect corrected
> # classification + repair ruling evt_62ec7mwa8pgj2 (thr_7wy5wy45p7abm), on the
> # fork-4 observation (runtime-implementer evt_28xfh93fkh8hk, complete 45-row
> # stream, hashes recorded in-thread) at byte-clean c002f2d47. After the HS6
> # frame isolation removed the physical overflow, the FIRST unmutated native COW
> # left a planner-projected causal edge residual at the detached-result seat
> # while that unit's lowered result was not the specialized producer constructor
> # the five-guard contract requires. That is stop 7.
> #
> # CLASSIFICATION: a ONE-LAYER CONSUMER MISMATCH; the native route is LAWFUL (no
> # operator-surface unlawful-route fork). The Architect's earlier four-fork list
> # OMITTED the already-landed non-discharge disposition
> # `CandidateDisposition::InlineNoCall`. At c002f2d47 the deferred bridge in
> # `lower_computational_producer_construct` resolves the same full identity
> # (construct 825, frame 12, alternative 1, position 1 => the reported
> # `ContinuationCallIdentity`) and, after inline completion, settles it
> # `InlineNoCall` at `BridgeExit`. That disposition is a LAWFUL no-call, NOT a
> # discharge; the ledger closeout already excludes it from the
> # `DirectCall ∪ ComposedCall` call-obligation subset. The earlier per-function
> # residual filter in `eliminate_detached_producer_continuation` reads only
> # direct/composed feeds, so it re-demands a producer constructor for an identity
> # whose authoritative disposition says no call was owed. The disposition fix
> # landed at HS6; this earlier consumer was never reconciled. The five reported
> # facts are correct — the carried unit is NOT discharge evidence and the
> # non-constructor guard must NOT be weakened — only the fork-4 "constructor
> # genuinely required" label was wrong.
> #
> # REPAIR (Architect-ruled — teach ONLY the detached residual classifier about
> # the existing exact `InlineNoCall`): reapply the preserved frame-pop attempt
> # (ccbf428a) to byte-clean c002f2d47. Add ONE read-only accessor
> # `disposition(&self, identity: &ContinuationCallIdentity) ->
> # Option<CandidateDisposition>` beside `ContinuationCandidateLedger::is_settled`
> # in `lowering/units.rs` (returns `self.settled.get(identity).copied()`). In
> # `lowering/core.rs::eliminate_detached_producer_continuation`, inside the
> # existing `result_edges.iter().filter(...)`, derive `inline_no_call` from
> # `continuation_candidates.disposition(&edge.identity) ==
> # Some(CandidateDisposition::InlineNoCall)` and add ONLY `&& !inline_no_call`
> # to the residual predicate. `None` (no ledger, unknown, or unsettled identity)
> # stays residual and fail-closed. `DirectCall`, `ComposedCall`, and
> # `TransportDormant` do NOT satisfy it. Do NOT use `is_settled` — clearing every
> # settled disposition would let a broken direct/composed feed bypass the five
> # guards. Do NOT add `InlineNoCall` to any discharge set or to claim equality.
> # An empty residual after the filter returns the already-lowered result through
> # the existing `[] => Ok(lowered)` arm. No constructor is fabricated and no
> # identity is inferred from target, origin, shape, value, count, or SSA equality.
> #
> # REQUIRED new causality control: under `cfg(any(test, feature =
> # "px8-ds-test-support"))`, add `D5bHs7DetachedDispositionMutation {Exact,
> # IgnoreInlineNoCall}` + a `with_d5b_hs7_detached_disposition_mutation(mutation,
> # body) -> (T, usize)` wrapper (RAII restores `Exact`). `IgnoreInlineNoCall` may
> # act ONLY when the full identity's disposition is exactly `InlineNoCall`;
> # increment an application count and treat that one edge as residual. On the
> # exact COW program: assert ONE application + restoration of the HS7
> # non-constructor refusal; restore `Exact`, then the unmutated positive must
> # complete. This proves the disposition filter — not an unrelated frame-pop or
> # lowering change — removes HS7.
> #
> # ALSO re-run (green required): `ced_d2_..._is_not_a_call_obligation`; all five
> # `ced_d3_m*` mutation rows + their cross-arm independence test;
> # `d5a_the_detached_result_seats_five_guards_are_each_reached_by_a_real_mutation`
> # (must still reach all five guards: multi-member, non-constructor, identity,
> # position, field-run); the still-owed HS6 inside-inner-dispatch overflow
> # control; every HS3–HS5 control from the prior ruling; the scoped Runtime
> # default/feature checks; staticlib materialization; the exact unchanged-stack
> # COW native/interpreter/parity assertions; and the candidate ELF prologue/call
> # census.
> #
> # FENCES: do NOT change the planner projection, `ContinuationResultEdge`,
> # candidate settlement sites, candidate totality/order, call-obligation
> # derivation, claim/discharge equality, response-owner coverage,
> # direct/composed/transport emission, any of the detached seat's five guards,
> # constructor formation, source return, the frame-pop mechanism, the HS3
> # partition, HS4 tuple transport, HS5 mint/compose authority,
> # `SourceContinuation`, `SourceMachineState`, carriers, ABI/wire/runtime tags,
> # stack settings, or the TCB. The ONLY production semantic change: an edge
> # already settled `InlineNoCall` by exact identity no longer stays in the
> # earlier constructor-required residual. NO TCB delta, NO ban-lift => Steward
> # amend-in-place packaging call, no operator sign-off (as at HS2–HS6).
> #
> # This remains STOP 7; no §1a Research trigger is due until stop 9 (Architect
> # owns the count). Durable classification: Architect inventory tip 41a87a3c
> # (symptom entry 7 + the disposition-mismatch paragraph recorded below), based
> # on landed HS6 30062dbdb. Runtime STAYS byte-clean at c002f2d47 until THIS
> # amendment LANDS; runtime-leader then explicitly re-kicks the repair.

# D5b NATIVE HARD-STOP 6 — CLOSED, AMENDED IN PLACE 2026-09-11 (Steward).

> # D5b native-lowering track, HARD STOP 6 — CLOSED. Architect classification +
> # repair ruling evt_700s4qs9bvp3c (thr_7wy5wy45p7abm), grounded on the
> # observation-only diagnostic (runtime-implementer evt_6x3kmb11454z2, 246-row
> # trace) at byte-clean 34d11b8053. The HS5 fork-1 repair PASSED compiler checks
> # + staticlib materialization, but the FIRST unmutated native COW execution
> # OVERFLOWED the test thread stack before an observable result. This was the
> # §1a-mandatory 6th stop: Research delivered the prior-art advisory
> # (evt_1r9vwnfpr7caq — prior art cannot classify without a runtime trace) and it
> # is now DISCHARGED by the trace.
> #
> # CLASSIFICATION: CLASS 3 — finite, progressing lowering with PHYSICAL STACK
> # ACCUMULATION. NOT a semantic cycle, NOT changing-signature non-progress. The
> # native route is LAWFUL (no operator-surface unlawful-route fork). The trace
> # showed the apparent active repeat 69->112 is the DELIBERATE rule that a
> # synthesized match default reuses the match occurrence's own origin (returns
> # through both exits — not a back-edge); the aborting ancestry (source
> # 313->97->94->91->expr 90) changes origin on every descent with NO active
> # signature repeat; the selected scope (scope 1, frame-origin 12, prov 0, frame
> # 0, invocation 1, ComputationalIHCall(3), depth 1) never targets a descendant
> # or replays child 2. Physical mechanism (exact pass-B ELF): large debug
> # prologues — lower_source_machine_with_continuation_inner 0x28198 (164,248 B),
> # lower_expr 0x10528 (66,856 B), producer-once 0x9c38 (39,992 B) — accumulate to
> # ~1,417,640 B of live frames before helpers/Cranelift/CLI/test exhaust the
> # physical stack. Max semantic depth 20 (abort at depth 18) => BYTE accumulation,
> # not unbounded semantic descent.
> #
> # §1b ANSWER: NO. Entries 4-5 are the parent-provenance/child-qualification role
> # collision; ENTRY 6 IS INDEPENDENT — a large compiler traversal frame stays live
> # across ordinary recursive source-branch lowering. The diagnostic DISPROVED the
> # condition under which entry 6 would have joined 4-5.
> #
> # ARCHITECT RULED FIX (full mechanism in evt_700s4qs9bvp3c): POP the large
> # source-machine inner frame BEFORE every recursive source descent. Add only
> # compiler-private TRANSIENT return vocabulary in lowering/source.rs —
> # SourceMachineExit { Complete | Reenter | DynamicMatch } +
> # SourceDynamicMatchRequest + SourceDynamicMatchScrutinee (5 variants:
> # BoundedNat/Bool/HostResult/
> # DynamicConstructor/Carried). KEEP the public-to-module wrapper signature
> # unchanged; change ONLY the private inner return type; add ONE private
> # dispatcher lower_source_dynamic_match_request. #[inline(never)] on the inner +
> # dispatcher is LOAD-BEARING (function-frame isolation, NOT stack provisioning).
> # The wrapper increments live_source_continuations + replaces source_control_root
> # exactly where it does now, calls the inner, then dispatches
> # Complete/Reenter/DynamicMatch BEFORE decrementing/restoring — so a delegated
> # child observes the same live depth/root while the 0x28198 inner frame has
> # returned. Closed inner-site conversion at exact lines (1830/1842 BoundedNat/
> # StructuralNat; 1899 Bool with exact True/False case indices; 1917 HostResult;
> # 1932 DynamicConstructor; 2029 classified Carried; 2345 deforested-selected-ret
> # -> Reenter); every other terminal return -> Complete(value). The dispatcher is
> # exhaustive, invokes the SAME five existing helpers with the same values/cases/
> # default/origin/env/control, re-derives Bool bodies only from the transported
> # exact case indices + parent origin, and must NOT rediscover a variant from
> # arity/values/counts/names/SSA/frame-state.
> #
> # FENCES: NO RUST_MIN_STACK / Builder::stack_size / ulimit / guard / depth cap /
> # test-thread change (a stack bump is NOT a valid repair — Research + Architect).
> # No SourceContinuation / SourceMachineState variant; no stored carrier / plan /
> # edge field / owner / invocation / frame / ABI / wire / runtime tag / TCB change.
> # Do NOT alter lower_forked_branch, any dynamic-match helper body,
> # CheckedFrameBranchScope, join/predecessor construction, resume behavior, the
> # HS3 partition, HS4 validator, HS5 mint/compose authority, the exact-sequence
> # check, or the dynamic-edge tree checks.
> #
> # STEWARD SCOPE CALL (Architect-directed): AMEND D5b IN PLACE, no predecessor, NO
> # new WP — a bounded lowering-internal frame-isolation refactor with no
> # independent user-visible deliverable or standalone acceptance beyond D5b.
> # Grows NO TCB (fenced explicitly) and needs NO ban-lift (lowering-internal, no
> # frozen wire/bridge), so it is the Steward's call and needs no operator
> # sign-off. The Architect's HS6 inventory + classification (its commits 71ebb7bf,
> # 75937f0d5, 4929b8ac on origin/main 4840337) are INCORPORATED into this
> # amendment (entry 6 + the independence paragraph in the symptom inventory below).
> #
> # CONTROLS (full in evt_700s4qs9bvp3c): (1) the exact unmutated COW positive
> # completes under the UNCHANGED ambient test stack + passes build / native-child /
> # interpreter / parity / COW / release-set / file-preservation; (2) a temporary
> # observation mutation keeping the dynamic request INSIDE the inner frame
> # reproduces the build-phase overflow (proves frame-pop is the repair), restored
> # before any other control; (3) report candidate-ELF prologue reservations +
> # active-frame census — NO numeric threshold is acceptance; the STRUCTURAL
> # property is that no call from the inner reaches the wrapper or any helper that
> # can reach lower_forked_branch; (4) re-run every prior HS3/HS4/HS5 positive +
> # mutation control (external inv/frame mismatch, retain-parent, drop-at-mint,
> # drop-at-compose, duplicate-worker, missing-context); MappingAllocate
> # byte-for-behaviour unchanged; (5) preserve the HS6 attempt + mutation artifacts
> # with hashes, then remove all phase/trace/mutation instrumentation.
> #
> # RUNTIME: HOLDS byte-clean at 34d11b8053 until THIS LANDS; runtime-leader
> # then explicitly re-kicks the frame-pop repair. Candidate returns to Architect
> # (REQUIRED reviewer) + runtime-qa + CI -> Steward M1-M4 -> lieutenant. COUNT IS 6;
> # the next §1a Research trigger is stop 9 (Architect owns the count) — a 7TH
> # structural refusal stops AGAIN (preserve, revert byte-clean, do not solve ahead)
> # but pulls NO research until stop 9.

# D5b NATIVE HARD-STOP 5 — CLOSED, AMENDED IN PLACE 2026-09-11 (Steward).

> # D5b native-lowering track, HARD STOP 5 — CLOSED. Architect fork-1 repair
> # ruling evt_6j9g88qtwhfrr (thr_7wy5wy45p7abm), grounded at byte-clean
> # 90df55be. The HS4 whole-tuple preservation WORKS for the dynamic edge, but
> # copying the non-root parent tuple INTO the selected frame-0 layer left it
> # OCCUPIED by parent invocation 1 when instantiate_checked_invocation_segment
> # had to qualify the call's expected frame-0 sequence for child invocation 2:
> # it instantiated no child frame and refused expected={0} instantiated={}
> # (evt_5246mrbnxprdb). The diagnostic (runtime-implementer evt_2hsw9q6sr14v0)
> # established FORK 1: the exact OwnedSelectedScope parent is NOT available at
> # make_computational_recursor (second-layer construction, core.rs:14531), so
> # HS4's whole-tuple transport there is NECESSARY — but the same scope IS
> # independently owned at the source mint (source.rs:4882) and stays live
> # through the caller of installation. No new stored carrier is justified. Same
> # D5b chain, count 5, a DISTINCT role-collision at the mint/compose boundary;
> # does NOT change the HS3 or HS4 rulings. NO §1a research until stop 6 — but a
> # SIXTH structural refusal MECHANICALLY triggers Research (Architect owns the
> # count).
> #
> # ARCHITECT RULED FIX (full mechanism in evt_6j9g88qtwhfrr): the frame-0 tuple
> # has SUCCESSIVE roles — make_computational_recursor transports parent
> # invocation 1; then, BEFORE source mint, OwnedSelectedScope takes over parent
> # authority and the selected layer becomes unqualified frame 0; mint writes
> # parent 1 into the existing DynamicSpliceEdge and instantiation qualifies the
> # layer as child 2. KEEP the HS4 checked_tuple() projection and the
> # make_computational_recursor(..., checked: CheckedComputationalFrame, ...)
> # signature and its initial non-root copy. REPLACE the inline closed match with
> # ONE private classifier CheckedComputationalFrame::nonroot_invocation() reused
> # at every seat; add the identical checked_tuple() to
> # ComputationalRecursorFramePayload; expose nothing wider from
> # OwnedSelectedScope. ATOMIC TRANSFER at mint: change only the private signature
> # mint_checked_computational_ih_instance(..., source_open_parent:
> # Option<&OwnedSelectedScope>) — the two core.rs recursor-as-callee sites and
> # finish_checked_computational_ih_marker pass None, only source_call_state
> # passes control.selected.selected_scope; transfer the parent BEFORE mint while
> # both exact representations are present and compared, then clear the three
> # invocation fields in the selected layer (NEVER clear after mint); a non-root
> # mismatch REFUSES, never falls back; a source root takes the unchanged
> # existing-parent arm. TRANSIENT source parent at compose: keep
> # validate_source_dynamic_splice_parent UNCHANGED; after it succeeds thread a
> # new final private external_source_parent: Option<CheckedComputationalFrame>
> # through all three source calls to install_recursor_invocation then
> # compose_oriented_subcontinuation (other callers/tests pass None); key
> # external_children by (parent_invocation_id, parent_frame_id), admit an absent
> # nonzero parent only when its exact key equals external_source_parent, every
> # other absent nonzero parent keeps "stale parent invocation". Add NO field to
> # any segment/layer/edge/plan/continuation. NO planner / plan schema /
> # continuation kind / ABI slot / carrier / runtime tag / surface / wire / owner
> # / TCB change.
> #
> # SUPERSEDES TWO HS4 ELEMENTS (edited in place below, not left contradictory):
> # (1) HS4's "non-root tuple copied byte-for-value and LEFT in the selected
> # layer" (persistent parent occupancy) is replaced by transfer-BEFORE-mint then
> # clear-the-three-fields — the occupancy was the HS5 cause. (2) HS4's
> # feature-gated ambient-at-make mutation control is RETIRED (it attacks a
> # boundary this repair retires and proves neither surviving authority edge),
> # replaced by three HS5 mutations: RetainParentInChildLayer (restores HS5
> # expected={0} instantiated={}), DropSourceParentAtMint (restores the HS4
> # source-open-parent refusal), DropSourceParentAtCompose (restores "stale
> # parent invocation" after child instantiation).
> #
> # STEWARD SCOPE CALL (Architect-directed): AMEND D5b IN PLACE, no predecessor —
> # one transient edge between the existing D8m tuple, the source-open validator,
> # and the composer, with no independent user-visible deliverable or standalone
> # acceptance beyond D5b (B-consumes-unbuilt-A). NO FURTHER BAN-LIFT
> # (Architect-confirmed; HS2/HS3/HS4 stand). Grows NO TCB (adds no stored form,
> # planner claim, continuation kind, ABI, wire, surface, runtime tag, owner, or
> # TCB entry), so it is the Steward's call and needs no operator sign-off. The
> # Architect's current-main symptom inventory (its commit 4c352d41, based on
> # 96200df25) is INCORPORATED as entries 5 in the symptom-inventory section
> # below.
> #
> # RUNTIME: HOLDS CLEAN at 90df55be until THIS amendment LANDS; runtime-leader
> # then explicitly re-kicks the fork-1 repair. Candidate returns to Architect
> # (REQUIRED reviewer) + runtime-qa + CI -> Steward M1-M4 -> lieutenant. A SIXTH
> # structural refusal stops AGAIN (preserve the attempt, revert byte-clean, do
> # not solve ahead) and mechanically triggers §1a Research. Architect controls:
> # positive COW — edge-2 parent == edge-1 child, frame 0 instantiated under
> # edge-2's child, the external parent key matches exactly one edge (assert
> # RELATIONS, not absolute ids); the three mutations above each apply exactly
> # once; a direct compose pair accepts a child-only segment with a matching
> # non-root external tuple and refuses on a changed external invocation/frame;
> # the HS3 duplicate-worker + missing-context controls stay independently
> # reaching; all frame/source-parent, D8m/order/population/edge/origin/ownership/
> # lifetime controls stay red; MappingAllocate byte-for-behaviour unchanged and
> # avoids the new non-root transport case.

# D5b NATIVE HARD-STOP 4 — CLOSED, AMENDED IN PLACE 2026-09-11 (Steward).

> # D5b native-lowering track, HARD STOP 4 — CLOSED. Architect fork-B repair
> # ruling evt_2c6snkcgb7bnf (thr_7wy5wy45p7abm), grounded at rebased clean
> # ee708274. The HS3 operand partition WORKS (the 11/6 refusal is closed, both
> # ruled controls pass); the COW positive then advanced PAST it and tripped a
> # DEEPER pre-existing invariant, validate_source_dynamic_splice_parent ("source
> # open occurrence disagrees with the closure-selected dynamic parent"). The
> # diagnostic (runtime-implementer evt_49ycrt545sjh4) selected FORK B: only the
> # dynamic PARENT INVOCATION is wrong (child 2==2, parent frame Some(0)==Some(0),
> # parent-invocation edge 0 != source-open 1). Cause: an existing 4-field D8m
> # checked tuple was NARROWED at the recursor-layer constructor —
> # make_computational_recursor took only checked_frame_id and re-derived
> # (invocation_id, source, depth) from the EMPTY ambient
> # active_recursive_invocations stack. Same D5b chain, count 4, a DISTINCT
> # pre-existing dynamic-splice invariant; does NOT change the HS3 ruling. NO §1a
> # research until stop 6 (the Architect owns the count).
> #
> # ARCHITECT RULED FIX (full mechanism in evt_2c6snkcgb7bnf): preserve the WHOLE
> # D8m tuple at the frame-to-layer edge. Add a private checked_tuple() projection
> # on ComputationalEliminatorFrame; change make_computational_recursor to take
> # `checked: CheckedComputationalFrame` (replacing the scalar frame_id);
> # classify the tuple with a CLOSED match (two root spellings normalize to the
> # unqualified selected-layer state, a non-root tuple is copied byte-for-value,
> # a malformed combination refuses) consulting NO ambient state [SUPERSEDED BY
> # HS5 (persistent parent occupancy): the make-time copy is retained but the
> # selected layer is CLEARED before mint — see the HS5 banner]; update all 4
> # (and only 4) callers to pass `.checked_tuple()`. segment_checked_invocation
> # stays semantically unchanged — it qualifies the segment being installed, NOT
> # the selected layer's dynamic parent (conflating those two roles caused HS4).
> # validate_source_dynamic_splice_parent is NOT weakened/bypassed. NO planner /
> # plan schema / continuation kind / ABI slot / carrier / runtime tag / surface /
> # wire / owner / TCB change.
> #
> # STEWARD SCOPE CALL (Architect-directed): AMEND D5b IN PLACE, no predecessor —
> # this is the missing preservation edge of the already-required D8m tuple, with
> # no independent user-visible deliverable or standalone acceptance beyond D5b
> # (B-consumes-unbuilt-A). NO FURTHER BAN-LIFT (Architect-confirmed; the HS2/HS3
> # amendments stand). Grows NO TCB (the fix's own fence forbids it; trusted_base()
> # untouched), so it is the Steward's call and needs no operator sign-off. The
> # Architect's current-main symptom inventory (its commit 586f30071, based on
> # 8aafeee33) is INCORPORATED into this amendment as the "## D5b static-operation
> # ownership hard-stop symptom inventory" section below.
> #
> # RUNTIME: HOLDS CLEAN at ee708274 until THIS amendment LANDS; runtime-leader
> # then explicitly re-kicks the fork-B repair. Candidate returns to Architect
> # (REQUIRED reviewer) + runtime-qa + CI -> Steward M1-M4 -> lieutenant. A FIFTH
> # structural refusal stops AGAIN (preserve the attempt, revert byte-clean, do
> # not solve ahead). Architect controls: the COW path forms 2 checked-IH edges
> # root -> inv1 -> inv2 (assert the RELATION, not absolute ids); a feature-gated
> # make_computational_recursor mutation restoring the ambient-derived tuple
> # reproduces the HS4 refusal exactly once [SUPERSEDED BY HS5: this control is
> # RETIRED — it attacks a boundary the HS5 repair retires; replaced by the three
> # HS5 mutations RetainParentInChildLayer / DropSourceParentAtMint /
> # DropSourceParentAtCompose]; the cross-check test refuses on a
> # changed checked_invocation_id; the HS3 duplicate-worker + missing-context
> # controls stay independently reaching; all D8m tuple-withdrawal / occurrence /
> # dynamic-edge sibling / origin / body / header / membership / ownership /
> # lifetime controls stay red; MappingAllocate byte-for-behaviour unchanged and
> # avoids the new non-root transport case.

# D5b NATIVE HARD-STOP 3 — CLOSED, AMENDED IN PLACE 2026-09-11 (Steward).

> # D5b native-lowering track, HARD STOP 3 — CLOSED. Architect re-ruling
> # evt_5781rmnajdpbb (thr_7wy5wy45p7abm), grounded at exact b94c5ae7, resolving
> # the §1a 3rd-stop chain. After HS2's authorized creation-route bridge fired
> # with the exact key, its CONSUMER refused the constructed frame: it would
> # append the 5 worker captures AGAIN onto a 6-input Parameter run that already
> # carries them (11 operands for 6 parameters). Per §1a the 3rd stop pulled a
> # mandatory Research advisory (evt_1w0grnt5xrseq) — permissive/behavior-only
> # closure-conversion + CPS prior art — and an independent Ken diagnostic
> # (runtime-implementer evt_6ecv3f40wtbtx, diagnostic-only on b94c5ae7, reverted
> # byte-clean). BOTH AGREE on the FIRST partition: the reaching producer is
> # lower_expr's specialized Lowered::Closure recursor-call arm, which assembles
> # `arguments ++ selected captures`; the 6 retarget inputs ARE the complete
> # direct-worker Parameter run, so the exact-key frame contributes ONLY the 4
> # context Capture operands. Position 0 does not weaken this (retarget input 0 =
> # call-origin-102/arg-origin-100 response binder, creation ordinary 0 = Vis
> # origin 825/824; both encode as wordless StaticResponseDeferred so
> # D9OperandIdentity reports equality but is NON-INJECTIVE there — proof is the
> # caller's structural application route, not value equality).
> #
> # ARCHITECT RULED FIX (full mechanism in evt_5781rmnajdpbb): add a CLOSED
> # compiler-local input distinction so the consumer never guesses from length —
> # a private sum RecursivePositionCallInputs { DeclaredArguments |
> # CompleteDirectWorker { arguments, worker_captures } } in lowering/calls.rs,
> # with two sibling constructor methods (the raw/carried-residual entry builds
> # DeclaredArguments; the specialized-closure entry keeps its params==args
> # refusal and builds CompleteDirectWorker), both routed through ONE private
> # resolver. Classify all 6 b94c5ae7 call sites (4 carried-residual =
> # DeclaredArguments; the 2 specialized-closure sites drop their local
> # extend(captures) and pass separate vectors). The 10-step
> # call_declared_context resolution selects by STRUCTURE, never by
> # inputs.len()/skip(5)/subtraction/SSA equality/D9OperandIdentity/frame-value
> # comparison. NO new planner claim, continuation kind, ABI slot, carrier,
> # runtime tag, boolean/tag parameter, 7th call site, or second frame — the
> # existing ConstructedContextFrame already separates worker_captures /
> # context_captures (plan-validated alternate-view storage, not an emitted
> # source on the complete route). The false "retarget supplies neither run"
> # comments (mod.rs:1030-1079, core.rs:10811-10852, calls.rs:940-975) are
> # corrected to state both contracts.
> #
> # STEWARD SCOPE CALL (Architect-directed packaging): AMEND D5b IN PLACE AGAIN —
> # do NOT cut a predecessor. Grounds: the consumer partition is ATOMIC with the
> # HS2 creation bridge (one predicate — operation selection and response
> # ownership on opposite sides of the checked-IH specialization boundary), has
> # NO independent user-visible deliverable, and its only acceptance is D5b's own
> # CompleteDirectWorker differential — a predecessor would have no standalone
> # green (the B-consumes-unbuilt-A defect) and would lengthen the priority lane
> # for nothing. NO FURTHER BAN-LIFT NEEDED: unlike HS2's closed-descriptor
> # bridge widening, this fix adds only a compiler-local private sum in calls.rs
> # and touches no wire / carrier / continuation form, so it does not trip the
> # frozen-wire ban — recording it here suffices; the HS2 ban-lift below stands
> # unchanged. Grows NO TCB (runtime lowering internals; trusted_base() delta
> # zero), so it is the Steward's call and needs no operator sign-off.
> #
> # RUNTIME: HOLDS CLEAN at b94c5ae7 until THIS amendment LANDS (the Architect's
> # explicit gate); runtime-leader then re-kicks the exact implementation.
> # Candidate returns to Architect (REQUIRED reviewer) + runtime-qa + CI ->
> # Steward M1-M4 -> lieutenant. Architect acceptance: D5b COW reaches
> # CompleteDirectWorker (1 application argument + 5 caller captures + 0 frame
> # worker operands + 4 frame context operands), native/interp parity + file
> # preservation pass; a test-only mutation appending frame worker captures
> # reproduces the 11-for-6 refusal; suppressing the 4 context operands still
> # refuses (the 2 producer-locals never fall back to defining_abi_operands);
> # existing foreign-origin/header/membership/injectivity/capture-order/ownership/
> # lifetime mutations stay red; MappingAllocate byte-for-behaviour unchanged and
> # avoids this route. HS3 chain CLOSED — no 4th attempt without returning to the
> # Architect.

## D5b static-operation ownership hard-stop symptom inventory

1. The source `MappingSource` match was already iota-reduced, but the selected
   `FSOp` operation constructor was materialized in the predeclared handler
   owner before its dispatch match, losing the known selector at the response
   owner (`evt_2vc32z6qn13bm`).
2. The widened checked-slot operation bridge reached the selected operation but
   bypassed `assemble_continuation_call_operands`, so it built no exact-key
   `ConstructedContextFrame` for producer-local context inputs
   (`evt_4by5d3mhtpzbn`).
3. Once that frame existed, the direct-worker retarget's six-input `Parameter`
   run already held its five worker captures; appending the frame's same worker
   vector produced eleven operands for six parameters (`evt_6399w1x64cmqm`).
4. After the caller partition made the parameter source structural, the same
   recursor occurrence's checked parent invocation survived in its source-open
   `ComputationalEliminatorFrame` as instance 1 but was rebuilt in the selected
   `ComputationalRecursorLayer` from an empty ambient invocation stack as root
   instance 0. Child instance and parent frame still agreed; only the dynamic
   parent invocation disagreed (`evt_49ycrt545sjh4`).
5. Copying that parent tuple into the selected layer repaired the dynamic edge,
   but the same frame-0 layer then remained occupied by parent invocation 1 when
   `instantiate_checked_invocation_segment` had to qualify the call's expected
   frame-0 sequence for child invocation 2. It instantiated no child frame and
   refused `expected={0} instantiated={}` (`evt_5246mrbnxprdb`).
6. After the exact pre-mint role transfer and transient source-parent authority
   advanced past both checked-frame-sequence and stale-parent refusals, the
   unmutated native COW witness overflowed its thread stack before returning an
   observable result (`evt_5ss07dc2h1wy0`).
7. After frame isolation removed the physical overflow, a planner-projected
   causal edge remained residual at the detached-result seat while that unit's
   lowered result was not the specialized producer constructor the existing
   five-guard contract requires (`evt_1rn0v625yq2m6`).
8. After the exact `InlineNoCall` disposition exclusion advanced the unchanged
   witness past entry 7, a `StaticResponseDeferred` compiler-control value
   reached the generic boundary-transfer preflight during object emission and
   correctly refused instead of crossing as a runtime value. The refusal does
   not identify its production origin, enclosing lowered-value path, transfer
   site, defining owner, selected response owner, or causal-call disposition,
   so its relation to entry 7 is not yet classified (`evt_4cwyw14tmcmv6`).

Entries 1–3 share the predicate already ruled at hard stop 3: operation
selection and response ownership lie across the checked-IH specialization
boundary without one structural authority for the explicit-argument,
worker-capture, and context-capture partition. Entry 4 is downstream of that
boundary but is a distinct existing dynamic-splice transport invariant: a
selected recursor layer must preserve the source computational frame's whole
checked tuple rather than re-derive any member from ambient state.

Entry 5 refines entry 4 rather than reversing its measured fact: the parent
identity must survive long enough to mint the edge, but placing it in the child
layer conflates two roles. Parent-edge provenance and child-frame qualification
must have separate existing authorities; one invocation scalar cannot name both.

Entry 6 does not share that predicate. The complete observation trace
classified it as finite, progressing lowering with physical stack accumulation.
The first active signature repeat was the source match's synthesized default,
which deliberately reuses the match occurrence's origin and returned through
both matching exits. The aborting ancestry instead changed origins on every
source descent and contained no active signature repeat. The selected parent
scope stayed stable; it neither targeted a descendant nor replayed a child.
The compiler's large source-machine inner frame remained live across each
recursive branch descent, independently of the entry 4–5 parent-provenance and
child-qualification role split (`evt_6x3kmb11454z2`).

Entry 7 is a one-layer disposition mismatch, not a genuinely missing call. The
complete observation proved the exact result edge had no direct emission,
composed claim or discharge, or checked-IH transport. That absence does not make
the producer constructor mandatory: the existing deferred bridge resolves the
same full identity from `(construct 825, frame 12, alternative 1, position 1)`
and, after successful inline completion, settles it `InlineNoCall` in the
artifact-wide `ContinuationCandidateLedger`. Its closeout already excludes that
exact disposition from the call-obligation subset. The earlier per-function
detached-result filter reads only direct/composed discharge feeds and therefore
re-demands a producer constructor for an identity the later authoritative
closeout says is not a call obligation (`evt_28xfh93fkh8hk`).

# D5b NATIVE HARD-STOP 2 — AMENDED IN PLACE 2026-09-11 (Steward scope call)

> # D5b native-lowering track, HARD STOP 2 (distinct from the surface HS#1-3
> # closed by the offset drop below). Architect ruling evt_139wvys9mv3z4
> # (thr_7wy5wy45p7abm), grounded at exact b94c5ae7. The prior operation-iota
> # attempt (eliminate the FSOp operation-dispatch match entirely inside
> # lower_computational_producer_construct) was ONE LAYER SHALLOW and is REVERTED:
> # direct descent through Predeclared(6) reaches the effect but BYPASSES the sole
> # seam that constructs the Specialization(1) response owner's context frame.
> #
> # ARCHITECT CLASSIFICATION: a proven bypass of an EXISTING authority — NOT a new
> # availability kind, owner transfer, continuation form, or carried aggregate.
> # The exact ruled repair (4 steps, full detail in the event) REUSES the landed
> # creation-site frame: widen the immediate checked-IH bridge's closed descriptor
> # by ONE exact shape (CheckedComputationalIHSlots{body: Match{scrutinee:
> # Var(operation_field)}} under a statically-known ITree::Vis), run the existing
> # assemble_continuation_call_operands / ConstructedContextFrame route
> # (core.rs:10853-10859) in DirectEmission to build the frame, THEN iota-select
> # the statically-known operation case and bind its lowered fields directly — no
> # FSOp carrier. Keyed STRUCTURALLY, never on ctor_551/MappingAcquireFile/FSOp/
> # origin numbers. Full "not authorized" fence + required controls/discriminators
> # in the event (no new claim/coordinate/slot kind; no owner transfer; no
> # continuation/carrier form; no verify_entry_frame relaxation; MappingAllocate
> # scalar path byte-unchanged; all D5b native/interp differentials green).
> #
> # STEWARD SCOPE CALL: AMEND D5b IN PLACE — do NOT cut a predecessor. Grounds:
> # (1) the bounded bridge-widening has NO independent consumer — it exists solely
> # to co-locate D5b's operation selection with its response ownership, and its
> # ONLY acceptance is D5b's own native==interp COW differential, so a predecessor
> # node would have no standalone green (the B-consumes-unbuilt-A decomposition
> # defect) and would lengthen the priority lane's critical path for zero benefit;
> # (2) one predicate, one differential, one reviewer set. This amendment
> # EXPLICITLY LIFTS the frame's own "frozen wire / no new form / no bridge
> # widening" ban FOR THIS ONE BOUNDED SHAPE ONLY (the closed-descriptor widening
> # above), reconciling the ban with the authorization the Architect just granted —
> # the D8m closed-descriptor comment and its exhaustive controls are UPDATED, not
> # contradicted silently. Nothing else in the frozen-wire / no-4th-op ban relaxes.
> # This is packaging, not design (Architect); it grows no TCB (the fence forbids
> # it), so it is the Steward's call and needs no operator sign-off.
> #
> # RUNTIME: HOLDS CLEAN at b94c5ae7 (backend/COW work + offset-less prelude
> # alignment PRESERVED) until this kick; then builds the ruled route. Candidate
> # returns to Architect (required) + runtime-qa + CI -> Steward M1-M4 ->
> # lieutenant. Any THIRD hard stop on this same static-operation/response-owner
> # question returns to the Architect BEFORE a new attempt (research trigger not
> # yet fired).

> # D5b RECUT 2026-09-11 (Steward) — §1b HS#3 STRUCTURAL CLOSURE, not a point fix
> # (Architect ruling evt_7c1adrctc3qf1, thr_7wy5wy45p7abm). D5b hit a hard stop
> # (runtime-implementer evt_6gejx0s8665qw): the frozen §1.9 FileBacked route in
> # withMapping must eval `eq_int offset 0` in-body before emitting the offset-less
> # MappingAcquireFile wire; calling eq_int on the carried-eliminated offset hits
> # the BoundaryCarrier wall — the SAME as the rejected D5a equality precheck.
> # Architect confirmed this is the 3rd hard-stop KEYED on the FORMING PREDICATE
> # (see below), so the fix is the general SURFACE RULE, not a 4th point ruling.
> #
> # THE STRUCTURAL CLOSURE (the ruled 1:1 surface-wire rule): the checked mapping
> # surface must be in 1:1 correspondence with the frozen wire — every checked
> # parameter passes through to a wire field; NONE is validate-and-discard. Entries
> # 1 (mapView token dropped -> window-direct) and 2 (mapWrite window-length dropped
> # -> payload carries extent) already closed this way; entry 3 gets the same move:
> # DROP the FileBacked offset field, `FileBacked (Resource FsHandle) Int Int` ->
> # `FileBacked (Resource FsHandle) Int` (length only), matching the offset-less
> # wire — make-illegal-states-unrepresentable, no eq_int, no BoundaryCarrier.
> #
> # RECUT SCOPE (replaces the point-fix framing): (a) §1.9 mapping-surface
> # 1:1-with-wire correction dropping the FileBacked offset (Path B, SPEC's call —
> # deciding question routed to spec-author/spec-leader: is a non-zero file offset
> # ever meaningful, or always 0? Expected always-0 per z4080's offset-less wire =>
> # drop it, §1.9 + seed-mapping update, Architect review + CV Spec-lane ->
> # spec-leader gate; Path A = a frozen-wire ABI change = Steward rescope, NOT
> # expected as it contradicts z4080); (b) a WHOLE-SURFACE validate-and-discard
> # CENSUS (Anonymous length, read window/mapBytes, mapWrite, protection,
> # withMapping source) proving every param is wire-carried 1:1 or removed = no 4th
> # entry (three known, two closed, this closes the third); (c) the checked-surface/
> # prelude alignment (drop the FileBacked offset field + remove the eq_int site),
> # riding the D5b WP. RETAINED as VALID (Architect): D3/D4/D5a landed; the D5b
> # native mmap-of-fd/MAP_PRIVATE backend + 0x0407 promotion + COW differential in
> # WIP ae014a36 (wp/ABI-S6-d5b-file-backed) — dropping the offset removes the
> # boundary check and turns the intentionally-red COW differential green. Runtime
> # ring HOLDS the boundary red (no in-body check, no carrier machinery — Architect)
> # until the surface correction lands. NO research pull (Architect §1a: prior art
> # has nothing further; the ITree single-unconditional-Vis advisory already
> # supports the shape; the predicate is our own surface/wire mismatch, 1:1 is
> # known-best). Runtime seat: implementer gpt-5.6-sol/high = T1. The CV seed-case-3
> # ResourceKindMismatch(Mapping) residual is a small nonblocking companion on a
> # DIFFERENT axis — not part of D5b (see the LANDED banner below).
> #
> # D5a-surface D1 LANDED 2026-09-11 (Steward) — origin/main 8c6136fa3 currently;
> # the D1 candidate bf84c1b1 landed at d8bbef963, blob-verified 3/3
> # (abi_s6_mapping_surface_native.rs, prelude.rs, px8f_buffer_io_surface.rs). Its
> # prerequisite, the §1.9 FORK below, resolved PATH B: the spec-author correction
> # (offset-based mapWrite window IS the payload extent) landed at d27bd8d13 (PR
> # #3483, spec-only), which discharged the mismatch seed and let D1 build. Path A
> # (independent length seat, host write-wire prerequisite) did NOT fire — no
> # Steward rescope. Gates on exact bf84c1b1: Runtime QA + Architect M4 + Adversary
> # NO-DEFECT (evt_78eaxbvtjbde8) + CV APPROVE (evt_5fgekq). ABI-S6 STAYS ACTIVE —
> # two things remain: (1) D5b MAP_PRIVATE file-backed COW seed (still RED/deferred,
> # the three-op wire frozen); (2) a CV RESIDUAL — seed case 3
> # ResourceKindMismatch(Mapping) arm is unnetted, so the merge is unblocked but
> # seed-case-3-full-green is gated on a small Mapping->buffer-op differential
> # (follow-on requested by CV). Decide post-land whether the residual folds into
> # continuing D5a or needs its own small coverage node (like
> # RT-MAPPING-DISPATCH-CONTROL-COVERAGE) — small, nonblocking. The FORMING
> # PREDICATE (native mapping surface admits no in-body control at an access site)
> # did NOT reach its HS#3 structural-closure trigger on this landing.
> #
> # D5a-surface D1 HS#2 (mapWrite bounds class) — Architect ruling 2026-09-10
> # (evt_5xsn7eb40xj8j, thr_2b9zky9skt6hc). CONFORMANCE / SURFACE-HONESTY gap, NOT
> # a soundness hole: the landed mapWrite bounds the write by the PAYLOAD extent
> # (host-checks start + bytes.len() <= extent), so no OOB/unchecked access — TCB
> # intact. The unmet promise is §1.9's claim that the DECLARED window (offset,
> # length) is checked: mapWrite drops the length on the frozen wire
> # (PrivateMappingWriteView carries no length seat), so mapping_window_length is
> # silently ignored. Semantic relation ruled EXACT; the declared write-window
> # length is REDUNDANT (the payload Bytes carries the extent; a write window
> # larger than its payload is incoherent under MAP_PRIVATE COW). The checked-
> # surface conditional precheck is REJECTED — it violates §1.9's own
> # no-in-body-response-transform invariant (764-766) and re-hits the native
> # BoundaryCarrier wall; do NOT add a wire field / 4th op / continuation
> # primitive, and do NOT fund a runtime-lowering change to transport a
> # conditional Ret/Vis at a mapping access.
> #
> # THE §1.9 FORK (contract-level, Spec's call — deciding question routed to
> # spec-author/spec-leader): is an INDEPENDENTLY-declared write-window length
> # (one that can differ from the payload) a meaningful part of the mapWrite
> # contract, or is the write window ALWAYS the payload extent?
> #   - Path B (Architect RECOMMENDED, expected under MAP_PRIVATE/COW scope):
> #     correct §1.9 to state mapWrite's checked window IS the payload extent
> #     [offset, offset+len(bytes)), bounds-checked against the extent (the landed
> #     host already does this); MappingWindow.length is not an independent write
> #     parameter. Spec-only (spec/ + conformance/ seed-mapping update), NO ABI
> #     change, NO build-lane rescope. Then Architect required review + CV
> #     Spec-lane. The landed mapWrite STANDS as correct for the binding property.
> #   - Path A (only if Spec names a real reason the independent length must be
> #     honored): add a length seat to the write op (host bounds-checks declared
> #     (offset,length) vs extent AND enforces length == bytes.len(); single
> #     unconditional Vis). Frozen-wire ABI change rippling to interp/native/
> #     planner MappingWriteView consumers — the Steward rescopes a small host-side
> #     write-wire PREREQUISITE node. Architect required review binds it.
> # STEWARD rescope/prerequisite call is made AFTER Spec answers (recorded here so
> # it is not lost). Meanwhile the mismatch seed case is BLOCKED-ON-§1.9-FORK, not
> # on the runtime ring; the held WIP 64453ef6 (matching-extent mapWrite bounds
> # control + 1/4097 acquisition parity) is VALID and becomes the candidate once
> # the fork resolves.
> #
> # SYMPTOM INVENTORY — D5a-surface composition (Architect §1a; HS#2 = z-count 2):
> #   1. (z4091 / HS#1) mapView mints a MappingSpan with no frozen-wire
> #      counterpart, forcing a body-level response transform native cannot
> #      transport — an abstract view-token carrying no invariant the request
> #      window does not. Resolved by the window-direct §1.9 respin (b33f8ac9b).
> #   2. (z4116 / HS#2) mapWrite's declared-window-length check cannot be enforced
> #      over the frozen wire without a checked-surface conditional Ret/Vis, which
> #      native cannot transport (BoundaryCarrier) — the mapping access site cannot
> #      carry any response shape beyond a single unconditional Vis.
> #   3. (HS#3, D5b) FileBacked carries a file offset the offset-less
> #      MappingAcquireFile wire drops after an `eq_int offset 0` check —
> #      BoundaryCarrier over the carried eliminated offset — keyed on a
> #      validate-and-discard checked-surface parameter absent from the frozen wire.
> #      Same predicate as 1-2; closed STRUCTURALLY by the 1:1 surface-wire rule
> #      (drop the offset field), not a point fix (Architect evt_7c1adrctc3qf1).
> # FORMING PREDICATE — NOW FIRED as the §1b structural-closure trigger (HS#3
> # landed 2026-09-11): "the checked mapping surface carries a VALIDATE-AND-DISCARD
> # parameter the frozen wire does not carry, forcing an in-body computation over a
> # carried eliminated value native owner-lowering cannot transport — every op can
> # only emit a single unconditional Vis." The three entries are ONE defect;
> # Architect ruled the general surface rule (checked surface 1:1 with the frozen
> # wire — every param wire-carried or removed, NONE validate-and-discard), NOT a
> # 4th point ruling. NO research pull (Architect §1a: prior art has nothing
> # further; ITree single-unconditional-Vis advisory already supports the shape).
> # Path B CONFIRMED by spec-author (evt_2gbnzp1qepvc8): a non-zero file offset is
> # never meaningful for FileBacked (always 0) — §1.9 drops the offset field; no
> # Steward rescope (Path A did not fire).
> #
> # D5a-surface D1 RE-RELEASED 2026-09-10 (Steward) — all three prerequisites
> # LANDED; runtime ring resume authorized. The held multi-op acceptance resumes
> # on current main d70db3299. The prerequisite frame (5bf1915e4, HS#4) named
> # RT-MAPPING-MULTIOP-DISPATCH as the blocker for D5a-surface's remaining multi-op
> # acceptance; it MERGED at 1c48b6c5c (blob-verified 17/17). With that, the three
> # things D5a-surface D1 composes over are all on main:
> #   - D5a-core native anonymous wire (29f64ff6f) — real mmap/munmap three-op wire;
> #   - window-direct §1.9 surface contract (b33f8ac9b) — mapView/MappingSpan
> #     dropped, MappingWindow passed directly to mapBytes/mapWrite (this is what
> #     dissolved the D0 continuation blocker, Architect evt_56e6jx62s0qpb);
> #   - RT-MAPPING general N>=2 same-producer dispatch (1c48b6c5c) — the runtime
> #     machinery that lets a SECOND mapping effect sequence and execute.
> # So D0 is resolved (via the respin) and D1 is buildable. RESUME D5a-surface D1
> # on a base carrying d70db3299: build the three checked procs (withMapping /
> # mapBytes / mapWrite) over the frozen three-op wire, discharge the TWO achievable
> # seed cases (4 KiB granule + opacity/bounds) with controls intact, and the
> # multi-op acceptance (a second mapping effect sequenced after the first executes
> # and matches interp) that RT-MAPPING unblocked. The MAP_PRIVATE COW seed STAYS
> # RED (BLOCKED-ON-D5b), the three-op wire is FROZEN, no fourth op, no §1.9
> # alteration — route to Steward+Architect on any of those (hard stops unchanged).
> # Re-measure the fixed inputs at pickup. Reviewers: Runtime QA + Architect
> # (required, TCB-adjacent) + Adversary -> Steward M1-M4 -> lieutenant.
> #
> # WP frame. Authority: `docs/program/10-linux-abi-completion.md §4` Track S.
> Fixed inputs measured at `origin/main` `2dbe90af798bdd72aacf95ee45e60c2130ad75cc`
> (the ABI-S1-closed tip). The `ken-host` / `ken-runtime` / prelude surfaces this
> node touches are byte-identical to `97aeab6cb` (the frame SHA runtime-leader
> named); the only delta between them is the ABI-S1 node status flip and its
> gen-progress regen. Re-measure the fixed inputs at pickup. ABI-S1 (D1-D6) is
> merged.

## Objective

Model ordinary anonymous and file-backed memory mappings as **opaque
runtime-owned regions** exposed to Ken only as **bounded byte views** — never as
Ken pointers. A mapping is acquired, held under a revocation-governed lifetime,
read (and, where the mapping is writable, written) through bounds-checked views
that yield owned bytes, and released — with every failure surfaced as a stable
typed refusal, and with no memory address ever crossing into Ken code.

This supplies the mapping / lifetime / bounded-access substrate that the later
L2-8 MMIO work builds on. Getting the ownership and bounds shape right here is
what keeps MMIO from needing raw pointers in application Ken, which is a stated
exit condition of the whole program (§6).

## Design judgment (front-loaded)

1. **The opacity / lifetime / bounds substrate ALREADY EXISTS. Do not reinvent
   it — reuse or extend it.** The tree already carries a complete, `NativeTested`
   opaque-region-plus-bounded-view mechanism, built for PX8:
   - `ResourceKindV1::Buffer` (`crates/ken-host/src/effect_v1.rs:789`) is a
     resource kind alongside `FsHandle`, held in the generation-checked
     `ResourceTableV1` (`effect_v1.rs:1010`).
   - `BufferRegionV1` (`effect_v1.rs:908`) is an opaque host-owned `Vec<u8>` with
     an initialized window and a bounds-checked `initialized_slice(start,len)` —
     no raw pointer ever crosses out.
   - `HostOpV1::BufferAllocate` (`0x0402`) mints a `Buffer` resource token;
     `HostOpV1::BufferFreeze` (`0x0403`) turns a bounded span of that region into
     a `CanonicalReplyV1::Bytes` value (owned bytes on the wire). This is
     literally "read a bounded byte view out of an opaque runtime-owned region
     and hand back owned bytes, never a pointer."
   - PX8-SPAN-PROV (`effect_v1.rs:1970` and enforcement at `:2626`) binds a
     span's originating acquisition token to its target before any byte exposure,
     and shares `InvalidBounds` with ordinary bounds failures so the two are
     indistinguishable to an observer.
   - Ken sees `Resource k` as an opaque primitive (`PrimReduction::OpaqueType`,
     `prelude.rs:1818`) indexed by the closed `ResourceKind` sum; `BufferHandle`
     / `BufferSpan` are private-constructor wrappers whose constructors are
     stripped from the public name map. Ken cannot construct, project, or inspect
     a `Resource` value — it only flows opaquely from a host call to a checked
     proc.

   ABI-S6 is therefore NOT the invention of a region/view/opacity model. It is
   the addition of an **OS-backed** region to the model that already governs
   in-process buffers.

2. **What is genuinely new is the OS backend seam, and ABI-S6 is a SPLICE of two
   landed precedents, not an extension of either alone.** `BufferAllocate` /
   `BufferFreeze` have NO `HostEffectBackendV1` method — they are pure in-process
   `Vec<u8>` operations. A real `mmap` / `munmap` needs a backend method, shaped
   like the `FsHandle` path's `fs_resource_duplicate` (`effect_v1.rs:1946`) and
   `resource_close` (`:1955`). So the region-acquire / region-release side
   follows the FsHandle real-syscall-backend pattern, while the bounded-view
   side follows the Buffer region/span pattern. Picking one precedent wholesale
   is wrong; the deliverable is their disciplined composition.

3. **Lifetime is revocation lineage, reused verbatim — not a new lifetime
   notion.** A region resource is minted through the existing
   `ResourceTableV1::insert_owner` (`effect_v1.rs:1053`) with an explicit
   `RightSet` and a `RevocationNodeId` provenance, and resolved through a
   kind/generation/rights-checked accessor. Revocation of a region's lineage node
   must make the region and every view derived from it inadmissible, using the
   same `RevocationDomain` (`revocation_v1.rs`) mechanism ABI-S1 D5 used for
   descriptor aliases (`copy` shares a node, `attenuate` narrows). Unmap and
   revocation-close interact exactly as `resource_close` and admission do today
   (`admit_resources` / `ResourceAdmissionLeaseV1`, `effect_v1.rs:1330`): a
   revoked or unmapped region can never yield a live view.

4. **Never Ken pointers is a hard invariant with a landed mechanism to inherit.**
   The region is a third `ResourceOwnerV1` / `ResourceKindV1` participant resolved
   only through the generation-checked slot table; the token that names it is the
   opaque `{slot: u32, generation: u32}` pair (`effect_v1.rs:770`), never an
   address. Any bounded view is a Ken-opaque wrapper with a private constructor,
   following `BufferHandle` / `BufferSpan` exactly. No deliverable may expose the
   region's real address, an offset that is an address, or a projectable pointer.

5. **Lands `RepresentedUnavailable`; native promotion is a separate later node.**
   This follows the ABI-S1 precedent exactly (the entire descriptor slice landed
   `RepresentedUnavailable`, proved by
   `abi_s1_partial_keeps_descriptor_operations_represented_unavailable`,
   `effect_v1.rs:4537`). ABI-S6 delivers the surface, the region/view/lifetime
   representation, the error vocabulary, the inventory registration, and the
   backend trait seam — NOT the differential native `mmap` harness. A node that
   lands `RepresentedUnavailable` owes NOTHING in the `ken-runtime` Cranelift
   lowering and adds NO `SYS_*` pinned fact (ABI-S1 D5 added zero — there is no
   "SYS_DUP3" precedent; SYS_ facts are added only at native promotion). Native
   promotion (backend implementation of `mmap`/`munmap`, `availability` ->
   `NativeTested`, `NATIVE_TESTED_TARGETS_V1`, `SYS_MMAP`/`SYS_MUNMAP` facts, the
   three-dimensional Cranelift arms, the differential harness) is framed when
   this lands.

## D0 — the representation-shape ruling is the ARCHITECT's, not this frame's

Per runtime-leader's steer, this frame fixes the BOUNDARY, not the API shape.
There is one genuine representation fork, and it is the Architect's call on
measured evidence, not a Steward or ring assumption:

**Does an OS-backed mapping REUSE `ResourceKind::Buffer` / `BufferHandle` /
`BufferSpan` / PX8-SPAN-PROV (a mapping is "a buffer whose backing store is
OS-managed"), or does it mint a PARALLEL `ResourceKind::Mapping` +
`MappingHandle` / `MappingSpan` family?**

D0 is the first deliverable: the ring measures whether the existing `Buffer`
view contract can carry an OS-backed region's semantics — file-backed vs
anonymous backing, writable vs read-only protection, faulting/partial-residency,
and unmap-vs-free lifetime — WITHOUT weakening PX8-SPAN-PROV's origin binding or
the bounds guarantees, and **HARD-STOPS to the Architect** with that
measurement. The Architect rules the shape. Reasons this cannot be pre-decided:
reusing `Buffer` risks conflating "free this in-process allocation" with "unmap
this OS mapping" (a real safety distinction), while a parallel `Mapping` kind
risks exactly the duplicated ownership model the objective forbids. The frame
requires the boundary (judgments 1-5); the Architect chooses which representation
meets it. Do not author either shape before that ruling.

## Deliverables

D0 gates the rest. After the Architect's shape ruling, the remaining work is the
region-acquire/hold/view/release surface under that shape. Per-increment partial
landing is authorized. Each operation is threaded through every registration
site (see AC-INVENTORY-BUILD-BREAK) and lands `RepresentedUnavailable`.

CURRENT POSITION (2026-09-10, measured at `origin/main` `9d614db31`): D0-D4 have
merged — the represented `Mapping` operation surface (acquire / bounded views /
registration) landed on **anonymous** and **file-backed** backing, fail-closed
and `RepresentedUnavailable` (D3 PR #3444 adversary M8 NO DEFECT; D4 file-backed
acquire landed cf894cdb5). The normative **§1.9 mapping-surface contract + seed**
landed (`cd62283b8`, spec `30-surface/38-ffi-io.md §1.9` +
`conformance/surface/ffi-io/seed-mapping.md`), so the checked surface and its
discriminating negatives are now pinned. **The Architect then SPLIT the native
D5a increment into D5a-core and D5a-surface** (relayed by runtime-leader
evt_580zc1y1jknn0 / evt_6x2krr2wfvax8): D5a-core owns the native anonymous
promotion (the frozen three-op wire), currently a candidate `7beecdb4e` on
`wp/ABI-S6-native-anonymous` under fresh Runtime-QA + Architect review; D5a-surface
owns the checked §1.9 composition over that frozen wire + the seed discharge (the
increment framed below). The node stays `active` across increments.

- **D0 — representation-shape measurement + Architect ruling (reasoning-dense,
  first).** As above: measure the reuse-vs-parallel fork against the boundary,
  hard-stop to the Architect, land nothing until ruled.
- **D1 — region acquire + release under lifetime (load-bearing).** A
  mapping-acquire operation that mints an opaque runtime-owned region resource
  (kind per D0) via `insert_owner` with an explicit `RightSet` and revocation
  provenance, plus the `HostEffectBackendV1` method seam for the real
  `mmap`/`munmap` (shaped like `fs_resource_duplicate` / `resource_close`), and a
  region-release path. Anonymous backing is the base case; file-backed acquire
  takes a held `FsHandle` and shares/derives its revocation lineage. Deny by
  default: acquisition requires the governing right; a region cannot outlive a
  revocation of its lineage.
- **D2 — bounded byte views over the region.** Reads produce bounded,
  generation- and bounds-checked byte views that yield owned bytes on the wire
  (the `BufferFreeze` shape), with the span's origin bound to its region per
  PX8-SPAN-PROV. Where D0/D1 admit a writable mapping, writes go through the same
  bounds-checked view discipline. No raw address, and no address-shaped offset,
  crosses to Ken.
- **D3 — registration, inventory, and sentinel maintenance (build-break).** Every
  new operation is threaded through all registration sites (judgment 2 of AC
  below). The enumeration sentinels that hand-list the represented/deferred set
  (`abi_a3_completion_leaves_only_the_non_track_a_deferred_tail`,
  `effect_v1.rs:4564`) are extended to include the new operations; the
  descriptor-band sentinel (`try_from(0x0315) == Err`) stays UNTOUCHED (ABI-S6 is
  not a descriptor op — it takes the next free band-04 slot `0x0404` or a new
  band, per D0). Pinned ABI-fact inventory updated only as the represented
  surface requires; NO `SYS_*` fact is added (native promotion is a later node).
  Whichever increment (D2 or D3) first makes the parallel `Mapping`
  `ResourceKind` tag reachable to the native reifier also closes the reification
  totality gap — see AC-NATIVE-REIFICATION-TOTAL.
- **D4 — file-backed mapping acquire (the FsHandle-backed backing axis).** D1-D3
  landed the represented `Mapping` surface on anonymous backing; D4 adds the
  file-backed acquire path — a mapping-acquire that takes a **held `FsHandle`**
  and shares/derives its revocation lineage from it (AC-LIFETIME-REVOCATION
  already binds this), exercising the source-`FsHandle` lineage/rights rule and
  the file-backed backing axis. The `HostEffectBackendV1` seam for the real
  `mmap`-of-fd / `munmap` is shaped as in D1 but lands `RepresentedUnavailable`;
  no `SYS_*` fact (native promotion is a separate later increment). Deny by
  default: acquisition requires the governing right on the source handle, and a
  file-backed region cannot outlive a revocation of the file's lineage.
  **The capacity-governance fork (AC-MAPPING-CAPACITY-GOVERNANCE) is a hard stop
  to the ARCHITECT before implementation** — whether the mapping surface adopts
  per-mapping and invocation-wide limits or an explicit unbounded-but-graceful
  contract is a design ruling, not this frame's call; land nothing on that axis
  until ruled.
- **D5a-core — native anonymous promotion (the frozen three-op wire).** Promotes
  the anonymous `Mapping` acquire/view/release wire from `RepresentedUnavailable`
  to native (real `mmap`/`munmap`), closing the native-reifier totality gap for
  the `Mapping` tag (AC-NATIVE-REIFICATION-TOTAL). Owned by the Architect's
  split; a separate candidate `7beecdb4e` on `wp/ABI-S6-native-anonymous`
  under fresh Runtime-QA + Architect review against the seven core gates (native
  reachability, opacity, 4 KiB shared accounting, MAP_PRIVATE/munmap, reifier
  totality, no new right/TCB, seed still honestly undischarged). Not this frame's
  deliverable — recorded here for the increment plan; it routes to the Steward on
  its own QA/Architect approval.
- **D5a-surface — the checked §1.9 composition over the frozen three-op wire
  (D0-first; THIS is the framed/released increment).** Compose the public checked
  Mapping surface — `withMapping` / `mapBytes` / `mapWrite`, exactly the **three**
  `spec/30-surface/38-ffi-io.md §1.9` procs — over the frozen three-op native wire
  D5a-core delivers, and DISCHARGE the **two ACHIEVABLE** RED-UNTIL-BUILT seed
  cases (4 KiB granule + opacity/bounds) in
  `conformance/surface/ffi-io/seed-mapping.md`. The MAP_PRIVATE COW seed case
  DEFERS to D5b (file acquisition) — see AC-SEED-DISCHARGE-D5A and the D5b
  deliverable below. Grounded on the landed **window-direct** §1.9 contract
  (`b33f8ac9b`, the respin that dropped `mapView` / private `MappingSpan` and
  passes `MappingWindow` directly to `mapBytes` / `mapWrite`; supersedes the
  earlier `cd62283b8` four-proc contract).
  - **D0 — RESOLVED 2026-09-10 (Architect evt_56e6jx62s0qpb -> §1.9 window-direct
    respin, landed `b33f8ac9b`).** The bracket did NOT compose natively over the
    original four-proc contract — `mapView` returned a `MappingSpan`, a
    capture-bearing continuation the native path cannot own. The Architect ruled
    the defect UPSTREAM: `mapView` / private `MappingSpan` did not earn their keep
    (they copied the `BufferSpan` mechanism without the capping condition that
    justifies it; a mapping's live subrange equals its window when live, so
    `MappingSpan` carried no invariant `MappingWindow` doesn't). The fix was the
    window-direct §1.9 respin, NOT a native continuation prerequisite; the
    represented-K seam is not needed. The original D0 characterization follows for
    the record.
  - **D0 (original) — characterize the continuation prerequisite; HARD-STOP
    if unmet.**
    `withMapping`'s bracket body `MappingHandle -> HostIO a (ResourceBodyResult e
    r)` composes a checked resource bracket exactly as `withBuffer` (`§1.7.1`) /
    the `FsHandle` acquire/release bracket do. D0 measures whether that checked
    bracket composition is buildable on the EXISTING resource-bracket /
    continuation machinery, or whether it requires a continuation capability the
    runtime does not yet have (the represented-K seam). Report the exact
    prerequisite and HARD-STOP to the Steward + Architect if it is unmet. Do NOT
    build new continuation machinery here, do NOT alter the §1.9 contract to route
    around it, and do NOT add an operation — any of those is out of this increment's
    scope and is a distinct node the Architect must rule.
  - **D1 (D0 cleared via the respin) — build the checked composition + discharge
    the two achievable seed cases.** Implement the three checked procs
    (`withMapping` / `mapBytes` / `mapWrite`) over the frozen three-op wire and
    turn the TWO achievable seed cases green WITH their discriminating controls
    intact: the fixed 4 KiB host-independent granule (1 B -> 4096, 4097 B -> 8192;
    native == interpreted), and opacity + bounds (no raw address in any Ken value
    or token; out-of-range `mapBytes` / `mapWrite` is a fail-visible
    `ResourceError`; a wrong-kind token is `ResourceKindMismatch` naming
    `Mapping`; `ReadOnly` refuses `mapWrite`). The MAP_PRIVATE COW case STAYS RED,
    BLOCKED-ON-D5b (file acquisition / `MappingAcquireFile`) — NOT weakened or
    deleted; discharging it is D5b's acceptance. Rationale (Steward ruling A,
    Architect-confirmed evt_5en9jgjb24the): COW is intrinsically file-backed —
    "writes through a file mapping do not reach the backing file" has NO anonymous
    analog — so it needs `withMapping (FileBacked ...)` -> `MappingAcquireFile`, a
    fourth op outside D5a-surface's frozen-three-op / no-new-op boundary.
  - **Constraints (hard stops, per the Architect split).** No alteration of the
    §1.9 contract to match D5a-core; no new operation beyond the three §1.9 procs
    (`withMapping` / `mapBytes` / `mapWrite`); the three-op native wire is frozen.
    Route to the Steward + Architect on any of these rather than absorbing it.
- **D5b — native file-backed mapping acquisition + the MAP_PRIVATE COW discharge
  (the COW successor; framed here per Steward ruling A, Architect-confirmed
  evt_5en9jgjb24the).** Promote the file-acquisition op `MappingAcquireFile` — the
  `withMapping (FileBacked ...)` route that D4 landed and the D5a-core split
  explicitly kept `RepresentedUnavailable` (Architect z4088) — to native (real
  `mmap`-of-fd / `munmap`), so a real file-backed MAP_PRIVATE mapping can be
  exercised. This is the fourth op OUTSIDE D5a-surface's frozen three-op wire, so
  it is its own increment (Architect's z4085/z4088 staging: D5a = anonymous
  MappingAllocate / ReadView / WriteView + ResourceRelease; D5b = file
  acquisition). DISCHARGE the deferred MAP_PRIVATE COW seed case
  (`conformance/surface/ffi-io/seed-mapping.md` case 1, keyed
  `BLOCKED-ON-ABI-S6-D5b`): a write through a MAP_PRIVATE file mapping is observed
  in-mapping but does NOT reach the backing file (the ordinary-file read observes
  the ORIGINAL bytes), so a MAP_SHARED / write-through surface reds — the
  discriminating control intact; a green-vs-green pass against its named
  non-conforming implementation is vacuous and is a HARD STOP, not a discharge.
  **The Architect is D5b's REQUIRED reviewer** (evt_5en9jgjb24the): the COW
  discriminator is soundness-relevant, and the file-backed lineage/rights rule (a
  file-backed region shares/derives its revocation lineage from the source
  `FsHandle` per AC-LIFETIME-REVOCATION) is the Architect's gate. The
  capacity-governance fork (AC-MAPPING-CAPACITY-GOVERNANCE) applies to any
  file-acquisition capacity ruling not already settled at D4.

## Acceptance criteria (each with its control)

- **AC-OPAQUE-NO-POINTER (hard invariant).** No Ken-visible value produced by any
  ABI-S6 operation is, contains, or can be projected to, a memory address. The
  region token is the generation-checked slot pair; views are private-constructor
  opaque wrappers. Control: a construction/projection path that would expose an
  address or an address-shaped offset is uncompilable in Ken (no public
  constructor / the name is stripped from the public map), and a mutation that
  makes a view's byte offset an absolute address reddens a named test.
- **AC-BOUNDS-CHECKED (load-bearing).** Every byte a view yields lies within the
  region's valid window; an out-of-range or wrong-origin view is refused before
  any byte is exposed, sharing the `InvalidBounds` refusal so it is
  indistinguishable from an ordinary bounds failure (PX8-SPAN-PROV discipline).
  Control: a view request past the window, and a span whose origin token does not
  match its region, each red a named test; neutering the bounds/origin check
  leaves the suite green only if the control is vacuous — so the control must
  fail against a check-neutered tree.
- **AC-LIFETIME-REVOCATION.** A region and every view derived from it become
  inadmissible when the region's revocation lineage node is revoked, and a
  released/unmapped region yields no further live view. A file-backed region
  shares or derives its lineage from the source `FsHandle` so revoking the file
  revokes the mapping. Control: revoke the lineage (or release the region), then
  a subsequent view/read is refused with the stable typed refusal; a mutation
  that lets a view survive its region's revocation reddens a named test.
- **AC-REFUSAL-TYPED.** Every failure path — acquisition denied, backend
  (`mmap`) failure, out-of-bounds, revoked/closed region, malformed token —
  surfaces a stable typed refusal through the existing `ResourceErrorV1` /
  `SemanticErrorV1::Resource` (kind-generic) vocabulary or, if D0/Architect deem
  a mapping-specific arm necessary, an explicitly ruled addition. No raw errno,
  no silent fallback, no fail-open. Control: each failure path asserts its exact
  refusal; a path that drops the refusal identity or defaults open reds a named
  test.
- **AC-INVENTORY-BUILD-BREAK.** Each new operation is registered through every
  `HostOpV1` site — enum variant + opcode, `next_in_inventory`, `availability`,
  `is_ambient`, `capability_requirement`, `resource_admission_requirement`,
  request/reply schema, `dispatch_host_op_v1` arm, the `effect_abi_v1.catalog`
  row, the `effect_abi_probe.c` layout, the `abi_v1.rs` mirrored struct + decode,
  and the `effect_wire.rs` codec — so that dropping any one registration is a
  build break (`E0004` or a reddened derived-inventory test). Assertions are
  named memberships and properties, never total counts. Control: drop a
  registration -> the build or the derived-inventory test reds. NOTE: this is
  ~12-14 sites across three Rust files plus the catalog text and the C probe
  header, not five — budget accordingly.
- **AC-REPRESENTED.** New operations land `availability =
  RepresentedUnavailable`; `host_effect_wire_layout_v1` returns
  `OperationUnavailable` for them; the generated `effect_abi_v1.catalog` status
  matches; they are absent from `NATIVE_TESTED_TARGETS_V1`; and NO `SYS_*` fact is
  added. There is no `ken-runtime` Cranelift obligation. Control: the
  represented-tail sentinel and the fact-inventory anchor both reflect the new
  ops with no native-status or SYS_-fact change; a native-status flip reds a named
  test.
- **AC-NATIVE-REIFICATION-TOTAL (D2/D3 seam; adversary bounded obs on D1,
  evt_7cnqjqpd9zv1e / lesson bbc87dc2b).** D0 ruled a PARALLEL `Mapping`
  `ResourceKind` and D1 landed it as an unreachable, fail-closed tag (correctly
  NOT touching the native reifier — no opcode makes it reachable). The native
  `ResourceKind` reification path is therefore currently NON-total over the
  closed sum for the `Mapping` tag: no `SynthesizedFixedConstructorRole::
  ResourceKindMapping` role, `ALL = [Self; 49]`, and `resource_kind_value` has
  only two `DynamicConstructor` alternatives — a native `Mapping` resource error
  would hit `malformed_dynamic_constructor_trap` rather than reify. In the SAME
  increment (D2 bounded-view / D3 registration) that first makes the `Mapping`
  tag reachable to the native reifier, extend that path: add
  `SynthesizedFixedConstructorRole::ResourceKindMapping` plus its process symbol
  and the third `resource_kind_value` alternative (native lowering,
  `crates/ken-runtime` `lowering/effects.rs` ~`:4052`), so the reifier is total
  over the closed `ResourceKind` sum. Control: a native `Mapping` resource error
  reifies to its typed Ken constructor rather than trapping; a mutation dropping
  the `ResourceKindMapping` alternative reds a named test. This is NOT a D1
  defect — D1's tag is unreachable and fail-closed — it binds whichever
  increment first makes the tag reachable.
- **AC-MAPPING-CAPACITY-GOVERNANCE (D4; adversary bounded obs on D3,
  evt_n041yasg5m26).** `MappingAllocate` currently has NO capacity governor:
  `try_new_anonymous` (`effect_v1.rs:1044`) rejects `length == 0` and fails
  gracefully on a huge length (`try_reserve_exact` -> `AllocationFailed`, no
  OOM-abort), but there is no per-mapping cap and no invocation-wide live-mapping
  accounting — unlike `BufferAllocate`, which enforces `per_buffer_max_capacity`
  (1 MiB, sealed catalog `buffer.per_buffer_max_capacity|1048576`) and
  `invocation_max_live_capacity` at `insert_buffer` (`effect_v1.rs:1266`). This
  is inert in D1-D3 (`MappingAllocate` is unreachable from Ken source — no
  producer syntax) but goes LIVE the moment a source program can emit
  `MappingAllocate` with an attacker-influenced `length: u64` or multiplicity: a
  represented-side resource-exhaustion vector capped only by the process
  allocator. D4 must CONSCIOUSLY rule — as an Architect hard stop BEFORE
  implementation (see Hard stop) — whether a `mapping.per_mapping_max_capacity`
  and an invocation-wide mapping limit belong in the sealed ABI, or whether
  unbounded-but-graceful is the intended contract (mappings are semantically
  meant to be larger than buffers, so unbounded MAY be by design). Whichever is
  ruled, it is a DELIBERATE ABI ruling, not a silent gap. Control: if a limit is
  adopted, an allocation past the per-mapping cap and the (N+1)th live mapping
  past the invocation cap each red a named test, and the control must fail
  against a governor-neutered tree (non-vacuous); if unbounded-but-graceful is
  ruled, a huge / many-mapping request still fails closed with the typed refusal
  (never an OOM-abort) and that graceful-failure path is asserted.
- **AC-RIGHT-BUDGET (measured constraint, not a deliverable).** `RightSet`
  (`capability.rs:94`) is a `u8` with 7 of 8 bits assigned — one bit remains. If
  the mapping needs a distinct capability right rather than reusing `READ` /
  `WRITE`, it consumes the last bit; a SECOND new right requires widening
  `RightSet` everywhere it crosses the wire (catalog, C probe, native lowering)
  and is a HARD STOP to the Steward + Architect. State in the candidate which
  rights the mapping surface uses and whether the last bit was consumed.
- **AC-AFFECTED-CLOSURE.** Cover every target that loads a changed module, not
  only diff-touched ones: the `effect_v1.rs` consumers in `ken-verify`
  (`imported_catalog_partition_is_exact_and_closed`), `ken-elaborator`
  (`export.rs` / `erasure.rs` / `prelude.rs`), the generated catalog data file,
  and the `ken-interp` reify path. Green in CI is the workspace verdict — build
  and test locally targeted only, never `--workspace` (COORDINATION §12).
- **AC-SEED-DISCHARGE-D5A (D5a-surface — the point of the increment; scoped to the
  TWO achievable cases per Steward ruling A, Architect-confirmed
  evt_5en9jgjb24the).** The **two** anonymous-composition-achievable
  `conformance/surface/ffi-io/seed-mapping.md` cases flip from RED-UNTIL-BUILT to
  GREEN, each with its discriminating control still refuting a non-conforming
  implementation — the pair, not a lone positive: the **4 KiB granule** case — the
  charged sizes are exactly 4096/4096/8192 and NATIVE == INTERPRETED, so a
  `sysconf(_SC_PAGESIZE)`-derived or byte-granular rule reds (host-independent by
  construction); and the **opacity + bounds** case — no raw address in any Ken
  value or token, out-of-range `mapBytes` / `mapWrite` is a fail-visible
  `ResourceError`, a wrong-kind token is `ResourceKindMismatch` naming `Mapping`,
  and `ReadOnly` refuses `mapWrite`. **The MAP_PRIVATE COW case is OUT of
  D5a-surface's scope and STAYS RED, keyed `BLOCKED-ON-ABI-S6-D5b`.** It is
  intrinsically file-backed — "writes through a file mapping do not reach the
  backing file" has NO anonymous analog — so discharging it needs
  `MappingAcquireFile`, the fourth op D5a-surface's frozen wire forbids; it is
  discharged by D5b, NOT here, and its discriminating assertions are NOT weakened
  or deleted in the interim. Control: neutering either achievable discriminator
  (host-page accounting, address exposure, unchecked view) reds the matching case;
  a case that passes green-vs-green against its named non-conforming implementation
  is vacuous and does NOT discharge. The seed text is not altered to make a case
  pass — a case that only passes after weakening its control is a HARD STOP.
  **Honesty of the gated axis (capability-gate lifecycle).** While the COW case is
  dormant behind D5b, the live opacity/bounds case and every case's
  native==interpreted parity assertion keep opacity, bounds-not-clamp, and parity
  enforced across the whole D5b interval — the axis has a live enforcer throughout,
  never an unguarded gap.

## Gate, reviewers, sequencing

`gate: none` (node field). The change is in the host trust boundary (`ken-host`)
plus the prelude and the interp reify path, so the MERGE carries TCB at Steward
routing. Adding operations and a backend trait method to the existing resource
model is the authorized ABI-completion program (§4), NOT a new TCB grab — but the
region/lifetime/opacity design is soundness-bearing, and D0 is a genuine
representation fork. On each candidate: **runtime-leader owns the merge Decision;
Architect required review** (D0 shape ruling first; then verify opacity is
absolute, bounds and origin binding hold, lifetime is revocation-governed, unmap
and revocation-close interact safely, and no new TCB-growing primitive beyond the
resource model + one backend method was introduced) **+ Runtime QA** on the exact
SHA, then Steward M1-M4 -> lieutenant. **No Decision is required to RELEASE** this
node (the frame is the Steward's, the program is operator-authorized); the merge
Decision is assembled from the Architect + Runtime QA votes on each candidate.
Runtime owns the WP; Foundation collaborates on the Ken-visible view types
(§4 records Runtime + Foundation), but a WP has a single owner.

## Contention

`crates/ken-host` (`effect_v1.rs`, `lib.rs`, `abi_v1.rs`, `effect_wire.rs`,
`capability.rs`, `revocation_v1.rs`, `effect_abi_v1.catalog`,
`effect_abi_probe.c`), `crates/ken-elaborator/src/prelude.rs`, the
`crates/ken-interp` reify path, and downstream consumers in `ken-verify` and
`ken-elaborator` export/erasure. Re-measure at pickup. Measured contention-free
at `2dbe90af7`: `ken-host` / `ken-runtime` are idle post-ABI-S1; the leftover
`wp/ABI-S1-*` and `wp/ABI-A2-metadata-lt-respin` branches are spent squash-source
remnants of merged nodes (ABI-A2 status `merged`, its branch tip not an ancestor
of main — publisher squashes), NOT live work. Lane 3 (foundation) is on
`catalog/` Tier-D — disjoint. Lane 2 (language) is stood down. The concurrent doc
track (`library/`, `agent/`) is disjoint.

## Hard stop

Route to the Steward if: the D0 measurement shows neither reuse nor a parallel
kind can meet the boundary without a new TCB-growing primitive beyond the
resource model plus one backend method (that needs operator authorization); a
second distinct capability right is required after the last `RightSet` bit is
consumed; or opacity / bounds / lifetime cannot be held without exposing an
address to Ken. Any of those means the work as framed is not what the tree needs,
not that scope should bend. The D0 representation fork itself is a hard-stop to
the ARCHITECT (not the Steward) — it is a design ruling, not a scope question.

The D4 mapping capacity-governance fork (AC-MAPPING-CAPACITY-GOVERNANCE:
per-mapping / invocation-wide limit versus an explicit unbounded-but-graceful
contract) is likewise a hard stop to the ARCHITECT, before D4 implementation. It
is a security-relevant ABI-shape ruling — like the D0 fork, a design decision,
not a scope question — and D4 lands nothing on that axis until the Architect
rules it.
