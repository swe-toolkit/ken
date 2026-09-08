---
id: CAT-MIGRATE-TIER-D-PARSING-GROUP
title: "Scaffold-retirement Tier D (Capability), slice 3 (final internal slice): migrate the three Decoder-successor modules — Parsing.Numeric, Parsing.Parsing, Process.Arguments — off fixture-scaffolding / ambient resolution onto real selective imports from the already-published providers (Parsing.Decoder + Parsing.Cursor, published by slices 1-2, plus the Tier A/B/C lower tiers), so each elaborates standalone. Publish each module's own export surface for its downstream consumers (ArgParse, Application.Configuration, Tier E Json/Config.Decoder), replace ambient resolution with real selective imports for the exact D0-measured sets, extend the loader-visible inventories, standalone-green. MEASURED at 613a7f5c9: no intra-group DAG (three independent single-module migrations) and every provider already published — so the three land in any order, or as per-module partials. The proven Tier-A/B/C/Cursor/Decoder publish-and-import shape; NO class-instance relocation, NO proof authoring beyond an attached-owner migration the Architect names. Delegated D0 measures each module's standalone missing-symbol set and full provider closure from loader evidence, and MUST hard-stop if it finds an intra-group edge or a provider owned by an unpublished module."
status: active
owner: foundation
size: M
gate: none
tier: T2
depends_on: [CAT-MIGRATE-TIER-D-DECODER]
blocks: []
github: null
origin: "Steward, 2026-09-08, framed one release ahead per CAT-SCAFFOLD-RETIREMENT sequencing as slice 2 (CAT-MIGRATE-TIER-D-DECODER) landed (squash b024b54eb, node merged 613a7f5c9, Adversary M8b NO OBJECTION evt_4s3nhvmkcx901). Third and FINAL internal slice of Tier D (Capability), per the Architect decomposition evt_2e0pee5jxzv07 internal order 'DC + Doc -> Cursor -> Decoder -> Numeric / Parsing / Process.Arguments'. Module identities + edges GROUNDED at landed origin/main 613a7f5c9: Parsing.Numeric = catalog/packages/Capability/Parsing/Numeric.ken.md (blob 7623901b0858), Parsing.Parsing = catalog/packages/Capability/Parsing/Parsing.ken.md (blob 0370e93bf979), Process.Arguments = catalog/packages/Capability/Process/Arguments.ken.md (blob 6818f34422 64). Coarse edge census at that SHA (D0 confirms exactly): Parsing.Parsing is a heavy Parsing.Decoder + Parsing.Cursor + Diagnostics.Core client reaching all three AMBIENTLY (its only real imports are Core.Classes.LawfulClasses / Data.Collections.Derived / Data.Numeric.Nat.Order); Parsing.Numeric references only its own symbols plus lower-tier Diagnostics/Data (no Cursor/Decoder/sibling edge); Process.Arguments imports only Core.Classes.LawfulClasses and references no group symbol. No measured Parsing->Numeric, Numeric->{Cursor,Decoder}, or Arguments->{group} edge, so there is no intra-group DAG and every provider is already published (Decoder slice 2, Cursor slice 1, Tier A/B/C)."
---

> # Scaffold-retirement Tier D, slice 3 (final): Parsing.Numeric + Parsing.Parsing
> # + Process.Arguments.
> # Three independent single-module publications + import clean-ifications (the
> # Tier-A / B / C / Cursor / Decoder shape): publish each module's own surface,
> # repoint its consumption to the already-published Parsing.Decoder +
> # Parsing.Cursor + lower tiers, extend loader inventory, standalone-green. NO
> # class-instance relocation.
>
> These three modules resolve one or more provider symbols ambiently (via the
> whole-catalog class-install / scaffolding fallback) and fail standalone
> elaboration. This slice gives each a real selective import from the
> already-published providers so it elaborates on its own. Every provider this
> slice imports is ALREADY PUBLISHED (the DAG axis) — MEASURED at 613a7f5c9: no
> group member consumes another, so there is no intra-slice ordering, and no
> member reaches a later-tier (unpublished) module.

## Not a regression fix (read before treating standalone-red as a bug)

All three modules elaborate TODAY in the full-catalog build via ambient
class-install (the operator's class-uniformity ruling, 2026-09-02). Nothing is on
fire. This slice is a standalone-CLEANNESS quality slice: it brings each module to
the scaffold-retirement end state (`zero catalog dependence on fixture
scaffolding / ambient resolution`, [[CAT-SCAFFOLD-RETIREMENT]]). "Module X fails
standalone at UnresolvedCon/UnboundName Y" is the STARTING condition this closes,
not a defect on `main`.

## Modules + the grounded edges (measured at 613a7f5c9; D0 confirms exactly)

Three single modules, each its own migration:

- **Parsing.Parsing** = `Capability/Parsing/Parsing.ken.md` (blob `0370e93bf979`,
  810 lines) — the heavy one. GROUNDED, not merely ordered: its body is a genuine
  `Capability.Parsing.Decoder` client (DecoderFailed / decoder_seq / Decoder /
  DecoderResult / decoder_many / decoder_alt / decoder_satisfy / decoder_recursive
  / decoder_pure / decoder_fail / decoder_error_location / DecoderRejected /
  DecoderError / DecoderZeroProgress) AND a `Capability.Parsing.Cursor` client
  (cursor_position / cursor_source / cursor_ops / cursor_remaining / cursor_locate
  / cursor_peek / cursor_advance / CursorOps), and it names `SourceId` from
  `Capability.Diagnostics.Core`. Today it reaches all three AMBIENTLY — its only
  real imports are `Core.Classes.LawfulClasses (leq_nat)`,
  `Data.Collections.Derived (list_append)`, `Data.Numeric.Nat.Order (sub)`. The
  migration adds real selective imports for the exact Decoder + Cursor +
  Diagnostics.Core symbols D0 measures, all published.
- **Parsing.Numeric** = `Capability/Parsing/Numeric.ken.md` (blob `7623901b0858`,
  285 lines) — located decimal parsing (chars -> arbitrary-precision `Int`). The
  coarse scan shows it references only its OWN symbols (parse_nat / parse_int /
  parse_digits_at / numeric_diagnostic / numeric_origin / numeric_argument_origin
  and their kin) plus lower-tier Diagnostics/Data; NO `cursor_`/`decoder_`/sibling
  edge. So its provider closure is expected to be Diagnostics.Core (its origin /
  diagnostic types) + Data numeric/char — all published, lower tier. D0 confirms
  the exact set; if it surfaces a Cursor/Decoder edge the coarse scan missed, that
  is fine (both are published) — but a group-sibling or later-tier edge is a
  HARD STOP.
- **Process.Arguments** = `Capability/Process/Arguments.ken.md` (blob
  `6818f3442264`, 88 lines) — argv projection + replacement. Its only import is
  `Core.Classes.LawfulClasses (leq_nat)` and it references no group symbol; its
  ambient reach is expected to be lower-tier only. D0 confirms.

**No intra-group DAG (the load-bearing measurement).** No measured
Parsing->Numeric, Numeric->{Cursor,Decoder}, or Arguments->{group} edge. So the
three are mutually independent single-module migrations — implementable in any
order and landable as per-module partials — and every provider each needs is
already published. This is a parallel slice, NOT a chain. Do NOT hand-fix the
import sets from this frame: the EC "exact-four" census under-counted once (an
a-priori surface was measurably insufficient), so measure each free-symbol closure
from loader evidence, do not guess it.

## NOT this slice — the remaining Tier-D steps (do not pull them in)

- **Diagnostics.Render, Filesystem.Path.Posix** — independent Tier-D singletons
  (Filesystem.Path.Posix's BK DecEq relocation already landed in Tier B).
- **System.IO theorem-rename erratum** — a separate one-module Tier-D step. NOT
  part of this slice. If a member increment appears to need a System.IO change,
  STOP and route to the Steward rather than folding the erratum in.
- **Tier E (Json / Config.Decoder, Input.Schema)** — downstream of this slice;
  these three modules FEED Tier E, they do not consume it.
- **The downstream consumers** `Application.CommandLine.ArgParse` and
  `Application.Configuration.Decoder` are CONSUMERS of this slice's exports, not
  members of it. Publish each module's surface to serve them; do not migrate them
  here (their acceptance fixtures are in the affected-target closure, per below).

## Deliverables

- **D0 — per-module census + provider closure at the pickup SHA (T1-adjacent
  judgment; Architect is the confirmer).** For EACH of the three modules, measure
  its standalone `UnresolvedCon`/`UnboundName` set (the loader is the authority)
  and the exact provider closure it needs, drawn only from the already-published
  Parsing.Decoder / Parsing.Cursor surfaces and the Tier A/B/C lower tiers.
  Confirm the exact Decoder + Cursor + Diagnostics.Core symbol sets Parsing.Parsing
  consumes (the grounded edges), and the lower-tier closures for Numeric and
  Arguments. **The DAG hard-stop:** if D0 surfaces, for any member, a provider
  owned by another group member (an intra-group edge) or by a still-scaffolded /
  unpublished module, CITE it and STOP / split it out — do NOT drag a sibling or an
  unpublished provider in. Being three single modules with no measured intra-group
  edge, there is no intra-slice DAG to order; the census is each module's provider
  closure + completeness. Confirm or refute the coarse edge census above (Numeric
  and Arguments lower-tier-only; Parsing = Decoder+Cursor+Diagnostics.Core).
- **D1 — publish + import + standalone, per module.** For each module: publish
  exactly the export surface its downstream consumers need (measured, not guessed
  — Parsing feeds ArgParse and Tier E; Numeric feeds ArgParse's numeric arguments;
  Arguments feeds ArgParse's argv); add a selective import from the published
  providers for the exact D0-confirmed set; retire the ambient reach; extend the
  module's loader-visible inventory to reflect both the new exports and the imports
  (imported names are not new exports — reflect them the way an existing import is
  reflected); the module elaborates standalone (exit 0). The three are independent,
  so each may land as its own increment (accepted partial) — a per-module green is a
  releasable increment, and Parsing (the heavy one) need not wait on Numeric /
  Arguments or vice versa.
- **Any attached-owner migration D0 surfaces** is handled the way Tier C handled
  Map's `bool_and` family: a reviewed verbatim-modulo-`pub` relocation to the
  canonical owner, NOT a blind delete, and only if the Architect names it. This
  slice authors no new proof content beyond such a named move; if none is
  surfaced, none is authored.

## Acceptance criteria, each with its control (proven Tier-A / B / C / Cursor / Decoder shape)

Each AC is per-module (holds for each of the three independently).

- **AC-EXPORTED (positive, per published symbol).** Each newly-published symbol is
  LOADER-VISIBLE from its module — a selective import resolves it to the module's
  `GlobalId`, measured by the loader, not a `^pub` grep. Control: the probe
  resolves; a still-private sibling name in the same module still rejects
  `UnboundName`.
- **AC-EXACT-INVENTORY.** Each module's loader-visible inventory equality extends
  by EXACTLY the intended names (exports + imports; nothing else changes
  visibility); population from the module's own definitions, verdict from the
  loader, a per-symbol reddening mutation each reds distinctly.
- **AC-STANDALONE-GREEN.** Each module elaborates standalone (exit 0) after its
  import block — no ambient/scaffolding fallback. Control: removing the new import
  line(s) restores the exact prior standalone failure (the
  `UnresolvedCon`/`UnboundName` D0 recorded); for Parsing.Parsing specifically,
  dropping the Decoder import reds at the exact Decoder-owned symbol it consumes,
  and dropping the Cursor import reds at the exact Cursor-owned symbol it consumes
  (two distinct provider edges, two distinct reds).
- **AC-NO-INTRA-GROUP-EDGE (the DAG guard, measured not assumed).** For each
  member, the D0-confirmed provider closure is DISJOINT from the export surfaces of
  the other two group members — no member imports a sibling. Control: the
  referenced-globals set of each member, intersected with each sibling's owned
  globals, is empty (the same `is_disjoint` shape the Decoder slice used against
  Diagnostics.Core). If any intersection is non-empty, the frame's parallel-slice
  premise is refuted — HARD STOP to the Steward (the slice must be re-cut into an
  ordered chain), do NOT silently add the sibling import.
- **AC-VISIBILITY-ONLY (class-uniformity).** Any `pub` added to a class or
  class-adjacent symbol changes visibility only: a differential shows the body
  BYTE-UNCHANGED; publishing mints no second class/instance; every consumer
  resolves to the single existing owner. No computational change. (A named
  attached-owner relocation, if D0 surfaces one, is the one exception that moves a
  body — reviewed as a verbatim move modulo `pub`.)
- **AC-NO-REGRESSION.** Re-run the COMPLETE affected-target closure (every target
  loading any of the three modules or a module whose closure this changes —
  including the ArgParse / env-config / numeric / parsing acceptance fixtures that
  root-load them), scoped by changed PATHS. Targeted via `scripts/ken-cargo`, never
  `--workspace` (green in CI is the workspace verdict). A consuming test fixture's
  root set is part of an increment's path set here (the established Tier-C/D
  precedent) — carry that authorization so it does not hard-stop for it.

## Format any edited catalog file before handoff

Every file under `catalog/` must already be in canonical `kenfmt` form — there is
a corpus-wide formatter fixed-point gate (`kenfmt_c_capstone` /
`ken-cli/tests/ken_fmt.rs`) keyed on every catalog file, firing only in a
full-workspace (CI-only) run. Run the formatter on each edited module before
handing the SHA to QA; do NOT run `--workspace` locally to check it.

## Capability tier: T2 (with a T1-adjacent D0), size M

Three single-module mechanical export publications + bounded import-block
clean-ifications (the proven shape) — no class-instance relocation, no proof
authoring beyond a named attached-owner move D0 may surface. The one judgment is
D0 completeness (are all ambient deps owned by an already-published module, and is
there truly no intra-group edge?). The mechanical D1 tail is T2; the foundation
implementer is currently T1, so the T1-adjacent D0 is well-matched —
foundation-leader verifies the seat against this estimate before pickup. Size M
(not S) because Parsing.Parsing carries a large ambient surface (Decoder + Cursor
+ Diagnostics.Core) atop the two small modules; per-module partial landing keeps
each turn within the one-hour target.

## Gate, reviewer, sequencing

`gate: none` (no TCB touch; the operator ruled the class-owner model). On the
candidate (or each per-module partial candidate): **Architect** (required —
surface correctness + class-uniformity + D0 provider-closure/completeness + the
no-intra-group-edge guard) + **Foundation QA + CV** on the exact SHA, then Steward
M1-M4 -> lieutenant. Third and FINAL internal slice of Tier D; successor to slice 2
(CAT-MIGRATE-TIER-D-DECODER, merged). When this lands, the Tier-D internal chain
(DC+Doc -> Cursor -> Decoder -> this group) is complete; the remaining Tier-D
singletons (Diagnostics.Render, Filesystem.Path.Posix, System.IO erratum) are
independent one-module steps framed separately, and Tier E follows. The Architect
is the required reviewer on each slice.

## Contention and sequencing

`catalog/` only, and additive. Re-measure contention at pickup, not from this
frame. The concurrent doc track touches `library/` and `agent/`, disjoint from
`catalog/`. [[CAT-CC-ORACLE-BEHAVIORALIZE]] is a Foundation-owned non-blocking
draft (test-only cc3/cc4/cc5 cleanup), not in flight and disjoint from the catalog
source. [[CAT-MAP-DEPENDENCY-CLOSURE-REPAIR]] is a Foundation-owned unsized draft,
not in flight. Foundation holds no other released node; this is the ring's work.

## Hard stop

Route to the Steward if:

- D0 surfaces, for any member, a required provider owned by another group member
  (an intra-group edge refuting the parallel-slice premise — the slice must be
  re-cut into an ordered chain) or by a still-scaffolded, unpublished module (a
  hidden cross-slice / later-tier edge). STOP and route it; do not drag the sibling
  or unpublished provider in; or
- a member CANNOT be brought standalone-green through a selective import from
  already-published providers; or
- delivering D1 appears to require binding a new class/instance, authoring proof
  content beyond a named attached-owner move, or the System.IO erratum. Any of
  those means the slice cut is wrong, not that the scope should bend.
