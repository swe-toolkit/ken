---
id: CAT-MAP-DOM-MEMBER-LAW
title: "Proof-backfill for Data/Collections/Map.ken.md: prove privately that the public dom preserves membership exactly -- for every comparator leq, query x and raw tree m, set_member leq x (dom m) = member leq x m -- instead of only the selected size (dom r) counts"
status: active
owner: foundation
size: S
gate: architect
tier: T1
depends_on: []
blocks: []
github: null
origin: "Architect nomination evt_3mf8t3zn0t38s at fdaecfa16, as the L3 runway successor to CAT-DEQUE-POPBACK-LAWS while CAT-ARGPARSE-LAWS and CAT-PARSING-LAWS are held for the operator. Operator ruling 2026-09-13: 'schedule the proof backfill before extending the catalog.' Steward-filed per COORDINATION section 2."
---

# `dom` is unproved against membership

## Settled inputs -- measured at `fdaecfa16`. Re-ground before acting.

- Map publishes `Tree` and `dom`. `dom` replaces each node value by `MkUnit`
  and keeps shape, keys and subtrees (`pub fn dom`, and spec
  `58-maps-sets-relations.md §4.7`). It is total even on raw trees.
- `member` is `lookup` followed by `Some ↦ True`. `lookup` decides on
  `leq key k2`, then `leq k2 key`. `set_member a leq x s` is
  `member a Unit leq x s`. Both are private observations in the package.
- No `dom` law exists. The `size (dom r)` examples assert selected counts
  only, and a count cannot see a changed key.

## Deliverable

One checked private law in Map, next to `dom`, over every `k`, `v`,
`leq : k → k → Bool`, `x : k` and raw `m : Tree k v`:

`Equal Bool (set_member k leq x (dom k v m)) (member k v leq x m)`

Both sides use the same comparator. There is **no `Ordered` precondition**,
because `dom` keeps the whole decision-tree shape. No provider publication
is needed. `set_member`, `member`, `dom` and `Tree` are all already in the
package.

The Architect's route: a structural split on `m`, with the two `leq`
decisions shared on both sides. A node hit is `True = True`, and the left
and right branches reuse the induction hypotheses.

## Acceptance criteria

- **AC-1 (expressibility first).** State the proposition in Ken inside Map
  before proving it.
- **AC-2 (falsifier).** Apply two natural-site one-line mutations of `dom`,
  one at a time, restoring the exact tree between them: replace only the
  left subtree by `Leaf k Unit`, then only the right. For each mutation:
  - with the new law temporarily omitted, the changed `dom` stays
    well-typed and the package checks;
  - with the unchanged law restored, the whole Ken check rejects **at the
    law**, not at an earlier parser or definition error.

  A query present only in the changed subtree is the non-vacuity witness.
  It does not replace the law-span rejection.
- **AC-3.** No new trust and no public API change. Main has no exact
  inventory pin for Map's exports today; the `map_build_acceptance.rs`
  checks are selected, so an extra `pub fn` would keep them green.
  - Measure Map's loader-visible public-export name set at the base.
  - Pin that fixed inventory in a targeted test, or compare it with an
    independently captured base on the candidate.
  - Show that an added exported name is refused.
  - Verify trust before and after, separately.
  - Existing Map acceptance stays green.

  The property is no new publication, not the absence of new `pub` lines.
- **AC-4.** Targeted builds only, through `scripts/ken-cargo`. No-regression
  means green in CI.

## Stop conditions

- If the checker cannot carry an arbitrary comparator decision into both
  result views, return the exact stuck goal to the Steward. Do not add a
  fact, factor `dom` or `lookup`, or change any production definition.
- String or Bytes literal identity, a new operation, a publication or a
  catalog extension is not authorized here.
