---
id: SPEC-TIME-SORT-IDEMPOTENT
title: "Spec 69 §§21-24 rewrite merges and windows into stable time sort, but no public law says that sorting is idempotent or that it leaves an ordered trace unchanged: the fixed-point fact exists only as EventTrace's private trace_ordered_sort_fixed, so a client holding sort (sort tr) or an ordered trace under sort must re-derive it. Rule from a checked D0 at zero trust that stable time sort is idempotent, and state it with a seed whose accepted row is open"
status: ready
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Time follows Numerical in the L3 layer rotation (docs/program/06-catalog-campaign.md). The ordered fixed point is private (EventTrace.ken.md:4115). Steward-filed per COORDINATION section 2."
---

# Sorting a trace twice is sorting it once

## Objective

`Data.Time.EventTrace` exports a public checked law. For every `p : Type`
and `tr : List (Event p)`, it states

`Equal (List (Event p)) (sort (Event p) (event_before p) (sort (Event p)
(event_before p) tr)) (sort (Event p) (event_before p) tr)`.

It holds at zero trust, and spec 69 states it by its public type.

## Settled inputs (main `f68605014`; anchors re-checked at release)

- **The argument, for AC-0 to check.**
  - The private `trace_ordered_sort_fixed` (`EventTrace.ken.md:4115`)
    gives `time_ordered p tr → Equal (sort … tr) tr`.
  - `(proof sorted for sort)` (`Derived.ken.md:1963`) gives the sorted
    output, with totality from `(proof total for leq_nat)` on event times.
    `EventTrace.ken.md:4209` already builds this term.
  - Applying the fixed point to `sort … tr` closes the law.
- **No such law exists.** EventTrace has 35 public theorems. Its sort laws
  rewrite merges (§§21-23) and windows (§24) into sort; none states sort
  of sort or sort of an ordered trace. Derived's generic
  `derived_sort_sorted_fixed` (`Derived.ken.md:2868`) is also private.
- **The ordered premise is needed for the fixed point.** Without it,
  `sort tr = tr` fails: for timestamps `[3, 1]` the sort is `[1, 3]`.
- **Spec and tests.**
  - Spec 69 ends with §24 (`69-time-and-events.md:1903`), so the new
    section is §25.
  - The seed directory is `conformance/stdlib/time-and-events/`.
  - The test pattern is
    `crates/ken-elaborator/tests/cat_time_window_sort.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Spec 69 §25, the catalog law, a seed, the package test rows and the status
lines. AC-0 rules whether `trace_ordered_sort_fixed` also becomes public.
No §1 to §24 operation or law type moves, and there is no kernel, surface
or Derived change.

## Deliverable

One candidate carrying the spec, seed and catalog halves, as for
SPEC-TIME-WINDOW-SORT-COMMUTE.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on the
  landed base, with rc 0, no `Axiom`, and `trusted_base()` unchanged as an
  identity set. It rules:
  - the name, the binder order and the placement;
  - whether the ordered fixed point is promoted to a public law beside the
    idempotence law, or stays private;
  - every consumer root of the touched exports, swept across `catalog/`,
    `conformance/`, `crates/`, `spec/` and `library/`, including any exact
    export roster and count-bearing sentence.
- **AC-1.** §25 states the law by its public type. CV and the Architect
  review it, with a Decision.
- **AC-2.** The seed's accepted block must not close by computation alone.
  - Its accepted row is open in the trace. AC-0 shows that neither `Refl`
    nor `Proved` closes it without the law.
  - Its refused twin, on a fresh accepted prefix and in the same form,
    claims `sort tr = tr` for an open trace with no ordered premise. That
    claim must refuse, and a closed control at `[3, 1]` carries its
    falsity.
  - AC-0 picks the closing terms and may strengthen the seed.
- **AC-T.** The package test asserts `trusted_base()` unchanged as an
  identity set on the assembled candidate. No Decision opens until it
  passes.
- **AC-3.** Each touched package test target passes on the exact
  candidate with no name filter. Full-workspace CI is green on the exact
  candidate.

## Stop conditions

- The law needs an `Axiom`, a kernel or surface change, or a premise.
- The proof needs a new public export outside EventTrace; report it rather
  than widening Derived.
- AC-T finds `trusted_base()` changed as an identity set.
- A change to an existing §1 to §24 operation or law type.

Checks: 4 fired. I read `trace_ordered_sort_fixed`, the sortedness term at
`:4209` and `derived_sort_sorted_fixed` on `9720876d5`; the law is the
fixed point applied to a sorted output. 3 fired. I listed EventTrace's 35
public theorems; none states sort of sort or of an ordered trace. 8 fired.
A closed seed row computes without the law (EDGE-EQUIV AC-0,
`evt_553g793zejmhh`), so the witness row is open, and the twin drops the
ordered premise, which `[3, 1]` refutes.
