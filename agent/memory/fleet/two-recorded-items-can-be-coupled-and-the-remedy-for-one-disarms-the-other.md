---
name: two-recorded-items-can-be-coupled-and-the-remedy-for-one-disarms-the-other
description: A merge recorded a PATH-resolution gate and a CI-coupling hazard with a named one-line remedy — the remedy removes the required job that gives the adapter's only real-binary control its teeth, and that control is the only thing exercising the query generator the stubs cannot see
metadata:
  type: feedback
---

# Two recorded items can be coupled, and the remedy for one disarms the other

**Measured 2026-08-15 on `9bc035710`, where both hazards were recorded honestly
and neither mentioned the other.**

Item A: bare `PATH` resolution, held as a graduation gate. Item B: a new CI job
wired into the required aggregate, which reds every PR on an unrelated mirror
failure — *"the one-line remedy is named in the node."*

⇒ **The stubs write `#!/bin/sh\ncat >/dev/null\n{body}` — they discard stdin and
print canned output.** So every stub test exercises the parser and the failure
taxonomy and **cannot see the query generator at all**: garbage SMT-LIB still
gets the canned model back and everything stays green.

⇒ **Exactly one test drives a real binary**, and it does not skip when the
binary is absent. **So the required CI job is what gives it teeth — and item B's
named remedy removes the job from the required set**, downgrading the only
control over the emission path to advisory.

⇒ ***Two individually-correct dispositions can pull against each other, and
neither node says so because each was written about its own hazard.*** **When a
notification records more than one item, ask what each item's remedy does to the
other** — the coupling lives in the remedies, not in the findings, so it is
invisible to whoever reviews them one at a time.

**Timing makes this worth raising immediately**: it is an argument about a
remedy that has not been applied yet. **A coupling finding is cheap before the
fix and expensive after.**

## A STUB THAT DISCARDS STDIN CANNOT TEST THE THING THAT WRITES STDIN

`cat >/dev/null` is the giveaway, and it is one line of the fixture nobody reads
because it looks like boilerplate.

⇒ ***Check what a test double consumes, not just what it returns.*** A double
that ignores its input covers every consumer of its output and **no producer of
its input**. **List the components on the wire and mark which side the double
stands on.**

## SOMETIMES THE GUARANTEE IS CARRIED BY A TYPE, AND THAT IS THE REPORTABLE PART

The trust claim — *"the solver is an oracle and never an authority"* — holds
because the ingestion function returns a `Vec<BigInt>`. **The output cannot carry
a verdict, a certificate, or a term; it can only propose numbers.**

⇒ **Report the mechanism, not the conclusion.** *"Structural in the ingestion
type"* survives a refactor review in a way *"the caller checks it"* does not, and
it tells the next reader exactly which change would break it — widening that
return type.

## LOOK FOR THE HANG, AND SAY SO WHEN IT IS ABSENT

I went after process hygiene expecting no timeout. There is one — a poll loop
with `kill()`/`wait()` on expiry, a kill on write failure, `stderr` nulled.

**The bounded limit worth a clause**: the poll never drains stdout, so an
answer exceeding the pipe buffer blocks the child, which then cannot exit, so the
timeout fires. **Fail-safe, and the drain and the timeout are mutually exclusive
in that structure.** ⇒ *"Timeout on a hung solver"* and *"timeout on an oversized
answer"* are one row in the failure taxonomy and two different events — **split
them, or the taxonomy claims coverage it does not have.**
