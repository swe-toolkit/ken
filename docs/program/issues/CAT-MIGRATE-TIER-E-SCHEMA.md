---
id: CAT-MIGRATE-TIER-E-SCHEMA
title: "Scaffold-retirement Tier E, serialization/application spine (successor to Json): migrate the shared schema module Application.Input.Schema off ambient/scaffold resolution onto its D0-measured selective imports from already-published lower tiers (Capability.Formatting.Doc for Doc/Text, Data.Collections.Derived for list_append, the D0-pinned owner of string_to_list_char; NonEmpty and Validation are already imported), and publish only its usable consumer surface. D0 measures Schema's actual provider closure and its client-consumed public surface, adopts exactly that import set, and publishes exactly that surface. NO new proof authoring, NO carrier change; the proven Tier A/B/C/D/E-Json publish-and-import shape."
status: merged
owner: foundation
size: S
gate: none
tier: T2
depends_on: [CAT-MIGRATE-TIER-E-JSON]
blocks: []
github: null
origin: "Steward-framed 2026-09-09 on the foundation-leader's naming (evt_2fk3r5y2g3r61) of the serialization/application spine Json -> Application.Input.Schema -> {ArgParse, Decoder}, released as foundation's next deliverable the moment CAT-MIGRATE-TIER-E-JSON landed (01d2a7511, blob-verified). Second Tier-E node of the [[CAT-SCAFFOLD-RETIREMENT]] DAG, the spine successor to the just-landed Json entry node. Module identity GROUNDED at origin/main 01d2a7511: Schema = catalog/packages/Application/Input/Schema.ken.md (blob f081944cf4e9)."
---

> # RELEASED 2026-09-09 (Steward) — Tier E spine, node 2: the shared schema.
>
> Released to the foundation ring as lane 3's next deliverable the moment the
> Tier-E entry node (Json) landed. Part of the [[CAT-SCAFFOLD-RETIREMENT]]
> 5-tier DAG; the SECOND Tier-E (serialization/application) node, the spine
> successor to `Data.Serialization.Json` (CAT-MIGRATE-TIER-E-JSON, merged). It
> is the client-independent schema shared by the two downstream spine consumers
> (`Application.CommandLine.ArgParse`, `Application.Configuration.Decoder`). Its
> provider closure resolves into already-published lower-tier modules
> (`Capability.Formatting.Doc`, `Data.Collections.Derived`,
> `Data.Collections.NonEmpty`, `Data.Sums.Validation`). On the candidate: fresh
> Foundation QA + CV on the exact SHA; Architect required only if D0 fires a
> hard stop (unpublished provider, or an unexpected Tier-E edge — see the D0
> stop rule), then Steward M1-M4.

## Not a regression fix (read before treating standalone-red as a bug)

`Application.Input.Schema` elaborates TODAY in the full-catalog build via ambient
resolution — the whole-catalog class-install / scaffolding fallback supplies the
formatting and list symbols it references (`Doc`, `Text`, `list_append`,
`string_to_list_char`) without an import line (the operator's class-uniformity
ruling, 2026-09-02). The module EXISTS and is substantial (237 lines) — this is a
CLEANNESS migration of a real module, not a new build. "Schema fails standalone
at UnresolvedCon/UnboundName Y" is the STARTING condition this node closes toward
the scaffold-retirement end state (`zero catalog dependence on fixture
scaffolding / ambient resolution`, [[CAT-SCAFFOLD-RETIREMENT]]), not a defect on
`main`.

## Measured surface (origin/main 01d2a7511 — carry, D0 re-measures at pickup)

Grounded from the module text (blob f081944cf4e9), for orientation only — D0
re-measures the free-symbol closure at the pickup SHA via the loader, not from
this frame:

- **Current selective imports (already correct, keep):**
  - `Data.Collections.NonEmpty (NonEmpty, nonempty_append, nonempty_cons)` —
    Tier-C, published (NonEmpty smart-ctor/validation nodes merged).
  - `Data.Sums.Validation (Invalid, Valid, Validation)` — Tier-C, published
    (CAT-MIGRATE-TIER-C-VALIDATION-STRICT-IMPORT merged).
- **Ambiently resolved provider closure (no import line today — the reach this
  node retires).** Every one is OWNED and PUBLISHED by an already-migrated
  lower-tier module (confirm each at the pickup SHA via the loader; the owner is
  the module whose `export` line lists the name):
  - `Doc`, `Text` — owned by `Capability.Formatting.Doc`
    (`data Doc : Type where { ... }`; `export Doc, Text, Line, Concat, Group`).
    The material cross-tier edge: a Tier-E application module consuming the
    already-published `Capability.Formatting.Doc` surface. PREDICTED, not an
    unexpected edge.
  - `list_append` — owned by `Data.Collections.Derived`
    (`export ... length, list_append`), a Tier-A provider, already published and
    already the source of Json's `length` import.
  - `string_to_list_char` — a string/char decomposition helper used in the help
    traversal. D0 PINS its owning published module (candidate owner
    `Data.Text.StringBijection` or `Data.Collections.Derived`; if it is instead
    prelude/built-in, D0 records that and needs no import for it — the same
    prelude-vs-provider determination Json's D0 made for `and_intro`/`And`).
- **Prelude-ambient symbols** used in the definitions (`List`, `Cons`, `Nil`,
  `Char`, `String`, `Type`, `↦`/`match`): prelude/built-in, not an
  unpublished-catalog provider. D0 confirms via the loader whether the
  scaffold-retirement surface needs any import for them or they stay prelude.
- **Definitions in the module:** `data SchemaPresence` (`SchemaRequired`,
  `SchemaOptional`); `data SchemaValueShape` (`SchemaFlag`, `SchemaBytes`);
  `data SchemaField` (`MkSchemaField`); `data Schema` (`MkSchema`);
  `data SchemaIssue` (`MkSchemaIssue`); `data SchemaFieldCheck`
  (`SchemaFieldAccepted`, `SchemaFieldRejected`); the `const SchemaValidation`;
  and the field/validation/help fns (`schema_field_name` ..
  `schema_field_documentation`,
  `schema_name`/`schema_documentation`/`schema_fields`, `schema_field_accept`,
  `schema_field_reject`, `schema_check_presence`, `schema_issue_origin`,
  `schema_issue_code`, `schema_validation_cons`, `schema_validate_fields`,
  `schema_validate`, `schema_presence_chars`, `schema_shape_chars`,
  `schema_field_label_chars`, `schema_field_detail_chars`,
  `schema_field_help_chars`, `schema_fields_help_chars`, `schema_help`).
- **Declared public API: THERE IS NO `export` LINE TODAY.** Part of this
  migration is ADDING the export line carrying exactly the D0-measured usable
  consumer surface. D0 measures which names the two downstream clients
  (`ArgParse`, `Decoder`) actually consume — the schema-description carriers and
  the two traversals (`schema_validate`, `schema_help`) plus the accessors/
  constructors those clients need — and publishes exactly that set, no internal
  helper over-published.
- **Client-consumed surface:** the two spine consumers `ArgParse` and `Decoder`
  are the downstream clients (framed after this node lands). D0 measures the
  exact set they consume — measured, not inherited from a live importer if they
  do not yet import Schema — and publishes exactly the surface Schema's shared
  role requires and nothing more.

## DAG position

The SECOND serialization/application spine node, the successor to the now-landed
Tier-E entry node `Data.Serialization.Json` (CAT-MIGRATE-TIER-E-JSON, merged;
hence the `depends_on`). This node PRECEDES its two sibling consumers
`Application.CommandLine.ArgParse` and `Application.Configuration.Decoder`, which
are framed after this lands (Schema is their shared predecessor). Schema's
provider closure crosses into `Capability.Formatting.Doc` (a Tier-E application
module consuming the already-published `Capability.Formatting.Doc` surface) —
that cross-tier edge is PREDICTED (Doc is published), not an unexpected edge.

## NOT this node — independent / downstream Tier-E work (do not fold in)

`Application.CommandLine.ArgParse` and `Application.Configuration.Decoder` are
the DOWNSTREAM spine consumers framed after this lands (Schema is their shared
predecessor), NOT part of this node. `Algorithm.Sorting.InsertionSort`,
`Algorithm.Searching.OrderedSearch`, `Algorithm.Numeric.Gcd`, and
`Tooling.Testing.Property` are INDEPENDENT Tier-E leaves on their own axes and
MUST NOT be folded in. If a Schema increment appears to need one of the client
modules, that is a signal to STOP and route to the Steward, not to widen scope.

## Deliverables (D0-first, the proven Tier A/B/C/D/E-Json shape)

- **D0 — remeasure the provider closure + the consumer surface (the judgment
  step).** D0 is a MEASUREMENT plus a LEDGER, not an edit:
  1. Measure Schema's standalone `UnresolvedCon`/`UnboundName` set (the loader is
     the authority) and the exact provider that owns each unresolved symbol.
     Emit the exact selective-import lines to adopt (module path + name list),
     grounded in the ambient closure above — confirm each provider is an
     already-published lower-tier surface (Capability.Formatting.Doc,
     Data.Collections.Derived, the string_to_list_char owner, NonEmpty,
     Validation), re-measured at the pickup SHA. PIN string_to_list_char's owner
     (or record it prelude).
  2. Measure the client-consumed public surface — the exact set of names to
     publish so Schema is a usable shared spine node for ArgParse and Decoder —
     and confirm it is the minimal usable surface (no internal helper
     over-published; no unused export). Since there is no `export` line today,
     this is the set the added `export` line carries.
  3. Emit the ledger: the exact imports to adopt and the exact surface to
     publish. This is the deliverable D1 executes verbatim.
- **D0 HARD-STOPS to the Steward** on either of:
  - (a) an UNPUBLISHED PROVIDER — a symbol Schema needs whose owning module has
    NOT yet published it as a consumer surface (a hidden dep on a still-scaffolded
    module). Cite it and split it out; do NOT drag an unpublished provider in.
  - (b) an UNEXPECTED TIER-E EDGE — a dependency the DAG did not predict (any
    provider outside {Capability.Formatting.Doc, Data.Collections.Derived, the
    string_to_list_char owner, Data.Collections.NonEmpty, Data.Sums.Validation,
    prelude}). Cite it; the slice cut is wrong, do not bend scope to absorb it.
- **D1 — migrate + publish, executing the D0 ledger.** Replace the ambient/
  scaffold resolution with the exact D0-measured selective imports (retire the
  ambient reach for Doc/Text/list_append/string_to_list_char); ADD the `export`
  line publishing exactly the D0-measured usable consumer surface (nothing more);
  extend the module's loader-visible inventory to reflect both the new imports
  and the published surface (imported names are not new exports — reflect them
  the way an existing import is reflected); the module elaborates standalone
  (exit 0). No carrier change, no new proof content — every `data`/`fn`/`const`
  definition is preserved verbatim; `trusted_base()` unaffected.

## Acceptance criteria, each with its control

- **AC-IMPORTS-EXACT (the D0 ledger is adopted verbatim).** Schema's import block
  after D1 equals exactly the D0-measured selective-import set — same module
  paths, same name lists, no extra name and no ambient residue. Population from
  the module's own import block; verdict from the D0 ledger. Control: adding a
  name D0 did not measure, or dropping one it did, reds the equality distinctly;
  a per-import reddening mutation (delete one adopted name) reds standalone
  elaboration at the exact `UnresolvedCon`/`UnboundName` D0 recorded for that
  provider.
- **AC-NO-AMBIENT / STRICT-RESOLUTION (the scaffold reach is gone).** Schema
  elaborates standalone (exit 0) with NO whole-catalog class-install /
  scaffolding fallback in the resolution path — every provider symbol resolves
  through a real selective import or the prelude, measured by the loader.
  Control: removing the newly-adopted Doc (or Derived) import restores the exact
  prior standalone failure (the `UnresolvedCon`/`UnboundName` set D0 recorded for
  the formatting/list symbols) — the reddening proves the strict path, not the
  ambient one, now carries resolution.
- **AC-SURFACE-MINIMAL (only the usable consumer surface is published).** The
  set of loader-visible exports from Schema equals exactly the D0-measured usable
  consumer surface — each published name resolves to Schema's `GlobalId` via a
  selective import (measured by the loader, not a `^export` grep), and each name
  D0 ruled internal still rejects `UnboundName` from an external import. Control:
  a probe import of an intended export resolves; a probe import of a
  D0-internal sibling rejects `UnboundName`; a per-symbol reddening mutation
  (publish one extra, or private one intended) reds distinctly.
- **AC-INVENTORY-EXACT.** Schema's loader-visible inventory equality extends by
  EXACTLY the intended names (the published surface + the adopted imports;
  nothing else changes visibility). Population from the module's definitions,
  verdict from the loader, a per-symbol reddening mutation each reds distinctly.
- **AC-TRUST-INVARIANT (carrier + definitions preserved).** Every `data`/`fn`/
  `const` body is byte-unchanged; no new axiom/postulate/primitive/opaque
  constant is introduced. Control: a `trusted_base()` differential shows delta
  zero; a diff of the definition bodies shows BYTE-UNCHANGED (only the import
  block and the added `export` line differ).
- **AC-NO-REGRESSION.** Re-run the COMPLETE affected-target closure (every target
  loading Schema or a module whose closure this changes), scoped by changed
  PATHS. Targeted via `scripts/ken-cargo`, never `--workspace` (green in CI is
  the workspace verdict). If no catalog module imports Schema today, the
  external-consumer closure is expected empty — confirm that census rather than
  assume it.

## Hard stop

Route to the Steward if:

- D0 surfaces an UNPUBLISHED PROVIDER — a symbol Schema needs whose owning module
  has not yet published it (hidden dep on a still-scaffolded module). Split it
  out with a cited reason; do not drag the unpublished provider in; or
- D0 surfaces an UNEXPECTED TIER-E EDGE — a provider outside the predicted set
  {Capability.Formatting.Doc, Data.Collections.Derived, the string_to_list_char
  owner, Data.Collections.NonEmpty, Data.Sums.Validation, prelude}. The DAG cut
  is wrong; do not bend scope to absorb it; or
- delivering D1 appears to require a carrier change, new proof content, binding a
  new class/instance, or pulling in a downstream spine consumer (ArgParse,
  Decoder). Any of those means the slice cut is wrong, not that the scope should
  bend.

## Format any edited catalog file before handoff

Every file under `catalog/` must already be in canonical `kenfmt` form — there is
a corpus-wide formatter fixed-point gate (`kenfmt_c_capstone` /
`ken-cli/tests/ken_fmt.rs`) keyed on every catalog file. It fires only in a
full-workspace run, which is CI-only by operator hard rule, so a targeted
`-p ken-elaborator --test <yours>` cannot see it. Run the formatter on
`Schema.ken.md` before handing the SHA to QA; do NOT run `--workspace` locally to
check it.

## Tier: T2

Single-module D0-measured selective-import migration + surface publish — the
proven Tier-A / B / C / D / E-Json publish-and-import shape, no novel design. The
one judgment lives in the D0 measurement (is every ambient provider owned by an
already-published lower-tier module? which module owns string_to_list_char?) and
the two hard-stops; the D1 tail is mechanical (adopt the ledger's imports, add
the `export` line with the ledger's surface, extend the inventory,
standalone-green). No class-instance relocation, no proof authoring, no carrier
change.

## Gate, reviewer, sequencing

`gate: none` (no TCB touch; additive `catalog/`-only surface + import work). On
the candidate: **Foundation QA + CV** on the exact SHA, then Steward M1-M4 ->
lieutenant. **Architect** is required only if D0 fires a hard stop (unpublished
provider or unexpected Tier-E edge) — that is the only condition pulling an
Architect decomposition refresh; otherwise this is a mechanically completable
migration. Second Tier-E spine node; successor to the Tier-E Json entry node
(merged). The downstream spine consumers (`ArgParse`, `Decoder`) are framed one
release ahead after this lands, with fixed inputs re-measured at that SHA.

## Contention and sequencing

`catalog/` only, and additive. Re-measure contention at pickup, not from this
frame. The concurrent doc track touches `library/` and `agent/`, disjoint from
`catalog/`. The independent Tier-E algorithm leaves (`InsertionSort`,
`OrderedSearch`, `Gcd`, `Property`) are on their own axes and are not in flight
under this node. Foundation holds no other released node; this is the ring's
work.
