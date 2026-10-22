---
id: LANG-INFER-IF-SPLIT-DEMAND-ROUTE
title: "A call-site requires call or subset introduction inside an if elaborated in infer mode fails with the internal retry signal RefinementSplitDemand (E0082), because convoyable_frame treats every If frame as convoyable while infer_if has no route to answer the demand. LANG-CLOSE-GOAL-PATH-CONDITIONS extended the pre-existing subset gap to the requires route. Answer or decline the demand so these programs elaborate again"
status: ready
owner: language
size: S
tier: T1
gate: architect
depends_on: [LANG-CLOSE-GOAL-PATH-CONDITIONS]
blocks: []
github: null
origin: "Adversary finding evt_18rn9f8hhfmv8 on a2a4169d3: a new fail-closed refusal against that WP's AC-2 control, no soundness impact. Steward-filed per COORDINATION section 2."
---

# An inferred if answers or declines a split demand

## Objective

A program with a call-site `requires` call, or a subset-refinement
introduction, inside an `if` elaborated in infer mode elaborates instead of
escaping with `RefinementSplitDemand`. Base `77352e68e` accepted the
`requires` rows with a path-free hole.

## Settled inputs (Adversary `evt_18rn9f8hhfmv8`, on `a2a4169d3`)

- **The demand.** `precondition_proof` (`elab.rs:6700`) raises
  `RefinementSplitDemand` for the innermost unwitnessed frame whenever
  `convoyable_frame` (`:14624`) is true, and that returns true for every
  `SplitKind::If` frame.
- **No answer.** `infer_if` (`:2223`) pushes its If frames through
  `push_path` but has no catch and no convoy arm, unlike `check_if`. The
  demand escapes with a fresh id, or retries the enclosing split once and
  then escapes.
- **Rows.** With `fn need (b : Bool) : Bool requires Equal Bool b True = b`
  (a3: `fn needi (x : Int) : Int requires Equal Int x 5 = x`):
  - a1 `let y = if b then need b else b in y`;
  - a2 `match (if b then need b else b) { True |-> True ; False |-> False }`;
  - a3 `let y = if b then needi x else x in y`;
  - a4 the a1 body nested in the `then` arm of an outer checked `if c`.
  All four are E0082 on `a2a4169d3` and OK on base.
- **Sibling, pre-existing.** s1:
  `fn h (x : {v : Bool | Equal Bool v True}) : Bool = x` with
  `let y = if b then h b else b in y` is E0082 on both trees, through
  `emit_subset_evidence` (`:14811`).

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

The Architect first rules the route: give `infer_if` a convoy arm that
answers the demand, or key `convoyable_frame` on whether the frame's split
has a retry route so an unanswerable frame keeps the path-free closure. One
repair covers both the `requires` and the subset route, plus a test with
the five rows.

## Acceptance

- **AC-1.** a1 to a4 and s1 elaborate. Each `requires` row has no more
  open holes than on base, and any hole the ruled route closes checks
  against its hole type.
- **AC-2 (controls).** The Adversary's controls (checked-mode `if`,
  inferred `let` body, lambda in a checked `if`, literal match, polymorphic
  argument, `/` in an inferred `if`) keep their `a2a4169d3` outcome, and no
  hole closed on `a2a4169d3` is open on the candidate across catalog,
  library and examples.
- **AC-3 (mutation, QA).** Restoring the unconditional If arm in
  `convoyable_frame`, or removing the new `infer_if` arm, reddens a1 to a4.

## Stop conditions

- Any kernel, `trusted_base()` or spec change.
- A demand that still escapes as E0082 from any other `push_path` writer.

Checks: 2 fired: the Adversary enumerated every `push_path` writer against
the seven catch sites; `infer_if` is the one with no catch. 7 fired: AC-1
names the sibling s1 route, so a fix limited to `precondition_proof` fails.
