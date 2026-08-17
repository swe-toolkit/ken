---
id: RT-REFUSAL-PINS-REHOMED
title: "Constructs 1 and 2 were ruled CORRECT SEMANTICS, so AC-9 owes them asserted-refusal pins -- but the only pin that asserts them today rides RecursiveDescentResidual, which D3 deletes. Re-home the assertion onto a mechanism that survives the retirement, before D6 deletes the tests carrying the fact"
status: merged
owner: runtime
size: S
gate: none
depends_on: []
blocks: [RT-DESCENT-RETIRE]
github: null
origin: "Steward, 2026-08-16, discharging the undelivered half of RT-DESCENT-LANE-COMPLETENESS AC-9 once D1 (Architect evt_5cxzxp4b6q31v) ruled constructs 1 and 2 CORRECT SEMANTICS and D5 (evt_6tveatdhcz72y) closed the measurement. AC-11's re-home requirement is the Architect's, same ruling. Cut as a separate node because the lane-completeness node authorizes no implementation and can never reach merged."
---

Frame: `docs/program/wp/RT-REFUSAL-PINS-REHOMED.md`.

## MERGED 2026-08-17 at exact `d6a9760a9`. `D1` found the mechanism; it exists.

**PR #2506, CI green, blob-verified on `main`.** Decision `dec_ezzkjz9vtrr2`
resolved by the Architect at that exact SHA; QA `evt_55yet6zerfkdm` approved the
same SHA and independently reproduced both mutations. One commit, one test-only
path, `+111/-0`.

**`D1`'s question was answered YES, and the hard stop was not needed.** An
exclusion-free assertion does exist, because the refusals can be reached by
calling the production code that issues them rather than by forcing a lane:

| construct | how the pin reaches the refusal | asserted at |
|---|---|---|
| 1, `ComputationalMatch` | real `boundary_transfer_admissibility` on a built `ComputationalRecursorClosure` | `mod.rs:11985` |
| 2, `StaticWorkerBinding` | real `StaticWorkerFieldLedger::recognize` then `close` | `mod.rs:4729-4736` |

**Neither mentions `RecursiveDescentResidual` or `set_selector_variant_exclusion`**
(`AC-1`), and every symbol they depend on is lane-independent, so they survive
`D3`-`D8` and hold today with both lanes present (`AC-4`).

**`AC-3` was discharged by observation, not assertion.** Each arm was made red
alone and restored: replacing only the capsule with `Lowered::Trap` gave
computational red / static green; transitioning and consuming only the static
recognition gave static red / computational green. **Two independent detectors**
(`AC-2`), which is the ruling that a shared reason does not license shared
coverage.

**The candidate is purely additive, so the original detector is retained.** The
replacement landed **before** the carrier is retired rather than in the same
commit — the `nc22` ordering that was the whole point of `AC-9`, and the
candidate got it right without being told.

**Architect's note on this pin's domain, recorded so nobody over-cites it:**
construct 1's sentence appears at **two** sites — `mod.rs:11985`, the walk arm
the pin exercises, and `mod.rs:12214` as a `why:` field on a different
structure. **A change confined to `:12214` would not red this pin.** The walk is
the law and pinning it is correct; this states the pin's domain rather than a
defect.

**`D2c` `036e8ee91` unchanged** (`AC-5`), no production change (`AC-6`).

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
