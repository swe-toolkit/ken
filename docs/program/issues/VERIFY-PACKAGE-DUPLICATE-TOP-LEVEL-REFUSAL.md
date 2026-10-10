---
id: VERIFY-PACKAGE-DUPLICATE-TOP-LEVEL-REFUSAL
title: "A ken example fence, or a later plain source, may redefine an admitted top-level name: the duplicate check is per elaboration unit, and stable_symbols_for_env names ids by spelling from the flat globals table, so the admitted declaration falls to the owner#ordinal fallback, core_semantic_hash moves, and a redefined target becomes unselectable. Refuse a duplicate top-level name across units of one package"
status: active
owner: verify
size: S
tier: T1
gate: architect
depends_on: [VERIFY-PACKAGE-EXAMPLE-BINDING-SCOPE]
blocks: []
github: null
origin: "Adversary finding evt_3vjznkjfzawy7 on e1b609597 (spec/40-runtime/46-checked-core-package.md section 3.2). Pre-existing; latent in the catalog (0 clashes in 60 .ken.md files). Steward-filed per COORDINATION section 2."
---

# A package has one declaration per top-level name

## Objective

A top-level name that is already bound in the package is refused with
`DuplicateDefinition`. This holds whether the second binding is in an
example fence or in a later source. Admitted stable symbols and
`core_semantic_hash` do not depend on example names.

## Settled inputs (Adversary `evt_3vjznkjfzawy7`, at `e1b609597`)

- **The check is per unit.** `DuplicateDefinition` uses `unit_definitions`
  (`resolve.rs:1026-1031`). Each example fence is its own
  `elaborate_file_v1` unit (`lib.rs:701-708`).
- **Naming is by spelling.** `stable_symbols_for_env` names ids from
  `env.globals` (`compiler_driver.rs:4197-4202`), and the displaced id
  takes the `owner#ordinal` fallback (`:4219-4227`).
- **Repro** (`compile_ken_source`, NonRuntime, target `main`):
  - `const base : Bool = True` and `const main : Bool = base` give hash
    `48e43379ae5466ab`. An unrelated example leaves the hash unchanged.
  - An example `const base : Bool = False` gives hash `7b980c10618353f4`
    and decls `[base#0, main]`.
  - An example `const main : Bool = False` gives `MissingTarget { main }`.
- **Plain duplicates.** Within one source, across fences, the duplicate is
  refused. Two plain package sources that each define `base` are not
  refused; they give decls `[base, base#0, main]`.
- **The pin.** `denotation_excludes_example_only_checked_string_literal`
  (`compiler_driver.rs` ~:7990) holds only because its example name is
  fresh.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The Architect rules the seam at D0. The options are a package-wide
duplicate check in the driver, or a check in the resolver over the
session's bound names. The repair refuses the example and cross-source
duplicates, and stable-symbol naming no longer lets a later binding take
an admitted declaration's spelling.

## Acceptance

- **AC-1.** Each repro row with a shadowing example, and the two-source
  plain duplicate, is refused with `DuplicateDefinition` naming the name.
- **AC-2 (control).** The unrelated-example row keeps hash
  `48e43379ae5466ab`, and the existing example and package-route rows keep
  their results.
- **AC-3.** A catalog and corpus census at the base shows no package newly
  refused. Any hit is the stop below.
- **AC-4 (mutation, QA).** Restoring the per-unit-only check reddens AC-1.

## Stop conditions

- A catalog or corpus package is newly refused: stop with the list.
- The repair needs a kernel, trust or spec change.
