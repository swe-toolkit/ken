---
id: SPEC-TIME-WINDOW-LENGTH-BOUND
title: "Spec 69 §8 says windowing does not assert that it preserves a trace's length, and no section states the bound that does hold: a window keeps a subsequence, so it never has more events than its trace. Data.Time.EventTrace exports ten window laws but none about size, so a client that needs `length (trace_window p lo hi tr) <= length tr` must re-prove it from filter's definition. Rule from a checked D0 at zero trust a public length bound for trace_window, with a conformance seed"
status: active
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md); Time follows Numerical in the layer rotation. Steward-filed per COORDINATION section 2."
---

# A time window never adds events

## Objective

`Data.Time.EventTrace` exports a public checked law that, for every
`p : Type`, `lo`, `hi : Nat` and `tr : List (Event p)`:

```ken
IsTrue (leq_nat (length (Event p) (trace_window p lo hi tr))
                (length (Event p) tr))
```

Spec 69 states it by its public type.

## Settled inputs (main `51e5279c8`)

- **The operation.** `trace_window` (`EventTrace.ken.md:100`) is
  `filter (Event p) (event_in_window p lo hi) tr`, with `filter` from
  `Data.Collections.Derived` (`Derived.ken.md:579`).
- **The enablers.** `length` is `Data.Collections.List`'s
  (`List.ken.md:26`), and Derived re-exports the same checked
  identity. `IsTrue b` is `Equal Bool b True`
  (`LawfulClasses.ken.md:58`). `Data.Numeric.Nat.Order` exports
  `leq_nat_weaken_right` (`Order.ken.md:736`), which takes `a <= b` to
  `a <= Suc b`. `leq_nat (Suc a) (Suc b)` reduces to `leq_nat a b`. The
  `Cons` case is that step when the head is kept, and weakening when it
  is dropped.
- **Imports.** EventTrace imports Derived by an explicit selection
  (`:57` to `:73`), with no `length`, and imports only `lt_nat`, `max`
  and `min` from Nat.Order (`:53`).
- **Not yet stated.** `spec/`, `catalog/` and `conformance/` were
  grepped for a window or filter length bound. Spec 69 §8
  (`69-time-and-events.md:808`) disclaims length preservation, and no
  section states the bound. Derived exports no filter length law.
  `Algorithm.Graphs.Dependency` keeps private `filter_length_cons`
  (`:1387`) and `length_filter_strict` (`:1583`). Spec 69 runs to §11
  (`:994`, file end `:1061`).
- **Seeds.** `conformance/stdlib/time-and-events/`, next to
  `seed-window-commutation.md`, whose `e1`, `e4` and `e7` trace can be
  reused.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

The ruled law in EventTrace, a spec 69 section stating it by its public
type, a closed conformance seed under
`conformance/stdlib/time-and-events/`, a named package test, and the
status lines, all in one candidate.

## Acceptance

- **AC-0 (D0).** The Architect specifies a development on the landed
  base: the source, and the named-binary commands with a bounded
  timeout. The Foundation implementer runs it on scratch and reports
  rc, the `Axiom` grep and `trusted_base()` as an identity set before
  and after. The Architect rules from that report, and does not build
  or test. It rules:
  - the law's name, argument order and placement;
  - the section number, and whether §8's disclaimer gains a pointer to
    it;
  - whether the generic step is a public `filter` length law in
    `Data.Collections.Derived` that EventTrace instantiates, or a
    private EventTrace proof. If it is public, whether Graphs'
    private `filter_length_cons` is restated through it or left as it
    is.

  It confirms that no §1 to §11 operation or law type changes.
- **AC-1.** The section states each ruled law by its public type. The
  Spec Leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted client applies the law to the
  `e1`, `e4`, `e7` trace under `[3,9)`.
  - Its refused block claims the reverse bound, the trace's length at
    most the window's, under `[3,9)`. That is false, because the window
    keeps `e4` and `e7` only, so 3 is not at most 2.
  - Its accepted same-form control claims the same reverse bound under
    `[0,9)`, which keeps all three events, so the refusal is the
    dropped event, not the form.
  - The D0 rules the proof form that separates the two rows.

## Stop conditions

- A law needs an Axiom, a kernel or surface change, or any premise
  beyond the window and the trace.
- A change to an existing §1 to §11 operation or law type.

Checks: 4 fired: the law is written from public `trace_window`,
`filter`, `length`, `leq_nat` and Nat.Order's `leq_nat_weaken_right`,
each read at its line, with the two missing imports named. 3 fired:
spec, catalog and conformance were grepped for a window or filter
length bound; the only hits are §8's disclaimer and Graphs' two private
helpers, which AC-0 disposes of. 8 fired: the refused row and its
control share the trace, the operation and the claim form, and differ
only in the window's lower bound, `3` against `0`.
