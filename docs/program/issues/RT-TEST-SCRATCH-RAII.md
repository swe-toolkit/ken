---
id: RT-TEST-SCRATCH-RAII
title: "Runtime and CLI test fixtures mint a nanosecond-suffixed scratch directory per run and never remove it -- `temp_output_dir` returns a bare `PathBuf`, `tempfile` is not a dependency, and the resulting ~1200 leaked directories per hour under load have filled `/workspaces/ken` to 100% seven times, where the failure presents as a broad regression in the linker-invoking suites rather than as a disk condition"
status: ready
owner: runtime
size: M
gate: none
depends_on: []
blocks: []
github: null
origin: Steward measurement 2026-08-11, taken while diagnosing the seventh recurrence of the volume filling. The recurrence history and the reclaim ordering are the Steward's operational record; the generator identification and the 42-site count are measured at current main and stated below.
---

## The defect

`native_execution_differential.rs:3302`:

```rust
fn temp_output_dir(name: &str) -> PathBuf {
    let mut dir = std::env::temp_dir();
    dir.push(format!("ken-runtime-{name}-{}", …as_nanos()));
    dir
}
```

It returns a **bare `PathBuf`**. Nothing removes the directory — not on
success, not on failure, not at process exit. Each run of each fixture leaks
one directory permanently, and the nanosecond suffix guarantees a fresh one
every time rather than reuse.

Measured at `origin/main`: **42 `std::env::temp_dir()` call sites** across
`crates/ken-runtime/src/` and `crates/ken-cli/tests/`, and **`tempfile` is not
a dependency** of either crate.

**Not every site leaks, and the distinction is the scope.** Sites that join a
**fixed** name — `ken-fs-flip-e2e`, `ken-cli-i1-entrypoint-abi`,
`ken-rosetta-runner` — reuse one directory across runs and contribute nothing
to the mass. The leaking sites are exactly those that interpolate a timestamp
or pid. **Sort the 42 by that axis before estimating; the second group is the
node.**

## Why this is a delivery problem and not housekeeping

`/workspaces/ken` is a 229G volume of which roughly 121G is Ken's to use. It
has reached **100%** seven times. The measured refill rate at the most recent
recurrence was **304 new scratch directories in 15 minutes — about 1200/hour**,
roughly triple the ~400/hour recorded in earlier passes.

**The symptom does not look like disk, and that is what makes it expensive.**
It surfaces as linker `SIGBUS`, as `No space left on device` mid-link, and as
what one implementer accurately called *"a broad regression in the
linker-invoking suites"* that is not one. Costs actually paid:

- A Runtime implementer spent a **38-minute turn** reclaiming its own
  `target/debug/incremental`, then sat idle with two deliverables outstanding.
- Three `.git/config.lock` write failures during a publisher run, which
  silently left a stale local remote ref.
- QA cycles spent re-deriving verdicts from runs that died this way. **A
  verdict from such a run is not a verdict.**

⇒ The constraint is a **measured capability gap**, not an aesthetic one. Every
reclaim so far has been a Steward action on someone else's derived data, and it
buys hours.

## What a fix has to do

**One shared RAII scratch helper, removed on drop**, and the leaking sites
migrated onto it. Two routes:

- **`tempfile::TempDir`** as a dev-dependency. Conventional, handles the
  panic path, and `TempDir` already has the drop semantics. Adds a dependency
  to a test surface.
- **A local guard type** implementing `Drop`. No new dependency; you write the
  panic-safety yourself.

**Either is defensible and the choice is the owner's.** State it in one
sentence.

**The helper must survive a failing test.** A scratch directory removed only on
the success path leaks exactly when the suite is red, which is when runs are
most frequent. `Drop` runs during unwind; an explicit cleanup call at the end
of a test body does not.

## What is not yet known

- **Whether any fixture deliberately outlives its test.** A directory kept for
  post-hoc inspection of a failure is a real use, and blanket RAII would remove
  the artifact a debugging session wants. If such a site exists, it needs an
  opt-out that is explicit rather than accidental. **This is the one question
  that could make the node bigger than it looks.**
- **Whether `TMPDIR` should point at the container `/tmp` instead.** The fleet
  has `std::env::temp_dir()` resolving to `/workspaces/ken/tmp` — the repo
  volume — while the container `/tmp` is a separate 7.8G filesystem sitting at
  1% used. Redirecting would move the mass off the volume that fills without
  fixing the leak. **That is a mitigation, not this node**, and it should not
  be taken as a substitute for the RAII fix.
- Whether the `ken-cli` test sites belong in the same node or a sibling. They
  are the same defect and the same helper, but a different crate.

## Not this node

No change to what any fixture asserts, no change to runtime or CLI behavior,
and no reclaim tooling. The reclaim procedure is the Steward's operational
record and does not belong in `crates/`.
