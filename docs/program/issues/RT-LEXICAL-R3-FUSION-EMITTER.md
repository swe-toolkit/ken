---
id: RT-LEXICAL-R3-FUSION-EMITTER
title: "Row 5's before-hole expression is the one member of the eight-expression lexical-recursor population whose lawful repair requires static-continuation fusion -- it is carved out of RT-LEXICAL-RECURSOR-CONSUMERS together with its repair and discriminating-control obligations, because leaving the expression in the parent while moving the machinery would give the parent an AC it cannot discharge"
status: ready
owner: runtime
size: M
gate: none
depends_on: []
blocks: [RT-RECURSOR-TRANSPORT]
github: null
origin: Steward re-cut of RT-LEXICAL-RECURSOR-CONSUMERS after its AC reconciliation returned 0 of 8 discharged across eleven merged D2f partials (runtime-leader, evt_d9n0twj6y5sp). Separability ruled by the Architect at evt_7knsqyqg72103 on a Steward question (evt_2vbhz9kgp0b44). Steward-filed (agents cannot create tracked work per COORDINATION 2).
---

## What this node is, and why it exists as a node

`RT-LEXICAL-RECURSOR-CONSUMERS` (`#6d`) repairs an **eight-expression**
consumer population so `RT-RECURSOR-TRANSPORT`'s `D3` can retire the
`LexicalCallArgumentRecursor` residual class. **Seven of those eight do not need
static-continuation fusion. One does.** This node owns that one.

**Architect ruling `evt_7knsqyqg72103`**, which is the partition and is exact:

| cell | needs fusion? | why |
|---|---|---|
| rows 1 and 4 (4 expressions) | no | `D2a` removed their `R1` `ComputationalMatch` refusal; all advanced to the distinct `StaticWorkerBinding` wall |
| row 3 (1 expression) | no | `D2b` removed the `R2` closure/ordinary-ABI misclassification; advanced to its retained singular-specialization wall |
| row 5 **after**-hole (1 expression) | no | at the `StaticWorkerBinding` wall |
| row 5 **before**-hole (1 expression) | **yes** | this node |
| row 2 (1 expression) | no | already carved out to [[RT-LEXICAL-ROW2-MISSING-MINT]] |

**Why fusion is genuinely required for that one**, under constraints already
settled and not reopened here: the producer owner lacks the downstream call
arguments, so eager forcing changes CBV; the recursor closure is a live
activation/cursor, so representing or transferring it weakens `#6d`'s `AC-3`;
and the producer and its exact consuming suffix live in **different units**. The
ruled lawful repair is **one planner-identified producer-plus-suffix emission
region**.

## The carve-out rule that makes this node correct

**The expression moves WITH its repair and its discriminating-control
obligations.** Moving the machinery while leaving the expression in `#6d` would
give `#6d` an acceptance surface it cannot discharge — an impossible parent AC.
That is the Architect's load-bearing caveat and it is the reason this node is
scoped to a *cell of the population* rather than to *a pile of mechanism*.

⇒ `#6d`'s population drops from eight expressions to **six**: rows 1, 3, 4, and
row 5's after-hole expression. Row 2 is `RT-LEXICAL-ROW2-MISSING-MINT`'s. Row
5's before-hole is this node's.

## What is already landed, and it is substantial

**Eleven `D2f` partials merged into `main` under `#6d` between 2026-08-11 and
2026-08-12**, every one honestly labelled as inert and every one gated. They are
this node's substrate and **are not to be unwound**: the ABI class, the identity
plane, the arrival-control repair, the per-cause applied-root gate, the
complete-key redirect selector, the fusion arena, the claim facility, the
producer-side atomicity partial, and the full emitter chain (PR #1940) with its
two subsequent prose corrections (PRs #1942, #1943) and the empty-population
attribution repair (PR #1945).

**The whole chain is present and running on nothing.** `D2F_EMITTER_ARMED` is
`false` and gates exactly one call — `install_static_continuation_fusions`.
Everything else runs unconditionally on every production compile and is inert
**by empty population, not by the gate**. Read `core.rs:2163` onward before
forming any view about what arming would change; the comment there is current as
of PR #1945 and was corrected twice to get that right.

## The trap this node must not walk into, recorded before it is framed

**`px8j`'s `R3` before-hole compile structurally cannot carry an oriented
plan.** It is a **seed-lane** compile deliberately preserved as the *unmarked
negative* — no checked frame, no selected-IH slot, no checked-IH-invocation
marker — and `test_objects.rs:70` passes a literal `None` for
`oriented_subcontinuation_plan`. Production oriented plans decode from a checked
package's metadata (`planning.rs:144`), and a seed-lane compile has no metadata
to decode.

⇒ **The acceptance fixture is the checked `D2g`/`D2j` `R3`-shaped twin**, with
its own independently authored `OrientedSubcontinuationPlanV1`, entered through
`compile_expr_into_object_module` with `Some(oriented)`. **`px8j` is the
absence / ordinary-refusal comparator and must never again be described as the
fusion-positive.** That correction landed at `main` `17f68eb1`; this node
inherits it deliberately rather than by citation, because inheriting the old
witness by citation is exactly how the defect would propagate into a successor
frame.

**Forbidden and already ruled out** (Architect `evt_6vf66hmwv52y6`): no
`Some(plan)` handed to `px8j_capture_source_trace`; no synthesized default plan;
no marker inference from the Runtime shape; no weakening of the required
checked-transport key member; and **no making fusion independent of `oriented`**,
which would reopen `D2h`'s soundness-bearing identity.

## Frame

`docs/program/wp/RT-LEXICAL-R3-FUSION-EMITTER.md` — the interior seam, the
arming gate and its five causal controls, deliverables, acceptance criteria with
their controls, excluded scope, and stop conditions.

## Not this node

Retirement of the residual class, lane deletion, and the other seven
expressions. Row 2's missing-`Mint` cell. `#6d`'s six remaining expressions and
their `StaticWorkerBinding` and singular-specialization walls.
