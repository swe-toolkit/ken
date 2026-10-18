---
id: RT-NATIVE-BRACKET-SEAT-CLEAR-NECESSITY
title: "RT-NATIVE-BRACKET-BODY-EFFECT-ONCE added a seat clear in lowering/core.rs that no measured row needs: removing it alone leaves F1 and F2 green, because the graft-frame copy-up already prevents the double perform. Either pin the clear with a row the graft frames do not cover, or remove it and its two refusal arms once a census shows its population is contained in the graft-frame population"
status: ready
owner: runtime
size: S
tier: T1
gate: architect
depends_on: [RT-NATIVE-BRACKET-BODY-EFFECT-ONCE]
blocks: []
github: null
origin: "Architect approval of RT-NATIVE-BRACKET-BODY-EFFECT-ONCE f555f4a37 (evt_22nrte9hhk4dk, limit 1) and successor ruling evt_2xjrx0gd29z3g item 5: a separate S node, ordered before the px8ta implementation. Steward-filed per COORDINATION section 2."
---

# The seat clear is either necessary or gone

## Objective

Every mechanism in the native bracket-body route is shown necessary by a
row, or removed.

## Settled inputs (on `f555f4a37`)

Architect `evt_22nrte9hhk4dk`, QA `evt_11cgpqq5yv7kz`.

- **The clear.** `core.rs` ~:12445 stores `Cleared` at
  `frame.region.discriminant` only when this function minted the row. Two
  PlannerInvariant refusals cover no local mint and two mints.
- **Measured.** Removing the clear alone leaves F1 and F2 green. Removing
  only the graft frames traps F1 with -1. Removing both brings back F2's
  `x y z y z z`. So on F2 the graft frame also prevents the double perform.
- **Lean (Architect).** Remove, by subsume-don't-proliferate, unless a row
  needs it.
- **Population.** The clear's population is the body-effect frame chain
  (F1, F2), not bracket settlement.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

1. **Census.** For every function where the clear fires, record whether
   a graft frame also covers the same row, over the full
   `rt_native_bracket_body_once` suite, the Mapping rows, O0/W0 and px8ta
   depths 1 and 2.
2. **Then one of:**
   - **Pin it.** Add a row where the graft-frame copy-up does not overwrite
     the caller's record, and show that removing the clear alone reddens it.
   - **Remove it.** If the census shows containment, remove the clear and
     its two refusal arms.

## Acceptance

- **AC-1.** The census, with its population named, is in the handoff.
- **AC-2.** Pin path: the new row is red with the clear removed and green
  with it. Remove path: `rt_native_bracket_body_once` 5/5, and AC-2 of the
  predecessor (restoring the double route reddens F2) still bites.
- **AC-3.** The interpreter/native observation tuples on every census row
  are unchanged from `f555f4a37`.

## Stop conditions

- A census row where the clear fires and no graft frame covers it, yet no
  observation differs. Report it; do not guess.
- Any change outside `crates/ken-runtime` and the predecessor's test file.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40`,
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY` or `wp/RT-BRACKET-SETTLEMENT-PLANE`.
