---
id: LANG-STATED-EQUALITY-REPEATED-TRANSPARENT-HEAD
title: "stated_equality gives up when a goal's head chain unfolds the same transparent id twice, so a non-recursive wrapper used twice (Id (Id (Equal ...))) is reported as stating no equality and Refl is refused on every predicate route. Bound the unfolding by recursion, not by revisit"
status: merged
owner: language
size: S
tier: T1
gate: architect
depends_on: [LANG-REFL-GOAL-ONE-EQUALITY-PREDICATE]
blocks: []
github: null
origin: "Adversary M8 finding evt_6yep3anar4ppe on 437091eaa (LANG-REFL-GOAL-ONE-EQUALITY-PREDICATE): an incomplete fix of the class, fail-closed. Steward-filed per COORDINATION section 2."
---

# Repeated transparent heads reach the stated equality

## Objective

A goal reaches `Equal` through transparent definitions, as
`stated_equality` documents, however many times the chain passes through
the same non-recursive wrapper. Refl gets one answer on every route,
whatever the carrier type.

## Settled inputs (Adversary `evt_6yep3anar4ppe`, at `437091eaa`)

- **The guard.** `stated_equality` (ken-elaborator/src/elab.rs, the
  `Term::Const` arm) returns `None` on `!unfolded.insert(id)`. It is there
  to stop a recursive transparent group from looping, and it cannot tell a
  revisited id from recursion.
- **Repro.** The frame's `MyEq`, plus `const Id : Prop -> Prop = λt. t`
  and `const W2 : Pair Nat Nat -> Pair Nat Nat -> Prop = λp. λq. Id (Id
  (Eq (Pair Nat Nat) p q))`. Each goal has body `Refl`:
  - `Id (Equal (Pair Nat Nat) p p)` and `Id (MyEq p p)` are Ok;
  - `Id (Id (Equal (Pair Nat Nat) p p))` is refused with "Refl expects an
    `Eq`-shaped goal", on the standalone route and in the List arm;
  - `W2 p p` (standalone and List arm) and `Id (Id (MyEq p p))` are
    refused the same way;
  - control: `Id (Id (Equal Nat z z))` is Ok, because the gate's whnf-Eq
    arm never asks the predicate.
- **Cause, measured.** With the guard reduced to a bare insert, all nine
  rows are Ok. Restoring it brings back the five refusals.
- **Fan-in.** Two consumers reach the guard: the standalone `check` Refl
  gate and `simplify_branch_goal_unguarded`.
- **Not a regression.** The base refused these rows too. The REFL-GOAL
  closeout recorded the guard as unexercised; this population exercises
  it.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

D0 ruled (Architect `evt_12z122k4j4266`). The revisit guard keys on the
kernel's cycle predicate: `env.is_recursive_transparent(id) &&
!unfolded.insert(id)`, with no fuel and no depth bound. It terminates
because both consumers run `whnf` first and the walk follows a prefix of
that head reduction. The change is confined to `stated_equality`.

## Acceptance

- **AC-1.** The five refused rows are Ok, on both routes where listed.
  Both false twins are refused for non-convertibility.
- **AC-2 (control).** An elab.rs unit test declares an SCT-admitted
  Prop-valued recursive family `P : Nat -> Prop` through
  `declare_recursive_group`, and asserts that `env.is_recursive_transparent(P)`
  holds and `stated_equality` returns `None` on `P k`, for a neutral `k`
  and for `Suc Zero`. The REFL-GOAL suite, its false twin, the `Both`
  refusal and the message-pinning suites are unchanged.
- **AC-3a (mutation, QA).** Restoring the bare `!unfolded.insert(id)`
  guard reddens all five AC-1 rows.
- **AC-3b (recorded, not red).** Deleting the guard line leaves the AC-2
  recursive row terminating. Record it as evidence that the guard is
  defensive: no head-only cycle is constructible through the public API.

## Stop conditions

- Any kernel rule, conversion change or `trusted_base()` change.
- Any AC-1 row stays refused.
- Either false twin or the `Both` refusal checks or changes its reason.
- The REFL-GOAL suite or the message-pinning suites change.

## Closeout

Merged `cc2d62abe` from exact `77d67584d` (PR #4700). Language QA
`evt_14hqa85c3482j`; Architect `evt_3f3nft5xc67rd` with Decision
`dec_3chrbm1bvgkra`.

- `stated_equality` keys its revisit guard on the kernel's checked cycle
  set, `env.is_recursive_transparent(id) && !unfolded.insert(id)`, so a
  non-recursive wrapper used twice states its equality and a recursive
  transparent group still stops. No fuel and no depth bound.
- `lang_stated_equality_repeated_transparent_head.rs` holds the five
  formerly refused rows, controls and false twins (8/8), and a unit test
  covers a recursive `P : Nat -> Prop`.
- L2 moves to `LANG-NESTED-INDEX-MATCH-PROPOSITION-GOAL`.
