---
id: CAT-FORMAL-LANGUAGES-NFA
title: "The catalog has no nondeterministic automaton. Deliver Algorithm.FormalLanguages.Nfa (a Bool-relation Nfa with Omega path acceptance, the bit-mask subset construction into Dfa, its sound and complete laws, and NFA emptiness through section 2), with Core.Logic.And and public unit_finite/bool_finite, fully proved at zero trust"
status: merged
owner: foundation
size: M
tier: T2
gate: architect
depends_on: [SPEC-FORMAL-LANGUAGES-NFA-CONTRACT]
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Architect D0 boundary ruling evt_j3gq9c7y5ya5 with the checked 39-declaration development (evt_520cacgcw8wvh, evt_yw4322y6t1vx; rc=0 at 27a106c5d, no Axiom). Steward-filed per COORDINATION section 2."
---

# A nondeterministic automaton, determinized and proved

## Objective

`Algorithm.FormalLanguages.Nfa` lands as section 3 of
`spec/50-stdlib/61-formal-languages.md` specifies, with every law proved
and no new trust.

## Settled inputs (Architect `evt_j3gq9c7y5ya5`)

- **The development exists and checks.** The 398-line text in
  `evt_520cacgcw8wvh` and `evt_yw4322y6t1vx` type-checks rc=0 against the
  real catalog modules at `27a106c5d`. Its three mutants are
  kernel-rejected: M1 is final through `nfa_initial`, M2 is the reversed
  edge, and M3 is end-of-path finality ignored. It is the starting point,
  not a design to revisit.
- **Placement.**
  - `Core.Logic.And` is a new sibling of `Or`, with `data And (a : Omega)
    (b : Omega)` and constructor `Both`. A `Pair` of two Ω propositions is
    kernel-rejected.
  - `unit_finite` and `bool_finite` become public in `Data.Finite.Finite`.
  - The `Nfa` package carries the public surface the ruling names. `mask`
    and the proof lemmas stay private.
- **Measured authoring notes.**
  - Parenthesise a truncation passed as an argument.
  - Lift an `elim_trunc` λ that matches on a constructor into a named
    lemma.
  - A zero-parameter certificate is a `const`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The three packages, as the merged section 3 names them. Each package reads
top-down and has its own acceptance test target. Add a strict-resolution
ambient-census row for each new package; `CAT-FORMAL-LANGUAGES-DFA`'s first
CI red was the missing Dfa row.

## Acceptance

- **AC-1.** `ken fmt --check` and `ken check` pass for every changed
  package. The acceptance target roots-loads every public law and applies
  `determinize_sound`, `determinize_complete` and `nfa_is_empty_rejects` in
  a generic client. It also runs one concrete NFA on an accepted word and
  a rejected word through `determinize`.
- **AC-2.** `trusted_base()` is equal before and after. The packages
  declare no Axiom, postulate, primitive or foreign declaration.
- **AC-3 (consumers).** A whole-root sweep for the new public names
  (`And`, `Both`, `unit_finite`, `bool_finite`) over `catalog/`,
  `crates/*/tests`, `r_layer_tests`, `examples/`, `conformance/` and the CLI
  fixtures finds no collision. The strict-resolution D0 census passes
  with the new rows.
- **AC-4 (mutation, QA).** M1, M2 and M3 are each kernel-rejected at
  their proof spans, and restored.

## Stop conditions

- A law needs an Axiom, a kernel or surface-syntax change, or `DecEq q`.
- The merged section 3 differs from the ruling in a way the development
  cannot meet.

## Closeout

Merged `89ac3d285` from exact `18d7024ae` (PR #4617). Foundation QA
`evt_60bdfz09ejp15`, Architect `evt_73gqeaxc0bx91`, Decision
`dec_5qfky2dz8sps6`. `Algorithm.FormalLanguages.Nfa` delivers the
`Bool`-relation Nfa with Ω path acceptance, the bit-mask subset
construction into `Dfa` with its sound and complete laws, and NFA
emptiness through section 2. `Core.Logic.And` and public
`unit_finite`/`bool_finite` ship with it. `trusted_base()` is unchanged,
and M1, M2 and M3 are each kernel-rejected. The prelude `And` keeps its
binding (g233 before and after loading Nfa).
Carry (Architect): the private `Bool` truth lemmas (`or_left`, `or_right`,
`and_true`, `or_cases`, `and_cases`) are duplicated in Reachability, Map
and Nfa. They could be published beside `bool_or`/`bool_and` in
`Core.Classes.LawfulClasses`; not scheduled.
