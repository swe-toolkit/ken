---
id: CAT-DERIVED-RANGE-NTH-LAW
title: "Derived proves only the length of range, so nothing states that range n holds 0..n-1 in order. Prove range_from_nth and range_nth beside range_length"
status: active
owner: foundation
size: S
tier: T2
gate: architect
depends_on: []
blocks: []
github: null
origin: "L3 proof backfill (operator 2026-09-13: 'schedule the proof backfill before extending the catalog'; Architect evt_5f1ewknxv3m6h). Architect nomination evt_6f5sr5080q6af, from survey row Data/Collections/Derived ('list/range/zip operations lack general structural laws'). Steward-filed per COORDINATION section 2."
---

# Range contents are proved, not only range length

## Objective

`Data/Collections/Derived.ken.md` states and proves that the `i`-th element
of `range n` is `i` for every `i < n`, and the same for `range_from start n`
with `add start i`.

## Settled inputs (Architect `evt_6f5sr5080q6af`, probed on `49eafce90`)

- `zip_length`, `range_length` and `range_from_length` are proved. Range
  contents are not; only the length law and examples bear on them.
- The vocabulary is delivered and already imported: `nth` (`:95`, splits on
  the list), `range_from` (`:1051`, splits on `n`), Arithmetic's `add` (on
  its second argument), `proof suc_l for add`, `proof zero_l for add`,
  `trans`/`cong`, `IsTrue`/`leq_nat`. No import changes.
- The two theorems, in full, are in the nomination. They are inserted after
  `range_length` (`:1074`). The hypothesis form `leq_nat (Suc i) n` matches
  `nth::some_below_length`, and each hypothesis is λ-bound inside its arm
  so the splits refine it.
- The `Zero` arm of `range_from_nth` needs `Refl`, not `Proved`: `Proved`
  is rejected there (`TypeMismatch`).
- On the probe, `ken check` and `ken fmt --check` on Derived exit 0.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

1. `theorem range_from_nth` and `theorem range_nth`, private, after
   `range_length`.
2. A test pins `range_nth`'s checked proposition (the decoded Π and
   `Equal` endpoints, and the `nth` and `range` `GlobalId`s), as in the
   earlier Derived law WPs, not only its name.

## Acceptance

- **AC-1.** Derived's `ken check` and `ken fmt --check` pass. Catalog card
  verdict parity holds against the baseline, and `trusted_base()` has no
  delta.
- **AC-2 (falsifiers; each must redden).**
  - M1: `range_from`'s `Suc` arm head `start` becomes `Suc start`.
    `range_from_length` still checks, and the first rejection is inside
    `range_from_nth` (its `Refl`).
  - M2 (`evt_2zbp71629qfqs`): `range_nth`'s statement and body become the
    reflexive `Equal (Option Nat) (nth Nat i (range n)) (nth Nat i (range
    n)) = Refl`, keeping the name and the hypothesis binder. Derived still
    checks, and only the proposition pin reddens.

## Stop conditions

- The theorems need an import, a new lemma outside Derived, or a postulate:
  stop to the Architect.
- M1's first rejection is the length law, not `range_from_nth`.
