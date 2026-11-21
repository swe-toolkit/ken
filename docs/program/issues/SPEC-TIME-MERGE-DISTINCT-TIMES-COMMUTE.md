---
id: SPEC-TIME-MERGE-DISTINCT-TIMES-COMMUTE
title: "Spec 69 states trace_merge's ordering, membership, count, length and associativity, but no law says when argument order does not matter, so a client merging two event sources must fix an order even when no timestamp is shared. Rule from a checked D0 at zero trust that trace_merge p xs ys equals trace_merge p ys xs when no event of xs shares a timestamp with an event of ys, and state it with a seed that separates distinct timestamps from a shared one"
status: active
owner: spec
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Time follows Numerical in the L3 layer rotation (docs/program/06-catalog-campaign.md, time/events/traces). The merge laws give associativity but no argument-order law. Steward-filed per COORDINATION section 2."
---

# Merging traces with distinct timestamps does not depend on order

## Objective

`Data.Time.EventTrace` exports a public checked law. For every `p`, `xs`
and `ys`, if no event of `xs` has the same `event_time` as an event of
`ys`, then `trace_merge p xs ys` equals `trace_merge p ys xs` as
`Equal (List (Event p))`. It holds at zero trust, and spec 69 states it by
its public type.

## Settled inputs (main `21d4c6ccd`; anchors re-checked at release)

- **The operation.** `trace_merge` (`EventTrace.ken.md:119`) recurses on
  `xs` through the private `trace_merge_into` (`:474`). That function
  compares the two heads with `event_before` (`:107`, `leq_nat` on times)
  and takes the left head when it is true. So on a shared timestamp the
  left argument's event comes first, and swapping the arguments reorders
  the tied events.
- **The gap.** The public merge laws are `trace_merge_ordered`,
  `_count`, `_length`, `_nil_left`, `_nil_right`, `_separated`,
  `_member_sound`, `_member_left`, `_member_right` and `_assoc`. None of
  them relates `trace_merge p xs ys` to `trace_merge p ys xs`.
- **The premise shape.** `trace_merge_separated` (`:1321`) already states
  a pairwise premise over `list_elem` of both traces. The new premise
  quantifies the same way.
- **Spec and tests.** Spec 69 ends with §19, which says merging preserves
  the number of events, so the new section is §20. The seed directory is
  `conformance/stdlib/time-and-events/`. The test pattern is
  `crates/ken-elaborator/tests/cat_time_merge_length.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Spec 69 §20, the catalog law with any private helper, the exported-law
roster entry if `EventTrace` has one, a seed, a package test and the status
lines. No §1 to §19 operation or law type moves, and no kernel or surface
change.

## Deliverable

One candidate carrying spec, seed and catalog halves, as for
SPEC-NUMERICAL-INTERVAL-WIDTH-ZERO-MEMBERS-EQUIV.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on
  the landed base, with rc 0, no `Axiom`, and `trusted_base()` unchanged
  as an identity set. It rules:
  - the name and the binder order;
  - the distinct-timestamp premise's form, for example the two
    `event_before` results disagreeing or a Boolean `Nat` inequality;
  - whether any `time_ordered` premise is needed. The merge compares heads
    only, so none is expected; one is admissible only if the D0 measures
    the law false without it;
  - every consumer root of an exported `EventTrace` law, by the sibling
    `trace_merge_assoc` across `catalog/`, `conformance/`, `crates/`,
    `spec/` and `library/`, so the candidate's scope is complete.
- **AC-1.** §20 states the law by its public type. The Spec Leader and CV
  review it, with a Decision.
- **AC-2.** The seed's accepted block applies the law to two traces with
  distinct timestamps and concludes the two merges equal. Its refused
  block, on a fresh accepted prefix, uses two traces that share one
  timestamp with different payloads, in the same application form. The
  distinct-timestamp premise fails to check there, and the two merges
  really differ. The two rows differ only in that one timestamp.
- **AC-T.** The package test asserts `trusted_base()` unchanged as an
  identity set on the assembled candidate. No Decision opens until it
  passes.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The law needs an `Axiom`, a kernel or surface change, or a premise
  beyond distinct timestamps and, if the D0 measures it necessary,
  `time_ordered` on both traces.
- AC-T finds `trusted_base()` changed as an identity set.
- A change to an existing §1 to §19 operation or law type.

Checks: 3 fired. I listed every public `trace_merge` law on `21d4c6ccd`;
none states argument-order independence. AC-0 sweeps the consumer roots
by the sibling law, because Numerical's first QA pass found an exported
law missing from the `modules.rs` roster. 4 fired. I read
`trace_merge_into` (`:474`): it takes the left head on `event_before`, so
a law over distinct timestamps turns into two head comparisons that
cannot both be true. 8 fired. In the refused row, the left-biased tie
makes `trace_merge [a@1] [b@1]` yield `[a, b]` and the swap yield
`[b, a]`, so the law must refuse there, and it does at the premise.
