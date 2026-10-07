---
id: CAT-NAT-ORDER-COMPARE-COMPLETENESS-LAWS
title: "Proof-backfill for Data/Numeric/Nat/Order.ken.md: compare's laws only say what each result implies, so no law fixes compare n n to Eq or a strictly smaller left side to Lt. Prove privately that compare is reflexive at Eq, returns Lt under lt_nat, and flips Lt to Gt"
status: active
owner: foundation
size: S
tier: T2
gate: architect
depends_on: [CAT-NAT-ORDER-MIN-MAX-BOUND-LAWS]
blocks: []
github: null
origin: "Survey row Data/Numeric/Nat/Order (docs/program/CATALOG-PROOF-COMPLETENESS-SURVEY.md, CAT-NAT-ORDER-LAWS): the compare algebra is incomplete. Operator 2026-09-13: a catalog package is not finished until its proofs are complete. Architect evt_5f1ewknxv3m6h: L3 turns to proof backfill. Steward-filed per COORDINATION section 2."
---

# Nat compare is complete, not only sound

## Objective

Nat Order proves which result `compare` returns, so a `compare` that answers
`Lt` for equal arguments no longer satisfies every law.

## Settled inputs (measured at `e75479460`)

- `compare a b` in `catalog/packages/Data/Numeric/Nat/Order.ken.md` matches
  `leq_nat a b`, then `leq_nat b a`: `Eq` when both hold, `Lt` when only the
  first does, `Gt` otherwise. `lt_nat` is structural on both arguments.
- `compare` has three result-to-order laws: `lt_implies_leq`,
  `eq_implies_equal` and `gt_implies_reverse_leq`. No law states
  `compare n n`, nor the result for `lt_nat a b = True`, nor the relation
  between `compare a b` and `compare b a`.
- `sub n n = Zero` is already proved (`self_is_zero for sub`), so the
  survey's self-subtraction item is closed.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch; do not build around it.

## Deliverable

Three private checked proofs in Nat Order:

```
proof self_eq for compare (n : Nat) : Equal OrdResult (compare n n) Eq
proof from_lt for compare (a : Nat) (b : Nat)
  : Equal Bool (lt_nat a b) True → Equal OrdResult (compare a b) Lt
proof flip_lt for compare (a : Nat) (b : Nat)
  : Equal OrdResult (compare a b) Lt → Equal OrdResult (compare b a) Gt
```

No new import, export, operation, axiom or trust. Follow
`docs/program/07-catalog-style-guide.md` and
`agent/playbooks/tools/write-ken.md`.

## Acceptance

- **AC-1.** All three proofs check, and their stated types are the literal
  ones above (state them before proving).
- **AC-2 (proposition pin).** A test decodes each private declaration's
  checked type and asserts its binders, premise and conclusion by global
  identity and de Bruijn index. Control: weakening any conclusion to an
  owner-preserving trivial claim, such as `Equal OrdResult (compare n n)
  (compare n n)` proved by `Refl`, keeps the package loading and reddens
  that law's pin. A claim that does not mention `compare` is refused by the
  attached-proof rule (`evt_7q023c2bn93hm`).
- **AC-3 (mutation, QA).** Mutate a scratch copy, not `compare` in
  place: existing proof bodies compute on `compare`'s arms, so an in-place
  mutation is refused before the new laws are reached. In a scratch
  package that imports Nat Order, define `compareA` returning `Lt` in the
  `Eq` arm and `compareB` returning `Eq` in the `Lt` arm. Restate
  `self_eq` over `compareA` and `from_lt` over `compareB`: each is refused
  at its own span. Restate `lt_implies_leq` over `compareA` and record
  whether it checks. Delete the scratch package afterwards.
- **AC-4.** The loaded closure's trust ledger and Nat Order's exports are
  unchanged. Targeted builds only, through `scripts/ken-cargo`.
  No-regression means green in CI.

## Stop conditions

- Any trust delta, `Axiom`, kernel, prelude or spec change, or a needed
  change to `compare`, `lt_nat` or `leq_nat`.
