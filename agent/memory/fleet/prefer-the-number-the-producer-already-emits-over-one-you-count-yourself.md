---
scope: fleet
audience: (see scope README)
source: private memory `prefer-the-number-the-producer-already-emits-over-one-you-count-yourself` (R4 triage, 2026-09-26)
---

# Prefer the number a producer already emits over one you count yourself

When a tool, script, or frame already states a quantity, building a second
count of the same fact from your own grep or post-processed log constructs a
weaker instrument for something already measured. Your count reads a
*rendering* of the fact — a printed line, a stream of notices — not the fact
itself, and it inherits every distortion the rendering can introduce:
truncation, wrapping, interleaving, a stale filter. The producer's own figure
is usually stronger, because it is closer to the source and typically comes
out only after the producer's own internal consistency checks.

## Evidence

- A work-package frame stated its own row count and decomposition directly
  (a `grep -c` result plus a breakdown of what each row was). A separately
  self-derived grep count disagreed and was raised as a finding — it was a
  second, weaker instrument measuring the same population the frame had
  already named, not a discovery.
- A CI job printed a one-line summary (`Ignored-row sweep completed: {n}
  selected; {m} passed`) only after three internal cross-checks that redden
  the job on disagreement. Counting the job's `::notice` lines to
  reconstruct the same number is strictly weaker: it is vulnerable to
  truncation and wrapping that the summary line already survives.
- When no number is emitted at all — a guard that fails fast on the first
  defect instead of enumerating the population — building the producer
  first (make the guard non-halting, have it print every member and a
  total) beats grepping its raw output. A grep piped through `| head -20`
  once undercounted 29 true members as 20, and the truncation ate its own
  tell (a `test result:` line that would have proven the cut never
  appeared). Re-running with a self-counting probe caught a further
  one-line discrepancy between the producer's own total and an anchored
  grep over the same stream, from a `--nocapture` interleaving artifact —
  confirming that even a self-built probe's own post-hoc grep over its
  output is the weaker of the two numbers it can produce.

## How to apply

- Before counting anything yourself, check whether the artifact already
  states it: the frame's own section, the script's summary line, the job's
  report step. One read decides whether your number is corroboration or
  noise.
- When your count disagrees with a stated number, first suspect you are
  counting a different population, not that the stated number is stale.
- A summary emitted after the producer's own cross-checks outranks a count
  of its output lines. Treat the rendering as redundant with it, not as an
  independent check.
- Your own count still earns its keep when it comes from a genuinely
  different producer of the same quantity — free corroboration on
  agreement, a free finding on disagreement. The same source rendered
  differently is not a second producer.
- Label which artifact a number came from, in the same sentence as the
  number. Several true counts of different populations inside one turn are
  indistinguishable without that label.

## The same failure shows up in a number you specify, not just one you read

A criterion that names a discriminator without naming which artifact and
which level it is read from can be ambiguous the same way an uncredited count
is. One replacement criterion offered two acceptable signatures for a probe
firing: a designed literal string in captured output, or "exit status 101"
as a stand-in for a specific process outcome. Exit 101 is not unique to that
outcome — it is Rust's default panic exit code, and it is also `cargo`'s own
exit code for a failed compile — so a row that failed to compile is
indistinguishable, by status alone, from a row whose probe actually fired.
An unrelated edit that failed to compile (a non-`Copy` move error) surfaced
exactly that way within the hour of the criterion landing.

The fix is the same discipline as reading a producer's number: name the
artifact and the level, not just the value. "The subject process's own
recorded exit status inside the observation" is a signature; "exit status
101" alone is not, because it conflates two different producers — the
subject process and the test command wrapping it — that can both emit the
same code. A designed, unique literal was free the whole time; a derived
status that another failure mode also produces is not.

Related: [[grep-the-producer-not-the-cited-proxy]] — the neighboring
discipline for verification rather than counting: trace to the artifact's
real consumer or producer instead of a cited stand-in that merely looks
equivalent. Both failures substitute a proxy for the ground-truth artifact —
here a rendering of a number, there a precedented-but-not-identical helper.
