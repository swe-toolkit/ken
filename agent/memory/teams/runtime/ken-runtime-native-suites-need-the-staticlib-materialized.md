---
scope: teams/runtime
audience: runtime-leader, runtime-implementer, runtime-qa
source: runtime-qa retro evt_74n1bvzqa2mve on RT-PLANNER-UNITS-ABI-SPLIT D1
  (2026-08-18); carry directed to memory by the Steward (evt_3fjhtp3zfr0a5)
  because it bites every D2-D18 test move
related: (none)
---

# ken-runtime native suites red on a cold `test` — materialize the staticlib first

`ken-runtime` is crate-type `rlib + staticlib`. The native suites
(`native_execution_differential`, `object_linker_packaging`) link against
`libken_runtime.a`, which `cargo test` alone does **not** materialize in the
target state.

**Measured live (2026-08-18, RT-PLANNER-UNITS-ABI-SPLIT D1):** a first
`scripts/ken-cargo test -p ken-runtime --lib` run showed **40 reds** in
`native_execution_differential`/`object_linker_packaging`. The root cause was
the missing `libken_runtime.a`, not the diff under test — after
`scripts/ken-cargo build -p ken-runtime --lib` materialized it, the same test
run came back **fully green**.

## How to apply

For any scoped ken-runtime test run, in order:

1. `scripts/ken-cargo build -p ken-runtime --lib` first — this materializes
   `libken_runtime.a` in the target state.
2. Then `scripts/ken-cargo test -p ken-runtime --lib` (or the targeted suite).

A cold `test` that reports the native suites red is **not evidence of a diff
defect** — attribute it to the toolchain state before suspecting the code, and
confirm the absolute reading on the candidate and its base with the staticlib
present. This is the same base-absolute-reading discipline as the fleet
`failed-post-condition-probe-suspect-the-probe-first` lesson, applied to the
build environment rather than to a grep.
