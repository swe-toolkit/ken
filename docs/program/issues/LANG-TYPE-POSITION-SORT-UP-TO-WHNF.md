---
id: LANG-TYPE-POSITION-SORT-UP-TO-WHNF
title: "The type-position sort link compares types syntactically, so an Eq endpoint or former argument whose sort is written through a transparent alias, such as const Small : Type 1 = Type 0, or a former with a higher-order domain links no level and is refused with K0004 once an obligation follows. Link levels by unifying both sides in whnf, for every domain shape, in one helper"
status: active
owner: language
size: S
tier: T2
gate: architect
depends_on: []
blocks: []
github: null
origin: "Adversary M8 findings F1 and F2 on b3d6277b7 (evt_6wgm0thq6ef7d); Architect ruling evt_438a6j0bwer46: both real, fail closed, one S erratum on the language ring that does not preempt LANG-REFINEMENT-SUBSET-SIGMA's D0 or LANG-J-RECORDED-EQ-FORMATION. Steward-filed per COORDINATION section 2."
---

# The type-position sort link works up to whnf

## Objective

An unannotated `Eq` binder or former argument whose sort is a transparent
alias, or whose former has a higher-order domain, checks and reports its
obligation, as its `Type 0`-annotated twin does.

## Settled inputs (Architect `evt_438a6j0bwer46`, on `b3d6277b7`)

- **The cause.** `unify_types` (elab.rs ~:761) links levels structurally
  and never reduces. Its fallback is `_ => {}`.
  - **F1.** `unify_eq_endpoint_sort` (~:12275) passes the endpoint's
    inferred type unreduced, so `Type ?l` against `Const Small` links
    nothing. The sibling `unify_type_position_argument_sort` (~:12249)
    does the same with `argument_ty` and never whnf's `domain`.
  - **F2.** The sibling returns early unless `domain` is syntactically
    `Term::Type(_)` (~:12263), so a domain such as `Type -> Type` never
    links.
- **The ruled repair.** One helper, `unify_sorts_up_to_whnf(cx,
  expected, actual)`: zonk `expected`, whnf both sides, then
  `unify_types`. `unify_eq_endpoint_sort` calls it with the carrier and
  the endpoint type. `unify_type_position_argument_sort` deletes the
  `Term::Type` guard and calls it with the domain and the argument type.
  Both sides are whnf'd because a level-polymorphic alias carrier has the
  same gap from the other side. The ruling's code is the text.
- **Repros (Adversary).** Each file opens with
  `fn use5 (x : {v : Int | Equal Int v 5}) : Int = x` and
  `const Small : Type 1 = Type 0`.
  - a1: `fn k (a : Small) (h : Eq Type a a) (n : Int) : Int = use5 n`.
  - a3: `fn k (a : Small) (b : Type) (h : Eq Type a b) (n : Int) : Int =
    use5 n`.
  - a5: `fn k (F : Type -> Type) (a : Small) (o : F a) (n : Int) : Int =
    use5 n`.
  - g3: `fn k (g : (Type -> Type) -> Type) (f : Type -> Type) (x : g f)
    (n : Int) : Int = use5 n`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The helper and the two call-site changes in
`crates/ken-elaborator/src/elab.rs`, with the rows below as a test
target.

## Acceptance

- **AC-1.** a1, a3, a5 and g3 check, each with its one obligation.
- **AC-2 (paired failing configuration).** Each AC-1 row with its helper
  call reverted to `b3d6277b7` gives K0004.
- **AC-3.** The four negatives stay refused, with the alias
  `Big := Type 1` as the Adversary wrote them:
  `(a : Big) (h : Eq (Type 0) a a)`, `(a : Big) (h : Eq Type a Int)`,
  `(F : Type -> Type) (a : Big) (o : F a) (q : F Int)`, and
  `(a : Type 1) (h : Eq (Type 0) a a)`.
- **AC-4.** Every landed catalog package emits byte-identical core.
- **AC-5 (census, not scope).** Run one alias-endpoint row in term
  position, through `infer_eq` and the term-position `unify_types` sites.
  Record it if green on main. If red, report it to the Steward as a
  separate item; do not fold it in.

## Stop conditions

- Making `unify_types` conversion-aware for the term-position sites.
- Any kernel or `trusted_base()` change.
