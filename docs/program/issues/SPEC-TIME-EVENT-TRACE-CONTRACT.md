---
id: SPEC-TIME-EVENT-TRACE-CONTRACT
title: "No time, event or trace contract exists in spec/50-stdlib, so the catalog's next roadmap layer after numerical computing has nothing to build against. Author its chapter with a first exact section: finite event traces over natural timestamps, with ordering and merge laws proved at zero trust"
status: merged
owner: spec
size: S
tier: T1
gate: architect
depends_on: [SPEC-NUMERICAL-RATIONAL-INTERVAL-CONTRACT]
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. 06-catalog-campaign.md:377 places time/events/traces after numerical computing. Same shape as SPEC-NUMERICAL-RATIONAL-INTERVAL-CONTRACT. Steward-filed per COORDINATION section 2."
---

# An exact event-trace contract

## Objective

A new `spec/50-stdlib` chapter exists, and its first section is the
normative contract for finite event traces stamped with natural-number
times. The first trace catalog node builds against it.

## Settled inputs (on `473ccf9b4`)

- **Natural order and lists are proved.** `Data.Numeric.Nat.Order`
  exports `leq_nat`, `lt_nat`, `min`, `max` and their order laws.
  `Data.Collections.List` exports `length` and the list operations.
- **No time, event or trace declaration exists** in `catalog/`,
  `spec/50-stdlib` or `conformance/`. Chapters 61 to 68 cover formal
  languages, graphs, statistics, rational, linear algebra, symbolic
  algebra, geometry and numerical intervals.
- **Wall-clock time is a host effect, not a value.** `ClockWallNow` is a
  host operation in the runtime. A pure trace contract stamps events
  with given `Nat` times and does not read a clock.
- **Known elaborator limits.**
  - Pass `map` a named helper, not a λ.
  - A dependent `match` refines only its scrutinee as written (CHECKS
    11).

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

1. **D0 (Architect boundary ruling, measured).** Rule from a checked
   development (rc=0, no Axiom) on:
   - the chapter number, the package's Section and its module path;
   - the carriers: for example an event of a `Nat` time and a payload,
     and a trace as a list of events, and what "time-ordered" means
     (non-decreasing or strictly increasing);
   - the operations: at least the time-window restriction of a trace
     and the merge of two time-ordered traces;
   - the laws: at least that window restriction and merge preserve time
     order, and that merge keeps every event of both inputs (by count
     or by membership);
   - what stays deferred: clocks as effects, real time, causality across
     processes (vector clocks), and infinite streams.
2. **The chapter, section 1**, stating the D0's operations and proved
   laws, with a conformance seed of closed cases.

## Acceptance

- **AC-1.** The Architect's D0 development checks generically, with no
  Axiom, and the section states exactly the laws it proved.
- **AC-2.** The seed has at least one case per operation and one per
  law, and a case that refuses a merge result which is out of time order.
- **AC-3.** `spec/50-stdlib/README.md` and `SPEC-PROGRESS.md` record the
  chapter.

## Stop conditions

- A law needs an Axiom, a kernel or surface change, or a new primitive.
- The D0 cannot check an order-preservation law generically. Report the
  measured blocker, and do not state that law as proved.
