---
scope: fleet
audience: (see scope README)
source: private memory
  `an-instruction-not-to-recheck-is-what-makes-a-false-negative-durable` (R4
  triage, 2026-09-26)
---

# An instruction not to recheck is what makes a false negative durable

A false negative decays on its own the moment anyone re-checks it: censuses
get re-run, someone stumbles on the missed case, the error is self-limiting.
An instruction attached to the result — "also cleared, so nobody re-runs
it" — removes exactly that self-correction. It converts a transient miss into
a durable one by instruction rather than by evidence, and it tends to attach
itself to the weakest instrument used, because the confidence that motivates
writing "don't recheck this" is produced by the same tedium that produces the
error.

## The evidence

A published catalog sweep read "Three exist... Also cleared, so nobody
re-runs it." The true answer was four. The grep had two independent
false-negative mechanisms in one pattern:

    fn (elem|mem|member|...)[ (]
              ^ alternation anchored at the name START -> misses e.g. `set_member`
                                     ^ delimiter REQUIRED -> misses any declaration
                                       whose parameters sit on the NEXT line

The second mechanism produced a clean miss on exactly the case the question
turned on — a declaration whose parameters wrapped to the next line — against
a population of thousands of declarations the grep never surfaced.

A harder variant does the same damage without an instruction to point at. A
campaign claim rested on a CI row's failure **text** as evidence that a
particular defect shape occurs in real programs; the ban on re-running the
census (correct, because the census's seam meant a cross-crate run could
never close the open question anyway) also, silently, stopped it from
reporting that the row's own refusal text had since changed to an earlier,
unrelated failure. A ban justified by what an operation *cannot establish*
also stops it from *reporting drift* in what it already established — those
are different jobs, and one sentence ended both. The fix is to scope the ban
by the question, not by the operation: still banned from re-running the
census to close the open question; not banned from re-reading the one row
whose text is the evidence, to check the witness still holds. A claim resting
on a specific failure message should treat that message as a pin — record
the exact text beside the claim, and treat a claim whose pinned text no
longer matches as withdrawn until re-measured, not as probably still fine.

A third variant carries no suppression sentence at all: a claim of the form
"X cannot occur" is self-sealing, because it is also, silently, an argument
that checking for X is wasted effort. A standing note asserted a fixed count
of ignored tests could not be reduced; a later commit removed one of them
outright, and the note's holder reported "zero cleared tonight" more than two
hours after the clear had landed — the claim deleted its own falsifier
without ever issuing an instruction to stop checking.

## How to apply

- Never publish a "nobody needs to re-check this" clause. If the point is to
  save others the work, publish the command instead — a re-run stays cheap
  and the result stays falsifiable.
- When a suppression instruction turns out to be wrong, ask where it landed,
  not just where the error surfaced. A correction in the thread that found
  the error does not reach a ring, frame, node, or memory lesson that still
  carries the original — that unreached copy is the one error class that will
  not decay on its own.
- Before trusting a census built on a hand-rolled pattern, ask what textual
  *form* the pattern requires (delimiter placement, anchor position, same-line
  parameters) and estimate how much of the population lacks that form.
- A ban on re-running a measurement should name the question it cannot
  answer, not the operation itself — an operation banned from closing a
  question may still be the only way to notice that its own prior evidence
  has gone stale.
- Keep a cheap, standing instrument beside any "X cannot occur" claim (a
  one-line count delta is enough) so the claim stays falsifiable instead of
  becoming a blindfold, and check whether you have already acted on the claim
  publicly before you catch it wrong.
