---
id: SPEC-OPTIMIZATION-ARGMAX-APPEND-SPLIT
title: "Spec 72 §1.6 splits argmin over two appended non-empty lists and names the argmax mirror as a separate successor, but §3 has no such law: a client cannot split a greatest-cost selection and combine the parts. Rule from a checked D0 at zero trust the two branch laws for argmax over nonempty_append, keeping the earlier part on a tie, and state them with a conformance seed"
status: merged
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md); Optimization is next in the layer rotation after Symbolic algebra. Spec 72 §1.6 names this mirror as its separate successor. Steward-filed per COORDINATION section 2."
---

# Argmax of an append is the better of the two parts

## Objective

Spec 72 states, as public checked laws over
`Algorithm.Optimization.Argmin`, that for every `cost` and non-empty `xs`
and `ys`, `argmax a cost (nonempty_append a xs ys)` is `argmax a cost ys`
when the left part's choice costs strictly less than the right part's,
and is `argmax a cost xs` otherwise, ties included.

## Settled inputs (spec 72 §1 to §3, main `85e628792`)

- **The operation.** `argmax` is `argmax_from`, that is `select_from`
  with `better = costlier`, where `costlier challenger incumbent` is
  `lt_nat incumbent challenger` (`Argmin.ken.md:220`, `:347`, `:350`).
  An equal-cost tail candidate does not replace the head, so ties go to
  the earlier candidate (§3.1).
- **The argmin half exists.** §1.6 (`72-optimization.md:362`) states
  `argmin_append_keeps_left` and `argmin_append_takes_right`, and says
  the argmax mirror is a separate successor (`:406`). The catalog
  exports them at `Argmin.ken.md:144` and `:166`.
- **The helpers are argmin-specific.** `argmin_view`,
  `argmin_via_view`, `argmin_append_choice`, `argmin_from_append` and
  `choose_earliest_assoc` (`:473` to `:622`) fix the `lt_nat (cost r)
  (cost l)` direction. The `select_from_cost_order` helper (`:699`) is
  already generic over `better`. AC-0 rules whether to generalize the
  append helpers or mirror them.
- **The premise form.** `argmax_keeps_head` and `argmax_takes_tail`
  (`:282`, `:288`) put the incumbent's cost on the left:
  `lt_nat (cost x) (cost (argmax … tail))`. The expected form is
  `lt_nat (cost (argmax a cost xs)) (cost (argmax a cost ys))`, with
  `False` keeping the left part and `True` taking the right.
- **Placement.** The §3.5 law sits after `argmax_takes_tail` and before
  `argmax_option`; the new pair follows it. §3's status block and §3.3's
  deferred list need their lines.
- **Seeds.** Under `conformance/stdlib/optimization/`, next to
  `seed-argmin-append-split.md`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

A §3.6 in spec 72 stating the ruled laws by their public types, the two
catalog theorems in `Argmin`, a closed conformance seed under
`conformance/stdlib/optimization/`, and the status lines, all in one
candidate.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  with rc 0, no `Axiom`, and `trusted_base()` unchanged as an identity
  set. It rules:
  - the names, argument order, premise form and placement;
  - whether the §1.6 helpers are generalized over `better` or mirrored,
    and which stay private.

  It also confirms that no §1 to §3.5 operation or law type changes,
  including §1.6's.
- **AC-1.** §3.6 states each ruled law by its public type. The Spec
  Leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted client splits labels `0, 1 | 2, 3` with
  costs `2, 7 | 7, 1`, so the maximum is tied across the split.
  - It applies the keep-left law and checks the selection independently
    by `Proved` to be label `1`.
  - A second accepted instance raises label `2`'s cost to `8` and checks
    the take-right law's selection to be label `2`.
  - Its refused block pins the later-tie claim on the first split: the
    selection equals `argmax` of the right part, label `2`.

## Stop conditions

- A law needs an Axiom, a kernel or surface change, or a premise that
  names a private helper.
- A change to an existing §1 to §3.5 operation or law type.

Checks: 4 fired: `argmax`, `argmax_from`, `costlier` and the two branch
laws were read at their lines on `b557bac0a`, unchanged on `85e628792`,
with §1.6's successor sentence at `:406`. The append helpers were read
and found fixed to the argmin direction, so AC-0 owns that choice. 8
fired: the accepted split ties its maximum across the two parts, so a
law that took the right part on a tie is refused by the seed, and the
second instance makes the take-right branch observable.
