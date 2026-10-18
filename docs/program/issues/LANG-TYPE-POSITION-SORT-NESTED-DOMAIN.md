---
id: LANG-TYPE-POSITION-SORT-NESTED-DOMAIN
title: "The type-position sort link whnf's only the head of each side, so an alias nested inside a structured former domain, such as g : (Small -> Type) -> Type with Small := Type 0, links no level and the argument is refused with TypeMismatch, while its unaliased twin checks. Link sorts by walking both sides structurally, with whnf at each node"
status: active
owner: language
size: S
tier: T2
gate: architect
depends_on: [LANG-TYPE-POSITION-SORT-UP-TO-WHNF]
blocks: []
github: null
origin: "Architect exact-SHA review of LANG-TYPE-POSITION-SORT-UP-TO-WHNF at 749f99960 (evt_1tp17m3dsgqcj): residual n1, a gap in the ruling rather than the candidate, filed as its own erratum and not folded in. Relayed by verify-leader evt_24jepxyn438m2. Steward-filed per COORDINATION section 2."
---

# The type-position sort link reaches nested domains

## Objective

A former argument whose domain carries a transparent alias below its
head checks and reports its obligation, as its unaliased twin does.

## Settled inputs (Architect `evt_1tp17m3dsgqcj`, on `749f99960`)

- **The gap.** `unify_sorts_up_to_whnf` zonks `expected`, whnf's both
  sides at the head, and calls `unify_types`. `unify_types` (elab.rs
  ~:761) recurses Pi, App and Lam structurally, links levels only, and
  never reduces. So `Const Small` under a Pi domain is never unfolded and
  links nothing.
- **Measured on the candidate** (each row after `use5` and
  `const Small : Type 1 = Type 0`):
  - n2: `(F : Small -> Type) (a : Type) (o : F a)` checks, 1 obligation.
  - n0: `(g : (Type -> Type) -> Type) (f : Type -> Type) (x : g f)`
    checks, 1 obligation.
  - n1: `(g : (Small -> Type) -> Type) (f : Type -> Type) (x : g f)` is
    refused with `TypeMismatch { expected: (Π g648. Type u3), found:
    (Π Type u2. Type u3) }`.
- **The ruled repair.** Link sorts by walking both sides structurally,
  with whnf at each node and the context extended under Pi and Lam
  binders, instead of whnf at the head only. The walk serves the two
  type-position callers of `unify_sorts_up_to_whnf`. `unify_types` and the
  term-position sites stay byte-unchanged.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The structural walk in `crates/ken-elaborator/src/elab.rs`, with the
rows below as a test target beside the SORT-UP-TO-WHNF rows.

## Acceptance

- **AC-1.** n1 checks with its one obligation. A twin with the alias
  one level deeper, `(g : ((Small -> Type) -> Type) -> Type)` over a
  matching `f`, checks too. n0 and n2 are unchanged.
- **AC-2 (paired failing configuration).** Each AC-1 row with the walk
  replaced by the head-only whnf gives today's `TypeMismatch`.
- **AC-3.** The SORT-UP-TO-WHNF rows a1, a3, a5, g3 and its four
  `Big := Type 1` negatives keep their outcomes. For the nested `Big`
  twin of n1 (Architect `evt_4wf8nv27wwrqy`, `evt_3031bnxwcrzq9`; operative
  `evt_9aq3qzbwfdb5`):
  - With `f : Type 0 -> Type`, it gives K0004 on base and candidate.
  - With a bare `f : Type -> Type`, it checks with one obligation, and
    its admitted kernel type shows `f`'s domain solved to `Type 1`. A
    bare `Type` is `Type ?l`, so the base refusal was incidental to the
    missing level link. Reverting the walk returns this row to K0004.
- **AC-4.** Every landed catalog package emits byte-identical core.
- **AC-T1 (amendment 1, durable timing; Architect `evt_brr6pry2jfxy` on
  Adversary `evt_c3e61b3kpxd3`).** The walk returns early through
  `sort_walk_is_inert` before any whnf when the zonked sides are
  identical or carry no open level meta (an out-of-range meta counts as
  open). The ruling's code is the text. With `T NN` an alias over an
  exponential `blow` at K=16, rows fa, eq, nested and fa_ne (as in the
  ruling) each elaborate in under 2s. Paired: deleting only the early
  return makes each exceed 2s.
- **AC-T2.** Under the guard every row above keeps its outcome.

## Stop conditions

- Any change to `unify_types` or a term-position site.
- Congruence-first linking, or any change to which levels are linked.
- A walk that needs to assign a term meta, not only link levels.
- Any kernel or `trusted_base()` change.
