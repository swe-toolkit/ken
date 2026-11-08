---
id: SPEC-OPTIMIZATION-OPTION-APPEND-SPLIT
title: "Spec 72 proves append-split laws for argmin and argmax over two non-empty parts (§1.6, §3.6), but not for argmin_option and argmax_option, the selectors over a possibly empty list: a client that appends two non-empty lists and selects with the Option form must unfold to the NonEmpty selector to know which part wins. Rule from a checked D0 at zero trust the four Option append-split laws, and state them with a conformance seed"
status: merged
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Optimization follows Symbolic algebra in the L3 layer rotation (docs/program/06-catalog-campaign.md). Steward-filed per COORDINATION section 2."
---

# The Option selectors split over append

## Objective

`Algorithm.Optimization.Argmin` exports four public checked laws:
`argmin_option` and `argmax_option`, applied to the `list_append` of two
`Cons` lists, equal the Option selector of the left part when the
non-empty sibling's keep-left premise holds, and of the right part when
its take-right premise holds. Spec 72 states them by their public types.

## Settled inputs (main `f830ce102`)

- **The non-empty laws.** In `Argmin.ken.md`, these are
  `argmin_append_keeps_left` (`:144`), `argmin_append_takes_right`
  (`:166`), `argmax_append_keeps_left` (`:331`) and
  `argmax_append_takes_right` (`:353`). Each takes two `NonEmpty a`
  parts and one `lt_nat` premise, and concludes about
  `nonempty_append`.
- **The Option bridge.** `argmin_option_cons` (`:199`) and
  `argmax_option_cons` (`:386`) equate the Option selector on
  `Cons a x rest` with `Some` of the non-empty selector on
  `nonempty_cons a x rest`. `argmin_option_cost_order_invariant`
  (`:220`) lifts a non-empty law through `Some` with `cong`; `cong` and
  `list_append` are already imported.
- **No such law exists.** A grep of catalog/, spec/, conformance/ and
  crates/ for `argm(in|ax)_option_append` returns no hits.
- **The spec.** Spec 72 states the non-empty split at §1.6 (`:392`) and
  §3.6 (`:1159`), and the Option selectors at §1.4 (`:236`) and §3.4
  (`:1009`). The last section is §3.7 (`:1237`), and the file ends at
  `:1300`. §1.4 deliberately leaves the §1.2 head, singleton, tie and
  branch laws on `argmin`; the append-split laws come from §1.6, so that
  exclusion does not cover them.
- **Seeds and tests.**
  - Seeds go under `conformance/stdlib/optimization/`. The nearest
    neighbours are `seed-argmin-append-split.md` and
    `seed-argmin-option-cost-order-invariance.md`.
  - The package test pattern is
    `crates/ken-elaborator/tests/cat_optimization_argmin_option_cost_order.rs`,
    which covers both Option selectors in one file.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The four ruled laws in `Argmin.ken.md`, spec 72 sections stating them by
their public types, a closed conformance seed and a named package test,
in one candidate.

## Acceptance

- **AC-0 (D0).** The Architect specifies a development on the landed
  base: the source, and the named-binary commands with a bounded
  timeout. The Foundation implementer runs it on scratch and reports
  rc, the `Axiom` grep, and `trusted_base()` as an identity set before
  and after. The Architect rules from the report and does not build or
  test. It rules:
  - the names, the parts' shape (two `Cons` lists, or a weaker form)
    and the placement;
  - that no existing spec 72 operation or law type changes.
- **AC-1.** The sections state the ruled laws by their public types.
  The Spec Leader and CV review them, with a Decision.
- **AC-2.** The seed's accepted client applies `argmin_option`'s
  keep-left law to two concrete lists whose right part's least cost is
  not lower, with the premise `Proved`. Its refused block applies the
  same law to lists whose right part is strictly cheaper, so the
  premise's `Proved` fails. The accepted same-form control is the first
  application. The refusal therefore comes from the failed premise, not
  the form. The `argmax_option` pair mirrors it. Each refusal is
  appended to a fresh, fully accepted prefix.

## Stop conditions

- A law needs an Axiom, a kernel or surface change, or a premise beyond
  the sibling's one `lt_nat` equation.
- A change to an existing spec 72 operation or law type.

Checks: 4 fired: the four non-empty laws, both `_cons` bridges and the
`cong` lift were read at their lines on `f830ce102`; each new law's
type uses only that vocabulary. 3 fired: grepped catalog/, spec/,
conformance/ and crates/ for `argm(in|ax)_option_append`; no hits. 8
fired: the refused application and its control differ only in the
lists, and the failed premise is the one `lt_nat` equation.
