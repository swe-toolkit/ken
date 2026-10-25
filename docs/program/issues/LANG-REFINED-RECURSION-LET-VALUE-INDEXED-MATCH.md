---
id: LANG-REFINED-RECURSION-LET-VALUE-INDEXED-MATCH
title: "Dependent-result structural recursion on a refined parameter is still refused with K0014 when an un-ascribed let binds a match on an indexed-family value whose arms all return the recursive call. The match lowers through the index-equation convoy to an applied eliminator whose methods take an extra equation binder, and the returned-call walk has no arm for it"
status: active
owner: language
size: S
tier: T1
gate: architect
depends_on: [LANG-REFINED-RECURSION-LET-VALUE-CONTAINS-CALL]
blocks: []
github: null
origin: "Adversary M8 finding evt_5z0grx6bwkkq1 (F1) on LANG-REFINED-RECURSION-LET-VALUE-CONTAINS-CALL (64747cb27): a sibling of the closed shapes. It fails closed, and the K0014 it reports is false. Steward-filed per COORDINATION section 2."
---

# A let-bound indexed match that returns the recursive call

## Objective

With the `lang_refined_recursion_let_bound_call.rs` harness (`AnyNat`,
`R = { y : Nat | Equal Nat y n }`, and `Vec : Nat -> Type`), each step
`let r = (match v { ... }) in Suc r` whose arms all return `g m ...` is
accepted:

- a2: `v : Vec Nat (Suc Zero)` bound by a let, one `VCons` arm (the `VNil`
  arm is impossible);
- c0: parameters `(j : Nat) (v : Vec Nat j)`, both arms;
- c2: c0 plus `(e : Equal Nat j j)`.

## Fixed inputs (main `64747cb27`, Adversary `evt_5z0grx6bwkkq1`)

- **The rows.** All three are refused with K0014 (`NotTerminating`, "SCT:
  idempotent self-loop has no strictly-decreasing parameter") at
  `64747cb27`. Each recurses only on `m < n`.
- **The mechanism, measured by the Adversary with instrumentation that
  was then reverted.**
  - A match on an indexed scrutinee lowers through
    `build_index_equation_convoy_body` (`elab.rs:4786`). The let's value
    is `App(Elim { .. }, [arg])`, an eliminator applied to one convoy
    argument (`refl` in c0 and c2). Each method takes one equation binder
    after its constructor and IH binders. In a2 the `VNil` method is a
    synthesized absurd.
  - `returned_call_refined_args` (`refined_recursion.rs:196`) has no arm
    for an applied eliminator. The value reaches the leaf arm (`:253`),
    which finds an `Elim` head, not a member `Const`, and grants nothing.
  - The `Elim` arm strips exactly constructor-plus-IH lambdas (`:234`),
    so seeing through the spine alone still stops at the equation binder.
    In a2 it would also intersect with the absurd method's empty set.
- **Accepted controls on `64747cb27`:**
  - a1: `Suc (match v { VCons i a xs ↦ g m })`, 3 goals;
  - c1: `Suc (match v { ... })` with c2's parameters, 4 goals;
  - c3: `let r = g m j v e in Suc r`, 3 goals;
  - a0: a2's `let v` followed by `let r = g m in Suc r`, 3 goals.
- **Ruling in force:** the Architect's AC-0 for
  `LANG-REFINED-RECURSION-LET-VALUE-CONTAINS-CALL` (the "returns the call"
  relation), and AC-0 of the parent WP (`evt_3vsjks22y4pnd`): authority
  belongs to a call whose value is in flight, inherited inward and never
  reset.

Treat anchors as perishable. If a fixed input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

The returned-call relation sees through an eliminator applied to its
convoy arguments, and through each method's equation binders, so a
convoy-lowered match whose every possible arm returns a member call
grants that call's refined arguments to the let's type and scope. The
rows join `lang_refined_recursion_let_bound_call.rs`.

## Acceptance

- **AC-0 (route, Architect).** Rule how an absurd method counts (no path,
  or a path that returns nothing), and whether the binder count comes from
  the convoy's recorded shape or from the method's lambdas.
- **AC-1.** a2, c0 and c2 are accepted with obligation counts pinned. The
  four controls and every row already in the test file are unchanged.
- **AC-2.** The kept refusals in the test file stay K0014. Add one more:
  a convoy-lowered let-bound match with one arm returning `g (g m)`.
- **AC-3 (mutation, QA).** Removing the applied-eliminator arm reddens
  the three rows back to K0014.

## Stop conditions

- Any kernel or `trusted_base()` change.
- A changed SCT verdict outside the rows named here.

Checks: 4 fired: `returned_call_refined_args` (:196), the `Elim` arm's
binder strip (:234), the leaf arm (:253), the let's `with_call` (:356)
and `build_index_equation_convoy_body` (`elab.rs:4786`) are on main
`15a1700a2` at the lines cited. 7 fired: AC-2's nested-call row tests
that the new arm does not admit a non-decreasing call. 8 fired: treating
an absurd method as granting everything would also turn a2 green, so
AC-0 rules it before the build.
