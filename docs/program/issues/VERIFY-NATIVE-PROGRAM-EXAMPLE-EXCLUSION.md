---
id: VERIFY-NATIVE-PROGRAM-EXAMPLE-EXCLUSION
title: "prepare_native_program_sources elaborates .ken.md example fences into the shared env, then admits every env declaration and passes an empty example set to emit_package_from_env, so a native-program package ships example declarations. Exclude example declarations on the native-program route as the package routes do"
status: active
owner: verify
size: S
tier: T2
gate: architect
depends_on: [VERIFY-PACKAGE-EXAMPLE-BINDING-SCOPE]
blocks: []
github: null
origin: "Architect carry evt_6qc2akyd4457c at the VERIFY-PACKAGE-EXAMPLE-BINDING-SCOPE review: pre-existing on the native-program route, outside that WP's scope. Steward-filed per COORDINATION section 2."
---

# Example declarations stay out of native programs

## Objective

A native-program package carries no `ken example` fence declaration, and
a `main` that references one is refused before emit, as on the package
routes.

## Settled inputs (measured at `e1b609597`)

- `compiler_driver.rs:2315-2324`: each source elaborates with
  `elaborate_ken_md_file_v1`, which also returns example declarations, and
  all of them join `admitted_ids`.
- `compiler_driver.rs:2349-2359`: `admitted_ids` is extended with every
  `env` declaration, and `emit_package_from_env` gets `&BTreeSet::new()`
  as the example set.
- The package routes collect `example_ids` (`:644-693`, `:1246-1267`), and
  `VERIFY-PACKAGE-EXAMPLE-BINDING-SCOPE` refuses a reference outside the
  package with `PackageReferenceOutsidePackage`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

`prepare_native_program_sources` collects each source's example ids, keeps
them out of `admitted_ids` (including the env-closure extension), and
passes them to `emit_package_from_env`. Prelude definitions that `main`
reaches stay admitted.

## Acceptance

- **AC-1.** A two-source native program whose `.ken.md` source has an
  example fence emits a package without the example declaration, and its
  plan and host spine are unchanged.
- **AC-2.** A `main` that references an example declaration is refused
  before emit with `PackageReferenceOutsidePackage` naming it.
- **AC-3 (control).** `rt_parity_native` and the native-build CLI rows
  keep their results.
- **AC-4 (mutation, QA).** Passing the empty example set again reddens
  AC-1 and AC-2.

## Stop conditions

- A catalog or corpus native program references an example declaration:
  stop with the list.
- Any kernel, `trusted_base()` or spec change.
