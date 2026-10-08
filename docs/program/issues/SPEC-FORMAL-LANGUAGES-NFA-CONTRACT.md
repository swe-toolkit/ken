---
id: SPEC-FORMAL-LANGUAGES-NFA-CONTRACT
title: "spec/50-stdlib/61-formal-languages.md section 3 is a deferred placeholder, so the catalog has no contract for nondeterministic automata or subset construction. Specify the Nfa carrier, its acceptance, the subset construction into section 1's Dfa, and its language-equality law, at zero trust"
status: ready
owner: spec
size: S
tier: T1
gate: architect
depends_on: [SPEC-FORMAL-LANGUAGES-FINITE-REACHABILITY-CONTRACT]
blocks: []
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

## Acceptance

- **AC-1.** Every declaration and law the CAT node must deliver is named
  with its type, and nothing required is outside the Architect's ruling.
- **AC-2.** Conformance-validator vote on the exact SHA.

## Stop conditions

- The contract needs a kernel, trust or surface-syntax change, or
  `DecEq q` for the subset construction (state equality is section 5's).
