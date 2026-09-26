---
name: a-detector-keyed-coarser-than-the-rule-it-enforces-fixes-the-half-of-the-finding-that-differs-at-its-grain-and-misses-the-other
description: When a finding has two halves and the follow-on detector is keyed on a coarser grain than the rule (module vs module plus import kind), it goes red only on the half that differs at that grain. Mutate each half separately, and add a fresh violation of the finer grain with no test edits.
metadata:
  type: feedback
---

# A detector keyed coarser than its rule catches only one half

**Measured 2026-09-23 on CAT-CONFIGURATION-DECODER-IMPORT-EDGES.** Squash
`1ed04b835e0e812711d890005c2bf4008f339e05`. Filed LOW at `evt_6zsjx2fm99n47`
(thread `thr_6z4d2qhb2rdz4`). It follows
[[removing-an-ambient-fallback-can-close-the-only-working-route-when-the-intended-route-was-already-broken]].

## The shape

The original finding had two halves:

- **Derived:** the module was not imported at all.
- **Schema:** the module was imported, but only selectively, and Decoder used
  qualified names that the selection does not grant.

The rule, spec 33 §3.2, is keyed on **module plus import kind**. The frame
asked for a detector keyed on **module only** ("a module Decoder imports"). The
fix added the right imports, and the detector's falsifier (AC-1) named Derived
and went red. But the detector counts a selective import as declared, so:

- deleting the fix's plain Schema import stayed green;
- a new qualified reference into a selectively imported module stayed green.

Only an exact import-set list pinned the Schema line, and nothing in it records
why the line is needed.

## How to hunt it

1. Write down the rule's grain, meaning the tuple it quantifies over. Then write
   down the detector's grain. If the detector's is coarser, some violations
   collapse onto a compliant key.
2. Revert **each half** of the original finding separately. Adjust only the
   exact-list pins, not the predicate. A half that stays green is not covered
   by the detector.
3. Add a **fresh** violation at the finer grain, with no test edits. This is the
   future regression the detector is supposed to catch.
4. Run one positive control that must go red, so a green result means
   something. See
   [[a-negative-check-passes-for-any-reason-so-it-needs-a-positive-control]].

Attribute the cause correctly. Here the frame set the grain and the
implementer met it, and the doc comment was accurate. So this is a LOW gap for
the Steward to accept or tighten, not an implementer defect.
