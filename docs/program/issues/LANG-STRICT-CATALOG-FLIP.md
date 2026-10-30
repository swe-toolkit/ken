---
id: LANG-STRICT-CATALOG-FLIP
title: "Spec 33 §3.3 says every source unit resolves in one strict mode, but ResolutionMode defaults to Legacy and no production caller selects Strict: ken check reaches the catalog through the legacy roots entry and isolated files through the default scope. With every ledger candidate ruled, move ken check's catalog and isolated-file routes to Strict, migrating every unit Strict refuses, so that no production elaboration resolves a bare name through the flat globals map"
status: ready
owner: language
size: L
tier: T1
gate: architect
depends_on: [LANG-PRELUDE-FLOOR-CONSOLE-AND-CHAR]
blocks: []
github: null
origin: "Operator 2026-09-25: one resolution mode; strict admits only the minimal prelude; legacy passthrough deleted; binding a prelude name anywhere is an error. lanes.md L2 orders the floor additions, then the flip and enforce (L2-4). LANG-PRELUDE-FLOOR-CONSOLE-AND-CHAR rules the last ledger candidate, which is the flip's entry condition. This is the flip's first slice; local-binder enforcement and deleting Legacy are its successors. Steward-filed per COORDINATION section 2."
---

# The catalog resolves in strict mode

## Objective

`ken check` elaborates every catalog unit, and every isolated `.ken` or
`.ken.md` file, in `ResolutionMode::Strict`. Each one that checks today
still checks, and each one resolves every bare name through its own
declarations, explicit imports, kernel names or the prelude floor.

## Settled inputs (main `05a15edee`; re-measure on the cut base)

- **The contract.** "One strict mode, for every source unit", including
  roots-loaded and isolated-file compilation (`33-declarations.md:265`,
  `:268-270`). An unresolved bare name never falls through to
  implementation globals (`39-elaboration.md:134-136`).
- **The mode.** `ResolutionMode` defaults to `Legacy`
  (`modules.rs:685-687`). One roots run cannot mix modes (`:136`).
  The strict entry is `elaborate_module_from_roots_strict` (`:2436`).
  Its doc says the real catalog caller moves to it only after the
  dependency census is migrated.
- **The production callers are all Legacy.**
  - `ken check` reaches catalog paths through
    `elaborate_module_from_roots_v1` (`ken-cli/src/main.rs:477`).
  - It reaches `.ken.md` and plain files through
    `elaborate_ken_md_file_v1` and `elaborate_file_v1` (`:485`, `:487`),
    which use the default scope.
  - `ken run` (`main.rs:540`) is not yet traced.
- **What Legacy admits.** `resolve_ref` returns an unbound bare name
  as-is, to be found in `cx.globals` (`modules.rs:916`). For an
  untracked attached proof, `resolve_attached_ref` (`:1143`) falls back to
  the ambient path only under `Legacy` (`:1170`). Strict refuses both.
- **The population.** 81 catalog `.ken.md` files, 16 files under
  `examples/` and 15 under `conformance/`. No count says how many need an
  ambient name. `catalog_evidence_frontier_is_closed_and_honest`
  (`r_layer_tests/lang_mod_catalog_evidence_frontier.rs:890`) records,
  for each catalog unit, whether Strict succeeds or refuses and at which
  stage. It asserts no count.
- **Carried into the flip.** `lanes.md` L2 lists the Architect's carries
  from SESSION-SCOPE and the floor slices. Two examples are the remaining
  `globals` writes (`evt_74ac7bpmpcd0c`) and Parsing's ambient
  `charToInt` (`evt_7001gdwsnh1q1`).

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

1. **AC-0 first (Architect, on the cut base).**
   - Run the evidence frontier, and give the units Strict refuses, each
     with its cause.
   - Trace `ken run`'s route.
   - Rule which of the L2 carries this slice must close for the routes to
     flip, and which stay with the successors.
   - If the refusal population is too large for one candidate, name the
     split. Each part must leave every route that has moved resolving
     strictly.
2. **One language branch.** Each refusing unit is migrated token-aware
   to explicit imports, with no new floor name. Then `main.rs:477`,
   `:485` and `:487`, and `ken run`'s route if AC-0 names it, select
   Strict. Legacy stays reachable only from tests, for the successor to
   delete. The Spec co-turn owns `spec/`, and CV votes on `spec/` and
   `conformance/`.

## Acceptance

- **AC-1.** On the candidate, the evidence frontier reports no unit that
  Strict refuses, and the test asserts that count.
- **AC-2.** `ken check` on a file that uses a non-floor prelude name
  without importing it refuses with the strict unbound-name error. The
  same file with the explicit import is accepted. Both are tested through
  the CLI binary, on a catalog path and on an isolated file.
- **AC-3 (mutation, QA).** Restoring the Legacy entry at `main.rs:477`
  makes AC-2's catalog refusal row accept.

## Stop conditions

- A unit that needs a new floor name, an identity clone, or an ambient
  fallback to check.
- A kernel or trust change, or a change to a spec 36 to 38 operation
  type.

Checks: 3 fired: the population counts cover `catalog/`, `examples/`
and `conformance/` on `05a15edee`. They are files, not consumers, and
AC-0's frontier run replaces them. 8 fired: AC-2's refusal row accepts
under Legacy, because `resolve_ref` returns the bare name, so AC-3's
mutant reddens it.
