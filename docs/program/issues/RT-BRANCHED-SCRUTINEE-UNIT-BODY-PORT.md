---
id: RT-BRANCHED-SCRUTINEE-UNIT-BODY-PORT
title: "Port the recursive-unit-body resolution through a branched scrutinee -- recursive_position_unit_body returns None whenever the scrutinee is a plain Match rather than a literal Construct, so a carried child whose owning form branches has no declared body unit and every consumer falls back to refusal"
status: active
owner: runtime
size: M
gate: none
depends_on: [NATIVE-HANDLE-CARRIER]
blocks: [PX8-F-CAP-41]
github: null
origin: "NATIVE-HANDLE-CARRIER residual. Route measured evt_4tqpqn2gpcsx6; Steward's two corrections to how that result reads evt_9jds3whs094h; scrutinee variant measured at the preserved exact 3d23f1182 by runtime-leader evt_2fzzxf778smjj (reported evt_42nvqwvj71mjb); plain-Match origin discriminator selecting the local-lowerer arm evt_5zknkg76cn3w5 (reported evt_1sp1jg2gsvh9h). Steward-filed per COORDINATION section 2."
---

> # THE MEASURED POPULATION DOES NOT SURVIVE THE PARENT'S RECUT
>
> **This node must carry its own witness.**
>
> Four of the five programs the gap was measured through **are** the `cap41_*`
> rows that [[NATIVE-HANDLE-CARRIER]]'s recut deletes, and the fifth is
> `#[ignore]`d. So there is **no fixture in the tree that reaches this gap** once
> the parent lands, and no test will red when this node is wrong.
>
> ⇒ **D1 is authoring a witness, not consuming one.** The only tree in which the
> gap is currently reachable is the preserved ref
> `refs/heads/preserved/native-handle-carrier-route1-3d23f118` at
> `3d23f1182cbc5a7a9af82ceef37d37f95d64478a` (verified present on origin,
> 2026-08-17). **Do not delete that ref**, and do not size D1 as if the fixtures
> were already there.

## The gap, stated at the site

`recursive_position_unit_body`, in
`crates/ken-runtime/src/cranelift_backend/lowering/core.rs` at `:15668` at
`3d23f1182` (`:15742` on `main` at `5bac56000`) — opens with

> **CORRECTED 2026-08-17: THIS FRAME NAMED `resolve_recursive_unit_body`, WHICH
> RESOLVES IN NO TREE.** At `3d23f1182:15668` — the exact coordinate this frame
> gives — the function is already `recursive_position_unit_body`. The line
> number was the half that was right, and the instruction "locate it by name,
> the number will move" pointed readers at the wrong half. The wrong name
> reached the runtime ring in the `D2` dispatch before it was caught (Adversary
> `evt_24xtqbfx7nec6`, coordinates Steward-verified against both trees).
>
> A stale line number degrades gracefully — it lands you near the site. **A name
> that matches nothing sends you looking for a function that never existed**, and
> a frame that tells you to trust the name over the number inverts which half to
> rely on.

```rust
let RuntimeExpr::Construct { args, .. } = scrutinee.expr else {
    return Ok(None);
};
```

That is **route 1**, and it returns before `args.get(position)` is ever reached.
`None` means "no declared body unit", and all four consumer guard sites dispatch
`if let Some(body) = recursive_unit_body { … }` with
`reject_carried_residual_arguments` as the `else` — so route 1 is a **refusal
producer** for every consumer at once. The sole call is at `:15951`, storing into
`invocation.recursive_unit_body`.

**Route 1 is undocumented.** The function's doc sentence — *"structural-data
recursive positions return `None`; they resume the eliminator directly and take
no arguments"* — describes the **recursive position's** form, decided at `match
argument.expr`, which is route 3 and is reached only after `args.get(position)`.
Route 1 turns on a property of the **scrutinee** and the doc says nothing about
it. Anyone reading the doc as a warrant for route 1 is reading one level too high;
that misreading is what the two Steward corrections above removed.

## What the scrutinee actually is, measured

At `3d23f1182`, across all five governed programs (`evt_5zknkg76cn3w5`):

| property | value |
|---|---|
| scrutinee form | plain `RuntimeExpr::Match`, **not** `ComputationalMatch` |
| the `Match`'s own scrutinee | a runtime `Var` |
| cases | two, non-recursive: `Result::Err` and `Result::Ok`, one binder each |
| both arm bodies | `Construct` |
| route taken | route 1, exactly once per program |
| later refusal | the same `BoundaryCarrier` fallback via `reject_carried_residual_arguments` |

**The plain `Match` is correct, not a missed `ComputationalMatch`.** That was the
live alternative and it was measured out: had the elaborator been at fault, this
node would not exist and the port would be upstream. The discriminating fact is
that the cases are non-recursive with `Construct` bodies — a branch that merely
selects between two constructions, which is exactly what a plain `Match` is for.

⇒ **The lowerer has no per-branch push-through capability.** Both arms declare a
body unit; the function cannot see them because it requires the scrutinee to be
*literally* `Construct` at the top.

## D1 — a Ken source witness that reaches route 1, on a tree that keeps it

Author a program whose carried child's owning form is a two-arm plain `Match`
over a `Var` with `Construct` bodies, and pin that it reaches route 1. It may be
derived from the preserved ref's `cap41_*` shape but **must not depend on those
rows**, because they are deleted by the parent.

**AC-1.** The witness reaches route 1 at least once, shown by instrumentation or
by the refusal it produces, at a SHA named in the report. **State the route by
measurement, never by inference from the refusal text** — the same
`BoundaryCarrier` message is produced by routes this node does not touch.

**AC-2.** The witness lives in a file the parent's recut does not delete. Name
the file and show it is not in the recut's deletion set.

## D2 — push the resolution through the branches, with agreement required

Extend `recursive_position_unit_body` so a plain `Match` scrutinee resolves each
arm's declared body unit and **requires the arms to agree**. Disagreement is a
refusal, not a choice: picking one arm's unit would silently mis-lower the other.

**AC-3.** With D2 landed, the D1 witness's refusal **advances** — it no longer
originates at route 1. **The acceptance is the advance, not a green test.**
**The evidence must be `entered >= 1` AND `route1 == 0`, measured by a recorder
that pushes a row at function ENTRY carrying a route tag.** A bare `route1 == 0`
does not discharge this criterion.

> ### `route1 == 0` IS CONSISTENT WITH THE PORT NEVER BEING REACHED
>
> Amended 2026-08-17 (Adversary `evt_24xtqbfx7nec6`, re-verified by the Steward
> against `5bac56000`). As `D1` shipped it,
> `record_branched_scrutinee_unit_body_route1` has **exactly one call site**: the
> `else` of the `let ... else` at `core.rs:15754-15755`, immediately before the
> route-1 `return Ok(None)`. **Nothing records at function entry**, and nothing
> records at the earlier `return Err` at `:15749`.
>
> ⇒ At `D2`, `route1 == 0` means *either* entered-and-advanced (the success)
> *or* never entered — bailed at `:15749`, or the single caller at `:16027`
> changed. **The instrument records nothing that separates them**, so the
> criterion as originally written is discharged by a measurement equally
> consistent with the port being unreachable.
>
> **The pattern is 14,700 lines up in the same file and was written for this
> hazard.** `MatchRecursorCensusRow` (`core.rs:983-1008`) pushes a row for every
> entry and partitions with `reached_selector`. Its comment states the rule in
> the imperative: *"a census of firings is a numerator, not a population"*
> (`:974-976`), and *"a silently empty census and a genuinely empty population
> are the two readings this must never conflate"* (`:979-981`). **The `D1`
> observer copies that helper's scope machinery verbatim** — identical `Restore`
> shape, `:1042-1061` against `:1085-1103` — **and drops both properties the
> machinery exists to carry.**
>
> **`D1`'s own AC-1 measurement is unaffected and remains tight.** The Adversary
> mutated the function to push a sentinel row at entry and re-ran: the count went
> 1 -> 2, so the function is entered exactly once per compile and that entry
> takes route 1. The defect is that the instrument stops being able to say so the
> moment route 1 stops firing — it is sound as a numerator and unsound as a
> denominator, and `D2` is the first deliverable that needs the denominator.

> ### THE `D1` PIN WILL GO RED INSIDE `D2`'s DIFF, ASSERTING A REGRESSION
>
> `crates/ken-cli/tests/rt_branched_scrutinee_unit_body_port.rs:93` asserts
> `route1.len() == 1` with the message *"the two-arm plain Match must take route
> 1 exactly once"*, and `:117` asserts `result.expect_err("D1 stops at the
> route-1 refusal")`. Both hold only while route 1 fires, so `D2` succeeding
> turns both red — **the first with text stating the opposite of what happened.**
>
> The callout below already warns that the advance "will read as this node
> failing when what happened is that it succeeded". **The pin manufactures
> exactly that misreading**, and a reviewer meets it cold inside the diff.
> `D2` updates the pin and puts one line in the test's doc header saying a red
> there is the AC-3 advance.

> ### DO NOT WRITE AN AC THAT SAYS THE WITNESS PASSES. ROUTES 2 AND 3 ARE UNMEASURED.
>
> Once the scrutinee resolves, the argument may still be a captured
> `LexicalClosure` (route 2) or a `Checked*` IH form (route 3). Those routes were
> **path-unreachable** while route 1 fired, so **their zero counts are counts and
> not evidence** — `StaticWorker` is ruled out as the *first* blocker only, never
> as a requirement of the port.
>
> **An advancing refusal is the evidence this increment worked.** A frame that
> demands the witness compile is demanding an unmeasured number of further ports,
> and it will read as this node failing when what happened is that it succeeded
> and the next gap surfaced. If the refusal advances to route 2 or 3, **file the
> next node and close this one** — do not extend scope inside it.

**AC-4.** Arm disagreement refuses with a message naming disagreement as the
cause, and a control shows the refusal is reachable — a witness whose two arms
declare different body units. **An untested refusal arm is not a refusal arm.**

**AC-5.** No regression across the workspace, green in CI. Local runs stay
targeted (`scripts/ken-cargo`, `-p ken-runtime`, plus `-p ken-cli` for the parity
suites); a full `--workspace` run is CI's job, never the laptop's.

## Contention

- **[[NATIVE-HANDLE-CARRIER]] must land its recut first**, which is why this node
  depends on it. The recut deletes the rows this gap was measured through; a
  fixture authored here before then would be edited by two seats at once.
- `core.rs` lowering is runtime's alone. No other lane touches
  `recursive_position_unit_body`.
- **Not the closed `RecursiveDescent` residual lineage.** A `Call`-shaped
  scrutinee would have put this in `RT-FNSPLIT-B2F` / [[RT-FNSPLIT-RECUR-PORT]]
  and required reopening them. The scrutinee is a match, so this is a new
  capability in the lowerer and those nodes stay closed.

## The edge this node exists to hold

[[PX8-F-CAP-41]]'s `depends_on` must name this node before
[[NATIVE-HANDLE-CARRIER]] is flipped `merged` **or** `closed`. `gen-progress.sh`
clears a dependency on **either** status, so closing the carrier with its residual
unowned would falsely unblock PX8 Phase 2 — the residual would have no owner and
the graph would report the work as available.
