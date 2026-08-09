---
id: RT-BODY-OCCURRENCE-PROVENANCE
title: "Non-root function seeds alias the scheduling entry as the body origin, so the source traversal enters the entry and never reaches the real body occurrence or its join subtree"
status: ready
owner: runtime
size: M
gate: none
depends_on: [RT-JOIN-ORIGIN-ATTRIBUTION]
blocks: [KERNEL-NESTED-IND]
github: null
origin: Architect standalone mechanism ruling evt_172ag7hdbttkc (2026-08-09), selecting the correction for the authority named by RT-JOIN-ORIGIN-ATTRIBUTION's merged record (exact 72e0d7c, PR #1686). Steward-filed and framed on that bound per COORDINATION §2.
---

> # THE TRAVERSAL CALL IS NOT MISSING. THE IDENTITY PASSED TO IT IS WRONG.
>
> ### THE PAIRING AUTHORITY IS ONE, AND IT COVERS EVERY SEED CLASS
>
> **Ruling `evt_5ncj9jd6fjt8f` withdrew the original `StaticBodyTarget`
> carve-out.** The first attempt implemented the bounded contract correctly and
> then **measured that it was insufficient** — `9e5b8d7f` is **held evidence,
> not a completed candidate, and must not be published.** Read that block in
> the construction section before implementing anything.

**[[KERNEL-NESTED-IND]] IS BLOCKED ON THIS AND ON NOTHING ELSE.**

## The defect, exactly

`define_unit_body` obtains a body origin and calls `lower_expr`, which
immediately calls the **sole** general `enter_source_occurrence_plan(static_origin)`.
That route exists and fires. What is wrong is **which identity reaches it**.

- `plan_expr` already returns the two lawful axes as
  `PlannedExpr { entry, occurrence }`.
- For ordinary expressions the axes **coincide**.
- For `ComputationalMatch` they **deliberately differ**: `entry` schedules the
  scrutinee; `occurrence` is the source-bearing `SourceReturnResume`.
- **Non-root function seeds currently set
  `PredeclaredFunction.origin = StaticOriginId(seed.0)`, which merely aliases
  the scheduling entry.**
- Ordinary non-root emission passes that alias as the body origin, so the source
  traversal **enters the scheduling entry and never enters the real body
  occurrence or its join subtree.**

⇒ **The correction belongs upstream at scheduling-entry registration and is
consumed by ordinary body emission. DO NOT ADD A SECOND TRAVERSAL CALL.**

## Required representation

Planning must **atomically record the pair it already has** at the visits that
register scheduling entries:

```rust
struct SchedulingEntryBody {
    entry: StaticNodeId,
    body_occurrence: StaticOriginId,
}
```

An exact **closed `entry -> body_occurrence` table** inside
`StaticTransitionPlan` is an equivalent representation **if changing the
existing `entries` shape would churn frozen topology.**

**Populated ONLY from the returned `PlannedExpr`:** root registration records
`root.entry -> root.occurrence`; each transparent-declaration registration
records `planned.entry -> planned.occurrence`.

> ### THE REGISTRATION HELPER MUST BE GENERIC
>
> **It must not mention `ComputationalMatch` or `SourceReturnResume`.** If your
> helper names either, you have written the special case this ruling exists to
> avoid.

**This recorded pair is the SOLE scheduling-entry/body pairing authority.**
`root_occurrence` and `declaration_occurrences` may remain **only** as derived
views or equality-checked projections. `declaration_origins` remains a
membership projection and is **never** a pairing authority.

### At function-unit construction

- every `SchedulingEntry` seed consumes its **exact recorded** body occurrence;
- ~~the already-grounded `StaticBodyTarget` seed classes continue to consume
  their defining child-0 occurrence relation~~ — **WITHDRAWN by ruling
  `evt_5ncj9jd6fjt8f`. THE CARVE-OUT WAS WRONG.** `StaticBodyTarget` seeds
  consume the **same issued `entry -> body_occurrence` pairing**; **there is no
  distinct child-0 authority.** Child-0 coincides with the body for ordinary
  expressions and **diverges for `ComputationalMatch`** — the identical two-axis
  split, in a second seat class. Measured on held evidence `9e5b8d7f`: owner 2,
  seed `n58` `Evaluate`, issued body **SOI(58)**, actual body **SOI(26)**;
- `PredeclaredFunction.planned_node` remains the **scheduling-entry** axis;
- its currently aliased `origin` becomes the **issued body-occurrence** axis,
  preferably renamed `body_occurrence`;
- that field flows through the existing `AbiDescriptor`, `EmittableUnit` and
  `OwnedUnitEmission` carrier chain;
- `define_unit_body` **always** resolves and lowers the carried body occurrence.
  **Remove the root-only body-origin substitution.**

**This is an internal provenance correction.** It adds no ABI slot, changes no
callable signature, and changes no function-unit population.

## Forbidden

Do **not**: select the "outermost" resume by graph traversal; infer the body
from completion-edge shape, owner scans, expression shape, origin arithmetic, or
first-match rules; mint a body claim at `push_node(SourceReturnResume)` — **that
site runs once per nested match and does not know which match is the unit
boundary**; add another `enter_source_occurrence_plan` call; or alter join
populations, owner partitions, call identities, selection or disposition laws,
ABI signatures/populations, or closeout equations.

> ### WHY, IN ONE FIXTURE — issued identity versus graph inference
>
> The frozen nested fixture has one scheduling entry `n18` and two resumes, `n5`
> and `n10`. **The outer `plan_expr` call already returns the exact
> construction-time pair `n18 -> n5`**; `n10` is only the occurrence returned by
> the nested call. **Recording the returned pair is issued identity. Recovering
> "outermost" later is graph inference** — and it is the same shape as every
> narrowing this chain has already refuted.

## Fail-closed validation

The planner must **reject**: a missing or duplicate pair for a scheduling entry;
a pair whose key is not in the exact scheduling-entry population; a body
occurrence absent from the source-occurrence table; two function seeds claiming
one body occurrence; and a body occurrence owned by a different function unit.

**Validation must not re-walk completion edges to reconstruct the answer.**

## Acceptance

| AC | criterion |
|---|---|
| `AC-1` | The nested construction control pins `n18 -> n5` and proves `n10` is **not** registered as that unit's body |
| `AC-2` | The exact `LiftRose` synthetic witness reaches and closes the owner-2 required set `{26,33,39,53}` |
| `AC-3` | A **body-to-entry collapse mutation** recreates the existing traversal/closeout failure |
| `AC-4` | Root and healthy-sibling topology, function population, join population, ownership, selection, call identity and ABI signature all unchanged |
| `AC-5` | The synthetic exact-witness control is carried **by this candidate**; the **first post-Kernel closure candidate** owns the committed runnable form once nested-inductive admission is on `main` |
| `AC-6` | Workspace green **in CI** |

**`AC-5` is the deferred control finally getting an owner.** It was deferred at
the attribution node with a named release condition; this is where the first
half lands and where the second half is assigned.

## Measurement basis — no second trace is required

**Ruled: no re-measurement on current `main` before framing.** The attribution
trace was **not** taken on Kernel's held parent alone — its synthetic venue
combines held Kernel `dd3cd050` with Runtime `f0217c67` and its projection
snapshot, and verifies `D3Event` is present in `lowering/units.rs`. **So the
classification was measured against the `D3` Runtime** while remaining honestly
non-`main` because Kernel admission was held.

The Architect re-grounded the live tree: the merged record's blob on
`origin/main` is byte-identical to `72e0d7c`, and `lowering/core.rs`,
`lowering/units.rs`, `planning/static_transition.rs` and
`planning/static_transition/abi.rs` have **no delta** from `f0217c67` to current
`origin/main`. **Remeasuring an unavailable-on-`main` Kernel witness against
byte-identical Runtime machinery would add no discrimination.**

## Scope

**This does not reopen [[RT-JOIN-ORIGIN-ATTRIBUTION]]**, which is closed as
record-only at merged exact `72e0d7c` and never authorized production work.

**After this merges independently**, Kernel rebases the retained work and re-runs
the exact differential: interpreter Nat 3, native present, verifier passed,
native Nat 3.

**If the correction cannot be made at the registration seat without touching a
forbidden surface, STOP AND ROUTE** rather than widening. Eight authorities on
this chain have each been resolved by routing.
