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

The count is not the only thing wrong. `host_effect_seat_contract` (lines
390-636, brace-matched) contains **two independent matches on two different
axes, and each is separately total over all 35 `HostOpV1` variants**:

    THE CAPABILITY HALF        :403-448   match operation
      Some(ObserveCapabilityToken)   11    the FS-path operations
      None                           24    every other variant
      distinct variants              35    TOTAL, and NO `_` ANYWHERE
    THE ARGUMENT TABLE         :449-635   match (operation, ordinal)
      distinct variants              35    TOTAL on the operation axis
      `_` patterns                    2    :615 and :633 -- ORDINAL axis only

    THE PARTITION (verified disjoint and covering, not eyeballed)
      admitted   CRANELIFT_HOST_EFFECT_CONSUMERS_V1 :350   25
      refused    represented-unavailable arm        :622-634   10
      overlap                                        0
      union                                         35

⇒ **The function is total over `HostOpV1`, not over the admitted operations.**
Those are different domains and the difference is ten variants wide.

**The safety mechanism is intact and this node does not touch it.** State it as
**"no `_` in the operation position"** -- that is checkable and true of both
matches (the capability half has no `_` at all). **"No `_` arm" is false as
literally written**: two exist, and they sit on the ordinal axis, exactly where
wildcarding is safe and absent exactly where one would hide a variant.

**Note `MappingAcquireFile` is one of the ten.** `main` does not give it a seat
contract -- it **names it in the represented-unavailable arm (`:631`) and
refuses it**, which is what makes a future admission a compile-visible edit at
this site rather than an operation whose seats silently answer `None`. That is
the mechanism working, not a defect, and this node must not "fix" it.

Its **second** occurrence, `:442`, is in the capability half's `None` list and
is **not** a refusal of the operation: that list also holds admitted operations
such as `ConsoleWrite`, so `None` there means "admits no capability token,"
a different axis with a different meaning. **Two occurrences, two axes; only
`:631` is the refusal.**

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

- Correct `:380` to the measured domain and number. **Name the partition, not a
  number** -- every figure must be checkable against an arm in the same
  function, and the sentence must say which arm. Indicative replacement:

  > Total over all 35 `HostOpV1` variants, with no `_` in the operation
  > position: the 25 admitted operations of `CRANELIFT_HOST_EFFECT_CONSUMERS_V1`
  > above, plus the 10 represented-unavailable lanes named in the final arm. A
  > new enum variant is therefore a compile error here, and promoting a lane
  > from the second group to the first is a compile-visible edit rather than an
  > operation whose seats silently have no contract.

- **State the admitted-vs-contracted gap explicitly. This is NOT optional and
  it is the load-bearing half** (Architect, `evt_mcv2c21favgj`). A correction
  that fixes the number and leaves the domain distinction implicit **re-creates
  the conditions for the identical error** -- the next reader still cannot tell
  which of the two populations a bare count refers to. The gap is what made
  this comment easy to get wrong in the first place.

# Not this node

- **Not the 26th consumer.** `HostOpV1::MappingAcquireFile`'s admission is
  [[RT-D5B-HOST-FILE-ACQUISITION-SURFACE]]'s subject, ruled out of the port node
  by the Architect. **This node is wrong at 25 and stays wrong whatever happens
  to 26.**

  What the measurement hands D5B, concretely: **admitting the operation is a
  MOVE between two named arms** (`:622-634` to the admitted table), which the
  compiler sees. It is **not** an addition to a table and **not** an
  array-length delta -- which is the part that looks small and is why the
  Architect ruled it out of the port node.
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

## It happened a SECOND time, to a second reader, in the same file and hour

Reviewing this node, the Architect tested the safety claim with the key
`^\s*_\s*=>` and got **NONE** -- which reads as "no wildcard arm, claim
confirmed." **It would have confirmed a true claim for a false reason.** The two
`_` patterns are written `_,` on their own line inside a tuple pattern
(`:615`, `:633`), so they never sit adjacent to `=>` and the key cannot reach
them. A looser re-run found both.

⇒ **Two readers, two different over-narrow keys, two false zeroes, one file.**
The shared shape is not the key's text -- it is that **a zero was accepted from
an instrument that had never been shown to return non-zero on anything.** In
both cases the fix was the same and cost one command: run the key against a
case it MUST hit before believing a case where it hits nothing.

**Note which direction each error pointed.** Mine would have inverted the
finding into a coverage hole -- loud, and it would have been caught. The
Architect's would have *confirmed the conclusion being tested* -- silent, and
nothing downstream would ever have contradicted it. **The false zero that
agrees with you is the expensive one.**
