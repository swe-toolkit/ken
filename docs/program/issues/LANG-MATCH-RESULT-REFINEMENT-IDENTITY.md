---
id: LANG-MATCH-RESULT-REFINEMENT-IDENTITY
title: "A match through the nested-pattern matrix or the indexed dependent-branch path checks its leaves against an inferred or δ-simplified substitute for the result type, so a refinement result is dropped even at the top level: fn g (n : Nat) (i : Ix n) (x : Int) : Five = match i { Z xs ↦ x; S ys ↦ 6 } checks with zero obligations and returns 6 as a Five. Check every leaf against the result type as written, under substitution only"
status: active
owner: language
size: M
tier: T1
gate: architect
depends_on: []
blocks: [LANG-NAMED-REFINEMENT-TYPE-ARGUMENT]
github: null
origin: "Architect ruling evt_6qq1wtsekh42j on LANG-NAMED-REFINEMENT-TYPE-ARGUMENT hard stop 2 (evt_5c3dr0890ywyy): a main-branch forgery, recut out as a prerequisite that lands first. Fails open: a program the refinement forbids is accepted. No kernel impact: predicates are erased and the kernel term stays well-typed. Steward-filed per COORDINATION section 2."
---

# Every match leaf is checked against the result type as written

## Objective

Every match leaf and branch body is checked by `check` against the match's
result type as written, instantiated per branch by substitution only, so a
refinement result introduces its predicate on every route.

## Settled inputs (Architect ruling `evt_6qq1wtsekh42j`, measured on `f5d6d7f54`)

- **The rows.** Header `def Five = { n : Int | Equal Int n 5 }`, plus the
  stop report's `Ix`.
  - q5: `fn g (n : Nat) (i : Ix n) (x : Int) : Five = match i { Z xs ↦ x;
    S ys ↦ 6 }` exits 0 with **0** obligations; `g (Suc Zero) (S Nil) 0`
    evaluates to 6.
  - q4: `fn g (x : Int) (ys : List Bool) : Five = match ys { Nil ↦ x;
    Cons True t ↦ x; Cons False t ↦ x }` exits 0 with 0.
  - pa (nested patterns, leaves `x : Int`) and pb (`Ix` match, leaves `x`)
    at `: Five` give 0.
  - Controls give the expected counts: flat `match ys { Nil ↦ x; Cons a t ↦
    x }` gives 2, `if b then x else x` gives 2, flat `match b { True ↦ x;
    False ↦ x }` gives 2, and `let y : Int = x in y` gives 1.
- **The predicate.** The match compiler checks a leaf against a substitute
  for the result type, so the refinement identity never reaches `check` or
  `emit_refinement_introduction`. The two known substitutes:
  - **Dependent branch.** The goal is reduced by `whnf` /
    `simplify_branch_goal` (`elab.rs:4253`). The `names_source_refinement`
    rescue (`:7137`) fires only while `expected_here` is still a bare
    `Const`.
  - **Matrix leaf.** `compile_match_leaf` (`:20521`) infers the first leaf
    under a seeded result (`:21594`); later leaves check against that
    inferred type.
- **Lift arm and structured method (scope ruling `evt_177wxkyv85trf`).**
  `simplify_branch_goal` (`:4253`) runs `whnf` at every depth and has seven
  call sites: `:4043`, `:5411`, `:5583`, `:5722`, `:5792`, `:7094` and
  `:7775`. The lift arm (`:5411`, checked at `:5426`) and the structured
  constructor method (`:5583`, checked at `:5598`) check against the
  simplified goal, and store it as `refined_target` (`:5420`, `:5592`; read
  at `:6400`). Header `data DRose : Type where { DLeaf : DRose ; DNode :
  List DRose -> DRose }`:
  - r0 `fn r0 (r : DRose) (x : Int) : Five = match r { DLeaf ↦ x; DNode
    kids ↦ x }` gives 1.
  - r1 is r0 with `DNode kids ↦ match kids { Nil ↦ x; Cons k rest ↦ x }`,
    and gives 1.
  - r2 is r1 with inner leaves `Nil ↦ 6; Cons k rest ↦ 7`. It gives 1, so 6
    and 7 are returned as `Five` with no obligation.
  - r3 `fn r3 (r : DRose) : Five = match r { DLeaf ↦ 6; DNode kids ↦ 7 }`
    gives 1.
- **The comment at `:2307`** ("each branch has already been checked against
  its result") is false on these routes.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

1. **D0 (measure only).** Before any product edit, a disposable probe at the
   two seams counts the leaves whose as-written result is refinement-rooted
   while the checked type is not, by owner, over the prelude, the 81 roots
   and the 55 `catalog/packages`. The Architect judges any moved corpus row;
   none is rebaselined silently. The probe also records, at each of the
   seven `simplify_branch_goal` call sites, the count of refinement-rooted
   `Const` subterms in the input and in the output, by site and owner, over
   the same population plus r0 to r3. Any nonzero loss outside the probes
   goes to the Architect before the edit.
2. **The closure.**
   - With a seeded result slot, every leaf, the first included, goes through
     `check(cx, leaf, seed)`.
   - The branch goal is the source expected type with the scrutinee and
     indices substituted, computed by `subst` with no `whnf` or δ. Where goal
     simplification must expose structure (index equations, convoy
     premises), a refinement-rooted `Const` head stays rigid at every depth.
   - At the lift arm (`:5411`) and the structured method (`:5583`), the
     checking goal and `refined_target` are `subst_term_generalize` of the
     weakened expected type, with no `simplify_branch_goal`. The `:7098`
     `refined_target` write stores `expected_here`, unless D0 shows it is
     unread. A corpus row that needs the simplified form is a stop; do not
     reinstate simplify.
   - A sweep by mechanism, starting from `check_match_arm_result`,
     `compile_match_leaf`, `infer_match*`, `check_match_dependent`,
     `refined_target` and `ret_ty_slot`, states for each site where an arm,
     leaf or branch body is elaborated what type it checks against, and
     whether that is the as-written result under substitution. `:4043` and
     `:7775` appear in it, each with what it feeds and whether a value is
     checked against it.

## Acceptance

Every row with `ken check`, base versus candidate.

- **AC-1 (routes).** Flat non-indexed, nested patterns, flat indexed,
  indexed with a nested `match` body, `match ... eqn:`, a match under result
  predicates, a recursive-group call result, a match in an `if` branch, and
  a `let`-bound match. At a bare `Five` result each leaves one obligation per
  leaf, including q4, q5, pa and pb. Base then candidate: r0 1 to 2, r1 1
  to 3, r2 1 to 3, r3 1 to 2.
- **AC-2 (controls).** The four control counts above are unchanged. Corpus
  and catalog rows move only as D0 reported and the Architect judged.
- **AC-3 (mutation, QA).**
  - M-first-leaf (infer the first leaf again) returns q4 and pa to 0.
  - M-goal-whnf (unfold refinement `Const`s in goal simplification) returns
    q5 and pb to 0.
  - M-structured-simplify (simplify again at `:5583` only) returns r3 to 1.
  - M-lift-simplify (simplify again at `:5411` only) returns r2 to 1. If
    r2's inner match does not reach `:5411`, use a D0 row that does, or
    record "not expressible" with the D0 count. A mutant that does not
    redden is a stop.

The `List Five` result on each route belongs to
`LANG-NAMED-REFINEMENT-TYPE-ARGUMENT`, refused by its guard once both land.

## Stop conditions

- A swept site that checks against anything other than the as-written result
  under substitution, and is not repaired here: stop with the site.
- A corpus or catalog row that moves outside what D0 reported.
- Any kernel, `trusted_base()` or spec change.
