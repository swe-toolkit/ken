---
id: CAT-MIGRATE-TIER-D-CURSOR
title: "Scaffold-retirement Tier D (Capability), first internal slice: migrate Diagnostics.Core (DC), Formatting.Doc (Doc), and Parsing.Cursor (Cursor) off fixture-scaffolding / ambient resolution onto real selective imports from the already-published Tier A/B/C providers, so each elaborates standalone. Per-module: publish the module's own export surface, replace ambient resolution with a real selective import from the published lower tiers, extend the loader-visible inventory, standalone-green. The proven Tier-A / B / C publish-and-import shape; NO class-instance relocation, NO proof authoring beyond attached-owner migrations the Architect names. D0 measures the intra-slice DAG (whether Cursor consumes DC/Doc or they are independent consumers of lower tiers) and confirms the exact import set per module."
status: merged
owner: foundation
size: M
gate: none
tier: T2
depends_on: [CAT-MIGRATE-TIER-C-DATA-VALUE]
blocks: []
github: null
origin: "Steward, 2026-09-08, framed one release ahead per CAT-SCAFFOLD-RETIREMENT sequencing (each tier's internal slice framed as its predecessor lands). Tier C is complete — CAT-MIGRATE-TIER-C-DATA-VALUE and the {NonEmpty, Validation} split-outs all merged — so the 'gated on Tier C' hold is discharged. This is the FIRST internal slice of Tier D (Capability), per the Architect decomposition evt_2e0pee5jxzv07 (internal order 'DC + Doc -> Cursor -> Decoder -> Numeric / Parsing / Process.Arguments; Diagnostics.Render; Filesystem.Path.Posix; System.IO erratum') and foundation-leader's frontier report evt_25e4ces4x6mgd. Module identities GROUNDED at origin/main 8c883e578: DC = catalog/packages/Capability/Diagnostics/Core.ken.md (blob ce8a88f0abd1), Doc = catalog/packages/Capability/Formatting/Doc.ken.md (blob cf55c8c9661e), Cursor = catalog/packages/Capability/Parsing/Cursor.ken.md (blob 593aa69a11f6)."
---

> # Scaffold-retirement Tier D, slice 1: the DC + Doc -> Cursor Capability modules.
> # Per-module publication + import clean-ification (the Tier-A / B / C shape):
> # publish own surface, repoint own consumption to already-published lower tiers,
> # extend loader inventory, standalone-green. NO class-instance relocation.
>
> Each of the three modules resolves one or more provider symbols ambiently (via
> the whole-catalog class-install / scaffolding fallback) and fails standalone
> elaboration. This slice gives each a real selective import from the
> already-published lower tiers (Tier A providers: Transport, Derived, Compare,
> Arithmetic, Nat.Order; Tier B: LawfulClasses; Tier C: the Data value modules)
> plus any intra-slice edge D0 confirms, so each elaborates on its own. Every
> provider this slice imports is ALREADY PUBLISHED — no consumer-only increment
> references an unpublished provider (the DAG axis).

## Not a regression fix (read before treating standalone-red as a bug)

These modules elaborate TODAY in the full-catalog build via ambient class-install
(the operator's class-uniformity ruling, 2026-09-02). Nothing is on fire. This
slice is a standalone-CLEANNESS quality slice: it brings each Capability module
to the scaffold-retirement end state (`zero catalog dependence on fixture
scaffolding / ambient resolution`, [[CAT-SCAFFOLD-RETIREMENT]]). "Module X fails
standalone at UnresolvedCon/UnboundName Y" is the STARTING condition each
increment closes, not a defect on `main`.

## Module set — this slice, per-module increments

Grounded at origin/main 8c883e578 (current import blocks, for orientation only —
D0 re-measures at the pickup SHA):

1. **DC = Diagnostics.Core** (`Capability/Diagnostics/Core.ken.md`) — today imports
   only `Core.Classes.LawfulClasses (leq_nat)`.
2. **Doc = Formatting.Doc** (`Capability/Formatting/Doc.ken.md`) — today imports
   `LawfulClasses (leq_nat)`, `Core.Logic.Or (Or, Inl, Inr)`,
   `Data.Numeric.Nat.Arithmetic (add)`.
3. **Cursor = Parsing.Cursor** (`Capability/Parsing/Cursor.ken.md`) — today imports
   `Data.Collections.Derived (length)`, `Data.Numeric.Nat.Arithmetic (add)`,
   `Data.Numeric.Nat.Order (sub)`.

## The a-priori order is an ORDER, not a measured edge — D0 decides the edge

The Architect's internal-order string is "DC + Doc -> Cursor". At origin/main
NONE of the three imports the other two by module name, so the `->` is the
umbrella's a-priori bottom-up ORDERING (providers DC/Doc before consumer Cursor),
NOT an established free-symbol edge. Whether Cursor actually consumes a DC/Doc
symbol (via ambient today) or the three are independent consumers of the lower
tiers is exactly what D0 MEASURES. Do NOT hand-fix an import set from this frame:
the EC "exact-four" census under-counted (an a-priori surface was measurably
insufficient), so measure the free-symbol closure, do not guess it. If D0 finds
the three are independent, the slice is three independent increments; if it finds
a real DC->Cursor or Doc->Cursor edge, the increment order honors it (provider
before consumer), same as Tier C's `SB -> StringKeys`.

## NOT this slice — later Tier-D steps (do not pull them in)

- **Parsing.Decoder, Parsing.Numeric, Parsing.Parsing, Process.Arguments** — the
  next Tier-D slices (Cursor is their predecessor; frame after this lands).
- **Diagnostics.Render, Filesystem.Path.Posix** — independent Tier-D singletons
  (Filesystem.Path.Posix gated on BK's DecEq relocation, already landed in Tier B).
- **System.IO theorem-rename erratum** — a one-module Tier-D step (rename the
  `write_all_all_success` theorem so it does not shadow its subject fn, keep the
  subject fn). It is NOT part of this slice. If a Cursor/DC/Doc increment appears
  to need a System.IO change, that is a signal to STOP and route to the Steward,
  not to fold the erratum in here.

## Deliverables

- **D0 — census + intra-slice order at the pickup SHA (T1-adjacent judgment;
  Architect is the confirmer).** For each of the three modules, measure its
  standalone `UnresolvedCon`/`UnboundName` set (the loader is the authority) and
  the exact provider it needs, and MEASURE the intra-slice value-dependency edges
  among {DC, Doc, Cursor} (does Cursor — or either other — consume a symbol owned
  by another of the three?). Emit the increment order from that measured DAG. If
  D0 surfaces a provider NOT owned by an already-published Tier A/B/C module (a
  hidden dep on a still-scaffolded module, e.g. another Capability module), CITE
  it and split it out rather than dragging an unpublished provider in (the same
  DAG axis every tier respects). D0 also confirms whether the slice stays one node
  with per-module increments or splits.
- **D1..D3 — per-module publish + import + standalone, in the D0 order.** For each
  module: publish exactly the export surface its downstream consumers need
  (measured, not guessed); add a selective import from the published lower tiers
  (and any D0-confirmed intra-slice provider) for the exact D0-confirmed set;
  retire the ambient reach; extend the module's loader-visible inventory to
  reflect both the new exports and the imports (imported names are not new
  exports — reflect them the way an existing import is reflected); the module
  elaborates standalone (exit 0).
- **Any attached-owner migration D0 surfaces** is handled the way Tier C handled
  Map's `bool_and` family: a reviewed verbatim-modulo-`pub` relocation to the
  canonical owner, NOT a blind delete, and only if the Architect names it. This
  slice authors no new proof content beyond such a named move; if none is
  surfaced, none is authored.

## Acceptance criteria, each with its control (proven Tier-A / B / C shape)

- **AC-EXPORTED (positive, per published symbol).** Each newly-published symbol is
  LOADER-VISIBLE from its owning module — a selective import resolves it to that
  module's `GlobalId`, measured by the loader, not a `^pub` grep. Control: the
  probe resolves; a still-private sibling name in the same module still rejects
  `UnboundName`.
- **AC-EXACT-INVENTORY (per module).** Each module's loader-visible inventory
  equality extends by EXACTLY the intended names (exports + imports; nothing else
  changes visibility); population from the module's own definitions, verdict from
  the loader, a per-symbol reddening mutation each reds distinctly.
- **AC-STANDALONE-GREEN (per module).** Each migrated module elaborates standalone
  (exit 0) after its import block — no ambient/scaffolding fallback. The control
  per module is that removing the new import line restores the exact prior
  standalone failure (the `UnresolvedCon`/`UnboundName` D0 recorded). If D0 finds a
  module is ALREADY standalone-green with no required new import (the Tier-C
  StringKeys/BytesKeys no-op shape), the reddening control is inapplicable there —
  say so, and deliver the honest inventory/consumed-set controls instead of adding
  an unused import.
- **AC-VISIBILITY-ONLY (class-uniformity).** Any `pub` added to a class or a
  class-adjacent symbol changes visibility only: a differential shows the body
  BYTE-UNCHANGED; publishing mints no second class/instance; every consumer
  resolves to the single existing owner. No computational change. (A named
  attached-owner relocation, if D0 surfaces one, is the one exception that moves a
  body — reviewed as a verbatim move modulo `pub`, not a visibility flip.)
- **AC-NO-REGRESSION.** Re-run the COMPLETE affected-target closure (every target
  loading any changed module or a module whose closure this changes), scoped by
  changed PATHS. Targeted via `scripts/ken-cargo`, never `--workspace` (green in
  CI is the workspace verdict).

## Format any edited catalog file before handoff

Every file under `catalog/` must already be in canonical `kenfmt` form — there is
a corpus-wide formatter fixed-point gate (`kenfmt_c_capstone` /
`ken-cli/tests/ken_fmt.rs`) keyed on every catalog file. It fires only in a
full-workspace run, which is CI-only by operator hard rule, so a targeted
`-p ken-elaborator --test <yours>` cannot see it. Run the formatter on any
`catalog/` file this slice edits before handing the SHA to QA; do NOT run
`--workspace` locally to check it.

## Capability tier: T2 (with a T1-adjacent D0)

Per-module mechanical export publication + bounded import-block clean-ification
(the proven Tier-A / B / C shape) — no class-instance relocation, no proof
authoring beyond a named attached-owner move D0 may surface. The one judgment is
D0 completeness per module (are all ambient deps owned by an already-published
Tier A/B/C module, or is there a hidden non-tier provider — another Capability
module?) plus the intra-slice ordering measurement, which either folds in or
splits out with a cited reason. Foundation-leader flagged (evt_25e4ces4x6mgd) that
the migration "appears normally T2" but the ordered Capability closure needs the
frame + D0 census before the estimate locks; the foundation implementer is
currently T1, so the T1-adjacent D0 is well-matched and the mechanical D1..D3 tail
is the over-provisioned part — foundation-leader verifies the seat against this
estimate before pickup (their request).

## Gate, reviewer, sequencing

`gate: none` (no TCB touch; the operator ruled the class-owner model). On each
increment's candidate: **Architect** (required — surface correctness +
class-uniformity + D0 order/completeness) + **Foundation QA + CV** on the exact
SHA, then Steward M1-M4 -> lieutenant. First internal slice of Tier D; successor
to Tier C (all merged). The next Tier-D slice (Decoder, then Numeric / Parsing /
Process.Arguments) is framed one release ahead after this lands, with fixed inputs
re-measured at that SHA. The Architect is the required reviewer on each slice.

## Contention and sequencing

`catalog/` only, and additive. Re-measure contention at pickup, not from this
frame. The concurrent doc track touches `library/` and `agent/`, disjoint from
`catalog/`. [[CAT-MAP-DEPENDENCY-CLOSURE-REPAIR]] is a Foundation-owned unsized
draft whose prioritization is unsettled — not in flight and not a substitute.
`CAT-C2` is Spec/Ergo-owned, draft, scheduling-blocked on `SPEC-IDENT-BLESSED` —
not a scaffold tier and disjoint. Foundation holds no other released node; this is
the ring's work.

## Hard stop

Route to the Steward if:

- a Capability module in this slice CANNOT be brought standalone-green through a
  selective import from already-published Tier A/B/C providers — i.e. D0 surfaces
  a required provider owned by a still-scaffolded, unpublished module. That is a
  hidden cross-tier edge; split it out, do not drag the unpublished provider in;
  or
- delivering D1..D3 appears to require binding a new class/instance, authoring
  proof content beyond a named attached-owner move, or the System.IO erratum. Any
  of those means the slice cut is wrong, not that the scope should bend.
