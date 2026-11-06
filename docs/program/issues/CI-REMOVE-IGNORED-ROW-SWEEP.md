---
id: CI-REMOVE-IGNORED-ROW-SWEEP
title: "Remove the ignored-row sweep from CI. The issue-tracker job compiles every workspace test binary on one standard runner to run the ignored rows, and it filled that runner's disk twice on PR #5196. Keep the job's work-item tracker checks and its check name; delete the sweep steps, the sweep script, its test and the exemption registry"
status: merged
owner: verify
size: S
tier: T2
gate: none
depends_on: []
blocks: []
github: null
origin: "Operator 2026-11-05: \"we don't need the ignored row sweep anymore. remove that part of the ci workflow.\" Steward-filed per COORDINATION section 2; the Steward does not edit .github/workflows (ken-steward section 7), so the Verify ring builds it."
---

# Remove the ignored-row sweep

## Objective

The `issue-tracker` job in `.github/workflows/ci.yml` keeps its work-item
tracker checks and drops the ignored-row sweep. No CI job runs
`--run-ignored=only` any more.

## Settled inputs (main `92addd6de`)

- **The job.** `issue-tracker` (`ci.yml:785`), named `work-item tracker and
  ignored-row sweep`, runs on `ubuntu-latest`. Its tracker steps are the issue
  schema, the test environment pins, and `gen-progress.sh --check`.
- **The sweep.** Two steps gated on `needs.classify-paths.outputs.mode ==
  'full'`: `Install cargo-nextest for ignored-row sweep` (`:809`) and `Select
  and run ignored rows` (`:814`). The second runs `cargo nextest list` and
  `cargo nextest run --workspace --run-ignored=only`, which builds every
  workspace test binary on the one runner.
- **Its only consumers.** `scripts/ci-ignored-sweep.py` (700 lines) is called
  only from that step. `scripts/test-ci-ignored-sweep.py` tests it and is not
  run by CI. `.github/ignored-test-exemptions.toml` is read only by the sweep
  script.
- **The check name.** The aggregate job (`ci.yml:620` to `:680`) reads
  `needs.issue-tracker.result`, and three relay jobs list `issue-tracker` in
  `needs`. The check context may be a required status check; branch
  protection is not readable from the fleet's token.

## Deliverable

One candidate that:

1. deletes the two sweep steps from `issue-tracker`;
2. deletes `scripts/ci-ignored-sweep.py`, `scripts/test-ci-ignored-sweep.py`
   and `.github/ignored-test-exemptions.toml`;
3. leaves the job id `issue-tracker`, its `name:`, its `needs:`, its tracker
   steps, its resource-monitor steps and every consumer of its result
   unchanged, so no required check context moves.

Not built: renaming the check, a replacement ignored-row gate, or edits to
comments elsewhere that mention the sweep.

## Acceptance

- **AC-1.** `grep -rn 'run-ignored\|ci-ignored-sweep\|ignored-test-exemptions'
  .github scripts` is empty.
- **AC-2.** On the candidate's PR, the `work-item tracker and ignored-row
  sweep` check passes with no nextest step in its log, and the aggregate job
  reports it as success.
- **AC-3.** The candidate's full-mode CI run is green.

## Stop conditions

- Any other job or script reads the deleted files.
- Removing the steps changes a required check's name or result wiring.

Checks: 2 fired: the sweep's fan-in was enumerated by grep across
`.github`, `scripts`, `crates` and `conformance`. The script is called only
from `ci.yml:818` to `:837`; the registry is read only by the script, and a
kernel test comment mentions it.
