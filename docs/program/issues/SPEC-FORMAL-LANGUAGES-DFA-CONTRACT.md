---
id: SPEC-FORMAL-LANGUAGES-DFA-CONTRACT
title: "No automaton or formal-language contract exists in spec/50-stdlib, so the catalog's automata layer has nothing to build against. Author spec/50-stdlib/61-formal-languages.md with its first section: a deterministic automaton over any state carrier and any alphabet, its operations, its proved law set and its trust boundary"
status: merged
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: [CAT-FORMAL-LANGUAGES-DFA]
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\" (the 06-catalog-campaign.md roadmap; automata follow parse/syntax/diagnostics at :374). Architect ruling evt_7z0tswfa1wd9: spec first, following 58a. Steward-filed per COORDINATION section 2."
---

# A deterministic automaton contract

## Objective

`spec/50-stdlib/61-formal-languages.md` exists, and its first section is
the normative contract that `CAT-FORMAL-LANGUAGES-DFA` builds against.

## Settled inputs (Architect `evt_7z0tswfa1wd9`)

- **Representation.** `data Dfa q a = MkDfa (q → a → q) q (q → Bool)`.
  States are any `q : Type` and the alphabet is any `a : Type`, with no
  `DecEq`. Transition is a function, not a table. Acceptance is `Bool`, so
  `accepts` computes and complement is `bool_not`.
- **Finiteness is not part of this record.** A `Fin n` automaton is the
  instance `q = Fin n`. Finiteness arrives later as separate evidence.
- **Operations.** `step`, `start`, `final`, `run` (over `List a`),
  `accepts`, `complement`, `product combine`, and the abbreviations
  `intersection := product bool_and` and `union := product bool_or`.
- **Laws.** `run_append`, `run_complement`, `accepts_complement`,
  `run_product` (quantified over both start states), `accepts_product`,
  `accepts_intersection` and `accepts_union`. The Architect's sketch
  checked at rc=0 with no Axiom; it is in the ruling.
- **Deferred, in order:** a `Finite q` certificate with decidable
  emptiness and reachability; NFA and subset construction; regex and a
  derivative matcher (`DecEq a` enters here); equivalence and
  minimisation; a `Bytes`/`Cursor` runner bridge for `Capability.Parsing`
  lexers.

## Deliverable

The chapter, with its first section normative for:

- the record, the operations and the two abbreviations;
- the law set above, each stated as a proved theorem;
- the trust boundary: no Axiom, primitive, kernel form or
  `trusted_base()` entry;
- the explicit statement that this record is finite only when a
  finiteness certificate is supplied.

The deferred list is recorded as later sections, not specified. Index the
chapter in `spec/50-stdlib/README.md` and `spec/SPEC-PROGRESS.md`.

## Acceptance

- **AC-1.** Every operation and law the CAT node must deliver is named in
  the chapter with its type, and nothing the chapter requires is outside
  the Architect's ruling.
- **AC-2.** Conformance-validator vote on the exact SHA.

## Stop conditions

- The contract needs a kernel, trust or surface-syntax change.

## Closeout

Merged `e5ec530dc` from exact `cdc4bb734` (PR #4593). Architect
`evt_4vdc9t01e5tb2`, CV `evt_3r8dccx1g7as2`, Decision `dec_1p1bvwjy5ft2x`.
`spec/50-stdlib/61-formal-languages.md` §1 is the contract for
`CAT-FORMAL-LANGUAGES-DFA`, which is now unblocked.
