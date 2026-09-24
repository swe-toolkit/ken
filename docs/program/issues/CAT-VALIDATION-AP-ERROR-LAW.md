---
id: CAT-VALIDATION-AP-ERROR-LAW
title: "Proof-backfill for Data/Sums/Validation.ken.md: prove privately that validation_ap accumulates two errors by the supplied semigroup in order -- for every e, a, b, sg and errors left, right, applying Invalid left to Invalid right gives Invalid (sg.op left right) -- instead of only the concrete NonEmpty String example"
status: active
owner: foundation
size: S
gate: architect
tier: T1
depends_on: []
blocks: []
github: null
origin: "Architect nomination evt_2w12ppsrhs4mx at 78d81cb2a, as the L3 runway successor to CAT-MAP-DOM-MEMBER-LAW (start only after it lands). Operator ruling 2026-09-13: 'schedule the proof backfill before extending the catalog.' Steward-filed per COORDINATION section 2."
---

# Error accumulation in `validation_ap` is unproved

## Settled inputs -- measured at `78d81cb2a`. Re-ground before acting.

- `pub fn validation_ap e sg a b vf vx` returns
  `Invalid e b (sg.op left_error right_error)` when both arguments are
  `Invalid`.
- `both_errors_accumulate` is one concrete `NonEmpty String` example, not a
  law over arbitrary errors and semigroups. The four Applicative class
  proofs are about lawful composition, not this equation.
- `validation_loader_visible_inventory_is_exact_and_transparent` in
  `cc1_nonempty_validation_acceptance.rs` pins the eight-name public set.
  `cc1_packages_have_zero_trusted_base_delta` checks trust. `Semigroup` is
  already a selective import.

## Deliverable

One checked private law in Validation, placed right after `validation_ap`
and before the later Applicative proofs. It holds for every `e`, `a`, `b`,
`sg : Semigroup e` and `left right : e`:

```
Equal (Validation e b)
  (validation_ap e sg a b (Invalid e (a → b) left) (Invalid e a right))
  (Invalid e b (sg.op left right))
```

No new operation, public export, assumption, trust or provider
publication. The only inventory change allowed is adding the new private
proof to the transparent-declaration inventory.

## Acceptance criteria

- **AC-1 (expressibility first).** State the law in Ken before proving it.
  If `Refl` does not close the abstract-`e` endpoint, use only the
  canonical `Transport.cong` already imported.
- **AC-2 (falsifier).** Apply two natural-site one-line mutations of the
  both-`Invalid` arm, one at a time, restoring the file byte-identically
  between them: return `left_error` only, dropping `right_error`; then
  swap the `sg.op` arguments. For each mutation:
  - the mutated `validation_ap` typechecks in isolation, in a scratch
    extraction that keeps the real imports and omits the new law and the
    dependent proofs. The existing `validation_ap_cmp` proof hard-codes
    the original `sg.assoc` endpoints, so the whole package cannot be
    required to check under a mutation;
  - with the unchanged law restored in the full package, the check rejects
    **at the law's own span**, before any older Applicative proof does.

  A `NonEmpty Nat` left and right pair with different heads is the
  no-literal non-vacuity witness.
- **AC-3.** The exact public set and the before/after trust are verified
  independently, and both are unchanged.
- **AC-4.** Targeted builds only, through `scripts/ken-cargo`. No-regression
  means green in CI.

## Stop conditions

- If the law cannot be stated or closed as above, return the exact stuck
  goal to the Steward. Do not add a fact or change `validation_ap`.
- The proof and its controls may not use `checked_record`,
  `name_failure`, `age_failure`, or String or Bytes literal conversion.
  A new operation, publication or catalog extension is not authorized.
