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

**AC-3 — the residue is accounted.** After the migration, enumerate every
remaining `env::temp_dir()` site and classify it as fixed-name or migrated.
**A count of unclassified sites must be zero.** Without this the next audit
cannot tell "examined and fine" from "missed."

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
