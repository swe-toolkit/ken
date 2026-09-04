---
id: CAT-MIGRATE-LF-SEMIGROUP-PUBLISH
title: "Scaffold-retirement Tier C predecessor: publish LF's private class Semigroup (visibility-only widen), so NonEmpty can selectively import it instead of resolving it ambiently. Exactly the proven Tier-B provider-publication shape; mints no second class, changes no body."
status: merged
owner: foundation
size: S
gate: none
tier: T2
depends_on: []
blocks: []
github: null
origin: "Steward, 2026-09-03, cite-and-split from the Tier C D0 census (foundation evt_19kq7r92attpy, Architect confirmation evt_4hp6qxkdaqgbz). The census found NonEmpty consumes LF Semigroup, which is private (LF:40). The Architect ruled it a visibility-only provider-widen on LF (pub class Semigroup, eligible exactly like the 10 decls already published in CAT-MIGRATE-EC-CLOSURE-PROVIDERS), a small PREDECESSOR for NonEmpty — split it out, NOT a NonEmpty-internal edit. No unlanded dependency: LF is already published; this is a pub-flip on landed code, releasable independently of the ready lane. Held (draft) only because the foundation seat is single-threaded on the ready lane; released one-ahead when the ready lane nears completion, ahead of [[CAT-MIGRATE-TIER-C-NONEMPTY-VALIDATION]]."
---

> # Tier C predecessor: publish LF's private class Semigroup. Visibility-only.
> # The proven Tier-B / EC provider-publication shape: pub the class so a
> # downstream selective import resolves it; no second class, no body change.

`Semigroup` in LF is private (LF:40). NonEmpty needs it. This node publishes it
by the same visibility-only mechanism the 10 EC-closure providers used
([[CAT-MIGRATE-EC-CLOSURE-PROVIDERS]], landed): add `pub`, nothing else. It is
NOT a NonEmpty edit and NOT a class relocation — the class stays in LF; only its
visibility changes, so a downstream module can selectively import the single
existing owner.

## Deliverable

- **D1 — publish LF `Semigroup`.** Add `pub` to the class `Semigroup` at LF:40
  (and any class-adjacent symbol a downstream selective import of it requires,
  measured — do not over-publish). Extend LF's loader-visible inventory by
  exactly the newly-public name(s). No body moves; no instance is minted.

## Acceptance criteria — proven EC / Tier-B provider shape

- **AC-EXPORTED.** `Semigroup` is loader-visible from LF — a selective import
  resolves it to LF's `GlobalId`. Control: a still-private sibling in LF still
  rejects `UnboundName`.
- **AC-VISIBILITY-ONLY.** A differential shows the class body BYTE-UNCHANGED;
  publishing mints no second class/instance; every consumer resolves to the
  single existing LF owner. No computational change.
- **AC-EXACT-INVENTORY.** LF's loader-visible inventory extends by EXACTLY the
  intended name(s); a per-symbol reddening mutation reds distinctly.
- **AC-NO-REGRESSION.** Complete affected-target closure, scoped by changed
  paths, targeted via `scripts/ken-cargo`, never `--workspace` (green in CI is
  the workspace verdict).

## Gate, reviewer, sequencing

`gate: none`. Candidate: **Architect** (required — visibility-only + no
over-publication) + **Foundation QA + CV** on the exact SHA, then Steward M1-M4
-> lieutenant. Predecessor to [[CAT-MIGRATE-TIER-C-NONEMPTY-VALIDATION]] (NonEmpty
arm). Releasable now on its own merits; sequenced one-ahead of the held node so
NonEmpty's import target exists when that node releases.
