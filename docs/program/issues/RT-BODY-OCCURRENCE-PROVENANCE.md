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
origin: Architect standalone mechanism ruling evt_172ag7hdbttkc (2026-08-09), selecting the correction for the authority named by RT-JOIN-ORIGIN-ATTRIBUTION's merged record (exact 72e0d7c, PR #1686). Steward-filed and framed on that bound per COORDINATION §2. AMENDED AND RE-RELEASED 2026-08-09 on ruling evt_5ncj9jd6fjt8f, which withdrew the StaticBodyTarget carve-out after the AC-2 integration gate measured the identical alias in that class; the relation is now keyed over plan.entries UNION every StaticBody target. No merge verdict transfers across the re-release.
---

> # THE TRAVERSAL CALL IS NOT MISSING. THE IDENTITY PASSED TO IT IS WRONG.
>
> ### THE PAIRING AUTHORITY IS ONE, AND IT COVERS EVERY SEED CLASS
>
> **RE-RELEASED 2026-08-09 on ruling `evt_5ncj9jd6fjt8f`, which withdrew the
> original `StaticBodyTarget` carve-out.** The first attempt implemented the
> bounded contract correctly and then **measured that it was insufficient** —
> `9e5b8d7f` is **held evidence, not a completed candidate, and must not be
> published.**
>
> **This node's text is the ruled contract, not the originally released one.**
> The relation is now keyed over `plan.entries` **UNION** every `StaticBody`
> target, issued at **two** seats. **No merge verdict transfers** across this
> re-release; fresh QA and Architect review are required.

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

## Required representation — ONE relation over the UNION population

**Amended 2026-08-09 on ruling `evt_5ncj9jd6fjt8f`.** The original text scoped
the relation to scheduling-entry registration and carved out `StaticBodyTarget`
as already-grounded. **That carve-out was withdrawn** — see the two-class block
below. What follows is the ruled contract, not the released one.

Planning must **atomically record the pair it already has**, over one relation
whose exact key population is

> **`plan.entries` UNION every `StaticBody` target.**

```rust
struct PlannedEntryBody {
    entry: StaticNodeId,
    body_occurrence: StaticOriginId,
}
```

An exact **closed `entry -> body_occurrence` table** inside
`StaticTransitionPlan` is an equivalent representation **if changing the
existing `entries` shape would churn frozen topology.**

**Populated ONLY from a returned `PlannedExpr`**, at exactly two issuance seats:

1. **Top-level registration, unchanged.** One returned `PlannedExpr` atomically
   records `entry -> occurrence` while registering `entry`. Root registration
   records `root.entry -> root.occurrence`; each transparent-declaration
   registration records `planned.entry -> planned.occurrence`.
2. **`StaticBody` edge registration, NEW.** Replace **each bare `StaticBody`
   edge write** with one generic registration operation taking the returned body
   `PlannedExpr`, which atomically emits `source -> body.entry` as
   `EdgeKind::StaticBody` **and** records `body.entry -> body.occurrence`.

**Seat 2 is an issuance seat and the ruling grounds it as one.** In both
`RuntimeExpr::Closure` and `RuntimeExpr::LexicalClosure`, `plan_expr(body, ...)`
returns the complete `PlannedExpr`, so at the statement that emits the
`StaticBody` edge the planner **already holds all three** of `node.entry`,
`body.entry` and `body.occurrence`. Selecting child 0 later, scanning the
source-occurrence graph, using `origin_of(edge.to)`, or recovering an outer
resume all **discard an already-issued identity and reconstruct it.** Forbidden.

> ### BOTH REGISTRATION HELPERS MUST BE GENERIC
>
> **Neither may mention `ComputationalMatch`, `SourceReturnResume`, or
> expression shape.** If your helper names any of them, you have written the
> special case this ruling exists to avoid.

**This recorded pair is the SOLE entry/body pairing authority for every seed
class.** `root_occurrence` and `declaration_occurrences` may remain **only** as
derived views or **keyed-equality** checks against it — set membership is
neither. `declaration_origins` remains a membership projection and is **never**
a pairing authority.

**A later `D2a` exclusion of a declaration-owned scheduling entry from the
emitted-function population does NOT remove its top-level registration row or
its keyed declaration projection.**

### At function-unit construction

- **every** function seed — `SchedulingEntry` **and** `StaticBodyTarget` —
  obtains its body occurrence **only** from this relation;
- **retire the `StaticBodyTarget` fallback `StaticOriginId(edge.to.0)`.**
- `PredeclaredFunction.planned_node` remains the **scheduling-entry** axis;
- its currently aliased `origin` becomes the **issued body-occurrence** axis,
  preferably renamed `body_occurrence`;
- that field flows through the existing `AbiDescriptor`, `EmittableUnit` and
  `OwnedUnitEmission` carrier chain;
- `define_unit_body` **always** resolves and lowers the carried body occurrence.
  **Remove the root-only body-origin substitution.**

> ### THE CARVE-OUT WAS WRONG, AND WHY IT IS THE SAME DEFECT TWICE
>
> ~~The already-grounded `StaticBodyTarget` seed classes continue to consume
> their defining child-0 occurrence relation~~ — **WITHDRAWN by ruling
> `evt_5ncj9jd6fjt8f`. There is NO distinct child-0 authority.**
>
> Child-0 coincides with the body for ordinary expressions and **diverges for
> `ComputationalMatch`** — the identical two-axis split, in a second seat class.
> **Measured on held evidence `9e5b8d7f`, venue 4:** owner 2, seed `n58`
> `Evaluate`, issued body **SOI(58)**, actual body **SOI(26)** — a
> `SourceReturnResume` with five `SourceReturnOwnedResume` edges converging on
> it. The attribution's entered-list contains `58` and **none** of
> `{26, 33, 39, 53}`.
>
> **`9e5b8d7f` is HELD EVIDENCE, NOT A CANDIDATE. It must not be published.**
> It implements the bounded contract correctly, carries the three Architect
> corrections, and then **measures that the contract was insufficient.** That is
> the correct outcome of a bounded contract, not a failure to follow it. It may
> be retained as held partial work; **no merge verdict transfers.**

**This is an internal provenance correction.** It adds no ABI slot, changes no
callable signature, and changes no function-unit population, owner partition,
call identity, traversal call, or closeout equation.

## Fail-closed validation — exact over the UNION

The planner must **reject**: a missing, extra, or duplicate key over
`plan.entries ∪ StaticBody targets`; a `StaticBody` target whose key is not its
**exact** edge target; a body occurrence absent from the source-occurrence
table; two **emitted function seeds** claiming one body occurrence; and a
selected body occurrence owned by a different function unit. Root and
declaration projections are **keyed equality** checks.

**Validation must not re-walk completion edges or reconstruct a body by graph
shape.**

Two exactness defects the Architect found in `9e5b8d7f`'s predecessor are
**already corrected there and must survive the widening** — carry them forward
rather than re-deriving them:

- `source_occurrences` is `Vec<Option<PlannedOccurrence>>` and an **in-range
  `None` is a control node with no source term.** A `.get(index).is_none()`
  test rejects only out-of-range ordinals, so `Some(None)` passes; the
  semantic-descriptor check does not close it, because descriptors exist for
  control nodes too and a same-owner control node satisfies the owner check.
  Use `.get(..).and_then(Option::as_ref)` with an **in-range-`None`** mutation.
- The declaration view must compare **each symbol's exact pair**, not set
  membership — swapping two declarations' body occurrences preserves the set.
  Its control must assert the swap preserves the value set **before** asserting
  the refusal, so it pins keyed equality rather than being discharged by the
  predicate it replaced.

**One consumer-axis defect is likewise already corrected and must not
regress:** `DeclaredUnitCall.origin` names the **callee scheduling entry** and
is filled from `edge.callee_origin()`, so `validate_declaration_unit_call` must
compare **`entry_origin()`**, not `body_occurrence()`. Current callable rows
coincide on both axes, which is exactly why the existing split-axis edge test
does not discriminate that site. **Coincidence is not authority.**

## Forbidden — BINDS BOTH SEED CLASSES

**The original forbidden list binds `StaticBodyTarget` exactly as it binds
`SchedulingEntry`.** This is one authority extended to the full planned-entry
population, **not a parallel `StaticBody` ledger.**

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

## Acceptance

| AC | criterion |
|---|---|
| `AC-1` | The `SchedulingEntry` nested construction control pins `n18 -> n5` and proves `n10` is **not** registered as that unit's body |
| `AC-1b` | A **`StaticBodyTarget` control pins `n58 -> SOI(26)`** and proves **`SOI(58)` is not** that unit's body |
| `AC-2` | The exact `LiftRose` synthetic witness **reaches and closes** the owner-2 required set `{26,33,39,53}`, measured on venue 4 |
| `AC-3` | A **body-to-entry collapse mutation** recreates the existing traversal/closeout failure, and **discriminates BOTH classes** — see below |
| `AC-3b` | The two carried-forward validation mutations red: an **in-range `None`** control node, and a **two-declaration body swap** that preserves the value set |
| `AC-4` | Root and healthy-sibling topology, function population, join population, ownership, selection, call identity and ABI signature all unchanged |
| `AC-5` | The synthetic exact-witness control is carried **by this candidate**; the **first post-Kernel closure candidate** owns the committed runnable form once nested-inductive admission is on `main` |
| `AC-6` | Workspace green **in CI** |

> ### `AC-3` — A GLOBAL MUTATION DOES NOT PROVE THE NEW ARM
>
> A collapse mutation that **reddens first through the `SchedulingEntry` class**
> says nothing about `StaticBodyTarget`. The `StaticBodyTarget` arm needs a
> **class-selective or exact-row** mutation that collapses **that row itself**.
> This is the A/B discipline the chain already pays elsewhere: the informative
> side is the one that would green if the arm were absent.

**`AC-2` IS THE GATE THAT CAUGHT THE CARVE-OUT, AND IT IS NOT RELABELLABLE.**
The prior candidate labelled an unrelated non-root split-axis planning test as
`AC-2`; the node's table is the authority. Return the exact composed
object/tree and the **measured closed-set result**. Do not claim current-`main`
execution, and do not turn the `AC-5` ignored placeholder into a fake green —
that placeholder's panicking body is **correct** and is what keeps un-ignoring
it red rather than vacuous.

**`AC-5` is the deferred control finally getting an owner.** It was deferred at
the attribution node with a named release condition; this is where the first
half lands and where the second half is assigned.

## Size — M, RECONFIRMED not inherited

**The population widened; the remaining work did not.** `9e5b8d7f` already
carries the `SchedulingEntry` class complete, QA-approved on its predecessor
and with three of four Architect findings fixed. The increment this amendment
adds is bounded and enumerable: **one** new issuance seat (the generic
`StaticBody` registration operation), **one** fallback retirement
(`StaticOriginId(edge.to.0)`), a validation domain widened to the union, and
**one** discriminating control plus its class-selective mutation. `AC-2`'s
venue is already built and reproducible.

**M holds.** If `D1` measures a third seed class, or the union key population
turns out not to be exactly `plan.entries ∪ StaticBody targets`, **STOP AND
ROUTE** — that is a third instance of the same defect and it re-sizes the node
rather than being absorbed.

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
