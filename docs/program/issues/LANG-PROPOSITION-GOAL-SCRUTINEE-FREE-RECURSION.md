---
id: LANG-PROPOSITION-GOAL-SCRUTINEE-FREE-RECURSION
title: "LANG-NESTED-INDEX-MATCH-PROPOSITION-GOAL routes every Omega goal through the equation convoy, so a recursive nested match whose propositional goal does not mention the inner scrutinee, which checked on f1ae73cc5, now fails with an Internal. Restore those proofs without losing the rows that WP admitted"
status: active
owner: language
size: S
tier: T1
gate: architect
depends_on: [LANG-NESTED-INDEX-MATCH-PROPOSITION-GOAL]
blocks: []
github: null
origin: "Adversary evt_6n4z79f2f8zvj on 033c19fa7: a completeness regression against f1ae73cc5. Trust is unchanged and false twins are still refused. Operator 2026-09-25 lane L2. Steward-filed per COORDINATION section 2."
---

# Scrutinee-free propositional goals check again

## Objective

Every program in the population below that checks on `f1ae73cc5` checks
on the repair, and the rows that `LANG-NESTED-INDEX-MATCH-PROPOSITION-GOAL`
admitted stay accepted. No elaboration path in this population reaches the
user as an `Internal`.

## Settled inputs (Adversary `evt_6n4z79f2f8zvj`, on `033c19fa7`)

- **The mechanism, measured.**
  - `goal_is_proposition` (elab.rs ~:4744) sets
    `has_nontrivial_coupled_sides` for every `Omega` goal, so a nested
    frame takes `equation_convoy` (~:4482).
  - The recursive arm reaches `check_large_convoy_recursive_arm`, which
    returns `None` at the `!scrut_occurs(expected_here, &stable_forward)`
    guard (~:6951). A goal that does not mention the convoyed scrutinee has
    no transport.
  - `build_large_convoy_recursive_method` (~:7431) turns that `None` into
    `Internal("large index convoy could not construct its recursive goal
    transport")`.
  - The Adversary tagged all eight early `return Ok(None)` sites in the
    function, and only :6951 fired.
- **Minimal repro**, which is Ok on `f1ae73cc5` and `Internal` on
  `033c19fa7`:

  ```
  data Vec (a : Type) : Nat -> Type where { VNil : Vec a Zero; VCons : (n : Nat) -> a -> Vec a n -> Vec a (Suc n) }
  theorem t (a : Type) (b : Type) (n : Nat) (xs : Vec a n) (ys : Vec b n) : Top =
    match xs { VNil ↦ Proved; VCons k x r ↦ match ys { VCons _ y r2 ↦ t a b k r r2 } }
  ```

- **The probe population.** These are Ok on the base and `Internal` on the
  head:
  - `Top` (Dual and Vec);
  - a constant `pp : Prop` with `h : pp` (Dual and Vec);
  - `same_len n n`;
  - a motive over `i` only;
  - `Equal Nat Zero Zero` closed by `Proved`.

  The `Bottom` false twin goes from a kernel `TypeMismatch` to an
  `Internal`. The controls stay Ok: `Not Bottom`, and non-recursive step
  arms. The full row text is in the finding.
- **Gains to keep.** Each of these moves to Ok on the head with trust
  unchanged:
  - a motive over `dual_lookup`;
  - `same_len` over `dual_lookup`;
  - an `And` of two motives;
  - the merged WP's four rows, including RECURSIVE-STEP's T2.
- **Not this WP.** RECURSIVE-STEP's step-reachable control already failed
  on base. Its fix is in `LANG-RECURSIVE-STEP-COORDINATE-COHERENCE`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The D0 is ruled (Architect `evt_mgkvvs0f31b7`, part 1). In
`plan_coherent_frame_motive` (elab.rs ~:4469), `has_nontrivial_coupled_sides`
keeps the syntactic-Eq route and convoys any other proposition only when
the zonked goal mentions the scrutinee (`scrut_occurs`, ~:2668). The ruling
has the exact text. On a prototype without the zonk, G1, G2, G3 and G5 went
Ok, the `Bottom` twin G4 went to `KernelRejected TypeMismatch`, the E and H
rows and K1/K3 were unchanged, and the three suites stayed at 19/19.

The repair is that gate, with the zonk, measured as written. Add a
regression test file that pins G1-G5, every population row above, its
false twin, and the merged WP's rows.

## Acceptance

- **AC-1.** Every base-Ok row in the population checks, with exact
  `trusted_base()` identity.
- **AC-2.** The `Bottom` false twin is refused with a typed error, not an
  `Internal`. The two controls stay Ok.
- **AC-3 (control).** The gains above, and
  `lang_nested_index_match_proposition_goal.rs`, stay green.
- **AC-4 (mutation, QA).** Reverting the repair returns the minimal
  repro to the `Internal`.

## Stop conditions

- Any kernel, conversion or `trusted_base()` change.
- A row that is accepted on `033c19fa7` is refused, or changes diagnostic
  class.
- A catalog census file changes.
