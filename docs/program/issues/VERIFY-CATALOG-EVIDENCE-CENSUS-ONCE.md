---
id: VERIFY-CATALOG-EVIDENCE-CENSUS-ONCE
title: "The ken-elaborator lib's whole-catalog evidence census is built twice, and each build elaborates every catalog unit three times in series. It is the last work in the z3 job's whole-lib step, which has a 60-minute limit. On a slow runner the step is cancelled with one test outstanding, and that cancel holds every elaborator-touching candidate. Build the census once and drop the duplicate elaboration, with the census and every assertion unchanged"
status: closed
owner: kernel
size: S
tier: T2
gate: architect
depends_on: []
blocks: []
github: null
origin: "Measured CI cancels on PX10-PIDFD-CHILD-SIGNAL I1 (run 43736787210) and LANG-DELETE-LEGACY-RESOLUTION I1b-b (run 43743468649). Both stopped at 767 of 768 lib tests in the z3 job. Operator 2026-09-27: \"keep the 20-job cap; fit CI under it.\" Lane 2, Kernel ring (idle; the Verify ring takes CI-Z3-SCOPE-AND-FORMATTER-CORPUS-GATE). Steward-filed per COORDINATION section 2."
---

# Build the catalog evidence census once

**Closed without a candidate (Steward).** CI-Z3-SCOPE-AND-FORMATTER-CORPUS-GATE
(`759a4cb8c`) removes both census tests from the z3 job's lib step, which was
the limit this WP protected. They still run once in the shards.

## Objective

The z3 job's `--lib` step (`cargo test -p ken-elaborator --features
z3-process --lib`) builds the whole-catalog evidence census in
`r_layer_tests/lang_mod_catalog_evidence_frontier.rs` once per run, not
twice, without repeating an elaboration. The census rows and every
assertion stay as they are, so the step regains the headroom it needs
under its 60-minute limit.

## Settled inputs (main `4f24d43eb`)

- **Two builds.** `catalog_evidence_frontier_is_closed_and_honest`
  (`:892`) calls `build_census()` directly (`:898`), between reading the
  source bytes before and after (`:894`, `:918`). The only other
  consumer, `population_pin_rejects_omission_replacement_and_duplicate`
  (`:1521`), builds a second census through the `OnceLock` in
  `collect_census` (`:788`). Nothing outside this file calls either
  function.
- **Three elaborations per unit.** `build_census` (`:763`) elaborates
  each discovered unit from a fresh `ElabEnv` in `observe_strict`
  (`:280`), in `observe_legacy` (`:291`), and in `legacy_evidence`
  (`:582`). `legacy_evidence` makes the same
  `elaborate_module_from_roots` call as `observe_legacy` (`:598`, `:293`).
  All of this runs serially on one test thread.
- **Measured.** On the green run 43705015469 the step took 2103 s, and
  the census tests ran from about 11:00:50 to 11:15:05. In the cancelled
  runs 43736787210 and 43743468649, every earlier test ran about twice
  as slowly. The census test started about 49 minutes in, against about
  25, and `population_pin_...` was the one test still running at the
  cancel. The job's limit is set in `.github/workflows/ci.yml`, which
  this WP does not touch.
- **Repair hypothesis.** Build one census for both tests: for example,
  the `OnceLock` initialiser records the before-bytes and the census,
  and the byte-stability assertion compares against them. Derive the
  legacy observation from the elaboration `legacy_evidence` already
  makes, rather than elaborating the unit again. Parallelising units
  across worker threads is admissible only with a stated stack
  (`stated-stacks`). It is not required.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

`crates/ken-elaborator/src/r_layer_tests/lang_mod_catalog_evidence_frontier.rs`
only. No production code, no CI workflow, and no assertion, test name,
or `LoaderObservation` or row field changes meaning.

## Deliverable

One candidate on the Kernel ring.

## Acceptance

- **AC-1 (same census).** Locally, the `{row:#?}` lines that
  `catalog_evidence_frontier_is_closed_and_honest` prints are
  byte-identical on base and candidate, compared as one sorted file per
  side. Equal counts are not evidence, so compare identities.
- **AC-2 (one build, fewer elaborations).** A test-local counter, or an
  equivalent instrument at the elaboration call sites, shows one census
  build per test process and two elaborations per unit, against six on
  base. Run both census tests in one invocation, and report the wall
  time on both sides:

  ```sh
  M=r_layer_tests::lang_mod_catalog_evidence_frontier
  scripts/ken-cargo test -p ken-elaborator --lib -- --exact \
    $M::catalog_evidence_frontier_is_closed_and_honest \
    $M::population_pin_rejects_omission_replacement_and_duplicate
  ```
- **AC-3 (the population pin still bites).** Mutate the census to drop
  one unit's row. The mutant must redden
  `catalog_evidence_frontier_is_closed_and_honest`, and a mutant that
  edits a source byte inside the census must redden its byte-stability
  assertion. Restore the file byte-identically.
- **AC-4.** Full-workspace CI is green on the exact candidate, including
  the z3 job. Report that job's `--lib` step time from its log.

## Stop conditions

- Sharing one census requires an assertion to weaken, or the before and
  after byte reads to lose their bracket around the census.
- The legacy observation cannot be derived from the `legacy_evidence`
  elaboration without changing what `observe_legacy` records.

Checks:
- **3 fired.** `collect_census` and `build_census` occur only in this
  file, five occurrences. The census's two consumers are the tests at
  `:892` and `:1521`.
- **4 fired.** Each observation builds its own `ElabEnv::new()` (`:281`,
  `:292`, `:588`), so the elaborations are independent, and one can feed
  two observations only where the call is identical (`:293`, `:598`).
- **8 fired.** A census that silently drops or reorders a unit passes a
  row count. AC-1 compares the printed rows as identities, and AC-3
  proves that the population pin and the byte bracket still fire on the
  shared census.
