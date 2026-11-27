---
id: RT-STATIC-UNSELECTED-CALL-DISPOSITION
title: "The native planner over-approximates which continuation calls lowering emits, and joins reconcile that by a statically-unselected disposition, but the continuation-call population has none: every consumer that requires a planned call to be emitted, discharged or resolved refuses, one plane at a time, on a call that lowering lawfully never emits. A HostIO proc whose guard folds statically to its Ret arm, with a Vis producer in the other arm, fails the native build. Give the call population one statically-unselected disposition that every consumer reads"
status: active
owner: runtime
size: L
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Architect ruling evt_45863hkv60etq on RT-FS-DESCRIPTOR-SEEK-LENGTH-SYNC's second overflow stop: entries 1 and 2 share one predicate, and the fix is a Runtime design, not a fold. Steward scope (a)+(b), evt_6gyt3n9jqxe08: DESCRIPTOR proceeds with the native boundary named, and this prerequisite carries the native row. Moved to the Runtime ring (L1) and kicked evt_2c2xekw0ygyh9 when PX11 Inc2 held on a provider flag (evt_64pjjqr32rmtg)."
---

# One statically-unselected disposition for planned continuation calls

## Objective

A planned continuation call whose producer lies in code that lowering
statically excludes is disposed as statically unselected, once, by one
relation. Every consumer of the call, result-edge, obligation and
specialization populations reads that relation. The overflow literal
from DESCRIPTOR builds natively and returns `InvalidOffset`.

## Settled inputs (DESCRIPTOR respin `b0d7b3938`; Architect `evt_45863hkv60etq`)

- **The predicate.**
  - The planner's case-producer flow treats a PrimitiveCall result as
    Open (`derive_case_producer_fact`, `closure.rs:483`, verdicts
    `:553-600`).
  - Lowering folds Bool primitives over known operands
    (`core/primitive.rs:457-459`), so it selects statically more often
    than the planner sees.
  - Joins reconcile that gap by disposition
    (`disposition_statically_unselected_source_subtree`). Calls have no
    such form.
- **Entry 1, built.** Commit `8cee29d96` is the join half: a least
  fixpoint in `close_statically_unselected_match_cases` that treats a
  planned call as dead when its producer construct is in the owner
  subtree of a case the final selection excludes. With it, the literal
  moves past the origin-710 join refusal.
- **Entry 2.** The detached-result seat,
  `eliminate_detached_producer_continuation` (`core.rs:13096`), keeps the
  dead call as a residual edge (`:13100-13143`) and refuses at
  `:13290-13297`. It clears an edge only when the edge is emitted,
  composed-discharged or claimed.
- **Further consumers named by the Architect, not yet closed by
  mechanism.**
  - The O-law, `units.rs:9460` (resolved = declared = planned).
  - The D2 close, `units.rs:9875-9993` (discharged = claimed =
    call_obligations).
  - The composed-discharge verification, `mod.rs:8549` (D8k). Its
    claim-then-verify ordering is the pattern the seat follows.
- **Reproduction.** The literal `18446744073709551616` in DESCRIPTOR's
  `setLength` row, with log `/workspaces/ken/tmp/fs-descriptor/overflow/`
  and red R1 on `wp/RT-FS-DESCRIPTOR-SEEK-LENGTH-SYNC-r2` at `b0d7b3938`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

- Base: origin/main after DESCRIPTOR lands. The first commit is
  `8cee29d96`, unchanged.
- Compute the dead-call relation once, at the generated-function
  closeout, from the fixpoint `8cee29d96` already computes.
- Each consumer reads it:
  - the join closeout;
  - the detached-result seat, which records a pending unselected claim
    and verifies it after closeout;
  - D2, where unselected is a third discharge form, disjoint from the
    other two;
  - the O-law, stated over the live population;
  - any other consumer the census finds.
- Out of scope: planner precision, which stays an over-approximation,
  and DESCRIPTOR's surface.

## Acceptance

- **AC-0 (Architect, before code).** Start with a census by mechanism:
  every consumer across `crates/ken-runtime/src` of the continuation-call,
  result-edge, obligation and specialization populations that requires
  emission, discharge or resolution. Then rule:
  - each consumer's unselected form;
  - whether a specialization whose only callers are dead is emitted
    harmlessly or excluded;
  - the arms, registered before the build.
- **AC-1.** R1: DESCRIPTOR's overflow literal returns `InvalidOffset`
  with no `FsSetLength` event, interpreter against native, in both
  real roots.
- **AC-1b.** R2: a HostIO proc that is not a descriptor operation, whose
  guard folds statically to its Ret arm and whose other arm is a Vis,
  builds natively and returns the Ret value.
- **AC-2.** One mutation per consumer arm, so that removing any
  consumer's unselected form reddens R1 or R2. Plus M-b from
  `evt_6k4h2m6hc5249`: with the predecessor contribution dropped, the
  D5a witness (`core/tests/specialization_binding.rs:16`) reddens.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- A consumer needs the planner itself to become precise, rather than a
  disposition.
- A dead call's producer lies in another owner. That is a cross-function
  edge, and it needs its own ruling.
- §1a: the chain carries at 2 from DESCRIPTOR, so the first stop here is
  the third, and it triggers a research pull.

## Inventory

1. The native join closeout's recursive-predecessor union counted a
   continuation call whose producer is in excluded code
   (`evt_6k4h2m6hc5249`).
2. The detached-result seat treated the same dead call as owed by the
   unit result (`evt_45863hkv60etq`).
3. statically-unselected relation: R2i's known-Bool `if` guard (interp/native Ret, exit 23, empty trace) recorded zero roots at the disposition_statically_unselected_source_subtree choke point and zero settlements; the relation's If-seat arms (core.rs:4322, :17723) have no witnessing producer — keyed on the guard's surface form (`if` vs match) reaching a lowering seat. (evt_38n77hdbscbj0)
4. M-settle's registered observation named candidate totality, but
   response-owner coverage (F5) reads the same settled disposition and
   closes first in close_continuation_claim_ledger, so R1 refused there and
   M-settle and M-resp were not separated on R1 — keyed on the close order
   across the consumers of one disposition (evt_e689rjqtpjb1)

§1a reached 3 at entry 3, and research advised (`evt_38g8yeaf3k7q`); the
Architect ruled (`evt_4wahmne7eeh60`). §1a reached 4 at entry 4, ruled
`evt_293131gty98bg`. The next research trigger is the 6th; the next §1b
answer falls due at the 6th entry.

Checks: 2 fired. The guarded property is "a planned call lowering never
emits is disposed, not refused". Its fan-in is every population whose
close requires emission, which is why AC-0 starts with a census by
mechanism and not by name. 4 fired. The anchors come from the
Architect's reading at `b0d7b3938`, and `8cee29d96` is built. 8 fired.
A fix at the seat alone would pass R1 until the D2 or O-law close
refuses the same call. So AC-2 mutates each consumer separately.
