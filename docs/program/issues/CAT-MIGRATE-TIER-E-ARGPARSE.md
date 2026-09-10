---
id: CAT-MIGRATE-TIER-E-ARGPARSE
title: "Scaffold-retirement Tier E, serialization/application spine (first real Schema client): migrate Application.CommandLine.ArgParse off ambient/scaffold resolution onto its D0-measured selective imports — Application.Input.Schema (the ~20 consumed names from the just-published 26-name surface), Capability.Diagnostics.Core (Diagnostic/MkDiagnostic/Origin/DiagnosticCode), Capability.Formatting.Doc (Doc/Text), Data.Collections.Derived (list_append), the bare Data.Collections.NonEmpty import made selective, and Data.Sums.Validation (already selective); string_to_list_char is prelude — and publish its usable consumer surface. D0 measures ArgParse's actual provider closure and client-consumed public surface, adopts exactly that import set, and publishes exactly that surface. NO new proof authoring, NO carrier change; the proven Tier A/B/C/D/E-Json/E-Schema publish-and-import shape."
status: merged
owner: foundation
size: S
gate: none
tier: T2
depends_on: [CAT-MIGRATE-TIER-E-SCHEMA]
blocks: []
github: null
origin: "Steward-framed 2026-09-09 on the foundation-leader's named spine (evt_2fk3r5y2g3r61: Json -> Application.Input.Schema -> {ArgParse, Decoder}), released as foundation's next deliverable the moment CAT-MIGRATE-TIER-E-SCHEMA landed (53ae947ad, blob-verified). Third Tier-E node of the [[CAT-SCAFFOLD-RETIREMENT]] DAG and the FIRST real client of the just-published Application.Input.Schema surface (Steward sequencing call between the two independent Schema consumers: ArgParse first as the fuller client — it references ~20 of Schema's 26 published names vs Decoder's ~15). Module identity GROUNDED at origin/main 53ae947ad: ArgParse = catalog/packages/Application/CommandLine/ArgParse.ken.md (blob df69db84a)."
---

# LANDED 2026-09-10 at 295ba35a3 — squash of candidate aaa50441e, all 6 paths blob-verified identical by the Steward. Foundation QA evt_13dzv1e8yns7c + CV evt_7e9vrz2g61qb8 on the exact SHA; Decision dec_391wchpvm88z7; no Architect (D0 cleared). ken-ci auto-close did not fire (github: null); Steward flipped active->merged by hand. The last Schema consumer, Application.Configuration.Decoder, is framed one release ahead (CAT-MIGRATE-TIER-E-DECODER).

> # RELEASED 2026-09-09 (Steward) — Tier E spine, node 3: the first Schema client.
>
> Released to the foundation ring as lane 3's next deliverable the moment
> CAT-MIGRATE-TIER-E-SCHEMA landed. Part of the [[CAT-SCAFFOLD-RETIREMENT]]
> 5-tier DAG; the THIRD Tier-E node, the FIRST real consumer of the just-
> published `Application.Input.Schema` surface (hence the `depends_on`). Its
> provider closure resolves into already-published lower-tier modules
> (`Application.Input.Schema`, `Capability.Diagnostics.Core`,
> `Capability.Formatting.Doc`, `Data.Collections.Derived`,
> `Data.Collections.NonEmpty`, `Data.Sums.Validation`). On the candidate: fresh
> Foundation QA + CV on the exact SHA; Architect required only if D0 fires a
> hard stop (unpublished provider, or an unexpected Tier-E edge — see the D0
> stop rule), then Steward M1-M4.

## Not a regression fix (read before treating standalone-red as a bug)

`Application.CommandLine.ArgParse` elaborates TODAY in the full-catalog build via
ambient resolution — the whole-catalog class-install / scaffolding fallback
supplies the Schema, Diagnostics, and formatting symbols it references without an
import line (the operator's class-uniformity ruling, 2026-09-02). The module
EXISTS and is substantial (512 lines) — this is a CLEANNESS migration of a real
module, not a new build. "ArgParse fails standalone at UnresolvedCon/UnboundName
Y" is the STARTING condition this node closes toward the scaffold-retirement end
state (`zero catalog dependence on fixture scaffolding / ambient resolution`,
[[CAT-SCAFFOLD-RETIREMENT]]), not a defect on `main`.

## Measured surface (origin/main 53ae947ad — carry, D0 re-measures at pickup)

Grounded from the module text (blob df69db84a), for orientation only — D0
re-measures the free-symbol closure at the pickup SHA via the loader, not from
this frame:

- **Current imports (adjust, do not assume correct):**
  - `Data.Sums.Validation (Invalid, Valid, Validation, validation_ap,
    validation_map)` — selective already; keep/confirm.
  - `Data.Collections.NonEmpty` — a BARE import with NO name list. The migration
    makes it SELECTIVE: adopt exactly the NonEmpty names ArgParse consumes
    (observed `nonempty_cons`, `nonempty_map`, and the `NonEmpty` type). Consume
    NonEmpty ONLY through its PUBLIC smart constructors, never a raw
    `NonEmptyCons` — the abstract-NonEmpty boundary the CV checked for Schema
    holds for this client too ([[abstract-pub-data-must-earn-its-keep...]]).
- **Ambiently resolved provider closure (no import line today — the reach this
  node retires).** Every one is OWNED and PUBLISHED by an already-migrated
  module (confirm each at the pickup SHA via the loader — the owner is the module
  whose `export` line lists the name):
  - The Schema vocabulary — `SchemaField`, `MkSchemaField`, `Schema`, `MkSchema`,
    `SchemaOptional`, `SchemaRequired`, `SchemaFlag`, `SchemaBytes`,
    `SchemaFieldCheck`, `SchemaFieldRejected`, `MkSchemaIssue`,
    `schema_field_presence`, `schema_validate`, `schema_help`, … — owned by the
    JUST-PUBLISHED `Application.Input.Schema` (the 26-name export from
    CAT-MIGRATE-TIER-E-SCHEMA, merged 53ae947ad). ArgParse is the FIRST real
    client of that surface; D0 confirms every consumed name is in the published
    set (it should be ~20 of the 26 — a live check that the Schema export was
    cut correctly).
  - `Diagnostic`, `MkDiagnostic` (and any `Origin`/`DiagnosticCode`/
    `MkDiagnosticCode` reached) — owned by `Capability.Diagnostics.Core`
    (`data Diagnostic = MkDiagnostic Origin DiagnosticCode`; `export Diagnostic,
    MkDiagnostic`; `export DiagnosticCode, MkDiagnosticCode`; `export Origin,
    …`), already migrated + published.
  - `Doc`, `Text` — owned by `Capability.Formatting.Doc` (`export Doc, Text, …`),
    published (help rendering).
  - `list_append` — owned by `Data.Collections.Derived` (`export … list_append`),
    a Tier-A provider, published.
  - `string_to_list_char` — the prelude primitive
    (`globals[id] == prelude_env.string_to_list_char_id`, as Schema's D0 pinned);
    needs no import. D0 re-confirms.
- **Prelude-ambient symbols** (`List`, `Cons`, `Nil`, `Char`, `String`, `Bool`,
  `Nat`, `Prod`, `Bytes`, `bytes_encode`, `Some`/`None`, `True`/`False`,
  `match`/`↦`, …): D0 confirms via the loader which are prelude/built-in (no
  import) and which — if any, e.g. a `Bytes`/`bytes_encode` owner — resolve to a
  published module and need a selective import.
- **Declared public API: confirm whether ArgParse has an `export` line today**
  (it does NOT at 53ae947ad). If none, part of this migration is ADDING the
  export line with exactly the D0-measured usable consumer surface (the argparse
  public entry points a CLI driver consumes). No catalog module imports ArgParse
  today, so the surface is measured, not inherited from a live importer — publish
  exactly the module's own role requires and no internal helper.

## DAG position

The THIRD serialization/application spine node and the FIRST real client of the
now-published `Application.Input.Schema` (CAT-MIGRATE-TIER-E-SCHEMA, merged;
hence the `depends_on`). Its sibling `Application.Configuration.Decoder` — the
other Schema consumer — is the next release after this (independent of ArgParse;
Steward-sequenced second as the smaller client). ArgParse's provider closure
crosses into `Capability.Diagnostics.Core` and `Capability.Formatting.Doc` (both
already published) — those cross-tier edges are PREDICTED, not unexpected.

## NOT this node — sibling / independent Tier-E work (do not fold in)

`Application.Configuration.Decoder` is the OTHER Schema consumer, framed as the
next release (a sibling, not part of this node — do not migrate both in one
slice). `Algorithm.Sorting.InsertionSort`, `Algorithm.Searching.OrderedSearch`,
`Algorithm.Numeric.Gcd`, and `Tooling.Testing.Property` are INDEPENDENT Tier-E
leaves on their own axes and MUST NOT be folded in. If an ArgParse increment
appears to need Decoder or a downstream driver, STOP and route to the Steward,
not widen scope.

## Deliverables (D0-first, the proven Tier A/B/C/D/E shape)

- **D0 — remeasure the provider closure + the consumer surface (the judgment
  step).** D0 is a MEASUREMENT plus a LEDGER, not an edit:
  1. Measure ArgParse's standalone `UnresolvedCon`/`UnboundName` set (the loader
     is the authority) and the exact provider that owns each unresolved symbol.
     Emit the exact selective-import lines to adopt (module path + name list),
     grounded in the ambient closure above — confirm each provider is an
     already-published lower-tier surface, re-measured at the pickup SHA.
     Explicitly confirm every consumed Schema name is in Schema's published
     26-name surface (the first-client validation), make the bare NonEmpty import
     selective, and re-confirm `string_to_list_char` is prelude.
  2. Measure the client-consumed public surface — the exact set of names to
     publish so ArgParse is a usable CLI-driver entry — and confirm it is the
     minimal usable surface (no internal helper over-published; no unused
     export). If there is no `export` line today, this is the set the added
     `export` line carries.
  3. Emit the ledger: the exact imports to adopt and the exact surface to
     publish. This is the deliverable D1 executes verbatim.
- **D0 HARD-STOPS to the Steward** on either of:
  - (a) an UNPUBLISHED PROVIDER — a symbol ArgParse needs whose owning module has
    NOT yet published it as a consumer surface (in particular, a consumed Schema
    name that is NOT in Schema's published 26-name surface — that would mean the
    Schema export was mis-cut; cite it). Split it out; do NOT drag an unpublished
    provider in.
  - (b) an UNEXPECTED TIER-E EDGE — a dependency outside the predicted set
    {Application.Input.Schema, Capability.Diagnostics.Core,
    Capability.Formatting.Doc, Data.Collections.Derived, Data.Collections.NonEmpty,
    Data.Sums.Validation, prelude}. Cite it; the slice cut is wrong, do not bend
    scope to absorb it.
- **D1 — migrate + publish, executing the D0 ledger.** Replace the ambient/
  scaffold resolution with the exact D0-measured selective imports (retire the
  ambient reach; make the bare NonEmpty import selective); ADD/adjust the
  `export` line to publish exactly the D0-measured usable consumer surface
  (nothing more); extend the module's loader-visible inventory to reflect both
  the new imports and the published surface; the module elaborates standalone
  (exit 0). No carrier change, no new proof content — every `data`/`fn`/`const`
  definition is preserved verbatim; `trusted_base()` unaffected.

## Acceptance criteria, each with its control

- **AC-IMPORTS-EXACT (the D0 ledger is adopted verbatim).** ArgParse's import
  block after D1 equals exactly the D0-measured selective-import set — same module
  paths, same name lists, no extra name and no ambient residue, and the NonEmpty
  import is now selective. Population from the module's own import block; verdict
  from the D0 ledger. Control: adding a name D0 did not measure, or dropping one
  it did, reds the equality distinctly; a per-import reddening mutation (delete
  one adopted name) reds standalone elaboration at the exact
  `UnresolvedCon`/`UnboundName` D0 recorded for that provider.
- **AC-NO-AMBIENT / STRICT-RESOLUTION (the scaffold reach is gone).** ArgParse
  elaborates standalone (exit 0) with NO whole-catalog class-install /
  scaffolding fallback in the resolution path — every provider symbol resolves
  through a real selective import or the prelude, measured by the loader.
  Control: removing the newly-adopted Schema (or Diagnostics.Core) import
  restores the exact prior standalone failure (the `UnresolvedCon`/`UnboundName`
  set D0 recorded for those symbols) — the reddening proves the strict path, not
  the ambient one, now carries resolution.
- **AC-SCHEMA-CLIENT-EXACT (first-client validation).** Every Schema name
  ArgParse imports resolves to `Application.Input.Schema`'s published `GlobalId`
  (measured by the loader), confirming the Schema export surface is usable by its
  first real client. Control: a probe import of a Schema name D0 measured as
  consumed resolves; a probe import of a Schema name D0 measured as INTERNAL
  (one of Schema's 12 private helpers) rejects `UnboundName`.
- **AC-SURFACE-MINIMAL (only the usable consumer surface is published).** The
  set of loader-visible exports from ArgParse equals exactly the D0-measured
  usable consumer surface — each published name resolves to ArgParse's `GlobalId`
  via a selective import (measured by the loader, not an `^export` grep), and
  each name D0 ruled internal still rejects `UnboundName` from an external import.
  Control: a probe import of an intended export resolves; a probe import of a
  D0-internal sibling rejects `UnboundName`; a per-symbol reddening mutation
  (publish one extra, or private one intended) reds distinctly.
- **AC-INVENTORY-EXACT.** ArgParse's loader-visible inventory equality extends by
  EXACTLY the intended names (the published surface + the adopted imports;
  nothing else changes visibility). Population from the module's definitions,
  verdict from the loader, a per-symbol reddening mutation each reds distinctly.
- **AC-TRUST-INVARIANT (carrier + definitions preserved).** Every `data`/`fn`/
  `const` body is byte-unchanged; no new axiom/postulate/primitive/opaque
  constant is introduced. Control: a `trusted_base()` differential shows delta
  zero; a diff of the definition bodies shows BYTE-UNCHANGED (only the import
  block and the added/adjusted `export` line differ).
- **AC-NO-REGRESSION.** Re-run the COMPLETE affected-target closure (every target
  loading ArgParse or a module whose closure this changes — the argparse
  acceptance suite `cc7` included), scoped by changed PATHS. Targeted via
  `scripts/ken-cargo`, never `--workspace` (green in CI is the workspace
  verdict). Retain every existing acceptance test unweakened (do not weaken a
  probe to pass — `merge-procedure.md`).

## Hard stop

Route to the Steward if:

- D0 surfaces an UNPUBLISHED PROVIDER — a symbol ArgParse needs whose owning
  module has not yet published it (including a consumed Schema name absent from
  Schema's published 26-name surface — a mis-cut Schema export). Split it out
  with a cited reason; do not drag the unpublished provider in; or
- D0 surfaces an UNEXPECTED TIER-E EDGE — a provider outside the predicted set
  {Application.Input.Schema, Capability.Diagnostics.Core, Capability.Formatting.Doc,
  Data.Collections.Derived, Data.Collections.NonEmpty, Data.Sums.Validation,
  prelude}. The DAG cut is wrong; do not bend scope to absorb it; or
- delivering D1 appears to require a carrier change, new proof content, binding a
  new class/instance, or pulling in the sibling Decoder or a downstream driver.
  Any of those means the slice cut is wrong, not that the scope should bend.

## Format any edited catalog file before handoff

Every file under `catalog/` must already be in canonical `kenfmt` form — there is
a corpus-wide formatter fixed-point gate (`kenfmt_c_capstone` /
`ken-cli/tests/ken_fmt.rs`) keyed on every catalog file. It fires only in a
full-workspace run, which is CI-only by operator hard rule, so a targeted
`-p ken-elaborator --test <yours>` cannot see it. Run the formatter on
`ArgParse.ken.md` before handing the SHA to QA; do NOT run `--workspace` locally
to check it.

## Tier: T2

Single-module D0-measured selective-import migration + surface publish — the
proven Tier-A / B / C / D / E-Json / E-Schema publish-and-import shape, no novel
design. The judgment lives in the D0 measurement (is every ambient provider
owned by an already-published lower-tier module? is every consumed Schema name in
Schema's published surface? which NonEmpty names does the bare import need?) and
the two hard-stops; the D1 tail is mechanical (adopt the ledger's imports, make
NonEmpty selective, add/adjust the export line, extend the inventory,
standalone-green). No class-instance relocation, no proof authoring, no carrier
change. Larger closure than Schema (512 lines, more providers) but the same
mechanical shape.

## Gate, reviewer, sequencing

`gate: none` (no TCB touch; additive `catalog/`-only surface + import work). On
the candidate: **Foundation QA + CV** on the exact SHA, then Steward M1-M4 ->
lieutenant. **Architect** is required only if D0 fires a hard stop (unpublished
provider or unexpected Tier-E edge) — that is the only condition pulling an
Architect decomposition refresh; otherwise this is a mechanically completable
migration. Third Tier-E spine node; first real client of the merged Schema
surface. The sibling consumer (`Application.Configuration.Decoder`) is framed one
release ahead after this lands, with fixed inputs re-measured at that SHA.

## Contention and sequencing

`catalog/` only, and additive. Re-measure contention at pickup, not from this
frame. The concurrent doc track touches `library/` and `agent/`, disjoint from
`catalog/`. The sibling `Decoder` and the independent Tier-E algorithm leaves
(`InsertionSort`, `OrderedSearch`, `Gcd`, `Property`) are on their own axes and
are not in flight under this node. Foundation holds no other released node; this
is the ring's work.
