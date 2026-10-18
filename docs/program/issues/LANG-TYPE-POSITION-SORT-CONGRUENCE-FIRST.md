---
id: LANG-TYPE-POSITION-SORT-CONGRUENCE-FIRST
title: "The type-position sort walk whnf's both sides whenever an open level meta is present, so a former argument over an expensive alias applied to a bare Type, such as T2 NN Type, still takes exponential time after the inert-case guard. Link congruence-first: unify a same-head spine, and reduce only on a head mismatch"
status: active
owner: language
size: S
tier: T1
gate: architect
depends_on: [LANG-TYPE-POSITION-SORT-NESTED-DOMAIN]
blocks: []
github: null
origin: "Architect ruling evt_brr6pry2jfxy on Adversary M8 finding evt_c3e61b3kpxd3 (53e5752e2): residual fa_meta, not closed by the inert-case guard folded into LANG-TYPE-POSITION-SORT-NESTED-DOMAIN; a linking-semantics change, so its own node. Steward-filed per COORDINATION section 2."
---

# The sort walk reduces only on a head mismatch

## Objective

A type-position argument whose sort carries an open level meta under an
expensive alias elaborates in time independent of the alias's reduction
cost, as its meta-free twin does under the inert-case guard.

## Settled inputs (Architect `evt_brr6pry2jfxy`, at `d774e9d05`)

- **The residual (fa_meta).** `fn T2 (n : Nat) (a : Type 1) : Type 1 =
  match blow n { Zero |-> a; Suc m |-> a }`, then
  `(F : T2 NN Type -> Type) (x : T2 NN Type) (o : F x)`. Each bare
  `Type` mints its own open meta, so the sides are neither identical nor
  meta-free and the guard does not skip them. At K=16: helper off
  3.5ms, `d774e9d05` 22.1s, inert guard 15.2s.
- **The direction.** Unify a same-head spine first and δ-reduce only on
  a head mismatch. This links level metas inside arguments that δ might
  discard, so it changes which levels are linked and needs its own
  completeness rows.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Acceptance

- **AC-0 (D0, Architect).** Rule the linking order and the completeness
  rows: which links congruence-first adds that whnf-first would not, and
  whether any is unsound to keep when δ would have discarded it.
- **AC-1.** fa_meta at K=16 elaborates in under 2s, with a paired
  whnf-first configuration that exceeds it.
- **AC-0 closed, build-go** (Architect D0 `evt_h497bj8yv4jk`; amendment
  1, operative `evt_6jh2j9sz1zf99`).
- **AC-1b.** fa_meta_ne, discard, small_arg and eq_meta (the D0's
  fixtures) at K=16 each elaborate in under 2s. Paired: deleting the
  same-constant fast return makes each exceed 2s.
- **AC-1c (Architect `evt_11dta2bhe2ktt`, Adversary
  `evt_4kp6rb0add1bz`).** bare_z at K=20 elaborates in under 2s:
  `fn D (u : Nat) (a : Type 1) : Type 1 = a -> a`, `const z : Nat = Zero`,
  and `fn k (F : <D Zero nested K deep around Type> -> Type)
  (x : <D z nested K deep around Type>) (o : F x) (n : Int) : Int = n`.
  Paired: deleting the same-constant early return makes it exceed 2s.
- **AC-C.** discard accepts with no obligations, fallback_suc at K=4
  accepts on the restore path, and the fixed `Type 0` nested Big row
  stays K0004.
- **AC-2.** The SORT-UP-TO-WHNF and SORT-NESTED-DOMAIN rows keep their
  outcomes, and the catalog emits byte-identical core.

## Stop conditions

- Any change to `unify_types` or a term-position site.
- Any kernel or `trusted_base()` change.
