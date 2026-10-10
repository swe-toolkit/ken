---
id: CAT-FORMAL-LANGUAGES-LEXER-BRIDGE
title: "No catalog lexer can claim that a DFA driven over a cursor agrees with run over the cursor's elements. Deliver section 6 of the formal-languages spec: Cursor's element-list view (cursor_take, cursor_elements and their two peek laws) and the new Capability.Parsing.Lexer (dfa_cursor_run, dfa_cursor_accepts and the two bridge laws), fully proved at zero trust"
status: merged
owner: foundation
size: M
tier: T2
gate: architect
depends_on: [SPEC-FORMAL-LANGUAGES-LEXER-BRIDGE-CONTRACT]
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Spec section 6 merged 718b726c2 from the Architect D0 development evt_2mn6kw5efep62 (rc=0, no Axiom). Steward-filed per COORDINATION section 2."
---

# The lexer input bridge, delivered and proved

## Objective

`Capability.Parsing.Cursor` gains its element-list view and
`Capability.Parsing.Lexer` lands, both as section 6 of
`spec/50-stdlib/61-formal-languages.md` specifies, with every law proved
and no new trust.

## Settled inputs (on `718b726c2`)

- **Section 6 is normative.** Section 6.1 adds four public names to
  Cursor (`cursor_take`, `cursor_elements`, `cursor_elements_peek_none`,
  `cursor_elements_peek_some`), taking its surface from 17 to exactly 21.
  Section 6.2 gives Lexer exactly four public names (`dfa_cursor_run`,
  `dfa_cursor_accepts`, `dfa_cursor_run_elements`,
  `dfa_cursor_accepts_elements`), with their types written out.
- **The development exists.** The Architect's D0 `evt_2mn6kw5efep62`
  checked a development rc=0 with no Axiom. It shows feasibility; it is
  the starting point, not a design to revisit.
- **Laws and their hypotheses.** The `None` law and both bridge laws hold
  for any `CursorOps`, with no law argument. Only the `Some` law takes
  `CursorLaws`, and uses only its `CursorAdvanceProgress` conjunct.
- **Imports run one way.** Lexer imports Dfa, Cursor and `cong` only.
  Neither Dfa nor Cursor imports Lexer, and nothing under `Algorithm/`
  imports `Capability.*`.
- **The seed** is `conformance/stdlib/formal-languages/seed-lexer-bridge.md`:
  seven runtime-oracle cases over `arg_cursor_ops`, not `Proved` byte
  equations.
- **The inventory pin.** `cat_tier_d_cursor_import.rs` pins Cursor's
  loader-visible surface at 17 names. Section 6.1 makes moving it to 21
  this node's work.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

1. The Cursor additions and the Lexer package, each in section 6's
   order, reading top-down, with the private fuelled scan in Lexer.
   Lexer has its own acceptance test target and a strict-resolution
   ambient-census row.

## Acceptance

- **AC-1.** `ken fmt --check` and `ken check` pass on both packages. The
  acceptance target roots-loads all eight new public names, applies
  `dfa_cursor_accepts_elements` and `cursor_elements_peek_some` in a
  generic client, and runs the seven seed cases with the listed results.
- **AC-2.** `trusted_base()` is equal before and after. Neither package
  declares an Axiom, postulate, primitive or foreign declaration.
- **AC-3 (consumers).** Cursor's inventory pin moves to exactly 21. A
  whole-root sweep over `catalog/`, `crates/*/tests`, `r_layer_tests`,
  `examples/`, `conformance/` and the CLI fixtures finds every other pin
  or importer of Cursor's surface (the Parsing group, Decoder,
  Arguments, ArgParse and Json import tests among them) and keeps each
  green. No new name collides in any importer. Sweep by mechanism too:
  the Rosetta runner and SEAL-2 pass.
- **AC-4 (mutation, QA).** Each mutant makes a law's proof kernel-reject,
  and is restored: `cursor_take` ignoring `peek = None` (keeps taking);
  the Lexer scan not stepping the state; `dfa_cursor_accepts` as
  `final d (start d)`, without running the cursor.

## Stop conditions

- A law needs an Axiom, `DecEq`, `CursorLaws` where section 6 says none,
  or a kernel or surface-syntax change.
- An import cycle, or an `Algorithm/` package needing `Capability.*`.
- A seed case fails at runtime on the delivered packages.

## Closeout

Merged `7a890f13d` from exact `260e102ea` (PR #4639). QA
`evt_6b6p59614zzzc`, with the three AC-4 mutants kernel-rejected and
restored; Architect `evt_52d0kk06byw42`; Decision `dec_1c3ejamt2g35f`.
Cursor gains `cursor_take`, `cursor_elements` and the two peek laws, and
its pin moves from 17 to 21 names. `Capability.Parsing.Lexer` delivers
`dfa_cursor_run`, `dfa_cursor_accepts` and the two bridge laws for any
`CursorOps`, importing Dfa and Cursor only. Acceptance is 8/8 with the
seven runtime seeds; the strict-resolution census row, Rosetta and SEAL-2
pass. No new trust. §1a: 0.
