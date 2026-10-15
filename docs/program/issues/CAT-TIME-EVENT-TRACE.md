---
id: CAT-TIME-EVENT-TRACE
title: "Spec 69 section 1 specifies Data.Time.EventTrace with three proved laws, but no catalog package delivers it. Deliver the package from the Architect's checked D0 at zero trust, factor its generic list proofs into Data.Collections.Derived, and run the seed as an acceptance target"
status: ready
owner: foundation
size: S
tier: T2
gate: architect
depends_on: [SPEC-TIME-EVENT-TRACE-CONTRACT]
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Spec 69 section 1 from SPEC-TIME-EVENT-TRACE-CONTRACT (Architect D0 evt_42rzxmvjwgb6d, review evt_2g55q7sr511zx). Same shape as CAT-NUMERIC-RATIONAL-INTERVAL. Steward-filed per COORDINATION section 2."
---

# Finite event traces, delivered and proved

## Objective

`Data.Time.EventTrace` exists in `catalog/`, matches spec 69 section 1
token for token, and proves its three laws with no Axiom. A client
outside the module can build events and use every operation.

## Settled inputs (on the SPEC-TIME landing)

- **The contract.** Spec 69 section 1 fixes:
  - the carrier `Event p` (`MkEvent` of a `Nat` time and a payload, with
    `export MkEvent`), `event_time`, `event_payload`, `event_before` and
    `time_ordered`, which is `is_sorted` under `event_before`, so
    non-decreasing (section 1.1);
  - `event_in_window`, `trace_window` (half-open, lo <= t < hi) and
    `trace_merge` (linear, ties take the left event) (section 1.2);
  - the laws `trace_window_ordered`, `trace_merge_ordered` and
    `trace_merge_count` (section 1.3).
- **A checked development exists.** The Architect's D0 module,
  `notes/trace-d0-development.ken` on `architect/work` (771 lines), checks
  all three laws generically, rc=0, no Axiom, on `48cda520b`.
- **The D0's factoring carry** (`evt_42rzxmvjwgb6d`, spec 69 section
  1.4):
  - Move the D0's generic list proofs (`filter_sorted`, `count_cons`,
    and the sorted cons, tail and head lemmas) into
    `Data.Collections.Derived`.
  - Reuse its existing private `derived_sort_sorted_cons`, `_tail` and
    `_head` (Derived.ken.md:1482-1509) rather than restating them.
  - The trace module itself is lede first.
- **The seed** is `conformance/stdlib/time-and-events/seed-event-trace.md`.
  Its positive block checks rc=0 against the D0 module, and the appended
  refusal of an out-of-order merge result is a kernel TypeMismatch at its
  own theorem (Architect `evt_2g55q7sr511zx`).
- **Coupled test.** `catalog_ambient_passthrough_migration_census` in
  `lang_mod_strict_resolution_d0.rs` needs the new package's measured row.
- **Known limits.**
  - Pass `map` a named helper, not a λ.
  - A dependent `match` refines only its scrutinee as written (CHECKS
    11).
  - Seed times are `Suc` chains, not numerals.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

1. The package at `catalog/packages/Data/Time/EventTrace.ken.md`, with:
   - the D0's declarations and proof bodies;
   - `export MkEvent`;
   - the three laws as public theorems, lede first.
2. The generic list proofs moved into `Data.Collections.Derived`, reusing
   its sorted-head helpers. Every existing Derived export stays
   unchanged.
3. An acceptance target in `crates/ken-elaborator/tests/` that:
   - loads the package;
   - runs the seed's positive block as a client import, and refuses its
     out-of-order merge case;
   - restates the three law types generically;
   - records `trusted_base()` before and after loading.
4. The package's measured row in the ambient census.

## Acceptance

- **AC-1.** The package and the extended Derived check, every law is a
  checked proof body, and `trusted_base()` does not grow on loading
  either.
- **AC-2.** The seed's positives pass from a client module, and the
  out-of-order merge result is refused at its own theorem. The census
  passes with the new row, and every other row is unchanged.
- **AC-3 (mutation, QA).**
  - Making `trace_merge` take the right event on a tie reddens the
    seed's tie case.
  - Making `trace_window` inclusive at `hi` reddens the half-open case.
  - Removing `export MkEvent` reddens the client import.

## Stop conditions

- A law needs an Axiom, or an existing Derived public law or signature
  changes.
- `trusted_base()` grows on loading either package.
- A change outside the package, Derived, the census test and the new
  acceptance target.
