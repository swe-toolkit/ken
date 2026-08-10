---
id: RT-BACKEND-MODULE-SPLIT
title: "Split the oversized ken-runtime backend files into modules — the follow-on to the recursive-descent retirement, not an interlude in it"
status: draft
owner: runtime
size: M
gate: none
depends_on: [RT-DESCENT-RETIRE]
blocks: [NATIVE-HANDLE-CARRIER]
github: null
origin: Operator directive 2026-07-31 — the ken-runtime backend files are oversized again; a previous interlude of this shape produced the cranelift_backend/ directory. Operator asked whether to repeat it now or after the campaign, and confirmed AFTER on the Steward's recommendation. Campaign docs/program/16-recursive-descent-retirement.md §4 node #8. Steward-filed (agents cannot create tracked work per COORDINATION §2).
---

> ## DELIBERATELY UNFRAMED UNTIL [[RT-DESCENT-RETIRE]] MERGES
>
> This node is `draft` on purpose, and it must **not** be flipped `ready`
> before the capstone lands. [[RT-DESCENT-RETIRE]] **deletes** the classifiers,
> `RecursiveDescentResidual`, `BodyEmissionAuthority::RecursiveDescent` and the
> whole recursive-descent emission lane across exactly the files this node
> splits. **The deletion changes where the natural module seams are**, so a
> frame written now would be sized against a tree that is about to disappear.
>
> ⇒ The frame is owed **after** #7 merges, measured on the **post-retirement**
> tree. Do not carry today's line counts into it.

> ## THE ENCLAVE PASS IS RUNNING NOW. THE RULE ABOVE IS UNCHANGED.
>
> **Operator instruction, 2026-08-10: frame this with the Architect, now.**
> Pass anchored at `evt_104nz8cedzyat`.
>
> **This does not relax the banner above, because the pass and the frames are
> different artifacts.** What the modules *mean*, the WP cut, which domain
> moves first, and the IR triage are architecture decisions with no dependency
> on the retirement's deletion. The **census and the sizing** are Stage A and
> must still be taken on the post-retirement tree.
>
> ⇒ **No WP releases before #7 merges.** The pass produces the cut; the frames
> are written from it; the measurements in them are re-taken after the capstone.
>
> Running it now is also the only part of #8 that does **not** contend with
> `lowering/core.rs`, so it is the one piece that can proceed while Runtime
> holds `#6d`. The operator anticipated this — *"there will be ample time for
> framing the post refactor WPs to keep the fleet running."*

## What it is

The `ken-runtime` backend has files well past the crate's average. **Re-measured
at `main = a6186741` (2026-08-10)** — crate **163,782 lines across 50 files**:

| file | `a6186741` | `837f9296` | `1e6eb5c6` |
|---|---:|---:|---:|
| `lowering/core/tests/control.rs` (test) | 29,095 | 26,443 | 9,847 |
| `planning/static_transition.rs` | 24,819 | 23,798 | 9,034 |
| `lowering/mod.rs` | 19,681 | 19,604 | 11,197 |
| `lowering/core.rs` | 18,298 | 16,640 | 9,788 |
| `boundary_value_clif.rs` | 9,116 | 9,116 | 8,691 |
| `lowering/core/tests/constructors.rs` (test) | 9,291 | 9,283 | — |

`cranelift_backend/` alone is **122,049** lines — the subtree is now larger than
the whole crate was when this node was filed (97,881).

**The `static_transition.rs` prediction resolved.** This node projected
*">20,858 in `RT-RECURSOR-TRANSPORT`'s in-flight delta"*; it reached 24,819 on
`main` **without** that node having started. It is the largest production file
in the crate.

⛔ **These are pre-retirement counts and they are not the frame's inputs.** The
rule above stands: the frames re-measure on the post-#7 tree. This table exists
so nobody reasons from the `1e6eb5c6` numbers, which are now off by 2-3x.

**What #7 subtracts**, as `RecursiveDescent` occurrences at `a6186741`:
`control.rs` 53, `core.rs` 32, `mod.rs` 5, `static_transition.rs` 3, `units.rs`
2, `object_linker_packaging.rs` 1. **Every count is higher than at
`837f9296`**, so the deletion is larger than campaign §4's estimate, not
smaller.

## Why this is cheaper than the precedent it is modelled on

The original interlude **created** `cranelift_backend/` from a monolith. This one
does not have to invent a structure: `static_transition.rs` **already has** a
sibling `static_transition/` directory holding `semantic_ir.rs` (2,729) and
`abi.rs` (1,601), and `lowering/` is already a directory. ⇒ This node **extends
established seams** rather than designing new ones.

## Sequencing

**Node #8**, immediately after [[RT-DESCENT-RETIRE]]. The full ruling and its
three grounds are in `docs/program/16-recursive-descent-retirement.md` §4 — read
that before framing this. In brief:

1. #7 **subtracts** from exactly these files, so splitting first re-homes a
   lane that is then deleted out of its new home — paid twice.
2. The two remaining ports are **consumers** of the transport, not authors, and
   both frames ban building a second one ⇒ the size peak is roughly now.
3. A split and the campaign **contend on the same files** and cannot run
   concurrently, so this is purely an ordering question.

## The open question this node does NOT settle

Whether large files are themselves making the campaign work harder. No evidence
was found for it — [[RT-DECL-CLOSURE-PORT]]'s three hard stops were **semantic**,
not navigational — but that is a Steward inference from reports, not a
measurement, and the ring is better placed to judge it.

**The cheap test as originally written is SPENT, and its replacement is live.**
It said to ask the Architect, at "#3-atomic's merge", whether a narrow split of
`static_transition.rs` should ride ahead of [[RT-PRODUCER-MATCH-PORT]]. Both
that merge point and that node are behind us — `RT-PRODUCER-MATCH-PORT` is
`merged` — so the question was never put and cannot be.

**Re-aimed, per campaign §4:** [[RT-RECURSOR-TRANSPORT]]'s `D2` may add a
planner-owned binding, and that would land in a 24,819-line
`static_transition.rs`. Whether it does is what its `D1` determines — `D1` may
close both classes for free and add nothing. ⇒ **Ask at `D1`'s checkpoint, not
before**, when there is a measured answer about whether any remaining node must
do real work inside that file. One exchange; a "no" disturbs nothing.

## The `NATIVE-HANDLE-CARRIER` edge, and what measurement says about it

**19 nodes are transitive dependents of this one** — the whole remaining Linux
ABI completion program (`NATIVE-HANDLE-CARRIER` → [[PX8-F-CAP-41]] → `PX8` →
{`ABI-R3`, `PX9`} → Tracks A/M/S/T). The campaign asks whether an early subset
of the split unblocks the first of them, and says **the enclave pass answers it
with a measurement**, not a Steward assumption in either direction. Measured at
`a6186741`:

- Its other three dependencies — `RT-NATIVE-FNSPLIT`, `RT-JOIN-DISPOSITION`,
  `RT-DECL-CLOSURE-PORT` — are **all `merged`**. This node is the only thing
  holding it.
- Its remaining `ken-runtime` work is one `match primitive.symbol.as_str()` arm
  inside `lower_primitive_call` (`core.rs:17977`, refusing at `:18208`), plus
  the CAP-41 fixture to native green, the fold with `c07e63c2`, and the six-axis
  matrix.

⇒ The region it needs re-homed is a **primitive-lowering emitter family** — the
class the program report's §5.2 puts last in its order, as "emitter families
whose producer and evidence boundaries are already closed." Whether that family
extracts early and cleanly is the Architect's call against the post-#7 tree, and
a "yes" makes it the first WP of the phase.

> **A second reading, surfaced and not acted on.** This `depends_on` edge is not
> semantic. `NATIVE-HANDLE-CARRIER` states its own rationale: the split first
> means it "rebases onto the new module layout once, instead of landing against
> the old layout and being moved by #8." That is **rebase-cost avoidance**, and
> the delta being rebased measures as one match arm plus a fixture and a matrix
> run. The `core.rs` **+3899/−1022** figure in that node's frame belongs to
> `RT-DECL-CLOSURE-PORT`, not to it.
>
> ⇒ The edge may be costing 19 nodes a full phase of latency to save a small
> rebase. **The edge was set by operator instruction on 2026-08-08, so it stands
> and the Steward does not drop it** — it is back with the operator. If the
> early-subset answer is yes, the point is moot.

> ## BINDING ON WHOEVER FRAMES THIS — operator, 2026-08-08
>
> **The frame must consult both landed research reports and reference them for
> the Architect.** They are on `main`:
>
> | report | what it supplies to this frame |
> |---|---|
> | `research/compiler-refactoring-program.md` (#1630) | the two-arc program, the recommended module-ownership map (§4), the stage breakdown (§5), the recommended WP cuts (§6), and nine named guardrails (§7) |
> | `research/compiler-obligation-ir-refactor.md` (#1628, #1631) | canonical planned/generated terms, a closed source machine, a hybrid checked transducer, immediate Cranelift command interpretation, concrete post-emission evidence |
>
> **Reference is not adoption.** Both are marked advisory and neither is an
> architecture ruling; the first says outright that the Steward and Architect own
> the node graph. **The frame cites them so the Architect has them in hand at
> review — it does not inherit their architecture**, and this node stays a
> behavior-preserving split unless the Architect rules otherwise.
>
> **This settles a routing question that was open.** The IR recommendation was
> deliberately left unrouted while the runtime ring held the same lowering
> surface. **This frame is its venue** — the recommendation gets triaged here, by
> the Architect, at the point where someone is actually about to restructure
> those files.
>
> **The reports agree with this node's existing constraint, which is a reason to
> trust it rather than to relax it.** The program report's structural arc *begins*
> with a post-retirement remeasure, and its guardrails independently warn against
> optimizing for equal-sized files, naming permanent modules after temporary
> campaign nodes, and combining pure moves with semantic rewrites. **The
> "do not carry today's line counts into the frame" rule above stands unchanged**;
> the report reinforces it, and a landed report is not a substitute for the
> measurement it tells you to take.
