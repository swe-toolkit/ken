---
id: LANG-MOD-LOADER-ENTRY
title: "WP-1 — route ken check catalog files through elaborate_module_from_roots instead of the direct single-file call; behavior-preserving, non-strict"
status: ready
owner: language
size: M
gate: none
depends_on: []
blocks: [LANG-MOD-STRICT-RESOLUTION, LANG-MOD-CATALOG-REALIZATION]
github: null
origin: "Architect component framing evt_hpnhqy1ex286 (WP-1), under [[LANG-MODULE-IMPORT-SYSTEM]]. Steward-filed per COORDINATION section 2, 2026-08-23. RELEASED 2026-08-23 (language ring finished embedding-adequacy D1; operator finish-then-switch gate satisfied)."
---

> # RELEASED 2026-08-23 — leads the module/import campaign
>
> Full frame: `docs/program/wp/LANG-MOD-LOADER-ENTRY.md`, fixed inputs measured
> at `origin/main c386635306`. The release gate (language ring finishes
> [[V3-FO-EMBEDDING-ADEQUACY]] per operator finish-then-switch) is satisfied: D1
> landed at `5ef0f0983` and the ring is idle. This WP does NOT change strictness
> (WP-2) and does not depend on any further spec merge — the spec surface is
> already merged at `def16ecf4`.

# Objective

Route catalog `ken check` through the module loader so catalog files observe the
same resolution path the campaign hardens, WITHOUT changing behavior yet.

# The measured seam (Architect evt_hpnhqy1ex286)

`elaborate_cli_file` (`crates/ken-cli/src/main.rs:239-295`, call site 268-272)
dispatches on the `.ken.md` suffix straight to `elaborate_ken_md_file`,
BYPASSING the loader. There is no path->root/module derivation in the CLI today.
The loader `elaborate_module_from_roots` exists and is well-tested; `source_path`
(`crates/ken-elaborator/src/modules.rs:424-467`) is the module->path mapping
whose inverse this WP needs.

# Deliverable

- Derive the root (catalog/packages) + the dotted entry module from a catalog
  path — the INVERSE of `source_path`'s component grammar; reuse that grammar,
  do not reinvent it.
- Call the roots API for catalog files; non-catalog files keep the direct path.
- Lands in NON-strict mode: the loader's existing bare-name passthrough still
  applies (behavior-preserving).

# Acceptance criteria

- AC-1. An existing catalog `.ken.md` checks through the loader with no
  hand-fed export map and no regression; an `import` in the file resolves.
- AC-2 (cross-cutting invariant). Zero `trusted_base()` delta; flat-Σ pin
  `module_elaborates_to_identical_flat_sigma`
  (`es3_modules_acceptance.rs:28`) stays green (extend, never weaken).
- AC-NO-REGRESSION. Whole-suite green in CI (COORDINATION section 12); local
  targeted `-p` only, never `--workspace`.

This WP does NOT change strictness — that is WP-2.

# Reviewers

Architect (component fit) + conformance-validator (catalog-front-end
reachability: a case that proves a catalog-root-addressed front end actually
uses the loader rather than isolated-file elaboration — CV group 4,
evt_2wejn8hekr4qw).

# Capability tier

T2 (mechanical routing + a path-grammar inverse; review is differential —
same behavior through a new entry). Size M.
