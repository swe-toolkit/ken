---
id: CAT-NAT-ORDER-MIN-MAX-BOUND-LAWS
title: "Proof-backfill for Data/Numeric/Nat/Order.ken.md: min and max are proved to be bounds but not the tightest ones, so a min that always returns Zero satisfies every law. Prove privately that min is the greatest lower bound and max the least upper bound under leq_nat"
status: active
owner: foundation
size: S
tier: T2
gate: architect
depends_on: []
blocks: []
github: null
origin: "Survey row Data/Numeric/Nat/Order (docs/program/CATALOG-PROOF-COMPLETENESS-SURVEY.md, CAT-NAT-ORDER-LAWS): min/max/sub/compare algebra is absent. Operator 2026-09-13: a catalog package is not finished until its proofs are complete. Architect evt_5f1ewknxv3m6h: L3 turns to proof backfill. Steward-filed per COORDINATION section 2."
---

# `min` and `max` are bounds, not the tightest ones

## Objective

Nat Order proves that `min` and `max` compute the meet and join of
`leq_nat`, not only a lower and an upper bound.

## Settled inputs (measured at `e1bbed2d0`)

- `min`, `max` and `sub` in `catalog/packages/Data/Numeric/Nat/Order.ken.md`
  are transparent and structural on both arguments. `leq_nat` (imported
  from `Core.Classes.LawfulClasses`) is `True` on `Zero`, `False` on
  `Suc _` against `Zero`, and recurses on `Suc`/`Suc`. `IsTrue b` is
  `Equal Bool b True`.
- The `min` laws are `zero_left`, `leq_left` and `leq_right`. The `max`
  laws are `zero_left`, `left_leq` and `right_leq`. A `min` that returns
  `Zero` in every arm still satisfies all three `min` laws as written.
- `sub` already has `self_is_zero`, `saturates`, `suc_decreases` and
  `add_cancel`. `compare` has its three result-to-order laws.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch; do not build around it.

## Deliverable

Two private checked proofs in Nat Order, over every `k`, `m`, `n`:

```
proof greatest for min (k : Nat) (m : Nat) (n : Nat)
  : IsTrue (leq_nat k m) → IsTrue (leq_nat k n) → IsTrue (leq_nat k (min m n))
proof least for max (k : Nat) (m : Nat) (n : Nat)
  : IsTrue (leq_nat m k) → IsTrue (leq_nat n k) → IsTrue (leq_nat (max m n) k)
```

No new import, export, operation, axiom or trust. Follow
`docs/program/07-catalog-style-guide.md` and
`agent/playbooks/tools/write-ken.md`.

## Acceptance

- **AC-1.** Both proofs check, and their stated types are the literal ones
  above (state them before proving).
- **AC-2 (proposition pin).** A test decodes each private declaration's
  checked type and asserts its binders, both premises and the conclusion by
  global identity and de Bruijn index. Control: weakening either conclusion
  to an owner-preserving trivial claim (`IsTrue (leq_nat Zero (min m n))`,
  `IsTrue (leq_nat Zero (max m n))`, body `λhm. λhn. Proved`) keeps the
  package loading and reddens that law's pin. The attached-proof rule
  refuses a claim that does not mention its subject (`evt_7q023c2bn93hm`).
- **AC-3 (mutation, QA).** A scratch `min` returning `Zero` in every arm
  keeps `leq_left` and `leq_right` checking and reddens `greatest` at its
  span. A scratch `max` that returns a larger upper bound in the
  `Suc`/`Suc` arm reddens `least` at its span. Restore byte-identically.
- **AC-4.** The loaded closure's trust ledger and Nat Order's exports are
  unchanged. Targeted builds only, through `scripts/ken-cargo`.
  No-regression means green in CI.

## Stop conditions

- Any trust delta, `Axiom`, kernel, prelude or spec change, or a needed
  change to `min`, `max` or `leq_nat`.
