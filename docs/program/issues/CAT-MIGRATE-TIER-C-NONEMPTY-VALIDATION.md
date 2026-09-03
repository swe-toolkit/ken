---
id: CAT-MIGRATE-TIER-C-NONEMPTY-VALIDATION
title: "Scaffold-retirement Tier C, held component: migrate the {NonEmpty, Validation} WCC off fixture scaffolding onto real imports, once their split-out predecessors are published. Same per-module publish+import+standalone shape as the Tier C ready lane; NonEmpty before Validation (the intra-tier edge). NO class-instance relocation, NO invented pub instance."
status: draft
owner: foundation
size: M
gate: none
tier: T2
depends_on: [CAT-MIGRATE-LF-SEMIGROUP-PUBLISH, CAT-MIGRATE-EC-APPLICATIVE-PROVIDERS, LANG-ROOTS-LOADER-LOCAL-INSTANCE-DICT-SCOPE]
blocks: []
github: null
origin: "Steward, 2026-09-03, split out of [[CAT-MIGRATE-TIER-C-DATA-VALUE]] on the confirmed D0 census (foundation evt_19kq7r92attpy, Architect confirmation evt_4hp6qxkdaqgbz). The census measured {NonEmpty, Validation} as a WCC (edge NonEmpty -> Validation) that sits behind UNPUBLISHED split-out providers, so the DAG-axis discipline holds it out of the ready lane: NonEmpty needs private LF Semigroup ([[CAT-MIGRATE-LF-SEMIGROUP-PUBLISH]]); Validation needs private EC apply_to/compose/functor_map_of/Applicative AND EC itself roots-loads red at Functor / Functor_instance_Identity, i.e. blocked on the Language roots-loader faces-3 cross-module export+import predecessor ([[LANG-ROOTS-LOADER-LOCAL-INSTANCE-DICT-SCOPE]] follow-up) PLUS the EC provider-widen ([[CAT-MIGRATE-EC-APPLICATIVE-PROVIDERS]]). Validation also names Semigroup_instance_NonEmpty (a synthesized dict) in a checked example — that resolves ONLY via the predecessor's imported-head carry, NEVER an invented pub instance. Held until all three predecessors land; then released one behind the ready lane."
---

> # Tier C held component: {NonEmpty, Validation}. Same shape as the ready lane.
> # Per-module publish own surface, real selective import from published lower
> # tiers, extend loader inventory, standalone-green. NO class-instance
> # relocation, NO invented pub instance.

This node is the held WCC that [[CAT-MIGRATE-TIER-C-DATA-VALUE]] split out. It is
NOT a regression fix — both modules elaborate today under ambient class-install;
this brings them to the scaffold-retirement end state, exactly as the ready lane
does for SB..Vector. Do not treat standalone-red as a bug: it is the starting
condition each increment closes (see the parent frame's "Not a regression fix").

## Blocked-on (the DAG axis — release only when all three have landed)

- **NonEmpty** needs private LF `Semigroup` -> [[CAT-MIGRATE-LF-SEMIGROUP-PUBLISH]]
  (a visibility-only provider-widen on LF, minted alongside this).
- **Validation** needs private EC `apply_to`/`compose`/`functor_map_of`/
  `Applicative` -> [[CAT-MIGRATE-EC-APPLICATIVE-PROVIDERS]], AND the EC module
  roots-loads red at `Functor` / `Functor_instance_Identity`, blocked on the
  Language roots-loader faces-3 cross-module export+import predecessor
  ([[LANG-ROOTS-LOADER-LOCAL-INSTANCE-DICT-SCOPE]] follow-up increment).
- **Validation** names `Semigroup_instance_NonEmpty` (a synthesized dict) in a
  checked example: it resolves via the predecessor's imported-head carry ONLY.
  Never mint a `pub instance` to satisfy it.

## Deliverables

- **D0 — re-measure at the release SHA.** By the time the predecessors land, the
  standalone `UnresolvedCon`/`UnboundName` sets and the exact provider heads may
  have shifted; re-census both modules against the then-published providers
  before authoring the import blocks. NonEmpty before Validation.
- **D1 NonEmpty, D2 Validation — publish + import + standalone, in that order.**
  Each: publish exactly the export surface its consumers need (measured); add a
  selective import from the published lower tiers for the exact set; retire the
  ambient reach; extend the loader-visible inventory (exports + imports); the
  module elaborates standalone (exit 0).

## Acceptance criteria — the proven Tier-A / EC / ready-lane shape

Identical to [[CAT-MIGRATE-TIER-C-DATA-VALUE]]: AC-EXPORTED (per published symbol,
loader-resolved with a still-private-sibling control), AC-EXACT-INVENTORY (per
module, per-symbol reddening mutation), AC-STANDALONE-GREEN (removing the import
line restores the exact prior standalone failure), AC-VISIBILITY-ONLY (any `pub`
is a byte-unchanged body, mints no second class/instance), AC-NO-REGRESSION
(complete affected-target closure, scoped by changed paths, targeted via
`scripts/ken-cargo`, never `--workspace` — green in CI is the workspace verdict).

## Gate, reviewer, sequencing

`gate: none`. On each increment's candidate: **Architect** (required — surface
correctness + class-uniformity + the imported-head carry for the synthesized
dict) + **Foundation QA + CV** on the exact SHA, then Steward M1-M4 ->
lieutenant. Released one behind the [[CAT-MIGRATE-TIER-C-DATA-VALUE]] ready lane,
once all three predecessors have landed.
