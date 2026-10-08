---
id: CAT-FORMAL-LANGUAGES-DFA
title: "The catalog has no automata. Land Algorithm.FormalLanguages.Dfa: a deterministic automaton over any state carrier and alphabet, with run, accepts, complement and a general product, and its law set fully proved and Axiom-free"
status: active
owner: foundation
size: M
tier: T2
gate: architect
depends_on: [SPEC-FORMAL-LANGUAGES-DFA-CONTRACT]
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\" (the 06-catalog-campaign.md roadmap; automata follow parse/syntax/diagnostics). Architect ruling evt_7z0tswfa1wd9 confirmed the node with changes and supplied a checked sketch. Fully proved per operator 2026-09-13. Steward-filed per COORDINATION section 2."
---

# A deterministic automaton package

## Objective

`catalog/packages/Algorithm/FormalLanguages/Dfa.ken.md` lands as a
literate entry that meets `spec/50-stdlib/61-formal-languages.md` §1,
with every law proved.

## Settled inputs (Architect `evt_7z0tswfa1wd9`)

- **The sketch.** The ruling's 18 declarations check at rc=0 with no
  Axiom. That was on a 2026-10-07 binary, earlier than current main. In the
  package, `list_append` comes from `Data.Collections.Derived`, `bool_and`
  and `bool_or` from `Core.Classes.LawfulClasses`, and `cong` from
  `Core.Logic.Transport`. `mk_pair`, `pair_fst` and `pair_snd` are prelude.
- **Complement is not definitional.** `accepts_complement := Refl` fails
  ("the two sides of the goal are not convertible"). The `run_complement`
  induction plus `cong` is required.
- **An inline `Refl` in the `Nil` arm of `run_product` failed** ("Refl
  expects an `Eq`-shaped goal"). The same goal as the standalone lemma
  `run_product_nil` accepts `Refl`.
- **`bool_not` is not public anywhere.** `Data.Collections.Map` has the
  only catalog duplicate (`Map.ken.md:5724`, private `fn bool_not`).
  `Vector.ken.md:513 vec_example_not` and `cat_vec_acceptance.rs:121` are
  example-local.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

- **The package**, from the sketch, plus `intersection`, `union`,
  `accepts_intersection` and `accepts_union` as one-line instances of
  `product` and `accepts_product`. The lede leads with the `accepts_*`
  laws and their operations; `run` and the record come below.
- **`pub fn bool_not`** in `Core.Classes.LawfulClasses`, next to
  `bool_and` and `bool_or`. Map's private duplicate is deleted and Map
  imports the public one.
- **The `run_product` `Nil` arm**, reproduced on the base: write the arm as
  inline `Refl` first. If it still fails, keep `run_product_nil` and post
  the exact error and the declaration in the WP thread. The Steward files
  the language node from that.

## Acceptance

- **AC-1.** The package checks with no Axiom, primitive or
  `trusted_base()` delta, and every law in the spec section is a proved
  theorem in it.
- **AC-2.** An acceptance test checks the package and evaluates
  `accepts` on concrete automata: a complement, an intersection and a
  union each agree with the Bool combination of their operands' results.
- **AC-3 (control).** Map and every other `LawfulClasses` consumer keep
  their results after the `bool_not` promotion.
- **AC-4 (mutation, QA).** Swapping `combine`'s operands in `product`'s
  final predicate reddens `accepts_product`. Making `complement` keep
  `final` unchanged reddens `accepts_complement`.

## Stop conditions

- A law needs an Axiom, a primitive, or a kernel or trust change.
- The spec section requires something outside the Architect's ruling.
