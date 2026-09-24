---
id: CAT-VEC-MAP-IDENTITY-LAW
title: "Proof-backfill for Data/Vector/Vector.ken.md: prove privately that map with the identity function returns its input -- for every a, n and xs : Vec a n, map a a n (λx. x) xs = xs -- with a test that pins the law's checked proposition, not only its name and trust"
status: ready
owner: foundation
size: S
gate: architect
tier: T1
depends_on: [CAT-VALIDATION-AP-ERROR-LAW]
blocks: []
github: null
origin: "Architect nomination evt_a7r2vmhr545 at 1a4495376, as the L3 runway successor to CAT-VALIDATION-AP-ERROR-LAW (start only after it lands). Operator ruling 2026-09-13: 'schedule the proof backfill before extending the catalog.' Steward-filed per COORDINATION section 2."
---

# `map` is unproved against the identity function

## Settled inputs -- measured at `1a4495376`. Re-ground before acting.

- `fn map (a b : Type) (n : Nat) (f : a → b) (xs : Vec a n) : Vec b n` in
  `catalog/packages/Data/Vector/Vector.ken.md` is transparent and
  structurally recursive: `VNil ↦ VNil b`, and
  `VCons m x tail_xs ↦ VCons b m (f x) (map a b m f tail_xs)`.
- `map_vnil` is proved. No generic identity law exists.
- The Map post-merge hunt (`evt_35kdyheqftzve`) showed that a test checking
  only a private law's name, declaration kind and trust passes when the
  statement is replaced by a trivial one. This frame pins the proposition.

## Deliverable

One checked private law in Vector, over every `a`, `n` and `xs`:

```
theorem vec_map_identity (a : Type) (n : Nat) (xs : Vec a n)
  : Equal (Vec a n) (map a a n (λx. x) xs) xs
```

Induct on `xs`. `VNil` collapses to `Top` (`Proved`). `VCons m x tail`
lifts the recursive proof under `(VCons a m x)` with the existing checked
`Core.Logic.Transport.cong`. No new public constructor, operation, instance,
axiom, primitive or provider law. Vector's public exports are unchanged.

## Acceptance criteria

- **AC-1 (expressibility first).** State the proposition in Ken inside
  Vector before proving it.
- **AC-2 (falsifier).** In a scratch copy, change only `map`'s `VCons`
  recursive tail to `map a b m (λ_. f x) tail_xs`, keeping
  `VCons b m (f x) …`. `map_vnil` still checks, and the new law rejects **at
  its own span** on an arbitrary two-element vector. Restore byte-identically.
- **AC-3 (proposition pin).** A test decodes the private declaration's raw
  checked type and asserts three binders `a`, `n`, `xs` and the exact
  `Equal (Vec a n)` endpoints `map a a n (λx.x) xs` and `xs`, by global
  identities and de Bruijn indices. Do not compare by definitional equality:
  the endpoints reduce to each other, so a reflexive replacement would pass.
  Control: replace the statement with `Equal (Vec a n) xs xs = Refl`, keeping
  production `map`; roots loading stays green and the pin reddens.
- **AC-4.** No new trust: check the provider closure's actual trust ledger
  before and after. Any exact public-export census is labelled a transition
  sentinel. Targeted builds only, through `scripts/ken-cargo`. No-regression
  means green in CI.

## Stop conditions

- If older checks fail first under the AC-2 mutation, or the index
  refinement blocks the proof, STOP and return the exact goal for rescope.
  That red is not proof of the new law.
- No String or Bytes literal use, new operation, publication or catalog
  extension.
