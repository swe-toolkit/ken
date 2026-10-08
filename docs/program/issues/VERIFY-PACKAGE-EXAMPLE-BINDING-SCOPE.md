---
id: VERIFY-PACKAGE-EXAMPLE-BINDING-SCOPE
title: "Since VERIFY-PACKAGE-ROUTE-EXAMPLE-DECLARATIONS a .ken.md source's example-fence declarations are dropped from the package but stay bound in the shared env, so a later source can reference one: the package validates with a dangling symbol and an executable entrypoint, and only erasure refuses. Close the reference before emit"
status: merged
owner: verify
size: S
tier: T2
gate: architect
depends_on: [VERIFY-PACKAGE-ROUTE-EXAMPLE-DECLARATIONS]
blocks: []
github: null
origin: "Adversary finding evt_7ns7vn638jktf on 1153a9fc6. Regression of VERIFY-PACKAGE-ROUTE-EXAMPLE-DECLARATIONS on the multi-source route; fails closed at erasure. Steward-filed per COORDINATION section 2."
---

# An example declaration is not visible to the package

## Objective

No admitted declaration of a package references a symbol the package
neither carries nor lists as external. A later source that names an
earlier source's example-fence declaration is refused before emit.

## Settled inputs (Adversary, measured at `1153a9fc6`)

- `compiler_driver.rs:1234-1245`: sources share one `env`. Example ids are
  collected but never unbound.
- `compiler_driver.rs:3414`: example ids are filtered out of
  `semantic.symbols` and never admitted.
- `lib.rs:585-586`: examples execute into the shared env.
- **Repro.** `compile_ken_package_sources`, package `zz_adv_pkgex`, target
  `main`. `a.ken.md` has a `ken` fence `const base : Bool = True` and a
  `ken example` fence `const zz_ex : Bool = base`. `b.ken` has
  `const main : Bool = zz_ex`. The driver returns Ok for NonRuntime,
  Library and Executable, with declarations `[base, main]`. Erasure refuses
  with `body_reference_outside_selected_closure`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

Emit refuses a dangling reference before Ok: admitted content is encoded
against the package's own symbol set (Architect AC-0 `evt_1se89bhmwwwht`,
closure (2), spec 46 §1.1). In `emit_package_from_env`, a `package_table`
holds only non-example symbols; admitted declarations and obligation
metadata encode through it, and a `MissingStableSymbol` for a symbol the
package drops maps to `PackageReferenceOutsidePackage { declaration,
referenced }`, whose `Display` names both. Every other section built by
walking `env` rather than `admitted` either skips `example_ids` or encodes
through `package_table`; the handoff lists each one. The single-source fix
of the predecessor is kept.

## Acceptance

- **AC-0.** Done: closure (2), spec 46 §1.1 (`evt_1se89bhmwwwht`).
- **AC-1.** The repro is refused before Ok on NonRuntime, Library and
  Executable, with `PackageReferenceOutsidePackage` naming `zz_ex`.
- **AC-2 (controls).** `const main : Bool = base` still compiles and
  erases Ok. A same-source example still executes. The landed
  `compiler_driver.rs` test
  `package_and_denotation_exclude_example_declarations_from_admission`
  stays green with its assertions unedited: package hash
  `0x5ed0_ae5b_69b5_41df`, denotation hash `0xa102_7073_c2a9_9b86`, and
  literate equal to `.ken` for both.
- **AC-3 (mutation, M-full-table).** Encoding admitted declarations with
  the full `table` returns Ok on the repro, which reddens AC-1, while the
  AC-2 controls stay green.
- **AC-4.** A later source naming an example `data` type, and one naming
  its constructor, are each refused, naming the type or the constructor.
- **AC-4b.** An example fence declares an `instance` of a module class,
  and a later source's call resolves to it without naming it. It is
  refused the same way. This row measures reach that does not go through
  a name.
- **Census.** Before any source edit, report the catalog and corpus
  multi-source packages whose later source references an example
  declaration (expected none; any hit is the stop below).

## Stop conditions

- Any kernel, `trusted_base()` or spec change.
- A catalog or corpus package whose later source references an example
  declaration: stop with the list.

## Closeout

Merged `e1b609597` from exact `e5f6a86e9` (PR #4591). Verify QA
`evt_2x4vh4nbapt4e`, Architect re-review after changes requested
`evt_6qc2akyd4457c`, Decision `dec_1ce408cxz57hf`. Obligations route by
owner, and a later source naming an earlier source's example declaration
is refused before emit. Carry: the native-program route still ships
example declarations, filed as `VERIFY-NATIVE-PROGRAM-EXAMPLE-EXCLUSION`.
