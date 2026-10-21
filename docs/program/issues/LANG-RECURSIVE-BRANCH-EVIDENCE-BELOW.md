---
id: LANG-RECURSIVE-BRANCH-EVIDENCE-BELOW
title: "A constructor-branch equation on a recursive family has no evidence term, because Ken's one recursor shares its motive with the induction hypothesis. Carry it on a case-only Elim convoy when the match does not consume the hypothesis"
status: active
owner: language
size: M
tier: T1
gate: architect
depends_on: [LANG-REFINEMENT-SUBSET-SIGMA]
blocks: []
github: null
origin: "Architect recut evt_7729d1mezkcxk of LANG-PATH-CONDITION-EVIDENCE (W2b), on Research advisory evt_2pjgy01xhz4db after the third advancing stop evt_35tcy3w6b3e4f. Operator 2026-10-07: refinements are real kernel types. Steward-filed per COORDINATION section 2."
---

# Recursive branches carry their equation

## Objective

Inside the `C r` branch of a match on a recursive family, the branch
equation `Eq S s (C r)` is supplied when a subset-Σ introduction
beneath it demands it (`LANG-REFINEMENT-SUBSET-SIGMA`, Architect
`evt_41fxwysr833vv`), without changing any induction hypothesis.

## Settled inputs

- Ken's `Elim` is one recursor. Kernel `method_type` forms each induction
  hypothesis by applying the motive to the recursive field
  (`ken-kernel/src/inductive.rs` ~:2149, fields from `recursive_shapes`
  ~:1384). A convoy motive `λ y. Eq S s y → T` therefore turns every
  hypothesis into `Eq S s r → T`, which the branch cannot supply
  (`evt_6vjfqzyn2bx7s`). Dropping the equation loses discharges
  (`evt_70rp9sn0axc7j`).
- **Design (Architect `evt_7729d1mezkcxk`, after the advisory).** As in
  Goguen, McBride and McKinna, and in Lean 4 and Coq Equations: the branch
  equation rides on a case-only use of `Elim`, whose hypotheses are unused,
  and recursion goes through `below_D` and `brecOn_D`, derived from the
  same recursor with the plain motive `T`, so `below (C r)` reduces to the
  result for `r` paired with `below r`. No kernel former, no trust change.
- **The gap pin (on `main` at `245db5cfb`).** SIGMA landed the
  transition sentinel
  `recursive_sort_nil_goal_omits_unwitnessed_list_eq_and_uses_plain_sigma_motive`
  in `crates/ken-elaborator/tests/lang_refinement_subset_sigma_recursive_split.rs`:
  a recursive `sort` with a refined result leaves its `Nil` leaf open
  with no `Eq (List a) xs (Nil a)`, and the `xs` motive is the plain
  subset Σ. This WP flips it.
- Lean 4 compiles a recursive call by projecting from `below`; the
  advisory cites that lookup. It is prior art to understand, not code to
  copy.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

Amended by Architect ruling `evt_ffzy79zqax1b` at `664d86ee8`, which
finds that AC-1's `sort` recurses by a direct `Const sort` self-call
(SCT-admitted), and the method hypothesis is never read.

- One predicate, "this match consumes a method hypothesis", replaces
  the blanket recursive-family refusal in `convoyable_frame`, and is
  applied identically in `checked_constructor_convoy_permitted` and
  `emit_subset_evidence`. A match consumes one when an arm body holds
  `RExpr::RRecursiveResult` or its scrutinee has a `lift_bindings`
  entry. The family must still be unindexed.
- A recursive-family match with demand and no hypothesis consumption
  becomes the case-only `Elim` convoy. A consuming match, or one
  without demand, elaborates as on base.
- After `wrap_dependent_method_ihs` builds a convoy method, no
  hypothesis binder of it occurs free in its body; otherwise
  `ElabError::Internal`.

Out of scope: `below_D`, `brecOn_D` and their `compiler_driver`
lowering. They are filed as their own WP when a consuming match under
demand exists, carrying the D0 `evt_4jsk1apebx4jr` and its constraints.

## Acceptance

- **AC-0 (D0).** Probe that `below_D` for `Nat` and `List` checks through
  `Elim` into `Type` and reduces on a constructor (large elimination, spec
  14 section 3), then the Architect rules the cut.
- **AC-1.** The gap pin flips. The recursive `sort`'s `Nil` leaf goal
  carries a checked `Eq (List a) xs (Nil a)`, and the absent-equation
  assertion is replaced by one that requires it, with no proof
  invented. The `Cons` leaf's recursive call still computes the same
  result as on base, its checked method still holds `Const sort`, and
  its hypothesis binder is unused.
- **AC-2.** Every landed package's emitted core is byte-identical to
  base, and every obligation's count, kind and discharge status is
  identical. A goal proposition may change only by gaining the branch
  equation premise of an eligible match (Direct, non-consuming,
  demanded), and only in that match's own arms. Any other change in
  `Γ ⊢ φ` is the stop (Architect `evt_7f6b1pchpvyya`).
- **AC-3.** Native and reference runtime parity unchanged across the
  recursion corpus.
- **AC-4 (mutation).** A new sentinel, a `recursive result for` match
  under a refined result, is green on the candidate and elaborates as
  on base. Dropping the consumption exclusion reddens it; restoring
  the blanket refusal reddens AC-1.

## Stop conditions

- Any AC-2 outcome change or AC-3 parity change.
- A third hypothesis-producing source path, a consuming match under
  demand in the corpus (including nested-lift matches under a refined
  result), or a demanding member in the AC-3 corpus: query the
  Architect. A demanding convoy that hits native E0074 or E0079 is
  `RT-NATIVE-ERASED-OMEGA-CALL-ARGUMENT`; disclose it.

Checks: 4 fired: `method_type` (`inductive.rs:2138`) and
`recursive_shapes` (`:1384`) exist at `245db5cfb`; the AC-1 subject is
the landed sentinel test, since `pce_rec` is not on `main`.
Amendment: 4 fired: `convoyable_frame`, `checked_constructor_convoy_permitted`
and `emit_subset_evidence` are the three sites the Architect measured at
`664d86ee8`. 8 fired: AC-4 uses mutants that the new sentinel and AC-1
can tell apart, since no consuming match under demand exists.
