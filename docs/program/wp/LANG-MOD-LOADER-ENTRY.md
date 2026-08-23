# WP — LANG-MOD-LOADER-ENTRY (module/import WP-1): route catalog `ken check` through the module loader

Language lane (module/import campaign [[LANG-MODULE-IMPORT-SYSTEM]]). One WP, one
branch `wp/LANG-MOD-LOADER-ENTRY`, one PR. Owner: language. Size: M. Gate: none.
Depends on: none (independent, can lead the campaign). Blocks:
LANG-MOD-STRICT-RESOLUTION (WP-2 needs catalog files observable through the
loader), LANG-MOD-CATALOG-REALIZATION (WP-4).

Source: Architect component framing `evt_hpnhqy1ex286` (WP-1) and the
strict/legacy keying revision `evt_xtscdw8r3q3k` (strict is keyed on
root-loaded, so this WP — which does NOT flip strictness — lands purely
behavior-preserving). Campaign spec surface merged at `def16ecf4`.

Fixed inputs measured at `origin/main c386635306`.

## Objective

Make catalog `.ken.md`/`.ken` files check through the module loader
(`elaborate_module_from_roots`) instead of the isolated-file elaboration path,
WITHOUT changing resolution behavior. Today `ken check <catalog-file>` bypasses
the loader entirely; every later WP in the campaign hardens resolution AT the
loader, so the catalog front end has to reach the loader before any of that is
observable. This WP is the plumbing that makes it reachable, and nothing more —
strictness is WP-2, `pub`-eligibility is WP-3, catalog import rewrites are WP-4.

## Fixed inputs (SETTLED, at `origin/main c386635306`)

1. The bypassing seam: `crates/ken-cli/src/main.rs` — `elaborate_cli_file`
   (`:239-296`). It reads the file, makes a fresh `ElabEnv`, then dispatches on
   the `.ken.md` suffix straight to `elaborate_ken_md_file` else `elaborate_file`
   (`:268-272`) — both isolated-file paths. There is no path->root/entry
   derivation anywhere in the CLI. The result is `Vec<ken_kernel::GlobalId>`
   consumed unchanged by the caller (`:295`).
2. The target API: `crates/ken-elaborator/src/modules.rs` —
   `elaborate_module_from_roots(elab, roots, entry)` (`:625-648`). It requires
   EXACTLY ONE populated catalog root, records/locks `catalog_roots` on the
   `ElabEnv`, and calls `load_unit(elab, entry, ...)`. It returns the same
   `Vec<GlobalId>` shape as the isolated path — so a catalog file routed through
   it yields an interchangeable result for the CLI caller.
3. The grammar to invert: `source_path(root, module, span)`
   (`modules.rs:424-467`). It maps a dotted module to a file by pushing each
   dotted component as a directory segment under `root`, then trying the leaf
   with `.ken` then `.ken.md`. A valid component is ASCII-uppercase-initial then
   `[A-Za-z0-9_']*`. The strict bijection guarantees a path position is a leaf
   OR a directory, never both, and exactly one source spelling per leaf. WP-1
   needs the INVERSE (a catalog file path -> its single root + dotted entry) and
   must reuse THIS component grammar, not reinvent it.
4. The cross-cutting invariant + its pin: flat-Σ / zero `trusted_base()` delta.
   `crates/ken-elaborator/tests/es3_modules_acceptance.rs` —
   `module_elaborates_to_identical_flat_sigma` (`:28`). Extend it to cover the
   new front-end entry; never weaken it. Module routing allocates no kernel
   decl and imported names keep the provider's existing `GlobalId`.

## Design judgment (front-loaded)

- **What "catalog file" means, operationally.** A file whose path lies under a
  `catalog/packages` root. The robust inverse: walk up from the CLI file path to
  the nearest `catalog/packages` ancestor directory; that ancestor is the single
  root, and the remaining path (with the `.ken`/`.ken.md` leaf extension
  stripped, components joined by `.`) is the dotted entry. Validate each derived
  component against `source_path`'s `valid_component` predicate; if the
  derivation fails (no catalog root ancestor, or a component that is not a legal
  module component), fall back to the existing isolated-file path. Non-catalog
  files therefore keep the direct call verbatim.
- **Where the inverse lives.** Prefer a small public helper beside `source_path`
  in `modules.rs` (it is the natural home of the component grammar and keeps the
  bijection's two directions co-located), exported for the CLI to call. A
  CLI-local reimplementation of the grammar is the anti-goal (it would drift from
  `source_path`).
- **Why this is behavior-preserving.** The loader lands in its existing
  bare-name passthrough (non-strict) mode; `elaborate_module_from_roots` does not
  itself impose strict resolution — that is WP-2, keyed separately on root-loaded
  units. A catalog file that resolves today under isolated elaboration continues
  to resolve, now via `load_unit`; a catalog file carrying an `import` gains
  working resolution it did not have in isolation. No catalog file's accept/reject
  verdict regresses.
- **Extension handling is the loader's job, not the CLI's.** Once on the roots
  path the CLI hands only the dotted entry; `source_path` inside the loader
  finds the `.ken` vs `.ken.md` leaf. The CLI's own suffix branch survives only
  on the non-catalog fallback.

## Deliverables (each targets a releasable increment or a hard stop)

1. A path->{root, dotted-entry} inverse of `source_path`, reusing its component
   grammar, with the fallback contract above (returns "not a catalog path" rather
   than erroring, so the CLI can fall through).
2. `elaborate_cli_file` routes catalog paths through
   `elaborate_module_from_roots([root], entry)`; non-catalog paths keep the
   isolated `elaborate_ken_md_file`/`elaborate_file` dispatch unchanged.
3. Test coverage: an existing catalog `.ken.md` checks through the loader with no
   hand-fed export map; a catalog file with an `import` resolves through it; a
   non-catalog file still takes the isolated path (the fallback fires).

## Acceptance criteria (property + closure axis + loud failure)

- **AC-1 (loader reachability).** An existing catalog `.ken.md` checks through
  `elaborate_module_from_roots` with no hand-fed export map and no verdict
  regression, and an `import` in a catalog file resolves. CONTROL: a
  catalog-root-addressed front-end case that FAILS if the file is elaborated in
  isolation rather than through the loader (proves the route is actually taken,
  not that isolated elaboration happens to accept the same file) — the
  conformance-validator group-4 catalog-front-end-reachability discriminator
  (`evt_2wejn8hekr4qw`).
- **AC-2 (fallback).** A non-catalog file (no `catalog/packages` root ancestor)
  still elaborates via the isolated path — the inverse returns "not a catalog
  path" and the CLI falls through. CONTROL: a non-catalog fixture that would
  ERROR if fed to the single-root loader (`roots.len() != 1` or an
  invalid-component path) checks green via the fallback.
- **AC-3 (cross-cutting invariant).** Zero `trusted_base()` delta; the flat-Σ
  pin `module_elaborates_to_identical_flat_sigma`
  (`es3_modules_acceptance.rs:28`) stays green, extended to the new entry, never
  weakened. Imported names keep the provider's `GlobalId`.
- **AC-NO-REGRESSION.** Whole-suite green in CI (COORDINATION §12). Local builds
  targeted `-p` only (`ken-cli`, `ken-elaborator`), never `--workspace`.

This WP does NOT change strictness (WP-2), does NOT add a `pub`-eligibility gate
(WP-3), and does NOT rewrite any catalog file's declarations/imports (WP-4).

## Contention

Clear on the language ring. Writes land in `crates/ken-cli/src/main.rs` and a
helper in `crates/ken-elaborator/src/modules.rs`. The runtime priority-1 lane's
in-flight slice (RT-CHECKED-IH-CAPTURED-ENV-SCHEMA tier 2) is `crates/ken-runtime`
ONLY — no file overlap. No other language WP is in flight (embedding-adequacy D1
landed; D2/D3 deferred under finish-then-switch). WP-2/WP-3 are the same ring,
sequenced after WP-1, so the ring self-serializes. The Steward routes both
language and runtime merges to sequence any future window.

## Capability tier (§4h)

T2 (cheap coder). Mechanical CLI routing plus a path-grammar inverse that reuses
an existing predicate; the review is differential (same accept/reject behavior
through a new entry, plus the reachability control). No novel design, no
soundness-bearing invention. Size M.

## Reviewers

- Architect — component fit (the inverse lives with the grammar; the roots call
  is the sanctioned entry; behavior-preserving).
- conformance-validator — catalog-front-end reachability: a discriminator proving
  a catalog-root-addressed front end uses the loader rather than isolated-file
  elaboration (CV group 4, `evt_2wejn8hekr4qw`).

## Do-not guards

- Do NOT flip strict resolution or touch `resolve_ref`'s None-arm passthrough
  (`modules.rs:174-216`) — that is WP-2.
- Do NOT reimplement `source_path`'s component grammar in the CLI; reuse it.
- Do NOT weaken `module_elaborates_to_identical_flat_sigma`; extend it.
- Do NOT change any catalog file's contents — WP-4 owns catalog rewrites.

## Sequencing (Steward-owned)

Leads the campaign (WP-1 -> WP-3 -> WP-2 -> WP-4, WP-2's D0 optionally parallel).
Released now: the language ring finished its in-flight WP (embedding-adequacy D1)
and is idle, satisfying the operator finish-then-switch gate. On merge, flip the
node done and release WP-3 (LANG-MOD-PUB-ELIGIBILITY, independent) next.
