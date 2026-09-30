---
id: KERNEL-OBS-EQ-AT-TYPE-RIGID-BOTTOM
title: "P0 soundness: the kernel reduces Eq Type (A → B) (A → B), and any compound type against a neutral type, to Bottom, so a generic refl lemma instantiated at a Π or Σ type checks against Bottom and proves anything. Reduce Eq Type to Bottom only between two rigid, distinct type formers"
status: ready
owner: kernel
size: S
tier: T1
gate: architect
depends_on: []
blocks: [KERNEL-OBS-REDUCT-WITNESS-TYPING]
github: null
origin: "Architect finding evt_229qe9tgfetw1, reproduced at aa51bf9d7 while ruling KERNEL-OBS-REDUCT-WITNESS-TYPING AC-0. Kernel soundness hole: P0, ahead of all kernel work. Steward-filed per COORDINATION section 2."
---

# Eq Type reaches Bottom only between distinct rigid formers

## Objective

`Eq (Type l) A B` reduces to `Bottom` only when both whnf'd sides are rigid
type formers that differ. Π/Π, Σ/Σ, D/D with the same inductive id, and any
pair with a neutral side stay neutral. No closed term checks against
`Bottom`.

## Settled inputs (Architect `evt_229qe9tgfetw1`, read at `aa51bf9d7`)

- **The arm.** In `obs.rs::eq_at_type` (`:195-222`), the arm
  `(Pi|Sigma|Omega|Type, _) | (_, Pi|Sigma|Omega|Type) => Bottom` follows
  the equal-level Type/Type and Omega/Omega arms. So it also catches Π/Π,
  Σ/Σ and a compound type against a neutral type. The doc comment at
  `:189-194` says those cases stay neutral; they do not.
- **The reach.** `conv.rs:335` sends every whnf of `Eq (Type l) _ _`
  through `eq_reduce`, so every conversion sees the arm.
- **The exploit**, in `GlobalEnv::new()` with an empty context:
  - `lemma = λ(T:Type 2). λ(x:T). refl x` checks at
    `Π(T:Type 2). Π(x:T). Eq T x x`.
  - `p = lemma (Type 1) (Type 0 → Type 0)` infers
    `Eq (Type 1) (Π Type0. Type0) (Π Type0. Type0)`, which whnf's to
    `Bottom`.
  - `check(p, Bottom) = Ok`, and likewise with Σ. With `absurd`, that is a
    closed proof of anything.
- **The neutral case.** `Eq Type (A→B) X` reduces to `Bottom` for a
  variable `X`, which is false at `X := A→B`.
- `check`'s whnf-first `Refl` rule (`check.rs:483`) blocks a direct `refl`.
  "Refl is rejected" is not evidence that the hole is closed.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

`eq_at_type` returns `Bottom` only for two rigid heads with different formers
or different inductive ids. The rigid heads are Π, Σ, Ω, Type, an
inductive-headed application, `Quot` and `Trunc`. Every other pair is `None`
(neutral).

## Acceptance

- **AC-1 (fences).** Each of these fails `check` against its `Bottom` type:
  - the Π probe `p` above;
  - the Σ twin of `p`;
  - `λ(X:Type 1)(h : Eq (Type 1) (Type 0→Type 0) X). h` against
    `Π X. Π h. Bottom`.
- **AC-2 (controls).**
  - `Eq (Type 1) (Type 0) (Type 0 → Type 0)` still reduces to `Bottom`,
    since the heads are distinct and rigid.
  - Type/Type and Omega/Omega at equal and at unequal levels keep their
    verdicts.
  - Restoring the old arm reddens each AC-1 fence.
- **AC-3 (sweep).**
  - Every other reduction to `Bottom` in `obs.rs` decides only between two
    distinct rigid heads. Distinct constructors in `eq_at_inductive` and
    distinct literals qualify.
  - The level comparisons are checked the same way.
  - The handoff lists each site and its verdict.
- **AC-4.** The kernel lib and the targeted obs and conv suites stay green.

## Stop conditions

- A checked program in the catalog or tests that relied on the unsound
  `Bottom`: stop to the Architect with the site.
- Any change beyond narrowing what reduces to `Bottom`: stop to the
  Architect.
