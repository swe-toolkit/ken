---
id: LANG-MOD-STRICT-RESOLUTION
title: "WP-2 — strict root-loaded resolution: a bare name in a loader-built scope resolves only through locals, explicit imports, kernel vocabulary, and the closed prelude floor; no fall-through to arbitrary program globals (the soundness core)"
status: draft
owner: language
size: L
gate: none
depends_on: [LANG-MOD-LOADER-ENTRY]
blocks: [LANG-MOD-CATALOG-REALIZATION]
github: null
origin: "Architect component framing evt_hpnhqy1ex286 (WP-2), keying REVISED to strict-all-root-loaded in evt_xtscdw8r3q3k, under [[LANG-MODULE-IMPORT-SYSTEM]]. Steward-filed per COORDINATION section 2, 2026-08-23. FRAMED; release HELD (see campaign root release gate)."
---

> # FRAMED — HELD FOR RELEASE. Soundness core; D0 probe FIRST.
>
> Rides on [[LANG-MOD-LOADER-ENTRY]] for catalog observability (testable via the
> roots API alone). Release held until the language ring finishes
> [[V3-FO-EMBEDDING-ADEQUACY]] (finish-then-switch). Author the wp/ frame against
> the MERGED spec candidate 860c605 so ACs cite CV's landed conformance fold.
> This is the WP the Architect will review hardest.

# Objective

Make a bare name in a loader-built (root-loaded) scope resolve ONLY through
{locals, explicit imports, kernel/built-in vocabulary, the closed prelude floor
Bool/Char/List (30-taxonomy §4)}. Anything else is UnboundName/UnresolvedCon at
the surface — no fall-through to arbitrary implementation globals. Legacy
isolated-file programs keep the existing passthrough verbatim.

# The REVISED design key (Architect evt_xtscdw8r3q3k — supersedes evt_hpnhqy1ex286)

Strict resolution is keyed on ROOT-LOADED (loader use), NOT on a package
boundary header. /spec candidate 860c605 makes every root-loaded unit strict;
the Architect aligned to it — a boundary-header carve-out would introduce a
third mode (root-loaded-but-non-strict) the spec does not have. DROP the
boundary-header key.

- Mode threads from the ENTRY: `elaborate_module_from_roots` ⇒ strict;
  `elaborate_file`/`elaborate_ken_md_file` (isolated-file, non-root) ⇒ legacy
  passthrough retained VERBATIM.
- The sanctioned floor stays reachable by filtering the fallback to floor
  membership — identified structurally via the kernel's existing `is_prelude` +
  the kernel-builtin predicate (`crates/ken-elaborator/src/env.rs:568-579`
  already uses `is_prelude`) — NEVER by admitting arbitrary program globals.
- The mode threads through Scope + ElabCtx to BOTH fallback seams: the
  `resolve_ref` None arm (`modules.rs:210-216`, returns the name unchanged) AND
  the `elab.rs` RCon globals lookup (`elab.rs:3918-3925`, `cx.globals.get`). The
  returned-String loses the mode, so it rides on the ctx. Both seams must honor
  it.

# Deliverables

- D0 (buildability probe FIRST — the pattern that served M6/ABI-M1/embedding-
  adequacy). Confirm the floor (kernel vocabulary + prelude) is resolvable in
  strict mode WITHOUT the arbitrary-globals passthrough, and confirm ROOT-LOADED
  (loader entry) is the right strict-mode key. Both are grounded questions a
  probe answers before the D1 flip.
- D1. Thread the mode; fail-close the two seams for root-loaded units; retain
  the legacy passthrough for isolated-file units.

# Acceptance criteria

- AC-1. Remove one required import from a root-loaded unit ⇒ the name becomes
  UnboundName, NOT a global fall-through.
- AC-2. Imported names resolve to the EXACT provider GlobalIds (identity
  assertion, not text).
- AC-3 (the one non-degenerate pair the Architect reviews hardest). In the SAME
  root-loaded unit: a floor name (e.g. Bool) resolves strict WHILE a non-floor
  non-imported convenience global rejects strict.
- AC-4. The acyclic/cycle, cache-once, rename, prelude-clash, flat-Σ controls
  (`n2_in_repo_loader.rs`, `l4_export_reexport.rs`, `es3_modules_acceptance.rs`,
  `n3_import_exclusion.rs`) stay green.
- AC-5. Legacy isolated-file programs unchanged — `l_resolver_globals.rs` green
  in legacy mode.
- AC-6 (cross-cutting invariant). Zero `trusted_base()` delta; flat-Σ pin stays
  green.
- AC-CO-GATE. WP-2's strict flip flag-day is SEQUENCING: WP-2's CI-greenness is
  CO-GATED with WP-4's census-driven catalog migration (see
  [[LANG-MOD-CATALOG-REALIZATION]]). WP-2 may land strict-mode plumbing that is
  exercised by roots-API fixtures before the whole catalog is migrated, but the
  full-catalog strict flip does not go green until WP-4's census set migrates.
- AC-NO-REGRESSION. Whole-suite green in CI; local targeted `-p` only.

# Reviewers

Architect (the strict floor must admit the kernel/prelude floor and NOTHING
else; the legacy passthrough must be untouched) + conformance-validator
(dependency-closure/global-fallback discriminators with floor-accept — CV group
2, evt_2wejn8hekr4qw).

# Capability tier

T1 (soundness-bearing resolution change threading a mode through two fallback
seams; review turns on an argument, not a diff). Size L.
