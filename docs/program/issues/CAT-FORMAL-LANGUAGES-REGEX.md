---
id: CAT-FORMAL-LANGUAGES-REGEX
title: "The catalog has no regular expressions. Deliver Algorithm.FormalLanguages.Regex (the six-constructor carrier, its independent Omega denotation, a DecEq derivative matcher and its six sound and complete laws), with list_concat/list_all in Data.Collections.Derived and the five Bool truth lemmas published once in Core.Classes.LawfulClasses, fully proved at zero trust"
status: merged
owner: foundation
size: M
tier: T2
gate: architect
depends_on: [SPEC-FORMAL-LANGUAGES-REGEX-CONTRACT]
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Architect D0 evt_3bf35x1gz9evy with the checked development (architect/work 759779da8, notes/regex-d0-development.ken, rc=0, no Axiom). Bool-lemma publication and migration: Architect evt_5p54ndqe1raej and the CAT-FORMAL-LANGUAGES-NFA carry. Steward-filed per COORDINATION section 2."
---

# Regular expressions, matched and proved

## Objective

`Algorithm.FormalLanguages.Regex` lands as section 4 of
`spec/50-stdlib/61-formal-languages.md` specifies, with every law proved,
no new trust, and no private copy of a shared helper.

## Settled inputs (Architect `evt_3bf35x1gz9evy`, `evt_5p54ndqe1raej`)

- **The development exists and checks.** `notes/regex-d0-development.ken`
  at `759779da8` (635 lines) type-checks rc=0 with no Axiom, postulate or
  hole. It is the starting point, not a design to revisit.
- **Placement.**
  - `list_concat` and `list_all` go in `Data.Collections.Derived`, beside
    `list_append`.
  - `or_left`, `or_right`, `or_cases`, `and_true` and `and_cases` go in
    `Core.Classes.LawfulClasses`, with the types in section 4.3.
  - Regex carries the section 4 public surface. Its structural proof
    lemmas stay private.
- **Existing copies.** Nfa (`Nfa.ken.md:457-485`) and Reachability
  (`:1107`, `:1113`) define private copies of those lemmas with the same
  types. Nfa and Reachability import LawfulClasses selectively,
  LawfulFunctors imports it qualified, and PriorityQueue, the one
  wholesale importer, defines none of the five names. Map's
  `cat4_bool_or_*` lemmas are about Map's local `cat4_bool_or`, not
  `bool_or`, so they are not copies.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

1. The three package changes as section 4 names them. Regex reads
   top-down and has its own acceptance test target. Add a
   strict-resolution ambient-census row for Regex.
2. Nfa and Reachability import the five lemmas from LawfulClasses, and
   their private copies are deleted.
3. **CI-red repair (Architect `evt_2v7zfzpzxwjk`).** Derived imports
   `Core.Logic.And (And as Conj)` and spells the use `Conj`; an aliased
   whole-module import fails SEAL-2 closed by design. The Rosetta runner
   (`crates/ken-cli/tests/rosetta.rs`, the ninth path) removes that import
   directive and cuts `pub fn list_all` from its flattened Derived, as it
   already excludes Nat.Order exports outside its allowlist.
4. **Second CI-red repair** (PR #4623 run `37950411696`, shard 3).
   `r_layer_tests::cat_bool_pub_export::class_owner_provider_loader_visible_inventories_are_exact`
   (`crates/ken-elaborator/src/r_layer_tests/cat_bool_pub_export.rs:479`,
   the tenth path) pins LawfulClasses' loader-visible surface to a literal
   authorized set, which lacks the five published lemmas. Add them to the
   authorized set, and the same for any other exact-surface pin of
   LawfulClasses or Derived the sweep below finds.

## Acceptance

- **AC-1.** `ken fmt --check` and `ken check` pass for every changed
  package. The acceptance target roots-loads every public law and applies
  `regex_matches_sound` and `regex_matches_complete` in a generic client.
  It runs the seven `seed-regex.md` cases through `regex_matches` with
  `DecEq_instance_Bool` and gets the listed results.
- **AC-2.** `trusted_base()` is equal before and after. The packages
  declare no Axiom, postulate, primitive or foreign declaration.
- **AC-3 (consumers).** A whole-root sweep for the new public names (the
  Regex surface, `Split`, `Pieces`, `list_concat`, `list_all` and the five
  lemmas) over `catalog/`, `crates/*/tests`, `r_layer_tests`, `examples/`,
  `conformance/` and the CLI fixtures finds no collision. The existing
  Nfa and Reachability acceptance targets and the strict-resolution D0
  census pass. By mechanism: `seal2_tests::producer_closure` is 20/20 and
  `rosetta` is 2/2, with `rosetta_examples_match_their_oracles` green.
  Every test that pins a changed package's exact public or loader-visible
  surface (grep the roots for `LawfulClasses`, `Derived`, `Nfa` and
  `Reachability` beside a literal name set) is listed in the handoff and
  passes.
- **AC-4 (mutation, QA).** Each of these definition mutants makes a law's
  proof kernel-reject, and is restored: `d.eq` replaced by constant
  `True` in the `Sym` arm; the `Cat` derivative ignoring `nullable r1`;
  the `Star` derivative dropping the trailing `Star r1`.

## Stop conditions

- A law needs an Axiom, a kernel or surface-syntax change, or decidable
  state equality.
- The merged section 4 differs from the development in a way it cannot
  meet.
- Publishing a lemma collides with an existing name in any importer.

## Closeout

Merged `4598faaf8` from exact `99af342bd` (PR #4623). QA
`evt_24009n06m9m56`, Architect `evt_3r9m7wr1mq8x1`, Decision
`dec_3nvpxhcqxsdmc`. `Algorithm.FormalLanguages.Regex` lands with the
six-constructor carrier, its Ω denotation through `Split` and `Pieces`,
`nullable`, `deriv`, `regex_matches` and six laws proved at zero trust.
`list_concat` and `list_all` are public in `Data.Collections.Derived`, the
five Bool lemmas are public in `Core.Classes.LawfulClasses`, and Nfa and
Reachability import them in place of their private copies.

Two heads were red in CI, both on consumers outside the name sweep:
- `aa72cc7a0`: Derived's aliased whole-module import of `Core.Logic.And`
  failed SEAL-2 closed and broke the Rosetta runner's flattened Derived
  (`evt_7c7p0b6rgkpn7`). Repaired by an item import and a Rosetta cut.
- `02ee546e2`: the exact LawfulClasses loader-visible inventory pin in
  `r_layer_tests/cat_bool_pub_export.rs` lacked the five new theorems
  (run 37950411696). Repaired by adding them to the authorized set.

§1a: 0. AC-3 now sweeps consumers by mechanism and by exact-surface pin.
