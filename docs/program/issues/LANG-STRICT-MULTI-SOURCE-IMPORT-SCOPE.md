---
id: LANG-STRICT-MULTI-SOURCE-IMPORT-SCOPE
title: "Strict native preparation over several sources shares one root scope, so the first source's selective imports bind for every later source and a later source can use a name it never imported. prepare_isolated_strict_scope_v1 builds a fresh Strict root_scope only when the mode is not already Strict. Give each source its own import scope, as spec 33 requires; catalog roots stay one set per run"
status: active
owner: language
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Adversary evt_6rew51yakk2pw on Inc3b 4f7b64631, mechanism re-read by the Steward on 4f7b64631. L2 objective: one resolution mode (operator 2026-09-25). Steward-filed per COORDINATION section 2."
---

# Each source keeps its own imports

## Objective

When several sources are prepared together under Strict, each source
sees only the names it imports itself, plus the floor. An import in one
source never binds a name in another, and a later source does not see
an earlier source's top-level declarations either. Catalog roots stay one
set per elaboration run.

## Settled inputs (main `4f7b64631`)

- **The rule.** `spec/30-surface/33-declarations.md:312`: "A loaded
  dependency is not implicitly imported and units do not inherit each
  other's imports."
- **The loop.** `compiler_driver.rs:2474`-`2481` calls
  `prepare_isolated_source_strict_v1` (`lib.rs:728`) once per
  `CompilerSource`, on one shared `ElabEnv`.
- **The leak.** `prepare_isolated_strict_scope_v1` (`modules.rs`)
  replaces `root_scope` only when its mode is not already Strict
  (`:2796`). From the second source on it does not, so the first
  source's import bindings stay in place.
- **Roots do not leak.** A second source with different roots already
  refuses with "catalog roots cannot change during one elaboration run"
  (`:2759`-`2766`). Per-source roots in one env would be unsound:
  `loaded_units` is keyed by module path alone (`:2407`), so a cached
  unit from one root would serve a later source with another root.
- **The repro** (Adversary `evt_6rew51yakk2pw`, public API). Two sources
  through `prepare_native_program_sources`: a dependency that imports
  `Capability.Process.Exit (host_pure)`, and a main that uses
  `host_pure` with no import. Main alone refuses with UnboundName;
  both sources without the import refuse; with the import only in the
  dependency, main is accepted.
- **Reach.** Production passes one source (`ken-cli` `lib.rs:53`,
  `main.rs:330`). `prepare_native_program_sources` and
  `compile_native_program_sources` are public and take
  `Vec<CompilerSource>`, so any multi-source caller inherits the leak.
  The only two-source test, `native_program_example_exclusion`
  (`compiler_driver.rs:9525`), imports `host_pure` in main and nothing
  in the dependency, so it cannot tell the two scopings apart.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

One candidate: a per-source Strict scope and roots in native
preparation, with the rows below.

## Acceptance

- **AC-0 (D0, Architect, before product code).** Rule what a later
  source sees of an earlier source's own top-level declarations under
  spec 33, since the current shared scope also carries those. Then rule
  where the reset sits: in `prepare_isolated_strict_scope_v1`, or in the
  driver loop.

  Ruled (`evt_46g0y6w6bbj96`): a later source sees nothing of an earlier
  one (33 §3.3, "This is the complete scope"), though declarations still
  enter the shared env and the artifact. The reset is unconditional in
  `prepare_isolated_strict_scope_v1`, the seam every Strict route
  shares. The two-source form of
  `native_main_referencing_example_refuses_before_emit` becomes an
  UnboundName row, and a single-source form keeps
  PackageReferenceOutsidePackage covered. A scratch census and two
  observations precede product code: the duplicate-declaration refusal
  across sources, and the single-source example refusal.
- **AC-1.** The repro becomes a test: with the import only in the
  dependency, main refuses with UnboundName `host_pure`. With the import
  in main, it is accepted.
- **AC-2.** Two rows:
  - two sources with the same roots and different imports each resolve
    only their own imports;
  - two sources with different roots refuse with the existing "catalog
    roots cannot change during one elaboration run" error, and a test
    pins that refusal.
- **AC-3.** Putting the shared scope back makes AC-1's refusal row
  accept.
- **AC-4.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- AC-0 finds that a public caller depends on the shared scope.
- A kernel change, or a change to Legacy resolution.

Checks: 2 fired: both guards in `prepare_isolated_strict_scope_v1` were
read at `4f7b64631`, and the driver loop is their only multi-source
caller. The frame first said `catalog_roots` also leaked; the roots
refusal at `:2759` was not read, and the Architect corrected it. 6
fired: the multi-source population is the public preparation API's
callers in `crates/*/src`: production passes one source, and only
`native_program_example_exclusion` passes two. 8 fired: AC-1's two
controls differ only in where the import sits, so per-source and shared
scoping give different observations.
