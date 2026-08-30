---
id: CAT-MAP-DEPENDENCY-CLOSURE-REPAIR
title: "Repair the Map package's dependency closure so Data/Collections/Map.ken.md elaborates from its own declared imports rather than relying on the map_build_acceptance.rs fixture to preload Compare/Transport/Derived/Or and to resolve undeclared list_append."
status: draft
owner: foundation
size: unsized
gate: none
tier: T2
depends_on: []
blocks: []
github: null
origin: "Steward, 2026-08-30, filed on the Architect ruling evt_em72d9eh6ndg (base 0ddd49b3) taken during CAT-BOOL-REUSE-CONSUMERS D2's pre-edit hard stop. The ruling: the Map raw-standalone failure is a PRE-EXISTING module-dependency defect, not a D2 transition, and a separate closure repair is warranted but must NOT gate the one-provider is_some drain. Recorded as a distinct follow-on per that ruling; QUEUED, not released. Steward-filed per COORDINATION section 2."
---

> # DRAFT — recorded as a distinct follow-on, NOT released. `draft`.
>
> Filed only so the pre-existing defect the Architect surfaced during
> `CAT-BOOL-REUSE-CONSUMERS` D2 is tracked. It is NOT a lane objective and NOT a
> prerequisite for that drain. Do not start it without a Steward release, and
> interrogate the constraint's grounding and priority (`steward.md` §4c) before
> framing it into a real deliverable.

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

## Scope note (why this is not yet a framed deliverable)

The repair is potentially broad: it may span the full legacy fixture inventory,
not just `list_append`. Whether Ken should require every catalog module to
elaborate from its own imports (vs. an intentional fixture-provided ambient
closure) is a design/priority question for the operator and Architect, not a
settled constraint. Frame it only when the constraint is grounded and the lane
allows — until then it stays a tracked draft.
