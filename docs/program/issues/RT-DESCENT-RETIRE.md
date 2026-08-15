---
id: RT-DESCENT-RETIRE
title: "Retire RecursiveDescent — delete the migration selector, the residual enum, the authority variant, and the recursive-descent emission lane"
status: draft
owner: runtime
size: M
gate: none
depends_on: [RT-DECL-CLOSURE-PORT, RT-SEED-CALL-PORT, RT-PRODUCER-MATCH-PORT, RT-RECURSOR-TRANSPORT, RT-FNUNIT-RESULT-TOKEN, RT-LEXICAL-RECURSOR-CONSUMERS, RT-CLOSURE-CROSSING-ELIMINATE]
blocks: []
github: null
origin: Operator directive 2026-07-29 — "we should not let it linger in a half-migrated state. That just carries tech debt for no benefit." Campaign docs/program/16-recursive-descent-retirement.md. Steward-filed (agents cannot create tracked work per COORDINATION §2).
---

> # `draft`, NOT unframed — the frame is written and the premise is not yet true
>
> **Set back to `draft` 2026-08-13 by the Steward. It was mis-flagged `ready`
> while `RT-RECURSOR-TRANSPORT` — a `depends_on` and a genuine mechanism
> dependency — was itself still `ready`.** Nothing had landed, so the node's
> premise ("all residual classes retired") was false the whole time it sat on
> the frontier.
>
> **Runtime paid for that.** `RT-DESCENT-RETIRE` `D1` was pulled and
> hard-stopped at fresh base `c1b9a1e8`: the exhaustive unarmed enumeration
> found **89 intact non-empty residual rows** (74 `LexicalCallArgumentRecursor`,
> 15 `MatchScrutineeRecursor`) and production still selecting
> `BodyEmissionAuthority::RecursiveDescent` **29 times**. Both residual variants
> and both classifier arms remain in production. The observer was fully
> reverted; no candidate exists; `D2`-`D7` were never entered.
>
> **That measurement is worth keeping.** It is a clean unarmed census at a named
> base, and it is exactly the dead-code oracle this node's `D1` is supposed to
> capture. When the node is genuinely released, `D1` re-runs against a base where
> the answer should be zero — and the `c1b9a1e8` numbers above are the control
> that makes a zero meaningful rather than vacuous.
>
> **Flip to `ready` when `RT-RECURSOR-TRANSPORT` is `merged`**, not before, and
> not on a partial: this node's dependency is the mechanism, and `D1` re-run
> must find the residual population empty.
>
> **Two `depends_on` edges added 2026-08-15 to stop that same mis-flip recurring
> for the other residual class.** The census above found **74
> `LexicalCallArgumentRecursor`** rows, and nothing in the original dependency
> list governed them — so once the five listed nodes merged, `gen-progress.sh`
> would have shown this node flip-eligible with the larger residual class fully
> intact. **The prose guard alone was not enough; it is the same shape that put
> this node wrongly on the frontier the first time**, and `depends_on` is what
> the generator actually reads.
>
> ⇒ Added **[[RT-LEXICAL-RECURSOR-CONSUMERS]]** (the lexical class) and
> **[[RT-CLOSURE-CROSSING-ELIMINATE]]** (where that node's remaining population
> is now dispositioned). **The second edge is the load-bearing one**: retirement
> cannot be assessed while it is still open whether the remaining expressions get
> a repair or a recorded refusal, because a refusal makes retirement a
> **narrowing of a presently-compiling capability** rather than debt removal.
> That is a product call, and this node is where it lands.

> # RETIRING ALL FIVE RESIDUAL CLASSES IS NOT THE FINISH LINE
>
> With every class retired, the selector still exists, still evaluates on every
> compilation, and the `RecursiveDescent` emission lane is still compiled in —
> **dead**. **That residue IS the tech debt the directive names.** So this is
> a required node, not a tidy-up, and it is the node that actually banks the
> efficiency win.
>
> **Done is:** `select_body_emission_authority`, `RecursiveDescentResidual`,
> `declaration_recursive_descent_residual`, `recursive_descent_residual`,
> `BodyEmissionAuthority::RecursiveDescent` and the recursive-descent emission
> lane are **deleted**, and every program compiles through `FunctionizedUnits`.

## Why it is its own node and not a coda on the last migration

**Because a deletion this wide has a different risk profile than a port**, and
folding it into [[RT-RECURSOR-TRANSPORT]] would let "the last class is retired"
be reported as "the lane is gone." Those are different claims, and only the
second is the directive.

The lane's surface at `origin/main = 14c3c5f7` spans **five production files** —
`lowering/core.rs`, `lowering/mod.rs`, `planning/static_transition.rs`,
`object_linker_packaging.rs`, and the `core/tests/` control modules. A
deletion that misses a file leaves a dead branch that still compiles.

## The dead-code oracle is spent by the commit that clears it

Once the last residual class is retired, **nothing in the tree can any longer
distinguish "the lane is unreachable" from "the lane was deleted."** The
evidence that the lane is dead exists only *before* this node lands.

⇒ **`D1` captures that evidence first**, while it is still capturable, and the
acceptance criteria are written against it. Do not start deleting and then try
to prove the lane was dead.

## Sequencing

**Last** in the campaign, gated on the four migration nodes **and on
[[RT-FNUNIT-RESULT-TOKEN]]**. This is the only node here whose `depends_on`
list is a genuine mechanism dependency rather than file contention — it cannot
land until every class is retired.

**The fifth edge is a different kind of dependency and was added 2026-08-08
by the Steward** (sequencing call; the node was filed that morning, after this
list and the campaign DAG were written). The four migration edges say *the lane
is no longer selected*. `RT-FNUNIT-RESULT-TOKEN` says *the lane is no longer
needed* — it owns `nc22`, currently the only program exercising a shape that
**only the `RecursiveDescent` lane supports**.

**Landing this node first would silently narrow what Ken can compile, and
nothing would fail.** `nc22` is `#[ignore]`d under that node's own quarantine,
so the one witness is already suppressed; deleting the lane under a skipped row
retires the fallback and the detector together. Un-skipping `nc22` green on the
functionized lane is that node's closure condition, and it is this node's
release gate.

## THE FRAME IS WRITTEN

`docs/program/wp/RT-DESCENT-RETIRE.md`. Campaign context, the binding traps that
bind every node in this arc, and the full schedule:
`docs/program/16-recursive-descent-retirement.md` — **read it before the frame.**
