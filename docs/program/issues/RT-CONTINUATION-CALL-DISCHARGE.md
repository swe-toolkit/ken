---
id: RT-CONTINUATION-CALL-DISCHARGE
title: "A planned continuation call is neither directly emitted nor compositionally consumed once the Active resume path goes live — attribution, not repair"
status: merged
owner: runtime
size: S
gate: none
depends_on: [RT-SPECIALIZED-ACTIVE-RESUME]
blocks: [RT-CONTINUATION-EDGE-DISPOSITION]
github: null
origin: Architect ruling evt_vxqa83y4z3nt (2026-08-08) on the RT-SPECIALIZED-ACTIVE-RESUME D2/D3 sixth wall at exact d9175d05, with the Steward cut and release ruling evt_27jwdbz9h2t4c. Campaign docs/program/16-recursive-descent-retirement.md node #6h. Steward-filed (agents cannot create tracked work per COORDINATION §2).
---

> # CLOSED 2026-08-09. THE ATTRIBUTION IS DELIVERED AND `D2`/`D3` CLOSED AT THEIR NEW HOME.
>
> **The node was left `active` after `D2`/`D3` were withdrawn from it, and the
> tracker was never flipped once the remainder landed.** Verified against **its
> own ACs**:
>
> - **`AC-1`/`AC-2`** — the `D0`/`D1` attribution record is on `main`. It
>   establishes that every measured member has `pending_len=0`, so the activated
>   path reaches an empty `Active` resume **with no call** while the planner has
>   already minted one, and it refutes the other two classifications rather than
>   asserting one.
> - **`AC-3` to `AC-7` were RE-HOMED, not met here.** They describe *the
>   repair*, which Architect hard stop `evt_dakdkqk4wbg6` withdrew to
>   [[RT-CONTINUATION-EDGE-DISPOSITION]] (#6i) — **merged and closed
>   2026-08-09**, discharging them there. This is recorded in the frame's AC
>   table too, because the table alone reads as five unmet ACs on a closed node.
> - **`AC-8`/`AC-9`** — no `#[ignore]` added; `main` green in CI, zero failures.
>
> **`65639a13` and `a15a3e934766a1d075386ba561a9469e51a448b7` remain HELD
> EVIDENCE and must never be published.** The string `a15a3e93bd76...` that
> circulated is **NOT AN OBJECT** — it shares its first eight characters with
> the real one. `git cat-file -t` every SHA.

> # THIS IS THE SIXTH WALL, AND IT IS THE FIRST PLANNER-POPULATION AUTHORITY.

The first four were `BoundaryCarrier` refusals about how a carried operand may
**cross or be consumed**. The fifth ([[RT-SPECIALIZED-ACTIVE-RESUME]]) was about
the **value shape** a scrutinee has after ordinary lowering. **This one is
neither.** It is about whether the **planned population was discharged** — a
question no earlier wall on this chain has asked.

**This node is an ATTRIBUTION node. It does not begin with a repair.** Which
side is wrong is exactly what is unknown, and the deliverable is a classification
backed by a trace.

## What it is

Owner: **`ContinuationClaimLedger::close`**, `lowering/units.rs:3311`, at the
set-equality check on `units.rs:3362`.

```
the discharged continuation call population is not the planned one: 1 planned
tokens were neither directly emitted nor compositionally consumed, and 0
discharged tokens were never planned. Direct: 0, composed: 0
```

The law is **exact set equality, not counts**:

```
planned = direct-emitted  ⊎  composed-consumed
```

The two forms **partition** the planned population — `close` refuses an identity
appearing in both, on the ground that one obligation was answered twice.

## Why it only appears now

Under the fifth wall this program **aborted before `close()` ever ran**.
[[RT-SPECIALIZED-ACTIVE-RESUME]]'s `D2` cleared that stop for the routed cell,
so the ledger became reachable for this shape **for the first time**.

⇒ **Campaign Trap 2 — a fail-closed invariant meeting a newly reachable
population. It is not a defect in that `D2`**, and this node does not reopen it
or any of the five landed repairs.

## The owned fact

For every measured member `active.pending` is **empty**, and
`resume_active_continuation` **returns its operand unchanged**. So the activated
path reaches an empty `Active` resume **with no call**, while the planner has
already minted the causal call.

**That proves a planner/lowering obligation mismatch. It does not say which side
is wrong.** Both readings are live, and `pending_len == 0` is evidence for
neither.

> ### DO NOT DISCHARGE THE TOKEN IN THE EMPTY RESUME
>
> **Architect, explicit** (`evt_vxqa83y4z3nt`). Recording a discharge there
> would be **false evidence**:
>
> - it is **not direct-emitted** — no specialization call instruction was
>   emitted and decoded; and
> - it is **not composed-consumed** — that set is fed only after a recorded raw
>   worker call is found in **finished CLIF**, checked against the planned
>   worker, operands and result, and shown to return downstream.
>
> The ledger's own comment at `units.rs:3059` states that `composed` is fed
> **from `function_local.composed_discharges` and from nothing else**, because
> a composed instruction targets the raw worker while the direct gate requires
> the recorded instruction to decode to `identity.target()`. An identity in both
> sets would mean one of the two gates had been loosened.
>
> **Do not weaken the law, bulk-claim the token, manufacture a composed
> discharge, or treat an identity return as a call.**

## `D1` classifies exactly one of three. It does not pick a favourite

Trace the exact missing `ContinuationCallIdentity` — construct origin,
continuation origin, alternative, recursive position, call-site sequence,
target, and emission owner — **through the same program in both lanes**, then
classify:

1. **A real direct obligation was skipped.** Repair the actual producer/call
   seat and **retain finished-CLIF verification**.
2. **A real composed consumption occurred but its evidence was lost.** Restore
   the verified composed relation. **Do not claim it from the resume.**
3. **The activated path has no causal call obligation.** Correct the planner's
   issuance/projection **at planner authority**, proving why this exact path is
   not a member. **Do not infer that from `pending_len == 0` alone.**

**The planner is implicated but not convicted.** It mints the causal call while
discovering a recursive closure position under a computational continuation,
once the source environment is available — and **that mint is independent of the
test-only lane exclusion.**

**One fact bears directly on option 3 and is already recorded in the code.**
`ContinuationClaimLedger::open` carries an honest note that `planned == resolved`
is **structural today**, because `resolve_continuation_targets` walks the same
projection; the two would separate only if resolution ever dropped or added a
key. So a projection-level correction is not a free relabelling — **it moves the
set that `close` is checking against.**

## The retained lane is the control, and the population floor is two

**The retained lane closes the same program.** Record whether the same identity
is discharged **directly or compositionally** there. That is the discriminator,
and it is a non-degenerate pair on a shared input rather than a counter.

The **two independent A rows are the population floor, not the perimeter.** This
campaign has read a small-witness result as a class-wide property four times and
every correction cost more than the census would have.

> ### THE HELD LANE-PAIR OBJECT IS THIS NODE'S END-STATE ACCEPTANCE CONTROL
>
> `65639a13` on `runtime-implementer/sar-lane-pair-evidence` is **evidence, not
> a candidate, and it is not published.** It was correctly held rather than
> committed: divergence is the campaign premise failing, so committing it red is
> impermissible and weakening it to pass would absorb the very stop it detected.
>
> **This node inherits it as the exact assertion it must satisfy.** The end
> state is that the activated lane closes the same program the retained lane
> closes.

## Scope

Gates completion of [[RT-MATCH-RECURSOR-CONSUMERS]] and its `AC-1`. Does **not**
reopen [[RT-SPECIALIZED-ACTIVE-RESUME]]'s accepted `D2`/`D3`, or any of the five
landed repairs. Does not touch rows 1-5 or the `LexicalCallArgumentRecursor`
population ([[RT-LEXICAL-RECURSOR-CONSUMERS]]).

Frame: `docs/program/wp/RT-CONTINUATION-CALL-DISCHARGE.md`.

## Sizing is provisional, and `D0`/`D1` may overturn it

**`M`, provisional.** The three-way classification is a measurement, and which
branch it lands on changes the work by more than a size step: option 3 is a
planner-authority correction, option 1 touches a producer/call seat, option 2 is
an evidence-plumbing repair. **Re-size on the `D0`/`D1` handback**, as #6f and
#6g both were.

## WITHDRAWN: the option-3 ruling. `D2`/`D3` are re-homed, `D0`/`D1` stand

**Architect hard-stop ruling `evt_dakdkqk4wbg6`, 2026-08-08. This section
replaces the "RULED: OPTION 3 — the planner over-issues, re-sized `M` to `S`"
block that stood here.** That block is **false** and was removed rather than
annotated, because its claim sat in a heading where an appended correction does
not reach.

### What survives, and it is the whole point of an attribution node

**The exact-witness conclusion is UNCHANGED: no call occurred.** `D0`'s trace
and `D1`'s refutation of option 2 stand, and so does the 213-identity census
(`DIRECT 170`, `COMPOSED 34`, nine committed controls, no independent program
reaching `close` undischarged). **This node was filed as an ATTRIBUTION node
that does not begin with a repair, and attribution is exactly what it
delivered.**

### What is withdrawn, and why it is not a defect in `D0`/`D1`

**Option 3 is not implementable as planner-side edge exclusion.** The ruling it
rested on conflated **two roles carried by one planner edge**:

| role | who needs it |
|---|---|
| **binding projection** | the deferred constructor environment needs worker provenance to install IH / static-worker bindings at recursive positions |
| **causal call obligation** | only a direct specialization call, or a verified composed raw-worker call, owes a `ContinuationCallIdentity` discharge |

**Bridge selection cannot distinguish them**, and thirty-four bridge-taken edges
are genuinely compositionally consumed. Restricting to the ordinary arm still
conflates the ruled witness with `d8e`: **their planner coordinates are
identical**, and the semantic difference appears only when the arm body resolves
its de Bruijn callee against the materialized environment.

Both available narrowings are **real failures, not missing predicates**:

- removing the edge **before interning** loses the binding, so `d8e` compiles
  with a shifted environment;
- removing only `calls.insert` leaves an **interned-unit / caller population
  contradiction**.

> ### THE RED CONTROL IS LOAD-BEARING, AND COMMITTING IT RED WAS CORRECT
>
> Held object **`a15a3e934766a1d075386ba561a9469e51a448b7`** — **never
> published**, with `65639a13` and `aa78c973`. It is what proved the conflation.
> A control edited until it passes stops being evidence of anything.
>
> **The string `a15a3e93bd76...` that circulated in the handoff is NOT AN
> OBJECT.** It shares the first eight characters with the real one. Resolve this
> SHA with `git cat-file -t` before it enters a Decision; a prefix match is not
> an existence check.

### Disposition

`D2` and `D3` are **withdrawn from this node** and re-homed to
[[RT-CONTINUATION-EDGE-DISPOSITION]], the seventh authority. **This is the
frame's §5 outcome — a distinct authority, not an enlargement of `D2`** — and
the campaign's standing record is that treating a new authority as a defect in
the previous repair is the expensive mistake. **Do not re-open the five landed
repairs or [[RT-SPECIALIZED-ACTIVE-RESUME]]'s accepted `D2`/`D3` over this.**

**Sizing is retired rather than corrected.** `S` was granted against an
edge-exclusion repair that no longer exists; re-sizing this node would size work
it no longer carries. The successor is sized on its own frame.

**Green partial:** `2e267180dcbdb7a59df59edf0dde9924925cb7d5`, the `D0`/`D1`
attribution record, proceeds through fresh exact-SHA QA and a Decision that
**excludes all `D2`/`D3` mechanism and the held object**. Its own record says
the classification did not close, and it is `crates/`-identical.
