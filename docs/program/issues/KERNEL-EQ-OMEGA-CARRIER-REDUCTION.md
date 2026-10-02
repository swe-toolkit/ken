---
id: KERNEL-EQ-OMEGA-CARRIER-REDUCTION
title: "The kernel refuses Eq at an Omega-classified carrier, leaves the Sigma and inductive Eq reducts stuck whenever a component is Omega, and keeps a Trunc-to-Top reduct that breaks subject reduction at level 1. Form Eq with classify, make every Omega-carrier Eq neutral, discard Omega components, and delete the Trunc arm"
status: ready
owner: kernel
size: M
tier: T1
gate: architect
depends_on: [SPEC-EQ-FORM-OMEGA-CARRIER, KERNEL-OBS-PI-CAST-GATE]
blocks: []
github: null
origin: "Operator 2026-10-02: kernel change 'provisionally approved, given clean or mitigatable research findings'; Research evt_782fh9yxr4xk8 clean or mitigatable. Architect ruling evt_7tnycbxzgp75x as amended by evt_sbv31qypx3j7. Placed on the kernel ring after KERNEL-OBS-PI-CAST-GATE. Steward-filed per COORDINATION section 2."
---

# Eq at an Ω carrier is formed and stays neutral

## Objective

The kernel implements `SPEC-EQ-FORM-OMEGA-CARRIER`. `Eq P u v` is formed
for `P : Ω_l` at `Ω_l` and never reduces. The subset Σ and inductive Eq
reducts form when a component is Ω. No term reduces Eq to `Top` at the
wrong level.

## Fixed inputs (Architect `evt_7tnycbxzgp75x`, amended `evt_sbv31qypx3j7` and `evt_5hryk5pap4q78`, read at `948864d3e`)

- **Eq-Form** (`check.rs:324-330`) uses `synth_type`, which refuses Ω
  (`check.rs:180`). Cast admission (`:331-340`) also uses `synth_type`, and
  stays that way.
- **`eq_reduce`** (`obs.rs:122`) has the Trunc arm `Trunc → Top` at `:128`.
  `Top` is `Ω_0`, and `‖A‖ : Ω_l`.
- **`eq_at_sigma`** (`obs.rs:293`) and **`inductive_conjuncts`** (`:583`)
  build the dependent component through `type_eq_by_j`, which returns None
  for a non-Type source. Both are stuck today whenever a component is Ω.
- **Refl** (`check.rs:483-499`) checks by `convert_type` against
  `Eq (infer a) a a`. `convert_path`'s Ω-PI shortcut (`conv.rs:1018`)
  accepts any two terms at an Ω type, so no Refl change is needed.
- **No Ω-sorted inductive families.** `build_types` (`env.rs:1042-1045`)
  always forms `Type ℓ`, so large elimination out of Ω is closed by
  construction.
- **The kernel shape** is in the ruling: one `omega_classified` guard
  before `eq_reduce`'s head match, plus the R2 and R3 discards.

Treat anchors as perishable. If a fixed input is false on the landed base,
stop and report the mismatch; do not build around it.

## Deliverable

1. **Eq-Form** classifies its carrier: Type gives `Ω_l`, and Ω gives `Ω_l`.
2. **R1.** In `eq_reduce`, an Ω-classified carrier returns None before the
   head match. Delete the Trunc→Top arm.
3. **R2.** In `eq_at_sigma`, an Ω-classified codomain gives the second
   conjunct `Eq (B1 q.1) q.2 q.2`, with no Cast.
4. **R3.** In `inductive_conjuncts`, an Ω-classified field gives
   `Eq (A_j[b̄]) b_j b_j`, with no J witness, at the field's own level.
   Add no level lift or guard: the inductive rule's level gap is
   pre-existing and is a separate question (`evt_5hryk5pap4q78`).
5. **Rows.** The five `SPEC-EQ-FORM-OMEGA-CARRIER` conformance rows pass, as
   kernel tests beside `obs_sigma_quot_cast_gate.rs`.

## Acceptance

- **AC-0 (measure; no edit).**
  - Record each of the five rows on main as refused or stuck.
  - Sweep every source root (`crates/*/tests`, `catalog/`, `conformance/`,
    `examples/`) for a row that asserts Eq-Form refuses an Ω carrier, or
    that Eq at Trunc is Top. List each one; it flips or is reported.
- **AC-1.** The five rows pass. Row 2 is a `Type 0` family with two
  syntactically distinct Ω-field proofs.
- **AC-2 (falsifiers; each must redden).**
  - Removing the R1 guard reddens row 4's whnf assertion.
  - Restoring Trunc→Top without the guard reddens row 3.
  - Removing the R2 discard reddens row 1, and removing the R3 discard
    reddens row 2.
- **AC-3 (controls).**
  - `subset_sigma_neutral_equality_stays_stuck` and the other cast-side
    rows are unchanged.
  - The J rows over a Type-carrier singleton stay green.
  - `trusted_base()` is unchanged, and the catalog census has no verdict
    change.

## Stop conditions

- Any cast-side Ω sibling (`eq_at_type` Σ/Π, `cast_at_sigma`,
  `cast_at_inductive`) would need to compute: stop to the Architect.
- `trusted_base()` changes: an operator question.
- A term outside the five rows starts or stops checking: stop to the
  Architect with it.
