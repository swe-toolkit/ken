---
id: LANG-STRICT-CATALOG-FLIP
title: "Spec 33 §3.3 says every source unit resolves in one strict mode, but ken check reaches the catalog through the Legacy roots entry, and five catalog units refuse under Strict because the prelude-source names they use have no import home, which the Transparent-only package-home gate cannot give an inductive family. Extend package homes to checked inductive families, home those names in the five units that wrap them, and move ken check's catalog route to Strict"
status: merged
owner: language
size: L
tier: T1
gate: architect
depends_on: [LANG-PRELUDE-FLOOR-CONSOLE-AND-CHAR]
blocks: [LANG-STRICT-ISOLATED-ROUTES]
github: null
origin: "Operator 2026-09-25: one resolution mode; strict admits only the minimal prelude; legacy passthrough deleted; binding a prelude name anywhere is an error. Recut to the catalog route by Steward evt_7tgtgt0vc3q3c on Architect AC-0 evt_6069bcfays9wq, which found the import-only migration premise false; the isolated routes and ken run are LANG-STRICT-ISOLATED-ROUTES. Steward-filed per COORDINATION section 2."
---

# The catalog resolves in strict mode

## Objective

`ken check` elaborates every catalog unit through the strict roots entry.
Each unit that checks today still checks, and resolves every bare name
through its own declarations, explicit imports, kernel names or the
prelude floor. Legacy results are unchanged.

## Settled inputs (main `614203948`, Architect AC-0 evt_6069bcfays9wq)

- **The frontier.** `catalog_evidence_frontier_is_closed_and_honest`
  (`r_layer_tests/lang_mod_catalog_evidence_frontier.rs:890`) has 79
  rows. Legacy succeeds on all 79. Strict succeeds on 74 and refuses 5,
  all at Resolution, and no catalog unit imports any of the 5:
  - `Capability.Filesystem.Authority`: `FS`, `read_bytes`, `write_file`;
  - `Capability.System.Buffer`: the `BufferWindow` family,
    `buffer_span_budget`, `buffer_span_length`, `transfer_count_*`;
  - `Capability.System.Error`: `FileOperation` and its 16 constructors;
  - `Capability.System.IO`: `buffer_span_budget`, `transfer_count_*`;
  - `Capability.System.Resource`: the `ResourceBodyResult` and
    `ResourceBracketResult` families.
- **The cause.** Each name is a prelude-source declaration with no
  catalog export. `PRELUDE_PACKAGE_HOMES` has 5 entries, and
  `capture_strict_builtin_names` refuses a home whose identity is not
  `Decl::Transparent` (`modules.rs:631-635`), so an inductive family
  cannot be homed as written.
- **The precedent.** Floor families carry `constructor_members` with a
  completeness check (`modules.rs:597-611`).
- **The route.** `ken check` reaches catalog paths through
  `elaborate_module_from_roots_v1` (`ken-cli/src/main.rs:477`); the
  strict entry is `elaborate_module_from_roots_strict`
  (`modules.rs:2436`).
- **Carries.** Parsing's ambient `charToInt` is closed on this base.
  The `globals` test-harness writes, the two `main` lookups (`lib.rs:823`,
  `program_admission.rs:88`) and the 54 dynamic `globals.get(name)` sites
  stay with the successors.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

One language branch:

1. The package-home mechanism accepts a checked inductive family, keyed
   on its parent GlobalId with the complete constructor set. The
   Transparent-only gate stays for non-inductive homes.
2. The measured names are homed in the five units above, each published
   by that unit's `export`. The Spec co-turn writes the 33 §3.3 home
   text, and CV votes on `spec/` and `conformance/`.
3. `main.rs:477` selects the strict roots entry. Legacy stays reachable
   from tests and the isolated routes, for the successors.

## Acceptance

- **AC-1.** The frontier reports no unit that Strict refuses, and the
  test asserts that count. A row pins that each homed name still
  resolves bare under Legacy to the same identity.
- **AC-2.** `ken check`, through the CLI binary on a catalog path, refuses
  a non-floor prelude name used without its import with the strict
  unbound-name error, and accepts it with the explicit import.
- **AC-3 (mutation, QA).** Restoring the Legacy entry at `main.rs:477`
  makes AC-2's refusal row accept.

## Stop conditions

- A unit that needs a new floor name, an identity clone, or an ambient
  fallback to check.
- A home that changes a homed name's identity, or any Legacy result.
- A kernel or trust change, or a change to a spec 36 to 38 operation
  type.

Checks: 4 fired: the import-only premise was replaced by the measured
obligation; the home gate refuses inductives (`modules.rs:631-635`), so
the mechanism change is in scope. 6 fired: the population is the five
refusals the frontier measured at the strict seam. 8 fired: AC-2's
refusal row accepts under Legacy, because `resolve_ref` returns the bare
name, so AC-3's mutant reddens it.
