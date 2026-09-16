# WP frame — `RT-D5B-IMMEDIATE-BRIDGE-CLASSIFIER`

**Owner:** Team Runtime · **Size:** M · **Risk:** low (additive, no live path) ·
**Tier:** T1 · **Gate:** none · **Deps:** none

**Origin:** operator directive 2026-09-16, verbatim: *"factor small mergeable
pieces out of the long string of commits and merge those... Small achievable
pieces, not one monolithic PR."* PR #3676 is closed and its branch
`wp/ABI-S6-d5b-file-backed` is kept at
`0f71ab5b9267781ae1d91bc654011cad42b926af` as a read-only reference. This is
slice 1 of the re-cut, and it is the first `crates/` diff the lane has produced
since `ff9d0f0ca`.

## 1. Objective

Land the **plan-independent immediate-bridge classifier** as a standalone
module on `main`, exercised by tests that fail without it. No plan coupling, no
field on `StaticTransitionPlan`, no behavioural change to any existing path.

## 2. Why leaf-extraction was abandoned, so nobody retries it

Four extraction attempts against the 36-commit stack, all measured, all
refuted:

    24 of 36 commits are one- or two-file        looked promising
     5 cherry-pick cleanly onto main             looked mergeable
     0 survive inspection
       5fcfaabf5  include_str! a module ABSENT from main
       57ef38eeb  same module reference
       6cf61ea8c  cherry-pick produces an empty diff
       7e6eca324 + 936406cc3   comment-only, single file, and BOTH
                               CONFLICT on cherry-pick: units.rs has
                               diverged around the hunks

**A clean cherry-pick is not a compile, and a comment-only diff is not a
context-free diff.** The dependency root `bc7cb0304` is 17 files and
`+3194/-648`; every small leaf refines it. **There is no small piece to peel
off because the bottom of the stack is not small.** The mechanism is therefore
re-cut from `main`, using the branch as a source rather than as a merge
candidate.

## 3. Fixed inputs, measured at the implementation base

**Implementation base is `origin/main` at
`80d3ff78042d47b841ad64165f3ee4f50b524f4c`, and that is a ruling, not a
default.** Source is
`0f71ab5b9:crates/ken-runtime/src/cranelift_backend/planning/static_transition/immediate_bridge.rs`,
622 lines.

**The seam is a single truncation point.** `StaticTransitionPlan` is named at
`:342 :457 :515 :598 :609` — every occurrence at or past `:341`. Nothing
plan-coupled sits above it.

    STRATUM A   :1-340    this WP      ends on the closing brace of
                                       shifted_aggregate_ihs
    STRATUM B   :341-622  successor    derive_/build_/publish_/validate_,
                                       the extension impl, the field,
                                       the cfg-gated mutation guard

Verified in both directions: **Stratum A references zero Stratum-B names**, and
`impl ImmediateBridgeRealization` touches no plan.

**Stratum A's import surface, counted over the body with the `use` block
excluded:**

    USED       RuntimeExpr 31   StaticOriginId 8   BTreeSet 4
               RuntimeTrap 3    ContinuationCallIdentity 2
    NOT USED   planner_error 0   planner_capacity_error 0
               CraneliftBackendError 0   StaticTransitionPlan 0   BTreeMap 0

The error-handling surface belongs to Stratum B. Stratum A's import block is
two lines.

**The slice builds on `main`, measured rather than predicted:**

    baseline    main as-is, no slice          EXIT=0    88 warnings
    with slice  mod + 338-line Stratum A      EXIT=0   101 warnings
    CONTROL     inject a syntax error         EXIT=101
    RESTORE     revert the injection          EXIT=0

The control is recorded because a build that silently never compiled the module
would produce the same `EXIT=0`. Diagnostics cite line coordinates inside the
new file.

## 4. THE TRAP — the `+31` hunk on `static_transition.rs` is NOT registration

Architect ruling, `evt_2a3q52kafnsvw`, and this is the part most likely to be
got wrong by someone lifting the branch's diff:

    + mod immediate_bridge;  + the pub(in ..) use list   declarative, fine
    + the immediate_bridge_realizations field            STRATUM B
    + with_d5b_hs10_inline_response_mutation             test-support
    + let owns_seat = match row.sub_case() {             LIVE BEHAVIOURAL CHANGE
    +     DeferredResponseSubCase::InlineBridgeNoCall

**`InlineBridgeNoCall` is absent from `main`** (0 files, against a control of
`DeferredResponseSubCase` at 3 files). That hunk carries a new enum variant
**and** a rewrite of response-seat ownership. **Anyone taking "the module plus
its registration" by lifting that file's hunk gets an unreviewed behavioural
change inside what they believe is a declarative slice.** It is slice 3 at the
earliest and needs its own frame.

**This WP's diff to `static_transition.rs` is ONE LINE:** `mod
immediate_bridge;` between `mod effects;` and `mod joins_traps;`.

## 5. Deliverables

1. `crates/ken-runtime/src/cranelift_backend/planning/static_transition/immediate_bridge.rs`
   — Stratum A only, with the two-line import block from §3.
2. The one-line `mod` registration.
3. **Unit tests over `RuntimeExpr` values that fail without the classifier.**
   This is the WP's real work — see §7.

## 6. Acceptance

**AC-1. Stratum A only.** `StaticTransitionPlan` occurs **zero** times in the
new file; no field is added to `StaticTransitionPlan`; no extension `impl`
exists. Control: `grep -c StaticTransitionPlan` on the new file returns 0, and
`git diff --numstat` shows `static_transition.rs` at `1` insertion, `0`
deletions.

**AC-2. The tests FAIL WITHOUT THE MODULE, and this is demonstrated, not
asserted.** Control: stub the classifier's return (or delete the module and its
`mod` line) and record the tests going red; restore and record green. **Cite
both runs.** A green that survives the module's removal proves only that the
crate compiles, and this AC exists because that is precisely the failure mode
the slice was designed against.

**AC-3. No behavioural change to any existing path.** Control: the diff touches
exactly two files — the new module and the one `mod` line. Any other
`crates/**/src/` file in the diff fails this AC.

**AC-4. `InlineBridgeNoCall`, the `owns_seat` rewrite, the
`immediate_bridge_realizations` field, and
`with_d5b_hs10_bridge_plan_mutation` are ABSENT from the candidate.** Control:
each name greps to 0 hits across the diff. These are slices 2 and 3.

**AC-5. The production-build warning delta attributable to the new file is
accounted for explicitly** — see the open decision in §7. Control: build at the
base and at the candidate, diff the warning lists, and attribute every new
warning to a source file. **Warnings arising in files other than the new module
fail this AC.** At the base measurement the delta was 12, all in the new module
and none elsewhere.

## 7. The open decision, and the WP's real risk

**The tests are new authorship, not a lift.** The 36 commits proved this
classifier out *through the plan*, so no direct unit tests over `RuntimeExpr`
exist to carry across. The classifier is pure, so it is directly testable — but
writing those tests is design work, which is why this is T1 and M rather than a
file move.

**OPEN, for the Architect: the dead-code window.** In a production build the
classifier has no consumer until Stratum B lands, so it reports `never used`
twelve times. Measured: CI has no `-D warnings`, no clippy gate, no
`[workspace.lints]`, and no `deny(dead_code)`, **so this does not block a green
build** — it is a quality call, not a gate. Two options:

    (a) accept the 12 warnings for one slice's duration
    (b) #[allow(dead_code)] on the module, with a comment naming the
        successor node, and an AC on Stratum B REMOVING it

**Recommendation: (b), on the condition that the removal obligation is written
into the Stratum B frame at the same time.** An `allow` with no named remover is
how a temporary suppression becomes permanent. If the Architect prefers (a),
`AC-5` still holds and the warnings are simply recorded.

## 8. Contention

Writes one new file and one line of
`crates/ken-runtime/src/cranelift_backend/planning/static_transition.rs`. No
open candidate touches that file. `wp/RT-IGNORED-PASSING-ROWS-D0` is read-only
over test files and `.github/`. No contention.

**This WP produces a `crates/` diff, which sets `classify-paths` `mode=full`
and fires the `ignored-row sweep` job — the instrument `D0` needs and the only
compliant venue for it under `COORDINATION §12`.** That is a consequence worth
knowing, not a reason to rush the slice.
