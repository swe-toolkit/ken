---
id: RT-NATIVE-API-PROBE-RLIB-EXPLICIT-LOCATION
title: "px4b_native_production's public-API probes find ken_runtime's rlib and its dependency directory as the test binary's own directory, and under Cargo 1.100.0 the test binary no longer sits beside its dependencies' rlibs, so naked_process_ir_helpers_are_not_public_production_api fails 'no libken_runtime-*.rlib' on every runner image carrying Cargo 1.100.0 and any candidate whose shard draws that image is red with no source change. Locate the rlib the probe compiles against explicitly, so the probe no longer depends on Cargo's output layout"
status: merged
owner: runtime
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Steward base-blocker attribution on LANG-ATTACHED-PROOF-CHECKED-SELECTION PR #5407, CI run 44400765687 (Lieutenant evt_2e6jcp5s1dvm6). Same environment class as RT-NATIVE-RUNTIME-STATICLIB-EXPLICIT-BUILD (merged 4e6c4e62b), whose AC-3 1.100 shard did not run this test. Steward-filed per COORDINATION section 2."
---

# The public-API probes locate ken_runtime explicitly

## Objective

The two `px4b_native_production` probes that compile a small crate
against `ken_runtime` to show a helper is not public pass on Cargo
1.99.1 and 1.100.0 alike. They compile against an rlib that Cargo named,
not one found beside the test binary.

## Settled inputs (main `4e6c4e62b`)

- **The lookup.** `select_current_ken_runtime_rlib`
  (`crates/ken-cli/tests/px4b_native_production.rs:996`) takes
  `deps = current_exe().parent()` (`:1001`) and searches it for
  `libken_runtime-*.rlib`. It asserts that at least one exists (`:1018`),
  then compiles probes with `rustc --extern ken_runtime=<rlib> -L <deps>`
  (`:937`-`:947`). Its callers are
  `assert_packaging_helper_is_not_reachable_from_outside_ken_runtime` and
  `naked_process_ir_helpers_are_not_public_production_api`
  (`:1167`, `:1186`). No other `crates/*/tests` file uses
  `current_exe()` as an artifact directory; the other uses re-execute the
  test binary.
- **The environment split, on base `4e6c4e62b` both times.**
  - Run 44385242682, shard 3/7 on image 20261111.196.1, with Cargo
    1.99.1: the row passes.
  - Run 44400765687, shard 3/7 on image 20261116.202.1, with Cargo
    1.100.0: it fails "no libken_runtime-*.rlib under
    .../target/debug/build/ken-cli/64425912e886734a/out".
  - The candidate in that run touches only `ken-elaborator`, spec 33 and
    conformance paths.
- **The precedent.** RT-NATIVE-RUNTIME-STATICLIB-EXPLICIT-BUILD (D0
  `evt_7af482gtgwkfy`) replaced a side-effect archive search with a
  producer that prints the path Cargo's compiler-artifact JSON reports,
  over a named channel.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

The rlib and dependency-path lookup for these two probes, and any CI or
`scripts/ken-cargo` step the ruled channel needs. No change to what the
probes assert, to `ken-runtime`'s public surface, or to other tests.

## Deliverable

One runtime candidate.

## Acceptance

- **AC-0 (D0, Architect, before code).** Rule the family. The options
  are:
  - an explicit channel, as in the staticlib precedent, naming the rlib
    and every `-L dependency=` directory the probe needs;
  - re-expressing the two probes as `compile_fail` doctests in
    `ken-runtime`, which Cargo builds against the current crate, if they
    keep the probes' positive control and freshness meaning;
  - another form that removes the layout dependency.

  Rule whether the freshness post-condition (`:1100` region) survives.
- **AC-1.** On Cargo 1.100.0 with a fresh target directory, both probes
  pass. The rlib they compiled against is the one the ruled channel
  names. Report `cargo --version`.
- **AC-2 (QA mutation).** Make one of the two helpers public. The probe
  that guards it reddens on 1.100.0 because the probe compiled, not
  because of a lookup failure. Restore it byte-identically.
- **AC-3.** Full-workspace CI is green on the exact candidate. Each test
  shard's image version is read, and the shard that ran these two rows
  drew a Cargo 1.100.0 image. Absent that, the rows' 1.100.0 pass is AC-1's
  local report.

## Stop conditions

- The probes need a change to `ken-runtime`'s public surface or to what
  they assert.
- A second test outside these two shows the same layout dependency on
  1.100.0. Report it, so the Steward re-scopes rather than chasing it.

Checks: 1 fired. The environment attribution rests on two runs on the
same base, which differ only in the shard's image: one passed on Cargo
1.99.1 and one failed on 1.100.0. 3 fired. I grepped `current_exe` over
`crates/*/tests` at `origin/main`, and only `:1001` uses it as an artifact
directory. 8 fired. AC-2 reddens through the probe compiling, so a lookup
that silently finds nothing cannot pass it.
