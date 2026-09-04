---
id: CI-IGNORED-SWEEP-NEXTEST-GROUND-TRUTH
title: "Fix scripts/ci-ignored-sweep.py to count ignored rows from `nextest --list --ignored` ground truth instead of a source-attribute grep, which cannot attribute a `#[ignore]` written as a macro-invocation leading token and so mis-counts (census 37 vs discovered 45). The ignored-row-sweep is a REQUIRED check (it sits in the build-test aggregator's needs; result != success sets failed=1), so this mis-count blocks any candidate that legitimately writes `#[ignore]`s in macro-leading-token position. TWO halves: (1) count from nextest ground truth; (2) once the count passes, the sweep's RUN (--run-ignored=only) proceeds and would execute the 8 deferred inert AC-EDGE-CONTROL-REKEY controls (fail-by-design), so a run-exemption registry for exactly those 8 (inc2-re-key expiry) is required to prevent a 5th red."
status: active
owner: runtime
size: XS
gate: none
tier: T2
depends_on: []
blocks: []
github: null
origin: "Steward, 2026-09-04. Surfaced as F2 of the RT-COMPOSED-RETURN-FORWARD-RET-EDGE b2 inc1 CI-red (lieutenant M5a evt_c8fe51vjrym7): the candidate legitimately moved 8 AC-EDGE-CONTROL-REKEY `#[ignore]`s into the macro's leading-token position (so nextest now correctly discovers them as ignored), but scripts/ci-ignored-sweep.py's source-attribute grep cannot attribute a macro-leading-token `#[ignore]` back to its owning file/row, so its census (37) diverges from nextest's discovery (45) by exactly that 8 — the exact INVERSE of the prior candidate's mismatch. Architect ruled it a scripts/ tool blind spot on his review surface, distinct from the RT mechanism (evt_2tfjhm4ybxgkk area); fix = count ignored from `nextest --list --ignored` ground truth. Runtime-leader (evt_hxd3b9vdcw4v) + runtime-implementer (evt_67wcarfm08at) BOTH verified from ci.yml source that the sweep is a BLOCKING required check despite its '(findings non-blocking)' display name. Steward SEQUENCING call: this lands FIRST as its own standalone scripts/-only candidate; the RT b2 inc1 re-spin 8c2761be then re-runs CI on top of the updated main (not folded — unrelated files, keeps the already-long RT candidate focused). Built by the runtime-implementer (holds the F2 diagnosis + investigation; it unblocks their own RT candidate) and reviewed by the Architect (his scripts/ ruling evt_2tfjhm4ybxgkk-area + review surface — the ideal reviewer, which is why he does NOT build it) plus CI green. Steward corrected an initial mis-assignment to the Architect-as-builder (evt_277w84f978da6 -> evt_4gf06s95b0y9t) because that put the ideal reviewer in the builder seat, creating the build-review gap."
---

> # RELEASED (Steward, 2026-09-04) as a standalone scripts/-only prerequisite that
> # unblocks the RT-COMPOSED-RETURN-FORWARD-RET-EDGE b2 inc1 re-spin (8c2761be) from
> # a guaranteed CI red on the required ignored-row-sweep check. BUILT by the
> # runtime-implementer (holds the F2 diagnosis; unblocks their own RT candidate);
> # REVIEWED by the Architect (scripts/ ruling + review surface); CI green. Steward
> # M1-M4 -> lieutenant. This lands
> # BEFORE 8c2761be's CI re-run: the Steward does not route 8c2761be until this is on
> # main, so 8c2761be's CI reads the fixed sweep.

## What the defect is

`scripts/ci-ignored-sweep.py` derives its "ignored rows" census by grepping test
source for `#[ignore]` attributes and attributing each to its file/row. That
attribution FAILS when a `#[ignore]` is written as the leading token of a macro
invocation (`$(#[$attr:meta])*` capture position) — the grep cannot resolve it to
the owning file/row. So a candidate that legitimately writes `#[ignore]`s in that
position produces a census count that diverges from the true suppressed set.

The RT b2 inc1 candidate moved 8 `AC-EDGE-CONTROL-REKEY` `#[ignore]`s into the
macro's leading-token position (a CORRECT fix — nextest now discovers them as
ignored). The sweep then reported census 37 vs nextest discovery 45 (the 8
macro-token ignores uncounted by the grep). Because the sweep is wired into the
required `build-test` aggregator (`needs`; `result != "success"` => `failed=1`),
this mis-count fails the required check.

## Deliverable — TWO halves (both required; the second prevents a 5th red)

The ignored-row-sweep does not only COUNT ignored rows, it also RUNS them
(`ci.yml:206`, `cargo nextest run --run-ignored=only`). Today `verify-list` aborts
(`set -e`) on the 37-vs-45 count mismatch BEFORE the run. Fixing only the count
lets the run proceed and execute the 8 `AC-EDGE-CONTROL-REKEY` capsule ignores,
which are inert-under-read BY DESIGN (that is why they are deferred) and FAIL if
run — the 5th red. So the candidate must carry both (implementer's catch,
`evt_1ta793z9jx15f`; Architect scope ruling `evt_5j8jcjpant1ca`):

1. **Count from ground truth.** Replace the source-attribute grep census
   (`ignored_attribute_count` / `ignored_test_reasons`, the git-greps at ~`:301`/
   `:518` that cannot see a macro-leading-token `#[ignore]`) with the `nextest
   --list --ignored` count, so a `#[ignore]` in any syntactic position is counted
   by the same authority that suppresses it.
2. **Run-exemption registry for exactly the 8 deferred controls.** Exempt exactly
   the 8 `AC-EDGE-CONTROL-REKEY` deferred inert controls from the sweep's RUN via a
   named registry (e.g. `.github/ignored-test-exemptions.toml`), class "deferred
   inert control, do not run until inc2 re-keys", carrying the inc2-re-key EXPIRY.
   Not a blanket "skip ignored from the run" (that would blunt the sweep's power to
   catch a rotted ignored test).

## Acceptance criteria

- **AC-GROUND-TRUTH.** The census counts ignored rows from `nextest --list
  --ignored` (the suppression authority), not a source grep. A `#[ignore]` written
  as a macro-leading token is counted.
- **AC-CENSUS-AGREES.** On a tree containing macro-leading-token `#[ignore]`s (the
  RT b2 candidate's 8 `AC-EDGE-CONTROL-REKEY` rows), census == discovery (no
  37-vs-45 divergence). The `verify-list` count check passes.
- **AC-STILL-CATCHES (count).** The sweep still reds on a genuine ignored-row
  discrepancy (a `#[ignore]` present in one authority but not the other) — the
  count fix must not make the check vacuous.
- **AC-RUN-EXEMPT-EXACT.** The run-exemption covers EXACTLY the 8 named
  `AC-EDGE-CONTROL-REKEY` controls (each enumerated), not a blanket ignored-run
  skip; it carries the inc2-re-key expiry so it is not permanent.
- **AC-RUN-STILL-RUNS.** A non-exempt ignored row is STILL run by the sweep — the
  exemption must not blunt the run's power to catch a rotted ignored test
  (non-degenerate against a blanket skip).
- **AC-CONFIG-ONLY.** Diff is confined to `scripts/` + `.github/` CI config (the
  sweep script, the exemption registry, any `ci.yml` wiring). No `crates/`,
  `catalog/`, `spec/`, or kernel/TCB path.

## Reviewers

Builder: runtime-implementer (holds the F2 diagnosis + investigation; it unblocks
their own RT candidate). Reviewer: Architect (his scripts/ review surface + his fix
ruling — the ideal reviewer, which is why he does NOT build it, avoiding the
build-review gap; §8a makes scripts/ his REVIEW surface, not authoring).
Independent mechanics reviewer: runtime-qa (saw the census failure firsthand) or
verify-qa. Plus CI green on the exact SHA. scripts/ + .github/ config only, no TCB:
light gate. Merge via Steward M1-M4 -> lieutenant.

Architect review bar (`evt_5j8jcjpant1ca`), non-degenerate on both axes: (1) the
count is nextest-ground-truth (macro-generation-agnostic) AND still fails on a
genuine ignored-row drift that is not the macro-attribution artifact; (2) the
run-exemption is scoped to EXACTLY the 8 named deferred controls (not a blanket
skip that would blunt the sweep) AND carries the inc2-re-key expiry.

## Sequencing

Lands FIRST, before the RT b2 inc1 re-spin `8c2761be` re-runs CI. The Steward does
not route `8c2761be`'s git_request until this fix is on `main` (so `8c2761be`'s CI
reads the fixed sweep). The RT candidate's review work (runtime-qa mechanics,
Architect tie/sentinel verify) proceeds IN PARALLEL — it has no dependency on this
fix; only the final CI-green + merge ordering does.
