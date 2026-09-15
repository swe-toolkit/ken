---
scope: fleet
audience: (see scope README)
source: 2026-09-15, PR #3676 census — "main is green" was read as a base
  measurement by three seats, while every test shard and every native-slow job
  had been skipped by path classification in the green runs and cancelled in
  the only two runs that executed them at all
---

# A workflow green can mean the jobs that matter were SKIPPED, not that they passed

**Measured 2026-09-15.** `#3676` was red and the question was whether its
failures were its own regressions. Main's last runs all read `success`, so main
looked like a clean base. It is not a base at all:

    latest main runs, conclusion success   test shard 1..8/8      SKIPPED
                                           ALL native-slow jobs   SKIPPED
                                           job names still read "${{ matrix.shard }}"
                                           -- the name template was never
                                              substituted

    only 2 of the last 60 main runs        every test shard       CANCELLED
    executed those jobs at all             every rt_parity shard  CANCELLED
                                           px8f_write_partition   success

**The seventeen failing checks partition exactly, and the partition is the
finding:**

    15   test jobs, NO base measurement    8 test shard + 6 rt_parity
         anywhere in sixty main runs       + px8f_buffer_native
     1   test job WITH a real base         px8f_write_partition
                                           (success on main, fails on candidate)
     1   NOT A TEST                        build + test -- the roll-up

⇒ **For the fifteen, whether they were the candidate's regressions or standing
main-wide breakage was unestablished in EITHER DIRECTION.** Exactly one job had
a real base, and because it was the only one carrying evidence it was mistaken
for the whole story — a **selection effect**, not a diagnosis.

⇒ **The seventeenth is the roll-up, and it belongs to neither class.** It runs
on `main` every time and succeeds in two to six seconds — but it succeeds *by
accepting the skips* (below), so it is not a base measurement of anything. A
job that always runs and never exercises the code is the same blindness as one
that never runs, wearing the opposite appearance.

**The green was produced by the jobs not running, and the mechanism is
deliberate.** Verified at `origin/main`: nine jobs carry
`if: needs.classify-paths.outputs.mode == 'full'`, and `classify-paths` runs
`scripts/ci-doc-only.py`, which returns `doc-only` when every changed path is
under `docs/`, `agent/` or `library/` and outside the deny list (`crates/`,
`spec/`, `catalog/`, `conformance/`, `.github/`, `scripts/`,
`docs/program/evidence/`). The roll-up even codifies it — `ci.yml:504` passes a
`skipped` result **on purpose** when the mode is `doc-only`.

⇒ **The filter is not a bug and must not be "fixed".** Its logic is sound: a
doc-only push cannot break `crates/`, and removing it pays full native cost on
every documentation commit. **The entire defect is in what gets read off the
result.** "Main is green" is heard as *the suite passes*; it means only
*nothing this push could break, broke*.

**The remedy is therefore additive, not a repair: a scheduled full run on
`main`.** That manufactures the standing base measurement, leaves the PR-side
cost optimization intact, and is the cheapest thing that makes a red candidate
classifiable at all.

**THE DOC TRACK IS A GENERATOR OF UNINFORMATIVE GREENS.** It runs concurrently
by standing operator exception *precisely because* it touches `agent/` and
`library/` rather than `crates/` — which is exactly the condition that
classifies `doc-only`. So every doc-track publish adds another green run with
the native suite skipped, and it has been running all week. **This very lesson
is `agent/`-only and lands the same way**: the artifact recording the invisible
suite is published through the mechanism that makes it invisible.

## The tells, in the order they are cheap

- **A job conclusion of `skipped` is not a pass.** `gh pr checks` and the run
  summary present a skipped required job indistinguishably from a passing one
  at the run level; only the per-job conclusion separates them.
- **A literal `${{ matrix.shard }}` in a job name means that job never ran.** A
  job reported as `test shard ${{ matrix.shard }}/8` was skipped; a real one is
  named `test shard 3/8`. This is the fastest signal in the whole listing.
  **It is a symptom, not a defect** — the matrix is well-formed
  (`shard: [1,2,3,4,5,6,7,8]`); GitHub simply reports a skipped matrix job with
  its name template unsubstituted, because nothing ran to substitute it.
- **A roll-up check is not the thing it is named after.** `build + test` here
  has one step, `All test jobs passed`, and finishes in 3 seconds. It is a
  downstream aggregator; its failure carries no independent cause, and its
  *name* invites you to report a compile failure that did not happen.

## How to apply

- **Before calling a ref a base, confirm the specific jobs EXECUTED on it.**
  Not the run conclusion — the per-job conclusion for the jobs you are
  comparing:
  ```sh
  gh api "repos/<o>/<r>/actions/runs/<id>/jobs?per_page=60" \
    --jq '.jobs[]|select(.name|test("<job pattern>"))|"\(.conclusion)\t\(.name)"'
  ```
- **"Is it red on main?" has three answers, not two:** passes, fails, and
  **never ran there**. The third is the common one for expensive
  path-gated suites, and it is the one that silently becomes "it passes."
- **When exactly one job in a failing set has a base measurement, expect it to
  look decisive and distrust that.** It is the only one that *could* produce
  evidence; its prominence is an artifact of the instrument.
- **AN EXPLANATION THAT FITS IS THE THING THAT STOPS YOU RUNNING THE CENSUS.**
  The census that settled all of this was four tool calls. Three seats skipped
  it, and not one of us was being lazy — each had a hypothesis that accounted
  for the evidence in hand. **The tell you can notice in the moment is the
  feeling of having understood it**, arriving before you have enumerated the
  population. Treat that feeling as the cue to enumerate, not as the result.

**THIS FILE IS ITSELF THE FIFTH INSTANCE OF ITS OWN NEIGHBOUR.** The commit
that corrected the mechanism disowned the phrase *"the matrix never expanded"*
in its message, replaced it in the tell at the bottom, **and left it standing
in the evidence block twenty-five lines above** — where a reader meets the
disowned wording first. The lesson it failed to sweep against is
[[a-new-criterion-is-applied-forward-and-never-swept-backward-over-the-document-that-states-it]],
which landed in the very commit this branch was cut from. **Cutting a
correction obliges one pass over the whole containing document for the phrasing
you just disowned**, not only over the sentence you came to fix.

**The scope this failed at.** Three seats read the same green — a build leader,
the Architect, and the Steward — and none of us asked whether the suite had
run. It is
[[a-claim-inherits-the-scope-of-the-site-you-checked-not-the-scope-you-stated]]
applied to CI, and it is the workflow-scale form of
[[libtest-reports-ok-for-zero-executed-tests-so-a-green-probe-may-have-run-nothing]]:
**the criterion is `failed == 0`, and zero executed jobs satisfies it by
construction.** Related:
[[a-green-pilot-is-not-evidence-for-a-shape-it-never-produced]],
[[ci-history-answers-does-it-pass-on-main-for-free-but-read-the-job-conclusion-not-the-runs]].
