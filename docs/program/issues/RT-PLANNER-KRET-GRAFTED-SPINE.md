---
id: RT-PLANNER-KRET-GRAFTED-SPINE
title: "The planner derives k_ret_identity from a continuation the spec does not let the runtime invoke. exact_response_ret_identity (ken-runtime cranelift_backend/planning/static_transition/responses.rs:1379) takes the continuation_origin, destructures that ONE syntactic occurrence as a ComputationalMatch, filters its own cases for the one ending ::ITree::Ret and returns that identity -- it composes nothing. Spec 42 §6.4 fixes the effect spine by bind's grafting (36 §2.2: bind (Vis e f) k = Vis e (\\r. bind (f r) k)), so the tree a driver walks is ALREADY grafted and the node's continuation IS the composition; after grafting the immediate syntactic continuation of a source occurrence is not an object in the tree at all. Make the planner's derivation follow the grafted spine rather than the immediate occurrence. NOT an emission-side change: 42 §6.4 makes resuming the immediate continuation observable and wrong, so a runtime altered to invoke it would become non-conformant."
status: draft
owner: runtime
size: M
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "Steward, 2026-09-19, at origin/main ddbc803228dbb56f73df31a906ec1fe9a413910e. Filed on the Architect's normative ruling at evt_3q1xzbhzn6c0v, pre-committed by the Steward at evt_5gzxar0zbnv1y before the ruling was cast. Successor to RT-PX7F-LINKED-PUBLIC-ROWS, whose D1 closed on a grounded ruling rather than a repair: that node established, by measurement, that at least one row's runtime K carries an operation that is not its own origin's, and the Architect then settled the fork normatively. Filed by the Steward because COORDINATION §2 reserves tracked-work creation to this seat."
---

> ## FRAMED 2026-09-19 — `ready`, size M, tier T1
>
> Frame: [`wp/RT-PLANNER-KRET-GRAFTED-SPINE.md`](../wp/RT-PLANNER-KRET-GRAFTED-SPINE.md)

**The defect is two sentences**, and both are now established rather than
hypothesised:

    exact_response_ret_identity reads ONE syntactic occurrence and composes
    nothing.
    42 §6.4 requires the grafted spine.

**What established each.** The first is the Architect's, read from the code
(`evt_2dcp5ej098gy2`): the function takes `unit.continuation_origin()`,
destructures `plan.planned_occurrence_expr(continuation_origin)` as a
`RuntimeExpr::ComputationalMatch`, filters **that occurrence's own cases** for
the one ending `::ITree::Ret`, and returns its identity. The second is the
normative ruling at `evt_3q1xzbhzn6c0v`.

**`42 §6.2` alone does NOT settle this, and the distinction matters for how the
ruling is cited.** `drive_H t = case whnf t of … Vis e k -> drive_H (apply k (H
e))` applies "the continuation", and which continuation that is depends on what
tree the driver walks — read by itself, both forks survive. The settling clause
is `42 §6.4`, which fixes the spine by `bind`'s grafting and cites `36 §2.2`;
`36` carries that law as the *definition* of `bind`
(`spec/30-surface/36-effects.md:543`). So the tree is already grafted before the
driver sees it, and `apply k (H e)` invokes the composed continuation not
because a rule names it but because **it is the only `k` the node has**.

**Why this is a requirement and not the merely natural reading** (both the
Architect's):

1. `§6.4` makes the alternative **observable and wrong** — *"a reordered or
   dropped interaction is a different envelope, so the case flips"*. A driver
   resuming the immediate continuation performs the enclosing computation's
   effects out of spine order, and inside a bracket it skips the release. That
   is a conformance failure with a discriminating trace, not latitude.
2. `§6.2`'s loop is tail-resumptive by `OQ-9` / `36 §5.2`: `k` is applied
   **exactly once**, in tail position, and it is the node's own continuation.
   There is no reification step at which a different `k` could be selected.

⇒ **The emission-side repair direction is FORECLOSED, not merely disfavoured.**
Changing the runtime to invoke the immediate continuation would convert a
conformant runtime into a non-conformant one. If the runtime were invoking a `K`
that is *neither* the immediate nor the composed continuation, that is a third
defect and a different node — nothing measured points there.

**The witnesses live in `RT-PX7F-LINKED-PUBLIC-ROWS`'s two rows**, which this
node's repair is expected to clear:

    right-denial     origin PrivateFsHandleMetadata (542)   runtime K op 543
    double-release   origin PrivateResourceRelease  (543)   runtime K op 543

**`right-denial` is the row that discriminates**, and it departs onto the
bracket's own release — what composition predicts specifically and what a
selection defect gives no reason for. **`double-release` discriminates
nothing**, and that is worth carrying forward because it is the parent node's
recurring trap once more: its `K` operation equals its own origin's operation,
but that is **value equality, not identity** — in that program the composed
continuation after the inner release also reaches `PrivateResourceRelease`, so
immediate and composed agree there by coincidence of constructor. The whole
discrimination rests on `right-denial`, and `right-denial` is sufficient.

## Parked -- do not release from a successor search

The single authorized atomic D0+D1+D2 attempt is SPENT. It ended at structural
stop 16 (`evt_6freqarew56yh`): the preserve arm needs a durable preserved
`Vis`/K representation across the generated boundary, and all four existing
representation-shaped exits were exercised and denied. Full reconstruction adds
unprovisioned carrier state; the two allocation-free words erase either the
`Vis` constructor or its K.

The remaining requirement is the closure lane that is explicitly WITHHELD, or a
new return protocol, which is representation design beyond this node. That is
why this node is `draft` and not `ready`: it is framed but not releasable, and
it must not be picked up as available work.

Durable branch state is clean at
`4f6b2a454786dd66eb2a1eb9681913e1e4f69b9d`. No host logging, global diagnostic
recoding, aggregate relaxation, or capacity change was retained. The Architect
owns the stop-16 disposition and the symptom entry; neither is recorded here.
Reopening this node requires that disposition, not a successor search.
