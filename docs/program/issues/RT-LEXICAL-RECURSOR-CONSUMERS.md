---
id: RT-LEXICAL-RECURSOR-CONSUMERS
title: "Repair the LexicalCallArgumentRecursor consumer population on the functionized lane, activated by B-only exclusion before the retirement removes the seam"
status: active
owner: runtime
size: M
gate: none
depends_on: [RT-MATCH-RECURSOR-CONSUMERS]
blocks: [RT-RECURSOR-TRANSPORT]
github: null
origin: Architect ruling evt_5w09dcwbf7k70 (2026-08-08) on RT-RECURSOR-TRANSPORT hard stop 4, narrowed to rows 1-5 by the re-rule evt_3r4j14fv1jtj2 on the nine-expression census evt_16cmej481q7ns. Campaign docs/program/16-recursive-descent-retirement.md node #6d. Steward-filed (agents cannot create tracked work per COORDINATION §2).
---

> # THIS NODE IS ROWS 1-5 ONLY. ROW 6 IS NOT ITS WORK.
>
> **Narrowed 2026-08-08 by Architect re-rule `evt_3r4j14fv1jtj2`**, on the
> measured census. An earlier revision of this file claimed all six red rows and
> asserted they shared `host_result_closure_match`. **Both claims are false and
> are withdrawn.**
>
> Row 6 (`d8d`) enumerates exactly `{MatchScrutineeRecursor}` — **it was never in
> this node's population.** It belongs to [[RT-MATCH-RECURSOR-CONSUMERS]].
>
> **Do not fold the two nodes back together.** The exact residual producer, the
> activation hook, the observed boundary and the completion owner all differ. If
> the two `D1` causal partitions later prove one exact shared production root,
> **route a subsumption proposal before coding** — it may not be inferred from
> shared retirement timing or shared syntax.

> # FILED `draft` ON PURPOSE — RELEASED WHEN THE A NODE MERGES
>
> The frame is written and shovel-ready; the node is held at `draft` only to
> enforce the ruled release order. **The A node goes first** because it closes
> the Position-A claim that the `D2` record correction is narrowing, and the
> fleet runs one Runtime ring at a time.
>
> ⇒ **Nothing here is owed further analysis.** The moment
> [[RT-MATCH-RECURSOR-CONSUMERS]] merges, this flips `ready` and releases.

## What it is

**Eight expressions across five test families**, previously green, that fail
closed on the functionized lane once [[RT-RECURSOR-TRANSPORT]]'s `D3` retires
the `LexicalCallArgumentRecursor` residual class:

| row | fixture family | expressions |
|---|---|---|
| 1 | `owned_scope_deletion` | 1 |
| 2 | `all_three_producer_paths` | 1 |
| 3 | `siblings_share_an_origin` | 1 |
| 4 | `scope_segments` depth 1, 2, 3 | 3 |
| 5 | `selected_scope` before / after hole | 2 |

Every one enumerates **exactly `{LexicalCallArgumentRecursor}`**, and every
unexcluded compile returns `Ok` — so each row's red is produced by the lane
change and not by a fixture that was already broken.

This node repairs that consumer population **on the pre-retirement tree**, so
`D3` can then retire the class and prove these rows green with no exclusion hook
and no `#[ignore]`.

## The activation seam, measured rather than assumed

**B-only exclusion is this node's seam and it is proven.** At exact `D2`
`8efdfdb3`, excluding **only** `LexicalCallArgumentRecursor` leaves the residual
set empty for each of the eight expressions, so the selector reaches
`FunctionizedUnits` while production continues selecting `RecursiveDescent`.
Every row carries a real activation denominator — a compile through
`px8j_capture_source_trace` — so a refusal cannot be credited where the harness
never reached the path.

⇒ **The repair can be built and proven before the retirement**, which is what
makes this an independently mergeable node rather than a quarantine.

> **The seam was asserted before it was measured, and it nearly set the
> sequencing.** Both the Architect and the Steward stated that the existing hook
> activates all six fixtures; that was a candidate promoted to a fact. The hook
> removes B from the **complete** residual set and reaches `FunctionizedUnits`
> **only when the remainder is empty** — true for these eight, and inapplicable
> to row 6, whose set never contained B at all. The census
> (`evt_16cmej481q7ns`) is the object of record.

## The population is the production predicate, not this list

**`D0` closes the population from the production `LexicalCallArgumentRecursor`
predicate.** The eight measured expressions are a **floor**, not a perimeter.

Sweep every compilation entry that can supply the predicate. Helper spelling,
snake_case fixture spellings and `BodyEmissionAuthority::RecursiveDescent`
assertions are **candidate selectors, not closure** — a grep tells you which
fixtures might be in the family, never what any one of them enumerates.

## Size

**`M`, provisional.** It is a scoping figure taken from a symptom count, and
five rendered refusal strings are **not** five proven causes.

**`D0`/`D1` are authorized to return a partition instead of a repair.** If the
causal partition finds materially distinct authorities rather than downstream
symptoms of one root, or any repair needs a new planner/ABI population, **return
the partition before coding.** Do not silently turn one symptom-named node into
five repairs.

## The edge that is not in the frontmatter

This node's base is **post-`D2`-correction `main`**, and that correction is a
*partial* merge of [[RT-RECURSOR-TRANSPORT]], not its completion. A `depends_on`
naming that node would be a **cycle** — its `D3` is blocked on this one.
`depends_on` is empty and the base is stated in prose and in the frame.

⇒ **Read the base from the frame, not from the edge.** The machine-checked edge
that matters is `blocks: [RT-RECURSOR-TRANSPORT]`.

## What is ruled and not reopened

- `10369776252861e8b15e613576256a3682c70066` is **held evidence only** — not a
  candidate, not a repair base, not to be continued.
- **Zero new `#[ignore]`.** The Steward ruled these quarantinable at
  `evt_7vhjcstd37a50`; that ruling is **withdrawn** and was not revived by any
  later correction.
- The old-green semantic controls are **not disposable**. Surface-Ken
  reachability is unproved; old-green runtime capability is **proved**, and
  these rows are the only probes for the guards they exercise.

## `D2f` ABI-class accepted partial — MERGED 2026-08-11, PR #1897

Exact `006730d4085a04e95dc6b2ca7bebe19d1fbcb6d4` from declared base `84a8f66d`;
one commit, six paths, `+285/-35`, no added ignores. M6 blob identity 6/6 MATCH.
**This node stays `active`.**

> ### IT CLOSES ZERO OF THE EIGHT EXPRESSIONS, BY DESIGN
>
> **A landed partial on a node with 27 merges reads as progress against the row
> count unless it says otherwise.** This one is a **structural prerequisite**:
> the fail-closed `StaticContinuationFusion` ABI class and
> `ContinuationEmissionOwner::Fusion`, with every consumer disposition refusing.
>
> **No constructor, no emitter, no source-body emission authority, no redirected
> producer edge, and no fusion runtime behaviour.** Verified on the object:
> seven `Err(` sites added and **zero** panic-style macros, so the class refuses
> rather than traps; and the only two matches for redirected-edge or emission
> vocabulary are **doc comments stating their own absence.**

## Second `D2f` partial — identity-plane wiring, MERGED 2026-08-11, PR #1899

Exact `1b362f5ea3201ba4dc54d74f0dc88462e3fa4f19` from declared base `e0e4aeb3`;
one commit, five `ken-runtime` paths, `+123/-4`. M6 blob identity 5/5 MATCH.

**The landed fusion identity plan now reaches the sole production compile
path**, with a causal arrival control. **Empty resolution remains legal.**
Excludes definitions, descriptors, authorities, edge redirection, emitter
behaviour, planner signature widening, and every emitter AC claim.

**This one also closes zero of the eight expressions.** Two partials, two
structural prerequisites, zero rows.

> **Why its control is not vacuous** — and it is worth recording, because
> *"empty resolution is legal"* is exactly the condition that lets an arrival
> control pass for free. It takes `NonZeroUsize::new(planes.len()).expect(...)`,
> so **a zero arrival panics rather than passing**, and the assertion then
> equates the recorded planes with the established arrival count as **one
> population**. Resolved sizes are **recorded, not pinned** — this witness plans
> no admitted fusion, so a control asserting resolution *success* would have
> been either vacuous or wrong.

**The `D2f` emitter is the next increment**, scoped to the one `R3` before-hole
witness — **not** an eight-row repair. The `R3` after-hole / missing-`Mint` cell
is excluded from `D2f` and owned by [[RT-LEXICAL-ROW2-MISSING-MINT]].

**Measured remainder** (runtime-leader, `evt_645tm43wf1cne`): the `D2f` emitter
plus its review cycle is **closer to one working day**; **`#6d` closure is
closer to a week.** ⇒ **`D2f` completion and `#6d` closure are separate planning
milestones**, and the former does not discharge the latter. The Steward examined
a re-cut on that estimate and **declined** — the remainder is bounded and named,
so cutting further would manufacture nodes rather than reveal them.

**The `D2a` rider below is discharged by this partial** — it is one of the six
paths.

## Carried rider — the `D2a` control's durability. Owed, not optional.

**DISCHARGED 2026-08-11** by the `D2f` ABI-class partial above, which carries
`docs/program/wp/RT-LEXICAL-RECURSOR-CONSUMERS-D2a.md` among its six paths.
Retained below as the statement of what was owed and why.

**Land this with the next candidate that touches `control.rs`.** It is a rider,
not a deliverable, and it does not earn its own node. It is recorded here
because a rider stated only in a thread strands.

Adversary `evt_xyj8813ymrad`, Steward disposition `evt_3k5eg1trmw2q9`,
independently re-derived at `origin/main` before it was routed.

In the `D2a` arm, `arrivals` is incremented at `core.rs:6686` and `forwards` at
`:6696`, and **between them there is no branch, no fallible step, and no early
return.** In the non-suppressed leg every arrival forwards by construction, so
`assert_eq!(forwards, arrivals)` is decided by the test's own suppression flag
and by nothing about the mechanism.

⛔ **The hazard is not the tautology, it is that the tautology passes at
`0 == 0`** while looking like the stronger of the pair. A later trim that keeps
the equality and drops `arrivals > 0` leaves the whole control vacuous and
green, with every `!contains(R1)` holding because the marker never arrived —
the exact defect class the control was written to avoid. Both counters are
`#[cfg(test)]`, so a pass removing test-only machinery from production `core.rs`
has a motive to touch this precise block.

> **The property, stated so the route stays with the ring:** a later trim must
> not be able to retain the half that passes at zero. Label it, or make the two
> inseparable so there is no half to keep. Durability by instruction or by
> construction — the Steward leans to construction where it is clean, and that
> is a preference, not a ruling.

**Do not change the predicate to make the equality informative** — no failure
mode is available between those two increments, so it cannot be made
informative, only labelled or fused. **Do not pin a fixed count.**

**The same candidate corrects the `D2a` record's own sentence**, *"asserted as a
relation (`forwards == arrivals`, `arrivals > 0`), never as a fixed count"*,
which lists the two as equals and is where the mis-weighting was taught. Fixing
the code and leaving the record reproduces the finding one layer up, which is
the failure this lane has already had twice.

Severity is **control durability, not correctness.** `D2a` merged correctly at
`41b75c7c`; this reopens nothing.

## Carried rider — `D2j`'s non-degeneracy: two groups assert it, five claim it

**Adversary finding `evt_99agje0m3rx1`, measured on `22fb3a61` after `D2j`
merged. Confirmed. It does not reopen `e2907c5e`.**

`D2j`'s matrix closed seven member groups, and `D2j`'s own `AC-1` requires each
row to rest on a **reaching non-degenerate witness** — "an empty vector, a
single-element set, a `None`, or a value that coincides with its neighbour is
degenerate, and a row resting on one is not discharged."

**Measured across the 72 added assertions: two groups assert a non-degeneracy;
five state one in prose.** Of six cardinality pins, four are `len() == 1`
uniqueness pins, which are a different property. The two real guards are
`:438` (`ih.len() == 2`, "so neither lookup is forced") and `:639`
(`widened_args.len() == 2`), and **those are precisely the two rows where a
degeneracy was already caught** — the IH lookup, and the one-child producer
construct the Architect found.

⇒ **The population was the class and the repair took the instances.** All seven
groups share one witness, so a cardinality that collapses a distinction for one
row can collapse it for another. Nothing about either fix generalises to the
five groups nobody looked at. The prose claim for those five is the exact state
the producer-construct row was in until the Architect caught it.

### This is one read per group, not five assertions

**The Adversary's bound is carried and is load-bearing:** it searched for
cardinality pins only. A group could establish non-degeneracy by an
`assert_ne!` between two candidate positions or by a distinctness check, and
that search would not see it; and a member whose authoritative fact does not
depend on cardinality needs no such guard at all. So the honest claim is *five
groups carry no cardinality assertion*, **not** *five groups are unguarded*.

| finding for a group | action |
|---|---|
| no non-degeneracy established, and the fact is cardinality-sensitive | add the guard in `:438`'s form — the count **plus** what it buys |
| established by a different instrument | record where, and the group is done |
| the fact does not depend on cardinality | say why, and the group is done |

**Adding an assertion to a group in the third case is worse than the gap** — it
is a control that cannot fail, which is the failure this lane has now filed
against itself twice.

**`:438` is the model to copy:** assert the count *and* state the reason in the
message. A bare `len() == 2` with no reason is the next thing to rot.

Severity is **evidence completeness, not correctness.** `D2j` merged correctly
and its three deliverables are discharged; this is inherited by the next `#6d`
slice frame.
