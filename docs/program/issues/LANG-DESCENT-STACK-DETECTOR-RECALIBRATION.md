---
id: LANG-DESCENT-STACK-DETECTOR-RECALIBRATION
title: "local_prebinding_preserves_legacy_map_union_stack_budget is calibrated to a tree that no longer exists: its 512-byte discriminating reservation was bisected BEFORE LANG-REWRITE-DESCENT-FRAME-TAX removed the whole-surface frame tax that the margin was made of. Re-measure the discriminating range on the repaired descent and restate the constants to what it measures -- or establish that no reservation discriminates, which retires the instrument's claim rather than its boundary. This is the remeasure half of the operator's 'g then remeasure'."
status: ready
owner: language
size: S
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "Steward, 2026-09-20. The second half of the operator ruling 2026-09-19, verbatim 'g then remeasure', which selected option (g) on the A1 re-baseline fork and directed that the measurement FOLLOW the repair. Option (g) landed as LANG-REWRITE-DESCENT-FRAME-TAX (repair commit 6ce8aa0e9, 'split recursive rewrite arms into non-inlined monomorphs'); the measurement did not. Established by the producer's own history rather than by lane state: git log -S on both constants in crates/ken-elaborator/tests/map_build_acceptance.rs returns exactly one commit, 5a74301f4 (LANG-MOD-STRICT-RESOLUTION D1 stated-stack respin), which PRECEDES 6ce8aa0e9. The repair never touched the calibration. All current-code facts measured at origin/main fead292531d7f04ce99f449cedd8f45629935107."
---

# Recalibrate the legacy-map descent stack detector

`local_prebinding_preserves_legacy_map_union_stack_budget`
(`crates/ken-elaborator/tests/map_build_acceptance.rs:1494`) is not a budget
check. It is a **two-state discriminator**, and its entire discriminating power
lives in one 512-byte number that was bisected against a descent that has since
been rebuilt.

Treat anchors as perishable. If a fixed input is false on the landed base, stop
and report the mismatch; do not build around it.

## Settled inputs -- measured at `fead29253`. Do not re-derive.

**The instrument states its own calibration, and it is stale.** The doc comment
on `D1_LEGACY_MAP_STACK_RESERVATION_BYTES` (`:31,37`) records the original
bisection verbatim: *"Bisection at the stated stack found 512 bytes
candidate-green and inline-parent-SIGABRT; zero let both pass and 2,048 made
both abort."* So the discriminating window was **512-2048 bytes** against a
2 MiB stated stack -- a hair-trigger by construction, and the frame-tax issue
says so in those words.

**What the instrument claims** is in its own header: *"THE GAP: the workload
must reach the affected `expand_scope` recursion; the exact inline-parent
mutation closes that gap by making this test abort with SIGABRT while the
candidate remains green."* Discrimination IS the claim. A version that passes in
both states has not weakened -- it has stopped being a test.

**The margin it was calibrated against was the frame tax itself.** The tax was
*"the frame carries the entire language surface at every level, through every
node type"*; `LANG-REWRITE-DESCENT-FRAME-TAX` removed it by splitting the
recursive rewrite arms into non-inlined monomorphs. The repair's direct subject
is the per-level frame size, which is the quantity the 512-byte reservation was
trimmed to expose.

**Nothing recalibrated it afterward, and this is measured, not assumed.**
`git log -S` on `D1_LEGACY_MAP_STACK_BYTES` and on
`D1_LEGACY_MAP_STACK_RESERVATION_BYTES`, both scoped to that file, each return
exactly one commit: `5a74301f4`. The frame-tax repair `6ce8aa0e9` and A1
`e2e40e2b4` both landed after it and neither appears.

**The instrument's environment is being edited by unrelated nodes.** `mk_env()`
(`:134`) delegates to `mk_map_dependency_env()` (`:92`), so every change to that
fixture builder runs inside this detector's setup.
`LANG-MEMBERSHIP-OPERATOR-SURFACE` adds a `Core.Classes.Membership` roots-load
and a `globals.remove` there. That candidate's own suite measured 36/36, so the
detector PASSES at it -- which is the correct outcome for a candidate and says
nothing about whether the instrument still discriminates. Do not read that green
as evidence either way.

## Deliverable

Re-run the original bisection method on the repaired descent, then make the
constants state what it measures.

- Establish, at the stated 2 MiB stack, the reservation values at which the
  candidate is green and the exact inline-parent mutation SIGABRTs.
- Set `D1_LEGACY_MAP_STACK_RESERVATION_BYTES` (and
  `D1_LEGACY_MAP_STACK_BYTES` only if the measurement requires it) to the
  measured discriminating value, and rewrite both doc comments to record the new
  bisection and the base it was taken at.

One file. No new test, fixture, helper, or instrument. No change to the
workload, to `expand_scope`, or to any production path.

## Acceptance criteria

**AC-1 -- the new number is a measurement, and all three probe points are
reported.** State the reservation values tried and the outcome of each for BOTH
states, in the shape the existing comment uses: a value where both pass, the
value where candidate-green meets parent-SIGABRT, and a value where both abort.
A single passing value is not a bisection and does not satisfy this.

**AC-2 -- the instrument must be shown to DISCRIMINATE, not to pass.** Apply the
exact inline-parent mutation named in the header and show the detector goes red
at the new reservation; restore byte-exact. A recalibrated constant under which
the mutation stays green is a failure of this node, not a pass.

**AC-3 -- the old value must be shown to be stale.** Run the mutation at the
current `512`. If `512` still discriminates identically on the repaired descent,
the premise of this node is REFUTED -- report that as the result and stop. Do
not quietly keep the old number; a stale constant that drifts into being correct
cannot be caught by checking it, so this AC is the only thing that distinguishes
the two cases.

## Stop condition

Hand back rather than work around if either holds:

- **No reservation value discriminates.** If the repaired descent has so much
  headroom that the inline-parent mutation cannot be made to abort at the stated
  stack by reservation alone, that is a result and a Steward stop. Do NOT raise
  the stated stack, shrink it, enlarge the workload, or otherwise re-place the
  boundary to manufacture a margin. Option (g) was selected by the operator
  precisely because it was *"the only option that does not re-place the
  detector's boundary"*; a ring re-placing it here would spend that choice.
- **The measurement implicates production behavior** rather than the
  instrument's calibration -- for example the workload no longer reaches the
  `expand_scope` recursion the claim names. Report it; that is a different node.

## Not this node

- Retiring or rewriting the instrument's promise class. If the measurement says
  the claim can no longer be stated as a stack boundary, the frame amendment is
  the Steward's to author.
- Any change to `LANG-REWRITE-DESCENT-FRAME-TAX`'s landed repair.
- Generalizing the reservation technique to other stack detectors.
- `RUST_MIN_STACK` or ambient machine configuration; the explicit
  `Builder::stack_size` exists to make this independent of that.
