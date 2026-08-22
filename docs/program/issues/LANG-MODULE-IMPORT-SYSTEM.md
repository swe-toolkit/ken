---
id: LANG-MODULE-IMPORT-SYSTEM
title: "Module/import capability campaign — declaration visibility (public export), a selective-import surface, and cross-package plus prelude symbol resolution, sufficient for catalog packages to reuse canonical modules instead of reimplementing them"
status: draft
owner: language
size: XL
gate: none
depends_on: []
blocks: [CAT-GCD-REFACTOR]
github: null
origin: "Operator directive 2026-08-22: module/import is its own campaign and, because it blocks the foundation catalog trial, ranks above the smaller language surface-syntax items. Framing is routed to the spec enclave (spec-surface) plus the Architect (elaborator/loader component-design); the language ring builds it. Steward-filed per COORDINATION section 2."
---

> # CAMPAIGN ROOT — framing in flight with the enclave (2026-08-22)
>
> Framing routed at spec-leader/spec-author evt_wg636336eymv (spec-surface) and
> the Architect (component-design, joining after the runtime native framing).
> Member WP nodes are filed by the Steward from the enclave decomposition; this
> root carries the objective and the measured gap only.

## The measured gap (Steward, on main 2026-08-22)

The catalog-reusability standard requires a catalog entry to IMPORT canonical
operations rather than reimplement them. Ken cannot do that today:

- No public-export / import surface. `add`/`mul`
  (`catalog/packages/Data/Numeric/Nat/Arithmetic.ken.md:16,22`) and
  `leq_nat`/`sub` (`Order.ken.md:37,208`) are plain `fn`s — no `pub` marker — and
  there is no `import`/`use` statement anywhere in `catalog/packages/`. Each
  package elaborates as an island. No module-visibility surface exists under
  `spec/30-surface/`.
- No cross-package / prelude symbol resolution for standalone checking.
  `Arithmetic.ken.md`'s own proofs call `cong` (l.33/41/50...), which has no
  definition in `catalog/packages/`; checked in isolation the module fails
  `UnresolvedCon { name: "cong" }`. So even with an import keyword, a dependency
  module does not resolve on its own to be depended upon.

## Objective

Give Ken a module system sufficient for catalog reuse:

1. Declaration visibility — public export markers.
2. A selective-import surface — grammar and elaboration to bring named public
   declarations from one package into another.
3. Cross-package + prelude symbol resolution, so a package can both import
   canonical operations and resolve shared lemmas.

Success criterion: a catalog entry (`Gcd`) imports `add`/`mul` from
`Data/Numeric/Nat/Arithmetic` and `leq_nat`/`sub` from `Data/Numeric/Nat/Order`
instead of reimplementing them, and both dependency modules check.

## Framing split (who designs what)

- Spec enclave (spec-leader/spec-author): the spec surface — `spec/30-surface/`
  `33-declarations` (visibility), `32-grammar` (import syntax), `39-elaboration`
  (resolution/loader semantics). Likely one or more spec-enclave-owned member
  WPs.
- Architect: the elaborator/loader component-design and the WP decomposition.
- Steward: files the member WP nodes from the decomposition; sequences the build.

## Sequencing (operator directive)

Frame the campaign with the enclave WHILE the language ring continues its current
work; do NOT interrupt the in-flight FO re-elaboration
([[V3-FO-SOUNDNESS-SCT-EXPRESSIBILITY]]). When the campaign is READY (framed, WPs
shovel-ready), the language ring's then-current WP finishes and the ring switches
to this campaign AHEAD of the remaining surface-syntax items
([[LANG-SYMBOLIC-OPERATOR-NAMES]] and siblings). This campaign unblocks the
factored catalog trial ([[CAT-GCD-REFACTOR]], and the remaining CAT-* trial WPs
authored factored).
