---
id: RT-MATCH-SCRUTINEE-DISPOSITION
title: "MatchScrutineeRecursor retains three renderings and the tree records no reason -- establish why the functionized lane does not take it before dispositioning it"
status: ready
owner: runtime
size: S
gate: none
depends_on: []
blocks: [RT-DESCENT-RETIRE]
github: null
origin: "Architect ruling evt_620806vfy5kwm (2026-08-16) on RT-DESCENT-RETIRE's D1 hard stop, verbatim: 'MatchScrutineeRecursor is UNMEASURED, and symmetry is not an argument for it. Its entire doc is one line. No reason is recorded. Nothing in the tree says whether it is an unbuilt port or a shape the functionized lane would correctly refuse. I will not rule it by analogy to its neighbour.' Population fixed by runtime measurement A/B at exact 3523868afe7cd84b47c7b07281ff7df7c3202d61 (runtime-implementer evt_4v0frfza70d2m). Steward-filed per COORDINATION section 2."
---

## What this node is

**The second of the two live `RecursiveDescentResidual` variants, and the one
nobody has measured.**

`RT-DESCENT-RETIRE`'s `D1` found production selecting
`BodyEmissionAuthority::RecursiveDescent` 31 times; 4 of those selections, over
3 exact renderings, carry `MatchScrutineeRecursor`.

**The entire recorded reason is one line** — `lowering/core.rs:2003`, verbatim
and complete:

> `/// An ordinary match consuming an active computational recursor.`

**That says what the shape is. It does not say why the functionized lane will
not take it.** Its neighbour `LexicalCallArgumentRecursor` (`:2005-2011`) has
seven lines that state plainly that the port was never built. **This variant has
nothing of the kind.**

> ### THE ANSWER IS NOT AVAILABLE BY SYMMETRY. THAT IS WHY THIS IS A NODE.
>
> Architect, `evt_620806vfy5kwm`: *"Nothing in the tree says whether it is an
> unbuilt port or a shape the functionized lane would correctly refuse. I will
> not rule it by analogy to its neighbour; that is the reading-instead-of-
> measuring move, and the four retired variants each got a node that built the
> port first."*
>
> **The two dispositions are opposite in consequence.** An unbuilt port is
> capability owed and blocks retirement until built or scoped out. A correct
> refusal is retired or re-described with the lane and costs nothing. **Guessing
> which, from the neighbour, is how the retirement would delete a real
> capability gap or build one that is not owed.**

## The population, measured

**Fixed at exact `3523868afe7cd84b47c7b07281ff7df7c3202d61`**; the complete
`crates/ken-runtime` tree is identical at `dc98f6f84`.

| # | hash | exact rendering | compiles | set |
|---|---|---|---:|---|
| 1 | `e1183de5e21770cc` | `D8d` `Match { scrutinee: px8j_deferred_recursive_field_fixture(), ... }` | 1 | {M} |
| 2 | `cc234d5c5f826979` | distinct deferred `Match` in `px8j_all_three_producer_paths...` | 1 | {M} |
| 3 | `55fc4ee2ca985f82` | `rt_match_scrutinee_recursor_executable()` | 2 | {M} |

**Row 3 is `#[cfg(test)]`** at `lowering/core/tests/control.rs:16150`, with
`scrutinee: rt_closed_active_recursor()` — a fixture authored to occupy the
position, the `M`-side twin of the lexical position-B fixture. Verified by the
Steward. **Rows 1 and 2 have not been opened.**

**The partition is measured, not assumed.** Measurement `A` re-read the
population against the non-short-circuiting
`enumerate_recursive_descent_residuals` and found **zero dual-retained
renderings**: removing `M` leaves exactly the twelve `L` renderings, removing
`L` leaves exactly these three. **These three are `M`-only.**

> **The masking hazard ran in this variant's favour and it must not be
> forgotten.** `recursive_descent_residual` short-circuits, and its `Match` arm
> tests `MatchScrutineeRecursor` **first**, `.or_else`ing the rest — so under the
> selector a doubly-retained program reports **only** `M`. The original `D1`
> probe read that selector. **The count of 4 was therefore an upper-biased
> reading for this variant specifically**, and it survived only because the
> set-valued re-read found no dual retention. **Use
> `observed_recursive_descent_residuals()`, never the selector.**

## Deliverables

**`D1` — establish WHY the functionized lane does not take the shape.** Read
the two exhaustive production classifiers and the `Match` lowering path. The
answer is one of:

- **An unbuilt port** — some capability the functionized lane lacks, in the
  shape of the lexical variant's recorded gap. Name what is missing.
- **A correct refusal** — a conservation law, a planner invariant, a semantic
  impossibility or a structural absence, in the shape of the four dispositioned
  at `evt_5h7vzc27mc11j`. Name which, and where it is enforced.

**`D1` is the node.** `D2` and `D3` are conditional on its answer and neither is
authorized in advance.

**`D2` — the per-rendering triage, for the three above.** Fixture or
source-reachable, under the method gate below. **Row 3 is settled as a fixture**;
do not re-derive it, and do not let it settle rows 1 and 2.

**`D3` — record the disposition where the retirement's gate reads it**, in
[[RT-DESCENT-RETIRE]]'s terms. A finding that is measured but not recorded where
the criterion looks leaves the lane exactly where it is.

## The method gate

> **A NEGATIVE EXISTENCE CLAIM IS NOT ESTABLISHED BY FAILED ATTEMPTS.** *"We
> wrote N programs and none reached it"* is consistent with the N+1th reaching
> it. **Argue from the surface grammar, the elaborator's admission rules and the
> kernel gates** — what a user can write at all — never from a sample of
> attempts. This is [[RT-REFUSAL-SOURCE-WITNESS-OR-INVARIANT]]'s gate, which the
> operator's 2026-08-16 instruction established and which governs here
> identically.
>
> **A witness is the easy direction** — one `.ken` file reaching the shape
> settles that row outright, and it is the better outcome.

## Acceptance criteria

**`AC-1`. `D1`'s answer names a mechanism, not a symmetry.** *"Like
`LexicalCallArgumentRecursor`"* and *"like the `FunctionizedUnits` refusals"* are
both rejected. Cite the classifier, the law, or the missing capability by file
and line.

**`AC-2`. All three renderings carry a disposition**, preserved by hash — the
handback normalizes `hash<TAB>rendering` and diffs against the table above.
**A row re-identified by description is a reconstructed analogue.**

**`AC-3`. Every "no source program reaches this" carries a gate argument and
states its population**, per the method gate.

**`AC-4`. No exclusion result is cited as capability evidence.** Measurement `B`
established sole-retention for all three, which is necessary and **not
sufficient** — an exclusion returning `FunctionizedUnits` says the classifier no
longer retains the program, not that the lane can emit it.

**`AC-5`. The `L` variant is untouched.** Its twelve renderings are
[[RT-LEXICAL-CALL-ARG-WITNESS-OR-PORT]]'s. **Do not disposition it here and do
not cite this node's answer as evidence about it** — the asymmetry runs both
ways.

**`AC-6`. No-regression, in CI** (`COORDINATION §12`). Targeted local validation
only. **This node is expected to land a measurement and a disposition and may
land no production change at all.**

## Banned scope

- **Building a port**, if `D1` finds one owed. Report it sized and stop; whether
  it is worth building is the Steward's and the operator's call.
- **Retiring anything.** [[RT-DESCENT-RETIRE]] deletes the selector, the enum,
  the authority and the lane.
- **Dispositioning `LexicalCallArgumentRecursor`.**
- **The `RecursiveDescent`-as-oracle framing** (operator, 2026-08-15). What a
  program does under `RecursiveDescent` is not evidence that it should compile.

## Sequencing

**`ready` at filing, `depends_on: []`.** The population is measured and fixed by
hash; nothing gates it.

**It blocks [[RT-DESCENT-RETIRE]]** alongside
[[RT-LEXICAL-CALL-ARG-WITNESS-OR-PORT]]. **The two are independent and the
retirement needs both** — a variant left undispositioned is a lane that cannot
be deleted.

**Lane 1 under the operator's 2026-08-15 two-lane directive.** Runtime is
single-threaded; the lexical node carries the larger population and goes first.
