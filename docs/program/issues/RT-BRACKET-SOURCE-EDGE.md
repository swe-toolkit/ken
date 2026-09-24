---
id: RT-BRACKET-SOURCE-EDGE
title: "Bracket redesign D0: mark the checked bracket's body-to-settlement sequencing edge at the authenticated bind producer as a typed explicit Control IR edge, instead of assigning AcquiredBody through a Ret payload; measure it on the px8ta nested-bracket artifact, STOP and park if the source-to-control tie cannot survive normalization and lowering"
status: ready
owner: runtime
size: M
gate: architect
tier: T1
depends_on: []
blocks: []
github: null
origin: "Operator 2026-09-24 ~03:50Z, 'option (a).': authorize a redesign of bracket provenance after the KERNEL-NORMALIZE-ORIGIN-TRACE Ret STOP (Architect evt_10ygybsnbap2q). Design basis: Architect evt_2bx08mhm8d8gd, corrected evt_7e0btf5130y87. Representation (explicit Control IR) unchanged from the operator's 2026-09-22 ruling. Steward-filed per COORDINATION section 2."
---

# Mark the edge, not the Ret

## Objective

Each checked bracket carries a typed, authenticated edge from its body's
completion to its own settlement continuation, so the lowering can release
nested brackets in strict LIFO without guessing which settlement belongs to
which body. Target row, unchanged:
`px8ta_oriented_subcontinuation.rs::public_two_three_level_brackets_finish_and_release_lifo`.

## Fixed inputs -- Architect `evt_2bx08mhm8d8gd` at held child 1

- At held child 1 `4b4c8565c543c148039ea06ac798c7c91eda2ce8`,
  `compiler_driver.rs::transform_bracket_producer_term` marks `AcquiredBody`
  on checked `bind` argument 4 (the whole body ITree) and `FinalSettlement`
  on argument 5's continuation body.
- The kept Ret observation (`evt_3cnvyjxn746fk`) is `ITree g214 / Ret g215`,
  marked only at its whole scrutinee root. `g215` is the selected Ret
  constructor. The source `bind` is the checked bracket producer whose body
  normalizes to that Ret and whose argument 5 is the independent settlement
  continuation. Carrying the Ret-root mark through `f x` into the later Vis
  would misstate source identity. The source role must change, not the
  `Ret` reducer.
- Held inputs, never moved, rebased, deleted or landed here: child 1
  `4b4c8565c`, child 2 checkpoint `21c039918`, kept kernel branch
  `7f1a04a40`.

Treat anchors as perishable. If a fixed input is false on the landed base,
stop and report the mismatch; do not build around it.

## Deliverable -- D0, on a fresh branch from current main

Record the body-completion exit (Ret success or error, and controlled trap
where it applies) to *that bind's own* settlement-continuation entry as a
typed explicit Control IR edge. Index it by exact source occurrence plus
call/return context and parent region. Authenticate it from the checked
`bind` arguments **before** normalization and keep the tie to the lowered
control edge. Do not discover it afterward by `HostOpV1`, first-match
origin, shared marker spelling or structural resemblance.

Acquire, effect-bearing FinalSettlement, OutcomeMerge and Resume stay bound
to their own checked producer points. Never move a whole-Ret role onto
`f x`, `g586` or `g216`. The body is a region, not a host response.

Runtime owns this node. A kernel change is not in scope. If D0 shows the
authentic Acquire/FinalSettlement sites need the kept narrow causal Vis
observation, report that; the Steward frames that slice separately for the
kernel ring on a fresh main-based branch. Never land the kept Ret rule.

## Acceptance -- measured on the actual px8ta checked artifact

- **AC-1.** For the source `bind` tied to the selected `g215` Ret event,
  record: (i) one authenticated source edge with its own body and
  continuation identities and context; (ii) which normalized and lowered
  settlement effect belongs to that continuation; (iii) whether the matched
  body completion stays an edge source without asserting a Ret-field origin;
  (iv) whether two nested contexts sharing a template stay distinct.
- **AC-2 (negative controls).** A swapped continuation, a duplicate edge and
  a missing edge each refuse.
- **AC-3.** `px7f_resource_native::linked_public_escape_is_exact_closed`
  keeps exactly one host response route.
- **AC-4.** Targeted builds only, through `scripts/ken-cargo`. No-regression
  means green in CI.

D0 does not claim depth-2 LIFO or the depth-3 `ContinuationSpecialization`
refusal fixed. Only after D0 establishes the typed edge does an
implementation WP target each separately.

## Stop conditions -- STOP means park

Stop and park if any of these holds:

- the source-to-control tie cannot be carried through normalization and
  lowering without a fabricated Ret descendant;
- two contexts collapse to one identity;
- a FinalSettlement port is guessed from a host-op shape;
- the guard still sees duplicate host routes;
- a proposed observer changes the normalized Term.

Do not move to child-2 assembly or the depth-2/depth-3 execution gate on a
partial D0. No guard suppression and no second host response route.
