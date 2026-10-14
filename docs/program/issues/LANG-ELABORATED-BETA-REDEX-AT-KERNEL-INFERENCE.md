---
id: LANG-ELABORATED-BETA-REDEX-AT-KERNEL-INFERENCE
title: "An elaborator-built term reaches a kernel inference position with a λ-headed redex, so the kernel refuses with \"cannot infer an introduction form\". Linear-algebra left_head fails this way when map's argument is a λ, and checks when the same function is a named helper. Find every producer and close them by one rule"
status: merged
owner: language
size: M
tier: T1
gate: architect
depends_on: [LANG-NESTED-INDEX-MATCH-PROPOSITION-GOAL]
blocks: []
github: null
origin: "Architect D0 evt_3yey05qyvbds0 in LANG-NESTED-INDEX-MATCH-PROPOSITION-GOAL: left_head is a different defect from the goal-shape predicate. Steward fork (i) evt_jqa7eq0hnx8w. Steward-filed per COORDINATION section 2."
---

# No λ-headed redex reaches kernel inference

## Objective

A term the elaborator builds and hands to kernel inference is never headed
by an applied λ. A declaration that checks with a named helper also checks
when the same function is written as a λ.

## Settled inputs (Architect `evt_3yey05qyvbds0`, at `cc2d62abe`)

- **The row.** `left_head` in `LeftHeadCombined.ken` (spec-author
  `a9cb232a`) fails: the earliest error is `Internal("coherent-frame
  convoy found an ill-typed ambient method: ... cannot infer an
  introduction form ...")`.
- **The redex.** The embedded planner's ambient check fails on `map`'s
  VCons method. It contains `(λcolumn. dot n row column) x`, created when
  `whnf` delta-unfolds `row_product_columns` and then `map`, and
  substitutes the λ argument into head position.
- **Control: rc=0 on the base.** Respelled with `fn dot_with (n) (row)
  (column) = dot n row column` and `map (Vec Nat n) Nat p (dot_with n row)
  columns`, the whole of `LeftHeadNamed.ken` checks, `left_head` included.
- **More than one producer.** A β-pass over the planner's expanded goal
  removes the Internal, logging three reductions, but the declaration
  still fails the final kernel check at the `left_head` span
  (17904..18451), with the same message. Skipping the ambient check fails
  the same way. So a later elaborator output also carries the redex.
- The goal is already `Eq`. The predecessor's predicate does not touch
  this row.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

D0 first. Enumerate every elaborator output that reaches a kernel
inference position from a `whnf`-unfolded term, and measure which of them
carry the redex for `left_head`. The Architect rules one rule that covers
the population, for example normalizing head redexes where the unfolded
term is built, or checking instead of inferring at those positions. Then
the repair.

## Acceptance

- **AC-1.** `left_head` checks as written in `LeftHeadCombined.ken`, with
  no Axiom and no trust change.
- **AC-2 (control).** `LeftHeadNamed.ken` still checks. The predecessor's
  AC-1 to AC-3 rows and the 74-file catalog census are unchanged, per file.
- **AC-3 (mutation, QA).** Removing the ruled rule at each producer the
  D0 names reddens AC-1.

## Stop conditions

- Any kernel, conversion or `trusted_base()` change.
- A census row or an Eq-goal convoy row changes.
- The rule would apply β-reduction inside a term the kernel later
  compares by identity. Stop to the Architect.
