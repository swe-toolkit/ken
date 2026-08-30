---
id: RT-HOST-APPLICATION-TRAP-PROVENANCE-D0
title: "Scratch-only provenance D0 (no production, no candidate, no QA, no merge): answer ONE question before any further caller-result/sink design is chosen — why does exact generated context body StaticOriginId(941) select the TRAP lane at the ResourceBodyResult match instead of producing the semantic application result? Identify the exact failing match by stable source/planner coordinates; classify the actual no-case scrutinee's semantic role and structural provenance (Host response / continuation-application result / environment material / nested declared-call result / other) from the producer, NOT from numeric value or proximity; trace it across each generated-unit boundary using the existing separate Result and Trap ABI slots (which lane written, which consumed, exact producer/consumer coords — aggregate seven-equals-seven is not pairing); establish the nearest same-match positive; run two separate scratch controls (change only the scrutinee producer to the expected constructor and require selection; suppress only the expected case and require the same no-case arm); reconcile the reference path on the identical checked program; restore the branch byte-exactly to base and return a report/digests only."
status: ready
owner: runtime
size: M
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "Architect ruling evt_56t3wm78ax81h (thr_3ht16h248rzqk), 2026-08-30, accepting the runtime-implementer's D1 feasibility NO on RT-HOST-RESPONSE-APPLICATION-RET-BUILD (hard-stop report eed27c1451c09d2c..., probe diff 5cea81fd4d8bd9..., read log 5a1ebce449..., baseline context CLIF 3d90e7e333...). The recut proved the current product NEVER produces a caller-visible successful result for the exact continuation application: the seven CheckedHostVisContinuation applications in funcid45 (binder-zero HostResult, generated context body StaticOriginId(941), direct Computational frame 12) all trap internally — the callee selects a real typed trap lane (planned PatternMatchFailure at ResourceBodyResult) — so ApplicationResultToRet cannot be instantiated (no successful result to carry) and the late direct edge fails Cranelift dominance (inst1269 from non-dominating block5). RT-HOST-RESPONSE-APPLICATION-RET-BUILD is CLOSED as a sound NO. This scratch-only D0 answers the provenance question the NO exposed, before another production design is chosen. Steward-recut per COORDINATION section 2. Binds released base/branch 0be25235b188bc67b3f9209d1ff0b6f8fa063258, tree 769c24708fb2052c3d6e719a8adc135423c28192 (WP ref resolves exactly there, zero diff). @steward owns close/reframe/release of the D0; runtime parked until this fresh release."
---

> # READY — SCRATCH-ONLY D0. Released to the runtime ring (lane 1). Runtime was
> # parked at base `0be25235b`; this IS the release.
>
> This is a MEASUREMENT node. It lands NO production, opens NO candidate, routes
> NO QA, and needs NO Decision or merge. It returns a report plus digests, and the
> branch is restored byte-exactly to base `0be25235b` at the end. The Architect
> reviews the D0 report and chooses the next production design from its outcome.
>
> **Why we are here.** RT-HOST-RESPONSE-APPLICATION-RET-BUILD is CLOSED as a sound
> NO: the current product never produces a caller-visible successful result for the
> exact continuation application — the callee TRAPS (real typed trap lane, planned
> `PatternMatchFailure` at `ResourceBodyResult`) before any successful
> application-result edge. A private carrier cannot carry a value this execution
> never produces. This is the SAME broader predicate recurring: a static
> phase/result identity is standing in for a semantic value role, and every prior
> production frame designed a caller-result/sink seat before proving which callee
> outcome lane the product actually reaches. **The structural response is
> provenance at the failing consumer — not a third result seat.** Do not design a
> carrier, retry the success seat, treat trap as suspension, weaken
> `ResourceBodyResult`, forward environment material, reapply `k`, or alter
> result/trap ordering. Measure first.

## The one question this D0 answers

Why does exact generated context body `StaticOriginId(941)` select the TRAP lane
at the `ResourceBodyResult` match instead of producing the semantic application
result?

## Exact base and coordinates (Architect `evt_56t3wm78ax81h`)

Binding released base/branch `0be25235b188bc67b3f9209d1ff0b6f8fa063258`, tree
`769c24708fb2052c3d6e719a8adc135423c28192`. The generated-unit ABI already has the
protocol this D0 measures against — do NOT propose adding one:

- Exact `calls.rs` blob `fa010fed973dfa8cb638c3a2a546594b93443efb` already has
  separate `AbiSlotKind::Result` and `AbiSlotKind::Trap` offsets.
  `call_declared_unit_target` checks call status, reads the TRAP slot first,
  propagates a nonzero trap word through the enclosing trap lane, and reaches the
  result block only when no trap exists.
- On this execution the callee selected a REAL typed trap lane: planned
  `PatternMatchFailure` at `ResourceBodyResult`. That is not an absent block
  parameter and must not be reclassified as suspension merely because a later
  result would be useful. The normative `drive_H (Vis e k) = drive_H (apply k (H
  e))` loop does not authorize turning a native `PatternMatchFailure` into a
  successful application result.

Re-measure the exact seams at this base; coordinates below name functions/planner
roles, not frozen line numbers.

## Required D0 evidence (transcribed from the ruling — each is mandatory)

1. **Identify the exact failing match by stable source/planner coordinates**, not
   only `funcid50` or planned scalar 36: declaration, match origin, generated
   context, enclosing specialization, selected frame, and the complete
   case/default inventory.
2. **Classify the no-case scrutinee's semantic role and structural provenance.** At
   the natural no-case branch, record the constructor identity/classification the
   match EXPECTS, the ACTUAL carrier origin, and the last compiler producer.
   Classify the actual scrutinee as one of: Host response, continuation-application
   result, environment material, nested declared-call result, or another exact
   role. Do NOT infer from numeric value or proximity.
3. **Trace the scrutinee across each generated-unit boundary** using the existing
   independent Result and Trap slots. For EVERY physical boundary, record which
   lane was written, which lane the caller consumed, and the exact
   producer/consumer coordinates. Aggregate seven-equals-seven is NOT pairing.
4. **Establish the nearest same-match positive:** an unchanged source-derived
   execution that reaches the same `ResourceBodyResult` consumer and selects a real
   `ResourceBodyOk` or `ResourceBodyErr` case. If no such execution exists, REPORT
   the missing positive rather than borrowing another match.
5. **Two separate scratch controls** (each must restore byte-identically):
   - preserve the case inventory while changing ONLY the scrutinee producer to the
     expected constructor, and require selection;
   - preserve the scrutinee while suppressing ONLY its expected case, and require
     the same natural no-case arm.
   These diagnose producer-word failure versus case-planning failure.
6. **Reconcile the reference path on the identical checked program:** name the
   expected `ResourceBodyResult` constructor and the effect/`Ret` continuation it
   enters. No new expected behavior may be invented from native output.
7. **Restore the branch exactly to `0be25235b...`; return a report/digests only.**
   No production edit, candidate, QA, or D2.

## Exhaustive outcome routing (the report must land in exactly one)

- **Wrong scrutinee before the match** — name the exact producer/transport edge and
  return for a producer-local design. Environment material remains ineligible.
- **Correct scrutinee but absent/wrong case** — the defect is match
  planning/selection, not return transport.
- **Actual semantic application result exists at an inner physical completion and is
  later dropped** — identify that completion plus every operand a context-local sink
  needs; ONLY then may the already-authorized private carrier be reconsidered.
- **A non-error suspension is genuinely encoded in the trap lane** — prove that from
  the producer and existing protocol before requesting an explicit internal-outcome
  redesign. `PatternMatchFailure` itself is NOT that proof.

## Prohibitions

No production edit, no candidate, no mutation net beyond the two named scratch
controls, no QA route, no Decision, no merge. Do not retry the success seat, treat
trap as suspension, weaken `ResourceBodyResult`, forward environment material,
reapply `k`, add a third result seat, or alter result/trap ordering. Do not infer
the scrutinee's role from numeric value or proximity. Do not invent expected
behavior from native output. Restore the branch byte-exactly to base at the end.

## Reviewers, sequencing, contention

- **Reviewer:** the Architect reviews the D0 report and its digests and chooses the
  next production design per the outcome routing. No Runtime QA (scratch-only
  measurement, no product), no Conformance Validator, no Decision, no publisher CI
  (nothing lands). This is a measurement node — like a prior D0 that returned NO, it
  is never `merged`.
- **Sequencing:** runtime ring (lane 1), single continuous turn to a complete report
  or a genuine blocker. Size M, tier T1 (deep provenance reasoning across the
  generated-unit boundary). Restore base at the end regardless of outcome.
- **Contention:** reads/scratch-probes `crates/ken-runtime/src/cranelift_backend/
  lowering/{calls,core,source}.rs` and the generated-unit planner; produces no
  landed change, so no crate/catalog contention with the concurrent lanes. Targeted
  builds ONLY via `scripts/ken-cargo` scoped to `ken-runtime`, never `--workspace`.
