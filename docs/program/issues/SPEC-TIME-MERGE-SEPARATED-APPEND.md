---
id: SPEC-TIME-MERGE-SEPARATED-APPEND
title: "Spec 69 says merge is not append, and no checked law says when it is: when every event of the left trace is at or before every event of the right one, the left-biased merge takes the whole left trace first, so trace_merge is list_append. Rule from a checked D0 at zero trust a public separated-merge law, with a conformance seed whose client joins two adjacent windows of an ordered trace back into one"
status: active
owner: spec
size: S
tier: T1
gate: architect
depends_on: [SPEC-NUMERICAL-INTERVAL-INVERTED-EMPTY]
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md); Time follows Numerical in the layer rotation. Steward-filed per COORDINATION section 2."
---

# A separated merge is an append

## Objective

`Data.Time.EventTrace` exports a public checked law that, for every
`p : Type` and `xs`, `ys : List (Event p)` in which every event of `xs`
is at or before every event of `ys`:

```ken
Equal (List (Event p)) (trace_merge p xs ys) (list_append (Event p) xs ys)
```

With §7's adjacent split, merging the two halves of a split window
gives back the window. Spec 69 states the law by its public type.

## Settled inputs (main `a1f4756c5`)

- **The merge.** `trace_merge` (`EventTrace.ken.md:104`) recurses on
  `xs` through the private `trace_merge_into` (`:406`). It keeps `x`
  ahead of `y` exactly when `event_before p x y` is `True` (`:416` to
  `:418`), so a separated left trace is taken whole.
- **The order.** `event_before` (`:92`) is `leq_nat` on event times.
  The private `event_before_trans` follows at `:422`.
- **The enablers.** `list_append` (`Data/Collections/Derived.ken.md:99`)
  is already imported for §7. `list_all` (`:111`) is an `Omega`-valued
  every-element predicate, available for the premise.
- **The claim's gap.** Spec 69 §4 (`69-time-and-events.md:505`) says
  merge "is not append". No section states when it is. Spec 69 runs §1
  to §12 (file end `:1126`), and the package exports 19 public theorems.
- **Seeds.** Under `conformance/stdlib/time-and-events/`, next to
  `seed-window-adjacent-split.md`. The package test pattern is
  `crates/ken-elaborator/tests/cat_time_window_append.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

The ruled law in EventTrace, a spec 69 section stating it by its public
type, a closed conformance seed under
`conformance/stdlib/time-and-events/`, a named package test, and spec
69's status lines, all in one candidate.

## Acceptance

- **AC-0 (D0).** The Architect specifies a development on the landed
  base: the source, and the named-binary commands with a bounded
  timeout. The Foundation implementer runs it on scratch and reports
  rc, the `Axiom` grep and `trusted_base()` as an identity set before
  and after. The Architect rules from that report, and does not build
  or test. It rules:
  - the law's name, argument order and placement;
  - the section number;
  - the separation premise's form: `list_all` over both traces, a
    membership-quantified premise, or the last-before-first form with
    both traces ordered.

  It confirms that no §1 to §12 operation or law type changes.
- **AC-1.** The section states the ruled law by its public type. The
  Spec Leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted client splits a closed ordered trace at
  `mid` with §7's law and uses the new law to show that merging the two
  windows gives `trace_window p lo hi tr`.
  - Its refused block claims the append equation for two traces whose
    times interleave, where the merge reorders.
  - Its accepted same-form control is the separated pair, so the
    refusal is the interleaving, not the form.

## Stop conditions

- The law needs an Axiom, a kernel or surface change, or a premise
  beyond separation.
- A change to an existing §1 to §12 operation or law type.

Checks: 4 fired: the law is written from `trace_merge` at `:104`,
`trace_merge_into`'s True arm at `:416`, `list_append` at `:99` and
`list_all` at `:111`, each read at its line. 3 fired: EventTrace's 19
public theorems and spec 69 §1 to §12 were read for a law equating
merge with append; none exists, and §4 says merge is not append. 8
fired: the refused row and its control share the claim and the form,
and differ only in whether the times interleave.
