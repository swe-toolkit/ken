---
id: LANG-MOD-STRICT-RESOLUTION
title: "WP-2 — strict root-loaded resolution: a bare name in a loader-built scope resolves only through locals, explicit imports, kernel vocabulary, and the closed prelude floor; no fall-through to arbitrary program globals (the soundness core)"
status: merged
owner: language
size: L
gate: none
depends_on: [LANG-MOD-LOADER-ENTRY]
blocks: [LANG-MOD-CATALOG-REALIZATION]
github: null
origin: "Architect component framing evt_hpnhqy1ex286 (WP-2), keying REVISED to strict-all-root-loaded in evt_xtscdw8r3q3k, under [[LANG-MODULE-IMPORT-SYSTEM]]. Steward-filed per COORDINATION section 2, 2026-08-23. RELEASED 2026-08-23 (WP-3 merged c39024f50; language ring clear; ring order WP-1->WP-3->WP-2)."
---

> # MERGED — delivered inside the module/import realization; node reconcile 2026-08-25
>
> WP-2's substance (D0 census + D1 strict enforcement) shipped as ancestors of
> `origin/main` and the node was never flipped from `ready` — a stale-node
> reconcile, not missing work. Verified by the Steward at `d7c6283ff`
> (independently confirmed by language-implementer evt_4fxnfxxy2583t +
> language-leader evt_3hk6gbmrhkqv2): D0 `c64c62190` and D1 `5a74301f4` are both
> ancestors; `crates/ken-elaborator/tests/lang_mod_strict_resolution_d0.rs` and
> `..._d1.rs` are present; `modules.rs` carries `ResolutionMode::{Legacy,Strict}`,
> `PRELUDE_FLOOR_NAMES = [Bool,Char,List]`, strict `resolve_ref` fail-close,
> roots-mode cache separation, and `elaborate_module_from_roots_strict`, with
> `lib.rs` exposing the strict entry. The D1 strict flip and WP-4's catalog census
> (Component A/B `574eb90c0`, `76426e9f9`) both landed, so the AC-CO-GATE is
> satisfied on main. A 2026-08-25 Steward re-release off the stale `ready` node was
> withdrawn on the implementer's hard stop (no non-duplicative delta;
> subsume-don't-proliferate). The frame below is the historical release record.

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
  membership — NEVER by admitting arbitrary program globals. The floor is
  installed by `install_prelude_floor` (`crates/ken-elaborator/src/modules.rs:83`);
  the exact floor-membership predicate to filter on is a D0 finding. (The earlier
  `env.rs:568-579`/`is_prelude` citation is STALE — no `env.rs` and no
  `is_prelude` exist on `c39024f50`; D0 identifies the current predicate.)
- The mode threads through Scope + ElabCtx to BOTH fallback seams: the
  `resolve_ref` None arm (`modules.rs:222`, `None => Ok(name.to_string())` —
  returns the name unchanged) AND the value-level `elab.rs` RCon globals lookup
  (the `RExpr::RCon` arm `elab.rs:3910`, `cx.globals.get` fall-through at
  `:3919-3926` yielding `UnresolvedCon` on miss). The returned-String loses the
  mode, so it rides on the ctx. Both seams must honor it. D0 confirms the complete
  seam set — e.g. whether the type-level `RType::RCon` globals lookup
  (`elab.rs:620`) is also a fall-through that needs closing.

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
