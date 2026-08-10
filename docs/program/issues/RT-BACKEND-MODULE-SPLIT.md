---
id: RT-BACKEND-MODULE-SPLIT
title: "Split the oversized ken-runtime backend files into modules — the follow-on to the recursive-descent retirement, not an interlude in it"
status: draft
owner: runtime
size: M
gate: none
depends_on: [RT-DESCENT-RETIRE]
blocks: []
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

> # THE PASS IS ANSWERED. THIS NODE IS NOW A PHASE RECORD, NOT A MERGE NODE.
>
> **Architect ruling `evt_54zvaqbrm752x`, 2026-08-10.** Recorded here because a
> ruling that lives only in a thread strands.
>
> ⇒ **`blocks` is now empty on purpose.** The phase's real edges live on its
> children. This node will never merge; when the full cut is filed it becomes
> `closed` — resolved-without-landing — not `merged`.

## The ruling — the eighteen-slice cut

**The structural arc is several accepted phase partials.** Each child is
complete for its named transfer, independently reviewable and mergeable, and
**does not claim #8 closure**. There is no atomic all-files candidate.

| # | slice | filed |
|---:|---|---|
| 1 | [[RT-BACKEND-SPLIT-CENSUS]] — Stage A, no code move | yes |
| 2 | [[RT-BACKEND-PRIMITIVE-LOWERING-SPLIT]] — early critical-path slice | yes |
| 3 | `RT-PLANNER-GRAPH-FOUNDATION-SPLIT` | |
| 4 | `RT-PLANNER-UNITS-ABI-SPLIT` — first planner domain | |
| 5 | `RT-PLANNER-OCCURRENCES-SPLIT` | |
| 6 | `RT-PLANNER-CONTINUATIONS-SPLIT` | |
| 7 | `RT-PLANNER-AGGREGATES-SPLIT` | |
| 8 | `RT-PLANNER-EFFECTS-SPLIT` | |
| 9 | `RT-PLANNER-JOINS-TRAPS-SPLIT` | |
| 10 | `RT-LOWERING-FUNCTION-STATE-SPLIT` | |
| 11 | `RT-LOWERING-VALUES-BOUNDARY-SPLIT` | |
| 12 | `RT-SOURCE-MACHINE-TYPES-SPLIT` — existing types/control only | |
| 13 | `RT-EMITTER-CALLS-RETURNS-SPLIT` | |
| 14 | `RT-EMITTER-CONTROL-JOINS-SPLIT` | |
| 15 | `RT-EMITTER-AGGREGATES-SPLIT` | |
| 16 | `RT-EMITTER-EFFECTS-SPLIT` | |
| 17 | `RT-EMITTER-TERMINALS-CLEANUP-SPLIT` | |
| 18 | `RT-BACKEND-SPLIT-CLOSURE` — delete adapters, narrow facades, remeasure | |

**Only the first two are filed, and that is deliberate.** They are the two with
structural consequence today: item 1 gates the phase, and item 2 releases
[[NATIVE-HANDLE-CARRIER]]. The census supplies the binding paths, counts and
sizes for the rest, and **it may prove two adjacent small families are one
lifecycle** — filing all sixteen now would create work ahead of the evidence
that sizes it.

⛔ **A census merge permits one frame with independently reviewable commits. It
does not permit a planner or lowering mega-diff, and it does not permit grouping
to reduce node count.** An exposed behavioural dependency **stops the move and
returns for a semantic ruling**; it is not repaired inside a "pure move."

## Gates binding every structural frame in the phase

- Exact old/new symbol and test-property ledgers.
- **No** representation, diagnostic, hash, serialization, behaviour or trust
  change.
- No widened production API, no facade recreation of the monolith.
- Affected library **and** targeted test configurations both compile.
- Each moved mutation reds the same reached property, with the same **nonzero**
  denominator and restoration.
- Plans and commands **never** count as emitted evidence.
- Source text is a census aid, not the only semantic oracle.
- Scoped local checks plus CI's workspace gate — **never** a local workspace
  run.

## Seating — this phase runs T2, and only this phase

**Operator, 2026-08-10:** the refactoring work is much more mechanical than the
initial implementation and discovery, so `runtime-implementer` switches to a
**T2** model for #8's slices. Recorded with its boundary and its trigger in
`agent/MODELS.md` — read it there before flipping anything.

Two things that block the obvious mistakes:

- **The switch happens at the phase boundary**, when the first slice is
  released. Not before. The seat is on `#6d` `D2b`'s closed-projection work,
  which is what Runtime's standing T1 exception exists for.
- **It covers #8 only.** `RT-RECURSOR-TRANSPORT`, `RT-FNUNIT-RESULT-TOKEN` and
  `RT-DESCENT-RETIRE` precede this phase and **stay T1**; so does the semantic
  arc deferred after #8 closure.

## Ownership map — accepted with amendments

Modules own **semantic lifecycles**, not line counts or campaign names. The
durable direction is `plan construction -> validated read-only views -> lowering
state/source machine -> concrete backend mutation -> independent evidence ->
closure -> publication`.

Thirteen owners: planning facade; planner graph foundation; occurrences; units
and ABI; continuations; aggregates; effects; joins and traps; lowering facade;
function-local state; values and boundary; source machine; backend event
families. Plus evidence/laws and tests.

Three amendments the frames must carry:

- **The graph foundation is not an `ids.rs` drawer.** `PredeclaredFunctionId`
  stays unit-owned; `StaticOriginId` and source/child correspondence stay
  occurrence-owned.
- **`boundary_value_clif.rs` is not absorbed merely for size** — Stage A must
  prove its lifecycle and consumers first.
- **The source machine is relocation only in #8**, not a transition IR.
- Generated traps receive **no fabricated source origin**.

## First planner domain

**Units and ABI move first**, after the graph foundation. `abi.rs`, predeclared
ids, descriptors, slots, call-edge views, pre-emission validation and the
read-only `EmittableUnit` boundary form the strongest closed seam; it is less
coupled to live source-machine work than continuations, and it gives the later
emitters a stable unit/call vocabulary. **Occurrences next. Continuations move
only after the live `#6d`/#7 churn is gone.**

The primitive slice is the first production slice overall but is **not** a
planner domain.

## The IR recommendation — triaged, which was this pass's other job

**Adopted as a target, outside #8:** the hybrid checked transducer, distinct
planned/generated term identities, closed source-machine state, typed rule
results, immediate typed-command interpretation within one function,
independent post-emission evidence before obligation commitment, and typed
domain-specific closure laws.

**Deferred to their own semantic nodes after #8 closes:** canonical terms,
transition results, commands, command/evidence separation, law extraction —
each has its own mutation and publication boundary. **The first command family
is declared calls, not primitives**, because its existing post-CLIF callee check
is the strongest discriminator.

**Declined now:** a persistent whole-function symbolic backend, a universal
ledger or key, fabricated source origins, and symbolic commands as proof.
Reassess a persistent IR **only** after the hybrid boundary exists and a
concrete multi-backend, optimization or replay need is **measured**.

Stable traces begin diagnostic and test-only over stable semantic ids. They are
**not** persisted artifacts, hash inputs, or compatibility surfaces in this arc.

⇒ **#8 stays behaviour-preserving**, and the IR recommendation stops being
unrouted without being smuggled into a structural slice.

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

> **ANSWERED: yes, and the edge is kept rather than dropped.** Architect
> `evt_54zvaqbrm752x` §5 ruled that a clean early subset exists, on the
> ownership proof above. [[RT-BACKEND-PRIMITIVE-LOWERING-SPLIT]] — cut item 2,
> immediately after the census — **is the architectural release point**, and
> `NATIVE-HANDLE-CARRIER` now depends on **that child**, not on every #8 child.
>
> ⇒ **The operator's 2026-08-08 instruction is honoured exactly, not
> overridden.** Its stated rationale was that the node should rebase onto the
> new module layout **once** instead of landing against the old layout and being
> moved by #8. Depending on the primitive child delivers precisely that, and
> stops 19 nodes waiting on the other sixteen slices. The Architect assigned the
> bookkeeping to the Operator and Steward and did not mutate the graph itself.
>
> The earlier reading — that the edge was rebase-cost avoidance costing 19 nodes
> a full phase — is **superseded by this refinement**, not by a deletion. The
> `core.rs` **+3899/−1022** figure cited for that edge does belong to
> `RT-DECL-CLOSURE-PORT` and not to `NATIVE-HANDLE-CARRIER`; that correction
> stands.

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
