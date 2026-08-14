---
id: RT-CONSUMER-CARRY-CONTROL-DEBT
title: "five carries on the consumer-descent-carry merge: two new planner refusals are unreachable in production because the interned target is not an independent authority, two of four equality assertions are vacuous, the lag law is NOT uniform and its depth-1 boundary is asserted nowhere, the primary Source branch is unexercised and sits one level off from the fallback, and the D8a twin clones where the real descent advances"
status: merged
owner: runtime
size: S
gate: none
depends_on: [RT-CONTKEY-CONSUMER-DESCENT-CARRY]
blocks: [RT-CONTKEY-ROUTE-CLOSURE-PROBE]
github: null
origin: "Five non-blocking carries the Architect recorded in the resolved Decision dec_7yg4qzfngjwtj (APPROVED on exact b0f9c2ff, resolved_at 2026-08-14T17:54:48Z), none of which amended that exact-SHA approval. Steward-filed per COORDINATION §2 because a carry recorded only in an approval verdict and a PR body evaporates -- the third time that failure was caught the same day, after RT-CONTKEY-REFUSAL-PROFILE-SPLIT and LANG-WITNESS-DIAGNOSTIC-STRICTNESS."
---

> # AMENDED AFTER RELEASE, 2026-08-14. TWO ITEMS ADDED: `C6` AND `C7`.
>
> **The ring was kicked at `evt_7v1250teswa9h` against the five-carry version.**
> An Adversary hunt on the same merge (`evt_76f8grx4y7rtt`) landed afterwards,
> was verified against the tree, and adds two items **it explicitly ranked as
> belonging here rather than as their own node.**
>
> **Neither invalidates work already done.** `C6` reads onto `C3` and sharpens
> what the existing controls establish; `C7` is a comment clause. **`AC-1` is
> unchanged and still leads.**

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

## `C6` -- THE LAW IS A CLAIM OVER ALL `N`, AND EACH COMPILE OBSERVES ONE `N`

**Adversary hunt `evt_76f8grx4y7rtt` on `b0f9c2ff`, verified against the tree
before filing. It reads directly onto `C3` and belongs beside it.**

**The good half first, because it is the half the Steward asked to have
attacked and it survived.** The cross-compile equalities are real:
`depth_2.required` is `(16,5)` **from a different program's compile**, matching
`depth_1.unit_consumer` exactly, and the same 3→2. There is no shared
derivation within a run, and a wrong carry produces the target's own value --
which is exactly what the production mutation showed. **That construction is
not weaker than it reads.**

**The raw stream the control already prints, and which nobody had quoted.**
Format `(is_child_push, continuation_origin, result_root, required.body,
required.eliminator)`:

```
depth-1  unit=(16,5)   (false, 21, 31, 16, 5)   (true, 21, 29, 16, 5)
depth-2  unit=(26,21)  (false, 31, 41, 16, 5)   (true, 31, 39, 26, 21)
depth-3  unit=(36,31)  (false, 41, 51, 26, 21)  (true, 41, 49, 36, 31)
```

**Two observations per compile at every depth -- one producer-use, one
child-push. NOT one per level.** The doc claims *"`required(N)` is the existing
consuming occurrence established at level `N-1`"*, which quantifies over `N`.
**The stream contains one `N` per compile.** The depth-3 program's inner
boundary is never observed *in the depth-3 compile*; it is inferred from the
separate depth-2 compile.

**The control's own structure proves this is not a misreading.** `required` is
collected into a `BTreeSet` (`control.rs:5806`) and destructured as
`[required]` (`:5827`). **If a depth-3 compile emitted producer-use carries at
two levels with correct -- therefore different -- values, the set would hold
two and the destructure would fail.** ⇒ **The test passing is itself the
evidence that only one level is observed.**

⇒ **"Correct at every level" is established as "correct at the OUTERMOST level
of three separately-sized programs."** For this uniformly generated chain that
is a good proxy -- numbering advances by exactly 10 per level and each depth's
`unit.eliminator_origin` is the previous depth's `continuation_origin`. **For
any non-uniform nesting it would be silent.**

**`C3` says the depth-1 boundary is un-ASSERTED. This says the intermediate
boundaries are un-OBSERVED.** They are different gaps in the same law and a
delivery that closes one is not evidence about the other.

## `C7` -- a doc clause is false in the direction that invites the WRONG repair

`control.rs:5774`:

> *"The equalities are between independently produced planner records, not
> fixture origin literals; **source-origin renumbering therefore cannot require
> re-recording the test**."*

**The first clause is true and it is the control's real strength. The
conclusion does not follow.** The equalities hold because the depth-2 program's
*inner* level is numbered identically to the whole depth-1 program -- the
uniform `+10` offset visible in `C6`'s stream. **That is a property of
`px8j_scope_chain_observation_result`'s numbering, not of the test.**

⇒ **The test is robust to renumbering it does not contain, and NOT robust to
renumbering in the generator.** If wrapping ever numbered the wrapper's nodes
before the inner ones, depth-2's inner level would stop being `(16,5)` and
**the control would red with a correct carry.**

**That is the safe direction -- a false red, not a false green.** It is filed
anyway because of where it sends the next author: reading *"renumbering cannot
require re-recording"*, they look for a defect in the carry rather than in the
generator, and **the cheapest way out of that confusion is to hardcode the
values, which destroys the very property the sentence was written to praise.**

**One clause naming the generator's wrapper-invariance as the premise fixes
it.** This is a comment change; no control moves.

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

**`AC-6a` -- `C6` is dispositioned in writing, beside `C3`'s remedy.** Say, in
the doc that states the law, that it is observed **once per compile at the
outermost level** and that the intermediate boundaries are inferred from the
sibling compiles. **Extending the observation to every level is NOT required
here** -- it may not even be constructible without changing what the control
compiles. What is required is that the next reader cannot mistake three
outermost observations for a per-level verification.

**`AC-6b` -- `C7`'s clause is corrected.** The sentence must name the
generator's wrapper-invariance as its premise. **Do not resolve it by deleting
the claim** -- the independent-records property is real and worth stating; it
is the unsupported conclusion that goes.

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

## The carried `.map(Source)` item CLOSES as measured — 2026-08-14

**Both framings of this item were wrong, including the one this node carried
forward, and the read that settles it has now been taken.** Recorded from the
Adversary's hunt on `afdabc502` (`evt_pg42y72y6hrx`), re-checked against the
tree. **Do not re-file this shape.**

The item read *"the `.map(Source)` primary branch is unexercised and sits one
level off from the fallback"*, and was later restated as *"can the two inputs
ever disagree?"* with that read named as unrun.

Measured at `initial_continuation_discoveries`, the primary and the fallback
are **not alternatives for one value**:

| branch | fires when | supplies |
|---|---|---|
| `.map(Source)` | there **is** a parent (depth >= 2) | the parent's seeds |
| `.or_else(...)` | there is **no** parent (depth 1) | this level's seeds |

The walk seeds every root as `(origin, None, None)`, so at the outermost match
`consuming_occurrences` is `None`, the `.or_else` fires, and `required` is
`Source(candidates.clone())` — the same `candidates` that becomes the pushed key
seeds. At every deeper level `required` is the **parent's** while the key seeds
are **this level's**.

⇒ **Both branches are exercised, by different depths, and they cannot disagree
because they never co-occur.** The "one level off" observation is not a defect:
**that offset is the lag this node exists to carry.**

⇒ This also independently confirms the depth-1 shared-seed provenance that
`RT-CONTKEY-ROUTE-CLOSURE-PROBE`'s `D6` clause asserts — measured at both sites
rather than inferred from one.
