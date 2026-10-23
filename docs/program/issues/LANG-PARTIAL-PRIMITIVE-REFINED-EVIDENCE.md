---
id: LANG-PARTIAL-PRIMITIVE-REFINED-EVIDENCE
title: "Integer division by a refined divisor {d : Int | Not (Equal Int d 0)} still leaves an open PartialPrim hole, though spec 35 section 3.1 says such an argument makes div and mod total and spec 21 section 6.3's evidence search lists Proj2 of a refined variable. Close the side condition from that evidence with a kernel-checked term"
status: merged
owner: language
size: S
tier: T1
gate: architect
depends_on: [LANG-REFINEMENT-SUBSET-SIGMA]
blocks: []
github: null
origin: "Architect ruling evt_50sa3e3dyfcsc: LANG-REFINED-PARAM-REQUIRES-DESUGAR is superseded by spec 21 section 6.3 (subset Σ), and its one undelivered objective is this residual. Steward-filed per COORDINATION section 2; Language successor after LANG-SOLVED-INDEX-SUBSTITUTION."
---

# Partial primitives use refined-variable evidence

## Objective

`fn f (n : Int) (d : {z : Int | Not (Equal Int z 0)}) : Int = n / d`,
and the same with `%`, elaborates with no open obligation. The divisor
side condition is a recorded PartialPrim obligation, discharged by a
term built from `Proj2 d` that the kernel checks.

## Settled inputs (Architect `evt_50sa3e3dyfcsc`, main `7eded7922`)

- **The contract.** Spec 35 §3.1: a `{d | d ≠ 0}` argument makes `div`
  and `mod` total. Spec 21 §6.3's checked-evidence search lists `Proj2 y`
  for a refined variable `y`.
- **The goal.** `NonZeroDivisor b := Eq Int b 0 → Bottom` (`numbers.rs`
  ~:454), the predicate the refinement carries at `Proj1 d`.
- **The gap.** The `NonZero` site (`elab.rs` ~:14257) consults only
  `premise_proof_in_scope`, which scans Γ entries and assumptions. It
  never tries `Proj2 d`, so the hole stays open.
- **The pin.** `int_div_mod_surface.rs` ~:148,
  `refined_divisor_keeps_the_open_operation_hole_without_an_unchecked_hypothesis`,
  asserts exactly one open `PartialPrim` hole for both spellings. This WP
  turns it into the discharged row.
- **Already delivered (do not rebuild).** A caller `g = f 1 0` owes one
  `RefinementIntroduction` (`int_div_mod_surface.rs` ~:264), and `f` is
  refused where `Int → Int → Int` is expected (K0004).

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The partial-primitive side condition runs the spec 21 §6.3 evidence
search, including `Proj2` of a refined variable, and closes the goal with
the found term. The pinned test asserts 0 obligations and no trust growth
for both spellings.

## Acceptance

- **AC-0 (D0, Architect).** Rule where the search runs and what term it
  emits. The suppression at ~:14257 is elaborator-only conversion that
  never reaches the kernel, so the new evidence must be a term the kernel
  checks in the elaborated body, not a skipped hole.
- **AC-1 (corrected by the D0, `evt_gjdwyw89122r`).** `/` and `%` with
  the refined divisor each record one PartialPrim obligation that is
  closed (`!is_open_hole`), with the transparent body `λ n. λ d. Proj2 d`
  and `ken_kernel::check` success. `trusted_base()` membership is
  unchanged. The caller `g = f 1 0` still owes its one open
  `RefinementIntroduction`.
- **AC-2 (negative).** A plain `Int` divisor, and a refined divisor whose
  predicate is not `Not (Equal Int z 0)` (for example `Top`), keep their
  one open `PartialPrim` hole.
- **AC-3 (mutation).** Emitting the evidence without the kernel checking
  it (skipping the hole only) reddens a kernel-check pin; dropping the
  `Proj2` arm reddens AC-1.

## Stop conditions

- Any kernel or `trusted_base()` change.
- A newly refused program in the `int_div_mod_surface` or
  `lang_refinement_subset_sigma_*` suites.

Checks: 4 fired: the ~:14257 site, `premise_proof_in_scope`,
`NonZeroDivisor` and the ~:148 pin exist at `7eded7922` (Architect,
measured). 7 fired: AC-2's `Top` divisor is a refined variable whose
`Proj2` exists but proves the wrong proposition, which a search that
accepts any `Proj2` would wrongly discharge.
