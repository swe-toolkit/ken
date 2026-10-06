---
id: VERIFY-PACKAGE-ROUTE-EXAMPLE-DECLARATIONS
title: "Since VERIFY-CALLER-OBLIGATION-REPORTING the compiler driver builds a .ken.md package's admitted list from elaborate_ken_md_file_v1, which also returns ken example fence declarations, so an obligation-free literate package gains example declarations and a new core_semantic_hash. Admit only the source's declarations"
status: ready
owner: verify
size: S
tier: T2
gate: architect
depends_on: [VERIFY-CALLER-OBLIGATION-REPORTING]
blocks: []
github: null
origin: "Adversary finding evt_4mzcxrjmb2mqf on fd1bafb0e. Regression against that WP's AC-2 (hash changes only where an open obligation exists); public compile_ken_source and compile_ken_package_sources only, no ken-cli caller. Steward-filed per COORDINATION section 2."
---

# A literate package admits its declarations, not its examples

## Objective

The driver's declaration set for a `.ken.md` source is the declarations of
its `ken` fences, as it was before `fd1bafb0e`. Example fences still
elaborate and still report their open obligations.

## Settled inputs (Adversary `evt_4mzcxrjmb2mqf`, on `fd1bafb0e`)

- **The widening.** `elaborate_ken_md_file_v1` (`lib.rs:553`) appends the
  example results to the declaration results. The package route
  (`compiler_driver.rs:1223-1230`) extends `admitted` from all of them, and
  `emit_package_from_env` writes each admitted id into
  `semantic.declarations` (`:3385-3399`).
- **Repro.** A `.ken.md` with `const main : Bool = True` in a `ken` fence and
  `const zz_example : Bool = main` in a `ken example` fence: at `c7b5c2b38`
  the declarations are `[pkg, main]` with hash `b797275d6e912a48`; at
  `fd1bafb0e` they add `[pkg, zz_example]` with hash `285f8165ad1325cd`.
  The obligation map is empty in both.
- **Other consumers.** The denotation route (`:632-639`) also derives
  `admitted` and `source_declarations` from the combined list. The native
  route (`:2279`) adds every env declaration and is unaffected.
- `elaborate_ken_md_file_parts` (`lib.rs:560`) already returns declarations
  and examples separately.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

The package and denotation routes admit only the declaration part of a
`.ken.md` result, and pass every result, examples included, to obligation
reporting. No change to `.ken` sources, the native route or the ID-only
wrappers.

## Acceptance

- **AC-1.** The repro package's `semantic.declarations` and
  `core_semantic_hash` equal the `c7b5c2b38` values on both the package and
  denotation routes.
- **AC-2 (controls).** An open obligation inside an example fence is still
  reported. The `.ken` AC-2 controls of VERIFY-CALLER-OBLIGATION-REPORTING
  keep their results.
- **AC-3 (mutation, QA).** Admitting the example results again reddens
  AC-1.

## Stop conditions

- An example-fence obligation whose recorded origin names a declaration
  that is no longer admitted: stop to the Architect for where it belongs.
- Any kernel, `trusted_base()` or spec change.
