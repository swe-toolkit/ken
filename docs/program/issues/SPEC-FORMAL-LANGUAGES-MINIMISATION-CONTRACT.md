---
id: SPEC-FORMAL-LANGUAGES-MINIMISATION-CONTRACT
title: "spec/50-stdlib/61-formal-languages.md section 5 is a deferred placeholder, so the catalog has no contract for automaton equivalence or minimisation. Specify DFA language equivalence and a minimisation construction over Finite evidence and DecEq state equality, with their laws, at zero trust"
status: ready
owner: spec
size: S
tier: T1
gate: architect
depends_on: [SPEC-FORMAL-LANGUAGES-REGEX-CONTRACT]
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\". The ruled automata order after regex: minimisation, then the Bytes/Cursor lexer bridge. Same shape as SPEC-FORMAL-LANGUAGES-REGEX-CONTRACT (Architect boundary ruling from a checked development, then contract, then CAT node). Steward-filed per COORDINATION section 2."
---

# An equivalence and minimisation contract

## Objective

Section 5 of `spec/50-stdlib/61-formal-languages.md` is the normative
contract that the minimisation catalog node builds against.

## Settled inputs (on `7043d1062`)

- **Sections 1-4** are normative and at zero trust (`b4030dbcd`):
  - `Dfa q a` with `run`, and `product`, `intersection` and `union`;
  - the `Finite q` record value, which neither prohibits duplicates nor
    constructs `DecEq q`, with `find_word` and predicate reachability;
  - the `Nfa` with `determinize`;
  - `Regex` with a `DecEq a` derivative matcher.
- **Section 5 today:** "Equivalence and minimisation require §2's finite
  evidence **and** decidable state equality. Section 2 supplies neither
  an equality decision for arbitrary states nor equivalence or
  minimisation procedures."
- `DecEq` exists as a catalog class (spec 51).

## Deliverable

1. **D0 (Architect boundary ruling, measured).** As for section 4
   (`evt_3bf35x1gz9evy`), rule from a checked development (rc=0, no
   Axiom) on:
   - how state equality enters: an explicit `DecEq q` dictionary, a
     field of a finite-carrier record, or both;
   - the equivalence relation and its decision: language equality of two
     DFAs, or of two states of one, decided through `product` and §2's
     reachability, and its sound and complete laws with their exact
     types;
   - the minimisation construction: its carrier for the quotient states,
     and the laws it carries (language preservation, plus whatever
     minimality statement is provable at zero trust);
   - what stays deferred to a later section.
2. **Section 5, normative** for every ruled declaration and law, each
   stated as a proved theorem, with the trust boundary: no Axiom,
   primitive, kernel form or `trusted_base()` entry. Update
   `spec/SPEC-PROGRESS.md` if the chapter set changes.
3. **A section 5 conformance seed** under `conformance/stdlib/`, in the
   existing seed shape. It covers equal and unequal languages, a DFA with
   redundant states, and the minimised automaton's acceptance on the
   same words.

## Acceptance

- **AC-1.** Every declaration and law the CAT node must deliver is named
  with its type, and nothing required is outside the Architect's ruling.
- **AC-2.** Conformance-validator vote on the exact SHA of the assembled
  scope: the spec paths plus the seed.

## Stop conditions

- The contract needs a kernel, trust or surface-syntax change.
- A minimality law cannot be stated at zero trust: the D0 rules what is
  stated instead, and nothing unprovable is specified.
