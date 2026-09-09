---
id: CAT-MIGRATE-TIER-D-RENDER
title: "Scaffold-retirement Tier D (Capability), independent singleton: migrate Diagnostics.Render off ambient resolution onto real selective imports from the already-published Diagnostics.Core and Formatting.Doc providers, so it elaborates standalone. Publish the module's own client-consumed export surface, replace whole-catalog ambient resolution with a real selective import block, extend the loader-visible inventory, standalone-green. The proven Tier-A/B/C/D publish-and-import shape; NO class-instance relocation, NO proof authoring. D0 confirms (via the loader, not a read) the exact free-symbol closure per provider and which of the three module functions are client-consumed vs private helpers."
status: merged
owner: foundation
size: S
gate: none
tier: T2
depends_on: [CAT-MIGRATE-TIER-D-CURSOR]
blocks: []
github: null
origin: "Steward, 2026-09-08, framed on foundation-leader's next-slice proposal evt_3dj8hwn9j4f9s (accepted evt_6zj25zsx57mbw) once the Tier-D Parsing group closed (Numeric + Parsing.Parsing + Process.Arguments all merged, group node terminal at ccdd698bb). Diagnostics.Render is an INDEPENDENT Tier-D singleton in the Architect decomposition evt_2e0pee5jxzv07 (internal order '... Numeric / Parsing / Process.Arguments; Diagnostics.Render; Filesystem.Path.Posix; System.IO erratum'); its two providers Diagnostics.Core and Formatting.Doc were published by CAT-MIGRATE-TIER-D-CURSOR (merged), so the 'providers must be published' hold is discharged. Module identity GROUNDED at origin/main ccdd698bb: Render = catalog/packages/Capability/Diagnostics/Render.ken.md (blob e95740c8b5a7f1e72888c9a284988f6fc259b134, 39 lines). Fixed inputs measured at ccdd698bb; the ring re-measures at its pickup SHA via the loader (this frame's counts are the a-priori D0 confirms, not a hand-fixed import set)."
---

> # Scaffold-retirement Tier D: the Diagnostics.Render singleton.
> # Single-module publication + import clean-ification
> # (the Tier-A/B/C/D shape): publish own client-consumed
> # surface, repoint ambient consumption to the already-published
> # Diagnostics.Core + Formatting.Doc, extend loader inventory,
> # standalone-green. NO class-instance relocation, NO proof
> # authoring.
>
> Render resolves every provider symbol ambiently (via the
> whole-catalog class-install / scaffolding fallback) and today
> carries NO import line at all, so it fails standalone elaboration.
> This slice gives it a real selective import from the
> already-published lower tiers plus its two published Capability
> providers, so it elaborates on its own. Every provider it imports
> is ALREADY PUBLISHED (Diagnostics.Core and Formatting.Doc landed
> in the Cursor slice) — no consumer-only increment references an
> unpublished provider (the DAG axis).

## Not a regression fix (read before treating standalone-red as a bug)

Render elaborates TODAY in the full-catalog build via ambient
class-install (the operator's class-uniformity ruling, 2026-09-02).
Nothing is on fire. This is a standalone-CLEANNESS quality slice bringing
the module to the scaffold-retirement end state (`zero catalog dependence
on fixture scaffolding / ambient resolution`,
[[CAT-SCAFFOLD-RETIREMENT]]). "Render fails standalone at
UnresolvedCon/UnboundName Y" is the STARTING condition this increment
closes, not a defect on `main`.

## Fixed inputs — measured at ccdd698bb (a-priori; D0 loader-confirms)

`catalog/packages/Capability/Diagnostics/Render.ken.md` (blob e95740c8b5a7,
39 lines) has exactly three top-level `fn` declarations in one `ken` block, all
bare (none `pub`), and NO `import` line:

1. `diagnostic_to_doc` — the client-consumed public API (measured consumers below
   call this name and only this name).
2. `diagnostic_code_string` — private rendering helper, used only inside
   `diagnostic_to_doc`.
3. `diagnostic_origin_label` — private rendering helper, used only inside
   `diagnostic_to_doc`.

A-priori free-symbol closure (the loader is the authority; D0 confirms):

- **Capability.Diagnostics.Core** (published by the Cursor slice) — a-priori
  10 names: `DiagnosticCode`, `MkDiagnosticCode`, `Origin`, `SourceOrigin`,
  `ArgumentOrigin`, `EnvironmentOrigin`, `ConfigKeyOrigin`, `Diagnostic`,
  `diagnostic_origin`, `diagnostic_code`.
- **Capability.Formatting.Doc** (published by the Cursor slice) — a-priori 5
  names: `Doc`, `Group`, `Concat`, `Line`, `text_string`.
- **`String`** is a prelude-floor builtin, NOT a selective-import target —
  it stays in the ambient-conveniences census bucket, not the strict-import
  block.
- NO LawfulClasses / Derived / Nat / List / Bytes edge (Render does no
  arithmetic or list operations itself).

So the a-priori import block is exactly two `import` statements. This is the
a-priori to CONFIRM, not a hand-fixed set: the EC "exact-N" census once
under-counted, so D0 measures the free-symbol closure via the loader and corrects
these counts if the loader disagrees.

## Design judgment (front-loaded)

1. **Publish exactly the client-consumed surface — `diagnostic_to_doc` only.**
   The measured consumers (Forge example, cc7, cc8) call `diagnostic_to_doc`
   and nothing else; `diagnostic_code_string` and `diagnostic_origin_label`
   are internal rendering helpers. Publish `diagnostic_to_doc`; keep the two
   helpers PRIVATE, proven by an `assert_private` control (mirrors
   Process.Arguments' private `argument_bytes_at` and Numeric's private
   implementation boundary). Measure, do not guess: if D0 finds any consumer
   that references a helper, publish that helper too and record why — but the
   measured evidence is single-operation.
2. **No usability AC is load-bearing.** Render exports only a function and
   owns no abstract type; every type across its public boundary — `Diagnostic`
   (client constructs via Diagnostics.Core's exported constructors), `Doc`
   (via Formatting.Doc's exported `Doc`/`Text`/`Line`/`Concat`/`Group`), and
   the concrete `String` — is client-constructible through an
   already-published provider. There is no inert-type-export /
   rejected-surface case here (same shape as Process.Arguments).
3. **Fully-ambient start ⇒ the standalone reddening control is clean.**
   Render has no import line today, so the AC-STANDALONE-GREEN control
   (remove the new import block ⇒ the exact prior standalone
   `UnresolvedCon`/`UnboundName` failure returns) applies without the Tier-C
   already-green no-op caveat.

## Deliverables

- **D0 — confirm the surface + closure at the pickup SHA (T1-adjacent judgment;
  Architect is the confirmer).** Via the loader (the authority), measure
  Render's standalone `UnresolvedCon`/`UnboundName` set and the exact
  provider name set it needs, and confirm the client-consumed surface (which
  of the three functions any consumer references). Emit the exact publish set
  and the exact per-provider import sets. If D0 surfaces a provider NOT owned
  by an already-published module (a hidden dep on a still-scaffolded module),
  CITE it and HARD STOP rather than dragging an unpublished provider in (the
  DAG axis every tier respects).
- **D1 — publish + import + standalone.** Publish exactly the D0-confirmed
  export surface (a-priori `diagnostic_to_doc`); add the selective import
  block from Diagnostics.Core and Formatting.Doc for the exact D0-confirmed
  name sets; retire the ambient reach; extend the module's loader-visible
  inventory to reflect the new export and the imports (imported names are not
  new exports — reflect them the way an existing import is reflected); the
  module elaborates standalone (exit 0).
- **D2 — consumer + census closure (AC-AFFECTED-CLOSURE).** Update every
  target that loads Render or a module whose closure this changes:
  - `crates/ken-elaborator/tests/lang_mod_strict_resolution_d0.rs`: remove
    `"Capability.Diagnostics.Render"` from `expected_residuals` (currently
    line ~1029, baseline-red) AND add a keyed entry to the `census`/`expected`
    map with ONLY its prelude-floor conveniences (mirror the Diagnostics.Core
    / Formatting.Doc / Cursor entries and their explanatory comment). Both
    sentinels (`census == expected`, `residual_names == expected_residuals`)
    enforce that the two edits land together.
  - `crates/ken-elaborator/tests/cc7_argparse_acceptance.rs` and
    `cc8_env_config_decoder_acceptance.rs`: adapt Render's load path (ambient
    `elaborate_ken_md_file` → roots-load + expose, the Parsing.Parsing / cc
    precedent) so the strict imports resolve by module name; acceptance
    bodies unchanged.
  - `catalog/examples/CommandLine/Forge.ken.md` (consumes `diagnostic_to_doc`
    ambiently at :54): measure at pickup whether it needs a strict import
    added or remains a full-catalog ambient consumer; do NOT edit it unless
    the loader shows it must.
- **D3 — the singleton D0 test fixture.** Mirror the SINGLETON template
  `crates/ken-elaborator/tests/cat_tier_d_cursor_import.rs` (two tests per
  module), NOT the group test. Add a Render block (a new
  `cat_tier_d_render_import.rs` or an added module block, the ring's call)
  with the shared helpers `load_module`, `assert_selective_identities`,
  `assert_providers_consumed`, `assert_private`, and
  `published_module_surfaces`.

This slice authors NO new proof content and performs NO class-instance relocation.

## Acceptance criteria, each with its control (proven Tier-A/B/C/D shape)

- **AC-EXPORTED (positive).** `diagnostic_to_doc` is LOADER-VISIBLE from
  Render — a selective import resolves it to Render's `GlobalId`, measured
  by the loader, not a `pub` grep. Control: the probe resolves; a
  still-private helper name in the same module still rejects `UnboundName`.
- **AC-PRIVATE (negative, load-bearing here).** `diagnostic_code_string` and
  `diagnostic_origin_label` stay private: `import Render
  (diagnostic_code_string)` fails with `ElabError::UnboundName { name ==
  "...Render.diagnostic_code_string" }` (and likewise for the label helper).
  Control: the exact `UnboundName` on each; a mutation that publishes a
  helper reds this test.
- **AC-EXACT-INVENTORY.** Render's loader-visible inventory equality extends
  by EXACTLY the intended names (the published export(s) + the imported
  names; nothing else changes visibility); population from the module's own
  definitions, verdict from the loader; `assert_selective_identities`
  re-imports the whole surface in one client and asserts no `GlobalId` is
  reminted (per-name identity preservation); a per-symbol reddening mutation
  reds distinctly.
- **AC-PROVIDERS-CONSUMED.** `assert_providers_consumed(Render,
  [Capability.Diagnostics.Core, Capability.Formatting.Doc])` walks each
  owned Transparent decl's ty/body and asserts every declared provider
  identity (the D0-confirmed 10 Core + 5 Doc names) is actually mentioned —
  no over-import (an imported-but-unused name) and no under-import (a
  used-but-unimported name). Control: dropping any imported name reds either
  standalone elaboration or this consumed-set check.
- **AC-STANDALONE-GREEN.** Render elaborates standalone (exit 0) after its
  import block — no ambient/scaffolding fallback. Control: removing the new
  import block restores the exact prior standalone failure D0 recorded.
- **AC-VISIBILITY-ONLY.** Every `pub` added changes visibility only: a
  differential shows the body BYTE-UNCHANGED; publishing mints no
  class/instance; `trusted_base` delta zero; every consumer resolves to the
  single existing owner. No computational change, no proof authored.
- **AC-NO-REGRESSION / AFFECTED-CLOSURE.** Re-run the COMPLETE
  affected-target closure — every target loading Render or a module whose
  closure this changes (the census fixture, cc7, cc8, the new D0 fixture, any
  Forge-elaborating target) — scoped by changed PATHS via `scripts/ken-cargo`,
  never `--workspace` (green in CI is the workspace verdict).

## Format any edited catalog file before handoff

`catalog/packages/Capability/Diagnostics/Render.ken.md` must be in canonical
`kenfmt` form — the corpus-wide formatter fixed-point gate
(`kenfmt_c_capstone` / `ken-cli/tests/ken_fmt.rs`) fires only in a
full-workspace run (CI-only by operator hard rule), so a targeted `-p
ken-elaborator --test <yours>` cannot see it. Run the formatter on the edited
catalog file before handing the SHA to QA; do NOT run `--workspace` locally
to check it.

## Capability tier: T2 (with a T1-adjacent D0)

Single-module mechanical export publication + a two-import selective-import
block (the proven Tier-A/B/C/D shape) — no class-instance relocation, no
proof authoring. The one judgment is D0 completeness (are all ambient deps
owned by the already-published Diagnostics.Core / Formatting.Doc, or is
there a hidden non-published provider?) plus confirming the client-consumed
surface. The mechanical D1..D3 tail is the over-provisioned part;
foundation-leader verifies the seat against this estimate before pickup
(their request).

## Gate, reviewer, sequencing

`gate: none` (no TCB touch; the operator ruled the class-owner model). On the
candidate: **Architect** (required — surface correctness + class-uniformity +
D0 completeness) + **Foundation QA + CV** on the exact SHA, then Steward
M1-M4 → lieutenant. An independent Tier-D singleton; the next Tier-D step
(Filesystem.Path.Posix, then the System.IO theorem-name erratum) is framed
one release ahead after this lands, with fixed inputs re-measured at that
SHA.

## Contention and sequencing

`catalog/` (one module) plus the elaborator test fixtures, and additive.
Re-measured at ccdd698bb: no commit or `wp/` branch touches Render's
migration, and the shared fixtures (`lang_mod_strict_resolution_d0.rs`, the
`cat_tier_d_*_import.rs` family, cc7/cc8) carry only already-merged Tier-D
edits baked into ccdd698bb — no live branch holds uncommitted edits to them.
Re-measure at pickup, not from this frame. Lane 1 (runtime,
`ken-host`/`ken-runtime`) and lane 2 (language, the FO/Kripke prover in
`ken-elaborator`) are disjoint from `catalog/` and from these Tier-D census
fixtures. The concurrent doc track touches `library/` and `agent/`, disjoint.
Foundation holds no other released node; this is the ring's work.

## Hard stop

Route to the Steward if:

- Render CANNOT be brought standalone-green through a selective import from
  already-published providers — i.e. D0 surfaces a required provider owned
  by a still-scaffolded, unpublished module (a hidden cross-tier edge; cite
  it, do not drag the unpublished provider in); or
- delivering D1..D3 appears to require binding a new class/instance, authoring
  any proof content, or the System.IO erratum. Any of those means the slice
  cut is wrong, not that the scope should bend.
