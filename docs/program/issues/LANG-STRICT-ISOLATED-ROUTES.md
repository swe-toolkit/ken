---
id: LANG-STRICT-ISOLATED-ROUTES
title: "After the catalog route is strict, ken check on an isolated .ken or .ken.md file and ken run still elaborate in Legacy, and 45 of the 57 isolated files that check today refuse under Strict: isolated files have no import loader, and the program vocabulary (host_program, host_exit, bind, FSOp, the State family and others) has no import home. Add strict file entries and an isolated-file import loader, home the program vocabulary under the 2026-09-25 rule, migrate the refusing files, and move the isolated routes and ken run to Strict"
status: ready
owner: language
size: L
tier: T1
gate: architect
depends_on: [LANG-STRICT-CATALOG-FLIP]
blocks: []
github: null
origin: "Operator 2026-09-25: one resolution mode; an ever-present intrinsic's name is reserved; no intrinsic module. Part 2 of the Steward recut evt_7tgtgt0vc3q3c of LANG-STRICT-CATALOG-FLIP, on Architect AC-0 evt_6069bcfays9wq. Steward-filed per COORDINATION section 2."
---

# Isolated files and ken run resolve in strict mode

## Objective

`ken check` on an isolated file, and `ken run`, elaborate in
`ResolutionMode::Strict`. An isolated file can import catalog modules,
and every file that checks today still checks.

## Settled inputs (AC-0 evt_6069bcfays9wq, rechecked on `0fdc10a5d`)

- **The routes.** `ken check`'s isolated arm (`ken-cli/src/main.rs:485`,
  `:487`) and `ken run` (`run_program_inner`, `ken-cli/src/lib.rs:161`,
  `:162`) call `elaborate_file_v1` and `elaborate_ken_md_file_v1`. Most
  test harnesses call those entries too, so they are not changed in
  place.
- **The scope trap.** The root scope carries the prelude's 331 bindings.
  Setting `mode = Strict` on it would still resolve 293 ambient names
  bare. A strict file entry starts from the strict unit's scope, as in
  the AC-0 probe.
- **No import loader.** An isolated `import Capability.Console.Text
  (print_line)` fails with UnboundName on the module in both modes; it
  resolves after a strict preload of that module.
- **The population.** Of 86 isolated files under `examples/`,
  `conformance/` and `crates/*/tests`, Legacy checks 57 and Strict 12.
  The 45 that refuse use 67 distinct ambient names: all 12 rosetta
  examples and 33 fixtures. Ken sources inline in Rust tests are not
  measured.
- **The home rule** (operator 2026-09-25, Steward evt_7tgtgt0vc3q3c).
  There is no new intrinsic or host module. Each name either joins an
  existing catalog unit that already owns its concept, or is a floor
  addition with 30 §4's two witnesses.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

1. **AC-0 first (Architect, on the cut base).**
   - Re-measure the 86-file population, which predates the flip.
   - Measure the inline Ken sources.
   - Classify each of the 67 names under the home rule.
   - Rule the isolated loader's catalog-root discovery (spec 33 §3.2).
   - Name a split if one candidate is too large.
2. **One language branch:**
   - the strict file entries;
   - the loader;
   - the homes;
   - the migrated files;
   - the four production calls switched.

   The Spec co-turn owns `spec/`, and CV votes on `spec/` and
   `conformance/`.

## Acceptance

- **AC-1.** Every isolated file that checks under Legacy on the cut base
  checks through the strict file entry, and a test asserts the count.
- **AC-2.** Through the CLI binary, on `ken check` and on `ken run`, an
  isolated file that uses a non-floor name without its import is refused
  with the strict unbound-name error, and accepted with the import.
- **AC-3.** An imported unit's `main` lands qualified and cannot replace
  the isolated file's entry point.
- **AC-4 (mutation, QA).** Pointing `main.rs:485` back at
  `elaborate_file_v1` makes AC-2's `ken check` refusal row accept.
- **AC-5 (carried from LANG-CLI-RESOLUTION-REPORT AC-2).** The six
  standalone shadow invocations in the modules seed
  (`conformance/surface/modules/seed-modules.md`), now recorded there as
  a coverage gap, become
  covered: the shadow `fn write` imports its provider on an isolated
  file, and `--resolution-report` records `write` as `source-declared`
  on `check`, `run` and `native-build`.

## Stop conditions

- A name with no existing catalog owner, where its floor addition lacks
  §4's witnesses.
- A kernel or trust change.

Checks: 6 fired: the population is the 86 probed files at the strict
seam, and the inline sources are named as unmeasured for AC-0. 8 fired:
AC-4's mutant restores the Legacy entry, which accepts the unimported
name, so AC-2's row reddens.
