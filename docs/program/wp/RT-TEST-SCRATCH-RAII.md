# RT-TEST-SCRATCH-RAII — test scratch directories that clean themselves up

Owner: runtime. Size: M. Node: [[RT-TEST-SCRATCH-RAII]].
Fixed inputs measured at `origin/main` = **`24933da4`**. Re-derive your
merge-base from `origin/main`; **do not take a SHA from this frame.**

**Seat tier: T2 build ring.** Architect votes at merge. **No Spec vote** —
this is test infrastructure and touches no runtime behavior.

## What this deliverable is

A shared RAII scratch-directory helper that removes its directory on drop, and
the leaking fixture sites migrated onto it.

**This is a test-hygiene fix with a delivery justification, not housekeeping.**
The leak has filled `/workspaces/ken` to 100% seven times. Today it cost a
Runtime implementer a 38-minute turn and left it idle with two deliverables
outstanding, and it produced three `.git` lock-write failures inside a
publisher run. The node exists because that keeps happening, not because the
directory is untidy.

## The design judgment, front-loaded

**Sort the 42 sites before you estimate.** `std::env::temp_dir()` appears 42
times across `crates/ken-runtime/src/` and `crates/ken-cli/tests/`, and **they
are not all the same defect**:

| shape | example | leaks? |
|---|---|---|
| fixed name | `temp_dir().join("ken-rosetta-runner")` | **no** — reused across runs |
| interpolated | `format!("ken-runtime-{name}-{nanos}")` | **yes** — fresh dir every run |

**Only the second group is the node.** The canonical instance is
`temp_output_dir` at `native_execution_differential.rs:3302`, which returns a
bare `PathBuf` with a nanosecond suffix and no cleanup on any path.

> ### `std::env::temp_dir()` IS NOT THE ONLY AXIS, AND THE OTHER ONE LEAKS MORE PER RUN
>
> **Added 2026-08-13 after the eighth recurrence.** The enumeration above, and
> `AC-3`'s residue census below, were both keyed on `std::env::temp_dir()`.
> **The site that filled the volume that day does not call it.**
>
> `r3_4b_observation_feature_is_native_artifact_identical` — the two-compilation
> identity control landed by `RT-4B-OBSERVATION-FEATURE-GATE` — keys its scratch
> on `std::process::id()` under **`CARGO_TARGET_TMPDIR`**, and removes nothing.
> Because it is an *artifact-level* A/B it must not share a Cargo target
> directory, so each run leaves **two full target trees**. Ten accumulated runs
> across implementer and QA were **11G**, against `/workspaces/ken` at 97% used.
>
> ⇒ **Enumerate on BOTH axes.** A grep for `temp_dir` returns zero hits on the
> single largest leaker in the tree, and the direction of that failure is the
> bad one: `AC-3` would report a complete residue census over a population that
> never contained it.
>
> **The per-run mass, not the per-run count, is what makes this the priority
> case.** The `temp_dir` sites leak ~1200 small directories an hour; this one
> leaks two Cargo target trees per invocation. Both fill the volume; only one is
> visible to a directory count.
>
> **It is partly covered already, and you must not treat that as done.**
> `RT-4B-UNIQUENESS-GATE-REACH` carries `AC-9`, which cleans this one site up,
> because that node's `AC-6` re-runs the control and re-arms the leak by
> construction. **`AC-9` is a point fix on one fixture; this node owns the
> class.** If `AC-9` has already landed when you start, the site is migrated
> rather than absent — classify it, do not skip it.
>
> **Keep the failure artifacts on failure.** For this fixture the two divergent
> target trees *are* the evidence when the identity control reds. Remove on
> success; preserve on failure and say where. That is a real instance of the
> deliberate-outlive question the node already flags as open.

**`Drop`, not an explicit cleanup call.** A scratch directory removed at the
end of a test body leaks precisely when the test fails — which is when runs are
most frequent and the volume is under the most pressure. `Drop` runs during
unwind; a trailing statement does not.

**The dependency choice is yours.** `tempfile::TempDir` is conventional and
already has the semantics, at the cost of a new dev-dependency; a local `Drop`
guard adds no dependency and makes you write the panic path. **Pick one and
state why in one sentence.**

## Deliverables

**D1 — the helper**, with removal on drop including the unwind path.

**D2 — the leaking `ken-runtime` sites migrated.**

**D3 — the leaking `ken-cli` test sites migrated**, or an explicit statement
that they belong in a sibling node and why. **Do not migrate half of them
silently.**

**D4 — the fixed-name sites left alone**, with a one-line note saying they were
examined and do not leak. This is what stops the next pass re-auditing them.

## Acceptance criteria

**AC-1 — a passing test leaves no directory behind.** Count matching entries
under `std::env::temp_dir()` before and after a named fixture run; the delta is
zero. **Assert the delta, not that the count is small.**

**AC-2 — a FAILING test also leaves nothing behind.** Force a failure inside a
migrated fixture and show the same zero delta. **This is the AC the node exists
for** — the success path is the easy half and the failure path is where the
mass came from.

**AC-3 — the residue is accounted, ON BOTH AXES.** After the migration,
enumerate every remaining `env::temp_dir()` site **and** every
`CARGO_TARGET_TMPDIR` site, and classify each as fixed-name, migrated, or
deliberately outliving its test. **A count of unclassified sites must be zero,
and the report must state both populations separately.** Without this the next
audit cannot tell "examined and fine" from "missed" — and a census over one
axis alone reports complete while missing the largest leaker, which is exactly
how this node reached its eighth recurrence.

**AC-4 — no fixture's assertions changed.** This is a lifetime fix. If a test's
expected values move, stop and say why.

**AC-5 — the A/B.** Revert the helper on one fixture and show the directory
survives the run; restore and it does not.

**AC-6 — no runtime or CLI behavior change**, no `spec/` edit,
`trusted_base()` unchanged.

## Excluded scope

- **No reclaim tooling.** The reclaim procedure is the Steward's operational
  record and does not belong in `crates/`.
- **No `TMPDIR` redirection.** Pointing scratch at the container `/tmp` moves
  the mass off the volume that fills without fixing the leak. It is a
  mitigation and it is not this node; taking it instead of the RAII fix would
  hide the defect rather than close it.
- No change to what any fixture asserts, and no performance work.

## Stop conditions — return to me, do not decide

- **A fixture deliberately outlives its test** — kept for post-hoc inspection
  of a failure. That is a legitimate use and blanket RAII would delete the
  artifact a debugging session wants. It needs an explicit opt-out, and I want
  to see which sites claim it.
- **The `ken-cli` sites need a different helper** rather than the same one.
- **Migration turns out to change a fixture's observable behavior** — that
  would mean a test was depending on the leak, which is a finding.

## Contention

`crates/ken-runtime/src/` and `crates/ken-cli/tests/`. **Check the intersection
against whatever D2j work is live** — [[RT-LEXICAL-RECURSOR-CONSUMERS-D2j]]
touches planning paths, not these fixtures, but re-derive it at candidate time
rather than trusting this sentence.

## Sizing and validation

`scripts/ken-cargo test -p ken-runtime` and the affected `ken-cli` suites.
**Never `--workspace`**; that is CI's gate. **Run AC-1's before/after count
with the volume in a known state** — a concurrent seat's fixtures will
otherwise appear in your delta and read as a leak you did not fix.
