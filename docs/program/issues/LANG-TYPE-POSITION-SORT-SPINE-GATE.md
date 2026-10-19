---
id: LANG-TYPE-POSITION-SORT-SPINE-GATE
title: "The same-constant spine attempt in unify_sorts_walk walks every argument pair before testing any, then restores metas on failure and lets the caller re-walk the reduced form, so a spine that fails at every level costs 2^K: a discarded True-against-False argument nested 20 deep takes 43s against the parent's 0.58s. Gate the attempt on level-erased conversion of each pair, in order, before any walk or snapshot"
status: merged
owner: verify
size: S
tier: T1
gate: architect
depends_on: [LANG-TYPE-POSITION-SORT-CONGRUENCE-FIRST]
blocks: []
github: null
origin: "Adversary M8 finding evt_77eqpt9hkm9ek on 7ac7a5cd1 (LANG-TYPE-POSITION-SORT-CONGRUENCE-FIRST); Architect ruling evt_28kwvc3ghdp7t: a regression to repair now, cost not soundness, by a level-erased conversion gate. Steward-filed per COORDINATION section 2."
---

# The spine attempt is gated on level-erased conversion

## Objective

A same-constant spine that cannot convert costs time linear in nesting
depth in the type-position sort walk, as the parent did, without losing
congruence-first's links.

## Settled inputs (Architect `evt_28kwvc3ghdp7t`, on `7ac7a5cd1`)

- **The mechanism.** `sort_walk_same_constant_spine` (elab.rs ~:12345)
  snapshots metas, walks every argument pair (~:12368), and only then
  tests conversion (~:12375-12382). On failure it restores metas
  (~:12383), and the caller whnf's and re-walks the reduced form, which
  contains the same sibling pair (~:12320). So T(d) = 2·T(d−1). The
  kernel's spine (conv.rs ~:1184-1196) stops at the first failing pair.
- **Why not plain convert-first.** AC-1's `fa_meta` needs the walk to make
  a pair convert: two `Type` arguments carry distinct open level metas
  until the walk links them. This is a prediction; the D0 measures it.
- **The ruled repair.**
  - Before snapshotting or walking, test each argument pair in order with
    kernel conversion on level-erased copies, where every level maps to
    one constant level. Return `false` at the first failing pair, with
    metas untouched.
  - Only if every pair passes, run the existing body unchanged.
  - Sound because the gate is a necessary condition: admitted globals are
    level-closed, so level erasure commutes with β, δ and ι.
- **The enabler.** `fn erase_levels(t: &Term) -> Term` over every
  level-bearing position, as one exhaustive `match` with no `_ =>` arm
  (COORDINATION §7b). `ken_kernel::subst_levels` maps declared parameters
  only, so it is not this helper.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The gate and `erase_levels` in `crates/ken-elaborator/src/elab.rs`, with
the rows below in the congruence-first test file.

## Acceptance

- **AC-0 (D0, run first).** Row P: replacing the gate with conversion of
  the un-erased pair takes `fa_meta` over 2s.
- **AC-1.** At K=20, each under 2s:
  - R1: `fn H (a : Bool) (b : Type 1) : Type 1 = b -> Int` nested with
    `True` against `False`, accepted with no obligation;
  - R2: `Type` against `(Type -> Type)` leaves, refused with the parent's
    K0004.
  - Paired: R1 and R2 exceed 2s on `7ac7a5cd1`, and deleting the gate
    returns R1 over 2s.
- **AC-1s (R1-swap, ratio; Architect `evt_bq2fze6dbp15`, bound restated
  in `evt_7wakdhmgfc1`).** R1 with the parameters swapped, so the failing
  pair comes last. The kernel's own conversion of that pair is
  exponential (13.8s at K=20), so this row is a ratio, not a bound. At
  K=14 and K=16, time full elaboration and a kernel-only `convert_type` of
  the same two towers declared as consts. Full is at most 20× kernel at
  both depths, which is twice the parent's measured 10×, and the K=16
  ratio is at most 1.10× the K=14 ratio. Main fails both clauses (40.3×
  and 45.7×, growth 1.13); the gate-deleted mutant must fail too.
- **AC-2.** AC-1, AC-1b and AC-1c of CONGRUENCE-FIRST stay green, as do
  the nested-domain and up-to-whnf suites. `fallback_suc` keeps its
  landed K=4 acceptance with no obligations, and gains a non-regression
  sentinel, labelled as not gate evidence: at K=12, full is at most 8× the
  kernel-only conversion of the same pair (main is 5.79×).

## Stop conditions

- Any kernel, conversion or `trusted_base()` change.
