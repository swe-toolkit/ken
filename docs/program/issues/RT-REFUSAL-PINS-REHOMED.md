---
id: RT-REFUSAL-PINS-REHOMED
title: "Constructs 1 and 2 were ruled CORRECT SEMANTICS, so AC-9 owes them asserted-refusal pins -- but the only pin that asserts them today rides RecursiveDescentResidual, which D3 deletes. Re-home the assertion onto a mechanism that survives the retirement, before D6 deletes the tests carrying the fact"
status: active
owner: runtime
size: S
gate: none
depends_on: []
blocks: [RT-DESCENT-RETIRE]
github: null
origin: "Steward, 2026-08-16, discharging the undelivered half of RT-DESCENT-LANE-COMPLETENESS AC-9 once D1 (Architect evt_5cxzxp4b6q31v) ruled constructs 1 and 2 CORRECT SEMANTICS and D5 (evt_6tveatdhcz72y) closed the measurement. AC-11's re-home requirement is the Architect's, same ruling. Cut as a separate node because the lane-completeness node authorizes no implementation and can never reach merged."
---

Frame: `docs/program/wp/RT-REFUSAL-PINS-REHOMED.md`.

## Why this node exists, and why it is not part of the node that ruled it

[[RT-DESCENT-LANE-COMPLETENESS]] `AC-9` requires that each of `D1`'s four
constructs leaves a **checkable artifact** before [[RT-DESCENT-RETIRE]]'s `D6`
deletes the tests that currently carry the fact.

| verdict | owed | state |
|---|---|---|
| construct 3, **missing port** | recorded obligation, named owner | **DONE** — [[RT-FNUNIT-MULTI-WORKER-CONTINUATION]] |
| construct 4, **missing port** | recorded obligation, named owner | **DONE** — [[RT-FNUNIT-CHECKED-ROOT-AUTHORITY-ROUTING]] |
| constructs 1 and 2, **correct semantics** | **asserted-refusal pin that reds if the behaviour changes** | **NOT DONE. This node.** |

**A recorded obligation is prose and that is correct for a missing port. A
correct-semantics verdict needs a detector**, because the claim is that the
refusal is *right* and must keep happening.

**It is a separate node because [[RT-DESCENT-LANE-COMPLETENESS]] authorizes no
implementation and can never reach `merged`.** A pin is code. It needs a home
that merges.

## The trap, and it is the whole difficulty

**Architect `evt_5cxzxp4b6q31v`.** The pin that asserts constructs 1 and 2
today is `d2k_0_the_five_no_longer_reach_a_static_worker_value_read`, and it
asserts them **through**
`set_selector_variant_exclusion(Some(RecursiveDescentResidual::…))`.

**`D3` of the retirement deletes `RecursiveDescentResidual`.** Verified at
`origin/main`: **15 occurrences in `lowering/core.rs`, 42 in
`lowering/core/tests/control.rs`.**

⇒ **The pin rides the mechanism the retirement removes. Citing it does not
discharge `AC-9`** — it is the same fact and its only detector being retired
together, one remove out.

## The bound that makes this non-trivial

**The pin must be valid BEFORE `D3` runs** — that is the point of `AC-9`; a pin
that only works after the deletion protects nothing during it.

**Both lanes still exist today**, so a compile does not automatically take the
functionized route. **After `D3` there is nothing to exclude.** So the pin has
to assert the same refusal in a world where the forcing mechanism exists and in
one where it does not.

**Whether an exclusion-free assertion is achievable is the node's first
question, not its assumption.** If it is not, that is a genuine hard stop with a
named blocker, and it is a real result.

## IF THIS NODE HARD-STOPS, CLOSING IT IS NOT A DISCHARGE

`AC-8` makes a hard stop a legitimate outcome — the finding is the product. **But
closing this node on a hard stop satisfies [[RT-DESCENT-RETIRE]]'s `depends_on`
anyway**, because `scripts/check-issue-schema.sh:189` treats `closed` as
*resolved-without-landing*. Constructs 1 and 2 would then be unpinned with the
retirement's gate reading green — **`AC-9`'s exact defect, arriving through the
dependency mechanism rather than through an increment.**

⇒ **A hard-stop closure routes to the Steward and the Architect for an explicit
ruling on whether `D6` may run unpinned.** It is not a decision this node's
closure makes by itself. The mirror is on [[RT-DESCENT-RETIRE]], at the `D6`
block where the reader who runs the deletion will be.

## Standing

- **`D2c` `036e8ee916844fb91a4f42f2a2b04ebaea0dde2f` is untouched, unpublished
  and NOT rebased.** Do not apply it here.
- **`D3`-`D8` of [[RT-DESCENT-RETIRE]] stay gated.** This node lands a pin; it
  deletes nothing.
- **The five programs with no refusing construct are out of scope**, as are the
  other constructs' obligations.
