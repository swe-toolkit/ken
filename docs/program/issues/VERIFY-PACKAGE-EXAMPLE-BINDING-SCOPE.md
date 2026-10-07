---
id: VERIFY-PACKAGE-EXAMPLE-BINDING-SCOPE
title: "Since VERIFY-PACKAGE-ROUTE-EXAMPLE-DECLARATIONS a .ken.md source's example-fence declarations are dropped from the package but stay bound in the shared env, so a later source can reference one: the package validates with a dangling symbol and an executable entrypoint, and only erasure refuses. Close the reference before emit"
status: active
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

The repro is refused before the driver returns Ok, by one of two
closures: example bindings are not visible to later sources, or emit or
validation refuses a dangling reference. **The Architect picks one at
AC-0**, before any edit. The single-source fix of the predecessor is kept.

## Acceptance

- **AC-0.** The Architect's choice, with the spec section it rests on.
- **AC-1.** The repro is refused before Ok on all three package kinds,
  with a diagnostic naming `zz_ex`.
- **AC-2 (controls).** `const main : Bool = base` still compiles and
  erases Ok. The predecessor's literate-versus-`.ken` hash equality
  (`1e355da83ed6da8f`) is unchanged. A same-source example still executes.
- **AC-3 (mutation).** Reverting the closure makes the repro return Ok
  again, which reddens AC-1, while the AC-2 controls stay green.
- **AC-4.** A later source naming an example-declared inductive or
  constructor is refused the same way. The Adversary did not measure this
  case.

## Stop conditions

- Any kernel, `trusted_base()` or spec change.
- A catalog or corpus package whose later source references an example
  declaration: stop with the list.
