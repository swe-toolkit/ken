---
id: RT-CONTINUATION-EDGE-DISPOSITION
title: "One planner edge carries both binding projection and a causal call obligation — split the representation so a binding candidate can be settled InlineNoCall without ever entering the call-discharge partition"
status: active
owner: runtime
size: M
gate: none
depends_on: [RT-CONTINUATION-CALL-DISCHARGE]
blocks: [RT-MATCH-RECURSOR-CONSUMERS]
github: null
origin: Architect hard-stop ruling evt_dakdkqk4wbg6 (2026-08-08), which accepted the held red control a15a3e934766a1d075386ba561a9469e51a448b7 as load-bearing and withdrew the planner-side option-3 mechanism it had previously ruled at evt_4ebpfvfrvv8qy. Predecessor RT-CONTINUATION-CALL-DISCHARGE delivered D0/D1 attribution; its D2/D3 are re-homed here. Campaign docs/program/16-recursive-descent-retirement.md node #6i. Steward-filed (agents cannot create tracked work per COORDINATION §2).
---

> ## `D0` IS MERGED, AND IT MOVED `D2`/`D3` RATHER THAN THE REPRESENTATION
>
> **Landed 2026-08-09 at exact `e93afb06783d7d7eff81a137ef6f92f1095411e6`,
> base `6be73d20`, PR #1659.** Record only — one path, `+207/-0`, `crates/`
> byte-identical to the base (tree `4c2bc579` both sides). Record:
> `docs/program/wp/RT-CONTINUATION-EDGE-DISPOSITION-D0.md`.
>
> **637 candidates, one disposition each, zero orphans.** `DIRECT` 193,
> `COMPOSED` 43, `BOTH` **0**, `INLINE_NO_CALL` 21, `BRIDGE_INCOMPLETE` 25,
> `PLANNED_ONLY` 355. `BRIDGE_INCOMPLETE` is a bridge scope **entered and not
> completed**, kept separate on purpose — the frame settles `InlineNoCall` only
> on a scope that **completes**, and folding the two would manufacture members.
>
> **RULED `evt_40rf074xsj3y1`: the 210-of-637 result is NOT a second hard
> stop.** The counts stand — 427 candidates carry `CLOSE_CHECKED = false`,
> including 52 `DIRECT` and 11 `COMPOSED` — and **637 is retained as the
> observational superpopulation**. What is withdrawn is the causal reading.
> **There is no per-owner closeout.** One artifact-wide ledger opens in the
> selected `FunctionizedUnits` arm before `define_unit_bodies`, is seeded from
> the plan's full `continuation_calls()`, and closes only after every definition
> pass and the root adapter succeed. ⇒ `CLOSE_CHECKED = false` means **that
> compile never reached a successful functionized-artifact closure, or selected
> another authority** — it does **not** show a healthy candidate lying outside a
> successful closeout's authority.
>
> **`D2`'s quantifier is narrower and exact:** every activated binding candidate
> in **one selected `FunctionizedUnits` artifact**, settled once before that
> artifact closes. **Plan-only rows, `Err` compilations, and non-selected
> `RecursiveDescent` plans are not obligations.** The candidate layer shares the
> ledger's artifact lifetime and does not widen it, add a per-owner close, or
> traverse failed compilations.
>
> **`AC-7` is measured OPEN, and it is the WITNESS CELL that is empty, not the
> class** — `D0` measured **21** `InlineNoCall` members. No
> `InlineNoCall` member has a binding, a closeout, **and** a successful compile.
> The three closeout-visible members are this campaign's own controls
> (`ccr_d3`, `coc_d3`, `sar_d3`), all refusing; the two inside successful
> compiles carry `CLOSE_CHECKED = false`, so they compile **because nothing
> looked**, not because anything was discharged. Counting them is exactly
> **Trap 3**. ⇒ **The witness must be AUTHORED, not found.** That is a
> measurement, not a shortfall in the search. *(Superseded in one detail by
> `evt_5n735c2e9r52k`: authoring is split — `D1` writes the refusing witness,
> `D2` converts it to compile-OK. See the phase block below.)*
>
> **THE §4 STOP IS UNFIRED AT `D0`, AND `D0` OWES NOTHING FURTHER ON IT.**
> Verbatim condition: **`UNFIRED AT D0; re-route only if D1 changes unit
> population, declaration, definition, ABI projection, or traversal.`**
> `px8j` is **non-selected**, measured with a same-run probe-alive control, so
> its `b2f_last_unit_emission() == (0, 0)` is a non-selected-authority result
> rather than a blind instrument. **No further direct ABI probe is required.**
> The selected-side reachability controls are **`sar_d3`, `ccr_d3` and
> `coc_d3`** — any later reachability claim is made against those, not `px8j`.
>
> **Unfired is not cleared.** The condition was evaluated and did not hold, so
> it stays live as a **`D1`** obligation on the five named axes.
>
> ### `AC-7` IS SPLIT ACROSS `D1` AND `D2` — ruling `evt_5n735c2e9r52k`
>
> **`D1` owes a real REFUSING witness and MUST NOT claim compile success.**
> This follows from the released component boundary: `open` seeds `planned`
> from the full `plan.continuation_calls()` projection, the unchanged `close`
> requires the disjoint exact equality `planned = emitted ∪ composed`, and a
> genuine `InlineNoCall` candidate is in the first set and in **neither**
> discharge set. ⇒ compile-OK before `D2` is impossible without weakening the
> law, silently doing `D2`, or reviving the withdrawn planner-side exclusion.
>
> The `D1` witness is non-vacuous because it pins **selection of a real
> `FunctionizedUnits` artifact, binding installation, disposition settlement
> after the deferred bridge succeeds, and arrival at the existing `close`**
> producing the **exact pre-`D2` missing-call refusal**.
>
> **`D2` converts that SAME witness to compile-OK**, after total/disjoint
> disposition close and subset derivation — the node's only lawful population
> change. **`D3` consumes the post-`D2` successful witness, and no `D3` control
> may substitute `D1`'s refusal for `D2`'s success.**
>
> **This is a phase correction, not a weakening.** The final bar is still a real
> binding-installed, closeout-checked, compile-OK member.
>
> ### `D1` IS MERGED at exact `fbc49ddd`, PR #1667, CI GREEN — 2026-08-09
>
> Base `71646eb1`, seven paths `+623/-4`: six `ken-runtime` lowering/test paths
> plus the amended `D0` correction record. Decision `dec_x31ydz3ac76p` resolved
> APPROVED by the Architect; Runtime QA approved the same exact SHA at
> `evt_1skf7xxyhm8x3`. **Blob identity verified on all seven paths against the
> landed tree.** Adversary notified. Size accepted as **M**, on the census plus
> this cut rather than inherited.
>
> **What landed:** the sibling candidate ledger — opened and taken at the **same
> artifact boundary** as the claim ledger, keyed from the same
> `ContinuationCallIdentity` projection — with three bounded settlement seats:
> `DirectCall` after claim/emit, `ComposedCall` after the existing verified-feed
> double-discharge refusal, `InlineNoCall` after an `Ok` bridge result while
> neither settled nor pending-composed. **Dispositions only**: no totality
> check, no subset derivation.
>
> **The `AC-7` witness REFUSES, and the exact string is the discriminator.** A
> green result would mean `D2` done early, a weakened equality, or the withdrawn
> planner-side exclusion returning — the first two green it and the third
> deletes the refusal, **so a control keyed on "it failed" would pass under all
> three and this one passes under none.** It is a transition sentinel named for
> the boundary, so `D2` inverts it under review rather than leaving it to drift.
>
> **Two held objects are now superseded and must never be published:**
> `652df2ea` and `487a06cc`. `a504aa96` was never publishable — it reverts the
> erratum.
>
> ### `D2` IS RELEASED — 2026-08-09. `D3` REMAINS STOPPED.
>
> **Released after the ring confirmed its reading of `D2`'s quantifier
> matches the ruling** (`evt_1kcyfdn7076yt`), which was the one gate I held it
> on. That check is not ceremony: **`D2` ranging over the wrong population
> would not red — it would go green and mean nothing**, which is exactly the
> failure the `D0` erratum existed to prevent.
>
> **The confirmed population**, verbatim in substance: every **activated binding
> candidate in one selected SUCCESSFUL `FunctionizedUnits` artifact**, each with
> exactly one disposition before that artifact closes. It is **neither** `D0`'s
> 637-row observational superpopulation **nor** all planned calls. Plan-only
> rows, `Err` compilations, and non-selected `RecursiveDescent` plans are
> **excluded by construction, not by post-hoc filtering** — that distinction is
> load-bearing, because a post-hoc filter is a place where the population can be
> quietly widened later.
>
> **Order, and the order IS the mechanism:** total/disjoint disposition close
> **first**; only then derive `DirectCall ∪ ComposedCall`; only then apply the
> **unchanged** exact equality. `InlineNoCall` stays outside the equality.
> **Compile-OK is required only here.**
>
> **`D2` converts the SAME `sar_d3`-shaped `D1` witness** from its exact
> missing-call refusal to compile-OK, and **must invert the transition sentinel
> under review**. The sentinel was named for the boundary precisely so this is
> visible rather than left asserting a refusal that no longer happens.
>
> ### ADVERSARY HUNT ON MERGED `f61b0b0d` — NO DEFECT, ONE ITEM UNTOUCHED
>
> **Cleared, so do not re-derive.** Shared artifact lifetime is **structural**:
> enumerated across `crates/`, exactly one open (`units.rs:3584`, inside
> `open_continuation_claim_ledger`, unconditional, no early return before the
> claim ledger's own assignment) and exactly one take (`units.rs:3609`, inside
> `close_continuation_claim_ledger`); everything else only `as_mut()`. **The
> silent failure mode is already inverted** — the take is
> `…take().ok_or_else(|| backend_module("the continuation candidate ledger went
> missing"))`, so a desync fails loudly. **Do not weaken that `ok_or_else`.**
> The refusal sentence has **one producer** today: `units.rs:3495` and the
> control constant at `control.rs:27427`, two occurrences in all of `crates/`.
>
> **The `ComposedCall` seat's ordering is MEASURED, not reasoned.** Settling
> first *did* preempt the law, replacing `d8f`'s `"discharged twice in a single
> function"` with a candidate-ledger message, which is why it now sits after the
> double-discharge refusal. ⇒ The invariant for all three seats: **a layer in
> front of the law must not preempt the law's own refusals; it derives from
> them, it does not speak for them.**
>
> **FOLDED INTO `D2` (`evt_39cbpgvqqb7n9`), not filed as new nodes:**
>
> 1. **`InlineNoCall`'s two negative conditions are UNCHECKED** — the highest-
>    value remaining item on this node. It settles on an `Ok` bridge result plus
>    *neither settled, nor pending-composed*; negative conditions read as safe
>    and are what rots when a fourth path to consumption appears. **`D2` is
>    where the population becomes total, so `D2` is when a fourth path starts
>    mattering.**
> 2. **A shared `const` between the production refusal and the control** would
>    collapse the "one producer *today*" residual into a compile-time link. **A
>    mechanism choice, offered not mandated** — a decline gets recorded as an
>    accepted trade so it does not resurface.
>
> **Still open, carried, NOT cleared:** `core/tests/effects.rs`'s six per-role
> fixtures are characterized but never verified to discriminate from one
> another. This cut's `+1` there is inert.
>
> ### `D2` IS MERGED at exact `495b0b36`, PR #1671, CI GREEN — 2026-08-09
>
> Original base `c6e6f23f`, three paths `+465/-131` (`lowering/core.rs`,
> `lowering/units.rs`, `lowering/core/tests/control.rs`). Decision
> `dec_6dhj49m7mqdn6` resolved APPROVED by the Architect **for this exact SHA
> only — no earlier `D2` verdict transfers.** Blob identity verified on all
> three paths against the landed tree. Adversary notified.
>
> **SUPERSEDED, NOT PUBLISHABLE:** `ebbc1ccc`, `b339f15c`, `008b9a15`.
>
> **The lifecycle is phase-accurate.** Candidate and claim ledgers share the
> selected `FunctionizedUnits` artifact lifetime; candidate population comes
> from the exact planned call/binding projection; settlement is **affine**; and
> **candidate totality closes BEFORE** the `DirectCall ∪ ComposedCall` subset is
> derived. `DirectCall` settles only after the shared resolved-continuation
> funnel returns `Ok` — covering **both** retained-frame and detached-result
> consumers; `InlineNoCall` only after a successful bridge result with neither
> prior settlement nor pending composed discharge; `ComposedCall` only after
> finished-CLIF verification and the double-discharge refusal.
>
> **Which law ranges over which population, and this is the whole point of the
> node:** `resolved` and `declared` stay exact over **full planned**;
> `discharged` and `claimed` are exact over the **derived call-obligation
> subset**. An unclaimed `InlineNoCall` is lawful; an extra inline claim and a
> missing obligation are both refused. `InlineNoCall` never enters the equality.
>
> **The sentinel inverted under review, as designed.** `D1`'s refusal string is
> now labelled **historical/free rather than an oracle** — the one drift I
> flagged at `D2`'s release, and it did not happen. The stale-claim sweep found
> no surviving singular-law assertion; the lone remaining "discharged population
> is not the planned one" is the intentional mismatch diagnostic.
>
> ### `D3` IS RELEASED — 2026-08-09, on the merge confirmation, no added gate
>
> **I said `D3` would release on `D2`'s handback with no extra condition and
> that is what happened.** The only reason it waited at all is that `D3`'s
> mutations consume the **post-`D2` successful witness**, which was not a merged
> fact until CI reported.
>
> **The five mutations must red INDEPENDENTLY** — each proven on its own, not
> the suite reddening when all five are applied. **No `D3` control may
> substitute `D1`'s refusal for `D2`'s success**; a mutation proven against the
> refusing witness proves nothing about the repaired path and is the cheapest
> available way to make this node's whole proof vacuous.
>
> **Check whether each control is free before writing it** — the campaign's
> standing trap is a control asserting the absence of a refusal the repair just
> deleted from production. And **derive witnesses; do not pin counts or literal
> indices.**
>
> **`D1` was released 2026-08-09 against the refusing-witness contract and has
> now MERGED — that release is history, not a pending instruction.**
>
> **`D2` and `D3` stay stopped, and this is not caution — it is a missing
> input.** `D1` authored the **refusing** witness, which is what it owed. `D3`'s
> five mutations need the **post-`D2` SUCCESSFUL** witness, and that does not
> exist until `D2` converts the refusing one after total/disjoint disposition
> close and subset derivation. **`D3` before `D2` would have to mutate against
> the refusal**, which proves nothing about the repaired path and is the
> cheapest available way to make this node's whole proof vacuous.
>
> **`AC-7` stays open** — it is `D1`'s to discharge, against the empty witness
> cell, not the class.
>
> **Sizing stays `TBD`, and I want the number at `D1`'s checkpoint, not before
> it.** The two inputs that made `D0` decline to size are now settled — the
> 427-candidate reading is withdrawn and the stop is unfired — but the third,
> the cost of authoring a witness, is exactly what `D1` measures first.
>
> **The merged `D0` is valid as a census artifact** — no code revert and no
> Runtime production work is requested. Its causal interpretation is what was
> corrected, in this node, its frame, the `D0` record, and campaign row #6i.

> # THIS IS THE SEVENTH WALL, AND IT IS A REPRESENTATION SPLIT, NOT A REPAIR.

The first four were `BoundaryCarrier` refusals about how a carried operand may
cross or be consumed. The fifth was the value shape of a scrutinee after
ordinary lowering. The sixth ([[RT-CONTINUATION-CALL-DISCHARGE]]) asked whether
the planned population was discharged, and answered it: **no call occurred.**
**This one is what that answer costs**, and it is the first wall on this chain
whose deliverable is a new representation rather than a correction to an
existing one.

## The owned fact

**One planner edge carries two roles, and bridge selection cannot distinguish
them.**

| role | who needs it |
|---|---|
| **binding projection** | the deferred constructor environment needs the worker provenance to install IH / static-worker bindings at recursive positions |
| **causal call obligation** | only a direct specialization call, or a verified composed raw-worker call, owes a `ContinuationCallIdentity` discharge |

**Thirty-four bridge-taken edges are genuinely compositionally consumed**, so the
bridge is not a proxy for the distinction. Restricting to the ordinary arm does
not separate them either: the ruled witness and `d8e` have **identical planner
coordinates**, and the semantic difference appears only when the arm body
resolves its de Bruijn callee against the materialized environment.

> ### THE TWO OBVIOUS NARROWINGS ARE REAL FAILURES, NOT MISSING PREDICATES
>
> - Removing the edge **before interning** loses the binding, so `d8e` compiles
>   with a **shifted environment**.
> - Removing only `calls.insert` leaves an **interned-unit / caller population
>   contradiction**.
>
> **Do not look for a stronger source-side predicate.** That is the move this
> node exists to rule out; it was tried, and the failure is in the
> representation, not in the sharpness of the test.

## The mechanism the Architect ruled

**A candidate/disposition layer IN FRONT OF the unchanged causal-call
partition.** The planner mints an opaque **binding candidate** carrying the
exact worker provenance and selector. Its existence **authorizes environment
installation but does not assert a causal call.**

Lowering settles each candidate **exactly once**, from an event only lowering
can observe:

| disposition | settled when |
|---|---|
| `DirectCall` | at the verified direct producer / call seat |
| `ComposedCall` | only after the raw-worker call is emitted **and enters the existing finished-CLIF verification** |
| `InlineNoCall` | only after the exact deferred bridge scope **completes successfully** with that candidate still unconsumed |

A **static-worker binding carries the candidate authority.** Actual
source-machine consumption promotes it to `ComposedCall`; a **value-position
read still reaches the existing fail-closed `StaticWorkerBinding` guard**, so
`d8e` must retain binding count 1 and refuse.

**Closeout requires an exact, disjoint disposition for every candidate first.**
It then derives the call-obligation subset from `DirectCall ∪ ComposedCall` and
applies the existing law **unchanged**:

```
call obligations = direct-emitted  ⊎  composed-consumed
```

> ### `InlineNoCall` IS NEVER A DISCHARGE AND NEVER ENTERS THAT EQUALITY
>
> **This is deliberately NOT "add a third discharge form".** A third arm in the
> partition would falsify the meaning of the call ledger — it would let a
> program with no call satisfy a law that exists to say a call was answered.
> The new layer sits **in front of** the partition; the partition itself is
> untouched.

## Measurements come before mechanism

1. **Census the full candidate/unit population** by installed binding, direct
   emission, verified composed consumption, successful inline completion, and
   unresolved-or-double disposition.
2. **Preserve the four-cell `d8e` table as the primary discriminator.** Both
   classified variants keep **one** binding; index 1 may finish inline, while
   index 2 **must still refuse in value position**.
3. **Measure declaration/definition and ABI reachability for `InlineNoCall`
   candidates.** If permitting a binding-only candidate requires a post-lowering
   **call-graph rebuild** or changes the **planner traversal contract**, **stop
   again** rather than silently allowing an uncalled executable unit.
4. **Five mutations must independently red:** suppress binding installation;
   mark inline **before** bridge completion; mark inline **after** a composed
   call; omit a final disposition; present one candidate in **two** dispositions.
5. **Untouched until the split representation proves otherwise:**
   `ContinuationClaimLedger::close`, finished-CLIF direct and composed
   verification, the both-sets refusal, the `composed` feed, the empty resume,
   and all five landed repairs.

## Scope

Gates completion of [[RT-MATCH-RECURSOR-CONSUMERS]] and its `AC-1`. Does **not**
reopen [[RT-SPECIALIZED-ACTIVE-RESUME]]'s accepted `D2`/`D3`,
[[RT-CONTINUATION-CALL-DISCHARGE]]'s `D0`/`D1`, or any of the five landed
repairs. Does not touch rows 1-5 or the `LexicalCallArgumentRecursor` population
([[RT-LEXICAL-RECURSOR-CONSUMERS]]).

> ### THE PREDECESSOR IS NOT WRONG, AND ITS `D0`/`D1` ARE THE INPUT HERE
>
> **A seventh authority is a normal outcome on this chain, not a defect in the
> sixth.** The campaign's standing record is that the expensive mistake has
> always been treating a new authority as a fault in the previous repair. The
> exact-witness conclusion **"no call occurred" is unchanged and is load-bearing
> for this node** — it is why `InlineNoCall` must exist at all.

## Sizing is `TBD` on purpose

**Do not inherit the predecessor's `S`.** That size was granted against an
edge-exclusion repair that the ruling withdrew. This node's work is a new
representation plus a five-mutation proof obligation, and measurement 3 carries
a **named hard stop** that would fork it again. **Size it on its frame, after
the census in measurement 1.**

Frame: `docs/program/wp/RT-CONTINUATION-EDGE-DISPOSITION.md`.
