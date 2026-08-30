---
id: RT-CHECKED-IH-CARRIED-ENVIRONMENT-FORCE-CALL-D1
title: "Scratch-only D1 prototype (no candidate, no QA, no Decision, no merge): prove the typed producer-local role split that makes the checked-IH carried environment and the checked-IH application result non-interchangeable. Retire the ambiguous two-role LoweringOperand-returning API (call_checked_ih_transport_from_case_environment's CarriedEnvironment arm) and introduce two private compiler-local result types — CheckedIhCapturedEnvironment (used only by ConstructArgument) and CheckedIhApplicationResult (used only by call_lowered after the eligible pending checked-IH application and exact CheckedIhEnvironmentTransport are selected) — with role-specific entry points carrying no runtime tag, storage, or Ken-visible identity. For an already-carried environment the application entry point emits ONE exact new direct force call through call_declared_unit_target and returns only its trap-checked Result as CheckedIhApplicationResult; the environment-materialization crossing stays a no-call. Pair the new call's Result to the natural match 451 and show a real ResourceBodyOk/ResourceBodyErr selection WITHOUT synthesizing that constructor at the environment producer. Restore the branch byte-clean and return a report."
status: ready
owner: runtime
size: M
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "Architect review evt_4m8eyrrpke50k (thr_tfcm3sp107x9, 2026-08-30), accepting the runtime-implementer's RT-HOST-APPLICATION-TRAP-PROVENANCE-D0 outcome `wrong scrutinee before the match` (report evt_5z6e6yge3pw9n, SHA-256 86b978f2a61bbb..., scratch-diff aa939332175a...). The D0 proved the live ResourceBodyResult match 451/450 in funcid42 receives the identity-free eight-field checked-IH captured environment (specialization 1, seat 671, record 608) returned inline by call_checked_ih_transport_from_case_environment's CarriedEnvironment arm — not the Host response and not the checked-IH application result. The shared untyped LoweringOperand return type makes environment material and application result interchangeable: correct for ConstructArgument (install an already-materialized environment in the ruled recursive field), wrong for call_lowered's selected checked-IH force (which needs the result of APPLYING the worker). Architect-selected repair: a typed role split at the producer-local substitution seam, funded as this scratch-only D1 before any production build. Steward-recut per COORDINATION section 2. Base origin/main 3d8fd27c8696a24d9fec254e6e520f8fef6923a2; core.rs blob eea98dc6ddb0ae2f7656b16fed7ee461b24de0a1 and source.rs blob c39f82e7854f626244b4398ba9941ae38b25485e both Steward-verified byte-identical to the reviewed base 0be25235b at this main tip. @steward owns close/reframe/release; runtime parked until this fresh D1 kick."
---

> # READY — SCRATCH-ONLY D1. Released to the runtime ring (lane 1) on `origin/main`
> # `3d8fd27c8`. Runtime is parked; this IS the release.
>
> This is a PROTOTYPE node. It lands NO production, opens NO candidate, routes NO
> QA, and needs NO Decision or merge. It may prototype ONLY the typed split, and
> the branch is restored byte-clean at the end. The Architect reviews the D1
> report and its scratch diff, and a YES enables a later production recut.
>
> **Why we are here.** `RT-HOST-APPLICATION-TRAP-PROVENANCE-D0` is CLOSED with the
> accepted outcome **wrong scrutinee before the match**: the live match 451/450
> receives the checked-IH captured environment (identity-free, eight fields, record
> 608) that `call_checked_ih_transport_from_case_environment`'s `CarriedEnvironment`
> arm returns inline as the scrutinee. That arm's shared untyped `LoweringOperand`
> return makes environment material and application result interchangeable — correct
> for one caller, wrong for the other. **The structural response is a typed role
> split at the producer-local substitution seam — not a third result protocol, not
> a global replacement of the carried-environment arm, not a tag/store/scan.**
> Measure the exact seams at this base before touching anything; coordinates below
> name functions/records/planner roles, not frozen line numbers.

## Exact base and the ambiguous arm this D1 splits (Architect `evt_4m8eyrrpke50k`)

Base `origin/main` `3d8fd27c8696a24d9fec254e6e520f8fef6923a2`. The blobs the split
touches are byte-identical to the reviewed D0 base `0be25235b`:

- `core.rs` blob `eea98dc6ddb0ae2f7656b16fed7ee461b24de0a1` holds the ambiguous
  arm in `call_checked_ih_transport_from_case_environment`:
  ```rust
  Some(LoweringEnvironmentBinding::Value(LoweringOperand::Carried(word))) => {
      if !self.continuation_candidate_is_consumed(&identity) {
          self.settle_continuation_candidate(
              &identity,
              super::units::CandidateDisposition::InlineNoCall,
          )?;
      }
      return Ok(LoweringOperand::Carried(*word));
  }
  ```
- `source.rs` blob `c39f82e7854f626244b4398ba9941ae38b25485e` holds
  `SourceContinuation::ConstructArgument`, which uses that arm to install an
  already-materialized environment in the ruled recursive field. `call_lowered`
  uses the SAME untyped result for a selected checked-IH force, where the required
  value is the result of applying the worker.

**If main advances before release, recheck those two blobs (and the aggregate/
planner/test blobs the split reads) before the runtime picks this up.**

## Selected design (Architect — the D1 prototypes exactly this, no more)

1. **Retire the ambiguous two-role API. No boolean, no mode enum.** Introduce two
   private compiler-local result types — one `CheckedIhCapturedEnvironment`, one
   `CheckedIhApplicationResult` — and two role-specific entry points. They carry no
   runtime tag, no storage, and no Ken-visible identity.
2. **Environment-materialization entry point — used ONLY by `ConstructArgument`.** A
   static worker may emit the existing environment-producing call; an
   already-carried environment returns the exact word without another call. Its
   result type cannot flow into source-machine `Value` as an application result.
3. **Application entry point — used ONLY by `call_lowered`** after the existing
   eligible pending checked-IH application and the exact
   `CheckedIhEnvironmentTransport` have been selected. It consumes a move-only force
   authority containing the pending
   application coordinate and the exact transport. It never accepts
   `CheckedHostVisContinuation`; the existing eligible `OrdinaryApplication` /
   `CheckedHostComputationTail` boundary remains.
4. **For a static worker, preserve the existing exact call assembly.** For an
   already-carried environment, resolve only `transport.source_record()`, prove it
   is the `CheckedIhCapturedEnvironment` record for the same source owner and seat,
   require the runtime word to have Constructor class, no tag, and exactly the
   record's child count, and project each child once in planner ordinal order.
   Class, tag, and count are INTEGRITY CHECKS after exact transport selection, never
   selectors.
5. **Reconcile every projected ordinal and source origin against the unit's
   `WorkerCapture` envelope.** Assemble nonrecursive fields from the selected case
   environment and continuation inputs from the existing domain-tagged morphism.
   Then call ONLY `transport.source_call_identity()`'s declared target, exactly
   once, through `call_declared_unit_target`.
6. **Record that force call in `checked_ih_transport_emissions`; retain the
   finished-CLIF callee closure in `verify_emitted_continuation_calls`.** Return
   only the call's trap-checked Result as `CheckedIhApplicationResult`, through the
   unchanged `RoutedAnswer::checked` continuation. The environment word is never a
   semantic result.
7. **The earlier `funcid50 -> funcid47` call that materializes the captured
   environment and the later force call are DIFFERENT ROLES, not a duplicate.**
   Count and pair them by the move-only application/transport coordinate — never by
   callee count, adjacency, or equal totals.

## D1 gate — a YES must prove ALL of these on the exact checked program

- Identify the live pending application kind and invocation/application origins
  BEFORE consuming it.
- Show one carried force authority produces one exact new direct call, while the
  environment-materialization crossing remains a no-call for an already-carried
  word.
- Pair the new call's trap-checked Result to the natural same match 451 and show a
  real `ResourceBodyOk` or `ResourceBodyErr` selection WITHOUT synthesizing that
  constructor at the environment producer.
- Match the reference exit status, terminal-error absence, exact four effects
  (`FsOpen -> BufferAllocate -> ResourceRelease(Buffer) -> ResourceRelease(FsHandle)`),
  and buffer-before-file release order.
- Preserve every existing Result/Trap ordering and all prior C/I/E/S,
  source/member/projection/direction/delivery, sink, and inertness controls.

## Controls (per property, at natural production sites)

- Reintroducing the direct environment return must reproduce the exact
  `ResourceBodyResult` diagnostic.
- Suppressing or duplicating the force call must leave an outstanding or duplicate
  move-only force refusal.
- Synthesizing `ResourceBodyOk` at the environment producer must NOT satisfy the
  force obligation.
- Wrong source record/owner/seat, wrong runtime field count, and shifted/dropped/
  duplicated capture ordinals each need distinct exact refusals; wrong callee stays
  covered by finished-CLIF decoding; wrong continuation-input domain/index and
  Result-before-Trap retain their existing independent controls.
- Each negative needs its own restored positive and executed-count evidence.

## Closed axes remain closed (do NOT)

Do not tag the environment, add storage, scan for a seat, key on scalar identity/
proximity/count, return environment material as a result, synthesize the expected
constructor, weaken `ResourceBodyResult`, reinterpret Trap, reorder Result/Trap,
reapply the Host-Vis continuation, or reopen the spent caller-success
`ApplicationResultToRet` seat. This D1 is the already-selected checked-IH force at
the producer-local substitution seam, NOT a third result protocol.

## Outcome routing

- **D1 YES** — return the scratch diff, the pairing evidence, the per-property
  controls with red/green evidence, and a byte-clean restoration. Enables a later
  production recut (the Steward reframes the production node from the YES).
- **D1 NO** — return the exact obstruction WITHOUT widening. Do not attempt a
  remedy, reopen a closed axis, or design a carrier; hand the obstruction to the
  Architect.

## Reviewers, sequencing, contention

- **Reviewer:** the Architect reviews the D1 report and its scratch diff and selects
  the production recut per the outcome routing. No Runtime QA (scratch-only
  prototype, no product), no Conformance Validator, no Decision, no publisher CI
  (nothing lands). Like a prior D0/D1 measurement node, this is never `merged`.
- **Sequencing:** runtime ring (lane 1), single continuous turn to a complete YES/NO
  report or a genuine blocker. Size M, tier T1 (typed-role invention across the
  generated-unit substitution seam). Restore the branch byte-clean at the end
  regardless of outcome.
- **Contention:** reads and scratch-prototypes
  `crates/ken-runtime/src/cranelift_backend/lowering/{core,source,calls}.rs`, the
  generated-unit aggregate/planner, and the `rt_parity_native.rs` evidence
  wrappers; produces no landed change, so no crate/catalog contention with the
  concurrent lanes. Targeted builds ONLY via `scripts/ken-cargo` scoped to
  `ken-runtime`, never `--workspace`.
