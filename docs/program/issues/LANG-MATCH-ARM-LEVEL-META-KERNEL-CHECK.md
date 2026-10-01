---
id: LANG-MATCH-ARM-LEVEL-META-KERNEL-CHECK
title: "A dependent-match arm is kernel-checked while a bare-Type level metavariable is still unsolved, so fn vid (a : Type) (n : Nat) (xs : Vec a n) : Vec a n = match xs { … } is falsely rejected with Type 0 vs Type u0 and the prelude routes 38 arms per environment into the generalized fallback. Zonk levels before any kernel query, as spec 39 requires"
status: active
owner: language
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Adversary finding evt_3e414pny1xwjh (correctness, false rejection, pre-existing; identical on f76381399 and 0ae184458). Reachable from well-formed source. Steward-filed per COORDINATION section 2."
---

# No level metavariable reaches a kernel query

## Objective

Spec 39 (`39-elaboration.md:873-876`): a bare `Type` elaborates to
`Univ(?u)`, `?u` is solved during elaboration, and "the kernel never receives
a level metavariable". A dependent-match arm's kernel check honours this, so
an arm is accepted or refused on its own merits.

## Settled inputs (Adversary `evt_3e414pny1xwjh`, read at `0ae184458`)

- **The repro**, using the prelude and `data Vec (a : Type) : Nat → Type` with
  `VNil` and `VCons`:
  - `fn vid7 (a : Type) (n : Nat) (xs : Vec a n) : Vec a n = match xs
    { VNil ↦ xs; VCons m x t ↦ xs }` is rejected at the VNil arm with
    `KernelRejected TypeMismatch { expected: Type 0, found: Type u0 }`;
  - so are the non-recursive identity and its arm-swapped twin;
  - the same program with `(a : Type 0)` is accepted, and so is catalog
    `Vector.map`.
- **The site.**
  - `elab.rs:6132` kernel-checks the arm against `expected_unrefined`.
  - With no premise view this reaches `kernel_check_raw` (`:8292`) on an
    unzonked context.
  - On `RecursiveFieldIndexPath::PlainDeclared`, `:6144` returns that
    rejection as final. Otherwise the arm falls into
    `check_match_dependent_refined_fallback`.
- **Blast radius.**
  - All 266 of 266 first-attempt arm failures in the Adversary's run were
    this mismatch: 38 per `ElabEnv::new()`, all in the prelude.
  - Each of those arms is routed into the generalized fallback, so that
    path's population is set by the premature check.
  - The two `kernel_check_current` calls in `check_generalized_branch_goal`
    carry the same hazard.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

Every kernel query on the match-arm path, including the generalized-branch
checks, sees a context and terms with no unsolved level metavariable. The
levels are solved or zonked first, or the query waits until they are.

## Acceptance

- **AC-1.**
  - `vid7`, the non-recursive identity and its swapped twin are accepted,
    with the same core as their `(a : Type 0)` twins.
  - The prelude's first-attempt arm failures of this kind drop from 38 to 0
    per environment. Count them by error kind, as the Adversary did.
- **AC-2 (controls).**
  - A genuinely ill-typed arm under a bare-`Type` parameter is still
    rejected, at the same arm.
  - Reverting the fix turns the `vid7` row red.
  - The `LANG-SIBLING-GOAL-REFINEMENT` rows and controls keep their
    verdicts. An arm that now leaves the fallback path is named in the
    handoff.

## Stop conditions

- A level metavariable that cannot be solved before the arm's check without
  reordering elaboration: stop to the Architect.
- Any kernel change.
