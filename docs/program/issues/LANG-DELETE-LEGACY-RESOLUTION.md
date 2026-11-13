---
id: LANG-DELETE-LEGACY-RESOLUTION
title: "The elaborator still has two resolution modes. ResolutionMode::Legacy is the default of every fresh ElabEnv, and its passthrough returns an untracked bare name unchanged to the flat global table, so the REPL, the proof-erasure checker and every standalone elaborate_file, elaborate_decl and elaborate_module_from_roots caller still resolve names Strict refuses. Delete Legacy and its passthrough so every entry resolves Strict, migrating the sources that relied on it"
status: active
owner: language
size: L
tier: T1
gate: architect
depends_on: [LANG-STRICT-ISOLATED-ROUTES]
blocks: []
github: null
origin: "Operator 2026-09-25: one resolution mode, legacy passthrough deleted (L2 objective). LANG-STRICT-ISOLATED-ROUTES (closed) moved check, run and the isolated routes to Strict. Steward-filed per COORDINATION section 2."
---

# One resolution mode

## Objective

The elaborator has one resolution mode. A bare name that is not
scope-bound, compiler vocabulary or a floor name is refused through every
entry, the REPL and the proof-erasure checker included. Legacy and its
passthrough are gone, and spec 33's transitional Legacy contract is
replaced by the Strict rule.

## Settled inputs (main `2b0d05ef1`)

- **The mode.** `enum ResolutionMode { #[default] Legacy, Strict }`
  (`modules.rs:899`) sits on `Scope`, so every `ElabEnv::new()` starts
  in Legacy.
- **The passthrough.** `resolve_ref` falls through to
  `None => Ok(name.to_string())` (`:1222`) whenever the mode is not
  Strict. The ambient attached-proof path admits an untracked subject in
  Legacy only (`:1384`). The mode lock is `legacy_in_memory_source_started`
  (`:102`, `:2752`, `:6534`).
- **Legacy entries.** `elaborate_module_from_roots` and `_v1` pass
  `ResolutionMode::Legacy` (`:2659`, `:2671`). `elaborate_file`, `_v1`,
  `elaborate_ken_md_file_v1`, `elaborate_decl`, `_v1` and
  `expand_and_elaborate` run in whatever mode the env holds, which is
  Legacy for a fresh one.
- **Production on Legacy.** The REPL builds a fresh env and calls
  `elaborate_decl_v1` (`ken-cli/src/repl.rs:38`, `:165`). The
  proof-erasure checker calls `elaborate_file` and `elaborate_decl` on a
  fresh env (`ken-interp/src/proof_erasure_checker.rs:173`-`:214`).
  `ken check` and `ken run` are Strict already (`main.rs:504`-`:530`,
  `lib.rs:241`-`:246`).
- **Test reach.** At `2b0d05ef1`, `grep` finds about 195 files under
  `crates/*/tests` that call `elaborate_module_from_roots` and about 281
  that call `elaborate_file`, plus in-crate test modules. Most sources
  use only floor names, which Strict admits.
- **The spec.** `spec/30-surface/33-declarations.md:276`-`:279` and the
  "Transitional Legacy resolution" item (`:380`-`:392`) describe the
  Legacy contract this node deletes.

Treat anchors as perishable. If a fixed input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

`crates/ken-elaborator` (the mode, the passthrough, the Legacy entries),
the REPL and the proof-erasure checker, and the test and fixture sources
that bind through Legacy. Out of scope: the floor and B sets, the kernel,
and any catalog package's public names.

## Deliverable

The best current repair: make Strict the only mode, delete the Legacy
arms and the mode lock, and add the imports each migrated source needs.
The spec text moves in a Spec enclave co-turn on the product candidate.

## Acceptance

- **AC-0 (D0, Architect, before product code).** On scratch, flip the
  default to Strict and delete the passthrough. Count what refuses, by
  entry point and by source root: `catalog/`, `examples/`,
  `conformance/`, the `ken-cli` fixtures, and each `crates/*/tests` and
  in-crate test suite. The Architect chooses the census method; it is
  not a local crate sweep or population run (COORDINATION §12). Then
  rule:
  - the increments, if the reach exceeds one useful turn;
  - whether the standalone entries become Strict or are deleted, and
    the REPL's import surface;
  - what replaces each control that asserts a Legacy-only meaning, such
    as `legacy_bare_home_names_retain_the_pre_source_ids`
    (`modules.rs:10352`).
- **AC-1.** A bare unimported non-floor name refuses with UnboundName
  through `elaborate_file_v1`, `elaborate_module_from_roots_v1`,
  `elaborate_decl_v1` and the REPL. Choose a name each entry accepts on
  the base, so every row is a differential. Spec 33 keeps bare access to
  the five pre-source IDs (`:387`-`:390`), and `host_pure` is the
  likely candidate. Show the acceptance on the base.
- **AC-2.** The same name imported through its home resolves to the same
  checked identity through every one of those entries.
- **AC-3.** Restoring the `resolve_ref` passthrough returns the AC-1
  rows to acceptance.
- **AC-4.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The reach includes a catalog package whose public law or export name
  would change. Post it with the count; renaming a public identity is a
  separate node.
- A production caller needs a name that Strict cannot reach by any
  import.
- A kernel change.

Checks: 2 fired: the `ResolutionMode::Legacy` sites and the fresh-env
production callers were read at `2b0d05ef1`. 6 fired: the test reach is
a `grep` count of entry-point callers, not a closure; AC-0 measures it.
8 fired: AC-1's rows accept on main and must refuse after the change.
