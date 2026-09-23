---
id: CAT-PARSING-LAWS
title: "Proof-backfill for Capability/Parsing/Parsing.ken.md: inhabit the package's own ParserLaws proposition for parse_bool_expr (ParserValid, ParserTotal, ParserSourceLocal), and prove the Boolean printer/formatter round trip over the same syntax, spans, source and decoder representation, with no new trust"
status: ready
owner: foundation
size: L
gate: none
tier: T1
depends_on: [CAT-PARSING-CURSOR-LAWS, CAT-PARSING-DECODER-LAWS]
blocks: []
github: null
origin: "One of the seventeen proof-backfill follow-ons named by docs/program/CATALOG-PROOF-COMPLETENESS-SURVEY.md, under operator ruling 2026-09-13 ('schedule the proof backfill before extending the catalog'). Chosen next because its two lower-tier parsing packages (Cursor, Decoder) now carry laws. Steward-filed per COORDINATION section 2."
---

# `ParserLaws` has no inhabitant for the one parser the package ships

## Settled inputs -- measured at `1da3055e2`. Re-ground before acting.

- `catalog/packages/Capability/Parsing/Parsing.ken.md` defines
  `ParserLaws a p = And (ParserValid a p) (And (ParserTotal a p)
  (ParserSourceLocal a p))` (`:347`), with `ParserValid` / `ParserTotal` /
  `ParserSourceLocal` at `:314-345`. No term proves it for
  `parse_bool_expr`. Parsers are built by `parser_from_decoder` (`:286`).
- The survey row: `LessEqNat::{refl,zero_left}` and `valid_zero_width_span`
  are proofs, but `ParserLaws` is only a predicate, and there is no general
  printer round trip. `print_bool_expr` is at `:753` and `format_bool_expr` at
  `:769`. The external evidence is
  `crates/ken-elaborator/tests/cat5_parsing_package.rs`
  (`cat5_d1_*`, `cat5_d2_*`, `cat5_d3_bool_parser_printer_formatter_roundtrip_on_source_bytes`).
- The lower tiers now carry laws: `CAT-PARSING-CURSOR-LAWS` (landed
  `1da3055e2`) and `CAT-PARSING-DECODER-LAWS`. Use them. Do not re-prove them
  here.
- `LessEqNat` is written as `Equal Bool (leq_nat m n) True`, not `IsTrue`, so
  this package does not meet Cursor's `IsTrue` alias issue. If a new import
  of `IsTrue` becomes necessary, follow Cursor's settled input 3 (per-item
  alias, never bare).

## Deliverable

1. A public attached proof inhabiting `ParserLaws (Syntax BoolExpr)
   parse_bool_expr`.
2. The Boolean printer round trip: parsing `print_bool_expr e` yields `e`,
   and `format_bool_expr` is coherent with it. State it over the existing
   syntax, span, source and decoder representation.

## Acceptance criteria

- **AC-1.** No new trust: the added lines contain no `Axiom`, postulate,
  primitive, `Omega` carrier, or kernel/TCB change. The production
  representation is unchanged.
- **AC-2.** Shipped declarations are unchanged apart from added proofs and
  exports. Existing `cat5_*` tests stay green.
- **AC-3.** The diff touches `Parsing.ken.md` plus, under `crates/`, only
  consumer-view harnesses that mechanically mirror the package's public
  surface (for example
  `crates/ken-elaborator/src/r_layer_tests/cat_tier_d_parsing_group_import.rs`).
  Classify each assertion that goes red with Cursor's `AC-3c` rule: bring a
  MIRRORING assertion into agreement; for a DIRECTIONAL one (such as
  `catalog_ambient_passthrough_migration_census`), change the candidate,
  never the assertion.
- **AC-4.** Targeted builds only, through `scripts/ken-cargo`. No-regression
  means green in CI.

## Stop conditions

- If the round trip needs a fact about primitive `Bytes` that no existing TCB
  contract states, prove everything else and STOP on that fact. Report it to
  the foundation leader; do not add a contract or axiom.
- Any `crates/**/src/**` path other than a consumer-view harness is a hard
  stop.
