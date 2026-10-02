---
id: KERNEL-REFL-ENDPOINT-TYPED-CONVERSION
title: "Since Refl checks by conversion, its endpoints are compared by the type-agnostic Eq/Eq congruence, so Ω proof irrelevance no longer applies there and a well-typed refl between two proofs of a proposition is rejected (accepted before 884f493fe). Compare the endpoints by typed conversion at the carried type"
status: ready
owner: kernel
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Adversary M8 finding 1 evt_1rnwn0jc09jw9 on 884f493fe (KERNEL-OBS-TYPE-EQ-STRUCTURAL): a regression that fails closed. Reachable from well-formed source. Steward-filed per COORDINATION section 2."
---

# Refl keeps proof irrelevance at its endpoints

## Objective

Spec 15 §2: `refl a : T` iff `T ≡ Eq (infer a) a a`, where that conversion
includes Ω proof irrelevance (spec 16 §1.2, §8.2). A `refl` between two
proofs of the same proposition checks again, and the Σ-eta and Π-eta `refl`
rows that TYPE-EQ-STRUCTURAL enabled stay accepted.

## Settled inputs (Adversary `evt_1rnwn0jc09jw9`, read at `884f493fe`)

- **The site.** The `Refl` arm (`check.rs:483`) checks
  `convert_type(ty, Eq (infer a) a a)`. When neither side reduces, this lands
  in the `Eq`/`Eq` congruence (`conv.rs:1374-1378`). That congruence compares
  the endpoints with the type-agnostic `conv_struct_path_memo`, not typed
  conversion at the carried type. The parent's arm called typed
  `convert(a_ty, a, x)`.
- **Surface repro.** `data Pos : Type where { MkPos : (n : Nat) -> Equal Nat
  n n -> Pos }` with `theorem pos_eq (n : Nat) (a b : Equal Nat n n) : Equal
  Pos (MkPos n a) (MkPos n b) = Refl` is `KernelRejected TypeMismatch` at
  `884f493fe`, and accepted at `5d139f422`.
- **Kernel repros.** `refl x : Eq P x y` with `P : Ω`, and `refl v : Eq (Σ
  n:Nat. P n) v (v.1, k)` with `P : Nat → Ω`, are rejected at head and
  accepted at the parent. `convert(B, w, (w.1, h))` is true at both.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

The endpoints of an `Eq`/`Eq` comparison reached from `Refl` are compared by
typed conversion at the carried type, so the Ω-PI shortcut applies. The
Architect rules at AC-0 whether the fix belongs in the `Refl` arm or in the
`Eq` congruence itself (check 7: every other caller of that congruence).

Also owed, carried from `KERNEL-LEQ-INT-LITERAL-REDUCTION` (CV
`evt_3jb6z15rak6ay`, Steward `evt_4f1evy7egy04z`): one discriminating
kernel-conversion case for spec 17 §1's ζ row (non-recursive `let`) in
`conformance/kernel/conversion/seed-conversion.md`.

## Acceptance

- **AC-1.** The surface repro and both kernel repros are accepted.
- **AC-2 (controls).**
  - The Σ-eta and Π-eta `refl` rows stay accepted.
  - `refl x : Eq Nat x y` with distinct variables is still rejected with
    the ds6a/sec4 `TypeMismatch` payload.
  - Reverting the fix reddens the AC-1 rows.
- **AC-3.** `trusted_base()` is unchanged, the 57-package catalog census has
  no verdict change, and kernel lib, ds6a, sec4 and the TYPE-EQ rows stay
  green.

## Stop conditions

- The fix would equate two non-proof terms: stop to the Architect.
- A spec change.
