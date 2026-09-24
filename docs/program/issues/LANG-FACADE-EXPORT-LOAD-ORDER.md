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

## Deliverable

Give the facade edge one order-independent provider identity, consistent
with how the landed WP resolves `import N` beside a same-unit inline
`module N`. **The Architect pins which provider that is at frame review.**
The Adversary's reading of `§3.2` is the file `N` in every order: exempt
`ExportForm::Facade` from the pre-scan's inline filter, and select the
provider from `file_export_tables` in `apply_export`'s facade arm. No
`apply_export` path may read the ambient bare-name table.

## Acceptance

- **AC-1 (red first).** The repro above, in all four entry orders (A, D,
  C1, C2) and both declaration orders, plus the `module P` variant. At base
  the verdicts differ by order. On the candidate each entry gives the same
  verdict and the same provider `GlobalId` in every order.
- **AC-2 (mutation).** Restoring only the ambient fall-through in
  `apply_export` reddens AC-1.
- **AC-3.** The landed WP's Case A and Case B pins and the whole-catalog
  load stay green. Targeted builds only, through `scripts/ken-cargo`.
  No-regression means green in CI.

## Stop conditions

Stop if the pinned provider rule needs a spec change, or if an existing
catalog unit changes meaning under it.
