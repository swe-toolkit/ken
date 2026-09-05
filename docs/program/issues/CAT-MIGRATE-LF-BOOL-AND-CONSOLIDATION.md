---
id: CAT-MIGRATE-LF-BOOL-AND-CONSOLIDATION
title: "Core.Classes bool_and consolidation: drop LF's private bool_and dup (assoc/left_unit/right_unit) and repoint Semigroup_instance_Bool's field assignments to LC's canonical proofs, after Map's Tier C increment establishes LC's ownership of assoc + the identity laws. Reuses LC's canonical; mints nothing; no second assoc, no second unit-law identity."
status: merged
owner: foundation
size: M
gate: none
tier: T2
depends_on: [CAT-MIGRATE-TIER-C-DATA-VALUE]
blocks: []
github: null
origin: "Steward, 2026-09-03, minted per the Architect's bool_and companion-scope ruling (evt_4hp6qxkdaqgbz, ruling (b)+(c)) on the Tier C D0 census. The three bool_and dups carry different, partly-overlapping proof families: LC (public, :641) owns intro/left/right; Map (:5656) owns comm/assoc/idempotent/left_identity/right_identity/true_intro; LF (:123) owns assoc/left_unit/right_unit. Overlap is wider than assoc: Map.assoc == LF.assoc AND Map.left_identity/right_identity == LF.left_unit/right_unit (same propositions, divergent names). LC is the SOLE canonical owner; every proposition exists there EXACTLY ONCE under ONE name. Map's Tier C increment (a) relocates Map's six proofs to LC first; THIS node then drops LF's dup and repoints. Sequenced AFTER Map's (a) (ruling (c)); Core.Classes, OUT of Tier C Data scope, so a separate node. Held (draft) until Map's (a) lands."
---

> # Core.Classes: consolidate LF's private bool_and dup onto LC's canonical.
> # DROP LF's bool_and + assoc + left_unit + right_unit; repoint
> # Semigroup_instance_Bool's field assignments to LC's canonical proofs.
> # Reuse only — mint nothing. No second assoc, no second unit-law identity.

This node runs AFTER [[CAT-MIGRATE-TIER-C-DATA-VALUE]]'s Map increment (a) has
relocated Map's `bool_and` family to LC, so LC already owns the canonical
`assoc` + identity laws. It reconciles the last divergent-named dup — LF's
private copy — by dropping it and repointing, never by moving a proof (Map's (a)
already did the only move). Ruling (c): if for any reason this were to run before
Map's (a), invert which set is canonical — but never let both land the same
proposition.

## Deliverable

- **D1 — drop + repoint.** DROP LF's private `bool_and` and its `assoc`,
  `left_unit`, `right_unit` (LF:123-region), reusing LC's canonical (landed by
  Map's (a)). Reconcile the naming by repointing `Semigroup_instance_Bool`'s
  field assignments (LF:171-179) to LC's canonical proofs — the class field NAMES
  (`assoc`/`left_unit`/`right_unit`) stay; the PROOFS they bind become LC's
  `assoc`/`left_identity`/`right_identity`. Mint nothing; no second `assoc`, no
  second unit-law identity. The exact canonical spelling for the unit laws
  (identity vs unit) is a bounded sub-decision here, grounded in nearest landed
  precedent — either way one proof, one name.

## Acceptance criteria

- **AC-NO-DUP (the point of the node).** After the drop, `bool_and`'s `assoc` and
  the two unit/identity laws exist EXACTLY ONCE in the catalog (in LC); a census
  shows LF holds no `bool_and` proof. Control: grep/loader census for each
  proposition returns a single owner.
- **AC-REPOINT-SOUND.** `Semigroup_instance_Bool` still elaborates: its field
  assignments resolve to LC's canonical proofs at the same field names; the
  instance's type is unchanged. Control: removing the repoint reintroduces the
  `UnboundName`/dangling reference the drop created.
- **AC-NO-MINT.** Publishing/repointing mints no new class, instance, or
  proposition; a differential shows only deletions in LF + the field-binding
  repoint. No computational change to `Semigroup_instance_Bool`'s behavior.
- **AC-NO-REGRESSION.** Complete affected-target closure (LF, LC, every consumer
  of `Semigroup_instance_Bool` or the moved proofs), scoped by changed paths,
  targeted via `scripts/ken-cargo`, never `--workspace` (green in CI).

## Gate, reviewer, sequencing

`gate: none`. Candidate: **Architect** (required — the drop is exact, the repoint
binds LC's canonical, nothing minted, one-proposition-one-name preserved) +
**Foundation QA + CV** on the exact SHA, then Steward M1-M4 -> lieutenant.
Sequenced strictly AFTER Map's Tier C increment (a). Held (draft) until then.
