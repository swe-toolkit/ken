---
id: RT-C5-COMPOUND-HIGHER-ORDER-INDICES
title: "C5 canonicity for compound and higher-order indices: a closed, checked, reflexive cast over a Sigma-indexed or function-indexed family evaluates to its value instead of Unknown, by a rule whose soundness is argued before it is built"
status: ready
owner: runtime
size: M
gate: architect
tier: T1
depends_on: [RT-C5-PRIMITIVE-TYPE-ARGUMENTS]
blocks: []
github: null
origin: "Architect ruling 2026-09-28 (evt_7nf6ds3er67t1) on RT-C5-PRIMITIVE-TYPE-ARGUMENTS QA block evt_3azsc8dd9vgne: the next consumer of the same kind (Check 7) is out of that WP's scope and must be placed as its own node. Spec 40-runtime/42-evaluation.md §3.6 (C5 canonicity for every closed, well-typed, ground term). Steward-filed per COORDINATION section 2."
---

# C5 on compound and higher-order indices

## Objective

A closed, well-typed, ground cast `cast A A refl a` evaluates to `a` when
`A` is indexed by a pair or a function, as spec 42 §3.6 requires.

## Settled inputs (Architect `evt_7nf6ds3er67t1`, read at `ba9cdda70`)

- **The gap.** `eq_type_eq` in `crates/ken-interp/src/eval.rs` cannot
  compare closures. So a closed, checked, reflexive cast over
  `F (f : Nat → Nat)` falls to `_ => false`, and `cast_reduce` returns
  `Unknown`. A `Pair` arm has landed (eval.rs:1475-1482 at `1a84b2378`),
  so a Σ index with comparable components already reduces. A Σ index with a
  function component fails through it as the function case (AC-0,
  `evt_5wt6v7dyzxehf`).
- **No value-comparison arm can close the function case.** Spec 42 §1:
  "runtime closures are not compared by representation or identity".
- **The Architect's candidate is a proof-directed C5 rule**: a canonical
  `refl` proof licenses the cast. Whether that is sound for proof values
  under Ω proof irrelevance is an open question. It is not ruled.

## Deliverable

`cast_reduce` returns `a` for every closed, checked, reflexive cast whose
index carries a function value, bare, under `Pair` or in the All-support
witness's predicate slot. The rule for that is argued sound at D0 and ruled
by the Architect. An open index still fails closed.

## Acceptance

- **AC-0 (probe, then D0; no build).**
  - Done at `1a84b2378` (Steward correction `evt_5z1qfsy58wva6`; probe
    `evt_5wt6v7dyzxehf`): a Σ positive control reduces to `a`; a
    function-indexed and a Σ-with-function cast evaluate to `Unknown`.
  - Include the generated All-support witness (`lift_recursive_value`,
    eval.rs ~:909/:941). Its predicate slot carries the eliminator's motive,
    a function value, and RT-C5-PRODUCER-TYPE-SLOTS left it unfilled
    (Architect `evt_7rtbs5j2730es` item 6). Its unit row there pins
    `eq_type_eq(&w, &w) == false`.
  - Propose the mechanism for each shape, with a soundness argument that
    names what the evaluator may and may not read from a proof value under
    Ω proof irrelevance.
  - The Architect rules before any build. Research may be called on the
    proof-directed rule.
- **AC-1.** Both function-bearing counterexamples evaluate to `a`, and the
  Σ positive control stays `a`. For each new arm or rule,
  a differing-index negative stays `Unknown`.
- **AC-2 (controls).** Removing each new arm or rule returns its positive to
  `Unknown`. `neutral_inductive_type_app_index_does_not_cast` stays green
  and unedited.

## Stop conditions

- Any kernel, `trusted_base()` or spec change (an operator question).
- A rule that equates two closures by representation or identity (spec 42
  §1) is an Architect stop.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40` or
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY`.
