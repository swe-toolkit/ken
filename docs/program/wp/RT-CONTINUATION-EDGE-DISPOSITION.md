# RT-CONTINUATION-EDGE-DISPOSITION — split the edge, keep the law

**One planner edge carries both binding projection and a causal call
obligation. Split the representation so a binding can be installed without
asserting a call, and so a candidate can be settled `InlineNoCall` without ever
entering the discharge partition. The partition itself does not change.**

**Owner:** Team Runtime. **Branch:** `wp/RT-CONTINUATION-EDGE-DISPOSITION`.
**Size:** `TBD` — **`D0`'s census sizes it, and the frame will not guess.**
**Risk:** high — a new representation in front of a fail-closed law, with a
named hard stop that can fork the node again.

**Read `docs/program/16-recursive-descent-retirement.md` first** (node **#6i**),
then [[RT-CONTINUATION-CALL-DISCHARGE]]'s `D0`/`D1` record on `main` at
`docs/program/wp/RT-CONTINUATION-CALL-DISCHARGE-D0-D1.md`. **That record is this
node's input, not its background.**

> ### DO NOT INHERIT THE PREDECESSOR'S `S`
>
> `S` was priced against an **edge-exclusion** repair, and the Architect
> **withdrew** that mechanism (`evt_dakdkqk4wbg6`). Carrying the number forward
> is the single easiest way to under-size a representation split. **The size is
> `TBD` until `D0` reports.**

---

## 1. Fixed inputs

**Measure every one at your own pinned base.** These are **anchors to re-find,
never values to trust.** **Cite by grep-able phrase, not by line number** —
coordinates on this chain have rotted inside a single merge window three times,
most recently when `PX8-ERRID-ALLOC` moved `planning/static_transition.rs`.

| input | anchor |
|---|---|
| the two roles | **binding projection** (deferred constructor environment installs IH / static-worker bindings at recursive positions) versus **causal call obligation** (only a direct specialization call, or a verified composed raw-worker call, owes a discharge) |
| why the bridge cannot decide | **34 bridge-taken edges are genuinely compositionally consumed**, so bridge selection is not a proxy for the distinction |
| why the ordinary arm cannot decide | the ruled witness and `d8e` have **identical planner coordinates**; they differ only in the de Bruijn callee the arm body resolves against the materialized environment |
| the two refuted narrowings | removing the edge **before interning** loses the binding and lets `d8e` compile in a shifted environment; removing only `calls.insert` leaves an **interned unit with no caller** |
| the law, unchanged | `call obligations = direct-emitted ⊎ composed-consumed`, exact **set** equality, in `ContinuationClaimLedger::close` |
| the guard that must still fire | the fail-closed `StaticWorkerBinding` guard on a **value-position read** |
| your base | **not fixed here.** Branch from `main` and pin it in your first checkpoint post |

## 2. What is owed

### `D0` — the census, before any mechanism

Census the **full candidate/unit population** by: installed binding, direct
emission, verified composed consumption, successful inline completion, and
**unresolved-or-double disposition**.

**This is the deliverable that sizes the node**, and it is also the instrument
that catches the hard stop early. Report denominators, one disposition per
member, zero orphans, and **committed controls excluded from the denominator and
named** — the predecessor's census found nine such rows out of 213 and the node
would have misread its own population without them.

> ### A PROOF OVER AN EMPTY POPULATION IS VACUOUS — CAMPAIGN TRAP 3
>
> If a disposition class comes back **empty**, say so and stop treating it as
> proven. `InlineNoCall` in particular must have a **real, named member** before
> any control over it means anything.

### `D1` — the representation

The planner mints an **opaque binding candidate** carrying the **exact worker
provenance and selector**. Its existence **authorizes environment installation
and does not assert a causal call.**

Lowering settles each candidate **exactly once**, from an event **only lowering
can observe**:

| disposition | settled when |
|---|---|
| `DirectCall` | at the verified direct producer / call seat |
| `ComposedCall` | only after the raw-worker call is emitted **and enters the existing finished-CLIF verification** |
| `InlineNoCall` | only after the **exact deferred bridge scope completes successfully** with that candidate still unconsumed |

A **static-worker binding carries the candidate authority.** Actual
source-machine consumption promotes it to `ComposedCall`; a **value-position
read still reaches the fail-closed `StaticWorkerBinding` guard**, so `d8e`
retains **binding count 1** and **refuses**.

> ### `InlineNoCall` IS NOT A THIRD DISCHARGE, AND THIS IS THE WHOLE DESIGN
>
> A third arm in the partition would let a program **with no call** satisfy a
> law that exists to say a call was **answered**. The candidate layer sits **in
> front of** the partition. **`InlineNoCall` is never called a discharge and
> never enters the equality.**
>
> If your implementation makes it easier to add an arm than to add a layer,
> that is the signal you are building the forbidden thing.

### `D2` — closeout, in this order

**First** require an **exact, disjoint disposition for every candidate.**
**Then** derive the call-obligation subset from `DirectCall ∪ ComposedCall` and
apply the existing law **unchanged**.

**The order is the mechanism, not a style preference.** Deriving the subset
first and checking dispositions afterwards would let an unresolved candidate
pass silently, which is exactly the failure the predecessor's `close` refuses.

### `D3` — the five mutations, each reddening independently

| # | mutation | must red |
|---|---|---|
| 1 | suppress binding installation | yes |
| 2 | mark inline **before** bridge completion | yes |
| 3 | mark inline **after** a composed call | yes |
| 4 | omit a final disposition | yes |
| 5 | present one candidate in **two** dispositions | yes |

**Independently** means each is proven on its own, not that the suite reds when
all five are applied. **Check whether each control is free before you write
it** — the campaign's standing trap is a control that asserts the absence of a
refusal the repair just deleted from production.

**Preserve the four-cell `d8e` table as the primary discriminator.** Both
classified variants keep **one** binding; index 1 may finish inline, **index 2
must still refuse in value position.**

## 3. Acceptance criteria

| AC | criterion | control |
|---|---|---|
| `AC-1` | **DISCHARGED** at `e93afb06` (PR #1659) — 637 candidates, one disposition each, zero orphans, controls named and excluded | `D0` record |
| `AC-2` | Every candidate has exactly one disposition; no unresolved, no double | closeout check, plus mutations 4 and 5 |
| `AC-3` | `InlineNoCall` never enters the call-obligation equality | read the derivation site; mutation 3 |
| `AC-4` | The law is unchanged: exact set equality, both-sets refusal intact, `composed` still fed only from `function_local.composed_discharges` | verbatim check at the three sites |
| `AC-5` | `d8e` keeps binding count **1** and still **refuses** in value position | the four-cell table, both variants |
| `AC-6` | Each of the five mutations reds **independently** | five proofs, from the committed tree |
| `AC-7` | `InlineNoCall` has a real named member. **The CLASS is not empty — `D0` measured 21 members.** What is empty is the **non-vacuous witness cell**, `binding-installed ∩ closeout-checked ∩ compile-OK`, so the witness must be **AUTHORED under `D1`** | `D1`, not `D0`; **vacuous otherwise** |
| `AC-8` | No `#[ignore]` added; `issues/` untouched; the five landed repairs and the predecessor's `D0`/`D1` intact | mechanical |
| `AC-9` | Workspace green **in CI** | CI, never a local `--workspace` run |

**`AC-5` is the one that can fail silently.** A split that makes `d8e` stop
refusing has not implemented the distinction — it has erased it.

> ### DERIVE WITNESSES, DO NOT PIN INDICES
>
> Learned today at cost on this exact file. `PX8-ERRID-ALLOC` reddened CI
> because a negative control's witness was a **literal out-of-range index** that
> silently came **into** range when a population grew; it was repaired by
> deriving the index from the inventory's own length.
>
> **Every control here asserts a property over a population that will grow.**
> Any control that pins a candidate count, or selects a witness by literal
> index, has the same defect already in it.

## 4. The hard stop, and it is not hypothetical

**Measure declaration/definition and ABI reachability for candidates settled
`InlineNoCall`.**

> **If permitting a binding-only candidate requires a post-lowering
> call-graph rebuild, or changes the planner traversal contract — STOP AND
> ROUTE.** Do not allow an uncalled executable unit, and do not absorb the
> rebuild.
>
> **`D0`'s census is the early instrument for this**, which is why it comes
> first. A traversal-contract problem shows up there as a population that does
> not partition, rather than as a surprise at review.

### WHAT `D0` FOUND — half measured, half open, and a second candidate stop

**The instrument did its job, and it fired.** Recorded at `e93afb06`
(PR #1659), with the Architect ruling requested at `evt_7hzmgfyedd70v`:

- **`px8j` is NON-SELECTED**, measured with a same-run probe-alive control. It
  sits outside the selected `FunctionizedUnits` artifact, so its
  `b2f_last_unit_emission() == (0, 0)` is a **non-selected-authority result**,
  not a blind instrument. **No further direct ABI probe is required.**
- **The selected-side reachability controls are `sar_d3`, `ccr_d3` and
  `coc_d3`.** Those are the members inside the selected artifact, and they are
  what any later reachability claim must be made against — **not** `px8j`.
- **The 210-of-637 result is NOT a second hard stop** — ruled
  `evt_40rf074xsj3y1`. Production has **one artifact-wide ledger**, opened in
  the selected `FunctionizedUnits` arm before `define_unit_bodies`, seeded from
  the full `continuation_calls()`, and closed only after every definition pass
  and the root adapter succeed. So `CLOSE_CHECKED = false` means **that compile
  never reached a successful functionized-artifact closure, or selected another
  authority** — not that a healthy candidate fell outside a successful
  closeout's authority.

**`D2`'s production quantifier, stated exactly.** Every **activated binding
candidate in one selected `FunctionizedUnits` artifact** must be settled once
before that artifact closes. **Plan-only rows, `Err` compilations, and plans
compiled under the non-selected `RecursiveDescent` authority are not
obligations.** The candidate layer is a sibling in front of the unchanged call
ledger and **shares its artifact lifetime**: it does not widen
`ContinuationClaimLedger`, add a per-owner close, or traverse failed or
non-selected compilations.

**The population DID partition — 637, one disposition each, zero orphans**, and
637 is retained as the **observational superpopulation**.

> ### THE §4 STOP IS **UNFIRED AT `D0`** — AND HERE IS ITS RE-ROUTE CONDITION
>
> **`UNFIRED AT D0; re-route only if `D1` changes unit population, declaration,
> definition, ABI projection, or traversal.`**
>
> `D0` owes **nothing further** on this axis. Do not re-run the ABI probe, and
> do not treat the stop as an open measurement — it is **unfired**, not
> **cleared**, and those are different claims: unfired means the condition was
> evaluated and did not hold, so it stays live as a **`D1` obligation** against
> the five named axes above.

## 5. Untouched

`ContinuationClaimLedger::close`, finished-CLIF direct **and** composed
verification, the both-sets refusal, the `composed` feed, the empty resume, and
**all five landed repairs** — until the split representation proves otherwise.

**Do not reopen** [[RT-SPECIALIZED-ACTIVE-RESUME]]'s accepted `D2`/`D3` or
[[RT-CONTINUATION-CALL-DISCHARGE]]'s `D0`/`D1`. **The exact-witness conclusion
"no call occurred" is unchanged and is load-bearing here** — it is why
`InlineNoCall` must exist at all.

## 6. Contention

**Re-run this check at kickoff; a contention statement written at framing time
describes a tree that no longer exists.** As of `main` `28626055`, Runtime holds
no branch, but **two other lanes are open** and concurrency rests on **measured
crate disjointness**:

| lane | surface |
|---|---|
| Kernel — `KERNEL-NESTED-IND` | kernel crates |
| Verify — `PX8-ERRID-SCOPE` | verify and host surfaces |
| **you** | `cranelift_backend` |

`planning/static_transition.rs` and `lowering/units.rs` are your primary
surfaces. **`PX8-ERRID-ALLOC` moved `static_transition.rs`, `lowering/core.rs`,
`lowering/mod.rs`, `semantic_ir.rs` and `core/tests/effects.rs`** — re-derive
every coordinate in those, and note `effects.rs` gained **+223/-5** that was
**explicitly not audited** by the Adversary.

## 7. If it does not close

**Route it, do not absorb it.** Seven walls on this chain have each been a
distinct authority and every one was resolved by routing. **An eighth is a
normal outcome, not a failure** — and the campaign's record is that the
expensive mistake has always been treating a new authority as a defect in the
previous repair.
