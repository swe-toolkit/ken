---
id: CAT-MIGRATE-TIER-D-POSIX
title: "Scaffold-retirement Tier D (Capability), independent singleton: migrate Filesystem.Path.Posix off ambient resolution onto real selective imports from the already-published Core.Logic.Compare, Core.Classes.LawfulClasses, and Data.Collections.Derived providers, so it elaborates standalone. Publish the module's client-consumed export surface, replace whole-catalog ambient resolution with a real selective import block, extend the loader-visible inventory, standalone-green. The proven Tier-A/B/C/D publish-and-import shape; NO class-instance relocation, NO proof authoring. D0 DETERMINES the surface (there is no external importer) via the loader: the exact free-symbol closure per provider and which declarations the sole de-facto client (the cc6b acceptance fixture) consumes vs which are internal plumbing/lemmas."
status: active
owner: foundation
size: M
gate: none
tier: T2
depends_on: [CAT-MIGRATE-TIER-D-RENDER]
blocks: []
github: null
origin: "Steward, 2026-09-09, framed on foundation-leader's next-slice proposal evt_2tb3y9aweqt21 (accepted evt_6a8dmqvfkx30m) once the Diagnostics.Render singleton closed (merged ae621d61, node terminal at 6dd88d3fb). Filesystem.Path.Posix is the next INDEPENDENT Tier-D singleton in the Architect decomposition evt_2e0pee5jxzv07 (internal order '... Diagnostics.Render; Filesystem.Path.Posix; System.IO erratum'); its providers Core.Logic.Compare / Core.Classes.LawfulClasses / Data.Collections.Derived were all published in earlier tiers (the 'providers must be published' hold is discharged — verified below). depends_on names CAT-MIGRATE-TIER-D-RENDER as the SEQUENCING predecessor (shared census fixture lang_mod_strict_resolution_d0.rs, one-WP-at-a-time lane), NOT a publication dependency: Posix's real providers are Compare/LawfulClasses/Derived, all long-landed. Module identity GROUNDED at origin/main 6dd88d3fb: Posix = catalog/packages/Capability/Filesystem/Path/Posix.ken.md (blob 39be9eaf5823f8ba01ed728a626826de93ff19d1, 1793 lines). Fixed inputs measured at 6dd88d3fb via the loader-facing fixtures; the ring re-measures at its pickup SHA via the loader (this frame's counts are the a-priori D0 confirms, not a hand-fixed import set). The System.IO theorem-name erratum stays a SEPARATE, LATER node."
---

> # Scaffold-retirement Tier D: the Filesystem.Path.Posix singleton.
> # Single-module publication + import clean-ification
> # (the Tier-A/B/C/D shape): publish own client-consumed
> # surface, repoint ambient consumption to the already-published
> # Core.Logic.Compare + Core.Classes.LawfulClasses +
> # Data.Collections.Derived, extend loader inventory,
> # standalone-green. NO class-instance relocation, NO proof
> # authoring.
>
> Posix resolves every provider symbol ambiently (via the
> whole-catalog class-install / scaffolding fallback) and today
> carries NO import line at all, so it fails standalone (baseline)
> elaboration. This slice gives it a real selective import from the
> three already-published provider modules, so it elaborates on its
> own. Every provider it imports is ALREADY PUBLISHED (Compare,
> LawfulClasses, and Derived landed in earlier tiers and are strict-
> imported today by peer modules) — no consumer-only increment
> references an unpublished provider (the DAG axis).

## Not a regression fix (read before treating standalone-red as a bug)

Posix elaborates TODAY in the full-catalog build via ambient class-install
(the operator's class-uniformity ruling, 2026-09-02). Nothing is on fire. This
is a standalone-CLEANNESS quality slice bringing the module to the
scaffold-retirement end state (`zero catalog dependence on fixture scaffolding
/ ambient resolution`, [[CAT-SCAFFOLD-RETIREMENT]]). "Posix fails standalone at
UnresolvedCon/UnboundName Y" is the STARTING condition this increment closes,
not a defect on `main`.

## This module is NOT small — but the migration is still visibility-only

Unlike Render (39 lines, 3 functions), Posix is 1793 lines: 3 `data`, 4
`const`, 39 `fn`, 2 module-owned `proof … for`, and 74 internal `theorem`
lemmas. The SIZE is in the proof-dense body, all of which is BYTE-UNCHANGED by
this slice. The migration touches only visibility markers, the new import
block, the inventory, and the consumer/census fixtures. The two things that
make this M rather than S are (a) D0 must DETERMINE the public surface (there
is no external importer to read it off — see below), and (b) the sole client
fixture cc6b carries a delicate hand-built dependency pre-seed that must be
repointed. Neither authors a proof or moves a class.

## Fixed inputs — measured at 6dd88d3fb (a-priori; D0 loader-confirms)

`catalog/packages/Capability/Filesystem/Path/Posix.ken.md` (blob 39be9eaf5823,
1793 lines) has NO `import` line and NO `pub` marker on any declaration; it
declares NO class/instance/postulate/opaque/Axiom (its own §4 states
`trusted_base()` delta zero). Top-level declarations: 3 `data` (`Path` with
ctor `MkPath`, `PathOrdinarySegment`, `PathNormalForm`), 4 `const`, 39 `fn`, 2
module-owned `proof … for path_ordinary_bytes_of`, 74 `theorem`.

A-priori free-symbol closure — the loader is the authority; D0 confirms — splits
into exactly THREE selective-import providers plus native/prelude-floor residual:

- **`Core.Logic.Compare`** — a-priori 1 name: `list_eq` (used at :19). Already
  `pub` (Compare.ken.md:369) and already strict-imported today by
  `Data.Collections.Derived`.
- **`Core.Classes.LawfulClasses`** — a-priori: `uint8_deceq_eq` (:2255),
  `bool_and` (:641), and the `DecEq` class (:77) so the `UInt8` instance
  installs (instances are never `pub`-marked; the established BytesKeys /
  StringKeys / EmptyDec precedent imports the CLASS and relies on the instance
  riding in when LawfulClasses loads). Posix's `(DecEq_instance_UInt8).eq`
  reference resolves through the class import. All three `pub` and widely
  strict-imported today.
- **`Data.Collections.Derived`** — a-priori 1 name: `list_append` (:77), `pub`
  and strict-imported today by Formatting.Doc.
- **Attached proofs are a D0 spelling question, not a new provider.** Posix
  invokes `(proof intro|left|right for bool_and)` (owned by LawfulClasses) and
  `(proof assoc|right_unit for list_append)` (owned by Derived). Whether these
  attached proofs must be NAMED in the import stanza or ride with their subject
  function is exactly what D0 measures via the loader (the landed
  CAT-LAWFULFUNCTORS-STANDALONE-IMPORT migration of `list_append` attached
  proofs to Derived is the nearest precedent — consult it, do not guess).

**Correction to the a-priori provider claim (record it — do not import these).**
The proposal that Posix pulls "lower-tier `Bytes` / `UInt8` / `List` and
list/bytes conversion operations" as IMPORT TARGETS is wrong: `Bytes`, `UInt8`,
`List` (and its `Nil`/`Cons`), `Nat`, `Bool`, and the byte conversions
`bytes_to_list` / `list_to_bytes` / `list_bytes_roundtrip`, plus `map`, are
NATIVE PRELUDE / ten-type-floor builtins (declared in `ken-elaborator/src/
bytes.rs` and `modules.rs`), NOT catalog modules. They CANNOT and MUST NOT
appear in the import block — they stay AMBIENT (census residual). The only
confirmed lower-tier catalog edge is `DecEq UInt8` (LawfulClasses, Tier-B). The
real import set is exactly the three modules above.

So the a-priori import block is exactly three `import` statements. This is the
a-priori to CONFIRM, not a hand-fixed set: D0 measures the free-symbol closure
via the loader and corrects these names if the loader disagrees.

## Design judgment (front-loaded)

1. **D0 DETERMINES the public surface, because there is no external importer.**
   A current-main census finds NO catalog module or example that imports Posix
   or calls any `path_*` function; the ONLY consumer of its surface is the
   acceptance fixture `crates/ken-elaborator/tests/cc6b_path_posix_acceptance.rs`.
   So there is no assumed public API to preserve — the migration MUST derive the
   surface. The principled rule: **publish exactly what the sole de-facto client
   (cc6b, once switched to a real module load) resolves, and keep everything
   else private.** The a-priori client-consumed set (measured from cc6b's calls
   and theorem references, §1 of the measurement): the type `Path`/`MkPath` and
   the operations `path_parse`, `path_render`, `path_join`, `path_parent`,
   `path_normalize`, `path_valid`, `path_is_absolute` — plus any
   normalization/roundtrip theorem cc6b resolves by name
   (`path_parse_render_valid`, `path_parse_valid`, `path_parse_render_parse`,
   `path_normalize_idempotent`, `path_normalize_has_no_dot`,
   `path_normalize_absolute_has_no_dotdot`). Everything else — segment
   predicates, the cons-result splitters, the forget/normal-form plumbing, all
   74 lemmas not named by cc6b, and the dead `path_list_tail` (:1495, never
   referenced) — stays PRIVATE. D0 confirms the EXACT set via the loader.
2. **Publishing theorems is a surface question for the Architect, not a
   default.** Prior Tier-D slices published functions and types, not theorems.
   If D0 finds cc6b resolves theorem NAMES (not just re-checks them), publishing
   those theorems is required to keep cc6b green under a strict load — but
   whether a normalization lemma is genuinely part of Posix's public API, or
   whether cc6b should instead re-derive/inline it, is an Architect call. Do not
   publish the whole 74-lemma set to be safe; publish the minimal cc6b-resolved
   set and let the Architect confirm each published theorem earns its surface.
   If the minimal set is large or looks wrong as an API, HARD STOP to the
   Steward rather than freezing a large lemma surface by reflex.
3. **No usability AC is load-bearing for a new abstract type.** Posix's `Path`
   is a transparent record (`MkPath {path_absolute, path_segments}`) constructed
   from native `Bool` / `List (List UInt8)`; it exports no abstract type with a
   representation invariant, so there is no inert-type-export / rejected-surface
   earns-keep question (same shape as Process.Arguments / Render). The usability
   check is the ordinary one: a strict client can import a published operation
   and actually use it (D3's usability test), not an abstract-constructor-path
   audit.
4. **Fully-ambient start ⇒ the standalone reddening control is clean.** Posix
   has no import line today and fails baseline elaboration outright, so the
   AC-STANDALONE-GREEN control (remove the new import block ⇒ the exact prior
   baseline `UnresolvedCon`/`UnboundName` failure returns) applies without the
   Tier-C already-green no-op caveat.

## Deliverables

- **D0 — determine the surface + confirm the closure at the pickup SHA
  (T1-adjacent judgment; Architect is the confirmer).** Via the loader (the
  authority), measure Posix's standalone baseline failure and the exact provider
  name set it needs (a-priori: `Compare (list_eq)` + `LawfulClasses (DecEq,
  uint8_deceq_eq, bool_and)` + `Derived (list_append)`, plus the attached-proof
  spelling), and DETERMINE the client-consumed public surface by measuring
  exactly which declarations cc6b resolves once it loads Posix as a module.
  Emit the exact publish set and the exact per-provider import sets. Settle the
  attached-proof import spelling against the CAT-LAWFULFUNCTORS-STANDALONE-IMPORT
  precedent. If D0 surfaces a provider NOT owned by an already-published module
  (a hidden dep on a still-scaffolded module), CITE it and HARD STOP rather than
  dragging an unpublished provider in (the DAG axis every tier respects).
- **D1 — publish + import + standalone.** Publish exactly the D0-determined
  export surface with `pub` markers (body BYTE-UNCHANGED); add the selective
  import block from Compare, LawfulClasses, and Derived for the exact
  D0-confirmed name sets; retire the ambient reach; extend the module's
  loader-visible inventory to reflect the new exports and the imports (imported
  names are not new exports — reflect them the way an existing import is
  reflected); the module elaborates standalone (exit 0). Native builtins
  (`Bytes`/`UInt8`/`List`/`Nat`/`Bool`/`map`/byte-conversions) stay ambient —
  never in the import block.
- **D2 — consumer + census closure (AC-AFFECTED-CLOSURE).** Update every target
  that loads Posix or a module whose closure this changes:
  - `crates/ken-elaborator/tests/lang_mod_strict_resolution_d0.rs`: remove
    `"Capability.Filesystem.Path.Posix"` from `expected_residuals` (currently
    line ~1055, baseline-red) AND add a keyed entry to the `census`/`expected`
    map (alphabetical slot after `Capability.Filesystem.Errors`) with ONLY its
    prelude-floor conveniences — a-priori `["Equal", "Proved", "map"]` (the D0
    loader computes the exact vector; mirror the Render / Cursor entries and
    their explanatory comment). Both sentinels (`census == expected`,
    `residual_names == expected_residuals`) enforce that the two edits land
    together.
  - `crates/ken-elaborator/tests/cc6b_path_posix_acceptance.rs` (the sole client,
    the delicate one): today it loads Posix via `elaborate_ken_md_file` after a
    HAND-BUILT `dependency_env()` that manually pre-loads and re-exposes Compare
    / Transport / LawfulClasses / BytesKeys / a Derived `list_append` fixture,
    including the `withhold_lc_bool_and_flat_aliases` / `restore_…` dance
    (:16-38). Once Posix carries a real selective-import block, repoint this to a
    roots-load (the `load_module` / Parsing.Parsing / cc precedent) so the
    module's own imports resolve, and RETIRE the manual pre-seed and the
    withhold/restore dance that the strict imports make unnecessary. Acceptance
    bodies (the path_* behavior + theorem checks) unchanged. CONTENTION: two
    live CV branches touch this fixture (see Contention) — re-measure and
    coordinate before editing it.
- **D3 — the singleton D0 test fixture.** Mirror the SINGLETON template
  `crates/ken-elaborator/tests/cat_tier_d_cursor_import.rs` / `..._render_import.rs`
  (two tests per module, each with a MEASURED/CLAIMED/GAP promise-class
  docblock), NOT the group test. Add `cat_tier_d_posix_import.rs` using the
  shared support (`support/catalog_or.rs::catalog_root`,
  `support/catalog_publication.rs::published_module_surfaces`) and a local
  `provider_modules(Posix) = ["Core.Logic.Compare", "Core.Classes.LawfulClasses",
  "Data.Collections.Derived"]`, plus the local helpers the template uses
  (`load_module`, `selective_imports`, `assert_providers_consumed`,
  `assert_private`). Two tests: (1) loader-visible inventory is exact and usable
  (a strict client imports a published op and uses it, identity not reminted);
  (2) imports are exact/canonical and helpers private.

This slice authors NO new proof content and performs NO class-instance relocation.

## Acceptance criteria, each with its control (proven Tier-A/B/C/D shape)

- **AC-EXPORTED (positive).** Each D0-determined public name (a-priori
  `path_parse`, `path_render`, `path_join`, `path_parent`, `path_normalize`,
  `path_valid`, `path_is_absolute`, the `Path` type, and any cc6b-resolved
  theorem) is LOADER-VISIBLE from Posix — a selective import resolves it to
  Posix's `GlobalId`, measured by the loader, not a `pub` grep. Control: the
  probe resolves; a still-private helper name in the same module (e.g.
  `path_finish_segment` or the dead `path_list_tail`) still rejects
  `UnboundName`.
- **AC-PRIVATE (negative, load-bearing here).** The internal plumbing stays
  private: importing a private helper name (a segment predicate, a cons-result
  splitter, a forget/normal-form helper, or a lemma NOT in the D0 public set)
  fails with `ElabError::UnboundName { name == "...Posix.<helper>" }`. Control:
  the exact `UnboundName` on at least two representative private names; a
  mutation that publishes one reds this test.
- **AC-EXACT-INVENTORY.** Posix's loader-visible inventory equality extends by
  EXACTLY the intended names (the published exports + the imported names; nothing
  else changes visibility); population from the module's own definitions, verdict
  from the loader; the whole surface re-imported in one client asserts no
  `GlobalId` is reminted (per-name identity preservation); a per-symbol
  reddening mutation reds distinctly.
- **AC-PROVIDERS-CONSUMED.** `assert_providers_consumed(Posix, [Core.Logic.Compare,
  Core.Classes.LawfulClasses, Data.Collections.Derived])` walks each owned
  Transparent decl's ty/body and asserts every declared provider identity (the
  D0-confirmed name sets) is actually mentioned — no over-import (an
  imported-but-unused name) and no under-import (a used-but-unimported name).
  Control: dropping any imported name reds either standalone elaboration or this
  consumed-set check.
- **AC-STANDALONE-GREEN.** Posix elaborates standalone (exit 0) after its import
  block — no ambient/scaffolding fallback. Control: removing the new import block
  restores the exact prior baseline failure D0 recorded.
- **AC-VISIBILITY-ONLY.** Every `pub` added changes visibility only: a
  differential shows the body (including all 74 theorems and both module-owned
  proofs) BYTE-UNCHANGED; publishing mints no class/instance; `trusted_base`
  delta zero; every consumer resolves to the single existing owner. No
  computational change, no proof authored.
- **AC-NO-REGRESSION / AFFECTED-CLOSURE.** Re-run the COMPLETE affected-target
  closure — every target loading Posix or a module whose closure this changes
  (the census fixture, cc6b, the new D0 fixture) — scoped by changed PATHS via
  `scripts/ken-cargo`, never `--workspace` (green in CI is the workspace
  verdict).

## Format any edited catalog file before handoff

`catalog/packages/Capability/Filesystem/Path/Posix.ken.md` must be in canonical
`kenfmt` form — the corpus-wide formatter fixed-point gate (`kenfmt_c_capstone`
/ `ken-cli/tests/ken_fmt.rs`) fires only in a full-workspace run (CI-only by
operator hard rule), so a targeted `-p ken-elaborator --test <yours>` cannot see
it. Run the formatter on the edited catalog file before handing the SHA to QA;
do NOT run `--workspace` locally to check it.

## Capability tier: T2 (with a T1-adjacent D0)

The mechanical work is the proven Tier-A/B/C/D shape — add `pub` to the
D0-determined surface, a three-import selective block, the inventory reflection,
the census move, and the D0 fixture — over a large but BYTE-UNCHANGED
proof-dense body; no class-instance relocation, no proof authoring. The judgment
is D0: (a) DETERMINING the public surface with no external importer to read it
off (the cc6b-resolved set is the authority), (b) the theorem-publication
question (design judgment 2), and (c) the attached-proof import spelling. That
is the T1-adjacent part the Architect confirms; the D1..D3 tail is mechanical.
foundation-leader verifies the seat against this estimate before pickup.

## Gate, reviewer, sequencing

`gate: none` (no TCB touch; the operator ruled the class-owner model). On the
candidate: **Architect** (required — surface DETERMINATION correctness +
class-uniformity + D0 completeness + the theorem-publication call) + **Foundation
QA + CV** on the exact SHA, then Steward M1-M4 → lieutenant. An independent
Tier-D singleton; the next Tier-D step (the System.IO theorem-name erratum,
which stays SEPARATE and LATER) is framed one release ahead after this lands,
with fixed inputs re-measured at that SHA.

## Contention and sequencing

`catalog/` (one module) plus the elaborator test fixtures, and additive.
Re-measured at 6dd88d3fb:

- **Two live CV-owned branches touch the Posix module and cc6b**:
  `wp/cc6b-path-posix` and `wp/px-posix-campaign-frame` (with their
  `remotes/conformance-validator/` mirrors), both about the cc6b acceptance / px
  campaign, NOT this migration. Re-measure at pickup whether either is active
  and holds uncommitted edits to `Posix.ken.md` or `cc6b_path_posix_acceptance.rs`;
  coordinate on the cc6b loader edit (D2) before touching it. This is the one
  real contention risk in the slice.
- `lang_mod_strict_resolution_d0.rs` is the shared census fixture edited by every
  Tier-D migration (Render #3432, Arguments #3430, Parsing #3428, Numeric #3426,
  Decoder #3424, Cursor #3423). One Tier-D WP at a time on the lane means it is
  free while Posix holds it; re-measure that no other Tier-D WP is mid-flight on
  it at pickup.
- The `cat_tier_d_*_import.rs` family is one file per module (no shared file), so
  `cat_tier_d_posix_import.rs` is contention-free except for the read-only shared
  `support/catalog_or.rs` / `support/catalog_publication.rs` (reuse, do not edit;
  keep the Posix provider list local to the new file).
- Lane 1 (runtime, `ken-host`/`ken-runtime` — ABI-S1 D4 in flight) and lane 2
  (language, the FO/Kripke prover in `ken-elaborator`) are disjoint from
  `catalog/` and from these Tier-D census fixtures. The concurrent doc track
  touches `library/` and `agent/`, disjoint. Foundation holds no other released
  node; this is the ring's work.

## Hard stop

Route to the Steward if:

- Posix CANNOT be brought standalone-green through a selective import from
  already-published providers — i.e. D0 surfaces a required provider owned by a
  still-scaffolded, unpublished module (a hidden cross-tier edge; cite it, do
  not drag the unpublished provider in); or
- the D0-determined public surface required to keep cc6b green forces publishing
  a large or API-inappropriate theorem set (design judgment 2) — cite the set
  and stop rather than freezing it by reflex; or
- delivering D1..D3 appears to require binding a new class/instance, authoring
  any proof content, retiring cc6b's manual pre-seed changes an acceptance
  RESULT (not just its load path), or touching the System.IO erratum. Any of
  those means the slice cut is wrong, not that the scope should bend.
