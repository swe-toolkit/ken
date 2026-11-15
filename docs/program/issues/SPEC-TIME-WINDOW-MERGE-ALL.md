---
id: SPEC-TIME-WINDOW-MERGE-ALL
title: "Spec 69 §3 distributes a time window over the two-way trace merge, but §15's k-way trace_merge_all has no window law, so a client that windows a merged list of traces cannot window each trace first. Rule from a checked D0 at zero trust that, when every input trace is time-ordered, trace_window of trace_merge_all equals trace_merge_all of the windowed traces, and state it with a conformance seed"
status: active
owner: spec
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Time follows Numerical in the L3 layer rotation (docs/program/06-catalog-campaign.md). Steward-filed per COORDINATION section 2."
---

# A time window distributes over the k-way merge

## Objective

`Data.Time.EventTrace` exports a public checked law that, when every
trace in `traces` is `time_ordered`, `trace_window p lo hi
(trace_merge_all p traces)` equals `trace_merge_all p (map
(trace_window p lo hi) traces)` as `Equal (List (Event p))`. It holds
at zero trust, and spec 69 states it by its public type.

## Settled inputs (main `4f24d43eb`)

- **The gap.** §3 (`spec/50-stdlib/69-time-and-events.md:345`) gives
  `trace_window_merge` for two ordered traces
  (`catalog/packages/Data/Time/EventTrace.ken.md:814`). §15 (`:1259`)
  adds `trace_merge_all` (`EventTrace.ken.md:122`) with ordered,
  membership and count laws (`:2468`, `:2487`, `:2519`, `:2543`). No
  law relates a window to `trace_merge_all`. Spec 69 ends at §16, so
  the new section is §17.
- **The operations.** `trace_merge_all` is the right fold
  `trace_merge p t (trace_merge_all p rest)` with `Nil` at the base.
  `trace_window` is `filter` by `event_in_window` (`:113`). `map` is
  public in `Data.Collections.Derived` (`Derived.ken.md:553`) but is
  not imported by EventTrace today.
- **The premise form exists.** `trace_merge_all_ordered` takes
  `(t : List (Event p)) → list_elem (List (Event p)) t traces →
  time_ordered p t` and concludes that the merge is ordered.
- **Seeds and tests.** The seed goes beside
  `conformance/stdlib/time-and-events/seed-window-merge-distribution.md`.
  The test pattern is
  `crates/ken-elaborator/tests/cat_time_window_merge_distribution.rs`.
- **Catalog style.** The catalog half follows
  `docs/program/07-catalog-style-guide.md` and
  `agent/playbooks/tools/write-ken.md`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Spec 69 §17, the catalog law with the `map` import and any private
helper, a seed, a package test and the status lines. No §1 to §16
operation or law type moves, and there is no kernel or surface change.

## Deliverable

One candidate carrying spec, seed and catalog halves, as for
SPEC-NUMERICAL-INTERVAL-WIDTH-MUL.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on
  the landed base with rc 0, no `Axiom`, and `trusted_base()` unchanged
  as an identity set. It rules:
  - the law's name and binder order, following `trace_window_merge`
    and `trace_merge_all_ordered`;
  - its placement in the export order;
  - whether the premise is the `list_elem` form of
    `trace_merge_all_ordered` or another form;
  - whether the ordering premise is necessary for every trace.
- **AC-1.** §17 states the ruled law by its public type. The Spec
  Leader and the CV review it, with a Decision.
- **AC-2.** The seed works on closed traces with `Bool` payloads.
  - **Accepted block.** Three ordered traces under one window, where
    events of all three survive, two share a timestamp, and the second
    and third traces each have an event outside the window. It applies
    the law and concludes the windowed merge closed.
  - **Refused block.** It is on a fresh accepted prefix and in the same
    form as the accepted row. It drops the ordering premise, using
    §3.1's unordered `[5,1]` beside an ordered `[3]` under window
    `[0,4)`, where the windowed merge is `[3,1]` and the merge of the
    windowed traces is `[1,3]`.
  - The seed has a direct client that imports and applies the public
    law, and a refused row for each premise AC-0 rules necessary.
- **AC-3.** Readability. The catalog proof reads as the style guide
  requires: local names state a proof stage or a trace, and the
  candidate passes the independent local-naming and formatter-layout
  review.
- **AC-4.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The law needs an `Axiom`, a kernel or surface change, or a premise
  beyond the per-trace ordering hypothesis.
- A change to an existing §1 to §16 operation or law type.

Checks:
- **4 fired.** The obligation is written out. `map f (Cons t rest)`
  computes to `Cons (f t) (map f rest)`, so the `Cons` goal is
  `window (merge t (merge_all rest)) = merge (window t) (merge_all
  (map window rest))`. `trace_window_merge` with `time_ordered p t`
  (the premise at `list_elem_head`) and `time_ordered p (merge_all
  rest)` (`trace_merge_all_ordered` on the premise restricted by
  `list_elem_later`) rewrites the left side. `cong` with the induction
  hypothesis on the restricted premise closes it. The `Nil` case
  computes on both sides.
- **3 fired.** No window law over `trace_merge_all` occurs in
  `catalog/`, `spec/`, `conformance/` or the elaborator tests;
  `trace_merge_all` occurs in EventTrace, its two seeds, its test and
  spec 69.
- **8 fired.** A premise-free law accepts the refused block. A law
  that windows only the first trace keeps the later traces'
  out-of-window events, so the accepted block, which has such events
  in its second and third traces, refuses it. Only the ordered law
  accepts the accepted block and refuses the refused one. The direct
  client and the per-premise refused rows are the two gaps the CV
  rejected on SPEC-NUMERICAL-INTERVAL-WIDTH-MUL, required up front.
