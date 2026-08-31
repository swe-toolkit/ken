---
id: RT-LIVE-K-FUTURE-INPUT-OWNERSHIP-D0
title: "Scratch-only live-k future-input ownership map D0 (no production candidate, QA, Decision, CI, publication, or merge): HS12 was discharged with the STAGE-THE-APPLICATION design ruling (evt_5fczfzysqyca8) — app486/u0:53 is an environment-production/escape seat, not an execution seat; the repair is a typed staged-application boundary-closure where the later response-owner consumes the staged authority ONCE through call_static_worker_with_inputs. Production is NOT yet authorized because the exact later response owner is unproved. This D0 maps, over the SAME WRITE witness (app486/u0:53, template4, StaticWorkerBinding closure1246/body1238/arity1/captures7, route GeneratedContext, discharge DirectSpecializationCall, transport None) and WITHOUT changing production, five things: (1) the exact planner/source identity of the arity-one semantic response argument and the instruction/value that supplies it; (2) the exact later Function and application occurrence that OWN that argument; (3) how app486's capture environment dominates that seat or reaches it through explicit declared parameters; (4) the existing stable planner relation, if any, joining app486/template4/closure1246/body1238/route GeneratedContext to that later seat; (5) the current-Function target declaration the existing emitter would consume there. Controls independently DROP, DUPLICATE, and VARY each join coordinate and distinguish no-relation from ambiguity (HostResult, capture words, adjacency, matching ABI shape, first/only target, body-only lookup, and runtime tags receive NO credit). OUTCOME: a unique planner-issued future response/control edge proven -> return the exact map for an implementation frame under the staged design; NO unique edge -> stop with the confident negative that the semantic application is infeasible under the no-runtime-object constraint (neither an enum nor a join may manufacture the missing authority). Any other outcome is a hard stop to the Architect. Scratch-only; reuses the accepted runtime net at pickup 978b05dd29 (recheck the seven blobs); restore byte-clean; NEVER merged."
status: ready
owner: runtime
size: L
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "Architect HS12 post-advisory ruling evt_5fczfzysqyca8 (thr_443f4v7keb6tg, 2026-08-31), grounded on the mandatory HS12 Research advisory evt_7jm10b0q0f5ht and the HS12 hold evt_6rbxfaz48f9n2. The ruling binds released head 960263e6c888474c07609cbdfaacdd484d931307 (tree f926f8374719aad35bfe05e10ff83868c40e1b20) and accepted runtime pickup 978b05dd29ab1a40cdc5b89e4410bd3928cff111. Outcome B was confirmed at live app486/u0:53: the exact StaticWorkerBinding closure1246/body1238/arity1/captures7 carries route=GeneratedContext / discharge=DirectSpecializationCall / transport=None, and materialize_checked_ih_static_worker_application reads only arity/captures and returns the captured-environment word, leaving the route unused. RULING: STAGE the application; do NOT call at app486. app486 supplies zero explicit operands while the worker declares arity one, and route=GeneratedContext selects HOW a complete call executes, not the missing argument; per the runtime contract (spec/40-runtime/42-evaluation.md sections 6.2 and 6.4, k : Resp e -> ITree ..., response exists only after H e, apply k resp once in tail position) app486 is an environment-production/escape seat. The required representation is a typed staged-application boundary-closure: at app486 consume the checked marker into a compiler-only staged-application authority + a typed CheckedIhCapturedEnvironment (ordered captures only, no code pointer/tag/route tag/ABI discriminator/callable identity); keep ONE planner-issued stable descriptor beside it binding occurrence/template4 -> closure1246/body1238, declared arity, ordered captures, GeneratedContext route, discharge, destination owner; do NOT carry StaticWorkerBinding (function-local SSA captures) or a FuncRef across Functions — at the later owner project the environment fields under the descriptor, form a fresh current-Function binding with the preserved route, resolve that Function's own local target table; only the owner holding the planner-authorized response operand consumes the staged authority, passing exactly that one operand to call_static_worker_with_inputs (the sole owner of arity checking, capture ordering, GeneratedContext suffix, route-selected target table, call emission, Trap-before-Result); the emitter's returned result — not the environment word, not a ConsumedHere label — discharges the staged authority. call_boundary_closure_environment is precedent for descriptor-plus-environment reconstruction but is NOT the final emitter here (its unconditional worker_calls lookup would bypass StaticWorkerCallRoute). Production is blocked until this future-input ownership proof; the Architect authorizes the implementation frame only on a proven unique edge. Steward owns this D0 recut and Runtime release sequencing. Scratch-only doc recut per COORDINATION section 2; the doc commit advances current origin/main and the accepted runtime-net product blobs are unchanged (recheck the seven at pickup 978b05dd29)."
---

> # READY — SCRATCH-ONLY LIVE-K FUTURE-INPUT OWNERSHIP MAP D0. Released to the
> # runtime ring (lane 1). Runtime is parked; this IS the release.
> #
> # MEASUREMENT node. It lands NO production candidate, opens NO PR, routes NO QA,
> # needs NO Decision or merge. It reuses the accepted runtime net at exact pickup
> # `978b05dd29` (recheck the seven accepted blobs), returns a report plus a scratch
> # diff, source/CLIF/address maps, runtime logs, an evidence manifest, and digests,
> # and restores the branch byte-clean. The Architect ALONE reviews the report and
> # either authorizes an implementation frame or accepts the confident negative.
> # Like every prior D0 in this chain, it is never `merged`.
> #
> # **Why this recut exists (Architect HS12 post-advisory ruling
> # `evt_5fczfzysqyca8`).** HS12 is discharged: WRITE is B at app486/u0:53 (a typed
> # `StaticWorkerBinding` with `route=GeneratedContext` whose execution authority is
> # dropped at the materialization seam), and the design ruling is **STAGE the
> # application** — app486 is an environment-production/escape seat, not an
> # execution seat, so the arity-one worker must be consumed later, by the owner
> # holding the planner-authorized response operand, through the EXISTING
> # `call_static_worker_with_inputs` emitter. That staged design is only
> # IMPLEMENTABLE if a unique later response owner exists. **This D0 proves or
> # disproves that owner. It changes NO production.**

## The WRITE witness this map is anchored on (do NOT re-derive it)

- Live producer seat: **app486 / u0:53**, checked template **template4**,
  `OrdinaryApplication`.
- Selected worker: **`StaticWorkerBinding`** closure**1246** / body**1238** /
  arity **1** / captures **7**; **route `GeneratedContext`**, discharge
  `DirectSpecializationCall`, transport `None`.
- The source occurrence supplies **zero explicit operands**; the worker declares
  **arity one**. The missing arity-one operand is the **semantic response
  argument** — it is NOT the route, a capture, the captured-environment word,
  body identity, HostResult, target-table membership, or an empty vector.

## What this D0 must identify — five coordinates, WITHOUT changing production

1. **The response argument's origin.** The exact planner/source identity of the
   arity-one semantic response argument, and the exact instruction/value that
   supplies it.
2. **The later owning seat.** The exact later Function and application occurrence
   that OWN that argument.
3. **Environment reachability.** How app486's capture environment **dominates**
   that seat, or reaches it through **explicit declared parameters** (not by
   adjacency, not by shared ABI shape).
4. **The stable planner relation.** The existing stable planner relation, **if
   any**, joining app486 / template4 / closure1246 / body1238 / route
   `GeneratedContext` to that later seat. "If any" is literal — its absence is a
   first-class finding (see the confident negative below).
5. **The later target declaration.** The current-Function target declaration that
   the existing `call_static_worker_with_inputs` emitter would consume at that
   later seat.

## Controls — drop, duplicate, and vary each join coordinate

- For **each** join coordinate (response-argument origin, later owner, the
  environment-domination/parameter edge, the stable planner relation, the later
  target declaration), independently **drop**, **duplicate**, and **vary** it,
  and confirm the map **distinguishes no-relation from ambiguity** (a dropped
  coordinate must read as absence; a duplicated one as ambiguity; a varied one as
  mismatch), restoring the exact positive after each.
- **No credit** is given to HostResult, capture words, adjacency, matching ABI
  shape, first/only target, body-only lookup, or runtime tags. A join asserted on
  any of those is `CONTROL_NO_SUBJECT`, not a proven edge.

## The two ruled outcomes

- **Unique edge PROVEN** — a unique planner-issued future response/control edge
  joins app486's staged authority to a single later owning seat that supplies the
  arity-one operand. Return the **exact map** (all five coordinates, each with its
  drop/duplicate/vary control) for an **implementation frame under the staged
  design** (typed `CheckedIhCapturedEnvironment` + stable descriptor; the later
  owner consumes the binding once through `call_static_worker_with_inputs`).
- **No unique edge** — no unique planner-issued future response/control edge
  exists. **Stop with the confident negative:** the semantic application is
  **infeasible under the no-runtime-object constraint**; neither a typed enum nor
  a staged join may manufacture the missing authority. This is a terminal
  feasibility finding for the Architect (and operator), not a defect to repair.

**Any outcome other than these two is a HARD STOP to the Architect** (for
example: an edge that exists but is not unique in a way the controls cannot
resolve to ambiguity, or a later owner whose argument fails the
environment-domination test). Do not select a production representation, do not
mint a call, do not force a join.

## Preservation

Preserve the accepted WRITE witness facts, per-instance markers, adjacent
positives, exact effect order, Trap-before-Result, the app486/u0:53
StaticWorkerBinding record, and disabled-CLIF byte identity. Do not synthesize
`ResourceBodyOk`, treat environment as result, borrow HostResult, reapply `k`,
reify a runtime closure/tag/continuation, add a call at app486, or reorder
effects. This map is a READ of the planner/lowering state; it emits and consumes
nothing new. **Restore byte-clean.**

## Deliverable and the gate

A report plus a scratch diff, source/CLIF/address maps, runtime logs, an evidence
manifest, and digests — restored byte-clean — handed to the Architect, who alone
reviews it and either authorizes an implementation frame under the staged design
(on a proven unique edge) or accepts the confident negative (no unique edge).
Runtime production stays blocked until this future-input ownership proof is ruled.
Scratch-only, reuses accepted runtime pickup `978b05dd29` (recheck the seven
blobs), NEVER `merged`.
