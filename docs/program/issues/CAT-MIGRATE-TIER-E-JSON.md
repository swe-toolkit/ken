---
id: CAT-MIGRATE-TIER-E-JSON
title: "Scaffold-retirement Tier E, first serialization/application spine node: migrate the entry module Data.Serialization.Json off ambient/fixture-scaffold resolution onto its D0-measured selective imports from the already-published lower tiers (Tier A Derived, Tier D Capability.Parsing.Cursor), and publish only its usable consumer surface. D0 measures Json's actual provider closure and its client-consumed public surface, adopts exactly that import set, and publishes exactly that surface. NO new proof authoring, NO carrier change; the proven Tier A/B/C/D publish-and-import shape."
status: active
owner: foundation
size: S
gate: none
tier: T2
depends_on: [CAT-MIGRATE-TIER-D-CURSOR]
blocks: []
github: null
origin: "Steward-framed 2026-09-09 on the foundation-leader's naming (evt_2fk3r5y2g3r61) as foundation's next deliverable after CAT-MIGRATE-TIER-D-SYSTEM-IO landed — the terminal Tier-D leaf. This is the FIRST serialization/application spine node of the [[CAT-SCAFFOLD-RETIREMENT]] DAG, opening Tier E after the now-landed Tier-D Cursor provider (CAT-MIGRATE-TIER-D-CURSOR, merged). Module identity GROUNDED at origin/main 8726f73d6: Json = catalog/packages/Data/Serialization/Json.ken.md (blob cf8333a0cdd9)."
---

> # RELEASED 2026-09-09 (Steward) — Tier E opens: the serialization spine.
>
> Released to the foundation ring as lane 3's next deliverable after the Tier-D
> tail landed. Part of the [[CAT-SCAFFOLD-RETIREMENT]] 5-tier DAG; this is the
> FIRST Tier-E (serialization/application) node, the entry point of the spine
> that runs Serialization.Json -> Application.Input.Schema -> sibling consumers
> (Application.CommandLine.ArgParse, Application.Configuration.Decoder). Its
> provider closure resolves into the already-published Tier-D Cursor module.
> On the candidate: fresh Foundation QA + CV on the exact SHA; Architect
> required only if D0 fires a hard stop (unpublished provider, or an unexpected
> Tier-E edge — see the D0 stop rule), then Steward M1-M4.

## Not a regression fix (read before treating standalone-red as a bug)

`Data.Serialization.Json` elaborates TODAY in the full-catalog build via ambient
resolution — the whole-catalog class-install / scaffolding fallback supplies the
cursor-abstraction symbols it references without an import line (the operator's
class-uniformity ruling, 2026-09-02). Nothing is on fire. This node is a
standalone-CLEANNESS quality slice: it brings the module to the
scaffold-retirement end state (`zero catalog dependence on fixture scaffolding /
ambient resolution`, [[CAT-SCAFFOLD-RETIREMENT]]). "Json fails standalone at
UnresolvedCon/UnboundName Y" is the STARTING condition this node closes, not a
defect on `main`.

## Measured surface (origin/main 8726f73d6 — carry, D0 re-measures at pickup)

Grounded from the module text (blob cf8333a0cdd9), for orientation only — D0
re-measures the free-symbol closure at the pickup SHA via the loader, not from
this frame:

- **Current selective import (already correct):** `Data.Collections.Derived
  (length)` — a Tier-A provider, already published.
- **Ambiently resolved provider closure (no import line today — the reach this
  node retires):** the carrier-neutral cursor abstraction Json references but
  does not import. Every one of these symbols is OWNED and PUBLISHED by the
  just-landed `Capability.Parsing.Cursor` (Tier D, CAT-MIGRATE-TIER-D-CURSOR):
  `CursorOps` and `MkCursorOps` (`data CursorOps ... = MkCursorOps ...`;
  `export CursorOps, MkCursorOps`), `cursor_nat_lt` (`pub fn`),
  `CursorPeekHasRemaining`, `CursorAdvanceProgress`, `CursorEndValid`,
  `CursorLaws` (each `pub fn`).
- **Prelude-ambient symbols** used in the proof bodies (`and_intro`, `And`,
  `Option`, `Some`, `None`, `Nil`, `Cons`, `Pair`, `Proved`, `absurd`, `Equal`,
  `Bool`, `Int`, `String`, `List`, `Nat`, `Suc`, `Zero`, `Type`, `True`): no
  catalog module defines `and_intro`/`And`; they are prelude/built-in, not an
  unpublished-catalog provider. D0 confirms via the loader whether the
  scaffold-retirement surface needs any import for them or they stay prelude.
- **Definitions in the module:** `data Json` + its six constructors
  (`JsonNull`, `JsonBool`, `JsonNumber`, `JsonString`, `JsonArray`,
  `JsonObject`); helper fns `char_cursor_remaining`, `char_cursor_peek`,
  `char_cursor_advance`, `char_cursor_locate`; the dictionary `char_cursor_ops`;
  theorems `char_cursor_lt_suc`, `char_cursor_peek_has_remaining`,
  `char_cursor_advance_progress`, `char_cursor_end_valid`, `char_cursor_laws`.
- **Declared public API (§7):** `Json`, the six constructors, `char_cursor_ops`,
  and the four proofs `char_cursor_peek_has_remaining`,
  `char_cursor_advance_progress`, `char_cursor_end_valid`, `char_cursor_laws`.
  `char_cursor_lt_suc` and the four `char_cursor_*` helper fns are NOT listed —
  internal today.
- **Client-consumed surface:** ZERO catalog module imports `Data.Serialization.Json`
  today (its first spine consumer, `Application.Input.Schema`, is not yet built).
  So the "usable consumer surface" to publish is measured, not inherited from a
  live importer — D0 publishes exactly the surface the module's own role
  requires and nothing more, and does not over-publish an internal helper.

## DAG position

The FIRST serialization/application spine node, opening Tier E after the
now-landed Tier-D Cursor provider (`Capability.Parsing.Cursor`, merged). This
node PRECEDES `Application.Input.Schema`, which in turn precedes its sibling
consumers `Application.CommandLine.ArgParse` and `Application.Configuration.Decoder`.
Json's provider closure crosses into Capability (a Data.Serialization module
consuming the Tier-D `Capability.Parsing.Cursor` abstraction) — that back-edge
is exactly why Cursor is this node's `depends_on`, and it is PREDICTED, not an
unexpected edge.

## NOT this node — independent Tier-E work (do not fold in)

`Algorithm.Sorting.InsertionSort`, `Algorithm.Searching.OrderedSearch`,
`Algorithm.Numeric.Gcd`, and `Tooling.Testing.Property` are INDEPENDENT Tier-E
leaves on their own axes — they are NOT part of the serialization/application
spine and MUST NOT be folded into this JSON node. `Application.Input.Schema`,
`Application.CommandLine.ArgParse`, and `Application.Configuration.Decoder` are
the DOWNSTREAM spine nodes framed after this lands (Json is their predecessor),
not part of this node. If a Json increment appears to need one of them, that is
a signal to STOP and route to the Steward, not to widen scope.

## Deliverables (D0-first, the proven Tier A/B/C/D shape)

- **D0 — remeasure the provider closure + the consumer surface (the judgment
  step).** D0 is a MEASUREMENT plus a LEDGER, not an edit:
  1. Measure Json's standalone `UnresolvedCon`/`UnboundName` set (the loader is
     the authority) and the exact provider that owns each unresolved symbol.
     Emit the exact selective-import lines to adopt (module path + name list),
     grounded in the ambient closure above — confirm each provider is an
     already-published lower-tier surface (Tier A Derived, Tier D
     Capability.Parsing.Cursor), re-measured at the pickup SHA.
  2. Measure the client-consumed public surface — the exact set of names to
     publish so Json is a usable spine entry — and confirm it is the minimal
     usable surface (no internal helper over-published; no unused export).
  3. Emit the ledger: the exact imports to adopt and the exact surface to
     publish. This is the deliverable D1 executes verbatim.
- **D0 HARD-STOPS to the Steward** on either of:
  - (a) an UNPUBLISHED PROVIDER — a symbol Json needs whose owning module has
    NOT yet published it as a consumer surface (a hidden dep on a still-scaffolded
    module). Cite it and split it out; do NOT drag an unpublished provider in.
  - (b) an UNEXPECTED TIER-E EDGE — a dependency the DAG did not predict (any
    provider outside {Tier-A Derived, Tier-D Capability.Parsing.Cursor, prelude}).
    Cite it; the slice cut is wrong, do not bend scope to absorb it.
- **D1 — migrate + publish, executing the D0 ledger.** Replace the ambient/
  scaffold resolution with the exact D0-measured selective imports (retire the
  ambient reach); publish exactly the D0-measured usable consumer surface
  (nothing more); extend the module's loader-visible inventory to reflect both
  the new imports and the published surface (imported names are not new exports —
  reflect them the way an existing import is reflected); the module elaborates
  standalone (exit 0). No carrier change, no new proof content — the four cursor
  proofs and `Json`'s definition are preserved verbatim; `trusted_base()`
  unaffected.

## Acceptance criteria, each with its control

- **AC-IMPORTS-EXACT (the D0 ledger is adopted verbatim).** Json's import block
  after D1 equals exactly the D0-measured selective-import set — same module
  paths, same name lists, no extra name and no ambient residue. Population from
  the module's own import block; verdict from the D0 ledger. Control: adding a
  name D0 did not measure, or dropping one it did, reds the equality distinctly;
  a per-import reddening mutation (delete one adopted name) reds standalone
  elaboration at the exact `UnresolvedCon`/`UnboundName` D0 recorded for that
  provider.
- **AC-NO-AMBIENT / STRICT-RESOLUTION (the scaffold reach is gone).** Json
  elaborates standalone (exit 0) with NO whole-catalog class-install /
  scaffolding fallback in the resolution path — every provider symbol resolves
  through a real selective import or the prelude, measured by the loader.
  Control: removing the newly-adopted Cursor import restores the exact prior
  standalone failure (the `UnresolvedCon`/`UnboundName` set D0 recorded for the
  cursor-abstraction symbols) — the reddening proves the strict path, not the
  ambient one, now carries resolution.
- **AC-SURFACE-MINIMAL (only the usable consumer surface is published).** The
  set of loader-visible exports from Json equals exactly the D0-measured usable
  consumer surface — each published name resolves to Json's `GlobalId` via a
  selective import (measured by the loader, not a `^pub` grep), and each name D0
  ruled internal (e.g. `char_cursor_lt_suc`, the `char_cursor_*` helper fns
  unless D0 measures a consumer for them) still rejects `UnboundName` from an
  external import. Control: a probe import of an intended export resolves; a
  probe import of a still-internal sibling rejects `UnboundName`; a per-symbol
  reddening mutation (publish one extra, or private one intended) reds distinctly.
- **AC-INVENTORY-EXACT.** Json's loader-visible inventory equality extends by
  EXACTLY the intended names (the published surface + the adopted imports;
  nothing else changes visibility). Population from the module's definitions,
  verdict from the loader, a per-symbol reddening mutation each reds distinctly.
- **AC-TRUST-INVARIANT (carrier + proofs preserved).** `Json` and its six
  constructors are byte-unchanged; the four cursor proofs still check; no new
  axiom/postulate/primitive/opaque constant is introduced. Control: a
  `trusted_base()` differential shows delta zero; a diff of the `data Json`
  block and the proof bodies shows BYTE-UNCHANGED.
- **AC-NO-REGRESSION.** Re-run the COMPLETE affected-target closure (every target
  loading Json or a module whose closure this changes), scoped by changed PATHS.
  Targeted via `scripts/ken-cargo`, never `--workspace` (green in CI is the
  workspace verdict). Because zero catalog module imports Json today, the
  external-consumer closure is expected empty — confirm that census rather than
  assume it.

## Hard stop

Route to the Steward if:

- D0 surfaces an UNPUBLISHED PROVIDER — a symbol Json needs whose owning module
  has not yet published it (hidden dep on a still-scaffolded module). Split it
  out with a cited reason; do not drag the unpublished provider in; or
- D0 surfaces an UNEXPECTED TIER-E EDGE — a provider outside the predicted set
  {Tier-A Derived, Tier-D Capability.Parsing.Cursor, prelude}. The DAG cut is
  wrong; do not bend scope to absorb it; or
- delivering D1 appears to require a carrier change, new proof content, binding a
  new class/instance, or pulling in a downstream spine node
  (Application.Input.Schema and beyond). Any of those means the slice cut is
  wrong, not that the scope should bend.

## Format any edited catalog file before handoff

Every file under `catalog/` must already be in canonical `kenfmt` form — there is
a corpus-wide formatter fixed-point gate (`kenfmt_c_capstone` /
`ken-cli/tests/ken_fmt.rs`) keyed on every catalog file. It fires only in a
full-workspace run, which is CI-only by operator hard rule, so a targeted
`-p ken-elaborator --test <yours>` cannot see it. Run the formatter on
`Json.ken.md` before handing the SHA to QA; do NOT run `--workspace` locally to
check it.

## Tier: T2

Single-module D0-measured selective-import migration + surface publish — the
proven Tier-A / B / C / D publish-and-import shape, no novel design. The one
judgment lives in the D0 measurement (is every ambient provider owned by an
already-published lower-tier module?) and the two hard-stops; the D1 tail is
mechanical (adopt the ledger's imports, publish the ledger's surface, extend the
inventory, standalone-green). No class-instance relocation, no proof authoring,
no carrier change.

## Gate, reviewer, sequencing

`gate: none` (no TCB touch; additive `catalog/`-only surface + import work). On
the candidate: **Foundation QA + CV** on the exact SHA, then Steward M1-M4 ->
lieutenant. **Architect** is required only if D0 fires a hard stop (unpublished
provider or unexpected Tier-E edge) — that is the only condition pulling an
Architect decomposition refresh; otherwise this is a mechanically completable
migration. First Tier-E spine node; successor to the Tier-D tail (all merged).
The next spine node (`Application.Input.Schema`) is framed one release ahead
after this lands, with fixed inputs re-measured at that SHA.

## Contention and sequencing

`catalog/` only, and additive. Re-measure contention at pickup, not from this
frame. The concurrent doc track touches `library/` and `agent/`, disjoint from
`catalog/`. The independent Tier-E algorithm leaves (`InsertionSort`,
`OrderedSearch`, `Gcd`, `Property`) are on their own axes and are not in flight
under this node. Foundation holds no other released node; this is the ring's
work.
