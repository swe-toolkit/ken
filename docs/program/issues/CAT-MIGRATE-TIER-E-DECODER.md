---
id: CAT-MIGRATE-TIER-E-DECODER
title: "Scaffold-retirement Tier E, serialization/application spine (the LAST Schema client): migrate Application.Configuration.Decoder off ambient/scaffold resolution onto its D0-measured selective imports — Application.Input.Schema (the ~15 consumed names from the published 26-name surface), Capability.Diagnostics.Core (Diagnostic/DiagnosticCode and any MkDiagnostic/Origin reached), Capability.Formatting.Doc (Doc), with Data.Collections.NonEmpty (NonEmpty, nonempty_map) and Data.Sums.Validation (Invalid, Valid, Validation) already selective — and publish its usable consumer surface. D0 measures Decoder's actual provider closure and client-consumed public surface, adopts exactly that import set, and publishes exactly that surface. NO new proof authoring, NO carrier change; the proven Tier A/B/C/D/E-Json/E-Schema/E-ArgParse publish-and-import shape."
status: active
owner: foundation
size: S
gate: none
tier: T2
depends_on: [CAT-MIGRATE-TIER-E-SCHEMA, CAT-MIGRATE-TIER-D-PROCESS-ENVIRONMENT]
blocks: []
github: null
origin: "Steward-framed 2026-09-10 on the foundation-leader's named spine (evt_2fk3r5y2g3r61: Json -> Application.Input.Schema -> {ArgParse, Decoder}), released as foundation's next deliverable the moment CAT-MIGRATE-TIER-E-ARGPARSE landed (295ba35a3, blob-verified). Fourth and FINAL Tier-E serialization/application-spine node of the [[CAT-SCAFFOLD-RETIREMENT]] DAG and the SECOND (smaller) real client of the published Application.Input.Schema surface — the sibling of ArgParse, INDEPENDENT of it (Decoder consumes Schema, not ArgParse; the true dep is on the merged CAT-MIGRATE-TIER-E-SCHEMA, sequenced second under the one-WP rule as the smaller client: it references ~15 of Schema's 26 published names vs ArgParse's ~20). Module identity GROUNDED at origin/main 295ba35a3: Decoder = catalog/packages/Application/Configuration/Decoder.ken.md (blob 5a6ea273f, 159 lines)."
---

> # HELD 2026-09-10 — blocked on new predecessor CAT-MIGRATE-TIER-D-PROCESS-ENVIRONMENT.
>
> Decoder's D0 (foundation-implementer evt_18aqwyk476b4w) surfaced a genuine
> hard-stop (a): it consumes `process_environment`, owned by the UNPUBLISHED
> `Capability.Process.Environment` (no `pub`/`export` at 710479b5d). Ruling
> evt_155n7vtdndkm3: publish that provider FIRST — a near-trivial pure migration
> mirroring the landed `Capability.Process.Arguments`. Decoder resumes (the
> Steward re-releases it) the moment CAT-MIGRATE-TIER-D-PROCESS-ENVIRONMENT
> lands; its D0 picks up from the same point and the LawfulClasses ruling
> (evt_73wh7dgneqe6y) stands. depends_on updated; do NOT work Decoder until
> re-released.

> # RELEASED 2026-09-10 (Steward) — Tier E spine, node 4 (FINAL): last Schema client.
>
> Released to the foundation ring as lane 3's next deliverable the moment
> CAT-MIGRATE-TIER-E-ARGPARSE landed (295ba35a3, blob-verified). Part of the
> [[CAT-SCAFFOLD-RETIREMENT]] 5-tier DAG; the FOURTH and last Tier-E
> serialization/application-spine node, the SECOND real consumer of the
> published `Application.Input.Schema` surface (the `depends_on` is on Schema,
> not ArgParse — Decoder is independent of its sibling). Its provider closure
> resolves into already-published lower-tier modules (`Application.Input.Schema`,
> `Capability.Diagnostics.Core`, `Capability.Formatting.Doc`,
> `Data.Collections.NonEmpty`, `Data.Sums.Validation`). On the candidate: fresh
> Foundation QA + CV on the exact SHA; Architect required only if D0 fires a
> hard stop (unpublished provider, or an unexpected Tier-E edge — see the D0
> stop rule), then Steward M1-M4. After it lands, the serialization/application
> spine is complete; the remaining Tier-E work is the independent algorithm
> leaves (`InsertionSort`, `OrderedSearch`, `Gcd`, `Property`) on their own axes.

## Not a regression fix (read before treating standalone-red as a bug)

`Application.Configuration.Decoder` elaborates TODAY in the full-catalog build via
ambient resolution — the whole-catalog class-install / scaffolding fallback
supplies the Schema, Diagnostics, and formatting symbols it references without an
import line (the operator's class-uniformity ruling, 2026-09-02). The module
EXISTS and is real (159 lines) — this is a CLEANNESS migration of a real module,
not a new build. "Decoder fails standalone at UnresolvedCon/UnboundName Y" is the
STARTING condition this node closes toward the scaffold-retirement end state
(`zero catalog dependence on fixture scaffolding / ambient resolution`,
[[CAT-SCAFFOLD-RETIREMENT]]), not a defect on `main`.

## Measured surface (origin/main 295ba35a3 — carry, D0 re-measures at pickup)

Grounded from the module text (blob 5a6ea273f), for orientation only — D0
re-measures the free-symbol closure at the pickup SHA via the loader, not from
this frame:

- **Current imports (already selective — keep/confirm, no bare import to fix):**
  - `Data.Collections.NonEmpty (NonEmpty, nonempty_map)` — selective already.
    Consume NonEmpty ONLY through its PUBLIC smart constructors, never a raw
    `NonEmptyCons` — the abstract-NonEmpty boundary the CV checked for Schema and
    ArgParse holds for this client too
    ([[abstract-pub-data-must-earn-its-keep...]]). If D0 measures an additional
    consumed NonEmpty name, extend the selective list to exactly that set.
  - `Data.Sums.Validation (Invalid, Valid, Validation)` — selective already;
    keep/confirm (D0 re-checks whether `validation_ap`/`validation_map` are also
    consumed and, if so, adds them).
- **Ambiently resolved provider closure (no import line today — the reach this
  node retires).** Every one is OWNED and PUBLISHED by an already-migrated
  module (confirm each at the pickup SHA via the loader — the owner is the module
  whose `export` line lists the name):
  - The Schema vocabulary — observed references include `Schema`, `SchemaField`,
    `SchemaFieldCheck`, `SchemaIssue`, `SchemaValidation`, … — owned by the
    published `Application.Input.Schema` (the 26-name export from
    CAT-MIGRATE-TIER-E-SCHEMA, merged; ArgParse already validated the surface as
    its first client). D0 confirms every consumed name is in the published set
    (it should be ~15 of the 26).
  - `Diagnostic`, `DiagnosticCode` (and any `MkDiagnostic`/`Origin`/
    `MkDiagnosticCode` reached) — owned by `Capability.Diagnostics.Core`,
    already migrated + published.
  - `Doc` — owned by `Capability.Formatting.Doc` (`export Doc, Text, …`),
    published.
  - `bytes_deceq_eq` — owned by `Core.Classes.LawfulClasses` (`pub fn
    bytes_deceq_eq` with `pub proof sound/complete`, in its published
    inventory), a Core-tier provider already imported by 14+ migrated catalog
    modules (Diagnostics.Core, Formatting.Doc, Derived among them; Posix imports
    the sibling `uint8_deceq_eq`, StringKeys the analogue `string_deceq_eq`).
    D0 measured this AUTHORIZED edge on 2026-09-10 (foundation-implementer
    evt_7gb92rpvbafz6) and the Steward ruled it in (evt_73wh7dgneqe6y): adopt
    `import Core.Classes.LawfulClasses (bytes_deceq_eq)`, extended to exactly the
    LawfulClasses names D0 measures consumed. It is a published lower-tier
    provider omitted from the original orientation estimate, not a broken cut.
  - `process_environment` (in `decode_process_environment`) — owned by
    `Capability.Process.Environment`, which is UNPUBLISHED at 710479b5d (no
    `pub`/`export`). This is the genuine hard-stop (a) that HOLDS this node: the
    predecessor CAT-MIGRATE-TIER-D-PROCESS-ENVIRONMENT publishes that surface
    first; once it lands, Decoder adopts `import Capability.Process.Environment
    (process_environment, …)` — exactly the D0-measured names.
- **Prelude-ambient symbols** (`List`, `Cons`, `Nil`, `Char`, `String`, `Bool`,
  `Nat`, `Prod`, `Some`/`None`, `True`/`False`, `match`/`↦`, …): D0 confirms via
  the loader which are prelude/built-in (no import) and which — if any — resolve
  to a published module and need a selective import. (No `list_append` or
  `string_to_list_char` observed in Decoder, unlike ArgParse; D0 re-confirms.)
- **Declared public API: confirm whether Decoder has an `export` line today** (it
  does NOT at 295ba35a3). If none, part of this migration is ADDING the export
  line with exactly the D0-measured usable consumer surface (the decoder public
  entry points a configuration/env driver consumes). No catalog module imports
  Decoder today, so the surface is measured, not inherited from a live importer —
  publish exactly what the module's own role requires and no internal helper.

## DAG position

The FOURTH and final serialization/application spine node and the SECOND real
client of the published `Application.Input.Schema` (CAT-MIGRATE-TIER-E-SCHEMA,
merged; hence the `depends_on`). It is the sibling of ArgParse and INDEPENDENT of
it — Decoder does not consume any ArgParse surface, so it depends on Schema, not
ArgParse. Its provider closure crosses into `Capability.Diagnostics.Core` and
`Capability.Formatting.Doc` (both already published) — those cross-tier edges are
PREDICTED, not unexpected. After Decoder lands, the serialization/application
spine (Json -> Schema -> {ArgParse, Decoder}) is complete.

## NOT this node — sibling / independent Tier-E work (do not fold in)

`Application.CommandLine.ArgParse` is the OTHER Schema consumer and is already
MERGED (295ba35a3) — do not re-touch it. `Algorithm.Sorting.InsertionSort`,
`Algorithm.Searching.OrderedSearch`, `Algorithm.Numeric.Gcd`, and
`Tooling.Testing.Property` are INDEPENDENT Tier-E leaves on their own axes and
MUST NOT be folded in. If a Decoder increment appears to need ArgParse or a
downstream driver, STOP and route to the Steward, not widen scope.

## Deliverables (D0-first, the proven Tier A/B/C/D/E shape)

- **D0 — remeasure the provider closure + the consumer surface (the judgment
  step).** D0 is a MEASUREMENT plus a LEDGER, not an edit:
  1. Measure Decoder's standalone `UnresolvedCon`/`UnboundName` set (the loader
     is the authority) and the exact provider that owns each unresolved symbol.
     Emit the exact selective-import lines to adopt (module path + name list),
     grounded in the ambient closure above — confirm each provider is an
     already-published lower-tier surface, re-measured at the pickup SHA.
     Explicitly confirm every consumed Schema name is in Schema's published
     26-name surface (the second-client validation), and re-confirm the existing
     NonEmpty and Validation selective lists are exactly the consumed set (extend
     if D0 measures more).
  2. Measure the client-consumed public surface — the exact set of names to
     publish so Decoder is a usable configuration/env-decoder entry — and confirm
     it is the minimal usable surface (no internal helper over-published; no
     unused export). If there is no `export` line today, this is the set the
     added `export` line carries.
  3. Emit the ledger: the exact imports to adopt and the exact surface to
     publish. This is the deliverable D1 executes verbatim.
- **D0 HARD-STOPS to the Steward** on either of:
  - (a) an UNPUBLISHED PROVIDER — a symbol Decoder needs whose owning module has
    NOT yet published it as a consumer surface (in particular, a consumed Schema
    name that is NOT in Schema's published 26-name surface — that would mean the
    Schema export was mis-cut; cite it). Split it out; do NOT drag an unpublished
    provider in.
  - (b) an UNEXPECTED TIER-E EDGE — a dependency outside the predicted set
    {Application.Input.Schema, Capability.Diagnostics.Core,
    Capability.Formatting.Doc, Data.Collections.Derived, Data.Collections.NonEmpty,
    Data.Sums.Validation, Core.Classes.LawfulClasses,
    Capability.Process.Environment, prelude}. Cite it; the slice cut is wrong,
    do not bend scope to absorb it.
- **D1 — migrate + publish, executing the D0 ledger.** Replace the ambient/
  scaffold resolution with the exact D0-measured selective imports (retire the
  ambient reach; extend the NonEmpty/Validation selective lists only if D0
  measured more names); ADD/adjust the `export` line to publish exactly the
  D0-measured usable consumer surface (nothing more); extend the module's
  loader-visible inventory to reflect both the new imports and the published
  surface; the module elaborates standalone (exit 0). No carrier change, no new
  proof content — every `data`/`fn`/`const` definition is preserved verbatim;
  `trusted_base()` unaffected.

## Acceptance criteria, each with its control

- **AC-IMPORTS-EXACT (the D0 ledger is adopted verbatim).** Decoder's import
  block after D1 equals exactly the D0-measured selective-import set — same module
  paths, same name lists, no extra name and no ambient residue. Population from
  the module's own import block; verdict from the D0 ledger. Control: adding a
  name D0 did not measure, or dropping one it did, reds the equality distinctly;
  a per-import reddening mutation (delete one adopted name) reds standalone
  elaboration at the exact `UnresolvedCon`/`UnboundName` D0 recorded for that
  provider.
- **AC-NO-AMBIENT / STRICT-RESOLUTION (the scaffold reach is gone).** Decoder
  elaborates standalone (exit 0) with NO whole-catalog class-install /
  scaffolding fallback in the resolution path — every provider symbol resolves
  through a real selective import or the prelude, measured by the loader.
  Control: removing the newly-adopted Schema (or Diagnostics.Core) import
  restores the exact prior standalone failure (the `UnresolvedCon`/`UnboundName`
  set D0 recorded for those symbols) — the reddening proves the strict path, not
  the ambient one, now carries resolution.
- **AC-SCHEMA-CLIENT-EXACT (second-client validation).** Every Schema name
  Decoder imports resolves to `Application.Input.Schema`'s published `GlobalId`
  (measured by the loader), confirming the Schema export surface is usable by its
  second real client. Control: a probe import of a Schema name D0 measured as
  consumed resolves; a probe import of a Schema name D0 measured as INTERNAL (one
  of Schema's private helpers) rejects `UnboundName`.
- **AC-SURFACE-MINIMAL (only the usable consumer surface is published).** The set
  of loader-visible exports from Decoder equals exactly the D0-measured usable
  consumer surface — each published name resolves to Decoder's `GlobalId` via a
  selective import (measured by the loader, not an `^export` grep), and each name
  D0 ruled internal still rejects `UnboundName` from an external import. Control:
  a probe import of an intended export resolves; a probe import of a D0-internal
  sibling rejects `UnboundName`; a per-symbol reddening mutation (publish one
  extra, or private one intended) reds distinctly.
- **AC-INVENTORY-EXACT.** Decoder's loader-visible inventory equality extends by
  EXACTLY the intended names (the published surface + the adopted imports;
  nothing else changes visibility). Population from the module's definitions,
  verdict from the loader, a per-symbol reddening mutation each reds distinctly.
- **AC-TRUST-INVARIANT (carrier + definitions preserved).** Every `data`/`fn`/
  `const` body is byte-unchanged; no new axiom/postulate/primitive/opaque
  constant is introduced. Control: a `trusted_base()` differential shows delta
  zero; a diff of the definition bodies shows BYTE-UNCHANGED (only the import
  block and the added/adjusted `export` line differ).
- **AC-NO-REGRESSION.** Re-run the COMPLETE affected-target closure (every target
  loading Decoder or a module whose closure this changes — the
  `cc8_env_config_decoder_acceptance` suite included), scoped by changed PATHS.
  Targeted via `scripts/ken-cargo`, never `--workspace` (green in CI is the
  workspace verdict). Retain every existing acceptance test unweakened (do not
  weaken a probe to pass — `merge-procedure.md`).

## Hard stop

Route to the Steward if:

- D0 surfaces an UNPUBLISHED PROVIDER — a symbol Decoder needs whose owning
  module has not yet published it (including a consumed Schema name absent from
  Schema's published 26-name surface — a mis-cut Schema export). Split it out
  with a cited reason; do not drag the unpublished provider in; or
- D0 surfaces an UNEXPECTED TIER-E EDGE — a provider outside the predicted set
  {Application.Input.Schema, Capability.Diagnostics.Core, Capability.Formatting.Doc,
  Data.Collections.Derived, Data.Collections.NonEmpty, Data.Sums.Validation,
  Core.Classes.LawfulClasses, Capability.Process.Environment, prelude}. The DAG
  cut is wrong; do not bend scope to absorb it; or
- delivering D1 appears to require a carrier change, new proof content, binding a
  new class/instance, or pulling in the sibling ArgParse or a downstream driver.
  Any of those means the slice cut is wrong, not that the scope should bend.

## Format any edited catalog file before handoff

Every file under `catalog/` must already be in canonical `kenfmt` form — there is
a corpus-wide formatter fixed-point gate (`kenfmt_c_capstone` /
`ken-cli/tests/ken_fmt.rs`) keyed on every catalog file. It fires only in a
full-workspace run, which is CI-only by operator hard rule, so a targeted
`-p ken-elaborator --test <yours>` cannot see it. Run the formatter on
`Decoder.ken.md` before handing the SHA to QA; do NOT run `--workspace` locally
to check it.

## Tier: T2

Single-module D0-measured selective-import migration + surface publish — the
proven Tier-A / B / C / D / E-Json / E-Schema / E-ArgParse publish-and-import
shape, no novel design. The judgment lives in the D0 measurement (is every
ambient provider owned by an already-published lower-tier module? is every
consumed Schema name in Schema's published surface? are the existing NonEmpty /
Validation selective lists exactly the consumed set?) and the two hard-stops; the
D1 tail is mechanical (adopt the ledger's imports, add/adjust the export line,
extend the inventory, standalone-green). No class-instance relocation, no proof
authoring, no carrier change. Smaller closure than ArgParse (159 lines vs 512,
two imports already selective) — the same mechanical shape, lighter.

## Gate, reviewer, sequencing

`gate: none` (no TCB touch; additive `catalog/`-only surface + import work). On
the candidate: **Foundation QA + CV** on the exact SHA, then Steward M1-M4 ->
lieutenant. **Architect** is required only if D0 fires a hard stop (unpublished
provider or unexpected Tier-E edge) — that is the only condition pulling an
Architect decomposition refresh; otherwise this is a mechanically completable
migration. Fourth and final Tier-E spine node; second real client of the merged
Schema surface. After it lands, the serialization/application spine is complete
and the remaining Tier-E work is the independent algorithm leaves.

## Contention and sequencing

`catalog/` only, and additive. Re-measure contention at pickup, not from this
frame. The concurrent doc track touches `library/` and `agent/`, disjoint from
`catalog/`. The now-merged sibling `ArgParse` and the independent Tier-E
algorithm leaves (`InsertionSort`, `OrderedSearch`, `Gcd`, `Property`) are on
their own axes and are not in flight under this node. Foundation holds no other
released node; this is the ring's work.
