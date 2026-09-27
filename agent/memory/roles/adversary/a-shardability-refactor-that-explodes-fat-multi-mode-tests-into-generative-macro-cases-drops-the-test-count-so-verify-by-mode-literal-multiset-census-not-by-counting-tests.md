---
name: a-shardability-refactor-that-explodes-fat-multi-mode-tests-into-generative-macro-cases-drops-the-test-count-so-verify-by-mode-literal-multiset-census-not-by-counting-tests
description: When a duration/shardability refactor decomposes fat multi-mode negative-control tests into per-mode tests (generative macro cases, often process-isolated), the #[test] count drops and the deleted names are all negative controls, which reads as coverage loss. Verify by a mode-literal multiset census plus a machinery-token count, then check concurrency isolation, per-arm distinct expected messages, the shared helper's full conjunction, and child-process vacuity.
metadata:
  type: feedback
---

# Verify a mutation-suite shard refactor by mode-literal census, not test count

A sequential N-mode mutation loop decomposed into parallel or process-isolated
per-mode `#[test]`s can silently drop or weaken a mode, and a dropped negative
control is a false green that masks a future regression. The test count and the
named-function census both point the wrong way on a faithful refactor, so
neither can tell you. Two instances, both on
`crates/ken-cli/tests/rt_parity_native.rs`.

## Instance 1: the count fell and nothing was lost

**Measured 2026-08-28 on CI-NATIVE-PARITY-DURATION D1-D3 (`c555f843a`, range
`5fe12514b..1218bddfc`), `rt_parity_native.rs` + `.github/workflows/ci.yml`.**
A net -256-line rewrite. Six aggregate negative-control test fns were removed --
`checked_ih_continuation_inheritance_mutations_bite_their_own_arms`,
`checked_ih_generated_entry_admission_population_mutations_reject`,
`..._capsule_mutations_reject`, `..._confluence_and_route_mutations_reject`,
`..._per_arrival_operation_mutations_break_equality`,
`d1_route_control_full_program_mutations_are_fail_closed` -- and the
hand-written `#[test]` count fell 21 -> 18. Every removed name is a negative
control; the naive read is "the refactor deleted the mutation controls".

That read was wrong. What settles it:

- **Mode-literal multiset census.** Each old fat test looped over a fixed set
  of mutation MODES (match-arm strings / `$mode:literal` macro args). Extract
  every mode-ish literal both sides and diff the multiset:
  `git show <base>:<f> | grep -oE '"[a-z][a-z0-9-]+-[a-z0-9-]+"' | sort | uniq -c`
  vs the same on the tip. About 90 distinct modes (`wrong-slot`,
  `route-reversed`, `fresh-closure-body`, `drop-governed`,
  `duplicate-k-locator`, ...) appeared with IDENTICAL multiplicity. The only
  delta was 6 hand-written `"rt-parity-*-mutation-child"` thread-name constants
  replaced by the macros' `concat!("rt-parity-", stringify!($name), "-child")`
  (hence a new `"rt-parity-"` x3).
- **Machinery-token corroboration.** The count of the mutation-machinery token
  (`Mutation`) was identical at 96 both sides; the production seams
  (`with_checked_ih_*_mutation`, `*_mutation_is_exact()`) were all still driven.

The 6 fat tests were re-expressed as generative macros (`generated_entry_case!`,
`generated_entry_checked_case!`, `d1_route_case!`) emitting one `#[test]` per
mode, each run in an isolated child process -- a legitimate shardability
refactor (nextest shards by test and cannot subdivide one fat in-process test).

## Instance 2: parallel decomposition, cleared on isolation and fidelity

**Measured 2026-09-03 by the Adversary on CI-GATE-TIME-REDUCTION-D2**, squash
`4b9408b25` (PR #3263); NO OBJECTION, `evt_fmfj7sw4yg`. (Scope handling of the
rest of that WP is in
`roles/adversary/a-dispatched-m8-hunt-on-a-ci-tooling-only-candidate-is-answered-no-objection-on-scope-grounds-not-a-clean-product-hunt.md`.)
Two independent axes:

1. **Concurrency isolation.** The sequential -> parallel move is race-safe iff
   no shared mutable state leaks across the now-parallel tests. Confirm each
   test takes its own `tempfile::TempDir` (a per-name `output_dir` helper, not
   a shared path); mutations apply via scoped guards
   (`ken_runtime::with_*_mutation` RAII closures) and/or re-exec'd child
   processes reading a `Command` env var (`std::env::var_os($env)` +
   `current_exe()`); and grep proves no `set_var`/`remove_var`, no
   `static mut`, no shared-path `fs::write`, no `set_current_dir`. A parent
   that mutated process-global env or an unlocked static would race under
   parallelism -- that is the false-green bug; the child-process /
   thread-local pattern defuses it.
2. **Assertion fidelity.** Discrimination is preserved iff each per-arm macro
   invocation carries its own distinct expected-diagnostic string (there, 11
   `owner_body` mutations each asserting a unique message), not a shared
   generic "reds" -- a macro whose arms all assert the same green-vs-green is
   where a weakened arm hides -- AND the shared parity helper still asserts the
   full conjunction it did in the monolith (`assert_narrowed_alike`:
   `terminal_error == None` AND `terminal_exit` equality, not one half).

## Child-process vacuity audit

Each macro case, in parent mode, spawns
`Command::new(current_exe()).arg("--exact").arg(stringify!($name))` with an env
var set, then asserts on the child. If the child selects 0 tests it still exits
0 (libtest `--exact <nonexistent>` prints "0 tests", exit 0), so a parent
asserting only `status.success()` would pass asserting nothing.

- Non-vacuous in instance 1 because the macro is invoked at crate root, so
  `stringify!($name)` equals libtest's full test path and `--exact` matches
  exactly the case's own test. **Latent fragility:** nest the macro in a `mod`
  and the bare name no longer matches; the child selects 0 and the parent
  passes vacuously.
- The `_checked_case!` / `d1_route_case!` variants additionally assert
  `stderr.contains($marker)` (the "mutation reached its production seam"
  print), so they are immune. Only the plain no-marker variant depends on
  crate-root placement; flag it as latent.

## CI-side companion checks when the same change reshards the runner

A matrix job's `needs.<job>.result` collapses all legs to success-iff-all-pass
(`fail-fast: false`), so one required-aggregator entry still gates every shard;
confirm the resharded job keeps its four required places (shard-exclusion
filter, dedicated job, `needs:`, check-loop) per CI-SKIPPED-NATIVE-TESTS. At the
time, `--partition count:N/K` over a matrix `[1..K]` ran every bucket; it adds
by-design silent pass only for partially-empty buckets and does not change the
whole-binary zero-selection guard. (The round-robin partition was later
replaced by a duration-balanced plan with a union check,
`scripts/check-ci-shard-union.py`.)

## How to apply

For a perf, duration or shard refactor of a mutation- or parity-test suite:

1. Do NOT trust the `#[test]` count or disappeared-fn-name census: an explosion
   into macro cases moves tests off the grep that counts `#[test]`.
2. Census the MODE literals as a multiset both sides and assert equality;
   corroborate with an identical count of the mutation-machinery token. A
   missing mode is a real dropped control.
3. For a sequential-to-parallel move, check isolation (no process-global
   mutation) and fidelity (distinct per-arm expected messages; the shared
   helper's full conjunction).
4. For every generative macro that spawns a process-isolated child,
   `status.success()` alone is vacuous if 0 tests run; require a marker
   assertion, or confirm the `--exact` name is a guaranteed match and flag
   that dependency as latent.

Kin of [[a-negative-control-pins-only-the-check-that-alone-rejects-it]] (same
mutation suite; there a per-leg control was missing, here the census proved none
was), of
[[verify-an-out-parameter-or-sentinel-to-sum-type-trampoline-refactor-by-the-continuation-site-invariant-not-by-reading-each-mechanically-wrapped-arm]]
(same move: census the invariant set base vs candidate), and of the
verify-the-mechanism-not-a-proxy family (a test count is a proxy; the mode
multiset is the mechanism). Distinct from
[[a-negative-control-pins-only-the-check-that-alone-rejects-it]] (there a
control survives but is masked by an added conjunct; here controls are relocated
and the question is whether any mode vanished).
