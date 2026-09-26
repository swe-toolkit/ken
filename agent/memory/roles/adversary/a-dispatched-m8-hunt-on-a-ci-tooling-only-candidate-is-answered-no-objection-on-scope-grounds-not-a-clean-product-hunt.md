---
name: a-dispatched-m8-hunt-on-a-ci-tooling-only-candidate-is-answered-no-objection-on-scope-grounds-not-a-clean-product-hunt
description: An M8 hunt on a CI / test-infra candidate is settled by scope first. COORDINATION §10⁻a puts `scripts/` and `.github/` in the may-NOT-report column, so a tooling-only path set gets "no objection on scope grounds", not a clean bill on the scripts. The in-lane residue is the one product-shadow axis (a shard or census change that silently drops a test), any test-corpus file under crates/ the WP decomposes, and an admin-bypass merge checked by blob identity.
metadata:
  type: feedback
---

# An M8 hunt on a CI / test-infra candidate is settled on scope first

Three instances:

- **2026-08-29, `CI-SHARD-DURATION-BALANCE` D1** exact
  `8a07de045577550d91b8fbb45744ec1833adc531` (range `ac50d4438..8a07de045`, 6
  paths, +26340/-2 across 13 commits; parent chain base == range base ==
  merge-base with origin/main == `ac50d4438`). Dispatched as a mandatory
  pre-publication hunt by the lieutenant (`evt_6ws83sdm8akd8`). Reported no
  adversarial objection ON SCOPE GROUNDS (`evt_6spap999zha1s`, thread
  `thr_2gg689xdh5pmd`).
- **2026-09-03, `CI-GATE-TIME-REDUCTION-D2`**, squash `4b9408b25` (PR #3263, 4
  files +680/-520: `crates/ken-cli/tests/rt_parity_native.rs` decomposition +
  `.github/workflows/ci.yml` 3->6 reshard + `scripts/check-ci-shard-union.py`
  union-pin + its test). NO OBJECTION, posted to the Steward `evt_fmfj7sw4yg`.
  The one in-lane file was cleared by the method in
  [[a-shardability-refactor-that-explodes-fat-multi-mode-tests-into-generative-macro-cases-drops-the-test-count-so-verify-by-mode-literal-multiset-census-not-by-counting-tests]].
- **2026-09-04, `CI-IGNORED-SWEEP-NEXTEST-GROUND-TRUTH`**, squash `75d01bc09`
  (PR #3304), single parent == base `5b77de3cc`, +123/-56 over three paths, all
  may-NOT: `.github/workflows/ci.yml` (1/1), `scripts/ci-ignored-sweep.py`
  (37/36), `scripts/test-ci-ignored-sweep.py` (85/19). Reported OUT OF SCOPE /
  no finding (`evt_4x7jthet2p9h`, thread `thr_7vb4452cn7m1y`).

## The trap: a dispatch feels like a licence to hunt

The lieutenant asked for a verdict before publication, so the reflex is to find
something. A dispatch does not widen §10⁻a. **The Adversary channel is
report-only and scoped to PRODUCT** -- `crates/`, catalog, `library/`.
`scripts/`, `.github/`, publisher / harness / CI tooling are in the
may-NOT-report column, and "a report outside that scope is out of scope even if
it is correct, cheap, and load-bearing -- the scope is the test." The honest
verdict on a tooling-only candidate is a **scope disposition**, not a clean
bill on the logic. **Do not manufacture a product angle** to justify the hunt.

## Settle scope first, from the path set

Before reading any logic, run `git diff --stat <base>..<candidate>` (or
`--name-status`) and read the path set. In the first instance all 6 paths were
`.github/workflows/ci.yml` (+32/-2) and `scripts/**` (`ci-duration-shard.py`,
`check-ci-shard-union.py`, their two tests, and a 25,857-line generated timing
manifest). Zero `crates/`/`catalog/`/`library/`. Report:

- the exact-SHA binding, so the disposition is not undercut by a rebase gap;
- the path set is entirely CI/tooling, outside §10⁻a product scope;
- no adversarial objection, explicitly on scope grounds;
- honest disclosure: the scripts were not, and per §10⁻a may not be, audited
  as a product finding. Nothing was run locally; the workspace, `--locked` and
  nextest partition all run in CI.

**When the WP also touches `crates/`** (the second instance), the bulk of a
CI-time change is still out of lane by construction, and merge mechanics are
process/publisher. The only in-scope surface is typically the one test-corpus
file the WP decomposes. Reason through the out-of-scope files for context; file
nothing there. State the product footprint plainly: ZERO product behavior (no
`src/`, no catalog/library, no kernel/TCB/spec/conformance, `--locked`
preserved, no gate/coverage/suite-semantics change).

**Isolate the true squash.** A GitHub squash-merge's parent is the current-main
tip at merge time (`5a94581d8` in the second instance), NOT the PR merge-base
the notification names (`f1d7d4133`). So first-parent `git diff
<squash>^..<squash>` is the true single surface; confirm it matches the
reported shortstat (there, exactly 4 files +680/-520: no union artifact). See
[[never-complete-an-abbreviated-sha-cite-rev-parse]].

## The one axis where CI infra reaches product, and who owns it

A test-selection, sharding or census change is the one CI shape with a product
shadow: if it silently **drops** or **double-assigns** a test, a product
regression lands ungated -- the "green gate hides a full-workspace behavior"
shape, on the Adversary's standing attack surface. Ground it from the diff
rather than waving it off, then stop:

1. **Exclusion-set MEMBERSHIP unchanged.** In the first instance the old
   `--partition count:${shard}/8` round-robin became a duration-balanced filter,
   and the exclusion went `not (binary(rt_parity_native) or ...)` ->
   `not (binary(=rt_parity_native) | ...)` -- the SAME three native binaries
   (`rt_parity_native`, `px8f_buffer_native`, `px8f_write_partition`), only
   `=`/`|` syntax. Syntax is not a membership change; a newly dropped member
   would be the finding, and still a CI finding owned by verify.
2. **A union / completeness check exists.** The candidate added a
   `verify-realized-shard-union` CI job running `check-ci-shard-union.py` to
   assert `union(realized shards) == full filtered inventory` (no drop, no
   double-run). In the second instance the same union-pin is a **named
   required-arm set**, stronger than a count.
3. **For a census count, check the direction of safety.** The third instance
   replaced a static `git grep '^#\[ignore'` (which undercounted
   macro-leading-token `#[ignore]` in generated tests) with the nextest
   `--list --run-ignored=only` population. Grep to ground truth closes the
   undercount, so the census moves in the safe direction. The new
   `deferred-inert-control` exemption class skips the source-`#[ignore]`-reason
   cross-check (macro tests have no `fn NAME(` to key off), but its staleness
   safeguard is recovered by nextest identity resolution: an exempted test must
   still resolve to exactly one ignored identity, and when it is un-ignored it
   leaves the set and the sweep fails.

**That completeness axis is the owning ring's QA lane and CI's own, not the
Adversary's to duplicate** (verify-qa approved `8a07de04`; runtime-qa approved
the third instance, `evt_2jn12p6va4knz`). Confirm the safe direction and that
the safeguard exists (a short diff read), then stop. When the coverage
preservation of an in-lane decomposition rests on a `scripts/` union-pin you do
not own, state it as a residual coverage boundary, guarded by the WP's own
drop-an-arm mutation tests (AC-NO-FALSE-GREEN) -- not a finding (see
[[preventive-findings-are-unfalsifiable-so-keep-them-cheap]]).

## An admin-bypass merge: verify the one consequence that is yours

In the second instance the publisher used the publish identity's admin bypass
because `gh`'s `mergeStateStatus` was stuck BLOCKED despite MERGEABLE and all
required checks green. The mechanics are process/publisher, out of §10⁻a, but a
bypass carries one product-integrity risk that IS yours: it can land a tree
that differs from the reviewed candidate. Verify by **blob identity**: for
every changed path, `git rev-parse <squash>:<path>` == `git rev-parse
<reviewed>:<path>`, plus `git merge-base --is-ancestor <intervening-main>
<squash>` to confirm the base move lost nothing. All blobs identical and the
intervening main an ancestor means the bypass altered nothing you own. Report
only that consequence, never the bypass.

## The verdict rule

Path set entirely `.github/`/`scripts/`/tooling -> no product surface -> **no
adversarial objection on scope grounds**, disclosed as a scope disposition,
exact-SHA binding confirmed, and the product-shadow axis named as the owning
ring's and CI's. Do not hunt the script internals; do not file a tooling
finding however correct. When a `crates/` test file is in the diff, clear that
file on its own axes and keep everything else a scope disposition.

Sibling of [[my-reporting-scope-silently-became-my-measurement-scope]] (there
the §10⁻a *filing* scope wrongly contaminated the *measurement* scope in the
severity-amplifying direction; here §10⁻a correctly bounds the *verdict
disposition* -- same scope rule, applied to the right thing).
