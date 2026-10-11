---
id: LANG-COHERENT-FRAME-SINGLE-CONSTRUCTOR-EQUAL
title: "A true proof of Equal at a single-constructor data type inside a coherent-frame nested indexed match is refused: the embedded convoy cannot repair a method whose induction hypothesis needs predecessor-index evidence, so it declines to the ordinary frame, which mis-indexes. Make the true rows elaborate and give the false row a discriminating observation"
status: ready
owner: language
size: M
tier: T1
gate: architect
depends_on: [LANG-MATCH-ARM-REFL-PAIR-GOAL]
blocks: []
github: null
origin: "Architect closure ruling evt_5r6mxx87123sc on LANG-MATCH-ARM-REFL-PAIR-GOAL AC-0b (evt_2qywssxz8dwhc), measured at 4a46c6163 with the three ruled hunks: a recorded limitation, Steward to frame. Steward-filed per COORDINATION section 2."
---

# True Equal proofs in a coherent frame elaborate

## Objective

A true proof of `Equal` at a single-constructor data type, inside a
coherent-frame nested indexed match, elaborates. A false one is refused at
its false step, not incidentally.

## Settled inputs (Architect `evt_5r6mxx87123sc`, at `4a46c6163`)

- **True rows are refused on base.** T1 (both sides identical, `Refl` in
  both arms), T2 (`lookup` against `lookup_peer`, recursive step) and T3
  (the `DualTagged (Option a)` twin of `cross_structure`, recursive step),
  each `Equal (DualTagged a)` in a nested match over `i` and then `xs`.
  Each gives `KernelRejected TypeMismatch { expected: (Dg647 @2), found:
  (Dg647 (cg72 @5)) }` at the inner `DualFZ` match, the true arm.
- **The named coherent-frame row is a proxy.** `dual_recursive_false`
  gives that same observation, so its `KernelRejected` assertion cannot
  tell false from incomplete.
- **Mechanism.** With `LANG-MATCH-ARM-REFL-PAIR-GOAL`'s hunks the `Eq` head
  survives, and `plan_coherent_frame_motive` (elab.rs ~:4066) routes the
  goal to `plan_embedded_method_convoy`. Method 1:1 there is unrepairable:
  its IH needs predecessor-index evidence that only injectivity derives,
  and no context variable carries it. That WP's third hunk declines the
  convoy to the ordinary frame, which mis-indexes as base does.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

AC-0 first. Then the embedded convoy repairs the methods the true rows
reach, with the evidence their IH needs; the Architect rules the mechanism
at D0.

## Acceptance

- **AC-0 (measure).** On the landed base: T1, T2, T3, the named row and
  the false Option control F2; for each declined method, the evidence its
  IH lacks and where injectivity would supply it.
- **AC-1.** T1 and T3 elaborate and check. T2 as the D0 ruling places it.
- **AC-2 (discriminating).** The named row and F2 stay refused, and the
  named row's refusal is at its false step. Its assertion names that
  observation, which a true twin does not produce.
- **AC-3 (control).** The 61 rows of the eight suites in the ruling and
  the 72-file catalog census are unchanged.
- **AC-4 (mutation, QA).** Removing the new evidence reddens T1 and T3.

## Stop conditions

- The evidence needs a kernel rule (injectivity or otherwise), a
  `trusted_base()` change or a spec change.
- A census or suite delta outside the named rows: stop to the Architect.
