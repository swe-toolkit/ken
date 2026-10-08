---
id: CAT-FORMAL-LANGUAGES-FINITE-REACHABILITY
title: "A Dfa's emptiness and reachability cannot be decided in the catalog, because there is no finite-state evidence. Land Data.Finite.Finite (an Omega-membership enumeration certificate, fin_finite, pair_finite) and Algorithm.FormalLanguages.Reachability (a certificate-returning search with sound and complete laws, is_empty, accepted_word), fully proved and Axiom-free"
status: merged
owner: foundation
size: M
tier: T2
gate: architect
depends_on: [SPEC-FORMAL-LANGUAGES-FINITE-REACHABILITY-CONTRACT, CAT-FORMAL-LANGUAGES-DFA]
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\". Architect L3 order evt_7z0tswfa1wd9 section 4 item 1; boundary ruling evt_5aspyx6byry with a measured 99-declaration development (rc=0, no Axiom). Fully proved per operator 2026-09-13. Steward-filed per COORDINATION section 2."
---

# Finite-state evidence and Dfa reachability

## Objective

`catalog/packages/Data/Finite/Finite.ken.md` and
`catalog/packages/Algorithm/FormalLanguages/Reachability.ken.md` land as
literate entries that meet `spec/50-stdlib/61-formal-languages.md` §2,
with every law proved.

## Settled inputs (Architect `evt_5aspyx6byry`)

- **The development.** The ruling and its three appendix posts in
  `thr_2we3g3tqnhnd9` give 99 declarations that check at rc=0 with no Axiom
  on an `e1b609597` build. Catalog definitions are inlined there as
  stand-ins and become imports. `leq_trans` and `leq_weaken` stand for
  `proof trans for leq_nat` (LawfulClasses) and `leq_nat_weaken_right`
  (Nat.Order), with the same argument order.
- **Completeness is by counting stabilisation.** `reach k` is monotone in
  `k`; its count over the state enumeration is at most N; each unstable
  step raises the count. It needs both certificates and no `DecEq`.
- **`Fin` is private on `337faba62`.** The implementer measured
  `UnboundName` for the import (`evt_6f8kdn9px2bf7`); the Vector exposure
  in the Deliverable is the enabler.
- **Placement.**
  - Public in `Data.Collections.Derived`: `list_elem`, `list_elem_head`,
    `list_elem_later`, `list_elem_map`, `list_elem_append_left`,
    `list_elem_append_right`, `list_elem_concat_map`,
    `list_elem_transport`. No equivalent exists in the catalog.
  - `Data.Finite.Finite`: `Finite`, `MkFinite`, `elements`, `covers`,
    `fin_finite`, `fin_elements`, `fin_elements_cover` and `pair_finite`.
    It imports `Fin` from `Data.Vector.Vector`, which must not import it.
  - `Algorithm.FormalLanguages.Reachability`: the public decision and its
    four laws. `any_of`, `search`, `reach`, `stable`, `count_true` and the
    counting lemmas are private.
- **Measured traps.**
  - `Refl` in a match arm fails ("Refl expects an `Eq`-shaped goal"): use
    `Proved` for closed constructors, or a standalone `Eq` lemma.
  - `Equal` at `Pair` computes, so `J` refuses a pair equality: transport
    with `list_elem_transport` at `Pair`.
  - A match does not refine earlier parameters: take scrutinee hypotheses
    as arrows after the match.
  - A lambda in a type-position application does not parse: name it.
  - An Ω-valued result must be a `theorem`.
  - On `e1b609597`, `data Finite` placed before `list_elem` gave
    `UnresolvedCon list_elem`. `LANG-FORWARD-REFERENCE-ACROSS-DATA-EXPORT`
    has since landed (`6f3b0aceb`); re-measure before arranging, and keep
    data declarations before their use if it still fails.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

- **The two packages and the Derived promotions**, from the development,
  with every stand-in replaced by its import.
- **Vector publishes `Fin`, `FZero` and `FSuc`, and nothing else**
  (Architect `evt_7e5s9thdrf4v4`). In the Definition fence of
  `Data/Vector/Vector.ken.md`, `pub data Fin` followed by
  `export FZero, FSuc` (spec 33 §4.1: `pub data` publishes only the type
  former). Do not inline or move `Fin`. The Trust & derivation sentence
  says the `Fin` family and its constructors are public as the
  bounded-index provider for `Data.Finite.Finite`, everything else stays
  private, and no trust or `trusted_base()` changes. In
  `cat_vector_closeout.rs`, split and rename
  `vector_loader_visible_inventory_is_empty`: the three names import and
  resolve to Vector's checked ids, and every other owned name still
  rejects with the exact qualified `UnboundName`.
- **Census rows.** `Data.Finite.Finite` and
  `Algorithm.FormalLanguages.Reachability` get their
  `lang_mod_strict_resolution_d0` rows in this candidate, and Vector's
  rows stay green.
- **Arrangement.** Reachability leads with `is_empty`, `accepted_word` and
  their laws, then the `find_word` laws, then `search`, with the counting
  proof last.

## Acceptance

- **AC-1.** Both packages check with no Axiom, primitive or
  `trusted_base()` delta, and every declaration and law in §2 is present
  and proved.
- **AC-2.** An acceptance test evaluates, on concrete `Fin`-state automata:
  `is_empty` of a non-empty automaton is `False` and `accepted_word` returns
  an accepted word; `is_empty` of an automaton with no reachable final state
  is `True`; and `is_empty` of an `intersection` of two automata with
  disjoint languages is `True`, through `pair_finite`.
- **AC-3 (control).** Every `Data.Collections.Derived` consumer keeps its
  result after the promotions.
- **AC-4 (mutation, QA).** Each of the ruling's three rejected mutants
  reddens: fuel bound 0 in the completeness statement, stability claimed at
  0, and `is_empty` without its `bool_not`.

- **AC-5.** `import Data.Vector.Vector (Fin, FZero, FSuc)` resolves to
  Vector's checked ids, every other Vector name still rejects with
  `UnboundName`, and Vector's acceptance and closeout suites stay green.

## Stop conditions

- A law needs an Axiom, a primitive, or a kernel or trust change.
- An import's delivered form does not fit where the stand-in was used and
  the proof needs more than an argument reorder.

## Closeout

Merged `48c333620` from exact `b953a1496` (PR #4609). QA
`evt_2t1x7173y1re9`, Architect `evt_4xyn6ae06nd6t`, Decision
`dec_rmzvv4q5zb8m`. `Data.Finite.Finite` lands with `Finite`, `elements`,
`covers`, `fin_finite` and `pair_finite`, and
`Algorithm.FormalLanguages.Reachability` with `find_word`, `reachable`,
`accepted_word` and `is_empty` and their sound and complete laws, at zero
trust. Vector publishes `pub data Fin` and `export FZero, FSuc` as the
bounded-index provider; Derived gains the Ω-valued `list_elem` family.
