---
id: RT-HOST-EFFECT-SEAT-CONTRACT-STALE-TOTALITY-COUNT
title: "`host_effect_seat_contract`'s doc comment states its own totality claim over a count that is wrong and a domain that is not the one it matches -- `effects.rs:380` says \"Total over the 22 admitted operations, with no `_` arm\" while the admitted list at `:350` holds 25 and the match is in fact total over all 35 `HostOpV1` variants; the SAFETY MECHANISM IS INTACT (no wildcard arm, so a new variant is still a compile error) and what is wrong is the sentence that ARGUES for it, in a doc block whose own thesis is that two copies of an authority can silently disagree; NO behaviour change, NO new admission"
status: draft
owner: runtime
size: S
gate: none
depends_on: []
blocks: []
github: null
tier: T2
origin: "Steward filed 2026-09-16. Handed over by the Architect in evt_5a4fsmm3wx8wk while ruling the 26th host-effect consumer OUT of ABI-S6-HS18-CHECKED-IH-CONSUMER-PORT: 'Incidentally, main's own doc comment two lines below that table says Total over the 22 admitted operations beside an array of 25. Not yours, not this node -- @steward, that is a stale count on a host-ABI authority and someone should own it.' Every figure below re-measured by the Steward at origin/main 432d36254926a48db2beb44ee327da827495346a, including one that refutes the first reading I took. Filed separately because it is wrong TODAY at 25 and is independent of whether the 26th operation is ever admitted."
---

> # DRAFT -- not framed, do not start. T2: the repair is a sentence, the value
> # is in having measured which sentence.

# The measurement

`crates/ken-runtime/src/cranelift_backend/planning/static_transition/effects.rs`
at `origin/main` `432d36254926a48db2beb44ee327da827495346a`:

    :350   const CRANELIFT_HOST_EFFECT_CONSUMERS_V1: [ken_host::HostOpV1; 25]
           25 declared, 25 distinct members -- the array and its length agree

    :380   /// **Total over the 22 admitted operations, with no `_` arm**, so a
           /// new admitted operation is a compile error here rather than an
           /// operation whose seats silently have no contract.

**22 against 25.** The array is right; the sentence is stale.

# The second error, which is the one worth the node

The count is not the only thing wrong. Measured inside
`host_effect_seat_contract` (lines 390-636, brace-matched):

    admitted operations matched in the body        25 of 25   none missing
    distinct variants matched in the body          35
    matched but NOT admitted                       10
      ClockMonotonicNow  ClockSleepUntil  EntropyRandomBytes  FsDuplicate
      FsGetInheritance   FsSeek           FsSetInheritance    FsSetLength
      FsSync             MappingAcquireFile
    wildcard `_ =>` arm                            absent

⇒ **The function is total over `HostOpV1`, not over the admitted operations.**
Those are different domains and the difference is ten variants wide.

**The safety mechanism is intact and this node does not touch it.** No `_` arm
plus an exhaustive match means a new enum variant *is* a compile error here.
The claim is true; the sentence stating it names the wrong domain and the wrong
number.

**Note `MappingAcquireFile` is already among the 35.** `main` gives it a seat
contract while not admitting it as a consumer -- consistent with the Architect's
ruling that on `main` it is a known op and not an admitted one. **That is not a
defect and this node must not "fix" it.**

# Why a stale count here is worth filing at all

The same doc block, immediately above the array, argues:

> *"A second copy on the lowering side would be a second authority: the two
> could disagree about whether an operation is admitted, and the disagreement
> would show up as a seat with no planned record rather than as a contradiction
> anyone stated."*

⇒ **A stale count inside the sentence making that argument is that failure in
miniature.** The prose is a second authority on the admitted set, it disagrees
with the array, and nothing anywhere fails. The cost is not a bug today; it is
that the next reader reconciling a mismatch has a documented number to reconcile
*against*, and it is wrong.

# Deliverables (indicative -- not framed)

- Correct `:380` to the measured domain and number. Prefer phrasing that cannot
  go stale again: the totality is over `HostOpV1`, which is what the absent `_`
  arm actually buys, and the admitted list's size is a separate fact.
- Decide whether the admitted-vs-contracted gap (25 admitted, 35 contracted)
  deserves a sentence of its own. It is currently unstated and it is the thing
  that made this comment easy to get wrong.

# Not this node

- **Not the 26th consumer.** `HostOpV1::MappingAcquireFile`'s admission is
  [[RT-D5B-HOST-FILE-ACQUISITION-SURFACE]]'s subject, ruled out of the port node
  by the Architect. **This node is wrong at 25 and stays wrong whatever happens
  to 26.**
- **Not the port node.** [[ABI-S6-HS18-CHECKED-IH-CONSUMER-PORT]] carries zero
  checked-IH references in this file; they are disjoint.
- **No behaviour change, no new admission, no TCB movement.**

# A note on how this was measured, because the first answer was wrong

The first pass reported **zero** `HostOpV1` variants in the function body, which
would have made this look like a coverage hole rather than doc drift. The body
opens with `use ken_host::HostOpV1 as Op;` and spells every arm `Op::FsReadFile`.
**A key of `HostOpV1::` matches the declaration and none of the uses.**

⇒ **A false zero from an over-narrow key, in a file whose own subject is two
authorities disagreeing.** Re-measure with the alias before trusting any count
here; the control is that the corrected key returns 35 where the first returned
0.
