---
id: KERNEL-OBS-PI-CAST-GATE
title: "Cast and J rules fire without checking a side condition their result needs: the Π cast projects from a neutral Eq Type (NotASigma), the index, parameter and level cast arms build ill-typed reducts, and infer_j accepts a motive with a wrong second domain. Gate each rule on its side condition and leave the term neutral otherwise"
status: active
owner: kernel
size: M
tier: T1
gate: architect
depends_on: [KERNEL-LEVEL-CLOSURE-CHECK]
blocks: []
github: null
origin: "Adversary M8 evt_77xwvz1n3age0 on ed112ae4f (KERNEL-OBS-SIGMA-QUOT-CAST-GATE): pre-existing, fails closed. Architect evt_5kvvbn4ajxt1n: reachable from surface J, fix shape confirmed. Kernel change, placed after KERNEL-INT-DIV-MOD-NATIVE and ahead of KERNEL-J-NONREFL-ENDPOINT-SHARING; operator 2026-10-02: 'approve function-type cast fix'. Research sweep evt_7daqm6ydmbnsr S3/S4/S6/S11 folded on Architect evt_24070d0c9zgmw item 2; operator 2026-10-03: 'approve A and B'. Steward-filed per COORDINATION section 2."
---

# Cast and J rules fire only when their side condition holds

## Objective

Every `cast_reduce` arm that projects `e` fires only when `eq_at_type` on
the same endpoints yields the shape it projects from (spec 16 §3.2). A Π
cast whose `Eq Type` stays neutral is stuck. More generally (Architect
`evt_24070d0c9zgmw` item 2), no cast or eliminator rule fires without the
side condition its result needs: where the condition is not met, the term
stays neutral, and `infer_j` refuses an ill-formed motive.

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

## Settled inputs, folded arms (Research `evt_7daqm6ydmbnsr`, Architect `evt_24070d0c9zgmw`)

One predicate: a cast or eliminator rule fires without checking a side
condition its result needs. Fail-closed neutral is the shape.
- **S3** (`obs.rs:1015`, index cast): reduce only when every target-index
  template position the rewrite reaches is handled.
- **S4** (`obs.rs:941-970`, parameter cast): reduce only when the rebuilt
  constructor's indices convert to the target's.
- **S6** (`obs.rs:904`, level cast): reduce only when source and target
  `level_args` are `equiv`. A cross-level cast is P2's
  (KERNEL-QUOT-EQ-INTERIM-CAST-LEVEL).
- **S11** (`check.rs:773-814`): `infer_j` accepts a motive whose second
  domain is wrong, and the inferred type does not classify. Check the
  motive's full type against the J motive telescope.

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
   - S3 sits on the same index path. Its gate subsumes this one where
     they coincide; one gate, not two.
3. **S3, S4, S6:** each arm gated on its side condition above.
4. **S11:** `infer_j` checks the motive's full type against the J motive
   telescope.

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
- **AC-2b (folded arms, per arm).** Research's repro for each of S3, S4
  and S6 gives a neutral term where it gave an ill-typed reduct, and the
  well-typed reductions of the same arm are unchanged. Research's S11
  motive is refused, and a well-formed J motive checks. Removing each gate
  reddens its repro row.
- **AC-3.** `trusted_base()` is unchanged, the catalog census has no verdict
  change, and the kernel suite, cast and TYPE-EQ rows stay green.

## Stop conditions

- A cast that fires today on a decomposing equality would be stuck: stop to
  the Architect.
- The inductive path needs more than a gate: stop to the Architect.
- A spec change.
