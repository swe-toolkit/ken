---
id: RT-C5-COMPOUND-HIGHER-ORDER-INDICES
title: "C5 canonicity for compound and higher-order indices: a closed, checked, reflexive cast over a Sigma-indexed or function-indexed family evaluates to its value instead of Unknown, by a rule whose soundness is argued before it is built"
status: merged
owner: runtime
size: S
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

The Architect's D0 ruling (`evt_1p1wrct6dcx9`): in the `Term::Cast` arm of
`eval_in_erasure_scope`, a cast whose two endpoint terms are identical, with
no Unknown operand, returns its payload. This is the kernel's regularity
step (obs.rs:899-904), and it mirrors J's `lhs == rhs` rule. It reads no
proof value and compares no closures. The ruling's three lines are the
whole change; `cast_reduce`, `eq_type_eq`, J, the kernel and the spec are
unchanged. Convertible but non-identical endpoints (the ruling's rows 5 and
6) are `RT-C6-FUNCTION-INDEX-CONVERTIBLE-ENDPOINTS`, and the All-support
two-slot fill is not owed.

## Acceptance

- **AC-0.** Done: probe `evt_5wt6v7dyzxehf`, D0 `evt_1p1wrct6dcx9`.
- **AC-1.** The ruling's rows 2 and 3 (bare function and Σ-with-function
  index) evaluate to their value. Row 1 (Σ positive control) stays a value,
  and row 4 (differing endpoints, unchecked) stays `Unknown`.
- **AC-2 (controls).** Removing the arm returns rows 2 and 3 to `Unknown`
  while row 1 stays a value. An identical-endpoint cast with an Unknown
  operand stays `Unknown`. If no closed checked term evaluates to Unknown,
  record the guard as spec 42 §4 defense with no constructible row.
  `neutral_inductive_type_app_index_does_not_cast` stays green and
  unedited.

## Stop conditions

- Any kernel, `trusted_base()` or spec change (an operator question).
- A rule that equates two closures by representation or identity (spec 42
  §1) is an Architect stop.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40` or
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY`.
