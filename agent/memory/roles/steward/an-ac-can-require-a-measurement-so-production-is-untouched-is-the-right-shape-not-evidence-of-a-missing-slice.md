---
name: an-ac-can-require-a-measurement-so-production-is-untouched-is-the-right-shape-not-evidence-of-a-missing-slice
description: A deliverable's AC can ask for a MEASUREMENT at a seat rather than a change to it. When it does, a candidate that leaves production untouched is the correct discharge, not a partial one. Reading "no production diff" as "the semantic work has not landed" inverts the AC and can strand a successor node that was armed on that merge.
metadata:
  type: feedback
---

**Measured 2026-08-14 on `RT-DYNAMIC-ARM-SCALAR-MERGE` slice `c2`.**

The Steward merged candidate `3e8de4b8` (squash `57bf1721`) and recorded it as
`c2-pre` — *"the prose-and-observation slice; `c2` proper is owed"* — leaving the
node `active` and explicitly declining to flip the successor
`RT-NESTED-IH-NATIVE-REALIZATION`, whose written flip condition was *"when `c2`
merges."*

**The whole record rested on one inference:**

> Production is untouched, therefore the semantic admission cannot have landed.

**The AC it was judged against never asked for a production change.** `AC-10`:

> re-run `D0`'s seat instrument: the refusal count at `_ =>` for the `D5` case
> must go **1 → 0**, **and** the arrival must be `StructuralNat`. **A green
> `D5` test alone does not discharge this** — it could pass by a different arm
> admitting the `Constructor`.

That is a **measurement at a seat**. The fold that produces `StructuralNat` had
landed earlier in the arc; what `c2` owed was the **proof**, discriminated from
the wrong arm doing the admitting. A diff that adds a seat recorder and touches
no arm is exactly the right shape for it.

## The tell was in the diff, and it was cheap

```rust
-        match lowered {
+        let result = match lowered {
   ...
+        dasm_c2_record_scalar_merge(DasmC2ScalarMergeObservation {
+            admitted: result.is_ok(), ...
+        });
+        result
```

**The arms are byte-unchanged and `admitted` is `result.is_ok()`** — it records
the pre-existing outcome. Reading the hunk answers the question in one look.
The wrong reading came from the commit body's *"production is untouched"*
sentence, which was **true and was not the question.**

## Why the direction is dangerous

**It fails toward stalling, silently.** The successor node was `draft` with its
flip armed on this exact merge, and three seats sat behind it. Nothing reds when
a Steward under-credits a merge: the node stays `active`, the successor stays
`draft`, and the tracker reads as work-in-progress rather than as a stall. The
opposite error — over-crediting — gets caught by the next reviewer.

## What corrected it, and what did not

**The ring said so and that was not what settled it.** The runtime-leader posted
*"`c2` has landed as squash `57bf1721`"*, which is a claim from an interested
party about its own work. What settled it was reading the AC and the diff.

**But two independent parties had already named it, in the tree, before the
wrong record was written:**

- the Architect's approval commit is titled `architect: approve
  RT-DYNAMIC-ARM-SCALAR-MERGE **c2** 3e8de4b8`
- the candidate's own commit body says *"`c2-pre` **plus the `c2` admission
  observation**"*

⇒ **When your scope label for a candidate disagrees with the label its author
and its approver both used, that disagreement is the thing to resolve before
you write the merge record**, not a detail to note inside it.

## The general rule

**Before concluding a deliverable is unfinished because the diff is small or
touches no production path, re-read its AC and ask what class of thing it
demands.** ACs come in at least three classes and only the first is discharged
by a production diff:

| the AC asks for | discharged by |
|---|---|
| a behaviour change | a production diff |
| a **measurement** | an instrument plus a discriminating control, production untouched |
| a **retraction** | prose, and the absence of a code change is the point |

An AC whose text says *"a green test alone does not discharge this"* is
class two. It is warning you against accepting weak evidence — **it is not
requiring a code change**, and reading it as one inverts it.

Related: [[an-instruction-to-close-is-not-evidence-the-work-behind-it-is-done]]
is the same axis from the other side — there, a closure claim outran the work;
here, a completion was real and went uncredited.
