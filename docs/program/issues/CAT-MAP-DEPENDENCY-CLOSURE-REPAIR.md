---
id: CAT-MAP-DEPENDENCY-CLOSURE-REPAIR
title: "Repair the Map package's dependency closure so Data/Collections/Map.ken.md elaborates from its own declared imports rather than relying on the map_build_acceptance.rs fixture to preload Compare/Transport/Derived/Or and to resolve undeclared list_append."
status: active
owner: foundation
size: unsized
gate: none
tier: T2
depends_on: []
blocks: []
github: null
origin: "Steward, 2026-08-30, filed on the Architect ruling evt_em72d9eh6ndg (base 0ddd49b3) taken during CAT-BOOL-REUSE-CONSUMERS D2's pre-edit hard stop. The ruling: the Map raw-standalone failure is a PRE-EXISTING module-dependency defect, not a D2 transition, and a separate closure repair is warranted but must NOT gate the one-provider is_some drain. Recorded as a distinct follow-on per that ruling; QUEUED, not released. Steward-filed per COORDINATION section 2."
---

> # RELEASED 2026-09-10 (Steward) — the D0 breadth census. `active`.
>
> Filed (Architect `evt_em72d9eh6ndg`) as a tracked follow-on whose governing
> design question — must every catalog module elaborate from its own declared
> imports, or is fixture-provided ambient closure acceptable? — was
> operator-reserved. **The operator RULED it (2026-09-10): every catalog module
> MUST elaborate from its own declared imports. Rationale (mission-grounded): the
> catalog is the basis for users of Ken to build real programs and libraries; a
> package that cannot elaborate from its own declared imports is useless for that
> purpose.** The constraint is settled; the requirement is catalog-WIDE.
>
> **RELEASED to the foundation ring** (kick evt_13znqcbt7w1ge) once DS-9 was
> grounded as show_int-blocked (foundation-leader evt_5165pecbm48zv — no honest
> D4/D5/D5a slice survives without the number printer). D0 = the catalog-wide
> standalone-import census (raw `ken-cargo run -p ken-cli -- check <pkg>` per
> package; enumerate each undeclared dependency + its provider). The census is
> the sizing input for the Architect decomposition, handed back to the Steward.
> A minimal additive-declare candidate is acceptable if the census shows the
> repair is trivial (Foundation QA + CV -> Steward M1-M4); otherwise report-only
> and the Steward routes the decomposition. Build-lock caveat: kernel (L2) and
> runtime (L1) build locally on the shared serialized lock — the census is
> check-based, but do not hold the lock in a long foreground loop.

## The measured defect (Architect `evt_em72d9eh6ndg`, base `0ddd49b3`)

`Data/Collections/Map.ken.md` (blob `2d97ea7a4745705102d71c69b13d662fc6e60d79`)
imports only `Core.Logic.Or` at line 80, then consumes undeclared `list_append`
at lines 90-93. The raw consumer command

```
scripts/ken-cargo run -p ken-cli -- check catalog/packages/Data/Collections/Map.ken.md
```

exits 1 with `UnresolvedCon { name: "list_append", span: Span { start: 4210, end:
4221 } }`. Map's established acceptance path is fixture-backed:
`map_build_acceptance.rs` (blob `82576c772e5be8b76cc829f0ab5c2ca7948c1cab`),
`mk_env` lines 38-47, preloads Compare, Transport, Derived, and Or before
elaborating the real Map source. So Map does not elaborate from its own declared
imports today; the fixture supplies the closure.

## Scope — settled design, open decomposition (operator ruling 2026-09-10)

The design question is CLOSED (operator: import-self-sufficiency is required,
above). What remains before release is decomposition and sizing, NOT a design
call:

- **Breadth is a D0 measurement.** Map's `list_append` is one instance; the
  requirement is catalog-wide, so D0 must census which catalog packages fail to
  `check` standalone (raw `ken-cargo run -p ken-cli -- check <pkg>`, not through
  a preloading fixture) and enumerate each undeclared dependency. That census is
  the sizing input — it decides whether this is one repair node or a per-package
  campaign. Run it when foundation picks the node up (not on the local build
  lock while another lane is building).
- **Architect decomposition owed at release.** The Steward engages the Architect
  to decompose the census result into WP(s) and route each; the repair may be
  purely additive (declare the missing imports) or may surface real ordering or
  provider gaps per package.
- **Applies to DS-9 too.** DS-9's JSON codec is a catalog package, so under this
  ruling it must elaborate from its own declared imports — carried as an AC into
  the DS-9 re-release, not deferred to this node.
- **Standing requirement, beyond this node.** Every future catalog authoring /
  migration node carries an AC that the module elaborates standalone; foundation
  QA and the Librarian's as-built mandate enforce it. To be codified (ADR +
  review criterion).
