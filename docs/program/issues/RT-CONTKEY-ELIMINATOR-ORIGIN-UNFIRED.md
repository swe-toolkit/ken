---
id: RT-CONTKEY-ELIMINATOR-ORIGIN-UNFIRED
title: "consuming_occurrence carries two fields and only one of them is checked -- eliminator_origin is copied into the re-derivation before the comparison, so AC-1's assert is x == x on that field, and AC-2's mutation perturbs only the field that was already independent, leaving step 1 never fired"
status: merged
owner: runtime
size: XS
gate: none
depends_on: []
blocks: []
github: null
origin: "Adversary hunt evt_7b75nbgqbw04z on a998d3f6, triaged CONFIRMED by the Steward. The hunt RAN the control and read its output rather than reading the assertion, which is what exposed the copied field. Filed rather than folded into D2k-1c: that node is a route repair mid-turn and this is a control defect on the relation it consumes. Steward-filed per COORDINATION §2."
---

## What this is

**A landed control proves less than its AC claims, and the gap is on the exact
field that has no red between here and `D2k-1c`.** Nothing measured is wrong;
what is missing is any evidence that it is right.

**This is not a defect in [[RT-CONTKEY-CONSUMING-OCCURRENCE]]'s mechanism** and
it does not reopen that merge. The relation is carried correctly as far as
anyone can tell. **The controls around it are what fall short.**

## `D1` — fire step 1, by perturbing `eliminator_origin`

`rederive_consuming_occurrence` constructs every candidate with
`body_origin` genuinely re-derived — scan the eliminator's alternatives, match
each case-constructor identity against the key-selected body's
`continuation_result_constructor_identities`, require exactly one — and
`eliminator_origin: claimed.eliminator_origin`, **copied from the input before
the comparison**. Nothing reads `claimed.body_origin`.

⇒ `assert_eq!(direct, Some(carried))` is a real check on `body_origin` and
**`x == x` on `eliminator_origin`**. The control's own output shows it: both
rows print `eliminator_origin: 5` on both sides, and the `5` is the copy.

**And no mutation perturbs it.** The `#[cfg(test)]` seed hook rewrites
`body_origin` to the scrutinee; `eliminator_origin: origin` sits **outside** the
mutation. So the verbatim refusal that discharged `AC-2` is **the scan rejecting
a wrong body — step 1 has never been fired.**

**The hook already exists and the shape is next door.** Add a second arm to
`MUTATE_CONTINUATION_CONSUMING_OCCURRENCE_SEED`, or make it two-valued in the
style `Px8jSelectedScopePlacement` already uses, perturbing `eliminator_origin`
to **another match origin**. Assert the same sentence. That fires
`forward_match_scrutinee`'s guard rather than the scan.

## `D2` — state the structural premise the guard rests on, or fire it

Step 1's protection is
`forward_match_scrutinee(plan, claimed.eliminator_origin) != key.continuation_origin
=> Ok(None)`. That asks *"is the continuation origin this match's position-0
child?"*

**Whether that is as strong as a derivation rests on an unstated premise: that
at most one match has a given occurrence as its position-0 child.** If two did,
a wrong `eliminator_origin` naming the other would pass the guard.

**Write the premise down.** If it is cheap to establish at this base, establish
it and say how. **If it is not, record it as an unfired structural assumption
rather than asserting it** — that is a legitimate outcome and the honest one.
This node does not require proving it.

## `D3` — one sentence beside `AC-3`'s count

`AC-3` reported a population of two. **On the axes that matter it is one.** Both
rows carry `eliminator_origin: StaticOriginId(5)` and
`consumer_owner: PredeclaredFunctionId(0)`; only `body_origin` differs (`16` vs
`12`). They are **one eliminator with two bodies**, not two samples of the
relation — so a control passing for the wrong reason on `eliminator_origin`
passes identically on both rows.

**Record that in [[RT-CONTKEY-CONSUMING-OCCURRENCE]], where the count lives**,
so the number is not read as two independent samples by the next person sizing a
control against it.

## Acceptance criteria

**`AC-1` — the new mutation reds, and you report the verbatim text.** A mutation
that passes means it is not reaching step 1 and the arm is misplaced — **that is
the finding**, not a reason to weaken the assertion.

**`AC-2` — the existing `body_origin` mutation still reds, unchanged.** Both
arms fire independently. **If making the second arm work required changing the
first, say so** — that would mean they share a path and the two-arm split is not
what it appears.

**`AC-3` — the two arms are distinguishable in the failure output.** A reviewer
seeing a red must be able to tell which field was perturbed. Two mutations
producing one indistinguishable message is a single control wearing two names.

**`AC-4` — no production behaviour change.** `#[cfg(test)]` and test paths only.
The relation, the key, the seeding and the threading are all untouched. If the
premise in `D2` turns out to be false, **stop and report** — a false premise
there is a mechanism question for the Architect, not a repair to absorb here.

**`AC-5` — no-regression, in CI.** `COORDINATION §12` — the venue is CI, never a
local `--workspace` run. Build targeted, `-p ken-runtime`.

## Sizing

**`XS`.** One extra mutation arm on an existing hook, one assertion, and two
recorded sentences. **If `D2`'s premise turns into an investigation, that is the
report and it stops there** — this node's product is a fired control, not a
proof about the occurrence graph.

**Sequencing: after `D2k-1c`, or as fill-in if that stops early.** It shares
`static_transition.rs` with the in-flight increment, so it does not run
concurrently with it.

## Not this node

- **Not a reopening of [[RT-CONTKEY-CONSUMING-OCCURRENCE]].** That merge stands;
  its mechanism is not in question and no value is known to be wrong.
- **Not a proof of the occurrence-graph premise.** See `D2` and Sizing.
- **Not a change to the relation, the key, or the threading.**
- **Not `D2k-1c`'s route repair**, and not to be folded into that candidate.
