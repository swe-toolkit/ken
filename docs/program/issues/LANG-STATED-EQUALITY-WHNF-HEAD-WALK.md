---
id: LANG-STATED-EQUALITY-WHNF-HEAD-WALK
title: "stated_equality still answers by carrier: a true Refl goal at a Pair carrier is refused when its head chain passes a recursive id twice without a cycle, or passes a match or let that whnf reduces through, while the Nat twins check. Make the walk follow the head reduction whnf already takes"
status: active
owner: language
size: S
tier: T1
gate: architect
depends_on: [LANG-STATED-EQUALITY-REPEATED-TRANSPARENT-HEAD]
blocks: []
github: null
origin: "Adversary M8 finding evt_51rmh23m2r1kc on cc2d62abe (LANG-STATED-EQUALITY-REPEATED-TRANSPARENT-HEAD): an incomplete fix of the class, fail-closed. Check 7 (sibling of the ruled criterion). Steward-filed per COORDINATION section 2."
---

# stated_equality follows the head reduction whnf takes

## Objective

A goal reaches `Equal` by head reduction, and Refl gets one answer on every
route whatever the carrier type, as the predecessor's objective states.

## Settled inputs (Adversary `evt_51rmh23m2r1kc`, at `cc2d62abe`)

- **Finding 1: a recursive id that recurs without a cycle.**
  - The fixture: `const K : Prop -> Prop -> Prop = λa. λb. a` and
    `fn R (p : Prop) (n : Nat) : Prop = K p (match n { Zero ↦ Top; Suc k
    ↦ R p k })`. R is SCT-admitted, and `is_recursive_transparent(R)`
    holds. Its head is K, which returns `p`.
  - Refused with "Refl expects an `Eq`-shaped goal", at `(p q : Pair Nat
    Nat)`:
    - `R (R (Equal (Pair Nat Nat) p p) Zero) Zero`;
    - `R (R (Equal (Pair Nat Nat) p p) z) z`, standalone and in a List
      arm.
  - Controls that check: one visit, `Id (R (Id (Equal ..)) z)`, and the
    Nat carrier `R (R (Equal Nat z z) z) z`.
  - The locus is elab.rs:4772, `env.is_recursive_transparent(id) &&
    !unfolded.insert(id)`. With the guard disabled, the three rows check
    and the false twin `R (R (Equal (Pair Nat Nat) p q) z) z` moves to
    non-convertibility.
- **Finding 2: the walk stops at `Elim` and `Let` heads.**
  - `fn PE (b : Bool) (p : Pair Nat Nat) : Prop = match b { True ↦ Equal
    (Pair Nat Nat) p p; False ↦ Top }`: `PE True p` is refused
    standalone, in a List arm, and at the True arm of `match b`.
  - `const LetEq = λp. let x = p in Equal (Pair Nat Nat) x x`:
    `LetEq p` is refused standalone and in a List arm.
  - The Nat twins `PN True z` and `LetN z` check.
  - The locus is the catch-all arm at elab.rs:4789. Only Const, Lam and
    Ascript heads advance. The doc comment lists `Elim` as stating no
    equality; `Let` is not on that list.
- **No soundness defect, no regression.** All rows fail closed, the kernel
  re-checks every proof, and `trusted_base()` is unchanged. The base
  refused the same rows.
- **The predecessor's AC-3b** recorded that no head-only cycle is
  constructible through the public API. The termination argument already
  bounds the walk by the caller's `whnf`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

D0 first. The Architect rules one rule for the walk that covers both
findings, for example reading the head after the consumer's own `whnf`
step, or advancing through every head `whnf` reduces through, and whether
the revisit guard survives. Then the repair, confined to
`stated_equality` and its two consumers.

## Acceptance

- **AC-1.** The five Finding 1 and Finding 2 refused rows check on every
  listed route. Both false twins are refused for non-convertibility.
- **AC-2 (control).** The predecessor's suite
  `lang_stated_equality_repeated_transparent_head.rs` and its recursive
  `P : Nat -> Prop` unit test are unchanged. The REFL-GOAL suite and the
  message-pinning suites are unchanged.
- **AC-3 (mutation, QA).** Restoring the ruled guard or the Const/Lam/
  Ascript-only advance reddens the AC-1 rows it covers.

## Stop conditions

- Any kernel rule, conversion change or `trusted_base()` change.
- A false twin checks, or the recursive `P` unit test stops terminating.
- The repair needs a change outside `stated_equality` and its two
  consumers.
