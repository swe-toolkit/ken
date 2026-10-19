---
id: LANG-TYPE-POSITION-SORT-SPINE-GATE-NECESSITY
title: "SPINE-GATE's level-erased gate is not a necessary condition: delta-unfolding a level-closed alias restores its literal levels, so const Big : Type 2 = Type 1 against Type ?m is refuted although ?m := 1 converts. A discarded-result declaration accepted before the gate is now refused at K=2, and three alias rows are back on 2^K growth (14.5s to 21.9s at K=16). Gate only meta-free pairs, and memoize failed spine attempts within one walk"
status: ready
owner: verify
size: S
tier: T1
gate: architect
depends_on: [LANG-TYPE-POSITION-SORT-SPINE-GATE]
blocks: []
github: null
origin: "Adversary M8 finding evt_avnqpdcy3tr on 6f44c4dbf + bcde6b31e (LANG-TYPE-POSITION-SORT-SPINE-GATE); Architect ruling evt_4grwysh404yp: real, completeness and cost not soundness, elaborator-only repair, verify ring. Steward-filed per COORDINATION section 2."
---

# The spine gate is necessary by construction

## Objective

The type-position sort walk accepts every declaration the parent of
SPINE-GATE accepted, including aliases for universes above `Type 0`, and
keeps a failing same-constant spine linear in nesting depth.

## Settled inputs (Architect `evt_4grwysh404yp`, on `bcde6b31e`)

- **The defect.** `erase_levels` zeroes only the levels written in an
  argument. δ-unfolding a level-closed global puts its literal levels
  back, so the gate in `sort_walk_same_constant_spine` (elab.rs ~:12453)
  refutes pairs that a level link would convert.
- **Measured on `bcde6b31e`**, with the Adversary's fixtures
  (`fn T3 (n : Nat) (a : Type 2) : Type 2`, `fn D3 ... : Type`, both
  matching on `blow n`; `const Big : Type 2 = Type 1`):
  - big_discard is refused with TypeMismatch at K=2;
  - big_arg, big_arg_rev and big_eq take 14.9s, 14.7s and 21.9s at K=16;
  - small_arg (alias at `Type 0`) is green, which is why the landed rows
    miss it.
- **The ruled repair; the ruling's code is the code.**
  - (a) Inside the loop, test a pair with `convert_type` only when
    `sort_walk_is_inert` holds for it after zonking. A pair with no open
    level meta is untouched by any link, so the final check would refuse
    it the same way. Delete `erase_levels`, which then has no caller.
  - (b) Thread a `FailedSpines` set, `HashSet<(Vec<Term>, Term, Term)>`
    keyed on (`ctx.types`, zonked expected, zonked actual), through
    `unify_sorts_walk` and `sort_walk_same_constant_spine`. Skip a spine
    attempt whose key already failed. A failed attempt restores
    `metas.metas = saved`, and nothing else is written, so the memo
    cannot change a verdict.
  - (a) alone takes R2 to 51.36s at K=20; (a) and (b) together give R2
    39ms, R1-swap 10.13× at K=16, every alias row 0-1ms, and 11/11 on the
    existing suite.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The repair in `crates/ken-elaborator/src/elab.rs`, with the rows below in
`crates/ken-elaborator/tests/lang_type_position_sort_congruence_first.rs`.

## Acceptance

- **AC-1.** big_discard is accepted at K=2 with no obligations. big_arg,
  big_arg_rev and big_eq each finish in under 2s at K=16. big_wrong
  (`T3 NN Big` against `T3 NN (Type -> Type)`) is refused, with no timing
  assertion. The existing suite stays green.
- **AC-2 (controls).**
  - Restoring the erased gate makes big_discard refuse at K=2.
  - Dropping the memo, so every attempt runs, takes R2 over 2s.
- **AC-3 (subsume).** Drop gate (a) entirely and keep the memo. Report
  R1, R1-swap and R2. If all three stay green, delete the gate rather
  than keep two mechanisms.

## Stop conditions

- Any kernel change, or any change to what the kernel accepts: stop to
  the operator.
- A verdict that differs from the parent of SPINE-GATE (`7ac7a5cd1`) on
  any existing row: stop with the row.
