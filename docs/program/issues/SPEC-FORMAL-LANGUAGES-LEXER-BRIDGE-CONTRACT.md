---
id: SPEC-FORMAL-LANGUAGES-LEXER-BRIDGE-CONTRACT
title: "spec/50-stdlib/61-formal-languages.md section 6 is a deferred placeholder, so no Capability.Parsing lexer can claim that its byte runner agrees with a DFA's run. Specify the Bytes/Cursor runner bridge relating a DFA driven over a cursor to run over the cursor's element list, with its laws, at zero trust"
status: ready
owner: spec
size: S
tier: T1
gate: architect
depends_on: [SPEC-FORMAL-LANGUAGES-DFA-CONTRACT]
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\". The ruled automata order after regex: minimisation, then the Bytes/Cursor lexer bridge (SPEC-FORMAL-LANGUAGES-MINIMISATION-CONTRACT origin). Same shape as the section 4 and 5 contracts: an Architect boundary ruling from a checked development, then the contract, then the CAT node. Steward-filed per COORDINATION section 2."
---

# A lexer input bridge contract

## Objective

Section 6 of `spec/50-stdlib/61-formal-languages.md` is the normative
contract that the lexer-bridge catalog node builds against.

## Settled inputs (on `c55fdbb1f`)

- **Sections 1-5** are normative and at zero trust: `Dfa q a` with `run`
  over `List a` (section 1), and equivalence and minimisation
  (section 5, `d107f7633`).
- **Section 6 today:** "A later `Bytes`/`Cursor` runner bridge for
  `Capability.Parsing` lexers will relate byte-runner results to `run`
  over the corresponding byte list. Sections 1-5 use `List a` only and
  make no byte/lexer claim."
- **`Capability.Parsing.Cursor`** has `CursorOps c el loc` (remaining,
  peek, advance, locate) with `CursorLaws`. It makes no claim relating
  locations across instances. `arg_cursor_ops` over `Bytes` arguments
  carries `arg_cursor_laws`, and its elements come from the total
  `List UInt8` view.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

1. **D0 (Architect boundary ruling, measured).** As for sections 4 and 5,
   rule from a checked development (rc=0, no Axiom) on:
   - the runner: a DFA over the cursor's element type, stepped by
     `cursor_peek` and `cursor_advance`, and what it returns (final
     state, acceptance, or a longest accepted prefix with its end
     cursor);
   - the bridge law and its exact type: the runner's result equals `run`
     over the cursor's remaining element list, under `CursorLaws` alone
     or only for a named instance;
   - the package and import direction, so that FormalLanguages and
     Capability.Parsing do not form a cycle;
   - what stays deferred, such as tokenising a whole input or source
     locations in diagnostics.
2. **Section 6, normative** for every ruled declaration and law, each
   stated as a proved theorem, with the trust boundary: no Axiom,
   primitive, kernel form or `trusted_base()` entry.
3. **A section 6 conformance seed** under `conformance/stdlib/`, in the
   existing seed shape. It covers an accepted and a rejected byte input,
   the empty input, and agreement with `run` on the same bytes.

## Acceptance

- **AC-1.** Every declaration and law the CAT node must deliver is named
  with its type, and nothing required is outside the Architect's ruling.
- **AC-2.** Conformance-validator vote on the exact SHA of the assembled
  scope: the spec paths plus the seed.

## Stop conditions

- The contract needs a kernel, trust or surface-syntax change.
- The bridge law needs a cross-instance claim that `CursorLaws` does not
  give: the D0 rules what is stated instead, and nothing unprovable is
  specified.
