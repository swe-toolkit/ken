---
id: RT-TEST-HARNESS-STATICLIB-CURRENCY
title: "object_linker_packaging links the newest libken_runtime*.a it finds on disk and never establishes its currency, so a test can validate today's source against days-old compiled code -- and the dangerous direction is silent: a stale archive whose baked hash still matches passes while measuring the wrong artifact."
status: ready
owner: runtime
size: S
gate: none
tier: T2
depends_on: []
blocks: []
github: null
origin: "Isolated by runtime-implementer at evt_2jj57aq790r9e (2026-09-15) in the retraction of RT-EFFECT-CATALOG-GLOBAL-ARTIFACT-COUPLING, which this node supersedes. That node was filed on a false premise -- a global catalog coupling -- and the true cause was a stale staticlib linked by the test harness. Steward-filed per COORDINATION section 2."
---

> # FILED 2026-09-15 BY THE STEWARD. SUPERSEDES A WITHDRAWN NODE.
>
> **Read `RT-EFFECT-CATALOG-GLOBAL-ARTIFACT-COUPLING` first if you want the
> history** — it is `closed`, and it records the false premise, the four
> experiments that supported it, and why the strongest of them was the least
> sound. This node carries only the real defect.

## The defect

`object_linker_packaging`'s link step resolves its archive like this:

```
fn ken_runtime_staticlib() -> ... {
    // scan target/ and target/deps for libken_runtime*.a
    candidates.sort_by_key(|path| ...modified());
    candidates.pop()            // the NEWEST one on disk
}
```

It takes whatever archive is lying in the target directory and **never asks
cargo to build one**. `cargo test -p ken-runtime --lib` builds the rlib and the
test binary — **not the staticlib**. So the archive under test can be arbitrarily
old, and nothing in the harness notices.

## Why the silent direction is the one that matters

The observed instance failed **loudly**: the C starter bakes
`ken_host::HOST_EFFECT_ABI_V1_HASH` from the freshly compiled constant, the
stale archive carried an older hash, `ken_host_invocation_v1_init` mismatched,
and the starter returned 1 with both streams empty. That cost a day of
misattribution — but it was the lucky direction, because it fired.

**The dangerous direction is a stale archive whose baked hash still matches.**
Then the mismatch check passes, the suite goes green, and it has validated
today's source against days-old compiled code while reporting success. Nothing
in the harness or the suite would say so.

This is the same class as the other instrument defects this arc surfaced: an
instrument whose own inputs are unverified. See
`RT-MUTATION-ARM-CONTROL-COVERAGE` for the sibling case.

## Measured

Single-variable, same test binary untouched, only the archive rebuilt:

    before   libken_runtime.a Sep 14 21:05    1015 passed,  6 failed
    after    libken_runtime.a Sep 15 01:45    1021 passed,  0 failed

Stable across two runs.

## Not a CI defect

CI never hits this: the shard and px8f jobs run `cargo build --workspace
--locked` before testing, which materializes a current staticlib. **This is a
local-development correctness gap**, and its cost is paid in misattributed
diagnosis rather than in shipped defects — which is exactly what it cost here.

## Deliverables

- **D0 — establish currency.** Make the harness guarantee the archive it links
  corresponds to the source under test. Building it from the harness, asserting
  a freshness relation, or failing loudly when it cannot be established are all
  acceptable; picking silently is not.
- **D1 — a diagnostic on the failure path.** A hash mismatch that exits 1 with
  both streams empty is the worst available failure. It must say what
  mismatched.

## Acceptance criteria

- **AC-1.** With a deliberately stale archive present, the suite **refuses or
  rebuilds** — it does not run against it. Demonstrate with the stale archive in
  place; a run that merely passes does not show this.
- **AC-2 (positive control).** The refusal path names the actual mismatch
  (expected vs found hash, and the archive path with its mtime). Show the
  message from a real stale-archive run, not from a unit test of the formatter.
- **AC-3.** The ordinary green path is unchanged in cost — this must not become
  a full rebuild on every `--lib` run.
- **AC-4.** No regression, green in CI (never a local `--workspace` run;
  COORDINATION §12).

## Why T2

The cause is known, the fix is mechanical, and the design judgment is already
made above. This does not need a reasoning-tier seat.
