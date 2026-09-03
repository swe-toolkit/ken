---
id: CAT-MIGRATE-EC-APPLICATIVE-PROVIDERS
title: "Scaffold-retirement Tier C predecessor: publish EC's private apply_to/compose/functor_map_of/Applicative (visibility-only widen) so Validation can selectively import them, once EC itself roots-loads clean under the Language faces-3 cross-module export+import. Proven EC/Tier-B provider-publication shape; mints nothing, changes no body."
status: draft
owner: foundation
size: S
gate: none
tier: T2
depends_on: [LANG-ROOTS-LOADER-LOCAL-INSTANCE-DICT-SCOPE]
blocks: []
github: null
origin: "Steward, 2026-09-03, cite-and-split from the Tier C D0 census (foundation evt_19kq7r92attpy, Architect confirmation evt_4hp6qxkdaqgbz). The census found Validation consumes EC apply_to/compose/functor_map_of/Applicative, all private. The Architect ruled a visibility-only EC provider-widen (same shape as the 10 published EC-closure providers). BUT EC itself roots-loads RED at Functor / Functor_instance_Identity — a cross-module export+import that the Language roots-loader faces-3 predecessor ([[LANG-ROOTS-LOADER-LOCAL-INSTANCE-DICT-SCOPE]] follow-up increment) must land first, so this widen is HELD behind that language work, not releasable on its own like the LF-Semigroup widen. Predecessor for the Validation arm of [[CAT-MIGRATE-TIER-C-NONEMPTY-VALIDATION]]."
---

> # Tier C predecessor: publish EC's private Applicative-family providers.
> # Visibility-only, the proven EC/Tier-B shape. HELD behind the Language
> # roots-loader faces-3 work, because EC roots-loads red until that lands.

Validation needs EC's `apply_to`, `compose`, `functor_map_of`, and `Applicative`,
all currently private. This node publishes them by the same visibility-only
mechanism the EC-closure providers used ([[CAT-MIGRATE-EC-CLOSURE-PROVIDERS]],
landed). Unlike the LF-Semigroup widen, it is NOT releasable on its own: EC
roots-loads red at `Functor` / `Functor_instance_Identity`, which the Language
roots-loader faces-3 cross-module export+import predecessor must fix first.

## Blocked-on

- **EC roots-loads clean** — [[LANG-ROOTS-LOADER-LOCAL-INSTANCE-DICT-SCOPE]]
  faces-3 (cross-module export+import) must land so EC loads without the
  `Functor` / `Functor_instance_Identity` red. Re-measure EC's roots-load at the
  release SHA before authoring the widen.

## Deliverable

- **D1 — publish EC `apply_to`/`compose`/`functor_map_of`/`Applicative`.** Add
  `pub` to exactly these (and any class-adjacent symbol a downstream selective
  import requires, measured — do not over-publish). Extend EC's loader-visible
  inventory by exactly the newly-public names. No body moves; no instance minted.

## Acceptance criteria — proven EC / Tier-B provider shape

- **AC-EXPORTED.** Each published symbol is loader-visible from EC — a selective
  import resolves it to EC's `GlobalId`. Control: a still-private sibling in EC
  still rejects `UnboundName`.
- **AC-VISIBILITY-ONLY.** A differential shows each body BYTE-UNCHANGED;
  publishing mints no second class/instance; every consumer resolves to the
  single existing EC owner. No computational change.
- **AC-EXACT-INVENTORY.** EC's loader-visible inventory extends by EXACTLY the
  intended names; a per-symbol reddening mutation reds distinctly.
- **AC-STANDALONE-GREEN (precondition).** EC roots-loads clean (exit 0) at the
  release SHA — the faces-3 predecessor has landed — BEFORE this widen is
  authored; otherwise the publication rides on a red load.
- **AC-NO-REGRESSION.** Complete affected-target closure, scoped by changed
  paths, targeted via `scripts/ken-cargo`, never `--workspace` (green in CI).

## Gate, reviewer, sequencing

`gate: none`. Candidate: **Architect** (required — visibility-only + no
over-publication) + **Foundation QA + CV** on the exact SHA, then Steward M1-M4
-> lieutenant. Predecessor to [[CAT-MIGRATE-TIER-C-NONEMPTY-VALIDATION]]
(Validation arm); held behind the Language faces-3 work.
