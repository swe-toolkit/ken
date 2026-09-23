---
id: LANG-QUALIFIED-ACCESS-REQUIRES-IMPORT
title: "make resolve_ref grant qualified access M.foo only when the current module has a qualified import of M (import M or import M as N), as spec 33-declarations section 3.2 states, instead of resolving against any already-loaded module's exports"
status: ready
owner: language
size: S
gate: none
tier: T1
depends_on: [CAT-CONFIGURATION-DECODER-IMPORT-EDGES]
blocks: []
github: null
origin: "Operator ruling 2026-09-23, concurring with Steward recommendation evt_4w5ep25w9gqbe: wake L2 for this one WP. Defect found by Adversary evt_3spntbt1amt0p on the Decoder; the catalog side landed as CAT-CONFIGURATION-DECODER-IMPORT-EDGES (1ed04b835). Steward-filed per COORDINATION section 2."
---

# Qualified names resolve without an import

## Settled inputs -- measured at `11373f9f6`. Re-ground before acting.

- `spec/30-surface/33-declarations.md §3.2`: `import M` gives qualified
  access `M.foo`. A selective `import M (foo, Bar)` brings exactly those names
  **unqualified** and "nothing else of `M`".
- `crates/ken-elaborator/src/modules.rs::resolve_ref` (`:350`), dotted
  branch: when the prefix is not a scope binding, it maps it through
  `scope.prefixes` (aliases) or uses it verbatim. It then returns the leaf
  from `exports.get(&canonical_module)`, the export map of **any loaded
  module**. Nothing checks that the current module imported it.
- Consequence: `M.foo` resolves whenever some other module loaded `M`. The
  Decoder relied on this for 30 `Data.Collections.Derived.nth` references,
  since fixed in the catalog (`1ed04b835`). The Adversary's 53-module scan
  found no other catalog module relying on it.

## Deliverable

Qualified resolution succeeds only if the prefix names a module the current
module imports in qualified form, either `import M` or `import M as N`
(through `N`). A selective-only import, or no import, gives `UnboundName`
for `M.foo`. The unqualified branch, the strict-mode floor and
`resolve_class_ref` / `resolve_attached_ref` routing are unchanged except
through this one rule.

## Acceptance criteria

- **AC-1 (falsifier, run first).** A new test in which module A selectively
  imports `M (foo)` and references `M.bar` elaborates on current `main`. That
  shows the leak. It must refuse with `UnboundName` after the fix. A second
  case (no import of `M`, and `M` loaded by a sibling) behaves the same way.
- **AC-2.** Controls stay green: `import M` gives `M.foo`; `import M as N`
  gives `N.foo`; a selective import gives bare `foo`.
- **AC-3.** Every catalog package still elaborates, and the
  `r_layer_tests` import and roster suites stay green. A fixture that relied
  on the leak gets the missing import. The rule is never relaxed. List every
  such fixture in the handback.
- **AC-4.** Targeted builds only, through `scripts/ken-cargo -p
  ken-elaborator` and the named tests. No-regression means green in CI.

## Stop conditions

- If a catalog package, or the prelude's own resolution, cannot be fixed
  by adding an import, STOP and report it verbatim to the Steward.
- **Not this node:** merging base's duplicate `IsTrue`
  (`decimal_char.rs:223`) with `Core.Classes.LawfulClasses.IsTrue`.
