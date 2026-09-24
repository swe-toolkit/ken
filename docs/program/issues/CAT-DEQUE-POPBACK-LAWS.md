---
id: CAT-DEQUE-POPBACK-LAWS
title: "Proof-backfill for Data/Collections/Deque.ken.md, keeping its zero-publication surface: prove in-package for an arbitrary deque that popBack agrees with the list view -- popBack q = None implies toList q = Nil, and popBack q = Some (x, rest) implies toList q = list_append (toList rest) (Cons x Nil)"
status: active
owner: foundation
size: S
gate: architect
tier: T1
depends_on: [CAT-DEQUE-POP-LAWS]
blocks: []
github: null
origin: "Steward-filed runway successor to CAT-DEQUE-POP-LAWS, which the Architect scoped as the popFront slice with popBack later (evt_4pd2scpkcv9x8). Operator ruling 2026-09-13: 'schedule the proof backfill before extending the catalog.' Steward-filed per COORDINATION section 2."
---

# Popping the back of an arbitrary deque is unproved

## Settled inputs -- measured at `8e3bfd479`. Re-ground before acting.

- `toList q = front ++ reverse back`. `popBack` takes the head of `back`.
  If `back` is empty, it takes the head of `reverse front` and returns
  `MkDeque Nil rest`.
- The only back-pop law is `popBack_pushBack`. Deque publishes nothing, and
  `deque_loader_visible_inventory_is_empty` pins that. This node keeps it.
- The empty-back case needs reverse involution. In
  `Data/Collections/Derived.ken.md`, `proof involutive for reverse` is
  checked but **private**. The standalone theorem `reverse_snoc` is also
  private and is not needed (Architect `evt_5tmwmhc8fqmry`).
- The `popFront` slice's private generalization and its use of public
  `list_append` laws are the pattern to follow.

## Deliverable

One checked in-package law for `popBack`, private like the rest of the
package, over every `a` and every `q`, stated as implications:

- `popBack a q = None` implies `toList a q = Nil`;
- `popBack a q = Some (x, rest)` implies
  `toList a q = list_append a (toList a rest) (Cons a x (Nil a))`.

**One provider publication allowed.** Change only `proof involutive for
reverse` to `pub proof involutive for reverse`. Its statement, body,
canonical identity and trust stay unchanged. Import it canonically; do not
write a local copy. `reverse_snoc` stays private and unchanged.

## Acceptance criteria

- **AC-1 (expressibility first).** State the result-indexed proposition in
  Ken inside Deque before proving it. Name any Derived law it needs.
- **AC-2 (falsifier).** Two natural-site one-line mutations of `popBack`
  each make the law fail to check: returning `MkDeque front (Nil a)` in the
  nonempty-back arm, which drops `rest`; and returning `None` in the
  nonempty reversed-front arm.
- **AC-3.** No new trust. Deque's zero-publication pin stays exact and
  unedited. Derived's directional publication pins gain exactly one
  attached law, `reverse::involutive`. Deque's provider census gains
  exactly the identities it uses.
- **AC-4.** Targeted builds only, through `scripts/ken-cargo`. No-regression
  means green in CI.

## Stop conditions

- If the proof needs literal convertibility, factoring of production code,
  a production change to `Deque` or `reverse`, a new provider function, or
  any Deque publication, STOP and return to the Steward. If `involutive`
  is not enough, return the exact goal for a new ruling; do not publish
  anything else. A private proof-local generalization of a computed
  scrutinee is allowed.
