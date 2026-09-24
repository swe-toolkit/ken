---
id: CAT-ARGPARSE-LAWS
title: "Proof-backfill for Application/CommandLine/ArgParse.ken.md: prove, for arbitrary specifications and argument lists, that argparse_run preserves raw argument Bytes, accumulates every located diagnostic in token order, and drives help from the same spec, over the existing representation with no new trust"
status: ready
owner: foundation
size: L
gate: none
tier: T1
depends_on: [CAT-SCHEMA-LAWS, CAT-PARSING-CURSOR-LAWS, CAT-PARSING-DECODER-LAWS, KERNEL-LITERAL-CHAR-VIEW]
blocks: []
github: null
origin: "Proof-backfill follow-on CAT-ARGPARSE-LAWS named by docs/program/CATALOG-PROOF-COMPLETENESS-SURVEY.md, under operator ruling 2026-09-13 ('schedule the proof backfill before extending the catalog'). Chosen next because every package it imports now carries laws. Steward-filed per COORDINATION section 2."
---

# ArgParse's guarantees are all external tests

## Settled inputs -- measured at `1ed04b835`. Re-ground before acting.

- `catalog/packages/Application/CommandLine/ArgParse.ken.md` (566 lines)
  contains no theorem or attached proof. It exports the spec and result
  types plus `argparse_run` (`:415`), `command_help` (`:521`) and
  `program_help` (`:523`).
- The core is `argparse_parse_tokens` (`:340`). It walks `arguments` with an
  `index` seed (`Suc Zero` after the subcommand), and handles each token as
  a flag, a value option (consuming the next token, `Suc (Suc index)`), an
  unknown `--` option, an unexpected positional, or a positional. Every error
  goes through `argparse_error` (`:284`) and `argparse_cons_validations`
  (`:297`), and parsing continues after an error. Missing positionals come
  from `argparse_missing_positionals` (`:326`).
- Help is `command_help spec = schema_help (command_schema spec)`, where
  `command_schema` (`:180`) builds from `argparse_option_schema_fields` and
  `argparse_positional_schema_fields`.
- The external evidence is
  `crates/ken-elaborator/tests/cc7_argparse_acceptance.rs`: flags, raw values
  and positionals with derived help; two bad arguments accumulating exact
  nonzero locations; invalid-UTF-8 value bytes surviving; adding one option
  changing help with no second edit.
- `CAT-SCHEMA-LAWS`, `CAT-PARSING-CURSOR-LAWS` and `CAT-PARSING-DECODER-LAWS`
  are merged. Use their laws; do not re-prove them here. The new provider
  laws below are additions, not re-proofs.
- **Law 2 needs provider laws that are not exported** (`evt_6s7xaffe0nz4`).
  `Data/Collections/NonEmpty.ken.md` has no public list-view append or map
  projection law, and `Application/Input/Schema.ken.md` has no ordered
  invalid-issue-sequence law. Add them in those packages as public proofs
  under the same AC-1 rule, with their consumer-view harnesses. Keep the
  `NonEmpty` constructor private.

## Deliverable

Attached proofs in `ArgParse.ken.md`, general over every spec and argument
list:

1. **Byte preservation.** Every `ParsedOption` or `ParsedPositional` value
   in a `Valid` result is the input `Bytes` term at its token position,
   unchanged.
2. **Located accumulation.** Each offending token contributes exactly one
   diagnostic carrying its own token index and byte range. Diagnostics
   appear in token order, and one error never suppresses a later one.
3. **Help from the spec.** `command_schema spec` has one field per option
   and then one per positional, in spec order, so `command_help` is derived
   from the same value that `argparse_run` parses against.

The ring chooses the exact statements. Each must quantify over arbitrary
input and must fail if the implementation changes the behavior it names.

**Parked for the operator (2026-09-23).** Deliverable 3 and its provider
laws landed (`6e235b746`, `3120a845c`). Deliverables 1 and 2 stopped on
bounded attempts (`evt_3jdh6yptdct5m`, `evt_4ts4acq2t3jxf`). The cause is
per-occurrence literal identity: `elab_str_lit` mints a fresh identity for
each `"--"`, so the parser's prefix literal and the proof's never convert
(Architect `evt_5sy6b16r05r48`). The kept extraction `71776ddd8` stays
unlanded. **Operator 2026-09-24:** resume laws 1 and 2 after
`KERNEL-LITERAL-CHAR-VIEW` lands. Every `"--"` here is
`string_to_list_char "--"`, which that node reduces to one `List Char`
across occurrences (its AC-4). No parser or elaborator change.

## Acceptance criteria

- **AC-1.** No new trust: the added lines contain no `Axiom`, postulate,
  primitive, `Omega` carrier, or kernel/TCB change. The production
  representation and shipped declarations are unchanged apart from added
  proofs and exports. For the laws 1-2 resumption after K3 there is **no
  parser or elaborator change**: the 2026-09-23 shared-step factorization
  fallback is withdrawn (2026-09-24). A distinct post-K3 structural blocker
  is a STOP for its own ruling. Exported types are unchanged and `cc7_*`
  stays green. The Architect confirms meaning on the actual candidate.
- **AC-2 (falsifier).** For each deliverable, the handback names one
  one-line natural-site mutation that makes that law's proof fail to check
  for its own property. For laws 1 and 2 the site is
  `argparse_parse_tokens`. For law 3 it is `command_schema` or
  `command_help`. For example, `Suc index` instead of `Suc (Suc index)` after
  a value option, at whichever site owns that branch, must break law 2. No
  law may be vacuous: its hypotheses must be satisfiable, and removing any
  one must make it false.
- **AC-3.** Under `crates/`, only consumer-view harnesses change, such as
  `src/r_layer_tests/cat_tier_e_argparse_import.rs`. Use Cursor's `AC-3c`
  rule: bring a MIRRORING assertion into agreement; for a DIRECTIONAL one
  (such as `catalog_ambient_passthrough_migration_census`), change the
  candidate, never the assertion. `cc7_*` stays green.
- **AC-4.** Targeted builds only, through `scripts/ken-cargo`. No-regression
  means green in CI.

## Stop conditions

- If a law needs a fact about primitive `Bytes` or `String` that no existing
  TCB contract states, prove everything else and STOP on that fact. Do not
  add a contract or axiom.
- Any `crates/**/src/**` path other than a consumer-view harness is a hard
  stop.
- Any new `IsTrue` import uses a per-item alias, never bare (Cursor settled
  input 3).
