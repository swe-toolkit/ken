---
id: LANG-QUALIFIED-ACCESS-REQUIRES-IMPORT
title: "make resolve_ref grant qualified access P.foo only through a granted authority -- a qualified import of P (import P or import P as N), P's local inline declaration in the current file unit, or P being an inline child of a module the current module imports in qualified form -- always checking foo against P's public export table, as spec 33-declarations sections 3.1-3.2 state, instead of resolving against any already-loaded module's exports"
status: active
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

Qualified resolution of `P.foo` succeeds only through one of three granted
authorities, and the leaf is always looked up in the exact public export
table of the resolved module. Nothing resolves against an ambient loaded
export (`exports.get(prefix)` with no grant).

1. **Qualified import.** The current module imports `M` in qualified form,
   either `import M` or `import M as N` (through `N`). A selective-only
   import, or no import, gives `UnboundName` for `M.foo`.
2. **Local inline declaration** (Architect `evt_63xfjnpep5e34`, spec 33
   §3.1). Inside file unit `A`, declaring `module N { ... }` lets `A`'s own
   scope name `A.N.x`. A sibling module in `A` gets it only through
   `import N`, which normalizes to the same canonical `A.N` that the loader
   and the child's export key use.
3. **Inline descendant of an imported owner.** `import A` in unit `B`
   grants `A.N.x` for a child `N` declared inline in `A`; `import A as K`
   grants `K.N.x` and not `A.N.x`. A selective import of `A` or a mere load
   of `A` grants nothing nested. Ownership comes from recorded declaration
   provenance, never from a dotted-string prefix: a separately file-backed
   `A.N` is not an inline descendant.

The unqualified branch, the strict-mode floor and `resolve_class_ref` /
`resolve_attached_ref` routing are unchanged except through these rules.
Landed as `04b30eab3` with arm 1 only; arms 2 and 3 repair the regression
measured in `evt_7ffv9v4tvkpxc`.

## Acceptance criteria

- **AC-1 (falsifier, run first).** A new test in which module A selectively
  imports `M (foo)` and references `M.bar` elaborates on current `main`. That
  shows the leak. It must refuse with `UnboundName` after the fix. A second
  case (no import of `M`, and `M` loaded by a sibling) behaves the same way.
- **AC-2.** Controls stay green: `import M` gives `M.foo`; `import M as N`
  gives `N.foo`; a selective import gives bare `foo`.
- **AC-3.** Every catalog package still elaborates, and the
  `r_layer_tests` import and roster suites stay green. A fixture that relied
  on the leak gets the missing import. The no-ambient-export rule for
  unrelated modules is never relaxed; the local-declaration and
  inline-owner arms above are not relaxations. List every such fixture in
  the handback.
- **AC-3a (roots loader).** Through `elaborate_module_from_roots`, with
  `A` and `B` as real file units and `module N` and a sibling `module P`
  declared inline in `A.ken`:
  same-unit `A.N.x`; sibling `import N` / `N.x`; cross-unit `import A` /
  `A.N.x`; alias `K.N.x`; selective and no-import controls refusing. Pair
  the private `A.N.s` refusal (from `A`'s outer scope, `P` and `B`) with the
  public `x` acceptance. The landed arm-1 negatives stay discriminating.
- **AC-4.** Targeted builds only, through `scripts/ken-cargo -p
  ken-elaborator` and the named tests. No-regression means green in CI.

## Stop conditions

- If a catalog package, or the prelude's own resolution, cannot be fixed
  by adding an import, STOP and report it verbatim to the Steward.
- If an inline child is still unreachable after the three allowed routes,
  or making it reachable needs an ambient export bypass, STOP and report it.
  Never add a fictitious `import A.N` file dependency to make it work.
- **Not this node:** merging base's duplicate `IsTrue`
  (`decimal_char.rs:223`) with `Core.Classes.LawfulClasses.IsTrue`.
