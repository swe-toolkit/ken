---
id: RT-CONSUMER-CARRY-CONTROL-DEBT
title: "five carries on the consumer-descent-carry merge: two new planner refusals are unreachable in production because the interned target is not an independent authority, two of four equality assertions are vacuous, the lag law is NOT uniform and its depth-1 boundary is asserted nowhere, the primary Source branch is unexercised and sits one level off from the fallback, and the D8a twin clones where the real descent advances"
status: ready
owner: runtime
size: S
gate: none
depends_on: [RT-CONTKEY-CONSUMER-DESCENT-CARRY]
blocks: []
github: null
origin: "Five non-blocking carries the Architect recorded in the resolved Decision dec_7yg4qzfngjwtj (APPROVED on exact b0f9c2ff, resolved_at 2026-08-14T17:54:48Z), none of which amended that exact-SHA approval. Steward-filed per COORDINATION §2 because a carry recorded only in an approval verdict and a PR body evaporates -- the third time that failure was caught the same day, after RT-CONTKEY-REFUSAL-PROFILE-SPLIT and LANG-WITNESS-DIAGNOSTIC-STRICTNESS."
---

## What this is

**The merge is correct and is not reopened.** The Architect verified the law
independently: `required(N)` = the consuming occurrence established at `N-1`,
pinned by cross-compile equalities between independently produced planner
records rather than fixture literals, with the production mutation reddening
depth 2 and QA reproducing it. **Route (c) with the one-level lag correction,
built as ruled.**

**What this node collects is control debt and one stated-law gap.** Every item
below is the Architect's, transcribed in substance.

## `C3` -- THE LAG IS NOT UNIFORM, AND ITS BOUNDARY IS ASSERTED NOWHERE

**This is the one that can produce a wrong successor, so it leads.**

`depth_1.required` comes from the new `or_else` fallback -- **this match's own
candidates** -- not the `.map(Source)` primary branch. From two independent raw
reports, `depth_1.required == (16,5)` and `depth_1.unit_consumer == (16,5)`.

⇒ **At the outermost source level, `required` COINCIDES with the same level's
consumer. It lags only from depth 2 on.**

The test destructures `depth_1.required` for uniqueness but **asserts nothing
about its value**, and the field doc says *"established one level outside the
discovery"* **without noting the boundary**. ⇒ **A successor applying a uniform
lag law is wrong at exactly that one level**, and nothing reds.

**Remedy:** `assert_eq!(depth_1.required, depth_1.unit_consumer)`, so the
boundary convention is a **stated law** rather than an incidental value. Correct
the field doc to say where the lag begins.

## `C1` -- the "already-interned target" is not an independent authority

`intern_specialization` is **full-key exact** (`interned.get(&key)`, plus its own
refusal *"interned continuation identity is not full-key exact"*), so
`target_unit.key == key` **by construction**.

⇒ `target_unit.key.consuming_occurrence` is a **restatement** of
`consuming_occurrence_from_seed(...)` computed ~30 lines earlier, and
`target_unit.key.worker != worker` is **`x == x`** on the same `worker` binding
used to build the key. **Both new `planner_error`s** -- *"a descent target was
not installed before its child"* and *"a descent target names a different worker
than the child push"* -- **can only fire under the existing `#[cfg(test)]`
`CONTINUATION_INTERN_MUTATION` relaxation.**

**Keep them as invariant pins if wanted, but record them AS pins.** The
handback, the QA text and the Decision text all describe **active verification
against a second authority**, and a successor author will inherit that belief.

## `C2` -- two of the four equality assertions are vacuous

`depth_N.advanced == depth_N.unit_consumer` **cannot fail** once
`units.len() == 1` is asserted, because `advanced` is read out of that same
single unit. **The law is carried by the other two.** ⇒ **Do not count four
controls where there are two**, and do not let a later reader size a change
against a control population that is half what it appears.

## `C4` -- the primary `Source` branch is unexercised, and it is one level off

The `.map(Source)` branch fires only for **nested SOURCE matches**, which no
fixture here has. It also sits at a **different level** from the fallback:
inherited seeds are **two levels out** from the pushed discovery, this match's
candidates are **one level out**. Its refusal *"one required-consumer source
relation names two outer eliminators"* **has no control.**

Note also that `required_consuming_occurrence_for_alternative` is entirely
`#[cfg(test)]`, so **the `Source` variant is constructed in production and
resolved only in tests.**

## `C5` -- the `D8a` twin clones where the real descent advances

The duplicate-descent twin clones `discovery.required_consuming_occurrence`
instead of advancing to the target's, so **it diverges from the descent it
mirrors.** Test-only scaffolding; flagged, not urgent.

## THE SCOPE STATEMENT THAT MUST TRAVEL WITH THIS NODE

> **`required_consuming_occurrence` is PRODUCTION-WRITTEN and TEST-ONLY-READ.**
> The Architect recorded this plainly *"so nobody later over-reads it"*: that is
> **the bounded discovery-only increment he authorized, not a defect** -- but
> **the carry has not been validated by any production consumer, and the
> successor that wires one must not treat this node as having done so.**
>
> **Later route closure or refusal remains out of scope**, here and in the
> predecessor. That was true when the predecessor was framed with no closure AC,
> and it is still true now that the carry exists.

## Acceptance criteria

**`AC-1` -- `C3`'s boundary is a stated law.** The depth-1 equality is asserted
and the field doc says where the lag begins. **This is the one item that
prevents a wrong successor**, and a delivery without it has not addressed the
node.

**`AC-2` -- `C1`'s two refusals are either removed or re-documented as pins**,
in terms that say they cannot fire in production. **A comment is sufficient
here** -- the point is that the next reader does not inherit "verified against a
second authority".

**`AC-3` -- `C2`'s vacuous assertions are removed, or annotated as vacuous.**
Either is fine; leaving them uncounted is not.

**`AC-4` -- `C4` and `C5` are dispositioned in writing**, which may be "left as
is, for this reason". They are the two least urgent and an explicit deferral
closes them.

**`AC-5` -- the merged law still holds and its controls stay green** on the same
derivation. This node changes controls and documentation, **never the carry**.

**`AC-6` -- no-regression, in CI.** `COORDINATION §12`; build and test targeted,
`-p ken-runtime`.

## Sizing

**`S`.** One assertion, two comment corrections, two annotations and two written
dispositions. **If `C1`'s refusals turn out to be reachable after all, stop and
report** -- that would mean `intern_specialization` is not full-key exact, which
is a mechanism finding well outside this node.

## Not this node

- **Not a reopening of [[RT-CONTKEY-CONSUMER-DESCENT-CARRY]].** Approved on the
  exact SHA, law verified independently; no value it produces is known wrong.
- **Not wiring a production consumer of the carry**, and **not the route
  question.** Both are the next increment and neither is authorized here.
- **Not [[RT-CONTKEY-REFUSAL-PROFILE-SPLIT]].** That node owns the unnamed-cause
  `Option` returns in this file. **Sequence the two; they share
  `static_transition.rs` and must not run concurrently.**
