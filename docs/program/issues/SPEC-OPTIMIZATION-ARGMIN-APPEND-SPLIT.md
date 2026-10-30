---
id: SPEC-OPTIMIZATION-ARGMIN-APPEND-SPLIT
title: "Spec 72 fixes argmin on a head and its tail, but no law relates argmin of two appended non-empty lists to the argmin of each part, so a client cannot split a selection and combine the results. Rule from a checked D0 at zero trust the two branch laws for argmin over nonempty_append, keeping the earlier part on a tie, and state them with a conformance seed"
status: merged
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md); Optimization is next in the layer rotation after Symbolic algebra. Same shape as SPEC-OPTIMIZATION-ARGMIN-COST-ORDER-INVARIANCE. Steward-filed per COORDINATION section 2."
---

# Argmin of an append is the better of the two parts

## Objective

Spec 72 states, as public checked laws over
`Algorithm.Optimization.Argmin`, that for every `cost` and non-empty `xs`
and `ys`, `argmin a cost (nonempty_append a xs ys)` is `argmin a cost ys`
when that part's cost is strictly less than `argmin a cost xs`'s, and is
`argmin a cost xs` otherwise, ties included.

## Settled inputs (spec 72 §1, main `e431f992a`)

- **The operation.** `argmin` is `argmin_from`, that is `select_from`
  with `better = lt_nat` (`Argmin.ken.md:56`, `:297`, `:305`). It keeps
  the head unless the tail's choice is strictly cheaper, so ties go to
  the earlier candidate (§1.1).
- **The append.** `nonempty_append` is public
  (`NonEmpty.ken.md:67`), with the public `list_view` proof that its
  list view is `list_append` of the two views (`:153`). `Argmin` does
  not import it today.
- **No append law.** §1.2 fixes the singleton and the head/tail branches.
  §1.4 and §1.5 add the Option selector and the cost transform. No
  section relates a selection over a concatenation to its parts, and
  §1.3's deferred list does not mention one.
- **Why it should hold.** The earliest-minimum choice is an associative
  binary operation, and `select_from` is its right fold. Induction on
  `xs` reduces the law to the head/tail branches and transitivity of
  `lt_nat`. A tie keeps the left part's choice, because the comparison
  is strict.
- **The public form.** `choose` is private (`:317`), so the law cannot
  name it. The expected form is two laws with a Boolean premise, like
  `argmin_keeps_head` and `argmin_takes_tail`. AC-0 rules it.
- **Seeds.** Under `conformance/stdlib/optimization/`, next to
  `seed-argmin-cost-order-invariance.md`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

A §1.6 in spec 72 stating the ruled laws by their public types, a closed
conformance seed under `conformance/stdlib/optimization/`, and the status
lines, all in one candidate. The catalog half travels in the candidate
when AC-0 rules it, as it did for SPEC-SYMBOLIC. Otherwise it is a
separate Foundation WP built from the D0. The argmax mirror is a
separate successor, not part of this candidate.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  with rc 0, no `Axiom`, and `trusted_base()` unchanged as an identity
  set. It rules:
  - the names, the argument order and the premise form;
  - which helpers stay private.

  It also confirms that no §1 to §3 operation or law type changes.
- **AC-1.** §1.6 states each ruled law by its public type. The Spec
  Leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted client splits labels `0, 1 | 2, 3` with
  costs `5, 2 | 2, 7`, so the minimum is tied across the split.
  - It applies the keep-left law and checks the selection independently
    by `Proved` to be label `1`.
  - A second accepted instance changes label `2`'s cost to `1` and checks
    the take-right law's selection to be label `2`.
  - Its refused block pins the later-tie claim on the first split: the
    selection equals `argmin` of the right part, label `2`.

## Stop conditions

- A law needs an Axiom, a kernel or surface change, or a premise that
  names a private helper.
- A change to an existing §1 to §3 operation or law type.

Checks: 4 fired: the tie rule and comparison direction are read from
`select_from` and `choose` on `e431f992a`, and `nonempty_append`'s
definition and public `list_view` proof were read at `NonEmpty.ken.md:67`
and `:153`. 8 fired: the accepted split ties its minimum across the two
parts, so a law that took the right part on a tie is refused by the seed,
and the second instance makes the take-right branch observable.
