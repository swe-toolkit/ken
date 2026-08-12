---
name: a-pre-existing-twin-distinguishes-a-convention-from-an-omission
description: My asymmetry finding was a false alarm and the deciding evidence sat in the CONSUMER, not the arms — before filing a structural asymmetry, look for a pre-existing site with the same shape, because an older twin means convention and no twin means omission
---

# A pre-existing twin distinguishes a convention from an omission

**Measured 2026-08-10. I filed an asymmetry between two adjacent walk arms; it
was a false alarm, and the way it resolved is the reusable part.**

I reported that `Closure` recursed into its body only while the adjacent
`LexicalClosure` also walked its captures, and bounded it on the fact I could
not reach: *does `Closure` have capture child origins at all?* It does not — so
the arms correctly differ.

**Two things settled it, and neither was in the arms I was reading.**

1. **The deciding evidence was in a CONSUMER.** Another site built `Closure`
   capture provenance by enumerating `captures.iter()` with **no child lookup at
   all**, while its `LexicalClosure` branch looked up `children` by ordinal and
   **errored when absent**. *If `Closure` had capture children, that code would
   read them.* ⇒ **When two producers disagree about a shape, the shape's
   answer is written in whatever consumes it** — go find a site that must know.
2. **A pre-existing twin carried the identical asymmetry** and predated the
   change entirely.

⇒ **Before filing a structural asymmetry, grep for an older site with the same
shape.** An older twin means **convention** — the codebase has answered this
before and the new arm is following it. **No twin** means the asymmetry arrived
with this change, which is when it is worth a question. This is one command and
it is the difference between a finding and a false alarm.

**The bound is what kept it cheap**, and that half worked: I named the deciding
fact instead of asserting a defect, so the answer was one lookup rather than an
argument. **Filing a bounded question you cannot close is legitimate; filing it
without having looked for the twin is not.**

## A direction argument can outlive the finding it came with

The finding failed and **the direction survived**: under-reporting into an
identity key means two distinct producers key the same — fail-open, not safe —
and that reasoning was kept as load-bearing text elsewhere.

⇒ **State the direction as a separate claim from the instance.** The instance
is refutable by one lookup; the direction is a property of the mechanism and is
often the durable half. Report them as two things so one can survive the other
([[state-the-DIRECTION-of-a-weakness-over-strict-or-unsound]]).

## A reachability defence is only as good as its enumeration

Same merge, the other half. An unchecked `params[p_idx]` in a **kernel**
function was defended as *"the caller can no longer reach it with a mismatched
host"* — singular. Measured: **~20 callers outside the module**, sixteen in the
elaborator and **two inside the kernel's own checker**. The repair added no
bound; it removed one path's ability to arrive mismatched.

⇒ **Count the callers before accepting a reachability defence**, and report the
**scope of the sentence** rather than a defect when the merge makes nothing
worse. The risk is not this change — it is *"the caller can no longer reach
it"* later being read as *"the function is safe"*, after which caller
twenty-one gets no scrutiny. **Prefer the repair that dissolves the question**
(bound the index) over the one that answers it for one path, for the same reason
as [[rank-a-controls-assertions-by-what-survives-a-redundancy-trim]].

## A SINGLE-WRITER structure closes findability, not the claim

**Measured 2026-08-11 on `75918c11`, on a reachability claim explicitly offered
as "the same shape as the `subst_outer` one, but narrower".** It was narrower
**structurally**, and the distinction is worth carrying:

| | `subst_outer` | the lift-binding map |
|---|---|---|
| producers | ~20 independent callers, each supplying its own operands | **one** `insert` site; the two installers funnel through it |
| a future producer | adds a 21st unaudited path | **cannot bypass** the single site |

⇒ **Enumerate at the narrowest waist, not at the callers.** A defence over N
callers is only as good as your grep on the day; a defence over a single writer
is **permanently one grep**, because anything new must route through it.

**But say what it does and does not buy.** The writer *inserts*; it does not
validate. A third installer supplying a bad operand is not stopped by
single-writer-ness — it is only made **visible**. ⇒ **Single-writer closes
FINDABILITY; the invariant is still a caller-side property.** Reporting that
distinction upgrades the author's confidence in a specific way instead of either
rubber-stamping or manufacturing doubt.

**And the cheap durability move follows from the shape**: both current
installers pair their insert with a validator call and a removal, so a third
that skipped validation shows up as *an insert path with no neighbouring
validator*. Naming that pairing at the write site turns a habit into a check —
one line, no mechanism change.

## Check the axis you were handed FIRST, and say so when it holds

I was told which axis could make the merge hollow (a second property whose
control might be weaker than described). It held, and was stronger: the control
asserted a **computed value** through the full pipeline, across three witnesses
with **two different expected values** — so a constant-returning or
wrong-subtree implementation fails. **Leading with that, before a bounded
observation, is what makes the observation readable as a weight rather than an
alarm.**
