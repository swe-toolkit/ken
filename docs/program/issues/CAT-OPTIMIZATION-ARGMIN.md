---
id: CAT-OPTIMIZATION-ARGMIN
title: "Spec 72 section 1 specifies a least-cost candidate selector over NonEmpty with seven proved laws, but no catalog package delivers it. Deliver Algorithm.Optimization.Argmin from the Architect's checked D0 at zero trust, and publish the two Nat-order bridges it needs in Data.Numeric.Nat.Order"
status: merged
owner: foundation
size: S
tier: T2
gate: architect
depends_on: [SPEC-OPTIMIZATION-SEARCH-CONTRACT]
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Spec 72 section 1 from SPEC-OPTIMIZATION-SEARCH-CONTRACT (Architect D0 evt_3gxyd2h6cn345, review evt_35g5j080tc5nh). Same shape as CAT-SERIALIZATION-JSON-TOKEN-CODEC. Steward-filed per COORDINATION section 2."
---

# The least-cost selector, delivered and proved

## Objective

`Algorithm.Optimization.Argmin` exists in `catalog/`, matches spec 72
section 1, and proves its seven laws with no Axiom. A client outside the
module can call `argmin` and use every law.

## Settled inputs (on the SPEC-OPTIMIZATION landing)

- **The contract.** Spec 72 section 1 fixes the module path
  `catalog/packages/Algorithm/Optimization/Argmin.ken.md`. The operation
  is `argmin (a : Type) (cost : a → Nat) (xs : NonEmpty a) : a`. It
  returns the earliest minimum: a tail candidate replaces the head only
  on strict `lt_nat`. The seven laws are `argmin_member`,
  `argmin_least`, `argmin_no_cheaper`, `argmin_head_on_tie`,
  `argmin_singleton`, `argmin_keeps_head` and `argmin_takes_tail`.
- **The D0 development is persisted.** It is on `architect/work` at
  `notes/argmin-d0-development.ken.md`. It is 216 lines, with no Axiom,
  postulate or primitive.
- **Proof constraints (section 1.3).**
  - Membership uses the canonical `NonEmpty` list-view shape equality.
  - `list_elem` is eliminated only into an Ω-valued proposition.
  - The Boolean branch binds the comparison together with its defining
    equality (CHECKS 11).
- **The Nat-order bridges.** Section 1.3 places two generic bridges in
  `Data.Numeric.Nat.Order`: `lt_nat m n = True` implies `leq_nat m n`,
  and `leq_nat m n` implies `lt_nat n m = False`. The D0 proved them as
  private helpers. Publish them there and import them, rather than
  keeping duplicate private copies.
- **Carry from the Architect's review** (`evt_35g5j080tc5nh`): the module
  is lede first.
- **The seed.** `conformance/stdlib/optimization/seed-argmin.md` checks
  against the D0 module: the positives give rc 0, and each refused claim
  gives rc 1 at its own `seed_neg`.
- **Coupled test.** `catalog_ambient_passthrough_migration_census` in
  `lang_mod_strict_resolution_d0.rs` needs the new package's measured row.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

1. The package at `catalog/packages/Algorithm/Optimization/Argmin.ken.md`,
   with the D0's declarations and proof bodies and the seven laws as
   public theorems, lede first.
2. The two bridges as public proofs in `Data.Numeric.Nat.Order`, with
   their export inventory updated.
3. An acceptance target in `crates/ken-elaborator/tests/` that:
   - loads the package;
   - runs the seed from a client import, refusing each negative at its
     own theorem;
   - restates the seven law types generically;
   - records `trusted_base()` before and after loading.
4. The package's measured row in the ambient census.

## Acceptance

- **AC-1.** The package checks, every law is a checked proof body, and
  `trusted_base()` does not grow on loading it.
- **AC-2.** The seed's positives pass from a client module, and each
  negative is refused at its own theorem. The census passes with the new
  row, and every other row is unchanged.
- **AC-3 (mutation, QA).** Apply each mutant to the production `argmin`.
  The package then fails to check. Because that rejection comes from a
  law proof and is not attributable, also run the seed's positives from a
  client against a scratch operation-only projection: the candidate's
  production `fn` bodies verbatim (identity checked against the
  candidate), proofs excluded, not committed. The named seed row must
  redden:
  - selecting a later tie reddens the earliest-tie case;
  - reversing the cost order reddens the strict-minimum case;
  - always keeping the head reddens the strict-minimum case.

## Stop conditions

- A law needs an Axiom.
- `trusted_base()` grows on loading the package.
- A new public name collides with a consumer's declaration. CHECKS 3
  applies: grep every source root for each new public name before the
  handoff.
- A change outside the new package, `Data.Numeric.Nat.Order` and its
  export inventory test, the census test and the new acceptance target.
