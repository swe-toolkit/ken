---
id: SPEC-FORMAL-LANGUAGES-NFA-CONTRACT
title: "spec/50-stdlib/61-formal-languages.md section 3 is a deferred placeholder, so the catalog has no contract for nondeterministic automata or subset construction. Specify the Nfa carrier, its acceptance, the subset construction into section 1's Dfa, and its language-equality law, at zero trust"
status: merged
owner: spec
size: S
tier: T1
gate: architect
depends_on: [SPEC-FORMAL-LANGUAGES-FINITE-REACHABILITY-CONTRACT]
blocks: [CAT-FORMAL-LANGUAGES-NFA]
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\". The ruled automata order (NFA, regex, minimisation, Bytes/Cursor lexer bridge) after CAT-FORMAL-LANGUAGES-FINITE-REACHABILITY. Same shape as SPEC-FORMAL-LANGUAGES-FINITE-REACHABILITY-CONTRACT (Architect boundary ruling, then contract, then CAT node). Steward-filed per COORDINATION section 2."
---

# A nondeterministic automata contract

## Objective

Section 3 of `spec/50-stdlib/61-formal-languages.md` is the normative
contract that the NFA catalog node builds against.

## Settled inputs (on `48f4bacb2`)

- **Section 1.** `data Dfa q a = MkDfa (q → a → q) q (q → Bool)`, with no
  `DecEq q` or `DecEq a`, and `run` by structural recursion over `List a`.
- **Section 2.** `Finite q` (an enumeration with an Ω `list_elem`
  covering proof, no `DecEq`), `fin_finite`, `pair_finite`, and
  `find_word` with its sound and complete laws, all at zero trust.
- **Section 3 today:** "A later contract will add NFA and subset
  construction with a language-equality law. Section 1 neither supplies
  nondeterminism nor postulates the subset-construction law."
- `DecEq a` enters only with regex (section 4); state equality and
  minimisation stay in section 5.

## Deliverable

1. **D0 (Architect boundary ruling, measured).** As for section 2
   (`evt_5aspyx6byry`), rule from a checked development (rc=0, no Axiom)
   of:
   - the `Nfa q a` carrier and its acceptance (Ω or `Bool`);
   - the subset construction's `Dfa` state carrier, and whether it
     needs `DecEq q` or `Finite q`;
   - the language-equality law's exact type;
   - whether section 2's decision applies to the constructed `Dfa`
     (a `Finite` for its carrier).
2. **Section 3, normative** for every ruled declaration and law, each
   stated as a proved theorem, with the trust boundary (no Axiom,
   primitive, kernel form or `trusted_base()` entry). Update
   `spec/SPEC-PROGRESS.md` if the chapter set changes.
3. **A §3 conformance seed** under `conformance/stdlib/` (Steward scope
   amendment on CV `evt_1b55d8xs5saa4`), authored by the CV in the existing
   seed shape. It covers path acceptance (the length and finality
   conventions), both determinization directions, and finite emptiness.
   Sections 1 and 2 are not backfilled here.

## Acceptance

- **AC-1.** Every declaration and law the CAT node must deliver is named
  with its type, and nothing required is outside the Architect's ruling.
- **AC-2.** Conformance-validator vote on the exact SHA of the assembled
  scope: the three spec paths plus the seed.

## Stop conditions

- The contract needs a kernel, trust or surface-syntax change, or
  `DecEq q` for the subset construction (state equality is section 5's).

## Closeout

Merged `3a949dd8f` from exact `a90173eef` (PR #4611). Architect D0
`evt_j3gq9c7y5ya5` (a checked 39-declaration development, rc=0), Architect
`evt_35q2xp7s50m3c`, CV `evt_631wqp3car0jq`, Decision
`dec_2kar6smjnpwh3`. Section 3 specifies the Bool-relation `Nfa`, Ω path
acceptance, the finite bit-mask subset construction with
`determinize_sound` and `determinize_complete`, NFA emptiness through
section 2, `Core.Logic.And`, and public `unit_finite` and `bool_finite`. It
requires `Finite q`, never `DecEq q`. The 10-case seed
`conformance/stdlib/formal-languages/seed-nfa.md` was added by scope
amendment `evt_4ymzbcbrn09dm`.
