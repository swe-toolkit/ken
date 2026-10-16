---
id: CAT-OPTIMIZATION-BOUNDED-SEARCH
title: "Spec 72 section 2 specifies find_below, the least Nat below a bound that satisfies a Boolean predicate, with three proved laws, but no catalog package delivers it. Deliver Algorithm.Optimization.BoundedSearch from the Architect's checked D0 at zero trust"
status: merged
owner: foundation
size: S
tier: T2
gate: architect
depends_on: [SPEC-OPTIMIZATION-BOUNDED-SEARCH]
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Spec 72 section 2 from SPEC-OPTIMIZATION-BOUNDED-SEARCH (Architect D0 evt_5mbgykn93snzc, cost correction evt_1sexdt34d5vet). Same shape as CAT-OPTIMIZATION-ARGMIN. Steward-filed per COORDINATION section 2."
---

# The bounded least-witness search, delivered and proved

## Objective

`Algorithm.Optimization.BoundedSearch` exists in `catalog/`, matches spec
72 section 2, and proves its three laws with no Axiom. A client outside
the module can call `find_below` and use every law.

## Settled inputs (on `8e9e2c2c7`)

- **The contract.** Spec 72 section 2 fixes the module path
  `catalog/packages/Algorithm/Optimization/BoundedSearch.ken.md` and the
  operation `find_below (p : Nat → Bool) (bound : Nat) : Option Nat`.
  The three laws are `find_below_sound`, `find_below_least` and
  `find_below_complete`, with the exact types in section 2.2. The
  contract is extensional: no search direction, call count or cost.
- **The D0 development is persisted.** It is on `architect/work` at
  `notes/bsearch-d0-development.ken.md`. It is 167 lines, with no Axiom,
  postulate, primitive or opaque declaration. It imports only
  `Core.Logic.Transport (cong, trans)` and
  `Data.Numeric.Nat.Order (lt_nat)`.
- **Private helpers stay private.** The outcome invariant (`find_next`,
  `found_at`, `none_below`, `found_spec`, their correctness theorems)
  and the Nat fact `lt_suc_not_lt_same` are not public laws (section
  2.3). The module exports exactly the operation and the three laws.
- **Proof constraint.** The `Bool` branch splits on a separately bound
  variable, not a dependent match on a computed `p b` (CHECKS 11).
- **The seed.** `conformance/stdlib/optimization/seed-bounded-search.md`
  imports the module and its laws. The CV checked its positive block
  (rc 0) and each refused claim alone (rc 1, K0004 TypeMismatch) against
  a scratch module made from the D0. Its refused claims are
  `reject_later_witness`, `reject_none_when_witness_exists` and
  `reject_non_hit`.
- **Coupled test.** `catalog_ambient_passthrough_migration_census` in
  `lang_mod_strict_resolution_d0.rs` needs the new package's measured
  row, as Argmin's did.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

1. The package at
   `catalog/packages/Algorithm/Optimization/BoundedSearch.ken.md`, lede
   first, with the D0's declarations and proof bodies and the three laws
   as public theorems.
2. An acceptance target in `crates/ken-elaborator/tests/` that:
   - loads the package;
   - runs the seed from a client import, refusing each negative at its
     own theorem;
   - restates the three law types generically;
   - records `trusted_base()` before and after loading.
3. The package's measured row in the ambient census.

## Acceptance

- **AC-1.** The package checks, every law is a checked proof body, and
  `trusted_base()` does not grow on loading it.
- **AC-2.** The seed's positives pass from a client module, and each
  negative is refused at its own theorem. The census passes with the new
  row, and every other row is unchanged.
- **AC-3 (mutation, QA).** Apply each mutant to the production
  `find_below`. The package then fails to check. Because that rejection
  comes from a law proof and is not attributable, also run the seed's
  positives from a client against a scratch operation-only projection:
  the candidate's production `fn` bodies verbatim (identity checked
  against the candidate), proofs excluded, not committed. The named seed
  row must redden:
  - preferring a later witness reddens `found_least_witness`;
  - never reporting a witness reddens `found_least_witness`;
  - including the bound reddens `no_witness_below_three`.

## Stop conditions

- A law needs an Axiom.
- `trusted_base()` grows on loading the package.
- A new public name collides with a consumer's declaration. CHECKS 3
  applies: grep every source root for each new public name before the
  handoff.
- A change outside the new package, the census test and the new
  acceptance target.
