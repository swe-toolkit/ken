---
id: RT-JOIN-ORIGIN-ATTRIBUTION
title: "A planner-required join origin is neither traversal-consumed nor structurally dispositioned, and the set difference does not say which of three authorities is wrong"
status: merged
owner: runtime
size: S
gate: none
depends_on: [RT-CONTINUATION-EDGE-DISPOSITION]
blocks: [KERNEL-NESTED-IND]
github: null
origin: Architect ruling evt_j8t0ktxbmck (2026-08-09) on the KERNEL-NESTED-IND scope hard stop. Component attribution is Runtime; Kernel must not edit crates/ken-runtime for this wall. Steward-filed and sequenced after #6i's D3 because it contends on all four D3 paths (COORDINATION §2).
---

> # CLOSED 2026-08-09. ALL FOUR ACs MET, RECORD-ONLY, ON `main`.
>
> Verified against **its own ACs**, not on the code having landed:
>
> - **`AC-1`/`AC-2`** — the record merged at exact `72e0d7c`, PR #1686, and
>   names the first missing general authority as **the ordinary body-emission
>   traversal** (the Architect's option 1), argued from SOI(26)'s source-table
>   resolution, owner, token and continuing-predecessor data, `required
>   {26,33,39,53}` with **empty** `consumed` and `dispositioned`, absent
>   traversal *and* selection for that owner, and **healthy sibling owner
>   closeouts**. The siblings are what make it an attribution rather than an
>   observation.
> - **`AC-3`** — `crates/` byte-identical to base `f0217c67`; one path,
>   `+407/-0`. No production change.
> - **`AC-4`** — returned at the boundary, and the Architect ruled
>   `evt_172ag7hdbttkc` on it rather than the ring proceeding to a repair.
>
> **The successor it produced is [[RT-BODY-OCCURRENCE-PROVENANCE]]**, which is
> where the repair lives. This node never authorized production work and did
> not do any.

> # THIS IS AN ATTRIBUTION NODE. IT DOES NOT BEGIN WITH A REPAIR.

**`KERNEL-NESTED-IND` IS BLOCKED ON THIS**, and it is the only thing blocking
it. The Kernel ring is held with its erasure projection and a durable RED
preserved, and it may **not** work around this in its own crate.

## What it is

At `finalize_join_disposition`, Runtime derives `required` from the planner's
owner-bound `required_join_origins(function)`, computes
`covered = consumed_join_origins ∪ dispositioned_join_origins`, and raises its
error from `required \ covered`.

**The measurement establishes exactly one thing:** SOI(26) is planner-required
and neither traversal-consumed nor structurally dispositioned.

> ### THE SET DIFFERENCE DOES NOT DISTINGUISH THREE AUTHORITIES
>
> 1. a **semantically reachable** join whose lowering traversal failed to call
>    `enter_source_occurrence_plan`;
> 2. a **statically unselected** join whose `Match`/`If` selection failed to
>    disposition its owner-bounded subtree; or
> 3. a join assigned to the **wrong planner owner/population**.
>
> **The terminal message is not enough to choose a sound correction**, which is
> why this node exists and why it is not a repair.

## Deliverable — ONE, and it is test-only

A **Runtime-owned causal checkpoint on the exact Kernel witness, with NO
production repair.**

1. Resolve SOI(26) through the planner's **sole** origin-to-expression table.
   Record its `RuntimeExpr` kind, semantic function owner, parent/child path,
   and — if under `Match`/`ComputationalMatch`/`If` — the exact enclosing
   branch/case.
2. Record its join token representation and continuing-predecessor bit, the
   closing function, and the exact `required`, `consumed`, and `dispositioned`
   membership for SOI(26).
3. Correlate the source traversal: whether that occurrence is entered; every
   enclosing static selection; the reached-case union used by
   `close_statically_unselected_match_cases`; and the subtree membership that
   would disposition it.
4. **Name the FIRST MISSING GENERAL AUTHORITY, not the final set difference.**
   Reachable ⇒ the correction belongs at the general traversal route. Dead ⇒ at
   the exact general selection/disposition seat. Wrong ownership/population ⇒ in
   planning.

**Return at that boundary for the mechanism ruling before changing production.**

> ### FORBIDDEN — all four make the equality green while establishing nothing
>
> Consuming SOI(26); inserting it into the dead set; deleting it from
> `required`; special-casing the origin.

## Acceptance

| AC | criterion |
|---|---|
| `AC-1` | All four measurements recorded against the exact Kernel witness |
| `AC-2` | The **first missing general authority** is named, with its classification argued from the trace rather than from the set difference |
| `AC-3` | **No production change.** Test-only, and `crates/` behaviour unchanged |
| `AC-4` | Returned at the boundary for a mechanism ruling; no repair attempted |

## Contention — re-derive at kickoff

**Sequenced deliberately after [[RT-CONTINUATION-EDGE-DISPOSITION]]'s `D3`**,
which touched `lowering/core.rs`, `lowering/mod.rs`, `lowering/units.rs` and
`core/tests/control.rs` — **the likely diagnostic surfaces here.** `D3` merged at
`b5aa079b`, so the branch is free; **re-derive contention against the
then-current tree rather than trusting this sentence.** A disjointness claim of
the Steward's died twice on 2026-08-08, once on the other lane's repair.

**The Architect stated plainly: no claim of path or semantic disjointness
exists.** Treat overlap as the default.

## After the correction merges

Kernel rebases the retained work and re-runs the exact differential control:
interpreter Nat 3, native present, verifier passed, native Nat 3. Until then
[[KERNEL-NESTED-IND]] is held, not merge-ready.
