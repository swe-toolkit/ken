---
name: sometimes-the-offered-run-is-weaker-than-the-argument-it-would-replace
description: A reviewer flagged a containment claim as "structural, not a run" and offered a corpus measurement as the control they had not required — reading four call sites showed the argument is total over every obligation while the run would sample it, so the right advice was not to require the control
metadata:
  type: feedback
---

# Sometimes the offered run is weaker than the argument it would replace

**Measured 2026-08-15 on `c189fa143`, where the Steward wrote: *"That containment
is a structural argument from quoted call sites, not a run. If you want one
thing here, a measurement that no obligation moved `Proved` → `Unknown` across
the corpus is the control I did not require."***

⇒ **The measurement would be evidence over whatever obligations the corpus
contains. The argument is a proof over all of them**, in four steps: the
dispatcher builds its context **once, before the match**, so all three route arms
get identical arguments; two of the three routes are the third function
*verbatim*; that function's only success arm is one call; and the new route's
**first statement** is that identical call.

⇒ ***Requiring the corpus control would replace a total argument with a sampled
one and produce a green that reads as stronger evidence than the thing it
replaced.***

**This inverts a standing rule.** *"Prose needs a run"* is right when the
claim is about a **population** — which programs, which sites, how many. It is
wrong when the claim is about a **code path**: there, reading the path is
exhaustive and a corpus run is a sample of it.

⇒ **Classify the claim before prescribing the instrument.** *"No obligation
moves Proved → Unknown"* looks like a population claim and is really a claim
that two call sites are the same call — **and the honest answer to "what would
you measure?" can be "nothing; here is why the argument is already total."**

## CLOSE THE ONE GAP IN SOMEONE ELSE'S STRUCTURAL ARGUMENT

The reviewer's version quoted the callees. **The gap was the dispatcher**: if it did
anything extra on one path — a different context, a mutation between
classification and the call — identical callees would not imply identical
outcomes.

⇒ **A "the callees are the same function" argument needs the CALLER to pass the
same arguments.** That is one read, it is the half that gets skipped, and
supplying it is what turns a good argument into a complete one. **Contribute the
missing step rather than re-deriving the steps already there.**

## MEASURE A MIS-NAMED PREDICATE'S REAL CONTRACT ON BOTH AXES

A predicate named for linearity over one type was flagged as *"accepts any
`Const` application"*. Reading it: it accepts **exactly binary** constant
applications — a three-ary application destructures to `App`, not `Const`, and is
rejected — over leaves that are **bound variables or integer literals only**.

⇒ **Unbounded on the operator axis, bounded on the leaf axis.** *"Accepts any
`Const` application"* states the first and reads as unbounded overall.
***When restating a mis-named predicate's contract, give both what it fails to
enforce and what it does*** — the successor written against the name will assume
more reach than exists otherwise.

**And check whether the widening is verdict-neutral before ranking it.** Here
both arms are guarded — success by the containment identity, refutation by a
kernel check — so a wrongly-captured term lands on the same verdict it would
have had. **The cost is the false contract, not the behaviour**, and saying so
keeps a naming finding from reading as a soundness one.

Related: [[structural-reachability-beats-empirical-probe-for-dead-code-fix]]
(`build/qa/`, the QA-side instance: a producer trace covers every expressible
term, a probe samples them).
