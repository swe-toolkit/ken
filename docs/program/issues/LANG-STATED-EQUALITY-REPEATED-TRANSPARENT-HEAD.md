---
id: LANG-STATED-EQUALITY-REPEATED-TRANSPARENT-HEAD
title: "stated_equality gives up when a goal's head chain unfolds the same transparent id twice, so a non-recursive wrapper used twice (Id (Id (Equal ...))) is reported as stating no equality and Refl is refused on every predicate route. Bound the unfolding by recursion, not by revisit"
status: ready
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

D0 first: the Architect rules the termination bound that admits a
repeated non-recursive head and still stops a recursive transparent group.
Then the change in `stated_equality`.

## Acceptance

- **AC-1.** The five refused rows are Ok, on both routes where listed.
- **AC-2 (control).** A recursive transparent group still terminates and
  states no equality. The REFL-GOAL suite, its false twin, the `Both`
  refusal and the message-pinning suites are unchanged.
- **AC-3 (mutation, QA).** Restoring the revisit guard reddens AC-1, and
  removing the termination bound makes the AC-2 recursive row fail to
  terminate within its test.

## Stop conditions

- Any kernel rule, conversion change or `trusted_base()` change.
- The bound needs fuel or a depth that changes another elaborator route's
  verdict: stop to the Architect.
