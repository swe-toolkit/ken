---
id: KERNEL-J-NONREFL-FORMATION-FALLBACK
title: "Since KERNEL-J-NONREFL-ENDPOINT-SHARING, a J over evidence at a one-parameter or one-index former reduces at the formation endpoints while infer_j types it at the whnf endpoints, so a well-typed check the kernel accepted before is refused. Use the formation read only where the guard types the motive at it, and otherwise reduce at exactly infer_j's endpoints"
status: active
owner: kernel
size: S
tier: T1
gate: architect
depends_on: [KERNEL-J-NONREFL-ENDPOINT-SHARING]
blocks: []
github: null
origin: "Adversary finding evt_50c6a892xebf on f973876aa, confirmed and extended to the Omega path by the Architect, who ruled a forward repair (evt_4v6x90s993kz9). A regression on main. Steward-filed per COORDINATION section 2."
---

# J reduces wherever infer types it

## Objective

Every J that `infer` types reduces as it did on `c0c49b874`. The J-NONREFL
linearity pins and the ruled conversion widening are unchanged.

## Settled inputs (Architect `evt_4v6x90s993kz9`, on `924e96472`)

- **The false premise.** Eq-at-Type can return an `Eq` formation. A
  one-parameter or one-index former's single conjunct comes back bare
  (`obs.rs:756`, `inductive_conjuncts`'s `j + 1 == a_tpl.len()` arm). So
  `whnf(Eq Type0 (B X) (B Y))` is `Eq Type0 X Y`. `infer_j` types J at
  `X, Y`, while `j_nonrefl`'s formation read uses `B X, B Y`.
- **Rows**, with `B : Type0 → Type0` and `e : Eq Type0 (B X) (B Y)`, or on
  the Ω path `P : Ω0`, `C : P → Type0` and `e : Eq Type0 (C q) (C q')`,
  where J = `J P zero e` and the check is `Refl zero : Eq Nat J zero`:
  - A (motive at `X`) and Ω-A (motive at `q`): infer Ok, but whnf is stuck
    and the check fails on main. Both pass on `c0c49b874`.
  - B (motive at `B X`) and Ω-B (motive at `C q`): infer is the named
    BadEliminator, whnf is `zero`, and the check is Ok. This is the ruled
    widening of `evt_5f79a35xp026p`, and it stays.
- **The repair is ruled.** In `j_nonrefl` only, replace the block from
  `let eq_ty = …` through the guard line with the ruling's code. The
  formation endpoints are used only if `infer_j_at` succeeds at them;
  otherwise `j_endpoints` plus `infer_j_at` runs at exactly `infer_j`'s
  endpoints. Everything after, including `typed_by_lemma = true`, is
  unchanged. The patch is at `/workspaces/ken/tmp/arch-jone-fallback.patch`,
  and the row builder is at `/workspaces/ken/tmp/arch-jone-rows.rs`.
- **Measured with the repair:** A and Ω-A are restored, B and Ω-B are
  unchanged, and `ken-kernel` passes 440/0 across 43 targets.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

The ruled fallback in `crates/ken-kernel/src/obs.rs`, and one committed test
with the four rows.

## Acceptance

- **AC-1.** A and Ω-A: infer Ok, whnf `zero`, check Ok. B and Ω-B: infer
  the named BadEliminator, whnf `zero`, check Ok. B and Ω-B are pinned as the
  ruled widening, so a later `infer_j` change moves them knowingly.
- **AC-2 (falsifier).** Without the fallback, as on main, A and Ω-A are red.
- **AC-3 (controls).** Full `ken-kernel` is green, with the J-NONREFL
  linearity pins (`k2_j_nonrefl_*` and the conv linear-entry pins)
  unchanged, and `trusted_base()` equal.

## Stop conditions

- Any change to `infer_j` or `j_endpoints`. Sharing one endpoint function
  between typing and reduction widens `infer` and is not this WP.
- Any change to the linearity pins' bounds.
- Any spec change.
