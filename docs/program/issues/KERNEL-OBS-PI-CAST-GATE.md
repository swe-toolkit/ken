---
id: KERNEL-OBS-PI-CAST-GATE
title: "The Π cast fires when its Eq Type stays neutral, because cast_at_pi checks only the domain levels, and it projects e.1 and e.2 from non-Σ evidence, so its reduct does not check (NotASigma). Gate Π as Σ and quotient are gated, and measure the inductive index-change projection, the remaining ungated arm"
status: ready
owner: kernel
size: S
tier: T1
gate: architect
depends_on: [KERNEL-INT-DIV-MOD-NATIVE]
blocks: []
github: null
origin: "Adversary M8 evt_77xwvz1n3age0 on ed112ae4f (KERNEL-OBS-SIGMA-QUOT-CAST-GATE): pre-existing, fails closed. Architect evt_5kvvbn4ajxt1n: reachable from surface J, fix shape confirmed. Kernel change, placed after KERNEL-INT-DIV-MOD-NATIVE and ahead of KERNEL-J-NONREFL-ENDPOINT-SHARING; operator 2026-10-02: 'approve function-type cast fix'. Steward-filed per COORDINATION section 2."
---

# The Π cast fires only on a decomposing equality

## Objective

Every `cast_reduce` arm that projects `e` fires only when `eq_at_type` on
the same endpoints yields the shape it projects from (spec 16 §3.2). A Π
cast whose `Eq Type` stays neutral is stuck.

## Settled inputs (Adversary `evt_77xwvz1n3age0`, Architect `evt_5kvvbn4ajxt1n`, read at `ed112ae4f`)

- **The site.** `cast_at_pi` (`obs.rs:807`) compares only the domain levels
  (`:817-819`). The Π dispatch at `:783` is not gated, though `eq_at_type`'s
  Π arm (`:410`, `:433`) returns None when the codomain levels differ. The
  cast then projects `e.1` (`:821`) and `e.2` (`:834`) from neutral
  evidence. SIGMA-QUOT gated only Σ and Quot (`:784`, `:790`). The earlier
  premise that TYPE-EQ-STRUCTURAL gated Π was false.
- **Kernel repros.** These use an empty context, `e : Eq (Type 1) S T` and
  an opaque `f : S`, testing `cast S T e f`:
  - PI_COD_LEVEL: `S = (x : Type 0) → Nat`, `T = (x : Type 0) → Type 0`.
    It fires to a λ, and `check(reduct, T)` gives `NotASigma`.
  - PI_COD_DEP: `S = (X : Type 0) → X`, with the same `T` and the same
    result.
- **Surface reachability.** Surface `J` reduces through `j_nonrefl`
  (`obs.rs:1194-1250`) to `Cast(P a refl, P b e, pair_eq, base)`, and this
  is well-typed:

  ```ken
  fn probe (e : Eq (Type 1) ((x : Type 0) → Nat) ((x : Type 0) → Type 0))
           (f : (x : Type 0) → Nat) : (x : Type 0) → Type 0 =
    J (λx _. x) f e
  ```

  It is unmeasured whether checking `probe`, or a use of it, forces that
  whnf. The Architect sees no path to accepting a false proposition.
- **The ill-typed raw row.** `k2_cast_computes_pi_to_lambda`
  (`acceptance.rs:1750`) casts between Π types at `Type 0` and `Type 1`
  under `Eq (Type 1)`; its target is `: Type 2`. It is the twin of the Σ
  row the SIGMA respin flipped, and the only row the gate reddens.
- **The other projecting arm.** `cast_at_inductive`'s index-change path
  projects `e` through `telescope_projection(e, index, ..)` and is ungated.
  Regularity, Ω/Ω, refl `Type`/`Type` and the parameter-agreeing inductive
  path do not project `e`.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

1. **The Π gate.** In `cast_reduce`, the Π arm fires only if
   `type_eq_has_components(env, ctx, a, b)`, the gate Σ and Quot use.
2. **The inductive index-change path.** At AC-0, measure whether
   `inductive_conjuncts` (`:583`) can return None, or a non-telescope, while
   that path fires. Probe with an index whose type differs between
   endpoints by level, or by a dependent earlier index.
   - If it can, gate the path on `eq_at_type(..)` yielding the telescope
     shape, as for Π.
   - If it cannot, record the measured reason as a guard row.

## Acceptance

- **AC-0 (measure; no edit).**
  - Run `probe` on main and record whether it is accepted, rejected or
    stuck.
  - Run the inductive index-change probe above.
- **AC-1.**
  - PI_COD_LEVEL and PI_COD_DEP are stuck, not reduced.
  - `probe` is recorded again on the candidate.
  - `k2_cast_computes_pi_to_lambda` becomes
    `k2_cast_pi_codomain_level_mismatch_stays_neutral`.
- **AC-2 (controls).**
  - A decomposing Π (`Nat → A` to `Nat → B`) still fires, and
    `check(reduct, T)` is Ok.
  - `(x : Type 0) → Ω0` to `(x : Type 0) → Type 0` still decomposes and
    checks.
  - The Σ and Quot gate rows are unchanged.
  - Removing the Π gate reddens the AC-1 rows.
- **AC-3.** `trusted_base()` is unchanged, the catalog census has no verdict
  change, and the cast and TYPE-EQ rows stay green.

## Stop conditions

- A cast that fires today on a decomposing equality would be stuck: stop to
  the Architect.
- The inductive path needs more than a gate: stop to the Architect.
- A spec change.
