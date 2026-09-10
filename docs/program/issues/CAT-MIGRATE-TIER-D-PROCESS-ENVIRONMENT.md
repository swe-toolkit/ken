---
id: CAT-MIGRATE-TIER-D-PROCESS-ENVIRONMENT
title: "Scaffold-retirement provider publish: make Capability.Process.Environment a published consumer surface (pub + export the D0-measured names — process_environment and the surface Decoder consumes) so it can be imported selectively, mirroring the already-landed Capability.Process.Arguments precedent. Pure module over the landed ProcessInput ABI; no carrier/proof change, no TCB. Unblocks CAT-MIGRATE-TIER-E-DECODER."
status: merged
owner: foundation
size: S
gate: none
tier: T2
depends_on: []
blocks: [CAT-MIGRATE-TIER-E-DECODER]
github: null
origin: "Steward-framed 2026-09-10 as the predecessor discovered by CAT-MIGRATE-TIER-E-DECODER's D0 (foundation-implementer evt_18aqwyk476b4w, hard-stop (a)): Decoder consumes `process_environment`, owned by Capability.Process.Environment, which declares no `pub`/`export` at origin/main 710479b5d and so cannot be imported selectively. Steward ruling evt_155n7vtdndkm3: publish that provider FIRST. Grounded at 710479b5d: Capability.Process.Environment = catalog/packages/Capability/Process/Environment.ken.md (43 lines, PURE — process_environment / replace_process_environment + a round_trip proof over the landed ProcessInput ABI; only prelude/List/Prod/Bytes; trusted_base() delta zero). Its sibling Capability.Process.Arguments is already migrated + published with the identical shape (pub fn process_arguments, selective imports, pub proof round_trip)."
---

> # LANDED 2026-09-10 at dd7a4360c (squash of 593f86edc) — export-only publish.
>
> Merged via PR #3451, 26/26 CI green, blob-verified on main. What actually
> landed, per the frame-owner ruling evt_3aewjkc8p13ba: the surface is published
> with `export process_environment` ONLY — NOT `pub` and NOT both. D0
> (evt_44kzse3fkdqx9) measured the exact one-name consumer surface
> `{process_environment}` (Decoder is the only client; `replace_process_environment`
> and the `round_trip` proof have no catalog consumer, so they stay unpublished),
> a prelude-only provider closure (empty import ledger), and confirmed that
> `pub`-only and `export`-only each publish the same canonical identity while
> their CONJUNCTION is unrepresentable (duplicate public target -> UnboundName).
> The `export`-only spelling matches the explicit-surface Tier-E migration cohort
> (Schema/ArgParse/JSON) that Decoder belongs to. Bodies byte-unchanged,
> `trusted_base()` delta zero (CV-verified). This unblocked
> CAT-MIGRATE-TIER-E-DECODER, re-released the same day. The "add pub + export
> line" phrasing in the deliverables/ACs below is SUPERSEDED by this banner —
> read it as export-only.

> # RELEASED 2026-09-10 (Steward) — provider publish unblocking the last spine node.
>
> The predecessor CAT-MIGRATE-TIER-E-DECODER's D0 surfaced (hard-stop (a)):
> Decoder consumes `process_environment` from the UNPUBLISHED
> `Capability.Process.Environment`. This node publishes that surface so Decoder
> can import it selectively. It is a near-trivial publish — the module is a
> 43-line PURE view of the landed `ProcessInput` ABI (`process_environment`,
> `replace_process_environment`, a `round_trip` proof), with a prelude-only free
> closure — the same mechanical shape as its already-landed sibling
> `Capability.Process.Arguments`. No carrier change, no new proof content, no
> TCB (`trusted_base()` delta zero). On the candidate: Foundation QA + CV on the
> exact SHA, then Steward M1-M4 -> lieutenant. Architect only if D0 fires a hard
> stop.

## Not a regression fix

`Capability.Process.Environment` elaborates today in the full-catalog build; it
simply does not publish a consumer surface (no `pub`/`export`), so a downstream
client (Decoder) cannot import its names selectively and falls back to ambient
resolution. This node retires that gap by publishing the D0-measured usable
surface — a scaffold-retirement provider publish, not a new build and not a
defect on `main`.

## Measured surface (origin/main 710479b5d — carry, D0 re-measures at pickup)

Grounded from the module text (43 lines), for orientation only — D0 re-measures
via the loader at the pickup SHA:

- **Free-symbol closure: prelude / landed-ABI only.** The bodies reference only
  `ProcessInput`/`MkProcessInput` (the landed ABI datatype), `List`, `Prod`,
  `Bytes`, `Equal`, `Refl` — all prelude/built-in, as the sibling
  `Capability.Process.Arguments` confirmed (its ProcessInput use needs no
  import). So the migration likely adopts NO new import at all (D0 confirms; if
  D0 measures any free symbol owned by a published module, adopt exactly that
  selective import — hard-stop if it is unpublished or outside the predicted set
  {prelude, and any provider the landed Arguments already imports:
  Capability.Parsing.Cursor, Core.Classes.LawfulClasses, Data.Collections.Derived}).
- **Declared public API: none today** (no `pub`, no `export`). The migration
  ADDS an `export` line carrying exactly the D0-measured usable consumer surface
  (export-only, NOT `pub` — the conjunction is unrepresentable; ruled
  evt_3aewjkc8p13ba, see the LANDED banner). Decoder consumes `process_environment`
  at minimum; D0 measures the exact set (it may include
  `replace_process_environment` and the `round_trip` proof if a client consumes
  them — publish exactly what is consumed, no internal helper over-published).

## Deliverables (D0-first)

- **D0 — measure the (near-empty) provider closure + the consumer surface.**
  Emit the exact selective-import ledger (expected: none, or a subset of what
  Arguments imports) and the exact names to publish (the surface Decoder and any
  other client consumes). Hard-stop to the Steward on an unpublished provider or
  an unexpected edge outside the predicted set above.
- **D1 — publish, executing the D0 ledger.** Add the `export` line with exactly
  the D0-measured surface (export-only — do NOT also add `pub`; ruled
  evt_3aewjkc8p13ba); adopt any D0-measured selective import; extend the
  loader-visible inventory. The
  module elaborates standalone (exit 0). No carrier change, no new proof content —
  every `data`/`fn`/`const`/`proof` body byte-unchanged; `trusted_base()`
  unaffected.

## Acceptance criteria, each with its control

- **AC-SURFACE-EXACT.** The set of loader-visible exports equals exactly the
  D0-measured usable consumer surface — each published name resolves to this
  module's `GlobalId` (measured by the loader, not an `^export` grep); a name D0
  ruled internal rejects `UnboundName` from an external import. Control: a probe
  import of an intended export resolves; a probe import of a D0-internal sibling
  rejects `UnboundName`; a per-symbol reddening mutation (publish one extra, or
  private one intended) reds distinctly.
- **AC-CLIENT-USABLE (the point of the node).** A probe selective import
  `import Capability.Process.Environment (process_environment)` from an external
  module resolves to this module's `GlobalId` — the exact edge Decoder needs.
  Control: before the `pub`/`export`, that probe import rejects (the current
  standalone failure); after, it resolves.
- **AC-TRUST-INVARIANT.** Every `fn`/`proof`/`data` body is byte-unchanged; no
  new axiom/postulate/primitive/opaque constant. Control: `trusted_base()`
  differential delta zero; a body diff shows BYTE-UNCHANGED (only `pub` markers
  and the added `export` line differ).
- **AC-NO-REGRESSION.** Re-run the COMPLETE affected-target closure (every target
  loading this module or a module whose closure this changes), scoped by changed
  PATHS, targeted via `scripts/ken-cargo`, never `--workspace` (green in CI is
  the workspace verdict). Retain every existing test unweakened.

## Hard stop

Route to the Steward if D0 surfaces an unpublished provider, an unexpected edge
outside the predicted set above, or D1 appears to require a carrier change, new
proof content, or binding a new class/instance. Any of those means the shape is
not the expected pure Arguments-parallel publish.

## Format before handoff

Run `kenfmt` on `Environment.ken.md` before the SHA handoff (the corpus formatter
fixed-point gate is CI-only). Do NOT run `--workspace` locally.

## Tier: T2

A pure-module surface publish mirroring the landed `Capability.Process.Arguments`
— no novel design, no carrier/proof change, prelude-only closure. The only
judgment is the D0 measurement of the exact consumed surface and the (expected
empty) import ledger.

## Gate, reviewer, sequencing

`gate: none` (no TCB; additive `catalog/`-only publish). Foundation QA + CV on the
exact SHA, then Steward M1-M4 -> lieutenant. Architect only on a D0 hard stop.
`blocks` CAT-MIGRATE-TIER-E-DECODER — on this node's landing, the Steward
re-releases Decoder (its D0 resumes from the same point; the LawfulClasses ruling
stands). `catalog/` only and additive; re-measure contention at pickup.
