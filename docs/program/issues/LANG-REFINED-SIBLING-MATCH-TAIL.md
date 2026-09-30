---
id: LANG-REFINED-SIBLING-MATCH-TAIL
title: "An indexed sibling binder whose index was fixed by matching a different scrutinee is not refined consistently: its tail does not reduce at an open index, a goal mentioning it cannot be classified, and its own match fails coverage. Name the mechanism, then repair match compilation"
status: draft
owner: language
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Architect rulings evt_5p7xx5e6tmegw and evt_31z4jzrt6v8wf (scope widened) on the CAT-VECTOR-DEFERRED-LAWS zip_with_vcons and lookup_zip_with stops: a capability gap in elaborator match compilation, not a proof-shape problem. Steward-filed per COORDINATION section 2. Scheduling it on the Language ring is an operator lane question."
---

# Refinement of an indexed sibling binder

## Objective

A law over two indexed values closes at an open index when one value's
index is fixed by matching the other. Today the sibling binder is not
refined consistently, so `zip_with_vcons` and `lookup_zip_with` in
`Data/Vector/Vector.ken.md` cannot be stated generically.

## Fixed inputs (Architect `evt_5p7xx5e6tmegw`, `evt_31z4jzrt6v8wf`)

The probes use scratch copies of `Vec` and `zip_with` taken verbatim from
`Vector.ken.md`. Each is one theorem closed by `Refl` at open `n`, `xs`
and `ys`.
- **e1**, generic `zip_with_vcons`: rejected with "cannot infer an
  introduction form".
- **e2**, a direct match on `ys : Vec b (Suc m)` (literal `Suc` index)
  returning `tail_ys`: OK.
- **e3**, an outer `match xs` and an inner `match ys` refined by it,
  returning `Suc m`: OK.
- **e6**, the same shape, non-recursive, returning
  `VCons c m (f x y) (g m tail_xs)` for a parameter `g`: OK.
- **e5**, e6 but returning `(g m tail_ys)`: rejected. This is the minimal
  repro, about 12 lines.
- At `n = Zero` the tail is `VNil` and reduces, so a length-one witness
  passes.
- The kernel's `cast_reduce` (`obs.rs`) returns `t` whenever the endpoints
  convert. So the refinement carries `tail_ys` through some other form
  (J on the index equation, or an Elim over an equation-indexed motive),
  which stays neutral at an open index. The introduction-form message is
  the kernel's fall-through, not the cause.

- **Goal refinement** (second ruling), with the same `Vec` and `Fin`
  scratch definitions:
  - **f1, f2, f3:** plain functions nesting `match i`, then `match xs`, then
    `match ys`, returning `Nat`. All OK.
  - **f8:** the same nesting in a theorem whose goal is `Equal Nat n n`.
    OK.
  - **f7, the minimal failure (5 lines):** `(xs : Vec a n) (i : Fin n) :
    Equal (Vec a n) xs xs` by `match i`, each arm matching `xs` into
    `Refl`. Rejected with `BadEliminator("refl a does not match Eq A x y")`.
  - **f4** (goal `Equal (Vec b n) ys ys`) and **f5** (goal
    `Equal (Fin n) i i`): rejected with `Internal("index refinement: could
    not classify the branch goal: TypeMismatch ...")`.
  - **`lookup_zip_with`:** `ExhaustivenessError` at the inner `match ys`
    after `i → xs`.
- **The shared predicate.** The failure follows a goal, tail or match that
  mentions a sibling refined by a different scrutinee, not nesting depth.
  `lookup_map` passes because its goal does not mention the refined index.

Treat anchors as perishable. If a probe's verdict differs on the landed
base, stop and report it.

## Deliverable

A match-compilation repair so a sibling binder refined by a different
scrutinee reduces, classifies its goal, and covers its match at an open
index. The landed `zip_with` definition is unchanged.

## Acceptance

- **AC-0 (dump, then ruling; no fix).** Dump the elaborated refinement for
  e5 and f7, and say whether one mechanism explains all three signatures:
  a non-reducing tail, an unclassified goal, and sibling exhaustiveness.
  Two mechanisms stop to the Architect before any fix.
- **AC-1.** e5, f7, f4 and f5 check. `zip_with_vcons` and
  `lookup_zip_with` check unchanged.
- **AC-2 (controls).**
  - e2, e3, e6, f1-f3 and f8 still check, and the `n = Zero` witness still
    passes.
  - Reverting the repair brings back the e5 and f7 rejections.
  - The whole-catalog census shows no verdict change, or each change is
    named and ruled.

## Stop conditions

- Any kernel conversion change, or any trust change.
- Any change to the landed `zip_with` definition.
- Once this lands, `zip_with_vcons` and `lookup_zip_with` resume in
  `CAT-VECTOR-DEFERRED-LAWS` unchanged.
