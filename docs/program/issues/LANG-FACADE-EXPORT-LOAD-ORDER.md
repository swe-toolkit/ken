---
id: LANG-FACADE-EXPORT-LOAD-ORDER
title: "A facade export M (…) must select the same provider whatever the caller loaded first: when the unit also declares an inline module M, the facade today fails cold but is admitted, publishing the file M's names, when a caller imported M earlier; route the facade edge through the same order-independent identity as import"
status: ready
owner: language
size: S
gate: architect
tier: T1
depends_on: [LANG-IMPORT-LOAD-ORDER-INDEPENDENCE]
blocks: []
github: null
origin: "Adversary M8 on 791d0fc2f (evt_5n8nhxesrbwz4), MEDIUM, not a regression. The fifth loader edge left out of LANG-IMPORT-LOAD-ORDER-INDEPENDENCE, whose Objective and operator authorization ('concur with rec on load order bug. fix it.', 2026-09-24) it falls under. Steward-filed per COORDINATION section 2."
---

# A facade export still borrows its caller's imports

## Objective

The last loader edge meets the landed WP's objective: a unit's meaning does
not depend on which files its callers loaded first (spec `33 §3.3`). Spec
`33 §3.2` makes `export M (…)` "a loader dependency edge to `M`, using the
role-blind dotted-path identity".

## Fixed inputs -- the Adversary's measurement at `791d0fc2f`

Re-establish these; the Adversary's scratch test was deleted.

- `N.ken`: `pub const x : Nat = Suc (Suc Zero)`. `A.ken`: `module N { pub
  const x : Nat = Zero }` and `export N (x)`, in either order. `D.ken`:
  `import A (x)` and `pub const y : Nat = x`. `C1.ken`: `import N` then
  `import D`. `C2.ken`: `import D` then `import N`.
- One fresh `ElabEnv` per entry: A, D and C2 give `UnboundName N` at the
  facade; C1 is **admitted**, and `D.y`'s body is the file `N.x`, not the
  inline `A.N.x`. With the facade inside `module P`, C1's diagnostic changes
  to `UnboundName A.x`. Base `d7f6c2bcf` gives the same output.
- The mechanism, per the Adversary: the `load_unit` pre-scan
  (`modules.rs:1509`) filters facade edges through `declared_inline_import`
  (`imported_module_paths` pushes `ExportForm::Facade` at `:1221`), so a
  facade naming a same-unit child is never loaded as a file. The
  `ExportDecl` arm (`:3534`) sets `selected_file = None`, and `apply_export`
  falls through to the ambient `exports.get(module)` table (`:1106`). That
  is the fall-through the landed WP removed from `apply_import`.

Treat anchors as perishable. If a fixed input is false on the landed base,
stop and report the mismatch; do not build around it.

## Deliverable -- provider rule pinned by the Architect (`evt_66d3vzv6d2c7`)

The facade follows the landed WP's Case A/B classifier, not "always file
`N`". Route `export N (…)` through the same classifier and provider
selection as `import N`: `lexical_inline_import` over this unit's ordered
inline set, then the same pubmap and member-id selection `apply_import`
uses. Factor that selection into one helper called by both `apply_import`
and `apply_export`'s facade arm, and delete the `ExportDecl` arm's separate
`selected_file` computation.

1. Available same-unit child → that child's exports at its canonical path.
2. Same-unit child declared later or not yet expanded → `UnboundName N` at
   the facade: never ambient `exports["N"]` and never file `N`.
3. Absolute → the catalog-root file `N` for file units, and the paired
   export provenance for in-memory units, exactly as `import`.

The pre-scan keeps filtering facade edges through `declared_inline_import`.
Do not exempt `ExportForm::Facade`. No `apply_export` path reads a
bare-name export table.

## Acceptance

- **AC-1 (red first, expected values).** On the candidate:
  - The `module N` then `export N (x)` order: A, D, C1 and C2 are all
    admitted, and `D.y`'s body is the inline `A.N.x` GlobalId, never file
    `N.x`, in every entry order.
  - The `export N (x)` then `module N` order: all four reject
    `UnboundName N` at A's facade.
  - File positive: A without an inline `N` selects file `N.x` cold and in
    every order.
  - `module P` variant: the consumer must reach the name through `P`'s
    interface. State the exact consumer spelling and show it resolves in
    the positive order. A variant whose every entry fails at D's own
    `import A (x)` measures D, not the facade, and does not count.
  At base the verdicts differ by entry order.
- **AC-2 (two mutations, one per rejected rule).** Both must redden AC-1.
  - (a) Restoring the bare `exports.get(module)` fall-through re-admits C1
    in the later-declared order.
  - (b) Selecting file `N` in the Available branch (the always-file rule)
    changes the positive order's provider to file `N.x`.
- **AC-3.** The landed WP's Case A and Case B pins, `l4_export_reexport`,
  `file_facade_uses_source_file_exports_not_memory_shadow` and the
  whole-catalog load stay green. Targeted builds only, through
  `scripts/ken-cargo`. No-regression means green in CI.

## Stop conditions

Stop if the pinned provider rule needs a spec change, or if an existing
catalog unit changes meaning under it.
