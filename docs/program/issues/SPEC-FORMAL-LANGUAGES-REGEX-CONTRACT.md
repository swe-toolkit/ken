---
id: SPEC-FORMAL-LANGUAGES-REGEX-CONTRACT
title: "spec/50-stdlib/61-formal-languages.md section 4 is a deferred placeholder, so the catalog has no contract for regular expressions. Specify the regex carrier, its denotation as a language, a derivative matcher, and the matcher's soundness and completeness laws, at zero trust"
status: active
owner: spec
size: S
tier: T1
gate: architect
depends_on: [SPEC-FORMAL-LANGUAGES-NFA-CONTRACT]
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\". The ruled automata order after NFA: regex, minimisation, the Bytes/Cursor lexer bridge. Same shape as SPEC-FORMAL-LANGUAGES-NFA-CONTRACT (Architect boundary ruling from a checked development, then contract, then CAT node). Steward-filed per COORDINATION section 2."
---

# A regular expression contract

## Objective

Section 4 of `spec/50-stdlib/61-formal-languages.md` is the normative
contract that the regex catalog node builds against.

## Settled inputs (on `69904cd9f`)

- **Sections 1-3.** `Dfa q a` with `run` over `List a` and no `DecEq`;
  `Finite q` with `find_word` and its laws; the `Bool`-relation `Nfa` with
  Ω path acceptance, `determinize` and its sound and complete laws, all
  at zero trust (`3a949dd8f`).
- **Section 4 today:** "A later contract will add regex and a derivative
  matcher. `DecEq a` enters there; it is not a prerequisite for §§1–3's
  alphabet."
- `DecEq` exists as a catalog class (spec 51). State equality and
  minimisation stay in section 5.

## Deliverable

1. **D0 (Architect boundary ruling, measured).** As for section 3
   (`evt_j3gq9c7y5ya5`), rule from a checked development (rc=0, no Axiom)
   on these questions:
   - the `Regex a` carrier (which constructors);
   - its denotation (an Ω membership relation over `List a`);
   - the derivative and nullability functions, and where `DecEq a`
     enters;
   - the matcher's sound and complete laws against the denotation, with
     their exact types;
   - whether a regex-to-NFA or regex-to-DFA link belongs in this section
     or a later one.
2. **Section 4, normative** for every ruled declaration and law. Each is
   stated as a proved theorem, with the trust boundary: no Axiom,
   primitive, kernel form or `trusted_base()` entry. Update
   `spec/SPEC-PROGRESS.md` if the chapter set changes.
3. **A section 4 conformance seed** under `conformance/stdlib/`, in the
   existing seed shape. It covers the empty language, the empty word,
   alternation, concatenation, star, and a `DecEq`-distinguished symbol.

## Acceptance

- **AC-1.** Every declaration and law the CAT node must deliver is named
  with its type, and nothing required is outside the Architect's ruling.
- **AC-2.** Conformance-validator vote on the exact SHA of the assembled
  scope: the spec paths plus the seed.

## Stop conditions

- The contract needs a kernel, trust or surface-syntax change.
- A law cannot be stated without decidable state equality (section 5's).
