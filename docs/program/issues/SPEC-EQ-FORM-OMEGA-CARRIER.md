---
id: SPEC-EQ-FORM-OMEGA-CARRIER
title: "Spec 16 forms Eq only at a Type carrier, so equality between two proofs of a proposition is ill-formed and the Trunc reduct to Top breaks subject reduction once it is reachable. Admit an Omega-classified carrier, make Eq there neutral at every head, and discard Omega components in the Sigma and inductive rules"
status: active
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: [KERNEL-EQ-OMEGA-CARRIER-REDUCTION]
github: null
origin: "Operator 2026-10-02: Eq-Form direction (a) ('approve 1 and 2 as recommended', question evt_1mj6gmyd2d56b); the kernel change 'provisionally approved, given clean or mitigatable research findings'. Research evt_782fh9yxr4xk8: clean or mitigatable. Architect ruling evt_7tnycbxzgp75x as amended by evt_sbv31qypx3j7 (all Omega-carrier heads neutral). Steward-filed per COORDINATION section 2."
---

# Eq at an Ω carrier is well-formed and neutral

## Objective

Spec 16 states that `Eq P u v : Ω_l` is well-formed for `P : Ω_l`, does not
reduce, and is inhabited by `refl`. Its Σ and inductive rules discard an
Ω-classified component instead of transporting it. Conformance carries the
five rows below.

## Fixed inputs (Architect `evt_7tnycbxzgp75x`, amended `evt_sbv31qypx3j7`, `evt_5hryk5pap4q78` and `evt_6s74556468q9y`, measured at `948864d3e`)

- **R1 (amended).** Eq at an Ω-classified carrier is neutral at every head,
  Π and Σ included. `refl u : Eq P u v` checks because `u ≡ v` at `P : Ω`
  (§1.2, §8.2). The Trunc→Top reduct is deleted, because `Top` exists only
  at `Ω_0` and Ken is non-cumulative.
- **R2.** For a Type-classified Σ whose codomain `B1` is Ω-classified (a
  subset): `Eq ((x:A1)×B1) p q ⇝ Eq A1 p.1 q.1 ∧ Eq (B1 q.1) q.2 q.2`, with
  no `cast` and no `cong`.
- **R3.** For a same-constructor inductive, a field `j` whose target-side
  type is Ω-classified has the conjunct `Eq (A_j[b̄]) b_j b_j`, with no
  `cast` and no J witness, at the field's own level with no lift. Later
  dependent fields keep the prefix transport.
- **Unchanged.** Cast admission stays Type-only. The Type-carrier Σ rule
  that J's singleton uses is untouched. The cast-side Ω siblings stay stuck.
- **Prior art** (Research `evt_782fh9yxr4xk8`). The result is OTT's
  treatment and gives definitional UIP. Normalization hazards need an
  impredicative Ω, and Ken's Ω is predicative (12 §2).

Treat anchors as perishable. If a fixed input is false on the landed base,
stop and report the mismatch; do not build around it.

## Deliverable

1. **Spec 16 text** (`spec/10-kernel/16-observational.md`).
   - §2.1: Eq-Form admits an Ω-classified carrier at its own level.
   - §2.2: "Eq at an Ω-classified carrier does not reduce; `refl u : Eq P u
     v` checks because `u ≡ v` at `P : Ω` (§8.2)." Add R2 and R3, and
     delete the Trunc→Top reduct.
   - §2.3: the UIP line `Eq (Eq A a b) e₁ e₂` is well-formed and neutral.
   - §8.4: the R1 case is "neutral, level of the carrier", plus the
     Architect's R2 and R3 sentence (`evt_5hryk5pap4q78`). The known-gap
     note is the class text in `evt_6s74556468q9y`: every `eq_reduce` arm
     whose reduct is built from `Top`/`Bottom : Ω_0` or from component
     propositions at their own level, propext included. Do not change
     propext and do not add a lift.
2. **Conformance** (Architect's revised list). Seed these in the corpus's
   form for a case the kernel does not yet satisfy; the kernel WP turns
   them green:
   1. Subset Σ: `(h, refl q) : Eq ((x:Nat)×P x) (a,p) (b,q)` checks, and
      the reduct's sort equals the Eq's sort.
   2. An inductive at `Type 0` with an Ω field and a later dependent
      `Type 0` field, with two syntactically distinct Ω-field proofs: the
      same-constructor reduct checks, and its sort equals the Eq's sort.
   3. Trunc at `A : Type 1`: `Eq ‖A‖ u v` is neutral, `refl u` checks, and
      the level is `Ω_1`.
   4. Conjunction and Π-into-Ω carriers: `whnf(Eq (P∧Q) u v)` and
      `whnf(Eq ((x:A)→Q) f g)` are `Eq`, `refl u` and `refl f` check, and
      the level equals the carrier's.
   5. Rejections: Cast between two props and Eq-Form at a non-type are
      still refused.

## Acceptance

- **AC-1.** The Architect confirms the text states R1 (amended), R2, R3 and
  the §8.4 case and known-gap note as ruled, with no rule for the
  cast-side siblings and no level lift.
- **AC-2.** The conformance validator votes on the five rows. Each row is
  a non-degenerate pair against current main.

## Stop conditions

- The text needs a rule the ruling does not give, such as a cast-side Ω
  transport: stop to the Architect.
- A row cannot be stated so that main and the ruled kernel differ on it.
