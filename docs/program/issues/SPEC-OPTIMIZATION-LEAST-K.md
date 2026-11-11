---
id: SPEC-OPTIMIZATION-LEAST-K
title: "Spec 72 selects one least-cost or greatest-cost candidate, and its own deferral list says it supplies no top-k selection, so a client needing the k cheapest candidates has no checked operation. Add least_k over a Nat cost and a List, with checked laws at zero trust and no equality decision on candidates: the result has min k (length xs) members, each is a member of xs, no unselected candidate is cheaper than a selected one, and at k = 1 it agrees with argmin_option, with a conformance seed"
status: active
owner: spec
size: L
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Optimization follows Symbolic algebra in the L3 layer rotation; top-k is the deferral spec 72 names (72-optimization.md:230, :1054). Steward-filed per COORDINATION section 2."
---

# The k cheapest candidates

## Objective

`Algorithm.Optimization.Argmin` exports `least_k`, the `k` least-cost
candidates of a `List` under a `Nat` cost, in non-decreasing cost
order with earliest-first ties. It needs no equality decision on
candidates. Its checked public laws, at zero trust:
- **Length:** the result has `min k (length xs)` members.
- **Member:** every selected candidate is a member of `xs`.
- **Separation:** for a selected `y` and any `z` in `xs` that is not a
  member of the result, `cost y` is at most `cost z`.
- **Unit agreement:** at `k = 1` the result is the list of
  `argmin_option`'s answer, so the existing earliest-tie rule carries.

Spec 72 states the operation and its laws by their public types.

## Settled inputs (main `f32af2a0b`)

- **The selectors.** `argmin` (`Argmin.ken.md:59`) and `argmin_option`
  (`:188`) choose one candidate, with laws for membership, least cost
  and the earliest tie (`argmin_member` `:62`, `argmin_least` `:72`,
  `argmin_head_on_tie` `:97`, `argmin_option_member` `:207`,
  `argmin_option_least` `:212`). Membership is `list_elem`, an Ω-valued
  proposition (`Derived.ken.md:167`), never an equality decision.
- **The deferral.** Spec 72 ends at §3.8 (`72-optimization.md:1349`).
  §1.3 and §3.3 defer "top-k and stable multi-result selection"
  (`:230`) and say §3 "supplies no top-k selection" (`:1054`).
- **The list floor.** `length` is `Data.Collections.List`
  (`List.ken.md:26`). `Data.Collections.Derived` has `sort` over a
  Boolean `le` (`:1640`), `insert` (`:1630`), `is_sorted` (`:636`) and
  `Perm` (`:1627`); `Perm` counts with an equality function, so this
  node does not state its laws through it. Derived has a private
  `take` (`:127`) and a private `length_take_min` (`:1164`), whose
  statement is take's length law; the candidate publishes that `take`
  rather than adding a second one (Architect `evt_6mvst2shcc9m2`).
  There is no `min` on lists anywhere under `catalog/packages`.
- **Seeds and tests.** Seeds go under `conformance/stdlib/optimization/`;
  tests are named `crates/ken-elaborator/tests/cat_optimization_<slug>.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

One candidate:
- `least_k` and its laws in `Argmin`, with names, argument order,
  placement and the definition route as AC-0 rules them;
- any list or Nat helper the definition or proofs need, proved, in its
  owning `Data` package;
- a new spec 72 section;
- a closed conformance seed;
- a package test;
- the status lines.

## Acceptance

- **AC-0 (D0, Architect specifies, Foundation implementer runs on
  scratch).** A checked development on the landed base, with rc 0, no
  `Axiom`, and `trusted_base()` unchanged as an identity set. It rules:
  - the definition, and whether it is a stable cost sort then a prefix
    or a direct selection;
  - the exact statement of each law, keeping Separation free of an
    equality decision on candidates;
  - whether an ordered-result law is public;
  - that no existing `argmin`, `argmax` or bounded-search operation or
    law type changes.
- **AC-1.** The new section states `least_k` and its laws by their
  public types. The Spec Leader and CV review it.
- **AC-2.** The seed's accepted client checks each law at closed `k`
  and `xs`, including a tie that the earliest-first rule decides. Its
  refused block pins, in the same form as an accepted control, the
  claim that a more expensive candidate is selected over a cheaper
  unselected one.
- **AC-3.** Every Optimization package test that checked before still
  checks. The new package test fails if `least_k` keeps the first `k`
  candidates of `xs` without regard to cost, and fails if it drops the
  earliest-tie order.

## Stop conditions

- A law needs an Axiom, an equality decision on candidates, or a kernel
  or surface change.
- A change to an existing operation or law type of spec 72.
- Arbitrary `Ord` costs, lexicographic costs or approximation become
  necessary: each stays deferred as a separate node.

Checks: 4 fired: `argmin`, `argmin_option` and their membership and
least laws were read at the cited lines on `f32af2a0b`, as were `length`,
`sort`, `is_sorted` and `Perm`. `Perm` needs an equality function, so
the frame states no law through it, and AC-0 rules the statements.
3 fired: `least_k`, `top_k` and `topk` were grepped over `catalog`,
`spec` and the issues, with no hit. The `take` grep read only public
names and missed Derived's private `take`; corrected above. 8 fired: AC-3's
first-k-by-position mutant passes Length and Member, so only
Separation and the seed's refused claim reject it.
