# WP frame — RT-NATIVE-TRACK0-REARM (Track 0 of the native carried-value program)

- Node: `docs/program/issues/RT-NATIVE-TRACK0-REARM.md`
- Program: `docs/program/issues/RT-NATIVE-CARRIED-VALUE.md`
- Owner: Runtime. Size: S. Capability tier: T2 (mechanical; design front-loaded).
- Inputs pinned @ origin/main `6425709fb4065db8b645c98c83376db6bd144b83`.
- Design authority: the Architect's native-program frame `evt_9kat78d438cb`
  (verified at that SHA via `git show origin/main:`).

## Why this is shovel-ready with no new mechanism

The first-order carried-observation mechanism — the (need,phase)-keyed
`EffectSeatClaimRoute` protocol at `crates/.../lowering/effects.rs:495-505` — is
already MERGED (ancestors of main: `ef32b6ced`, `569ba3d0d`). Its native
full-program rows are merely still `#[ignore]`d, and two native CI jobs are
vacuously green over a zero-test selection. This WP un-ignores and re-measures;
it introduces no lowering mechanism.

## Fixed inputs (measured @ 6425709fb)

- Stale first-order rows to un-ignore: `px8f_buffer_native.rs:203`,
  `px8f_write_partition.rs:354`, ResourceRelease half of `rt_parity_native.rs:694`.
- The oracle: `scripts/ci-ignored-sweep.py`, run via the workspace
  `--run-ignored=only` sweep (`ci.yml:142-166`). It catalogs every ignored row
  and verifies its expected-fail claim — run it FIRST; do not assert green from
  row prose.
- Vacuous CI jobs to re-arm/de-vacuum: native-write-partition (`ci.yml:250`),
  native-buffer (`ci.yml:299`). native-rt-parity (`ci.yml:352`) already runs
  non-vacuously.

## Deliverables, ACs, controls

See the node (`docs/program/issues/RT-NATIVE-TRACK0-REARM.md`) — deliverables 1-4
and AC-1/AC-2/AC-3/AC-SCOPE with their controls are stated there and are the
authoritative acceptance list. In brief: sweep-as-oracle, un-ignore the three
stale rows, re-arm the two jobs, drop `--no-tests=pass` per-binary once a binary
has zero ignored rows (NOT globally — that reds the board today), and remove
un-ignored rows from the sweep `expected` set.

## Contention check

Touches test files (`px8f_buffer_native.rs`, `px8f_write_partition.rs`,
`rt_parity_native.rs`), CI workflow (`ci.yml`), and the ignored-sweep expected
set. No overlap with the lane-2 language work (FoKripke / spec surface) or the
foundation catalog. Within the runtime lane it is contention-free with the
Track-1 design work (a separate representation decision, no shared files at this
stage). Workspace-green means green in CI, not a local `--workspace` run.

## No-regression

Native full-program rows that were expected-fail become green or advance to a
named Track-1 wall; no previously-green row may red. The `--locked` and
conformance gates run in CI.
