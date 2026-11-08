---
id: SPEC-TIME-WINDOW-NONE-EMPTY
title: "Spec 69 §10 says a window covering every event of a trace leaves it unchanged, and §9 says an inverted window selects nothing, but no law states the dual of §10: a window that contains none of a trace's events selects the empty trace, whatever its bounds. A client that proves a quiet period must re-derive it from membership soundness. Rule from a checked D0 at zero trust the public law that a membership-quantified miss empties the window, and state it with a conformance seed"
status: merged
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Time follows Numerical in the L3 layer rotation (docs/program/06-catalog-campaign.md, time/events/traces). Steward-filed per COORDINATION section 2."
---

# A window with no events in it is empty

## Objective

`Data.Time.EventTrace` exports a public checked law. For every `p`,
`lo`, `hi` and trace `tr`, given that every member of `tr` is outside
the window, `trace_window p lo hi tr` is `Nil`. Spec 69 states the law
by its public type.

## Settled inputs (main `21420fdeb`)

- **The window.** `trace_window` (`EventTrace.ken.md:103`) is `filter`
  by `event_in_window` (`:100`), which is
  `bool_and (leq_nat lo t) (lt_nat t hi)`.
- **The neighbours.**
  - `trace_window_all_identity` (`:364`, spec 69 §10 at `:933`) takes
    the premise
    `(e : Event p) → list_elem (Event p) e tr → Equal Bool (event_in_window p lo hi e) True`
    and concludes `trace_window p lo hi tr = tr`.
  - This law takes the same premise shape with `False`, and concludes
    `Nil (Event p)`.
  - `trace_window_inverted_empty` (`:341`, §9 at `:872`) empties only
    a window with `leq_nat hi lo`. A non-inverted window with no
    member is not covered.
- **The spec.** Spec 69 runs §1 to §13; §13 starts at
  `69-time-and-events.md:1129`, and the file ends at `:1198`.
- **Seeds and tests.**
  - Seeds go under `conformance/stdlib/time-and-events/`.
  - The package test pattern is
    `crates/ken-elaborator/tests/cat_time_window_split.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The ruled law in EventTrace, a spec 69 section stating it by its public
type, a closed conformance seed and a named package test, in one
candidate.

## Acceptance

- **AC-0 (D0).** The Architect specifies a development on the landed
  base: the source, and the named-binary commands with a bounded
  timeout. The Foundation implementer runs it on scratch and reports
  rc, the `Axiom` grep, and `trusted_base()` as an identity set before
  and after. The Architect rules from the report and does not build or
  test. It rules:
  - the name and placement;
  - that no §1 to §13 operation or law type changes.
- **AC-1.** The section states the ruled law by its public type. The
  Spec Leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted client applies the law to a closed
  trace with events at times 1 and 5 and the non-inverted window
  `[2, 5)`, and obtains `Nil`. Its refused block applies the law to the
  window `[1, 5)`, where the event at time 1 is inside, and the miss
  premise's arm for that event fails. The accepted same-form control is
  the `[2, 5)` application. The refusal therefore comes from the failed
  premise, not the form.

## Stop conditions

- The law needs an Axiom, a kernel or surface change, or a premise
  beyond the membership-quantified miss.
- A change to an existing §1 to §13 operation or law type.

Checks: 4 fired: `trace_window`, `event_in_window`,
`trace_window_all_identity` and `trace_window_inverted_empty` were read
at their lines on `21420fdeb`; the law's type is written in that
vocabulary. 3 fired: grepped catalog/, spec/ and conformance/ for a
window-empty law; only the inverted one exists. 8 fired: the refused
application and its control differ only in the window's lower bound,
and the failed premise is the in-window event's arm.
