---
id: CAT-MIGRATE-TIER-D-DECODER
title: "Scaffold-retirement Tier D (Capability), slice 2: migrate Parsing.Decoder off fixture-scaffolding / ambient resolution onto real selective imports from the already-published providers (Parsing.Cursor, now published by slice 1, plus the Tier A/B/C lower tiers), so it elaborates standalone. Publish Decoder's own export surface, replace ambient resolution with a real selective import for the exact D0-measured set, extend the loader-visible inventory, standalone-green. The proven Tier-A / B / C / Cursor-slice publish-and-import shape; NO class-instance relocation, NO proof authoring beyond an attached-owner migration the Architect names. Delegated D0 measures Decoder's standalone missing-symbol set and full provider closure from loader evidence."
status: merged
owner: foundation
size: S
gate: none
tier: T2
depends_on: [CAT-MIGRATE-TIER-D-CURSOR]
blocks: []
github: null
origin: "Steward, 2026-09-08, framed one release ahead per CAT-SCAFFOLD-RETIREMENT sequencing as slice 1 (CAT-MIGRATE-TIER-D-CURSOR) landed (admin-merge ee8a35351, node merged 7cc163bfb, Adversary M8b NO OBJECTION). Second internal slice of Tier D (Capability), per the Architect decomposition evt_2e0pee5jxzv07 internal order 'DC + Doc -> Cursor -> Decoder -> Numeric / Parsing / Process.Arguments'. Decoder is the predecessor of the Numeric/Parsing/Process.Arguments group. Module identity + edge GROUNDED at landed origin/main 7cc163bfb: Decoder = catalog/packages/Capability/Parsing/Decoder.ken.md (blob 3602824a2295); it references Parsing.Cursor throughout (a real Cursor -> Decoder value edge), and Cursor's export surface is now published on main (Cursor blob 07983b071e00) by slice 1 — so the provider Decoder needs is already published (the DAG axis)."
---

> # Scaffold-retirement Tier D, slice 2: Parsing.Decoder.
> # Single-module publication + import clean-ification (the Tier-A / B / C /
> # Cursor-slice shape): publish Decoder's own surface, repoint its consumption to
> # the already-published Parsing.Cursor + lower tiers, extend loader inventory,
> # standalone-green. NO class-instance relocation.
>
> Decoder resolves one or more provider symbols ambiently (via the whole-catalog
> class-install / scaffolding fallback) and fails standalone elaboration. This
> slice gives it a real selective import from the already-published providers —
> Parsing.Cursor (published by slice 1) plus the Tier A/B/C lower tiers — so it
> elaborates on its own. Every provider this slice imports is ALREADY PUBLISHED
> (no consumer-only increment references an unpublished provider — the DAG axis).

## Not a regression fix (read before treating standalone-red as a bug)

Decoder elaborates TODAY in the full-catalog build via ambient class-install (the
operator's class-uniformity ruling, 2026-09-02). Nothing is on fire. This slice is
a standalone-CLEANNESS quality slice: it brings Decoder to the scaffold-retirement
end state (`zero catalog dependence on fixture scaffolding / ambient resolution`,
[[CAT-SCAFFOLD-RETIREMENT]]). "Decoder fails standalone at UnresolvedCon/UnboundName
Y" is the STARTING condition this closes, not a defect on `main`.

## Module + the grounded edge

Single module: **Decoder = Parsing.Decoder** (`Capability/Parsing/Decoder.ken.md`,
blob 3602824a2295 at 7cc163bfb). Unlike the Cursor slice's a-priori arrow, the
Cursor -> Decoder edge is GROUNDED, not merely ordered: Decoder references
Parsing.Cursor throughout its body. So Decoder's provider closure is expected to
include Parsing.Cursor's published surface plus lower tiers. It may ALSO consume
Diagnostics.Core (via Cursor's re-exported origin types, or directly) — D0
measures which. Do NOT hand-fix the import set from this frame: the EC "exact-four"
census under-counted (an a-priori surface was measurably insufficient), so measure
the free-symbol closure from loader evidence, do not guess it.

## NOT this slice — later Tier-D steps (do not pull them in)

- **Parsing.Numeric, Parsing.Parsing, Process.Arguments** — the next Tier-D slice
  (Decoder is their predecessor; frame after this lands).
- **Diagnostics.Render, Filesystem.Path.Posix** — independent Tier-D singletons
  (Filesystem.Path.Posix's BK DecEq relocation already landed in Tier B).
- **System.IO theorem-rename erratum** — a separate one-module Tier-D step. NOT
  part of this slice. If a Decoder increment appears to need a System.IO change,
  STOP and route to the Steward rather than folding the erratum in.

## Deliverables

- **D0 — census + provider closure at the pickup SHA (T1-adjacent judgment;
  Architect is the confirmer).** Measure Decoder's standalone
  `UnresolvedCon`/`UnboundName` set (the loader is the authority) and the exact
  provider closure it needs — from Parsing.Cursor's published surface and the
  Tier A/B/C lower tiers. Confirm the exact set of Cursor symbols Decoder consumes
  (the grounded Cursor -> Decoder edge), and whether it also directly consumes
  Diagnostics.Core or any other Capability module. If D0 surfaces a provider NOT
  owned by an already-published module (a hidden dep on a still-scaffolded module,
  e.g. Numeric/Parsing which are LATER slices), CITE it and split it out / STOP —
  do NOT drag an unpublished provider in (the DAG axis every tier respects). Being
  a single module, there is no intra-slice DAG to measure; the census is the
  provider closure + completeness.
- **D1 — publish + import + standalone.** Publish exactly the export surface
  Decoder's downstream consumers need (measured, not guessed — Decoder feeds the
  Numeric/Parsing/Process.Arguments group and Tier E's Json/Config.Decoder); add a
  selective import from the published providers for the exact D0-confirmed set;
  retire the ambient reach; extend Decoder's loader-visible inventory to reflect
  both the new exports and the imports (imported names are not new exports —
  reflect them the way an existing import is reflected); Decoder elaborates
  standalone (exit 0).
- **Any attached-owner migration D0 surfaces** is handled the way Tier C handled
  Map's `bool_and` family: a reviewed verbatim-modulo-`pub` relocation to the
  canonical owner, NOT a blind delete, and only if the Architect names it. This
  slice authors no new proof content beyond such a named move; if none is
  surfaced, none is authored.

## Acceptance criteria, each with its control (proven Tier-A / B / C / Cursor shape)

- **AC-EXPORTED (positive, per published symbol).** Each newly-published symbol is
  LOADER-VISIBLE from Decoder — a selective import resolves it to Decoder's
  `GlobalId`, measured by the loader, not a `^pub` grep. Control: the probe
  resolves; a still-private sibling name in Decoder still rejects `UnboundName`.
- **AC-EXACT-INVENTORY.** Decoder's loader-visible inventory equality extends by
  EXACTLY the intended names (exports + imports; nothing else changes visibility);
  population from Decoder's own definitions, verdict from the loader, a per-symbol
  reddening mutation each reds distinctly.
- **AC-STANDALONE-GREEN.** Decoder elaborates standalone (exit 0) after its import
  block — no ambient/scaffolding fallback. Control: removing the new import line(s)
  restores the exact prior standalone failure (the `UnresolvedCon`/`UnboundName` D0
  recorded); dropping the Cursor import specifically reds at the exact
  Cursor-owned symbol Decoder consumes.
- **AC-VISIBILITY-ONLY (class-uniformity).** Any `pub` added to a class or
  class-adjacent symbol changes visibility only: a differential shows the body
  BYTE-UNCHANGED; publishing mints no second class/instance; every consumer
  resolves to the single existing owner. No computational change. (A named
  attached-owner relocation, if D0 surfaces one, is the one exception that moves a
  body — reviewed as a verbatim move modulo `pub`.)
- **AC-NO-REGRESSION.** Re-run the COMPLETE affected-target closure (every target
  loading Decoder or a module whose closure this changes), scoped by changed PATHS.
  Targeted via `scripts/ken-cargo`, never `--workspace` (green in CI is the
  workspace verdict).

## Format any edited catalog file before handoff

Every file under `catalog/` must already be in canonical `kenfmt` form — there is
a corpus-wide formatter fixed-point gate (`kenfmt_c_capstone` /
`ken-cli/tests/ken_fmt.rs`) keyed on every catalog file, firing only in a
full-workspace (CI-only) run. Run the formatter on the Decoder file before handing
the SHA to QA; do NOT run `--workspace` locally to check it.

## Capability tier: T2 (with a T1-adjacent D0)

Single-module mechanical export publication + bounded import-block clean-ification
(the proven shape) — no class-instance relocation, no proof authoring beyond a
named attached-owner move D0 may surface. The one judgment is D0 completeness (are
all ambient deps owned by an already-published module, or is there a hidden
non-published provider — a Numeric/Parsing module that belongs to a LATER slice?).
The mechanical D1 tail is T2; the foundation implementer is currently T1, so the
T1-adjacent D0 is well-matched — foundation-leader verifies the seat against this
estimate before pickup.

## Gate, reviewer, sequencing

`gate: none` (no TCB touch; the operator ruled the class-owner model). On the
candidate: **Architect** (required — surface correctness + class-uniformity + D0
provider-closure/completeness) + **Foundation QA + CV** on the exact SHA, then
Steward M1-M4 -> lieutenant. Second internal slice of Tier D; successor to slice 1
(CAT-MIGRATE-TIER-D-CURSOR, merged). The next Tier-D slice (Numeric / Parsing /
Process.Arguments) is framed one release ahead after this lands, with fixed inputs
re-measured at that SHA. The Architect is the required reviewer on each slice.

## Contention and sequencing

`catalog/` only, and additive. Re-measure contention at pickup, not from this
frame. The concurrent doc track touches `library/` and `agent/`, disjoint from
`catalog/`. [[CAT-MAP-DEPENDENCY-CLOSURE-REPAIR]] is a Foundation-owned unsized
draft, not in flight. `CAT-C2` is Spec/Ergo-owned, draft, scheduling-blocked on
`SPEC-IDENT-BLESSED` — disjoint. Foundation holds no other released node; this is
the ring's work.

## Hard stop

Route to the Steward if:

- Decoder CANNOT be brought standalone-green through a selective import from
  already-published providers — i.e. D0 surfaces a required provider owned by a
  still-scaffolded, unpublished module (a Numeric/Parsing module belongs to a
  LATER slice, so its being a Decoder dependency would reorder the tier). That is a
  hidden cross-slice edge; STOP and route it, do not drag the unpublished provider
  in; or
- delivering D1 appears to require binding a new class/instance, authoring proof
  content beyond a named attached-owner move, or the System.IO erratum. Any of
  those means the slice cut is wrong, not that the scope should bend.
