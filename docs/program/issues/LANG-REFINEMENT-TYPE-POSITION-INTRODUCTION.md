---
id: LANG-REFINEMENT-TYPE-POSITION-INTRODUCTION
title: "Two literal-side refinement fail-opens: a refinement nested inside a type argument (List ({ x : Int | phi })) is erased to its carrier and accepted with no obligation, and an application written in type position (RType::RApp) checks a refined parameter's argument with no obligation. Spec 34 §5 says every introduction emits phi a. Close both, after a D0 census"
status: ready
owner: language
size: S
tier: T1
gate: architect
depends_on: [LANG-REFINEMENT-INTRODUCTION-COVERAGE]
blocks: []
github: null
origin: "Architect evt_16s8ty301b4y4 on the LANG-REFINEMENT-INTRODUCTION-COVERAGE AC-1 fixture stops: F-A measured by the language implementer on b81ecf220 (evt_5ezfp5q44zmxm), F-B shown by the vacuous 0/0 theorem witness. Outside that WP's frame. No kernel impact: predicates are erased and the kernel term stays well-typed. Steward-filed per COORDINATION section 2."
---

# Every surface refinement is introduced or refused

## Objective

A refinement written in a type argument, and a refined parameter applied
in type position, each either emit `φ a` (spec 34 §5) or are refused.
Neither is silently erased.

## Settled inputs (Architect `evt_16s8ty301b4y4`, on `b81ecf220`)

- **F-A, nested refinement in a type argument.** `const xs : List ({ x :
  Int | Equal Int x 5 }) = Cons Int 5 (Nil Int)` is accepted with zero
  obligations, and `Cons Int 6 (Nil Int)` would be accepted equally.
  `elab_type` erases a nested `RRefine` to its carrier
  (`crates/ken-elaborator/src/elab.rs:1219`).
  LANG-REFINEMENT-INTRODUCTION-COVERAGE refuses the same shape in an alias
  declaration, so the literal form disagrees with the alias form.
- **F-B, refined parameters applied in type position.** In a theorem
  statement, `Equal Int (use_lit five) five` checks `five` against
  `{ x : Int | Equal Int x 5 }` with no obligation, because `RType::RApp`
  elaborates arguments without the value-check introduction route. A
  refinement precondition in any statement-position application is
  unchecked.
- **Candidate closures** (the Architect rules at D0): for F-A, the
  exhaustive nested-refinement walk from COVERAGE applied to every surface
  type annotation; for F-B, the introduction route at `RApp` argument
  elaboration, or a refusal.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

1. **D0, at the start of the repair (measure only).** Every `RApp`
   argument-elaboration site, every surface type-annotation site that can
   hold a nested `RRefine`, and the live catalog, library and examples
   population of each shape. The Architect rules the closure.
2. **The ruled closure**, with one emission or refusal path shared with
   the value-check route.

## Acceptance

- **AC-1.** The F-A row is refused or emits one open obligation per
  element as ruled, and the `Cons Int 6` twin does not pass silently. The
  F-B row emits the refined parameter's obligation, or is refused.
- **AC-2 (controls).** COVERAGE's rows and `lang_refinement_introduction`
  keep their results. Unrefined type applications and annotations are
  unchanged.
- **AC-3 (mutation, QA).** Reverting the closure brings back the zero
  obligation on each row.

## Stop conditions

- Any kernel, `trusted_base()` or spec change.
- A checked catalog, library or example program gains an obligation it
  cannot discharge, or is newly refused: stop to the Architect with it.
