---
id: LANG-REFINED-PARAMETER-STRUCTURAL-RECURSION
title: "Since subset-Σ landed, structural recursion whose decreasing parameter has a refinement type is refused by the termination gate (K0014), although the same declarations were accepted before. Restore the decreasing edge without weakening the gate"
status: ready
owner: language
size: M
tier: T1
gate: architect
depends_on: [LANG-REFINEMENT-SUBSET-SIGMA]
blocks: []
github: null
origin: "Regression measured by the Adversary's M8 hunt on LANG-REFINEMENT-SUBSET-SIGMA at 245db5cfb (evt_1hr0cnvcr887r). Ordinary structural recursion that base b22a41649 accepted is now refused. Steward-filed per COORDINATION section 2; placed before LANG-RECURSIVE-BRANCH-EVIDENCE-BELOW because the lane's own merge introduced it."
---

# Recursion on a refined parameter terminates again

## Objective

`fn f (n : AnyNat) : Nat = match n { Zero ↦ Zero; Suc m ↦ f m }`, with
`def AnyNat = { x : Nat | Top }`, is accepted with its refinement
obligation as on base, and a non-decreasing recursion through a refined
parameter is still refused.

## Settled inputs (measured on `main` at `245db5cfb`)

- **The refusal.** K0014 `NotTerminating("SCT: idempotent self-loop
  has no strictly-decreasing parameter")` on recursion through a
  refined parameter. Base `b22a41649` accepted the same programs
  (Adversary `evt_1hr0cnvcr887r`).
- **The elaborated shape.** A match on a refined value is now forgotten
  by `Proj1` (`forget_subset_at_elimination_head`,
  `crates/ken-elaborator/src/elab.rs`), and the recursive argument is a
  subset pair of the field and its proof.
- **The gate.** `crates/ken-kernel/src/sct.rs` gives a scrutinee
  provenance only when it is a bare `Term::Var` (~:423), and `size_rel`
  (~:260) relates only a `Var` argument. Each seam alone loses the
  decreasing edge. The SCT code did not change; the term did.
- **Unaffected shapes.** A refined parameter passed unchanged beside a
  decreasing plain one, and a refined result over a plain decreasing
  parameter, are accepted on both.

Treat anchors as perishable. If a fixed input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

A repair that returns the decreasing edge for recursion through a
refined parameter, with committed tests for the shapes below. The first
step reproduces q1 and its plain-`Nat` control on `245db5cfb` and on
`b22a41649`.

## Acceptance

- **AC-0 (D0, Architect).** Rule the seat of the repair. Prefer an
  elaborator-side emission that the unchanged gate already relates. A
  kernel SCT change is a stop (below).
- **AC-1.** These are accepted, each with the refinement obligation it
  had on base: the named `AnyNat` recursion; the inline
  `{ x : Nat | IsTrue (leq_n x ten) }` form; `len` over
  `{ l : List Int | Top }`; an accumulator variant `f m (Suc acc)`; and
  a two-level match `Suc m ↦ match m { Suc p ↦ f p }`.
- **AC-2 (negative).** Recursion through a refined parameter that does
  not decrease (`Suc m ↦ f n`, and `Suc m ↦ f (Suc m)`) is still
  refused with K0014. A repair that accepts either fails.
- **AC-3.** SIGMA's subset-Σ tests stay green, and its AC-2a census
  vector is byte-identical.

## Stop conditions

- The only working repair changes `sct.rs` or any other kernel code.
  Stop with the D0 evidence; a termination-gate change goes to the
  operator.
- Any `trusted_base()` change, or any newly refused program in AC-3.

Checks: 2 fired: read both seams at `245db5cfb`; the scrutinee lookup
(sct.rs ~:423) and `size_rel` (~:260) each accept only a bare `Var`.
7 fired: AC-1 carries all five refused shapes, not just q1, and AC-2
holds the non-decreasing siblings refused.
