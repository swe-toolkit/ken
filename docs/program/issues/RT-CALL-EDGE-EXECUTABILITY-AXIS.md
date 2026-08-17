---
id: RT-CALL-EDGE-EXECUTABILITY-AXIS
title: "executable_call_edges probes a body-axis set with an entry-axis key, so a template-only callee whose axes differ survives the filter and fails later as a forward-declaration error"
status: merged
owner: runtime
size: S
gate: none
depends_on: []
blocks: [RT-BACKEND-SPLIT-CENSUS]
github: null
origin: Adversary report evt_1gtad2keqngcq (2026-08-09) on merged 1f706520, the RT-BODY-OCCURRENCE-PROVENANCE accepted partial at exact 876450ab. Steward-triaged as a confirmed latent defect and filed per COORDINATION §2. NOT folded into RT-CANDIDATE-LEDGER-RESIDUALS: that node's leading claim is "neither is a defect", and this one is.
---

> # SEQUENCED AHEAD OF [[RT-BACKEND-SPLIT-CENSUS]] — operator ruling, 2026-08-16
>
> **This node lands before the backend-split census**, whose `depends_on` now
> names it. The edge lives there rather than only here, because
> `scripts/gen-progress.sh` reads `depends_on` and nothing else — a `blocks`
> edge alone would be invisible to every generated view.
>
> **Why: it edits `planning/static_transition.rs`, inside the split's own scope**
> (`crates/ken-runtime/src/cranelift_backend/` plus `boundary_value_clif.rs`).
> A split cannot run concurrently with semantic work on the files it partitions
> (campaign §4 ground 3), so this is pure ordering and landing first costs one
> rebase instead of a re-home followed by a fix.
>
> **Nothing about this node's own content changes.** It was `ready` before the
> ruling and is `ready` after; only its position moved. **It is not released
> yet** — lane 1 is on [[RT-DESCENT-RETIRE]], which hard-stopped on `D1` with
> two surviving classes still selecting the lane.

> # 2026-08-17 — COMPLETE. `D1` MERGED AT `e5286ea06` (PR #2533).
>
> **Both deliverables are in.** `D0` landed 2026-08-10; `D1a`/`D1b` landed
> today at exact `60065fc13`, squashed to `e5286ea0665d4b81c91427e42aab175dfd23cdbb`
> — one test-only path, `+19/-8`. Architect resolved `dec_59rm7x793vy9w`; QA at
> `evt_3v2423rs1ec5t`.
>
> **`AC-2` is discharged on its BOUNDED-NEGATIVE arm, which the frame
> authorized.** No unit is known to be both split-axis and template-only. What
> stands in its place is a **sentinel that reds when that population appears**:
> `divergent.is_empty()`, whose failure text announces the witness. It is green,
> so **the witness does not exist** — do not read that message as a report of
> one. No same-axis fixture was substituted, which the frame forbade.
>
> **`D1b` converged the detector onto production's join.** The sentinel used a
> linear first-match `units.iter().find(..)`; production builds a last-wins
> `BTreeMap`. The test now uses the identical construction. **`D1a`** corrected
> a diagnostic that claimed *"the defect's exact failure direction"* from a
> predicate that is one-sided by design.
>
> **OWED, non-blocking (Architect `evt_6csb4936b510g`):** production retains a
> descriptor-less edge (`None => true`) while the new test **reds** on one.
> Both are correct — production must not suppress a planner contradiction, the
> test must not scan a partial population. **One line at the assertion saying
> the divergence is deliberate**, or a later reader "aligns" them in good faith
> and silently restores the hollow-population hazard.
>
> ### OWED, SECOND HALF — the assertion's MESSAGE names the wrong cause.
>
> **Adversary `evt_2rbev0dq0nfjd`. Same two lines, different failure, and the
> two remedies are one edit.**
>
> `control.rs:31582` reads *"the disagreement scan silently skipped {} call
> edge(s) with no callee descriptor"* — **it blames the scan.** Production at
> the same join (`static_transition.rs:16108`) says the opposite: *"A callee
> with no descriptor is a **planner contradiction** this filter does not own."*
>
> ⇒ **A descriptor-less edge reaching `edges` is a PLAN defect that production
> deliberately propagates, not a scan defect.** So **if this assertion ever
> reds, the cause is the plan and the message sends the investigation to the
> scan.**
>
> **The repo already names this exact failure shape** — *"a true sentence about
> the wrong thing, naming a cause that is not the cause"*
> ([[RT-SPECIALIZED-MATCH-ATTRIBUTION]], and twice in the production file at
> `core.rs:7568` and `:16603`).
>
> **The remedy, borrowing production's own vocabulary:** *"{} call edge(s) have
> no callee descriptor — a planner contradiction production retains for
> downstream rejection; the scan covered the rest."* Keeps the coverage count,
> stops the misdirection, and the deliberate-divergence comment above serves
> both notes at once.
>
> **Correction to the finding, recorded because the citation matters more than
> the conclusion here:** it attributes the principle to a `D3` CARRIED arm
> *"three thousand lines away in the same file."* **That sentence is not in
> `control.rs` at all** — verified, including for wrapping. It is in
> [[RT-SPECIALIZED-MATCH-ATTRIBUTION]] and in `core.rs`. **The substance is
> unaffected and in fact stronger:** the principle sits twice in the
> *production* file this test stands in for, not in the test file.

> ### DO NOT BATCH EITHER NOTE INTO [[RT-SRCMACHINE-DISPATCH-REACHABILITY-CONTROL]].
>
> That node is active in this same file and its **`AC-5` requires blob identity
> on any file it does not intend to change.** Folding an unrelated two-line
> edit into it would break its own acceptance criterion. These notes wait for a
> node that intends to touch `control.rs`, or for a dispatch of their own.

> ### THE SHA BELOW IS A PRE-SQUASH HEAD. The landed `D0` is `b7bba72dd`.
>
> **`35265ca5` is NOT an ancestor of `main`** — the publisher squashes, so the
> reviewed branch head never lands. Checked 2026-08-17: `git merge-base
> --is-ancestor 35265ca5 origin/main` fails, while `b7bba72dd` passes and is the
> commit that introduced production's `body_axis` join.
>
> ⇒ **Cite the landed squash, or `merge-base...tip`. A bare reviewed-tip SHA
> resolves as an object and shows a reader the wrong history.**

> # SUPERSEDED — `D1` was owed here; it has since landed. Kept for its reasoning.
>
> **`D0` merged at `35265ca5` (PR #1797). This node returns to `ready` because
> `D1` is owed, not because anything about `D0` is in doubt.**
>
> **Confirmed Adversary finding `evt_51ekvw0fzthy6` on the merged `D0`,
> measured at `b7bba72d`, triaged by the Steward.** Two defects, both **detector
> fidelity between the sentinel and the filter it stands in for**, both
> over-claim in direction. **Neither touches production logic and neither is an
> `AC-2` claim.** I re-derived both against `origin/main` before accepting them.
>
> **The non-vacuity clause was independently checked and HOLDS.** Both counters
> are computed in the same loop iteration over the same `edge`, so a nonzero
> `superseded_callees` guarantees an edge with `template_only.contains(&body)`
> whose `divergent` is empty **only** because `template_only.contains(&entry)`.
> It is a genuine discriminator, not a reachability proxy. Do not touch it.
>
> ### `D1a` — the red message claims a symmetry the predicate does not have
>
> The predicate is `template_only.contains(body) && !template_only.contains(entry)`
> — **one of the two ways the filters can disagree**. The other polarity
> (`entry ∈ template_only && body ∉`) is a same-callee disagreement and is not
> detected.
>
> ⛔ **The one-sidedness is CORRECT and must not change.** Executability is a
> function of the body alone (`static_transition.rs:11355`), so retaining there
> is the right outcome and the benign polarity is deliberately out of scope.
>
> The defect is that the doc carries **both** the qualified sentence (*"that
> conjunction is the defect's own failure direction"*) and an unqualified one,
> and **the unqualified one is what fires**: *"The two filters disagree on this
> callee."* A future engineer reading that red goes looking for a symmetric
> population. Two words fix it — *disagree in the defect's direction*.
>
> ### `D1b` — the sentinel re-derives production's join with a different discipline
>
> Verified against `static_transition.rs:11361-11375`:
>
> | | production | sentinel |
> |---|---|---|
> | descriptor-less callee | `None => true` — **RETAIN**, handed downstream | `continue` — **dropped from the scan** |
> | duplicate `PredeclaredFunctionId` | `.collect()` into `BTreeMap` — **last wins** | `.find(...)` — **first wins** |
>
> **(a) is the one that matters.** The set production deliberately treats
> specially — and whose comment this very cut corrected — is the one set the
> detector **cannot see**. `joined > 0` does not bound it: that proves the scan
> was non-empty, never that it covered the edges. One line closes it: assert
> `joined == edges.len()`, or report the skipped count so an exclusion is
> visible rather than silent.
>
> **(b) is a fidelity defect, not a live one, and the bound is part of the
> claim.** `emittable_units()` maps 1:1 over `self.abi.descriptors` and
> **nothing establishes `descriptor.function` is unique**; no duplicate was
> constructed and there is no evidence the state is reachable. ⇒ **Build the
> sentinel's lookup with the same `BTreeMap` expression production uses**, which
> makes the question moot instead of answering it.
>
> ### Sequencing, and it is the whole point
>
> ⛔ **This does NOT go in front of the RecursiveDescent campaign.** Runtime is
> kicked onto `RT-MATCH-RECURSOR-CONSUMERS` `D9`/`D10` under the operator's
> 2026-08-10 priority ruling and must not pick this up. `ready` here means
> framed and shovel-ready for a later lane, not frontier.
>
> **Why it is recorded rather than deferred silently:** the Adversary noted this
> is the **same over-claim shape** as `evt_2xryrnxz7g0mb`, appearing inside a
> recut authored specifically to replace an over-claiming proxy, by a ring that
> had just been told about it. That is not carelessness — the precise sentence
> and the loose summary get written in different passes, and **only the precise
> one gets proofread as a claim.** The loose summary is what fires.

> # LATENT, FAIL-CLOSED, AND CHEAP. DO NOT WIDEN IT.
>
> No witness exists today. The failure direction is a **spurious refusal**, not
> a fabricated `FuncId` and not unsoundness. The fix is about one line.
>
> **Do not repair this inside `RT-SPECIALIZED-MATCH-ATTRIBUTION`**, which is
> measurement-only and forbids production changes.

> # EVERYTHING BELOW IS THE ORIGINAL FRAME AND IT IS HISTORICAL. `D0` FIXED IT.
>
> **Read this section as the defect that WAS, not one that is.** Verified at
> `origin/main`, 2026-08-17:
>
> - **The code shown below is gone.** `template_only.contains(&edge.callee_origin())`
>   returns zero hits at every base checked. Production's
>   `executable_call_edges` builds a `body_axis` map and probes
>   `body_axis.get(&edge.callee())` — that is `D0` option 1, landed at
>   `b7bba72dd`.
> - **Every `11354` coordinate below is stale.** The site is now **`16098`**,
>   and `template_only.contains(&body)` — the correct body-axis probe — sits at
>   `15480`.
> - **`D0`'s "decide the axis, one line either way" is DECIDED**, and the
>   `AC-1` phrasing keyed on `11354` is discharged by that landing.
>
> **This is why the section is banded rather than deleted:** the reasoning about
> *why* the two axes meet at exactly one site, and why the error was natural
> rather than careless, is the durable part and still reads true.
>
> ⚠ **A frame describes when it was written, not what the tree contains.** The
> Steward's 2026-08-17 kickoff quoted this section and handed `D0` to the ring
> as an open choice, six days after it had merged. The lede above said so; the
> body did not. **Read the producer, not the prose.**

## The defect

`crates/ken-runtime/src/cranelift_backend/planning/static_transition.rs:11354`

```rust
let template_only = self.template_only_worker_bodies()?;   // 11350
...
.filter(|edge| !template_only.contains(&edge.callee_origin()))   // 11354
```

**The set is body-axis; the key is entry-axis.** `template_only_worker_bodies`
builds its candidates at `11248-11252` from `context.worker_body_origin()`, and
the sibling method's comment at `11334-11336` says so: *"`template_only` is a
set of worker BODY origins ... so the membership test names the body axis."*
`edge.callee_origin()` is the scheduling entry — `entry_origin()`'s own doc at
`9633`, `resolve_call_edges` at `units.rs:671-673`, and the `AC-4` control
`call_identity_stays_on_the_entry_axis_after_the_body_axis_moved` (`16796`) all
say so.

**The invariant it violates is stated in the same file by the same candidate**,
at `11861`: *"executability is a function of the body alone."*

All three production membership probes of that set:

| site | probe | axis | verdict |
|---|---|---|---|
| `11337` `executable_units` | `unit.body_occurrence()` | body | correct — fixed by `876450ab` |
| `11354` `executable_call_edges` | `edge.callee_origin()` | entry | **mismatch** |
| `11861` composed-selector refusal | `answer.body_origin` | body | correct |

The outlier is the sibling of the one the candidate fixed, reads the same set,
and sits seventeen lines away.

## Why it is a natural error, not a careless one

Call identity legitimately lives on the entry axis, and `EmittableCallEdge`
carries only that axis (`9557`). The site asks an **executability** question
with the only origin it has to hand. The two invariants are genuinely
different and this is the one place they meet. Do not write the repair as
though someone was sloppy.

## Failure direction, bounded

A template-only callee whose two axes differ does not match the set, so the
edge survives the filter, reaches `units.rs:679` `bundle.function(edge.callee())`,
and gets `None` — because `executable_units` correctly excluded that unit from
both the declaration and definition passes. The result is a hard refusal
reading *"a call edge names a unit that was never forward-declared"*: a
spurious compile failure blaming forward-declaration rather than the retarget.
`UnitBundle::function`'s `Option` catches it, which is what it is for.

## What is NOT established

**No witness where both conjuncts hold.** Split-axis units exist now — `16831`
asserts `fixtures_with_split_axes > 0` and `computational-nested` is one
(`origin_of(n18) != n5`). Template-only-ness needs the `D5a` full-retarget
population. **Nobody has constructed a unit that is both**, and the Adversary
said plainly that it did not try.

So this is latent by exactly the argument `core.rs:16713-16719` already
rejects as authority: *"agreement on the current population is not authority."*
That is the grounds for fixing it, and also the reason not to claim a
regression.

## `D0` — decide the axis, one line either way

1. Either map callee to unit and read `body_occurrence()`, mirroring `11337`;
2. **or**, if the axes provably coincide for template-only units specifically,
   say so in a comment at `11354` and name why.

Either discharges this. The point is to convert an accident into a decision.
**The mechanism is the owner's call** — the Adversary explicitly declined to
choose it, and so do I.

## Acceptance

- `AC-1` The probe at `11354` and the set it reads name the **same axis**, or a
  comment at that site states why they need not and grounds it.
- `AC-2` If option 1: a control exercising a template-only callee **whose two
  axes differ**. If no such fixture can be constructed today, say so in the
  handoff and state what population would be needed — **do not** substitute a
  same-axis fixture and report the AC discharged.
- `AC-3` No control keyed on the source text of the accessor. The only existing
  control touching `callee_origin` is the source-text oracle at
  `control.rs:4080`, which pins the accessor's **declaration string**, not the
  axis, and therefore pins nothing here.

## Forbidden

- Widening beyond the two options in `D0`.
- Touching `876450ab`'s seven paths for any other reason.
- Reporting `AC-2` discharged on a fixture whose axes coincide. That is the
  green-vs-green-adjacent shape: the control would pass for a reason unrelated
  to the property.
