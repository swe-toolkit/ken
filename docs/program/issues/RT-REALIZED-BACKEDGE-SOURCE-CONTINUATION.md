---
id: RT-REALIZED-BACKEDGE-SOURCE-CONTINUATION
title: "Route the realized RecursiveBackedge protocol marker where it reaches the source-machine ordinary-Match continuation -- the honest hard-stop residual left after the checked-IH realization authority landed"
status: merged
owner: runtime
size: S
gate: none
tier: T1
depends_on: [RT-CHECKED-IH-REALIZATION-AUTHORITY]
blocks: [RT-NESTED-IH-NATIVE-REALIZATION]
github: null
origin: "Steward-filed 2026-09-07 (COORDINATION §2) on runtime-leader's request evt_4ee23xjg9q79a, from the accepted-partial hard-stop residual measured and reviewed on RT-CHECKED-IH-REALIZATION-AUTHORITY (merged e68ecd79 / code d29cc8ad0; implementer report evt_4f55thkpb21eg, Runtime QA evt_5cwezyz9w6ype, Architect approval evt_7zmy8t68brxg2). The withdrawn NativeJoinPlanV1 join-marker finding (evt_kvryke1ay21n) was prototype-induced and is NOT this residual -- see the correction evt_35gx7szdwj6s5 / evt_4y65zhtwrfpe6."
---

## MERGED 2026-09-08 as squash `897f1ea6a` (PR #3419) -- accepted partial, its one authorized forward delivered.

**Candidate `f53b081376257de563d7adea207892970e38e6f3`** (branch
`wp/RT-REALIZED-BACKEDGE-SOURCE-CONTINUATION`), landed as squash `897f1ea6a`
(lieutenant M9 `evt_1a3k3e9jvf7rp`). Base `5a875142b` was three commits behind
after PX9-INC2A landed; clean rebase, empty §14(5) intersection; all 3 paths
blob-verified byte-identical pre/post-rebase and against landed `origin/main`.
CI fully green. Decision `dec_59ab5qb2yztcs` resolved; Runtime QA + Architect
(required mechanism reviewer) `evt_7w6jmp2r8x128` on exact SHA. Adversary M8b
`evt_4hd1scyggb9q2` NO OBJECTION (provenance airtight, oracle discriminating and
non-vacuous, honest accepted-partial). Steward M0-M4 `evt_36hat6zc5zter`.

**What landed:** the realized `RecursiveBackedge` protocol marker now FORWARDS at
the source-machine ordinary-Match continuation (`source.rs`) -- guard
`matches!(value, Specialized(Lowered::RecursiveBackedge))` hoisted to the top of
the seat, forwarded via `RoutedAnswer::forward(value, incoming_route,
incoming_role)` before `enter_source_occurrence_plan`, both routing axes
preserved. Strictly narrow: only `RecursiveBackedge` is carved out; every other
`Specialized(_)` still refuses verbatim. Near-verbatim port of the D2a
`ComputationalMatchScrutinee` arm. Mints no authority, consumes no occurrence
plan, selects no case.

**Accepted-partial closeout -- this node is DONE, not superseded.** The frame's
only authorized deliverable (the forward) landed. Native execution then ADVANCES
to a new named refusal at the source-join ownership frontier
`crates/ken-runtime/src/cranelift_backend/lowering/joins.rs:2140`
(`source join StaticOriginId(50) was classified outside its owning function`),
fenced by a panicking `Ok(artifact)` arm that forbids spurious completion. Per
the Architect ruling (`evt_7w6jmp2r8x128`) and its AC-PARENT-FRONTIER, that
frontier is **`RT-NESTED-IH-NATIVE-REALIZATION`'s D3-D5 territory, not a reason
to keep this successor active** -- it is the parent's next measured ownership
frontier, a fresh mechanism question routed to the Architect on
[[RT-NESTED-IH-NATIVE-REALIZATION]]. The historical residual + successor
question below stand as the record of what this node delivered.

> # Tracker node, NOT a shovel-ready WP frame.
>
> This records the honest hard-stop residual and its successor question. A
> `docs/program/wp/` frame (deliverables, ACs + controls, fixed inputs at a
> named SHA, forbidden list, contention check) is authored AFTER the Architect
> rules the routing mechanism (§2c front-load). Do not release on this file.
> Runtime is idle-pending that ruling (accepted, short) -- runtime-leader has
> stated the ring will take the framed slice as soon as it exists.

## The measured residual (verified on the landed accepted partial)

RT-CHECKED-IH-REALIZATION-AUTHORITY minted the compiler-owned checked-IH
realization authority: on the real nested source, both checked-IH calls now
realize under the intended 1-frame / 2-slot / 2-call plan (call 0 -> slot 0,
call 1 -> slot 1, each parented by frame/segment 0). That authority is CHECKED
and landed; the seat still refuses, one edge further on -- which is exactly the
accepted-partial shape the frame required (authority minted and observed, wall
still standing).

The exact next operand, at the source-machine production specialized arm in
`crates/ken-runtime/src/cranelift_backend/lowering/source.rs`, is a realized
`RecursiveBackedge`. It is refused with the verbatim production message
`Match: scrutinee is not a constructor value`.

The stop is proven to have ADVANCED (not merely "stopped returning None") by a
non-degenerate pair on the same seat, byte-identical error message, distinguished
by site + operand:

- base / unplanned twin: `(SourceMachineSelector, ComputationalRecursorClosure)`
  -- the capsule is unrealized when it reaches the site.
- planned twin, after both realizations: `(SourceMachineSelector, RecursiveBackedge)`
  -- the capsule is realized; the operand has advanced to the backedge.
- an independent generic control records `(GenericExpressionSelector,
  ProcessExitStatus)`, distinguishing the two production emitters.

## The successor question (for the Architect -- routed with this filing)

A fully checked, realized `RecursiveBackedge` protocol marker now reaches the
source-machine ordinary-Match continuation. Routing that marker was explicitly
NOT authorized under the authority node. The mechanism question:

**How is a realized `RecursiveBackedge` to be lowered where it reaches the
source-machine ordinary-Match continuation, soundly and without widening the
ordinary-Match selector?** i.e. what is the correct continuation for a realized
backedge (recursive re-entry / tail edge / other), what checked provenance must
carry it, and where is the boundary against the routes the authority node kept
closed.

## Forbidden boundary inherited from the authority node

Whatever the routing mechanism, it must NOT (these stayed closed on A and gate
this successor too): widen the ordinary-Match selector or its catch-all; inspect
`.residual`; mint or accept an unchecked / caller-authored plan; collect join
markers (NativeJoinPlanV1 is withdrawn); read terminal-All / KERNEL-NESTED-IND
provenance; introduce a new `Lowered` / `LoweringOperand` variant or a carrier
conversion; or edit RT-TERMINAL-ALL scope.

## Why this node exists (constraint grounding, §4c)

It is the sole remaining live frontier between the landed checked-IH realization
authority and `RT-NESTED-IH-NATIVE-REALIZATION` completing its native
realization (D3-D5) -> `KERNEL-NESTED-IND` -> `DS-9`. This is the
operator-prioritized runtime kernel chain (operator ruling evt_7nkzsy27p7npw:
"Go with A, unblock the kernel chain then continue with ABI B"). The residual is
a measured capability gap witnessed by the checked realization, verified by
Runtime QA and the required Architect mechanism review -- not an aesthetic or
safety-of-main constraint.
