---
id: CAT-FORMAL-LANGUAGES-MINIMISATION
title: "The catalog cannot decide DFA equivalence or minimise a DFA. Deliver Algorithm.FormalLanguages.Minimisation (equivalent and equivalent_states through the product's disagreement language and section 2 reachability, canonical and minimise on the original carrier, and the eight laws of spec section 5) fully proved at zero trust"
status: active
owner: foundation
size: M
tier: T2
gate: architect
depends_on: [SPEC-FORMAL-LANGUAGES-MINIMISATION-CONTRACT]
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Architect D0 evt_30hjxcgryy66c with the checked development (architect/work notes/dfa5-d0-development.ken, 912 lines, rc=0, no Axiom). Steward-filed per COORDINATION section 2."
---

# Equivalence and minimisation, decided and proved

## Objective

`Algorithm.FormalLanguages.Minimisation` lands as section 5 of
`spec/50-stdlib/61-formal-languages.md` specifies, with every law proved
and no new trust.

## Settled inputs (Architect `evt_30hjxcgryy66c`)

- **The development exists and checks.** `notes/dfa5-d0-development.ken`
  on `architect/work` (912 lines) type-checks rc=0 on `bcf525d67`, with no
  Axiom, postulate or hole. It is the starting point, not a design to
  revisit.
- **Surface.** 13 public names: `same_future`, `equivalent`,
  `equivalent_states`, `canonical`, `minimise` and the eight laws
  (`equivalent_sound`, `equivalent_complete`, `equivalent_states_sound`,
  `equivalent_states_complete`, `canonical_same_future`,
  `canonical_unique`, `accepts_minimise`, `minimise_reduced`). About 28
  private helpers, including `disagree` and `run_minimise`.
- **No state equality.** Equivalence is the emptiness of the product's
  disagreement language. Neither `DecEq q` nor a cardinality notion is
  used.
- **Imports, all landed.** Dfa, Reachability, Finite, Derived
  (`list_elem`, `list_append`), Combinators (`is_some`), Or, Transport
  and LawfulClasses (`bool_eq`, `bool_not`).
- **Proof idiom.** Every case analysis on a computed Bool or Option goes
  through an `_at` helper that takes the value and its equation. `Equal q
  e e` at a variable carrier needs `Refl`, not `Proved`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

1. The package, in section 5's order: `equivalent` and its laws, then
   `equivalent_states`, then `minimise`, `canonical` and their laws, then
   the private helpers. It reads top-down and has its own acceptance test
   target. Add a strict-resolution ambient-census row for it.

## Acceptance

- **AC-1.** `ken fmt --check` and `ken check` pass. The acceptance target
  roots-loads every public law, applies `equivalent_sound` and
  `minimise_reduced` in a generic client, and runs the ten
  `seed-minimisation.md` cases with the listed results.
- **AC-2.** `trusted_base()` is equal before and after. The package
  declares no Axiom, postulate, primitive or foreign declaration.
- **AC-3 (consumers).** A whole-root sweep for the 13 public names over
  `catalog/`, `crates/*/tests`, `r_layer_tests`, `examples/`,
  `conformance/` and the CLI fixtures finds no collision. Sweep by
  mechanism too: any harness that flattens a package's imports or
  certifies them syntactically (the Rosetta runner, SEAL-2) passes. The
  Dfa, Reachability and strict-resolution D0 targets pass.
- **AC-4 (mutation, QA).** Each definition mutant makes a law's proof
  kernel-reject, and is restored: `canonical` as the identity;
  `equivalent_states` as constant `True`; `disagree` as constant `False`.

## Stop conditions

- A law needs an Axiom, `DecEq q`, a kernel or surface-syntax change, or a
  ninth law.
- The merged section 5 differs from the development in a way it cannot
  meet.
- A public name collides with an existing name in any importer.
