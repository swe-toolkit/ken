---
id: VERIFY-SORT-CONGRUENCE-STACK-HEADROOM
title: "The ken-elaborator test discarded_bool_last_pair_cost_is_bounded_by_kernel_conversion has less than 8 KiB of headroom on the 2 MiB default test thread: it overflows at 2040 KiB and passes at 2048 KiB on main. Any change that adds a frame on the declaration-elaboration path reddens it in CI, as LANG-STRICT-ISOLATED-ROUTES Increment 1 did. Either reduce the frames on its deep path or give it a stated stack, as the D0 rules"
status: ready
owner: verify
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Architect scope item evt_1yd4m9n2tw703, accepted by the Steward for the Verify ring (evt_1x2bbbvzhpe3m) on Lane 2. It was found as the M5 red of LANG-STRICT-ISOLATED-ROUTES Increment 1 (PR #5136). The Architect measured it at 8 KiB resolution (evt_4qjm40fvrft28). Steward-filed per COORDINATION section 2."
---

# The sort-congruence conversion test has stack headroom

## Objective

`discarded_bool_last_pair_cost_is_bounded_by_kernel_conversion`
(`ken-elaborator`, `--test lang_type_position_sort_congruence_first`)
passes on its CI stack with a stated margin. A one-frame addition on the
declaration-elaboration path no longer reddens it.

## Settled inputs (main `d8b135059`)

- **The margin** (Architect evt_4qjm40fvrft28, evidence in
  `/workspaces/ken/tmp/arch-m5-bisect/`). On base `3838d5a00` and on the
  Increment 1 repair `68666d462`, every cell from 1984 to 2040 KiB in
  8 KiB steps overflows with SIGABRT, and 2048 KiB and the default pass.
  The default spawned-thread stack is 2 MiB, so the headroom is under
  8 KiB. The test sets no stack of its own
  (`lang_type_position_sort_congruence_first.rs:345`).
- **The failing configuration a control can use today.**
  `RUST_MIN_STACK=2088960` (2040 KiB) overflows on both tips.
- **The prior art in the tree.** `LANG-ELAB-RECURSIVE-FRAME-BUDGET`
  (merged) outlined blocks from the elaborator's recursive cycle and
  added the guard `crates/ken-elaborator/tests/elab_recursive_frame_budget.rs`.
  `TEST-NATIVE-STACK-PROVISIONING-STANDARD` (merged) is the stated-stack
  standard: a test's stack is stated, not large.
- **A measurement hazard.** Two worktrees built into one
  `CARGO_TARGET_DIR` can reuse the first build's binary. Any A/B checks
  that the binary hashes differ (Architect evt_4qjm40fvrft28).

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

One candidate carrying the change the D0 rules: either frame reduction
on the test's deep elaboration or conversion path, or a stated stack for
this test under the stated-stack standard, with the reason written at
the site.

## Acceptance

- **AC-0 (D0, Architect).** Measure which frames on the test's path are
  the largest, and rule frame reduction or a stated stack. If reduction,
  name the frames, the margin to reach, and whether
  `elab_recursive_frame_budget` should grow a row. If a stated stack,
  name the size and its derivation.
- **AC-1 (control).** At `RUST_MIN_STACK=2088960` the test overflows on
  the base, and on the candidate the ruled configuration passes. For a
  reduction, the candidate also passes at the stack the D0 names as the
  target margin.
- **AC-2 (no regression).** The named targets the D0 lists pass, and the
  binary hashes differ between base and candidate.

## Stop conditions

- The reduction needs a kernel or trust change.
- The only passing configuration is a stack raised without a derivation.

Checks: 8 fired: the 2040 KiB configuration overflows on both tips today
and 2048 KiB passes, so AC-1's control discriminates. 4 fired: the
standard and the frame-budget guard were read at their files on main,
not assumed.
