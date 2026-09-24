---
id: CAT-DEQUE-POP-LAWS
title: "Proof-backfill for Data/Collections/Deque.ken.md, keeping its zero-publication surface: prove in-package for an arbitrary deque that popFront agrees with the list view -- popFront q = None implies toList q = Nil, and popFront q = Some (x, rest) implies toList q = Cons x (toList rest) -- instead of only the pushed-then-popped round trip"
status: merged
owner: foundation
size: S
gate: architect
tier: T1
depends_on: []
blocks: []
github: null
origin: "Architect nomination evt_4pd2scpkcv9x8 at 3120a845c, as the next L3 proof-backfill slice while CAT-ARGPARSE-LAWS (literal identity) and CAT-PARSING-LAWS (BYTES fork) are held for the operator. Operator ruling 2026-09-13: 'schedule the proof backfill before extending the catalog.' Steward-filed per COORDINATION section 2."
---

# Popping an arbitrary deque is unproved

## Settled inputs -- measured at `3120a845c`. Re-ground before acting.

- `Deque a = MkDeque front back` and `toList q = front ++ reverse back`.
  `popFront` takes the head of `front`. If `front` is empty, it takes the
  head of `reverse back` and returns `MkDeque rest Nil`.
- The only pop law is `popFront_pushFront` / `PopPreserves`: popping right
  after a push. Nothing constrains `popFront` on an arbitrary deque.
- Deque publishes **nothing**: no `pub` declarations and no `export`.
  `cat_deque_closeout.rs::deque_loader_visible_inventory_is_empty` pins
  that exactly (Architect `evt_3nt8xtetd2sr7`). This node keeps it.

## Deliverable

One checked in-package law for `popFront`, private like the rest of the
package, over every `a` and every `q`:

- `popFront a q = None` implies `toList a q = Nil`;
- `popFront a q = Some (x, rest)` implies
  `toList a q = Cons x (toList a rest)`.

The Architect's route: split on `front`, then on `reverse back`. The
empty-front case uses the public `list_append::right_unit` under `Cons`.
`popBack` is a later slice.

## Acceptance criteria

- **AC-1 (expressibility first).** Before writing the proof, state the
  result-indexed proposition in Ken inside the package, for example with an
  indexed result view like `PopPreserves`. Return it to the Architect if it
  needs a public surface or any form that cannot live in-package.
- **AC-2 (falsifier).** Two natural-site one-line mutations of `popFront`
  each make the law fail to check: returning `MkDeque rest (Nil a)` in
  the nonempty-front arm, which drops `back`; and returning `None` in the
  nonempty reversed-back arm.
- **AC-3.** No new trust: no `Axiom`, postulate or trust change. The
  zero-publication pin stays exact and unedited. Under `crates/`, only
  Deque harnesses change, to show the law is checked in the loaded
  package and applies to a concrete inhabited deque inside it.
- **AC-4.** Targeted builds only, through `scripts/ken-cargo`. No-regression
  means green in CI.

## Stop conditions

- If the proof needs String or Bytes literal convertibility, factoring of
  production code, a production change to `Deque`, or any publication,
  STOP and return to the Steward. Publishing Deque is catalog extension,
  which is not authorized here. A private proof-local generalization of a
  computed scrutinee such as `reverse a back` is allowed (Steward
  `evt_35hgzdc77kfcc`, Architect `evt_3pm09a42ksj0e`).
