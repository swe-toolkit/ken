---
id: LANG-REFINEMENT-INTRODUCTION-COVERAGE
title: "LANG-REFINEMENT-INTRODUCTION-OBLIGATION emits a named refinement's obligation only where the context carries the refinement table and the alias body is literally a refinement. Instance, class, record and expression-entry contexts, alias chains and named function-valued refinements still accept an introduction with no obligation and no refusal. Close every named introduction the way the literal form already is"
status: ready
owner: language
size: M
tier: T1
gate: architect
depends_on: [LANG-REFINEMENT-INTRODUCTION-OBLIGATION]
blocks: []
github: null
origin: "Adversary finding evt_23vexyxg1ya1e on 7ca183831. A fail-open gap in the landed deliverable of LANG-REFINEMENT-INTRODUCTION-OBLIGATION (spec 34 §5: every introduction emits phi a). No live catalog, library or examples site uses the three shapes, so it is placed on the language ring after LANG-ENSURES-PER-PATH-REALIZATION, which is in flight on the same emission code. Steward-filed per COORDINATION section 2."
---

# Every named refinement introduction emits or refuses

## Objective

A value introduced at a `def`-named refinement, in any elaboration context
and through any alias chain, emits `φ a` exactly as the literal refinement
does. Where the literal form is refused, the named form is refused too
(spec 34 §5).

## Settled inputs (Adversary `evt_23vexyxg1ya1e`, measured on `7ca183831`)

With `def Five = { x : Int | Equal Int x 5 }`:

- **F1, the context has no table.** `ElabCtx::refinement_facts` defaults to
  `None` (`elab.rs:668`). Only the view, let and mutual-group paths call
  `with_refinements`. In every other context the named arm of
  `emit_refinement_introduction` (`:11320`) returns the core unchanged.
  - `instance Pick Char { pick = 55296 }` gives 0 obligations, against 1
    for `const k : Char = 55296`. The same instance context does report a
    `/` obligation, so it has a channel and lacks only the table.
  - `elaborate_expr("(55296 : Char)")` and `(0 : Five)` are accepted with
    a trusted-base delta of 0. Their literal twin is refused with
    `ObligationWithoutChannel` (ruled route (a), `evt_3h5n5b9y0wzf`).
  - Class, record and associated-declaration contexts were not measured.
- **F2, an alias of a named refinement has no predicate.** The predicate
  is recorded only when the alias body is syntactically `RRefine`
  (`elab.rs:14089-14103`).
  - `def Five2 = Five` with `const c : Five2 = 0` gives 0 obligations,
    and so does `def MyChar = Char` with `const c : MyChar = 55296`.
  - In the other direction, `fn f (p : Five2) : Five = p` gives one
    obligation that can never be discharged.
- **F3, a named function-valued refinement erases its predicate.**
  `def FiveFn = Int -> { x : Int | Equal Int x 5 }` with
  `const f : FiveFn = \m. m` gives 0 obligations. The literal form is
  refused ("a refinement under a function-valued return type is not
  supported yet"), and that refusal is adjudicated.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The refinement table reaches every `ElabCtx` that can introduce a value, by
construction rather than by each caller remembering. An alias resolves to
its underlying named refinement's predicate. A named refinement in a
position whose literal form is refused is refused the same way. The
Architect rules the mechanism at AC-0.

## Acceptance

- **AC-0 (inside this WP).** List every `ElabCtx` construction site and
  whether it carries the refinement table, and every place an alias
  predicate is recorded. The Architect rules the mechanism, and the F3
  refusal's placement, against that list.
- **AC-1.** Each F1, F2 and F3 row above becomes a test with the literal
  form's outcome: 1 open obligation, or the same refusal. The reverse F2
  row emits nothing. Class, record and associated rows are added for any
  context AC-0 finds without the table.
- **AC-2 (controls).**
  - `const k : Char = 55296` still gives 1 open obligation.
  - Re-use at the same named refinement still emits nothing new.
  - `intToChar`'s two arms keep their composed obligation status in
    conformance.
  - `lang_refinement_introduction` and `decimal_char_acceptance` stay
    green.
- **AC-3 (mutation, QA).** Dropping the table at one context, or alias
  resolution, reddens its row.

## Stop conditions

- Any kernel, `trusted_base()` or spec change.
- A checked catalog, library or example program gains an obligation it
  cannot discharge: stop to the Architect with it.
