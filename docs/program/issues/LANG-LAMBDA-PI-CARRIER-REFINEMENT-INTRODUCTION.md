---
id: LANG-LAMBDA-PI-CARRIER-REFINEMENT-INTRODUCTION
title: "A lambda checked against a named refinement whose carrier is a function type emits no obligation: const c : Fn5 = \\y. 6 with def Fn5 = { f : Int -> Int | Equal Int (f 0) 5 } gives 0 obligations, while the literal refinement gives 1. Check's lambda arm whnf's the expected type to the Pi carrier and never reaches the refinement introduction. Emit the introduction obligation there"
status: ready
owner: language
size: S
tier: T1
gate: architect
depends_on: [LANG-NAMED-REFINEMENT-TYPE-ARGUMENT]
blocks: []
github: null
origin: "Adversary hunt evt_2zc1nvkwwy0sg on 68bd8ad73 (LANG-MATCH-RESULT-REFINEMENT-IDENTITY): fail-open on the verification layer, kernel unaffected, pre-existing on the direct route. Named by that WP's stop condition (a structural read through a refinement whose carrier is a Pi). Violates 22 §2.1. Steward-filed per COORDINATION section 2."
---

# A lambda at a Pi-carrier refinement emits its introduction obligation

## Objective

Checking a lambda against a named refinement whose carrier is a function type
emits `φ[λ]`, as `22-obligations.md` §2.1 requires and as the literal form
already does.

## Settled inputs (Adversary `evt_2zc1nvkwwy0sg`, on `68bd8ad73`)

- Header: `def Fn5 = { f : Int -> Int | Equal Int (f 0) 5 }`. Per-declaration
  obligation counts from `ElabEnv::elaborate_file_v1`:
  - `const c : Fn5 = \y. 6` gives 0; `fn g (u : Bool) : Fn5 = \y. 6` gives 0.
  - The literal twin `const c : { f : Int -> Int | Equal Int (f 0) 5 } = \y. 6`
    gives 1, goal `Equal Int ((λ. 6) 0) 5`.
  - `fn g (h : Int -> Int) : Fn5 = h` gives 1.
  - Flat `match b { True ↦ \y. 5; False ↦ \y. 6 }` and `if b then \y. 5 else
    \y. 6` at `Fn5` give 0; the indexed match with one `h` leaf gives 1.
  - Consumer: `fn use5 (f : Fn5) : { r : Int | Equal Int r 5 } = f 0`
    discharges from its refined parameter, so `use5 c` returns 6 under a
    verified "returns 5" contract.
- Mechanism: `check`'s `RLam` arm (`elab.rs` ~2206) whnf's `expected`, which
  delta-unfolds `Fn5` to its Pi carrier, and checks the body against the
  codomain. It never reaches `emit_refinement_introduction` (~11709), whose
  callers are all infer-and-compare fallbacks. The literal form emits through
  `literal_result_predicate` (~11692).

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

Per Architect `evt_7vpddt8z52k0j`, using the rigid conversion view that
`LANG-NAMED-REFINEMENT-TYPE-ARGUMENT` delivers: at the `RLam` check arm, take
the expected head through `with_rigid_consts`, or read its `refinement_root`
before any whnf. A root head is checked at the carrier and routed to
`emit_refinement_introduction` with the root as expected. Sweep by mechanism:
every `check` arm that whnf's `expected`, not `RLam` alone, and report the
arms found. The match and `if` routes inherit the fix.

## Acceptance

- **AC-1.** Each `Fn5` row above with a lambda leaf gives the literal twin's
  count: `const c` and `fn g` give 1 each, with goal `Equal Int ((λ. 6) 0)
  5`; the flat match and `if` give 1 per lambda leaf. The goal on `c` is
  false, so `use5 c` no longer passes with every obligation discharged.
- **AC-2 (controls).** The literal twin, the `h` variable row and the indexed
  match keep their counts. `examples/` and the 55 `catalog/packages` keep
  base results file by file.
- **AC-3 (mutation, QA).** Removing the new emission returns `const c` to 0.

## Stop conditions

- The emission needs a kernel, `trusted_base()` or spec change.
- Any newly refused corpus, catalog or example program.
- The site reads refinement presence off a `Const` spelling instead of the
  refinement's checked identity (`LANG-NAMED-REFINEMENT-TYPE-ARGUMENT` recut,
  `evt_5rpfj17mr3n9n`).
