---
id: RT-TEST-SCRATCH-RAII
title: "Runtime and CLI test fixtures mint a nanosecond-suffixed scratch directory per run and never remove it -- `temp_output_dir` returns a bare `PathBuf`, `tempfile` is not a dependency, and the resulting ~1200 leaked directories per hour under load have filled `/workspaces/ken` to 100% seven times, where the failure presents as a broad regression in the linker-invoking suites rather than as a disk condition"
status: merged
owner: runtime
size: M
gate: none
depends_on: []
blocks: [RT-SCRATCH-LIFETIME-REMAINING-CRATES]
github: null
origin: Steward measurement 2026-08-11, taken while diagnosing the seventh recurrence of the volume filling. The recurrence history and the reclaim ordering are the Steward's operational record; the generator identification and the 40-site count are measured at current main and stated below.
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

Measured at `origin/main`: **40 `std::env::temp_dir()` call sites** across
`crates/ken-runtime/src/` and `crates/ken-cli/tests/`, and **`tempfile` is not
a dependency** of either crate.

> ### THAT COUNT IS ONE AXIS OF TWO. Corrected 2026-08-13, eighth recurrence.
>
> `r3_4b_observation_feature_is_native_artifact_identical` keys its scratch on
> `std::process::id()` under **`CARGO_TARGET_TMPDIR`**, not `temp_dir()`, and
> removes nothing. It is an artifact-level A/B, so it must not share a Cargo
> target directory — **each run leaves two full target trees.** Ten accumulated
> runs were **11G**, taking `/workspaces/ken` to 97% used.
>
> ⇒ **A grep for `temp_dir` returns zero hits on the largest leaker in the
> tree.** Enumerate both axes; the frame's `AC-3` now requires both populations
> reported separately. `RT-4B-UNIQUENESS-GATE-REACH`'s `AC-9` is a point fix on
> that one fixture, because its own `AC-6` re-arms the leak by construction —
> **this node owns the class.**

## Implemented lifetime policy

The implementation census classified the 40 `std::env::temp_dir()` sites as
four fixed-name sites, eighteen sites with an existing drop guard, and eighteen
timestamp/PID sites migrated to `tempfile`. Every migrated system-temporary
site cleans unconditionally on success and unwind. None may preserve evidence.

The separate `CARGO_TARGET_TMPDIR` census found fourteen sites: thirteen
fixed/reused locations and the feature-off/on artifact identity fixture. The
one preservation exception is based on that Cargo-owned location, whose
contents are reclaimable by `cargo clean`, not on the fixture's evidentiary
value. The identity fixture removes both trees on success and on any failure
before its byte-identity assertions, and that path prints the preserved
directory before resuming the panic.

**Corrected 2026-08-14.** This paragraph used to end *"Only a failing identity
assertion keeps the trees."* **Two** assertions sit inside the preservation
scope, and the first is a **precondition** — that neither artifact came back
empty. Its failure is not an identity failure; it means the identity relation
was never validly evaluated. The trees are preserved on that path too, which is
the useful behaviour: they are what shows why an artifact came back empty. The
code is right and the sentence was too narrow, so the sentence changed.

**Not every site leaks, and the distinction is the scope.** Sites that join a
**fixed** name — `ken-fs-flip-e2e`, `ken-cli-i1-entrypoint-abi`,
`ken-rosetta-runner` — reuse one directory across runs and contribute nothing
to the mass. The leaking sites are exactly those that interpolate a timestamp
or pid. **Sort the 40 by that axis before estimating; the second group is the
node.**

> ### THAT SCOPE IS TWO DIRECTORIES, AND THE DEFECT IS NOT. Recorded 2026-08-14.
>
> **Both stated counts are exact and the classification of all 40 is correct** —
> an independent recount at the pre-merge base agreed on the 40 and on the 14,
> and found the unclassified population within the declared scope to be
> genuinely zero. **Nothing below disputes what landed.**
>
> But the scope is `crates/ken-runtime/src/` and `crates/ken-cli/tests/`, and
> unguarded sites of exactly this shape remain in `ken-interp`, `ken-host` and
> `ken-verify`. The strongest of them, `ken-interp/src/eval.rs`'s
> `rt_parity_root`, matches this node's own defect statement clause for clause
> and generates `ken-rt-parity-*` — **the second producer of a prefix
> `scripts/ken-cargo`'s reaper already names.**
>
> ⇒ **This is the ONE-AXIS-OF-TWO correction a second time, under a different
> axis.** That one was about environment-variable **spelling**, and `AC-3`
> requires both spellings reported separately. Nothing in it ranges over
> **path**. A census that declares its scope and then reports zero unclassified
> is stating a true fact about a set it chose; a reader takes it as a fact about
> the hazard, and the hazard here is the repo volume, which is not confined to
> two directories.
>
> **The mechanism is the boundary, not availability:** `tempfile` was added to
> `ken-elaborator`, `ken-cli` and `ken-runtime` only. **The fix's reach and the
> census's scope stop at the same line**, which is why neither shows the other's
> edge.
>
> Successor: **`RT-SCRATCH-LIFETIME-REMAINING-CRATES`**, which defines its
> population by the hazard property rather than by a directory list.

### The four fixed-name sites, both axes. Recorded 2026-08-14, post-merge

Architect finding at `evt_2gcg9gxb9yxgg` and its amendment at
`evt_6qae8sz45070n`: the four do not leak, they **collide** — a different hazard
from the one this node repaired, and the census counted them without saying why
a fixed name is safe.

**The count was right and the list above was illustrative.** The fourth,
unnamed there, is `ken-effect-composition-e2e`.

| site | fixed name | acquisition |
|---|---|---|
| `ken-cli/tests/rosetta.rs:158` | `ken-rosetta-runner` | `create_dir_all` |
| `ken-cli/tests/fs_read_file_lines_flip_e2e.rs:155` | `ken-fs-flip-e2e` | `create_dir_all` |
| `ken-cli/tests/cli_i1_entrypoint_abi.rs:14` | `ken-cli-i1-entrypoint-abi` | `create_dir_all` |
| `ken-cli/tests/effect_composition_state_console_e2e.rs:51` | `ken-effect-composition-e2e` | `create_dir_all` |

**Two independent axes, and they disagree — which is why both are recorded:**

- **Acquisition: `accept-existing`, four for four, measured.** `create_dir_all`
  succeeds onto an existing entry, including a symlink pointing elsewhere, and
  each site then writes beneath the accepted root with a plain `fs::write`. The
  discriminating property is **how the path is acquired**, not how it is used:
  a site can be the only writer and still write through a path someone else
  created first.
- **Usage: enumerated 2026-08-14, and the residual is closed.** This entry read
  *"NOT ESTABLISHED — nobody has looked"*: the Steward had asserted
  single-writer-and-idempotent from site shape, and Runtime's read refused it
  under the bar the Steward had just set. Someone has now looked, per binary
  rather than per site: `cli_i1_entrypoint_abi.rs` uses 10 distinct fixture
  child names across its 10 tests, `fs_read_file_lines_flip_e2e.rs` 2 distinct
  across 2, and the other two binaries each run a single test writing one child
  (`rosetta.rs` sequentially, one child per slug). **No two tests in any of the
  four binaries write the same child path**, so the intra-binary route is closed
  by enumeration rather than by inference. The refusal was still the correct
  entry — it is what got the axis measured instead of assumed.

**Reachability: measured closed on both routes.** `scripts/ken-cargo` defaults
`KEN_BUILD_SLOTS` to 1 (`:15`) and takes the single-`flock` path (`:88-96`), so
no two seats run test binaries concurrently; the lock is pinned outside `TMPDIR`
(`:55-58`) specifically so two seats cannot resolve different locks. And
`TMPDIR` is redirected to `/workspaces/ken/tmp` (`:30-32`) — the repo volume,
not a shared system `/tmp` — so reaching these paths means already being inside
it. CI overrides `KEN_TMPDIR` per runner.

**What would reopen it, stated as a config change rather than an attacker:**
raising `KEN_BUILD_SLOTS` above 1, or running these suites outside `ken-cargo`.
That is the sentence worth having, because it names something someone may
deliberately do.

**Not a node.** No reachable route, three test fixtures, and no observed
failure. The value is the record: the next author choosing a forty-first name
inherits the fact instead of the reassurance.

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
