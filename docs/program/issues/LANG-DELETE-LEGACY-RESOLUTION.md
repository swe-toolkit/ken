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

## I4 preconditions

`LANG-PRELUDE-NONHOME-FAMILIES-HOME` has landed, and the I1 rows held for
it (listed by identity in I1's handoff) migrate by import in I4
(Architect evt_13j52ac43jrw8, evt_3pxvhj0yngpxm).

`SPEC-COLLECTIONS-PERM-PIN-ALIGN` has landed: spec 37 §6 states the
57 §3.1 Perm, and C5 is re-rooted by import (Architect
evt_3agt8q8hbz7sb (5)).

I4 also closes the standalone route (Architect evt_3agt8q8hbz7sb (4)).
`elaborate_expr` (`lib.rs` :1181) resolves against flat globals outside
the Legacy census, so bare pre-source names would survive the deletion
through it. It resolves through the Strict root scope of a module body,
or accepts qualified checked keys only. Pin: bare `Not` through
`elaborate_expr` refuses with UnboundName, and `Core.Logic.Not.Not`
resolves to the catalog id. Before I4 is built, the residue census adds
its instrument at `resolve_expr_standalone`, so the 67 `.elaborate_expr(`
callers are counted, not inferred.

I4 also completes the rooted driver entries (Architect
evt_385y36vdagp66; I1b converts `compile_checked_target_denotation` and
`compile_ken_package_sources_with_env` to honor `CompilerSource.roots`).
Their empty-roots path becomes Strict, and an import on a rootless
source refuses at the import with a diagnostic that names the missing
roots, not a downstream unbound name (I4 decides whether the current
UnboundName on the module path suffices). `allow_rootless_strict` must
not let a rootless library row pass for the wrong reason: I4's pin shows
a library import refusing on a rootless entry.

`LANG-INLINE-MODULE-ENCLOSING-SCOPE` has landed: spec 33 rules inline
children lexical (Spec evt_318gqrbkefpmt), and the held
`lang_standard_infix_call_completion` and
`lang_membership_operator_surface` rows migrate in I4 with no fixture or
assertion edit (Architect evt_4em1d5vqwab2p, evt_13ms2ywx4pta0).

Carried, framed at I4's landing: retire the pre-source `Perm` and
`Perm_rel` registration (`prelude.rs` :1534-1762) with the rows whose
subject is that definition (Architect evt_3agt8q8hbz7sb (3), five rows by
identity). The es4 bridge keeps its Derived half. Count the pre-source
`is_sorted`'s remaining users and include it if none remain. This is the
same shape as the zip retirement carried by
`LANG-PRELUDE-NONHOME-FAMILIES-HOME`.

## I4 spec 33 co-turn carry

The fence import scope (Architect evt_4f4yshmbkj59e, on landed I0
`68d4a6ab4`; I1 carries the pin) enters spec 33 in I4, in the
spec-author's wording, with this content: a checked `ken example` or
`ken reject` fence in a literate module is elaborated in that module's
completed scope. Its imports resolve as the module's own imports do,
through the selected catalog roots in their resolution mode, so a fence
may import a module outside the module's tangled closure, which is then
loaded and checked with it. A `ken reject` fence demonstrates a refusal
of the language, not one that depends on which modules happen to be
loaded already.

## §1b symptom inventory

Chain: the increment cut and non-home reachability (Architect
evt_13j52ac43jrw8; §1a 4 at evt_4em1d5vqwab2p).

1. A generated prelude family (checked/saturating fixed-width arithmetic,
   `conversions.rs`) is source-reachable only through Legacy ambient
   lookup and has no taxonomy row or package home. Keyed on pre-source
   registration with no explicit-import home. I4 now depends on a
   homing node framed from the counted non-home population.
2. A pre-source definition (`Perm`, comparator-free truncation) whose
   only catalog namesake has a different signature and meaning
   (`Derived.Perm`, comparator-indexed count equality), so no import
   preserves the user's identity or meaning. Keyed on a spelling-equal,
   definition-distinct catalog provider. Disposition waits on the
   Architect's ruling from the full-spelling Perm population.
3. A test row's subject (the B1 checked-target export of a call-site
   Requires hole) is reached only through
   `compile_checked_target_denotation`, which elaborates its
   `CompilerSource` on a fresh env without its roots, so the bare name's
   home exists but cannot be selected from that route. Keyed on an entry
   that discards `CompilerSource.roots`. The causes of entries 1 to 3
   are independent (a catalog gap, a spec gap, a driver gap in this WP's
   scope); no recut.
4. A fixture-local class declared at a unit's root is used inside an
   inline child module of the same unit. Strict builds the child's scope
   fresh (`modules.rs` :5764), so the enclosing locals are not visible,
   and Legacy's passthrough masked this. Keyed on the inline-module scope
   boundary. Entries 3 and 4 share a shape the predicate question at
   entry 6 tests: a resolution context that does not carry what its
   enclosing context selected.

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
