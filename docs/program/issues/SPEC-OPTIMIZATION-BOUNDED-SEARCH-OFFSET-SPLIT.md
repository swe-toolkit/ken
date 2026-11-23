---
id: SPEC-OPTIMIZATION-BOUNDED-SEARCH-OFFSET-SPLIT
title: "Spec 72 splits every list selector over appended parts (§1.6, §1.8, §3.6, §3.8, §4.7), but find_below has no split over its range: §2.4 keeps a witness already found below a smaller bound, and nothing carries a hit found past it. So a client that searches [0, b) and then [b, b + c) cannot conclude the whole search's result from the two parts. Rule from a checked D0 at zero trust that, when find_below p b is None and q searches p shifted by b, a hit n of find_below q c gives find_below p (add b c) = Some (add b n), and state it with a seed that separates a refuted first part from a first part that already hits"
status: merged
owner: spec
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Optimization follows Symbolic algebra in the L3 layer rotation (docs/program/06-catalog-campaign.md). find_below is the one §72 selector without a split law. Steward-filed per COORDINATION section 2."
---

# A bounded search splits at an offset

## Objective

`Algorithm.Optimization.BoundedSearch` exports a public checked law. Take
`p` and `q : Nat → Bool`, with `q m` equal to `p (add b m)` at every `m`.
If `find_below p b` is `None` and `find_below q c` is `Some n`, then
`find_below p (add b c)` is `Some (add b n)`. It holds at zero trust, and
spec 72 states it by its public type.

Merged at squash `df1ade4e57c8beff171dbe1cb4b1dade8fe3b096` (candidate
`8af67e54a6cb8df95203d01a323754a3bac03efd`, full CI run `45209548371`).

## Settled inputs (main `9716dd74f`)

- **The operation.** `find_below p bound`
  (`BoundedSearch.ken.md` ~`:38`) recurses on `bound` through the private
  `find_next`. It returns the least hit below `bound`, or `None`.
- **The neighbours.**
  - `find_below_stable` (~`:90`, §2.4) keeps a `Some n` under a larger
    bound. That is the left arm of a split.
  - `find_below_least_hit` (~`:82`, §2.6) concludes `Some n` from a hit,
    its bound and the refutation of everything below it.
  - `find_below_ext` (~`:130`, §2.7) compares two predicates under one
    bound.
  - No law relates a search over `[b, b + c)` to a search of the shifted
    predicate.
- **The type grammar has no lambda.** The Graphs carry records this. So
  the shifted predicate is a second parameter `q` with a pointwise
  premise, as `find_below_ext` does, and not `λm. p (add b m)` in the
  type.
- **Vocabulary.** `add` is public in `Data.Numeric.Nat.Arithmetic`
  (`:18`). BoundedSearch imports `lt_nat` and `leq_nat` today. No public
  `Option` map exists, so the conclusion names `Some` directly.
- **Spec and tests.** §2 ends at §2.7 (`spec/50-stdlib/72-optimization.md`
  ~`:881`), so the new section is §2.8. The seed directory is
  `conformance/stdlib/optimization/`. The test pattern is
  `crates/ken-elaborator/tests/cat_optimization_bounded_search_least_hit.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Spec 72 §2.8, the catalog law with any private helper, a seed, a package
test and the status lines. No §1 to §4.7 operation or law type moves, and
there is no kernel or surface change.

## Deliverable

One candidate carrying the spec, seed and catalog halves, as for
SPEC-OPTIMIZATION-BOUNDED-SEARCH-AGREE.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on
  the landed base, with rc 0, no `Axiom`, and `trusted_base()` unchanged
  as an identity set. It rules:
  - the name and the binder order;
  - whether the `None` companion is also exported (both parts refuted
    gives `None` at `add b c`), or left to `find_below_refuted`;
  - the import of `add`.

  Sweep the consumers of the package's public roster for the sibling
  laws.
- **AC-1.** §2.8 states the law by its public type. The Spec Leader and
  CV review it, with a Decision.
- **AC-2.** The seed's accepted block takes `p` hitting exactly at 5 and
  7, with `b = 3`, `c = 4` and `q m = p (add 3 m)`.
  - `find_below p 3` is `None`, and `find_below q 4` is `Some 2`. The
    block concludes `find_below p 7 = Some 5`.
  - The refused block uses the same `p`, on a fresh accepted prefix and
    in the same application form, with `b = 6`, `c = 2` and
    `q m = p (add 6 m)`. Here `find_below q 2` is `Some 1`, but the
    premise `find_below p 6 = None` fails to check, because the first
    part already hits at 5. Without that premise the law would conclude
    `find_below p 8 = Some 7`, and the true result is `Some 5`.
- **AC-T.** The package test asserts `trusted_base()` unchanged as an
  identity set on the assembled candidate. No Decision opens until it
  passes.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The law needs an `Axiom`, a kernel or surface change, or a premise
  beyond the refuted first part and the pointwise shift.
- A change to an existing §1 to §4.7 operation or law type.

Checks: 3 fired. I listed every public BoundedSearch law on `9716dd74f`.
They are `find_below_sound`, `_least`, `_complete`, `_refuted`,
`_least_hit`, `_stable` and `_ext`. None relates a range to a shifted
search. 4 fired. I read `find_below`'s recursion and `find_below_ext`'s
pointwise premise. Graphs recorded that the type grammar has no lambda.
I found no public `Option` map. 8 fired. In the refused row the first
part already hits, so `find_below p 6` is `Some 5`. A law that ignored
the `None` premise would conclude `Some 7`, so the refused row pins that
premise and only that one: its shifted search does hit.
