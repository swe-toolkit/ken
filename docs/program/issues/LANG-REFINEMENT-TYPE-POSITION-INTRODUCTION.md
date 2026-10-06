---
id: LANG-REFINEMENT-TYPE-POSITION-INTRODUCTION
title: "Three literal-side refinement fail-opens: a refinement nested inside a type argument (List ({ x : Int | phi })) is erased and accepted with no obligation; an application written in type position (RType::RApp) checks a refined argument with no obligation; and a theorem, proof or recursive view with a literal refined parameter records no fact, so its call sites emit nothing. Spec 34 §5 says every introduction emits phi a. Close all three, after a D0 census"
status: active
owner: language
size: M
tier: T1
gate: architect
depends_on: [LANG-REFINEMENT-INTRODUCTION-COVERAGE]
blocks: []
github: null
origin: "Architect evt_16s8ty301b4y4 and evt_2phgfmsxqjyfa (F-C) on the LANG-REFINEMENT-INTRODUCTION-COVERAGE AC-1 fixture stops: F-A measured by the language implementer on b81ecf220 (evt_5ezfp5q44zmxm), F-B shown by the vacuous 0/0 theorem witness. Outside that WP's frame. No kernel impact: predicates are erased and the kernel term stays well-typed. Steward-filed per COORDINATION section 2."
---

# Every surface refinement is introduced or refused

## Objective

A refinement written in a type argument, a refined parameter applied in
type position, and a literal refined parameter on any declaration route
each either emit `φ a` (spec 34 §5) or are refused. None is silently
erased.

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
- **F-C, literal refined parameters off the annotated-view route**
  (Architect `evt_2phgfmsxqjyfa`). `refined_params` has one producer,
  `elaborate_v0` (`collect_refined_params` at `elab.rs:16197`, insert at
  `:16230`), and one reader (`:10014`). `theorem`/`proof` declarations
  and recursive views (`elaborate_recursive_view`, `:16172`) record no
  fact, so a call such as `proof_lit five` emits 0 obligations against 1
  for the named twin. COVERAGE keeps that pair as an `#[ignore]` row named
  for F-C; this WP un-ignores it.
- **Candidate closures** (the Architect rules at D0): for F-A, the
  exhaustive nested-refinement walk from COVERAGE applied to every surface
  type annotation; for F-B, the introduction route at `RApp` argument
  elaboration, or a refusal; for F-C, one refined-parameter producer on
  every route that elaborates a typed signature.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

1. **D0, at the start of the repair (measure only).** Every `RApp`
   argument-elaboration site, every surface type-annotation site that can
   hold a nested `RRefine`, every declaration route that elaborates a typed
   signature and whether it calls `collect_refined_params`, and the live
   catalog, library and examples population of each shape. The Architect
   rules the closure.
2. **The ruled closure**, with one emission or refusal path shared with
   the value-check route.

## Acceptance

- **AC-1.** The F-A row is refused or emits one open obligation per
  element as ruled, and the `Cons Int 6` twin does not pass silently. The
  F-B row emits the refined parameter's obligation, or is refused. The
  F-C `#[ignore]` row is un-ignored and gives 1/1.
- **AC-2 (controls).** COVERAGE's rows and `lang_refinement_introduction`
  keep their results. Unrefined type applications and annotations are
  unchanged.
- **AC-3 (mutation, QA).** Reverting the closure brings back the zero
  obligation on each row.

## Stop conditions

- Any kernel, `trusted_base()` or spec change.
- A checked catalog, library or example program gains an obligation it
  cannot discharge, or is newly refused: stop to the Architect with it.

## Symptom inventory

Seeded from Architect `evt_5m4fpkn01rhdj`, which also rules the respin of
WIP `482227c63` on `c49297983` (§1a count 1).

```text
SYMPTOM INVENTORY (append one line per hard-stop; never rewrite history)
1. type-position argument refinement decided by local kernel inference -- keyed on the elaboration context being kernel-ready, which collect_refined_params violates under anonymous arrows
2. collect_refined_params re-elaborates signature domains that elab_type already elaborated -- keyed on elab_type being pure, which F-B ended
```
