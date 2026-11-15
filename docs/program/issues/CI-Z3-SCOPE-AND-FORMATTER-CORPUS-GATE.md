---
id: CI-Z3-SCOPE-AND-FORMATTER-CORPUS-GATE
title: "The z3 CI job runs the whole ken-elaborator lib suite with the z3-process feature on, so its 60-minute limit is spent on census, formatter and catalog tests that never reach the Z3 route and already run in the shards. Every elaborator-touching candidate is cancelled on a slow runner. Narrow that step to the tests that can reach the Z3 route, and run the formatter whole-corpus tests only when a formatter input changes"
status: active
owner: verify
size: M
tier: T1
gate: steward
depends_on: []
blocks: []
github: null
origin: "Operator 2026-11-15: \"sounds good. you have permission to change ci.yml as above, or to delegate as you deem fit\", accepting options (a) and (b) of the Steward's slow-test brief. The operator designated the Verify ring as author of this workflow change under COORDINATION 14a. Lane 2. Steward-filed per COORDINATION section 2."
---

# Let the z3 job test Z3

## Objective

The `optional z3 process adapter` job runs the adapter integration test,
plus the `ken-elaborator` lib tests whose execution can reach the Z3
route, with `z3-process` on. It does not rerun the rest of the lib suite,
which the shards already run. The two formatter whole-corpus tests run in
CI only when a formatter input changes. No test leaves CI otherwise, and
none is marked `#[ignore]`.

## Settled inputs (main `4f24d43eb`)

- **The job.** `z3-process-adapter` (`.github/workflows/ci.yml:559`,
  `timeout-minutes: 60`) runs `--test v3_z3_process_adapter`, then
  `cargo test -p ken-elaborator --features z3-process --lib` (`:580`).
  Its comment says to keep the lib step whole, because `prover.rs`
  routes the Route-D fallthrough to z3. This WP replaces that rule under
  the operator ruling above. The `build + test` aggregator (`:614`) fails
  when this job's result is not success.
- **The Z3 route.** `prover.rs:452` calls `attempt_d_with_z3_process`
  (`:539`) under `#[cfg(feature = "z3-process")]`, which reaches
  `z3_process::candidate_assignment`. The module is compiled only with
  the feature (`lib.rs:59`).
- **Measured.** Runs 43736787210 and 43743468649 were cancelled at 767
  of 768 lib tests, with the catalog evidence census test outstanding.
  The green run 43705015469 took 2103 s for the step. Its long tests are
  the census, `kenfmt_b3_layout::ac7_whole_catalog_is_parse_preserved_idempotent_and_width_bounded`,
  `kenfmt_b4_splicing::ac6_whole_literate_corpus_preserves_prose_ast_and_idempotence_read_only`
  and the catalog acceptance rows. None of them calls the prover.
- **The shards.** The shards use nextest, with duration-balanced
  partitions from `scripts/ci-duration-shard.py`. Their union is checked
  by `scripts/check-ci-shard-union.py`. The exclusion comment at
  `ci.yml:229-247` is the existing pattern for keeping a test out of the
  shards on purpose.
- **The gate pattern.** `scripts/ci-z3-relevant.py`, with its tests in
  `scripts/test-ci-z3-relevant.py`, is a fail-safe changed-path gate. It
  prints `false` only when every changed path is known to be irrelevant.
- **Constraints.** The 20-job cap stands (operator 2026-09-27: "keep the
  20-job cap; fit CI under it"). No test may be ignored (operator
  2026-09-17 and 2026-10-17).

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

`.github/workflows/ci.yml`, the CI scripts it runs and their script
tests. A test-local instrument in `crates/` is allowed only for the
measurement in AC-1, and is not committed. No production code and no
test assertion changes.

## Deliverable

One candidate on the Verify ring.

1. **Z3 scope.** The z3 job's lib step selects, by a stated rule, every
   lib test that can reach `attempt_d_with_z3_process`. A nextest filter
   or a module filter are both acceptable. The adapter integration test
   stays.
2. **Formatter corpus gate.** A fail-safe path gate, in the
   `ci-z3-relevant.py` manner, decides whether the two kenfmt
   whole-corpus tests run. It is true for any change to a formatter,
   lexer, parser, layout, literate or resolver input that those tests
   reach, and for any error. When it is false, the shards skip exactly
   those two tests, and the shard-union check accepts that skip and
   nothing else.

## Acceptance

- **AC-1 (the population is measured).** On base, run the lib suite
  with the feature on and with a temporary counter at the Z3 route
  entry. Record the set of tests that reach it. The candidate's
  selection contains every test in that set. Report the set, the rule,
  and how a test added later that reaches the route is either caught by
  the rule or is a stated residual.
- **AC-2 (the gate fails safe).** Script tests show:
  - a formatter-input path gives true;
  - a catalog-only change gives false;
  - an unreadable path list or metadata gives true.
  A mutant that maps a formatter input to false reddens one of them.
- **AC-3 (nothing else is dropped).** `check-ci-shard-union.py` passes
  with the gate true and with it false. A mutant that skips one test
  other than the two formatter tests reddens it.
- **AC-4.** Full-workspace CI is green on the exact candidate. Report
  the z3 job's lib-step time and the longest shard's time. Report from
  the log whether the gate ran the formatter tests on this candidate.

## Stop conditions

- The set of tests reaching the Z3 route cannot be measured, or cannot
  be selected without a list that silently goes stale.
- The gate needs a job beyond the 20-job cap, or any test must be
  ignored.
- A required change falls in `crates/` production code.

Checks:
- **2 fired.** The property the z3 job protects is the feature-on Route-D
  path. Its fan-in is every lib caller that reaches `prover.rs:452`, so
  AC-1 measures that population rather than trusting test names.
- **6 fired.** "These tests never reach Z3" is a claim over the lib
  suite. AC-1 counts what arrives at the route entry, instead of
  inferring it from module names.
- **8 fired.** A green z3 job is not evidence of coverage, since an
  empty selection is also green. AC-1 ties the selection to the measured
  set, and AC-3's mutant shows that the union check still separates a
  dropped test from the permitted skip.
