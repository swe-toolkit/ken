---
name: before-arguing-which-knob-governs-a-phase-verify-the-run-reached-that-phase
description: "Two seats spent three exchanges arguing whether cargo's -j or codegen-units throttles within-crate LLVM parallelism, in a build that aborted on type errors and therefore never ran LLVM at all. A correction can replace a wrong mechanism with a right one and leave the false premise underneath both untouched, because the premise is what the disagreement presupposes rather than what either side asserts. Instrument the run for the phase before you reason about the phase's knobs."
metadata:
  type: feedback
---

# Before arguing which knob governs a phase, verify the run reached that phase

**Measured 2026-09-16 while probing the cargo `-j` bound on the dev box.**

A seat proposed `-j 3` as a safe build-parallelism bound on the evidence that
`-j 1/2/3` all completed and `-j 6` was killed. A second seat raised a
comparability confound. The first seat then named what it called the sharper
version: *"`-j` governs parallelism across crates; parallelism within one crate
is `codegen-units`, so a single-crate probe may have had nothing to
parallelise."* The second seat researched that and corrected it: cargo and
rustc share a GNU-make jobserver, rustc draws tokens from it for parallel LLVM
work, so `-j` **does** throttle within a crate.

**The confound and the correction came from the same seat** — the one that had
just established that `touch` controls for caching and not for extent, and was
therefore the seat most primed on this very question. A third seat reviewed the
result and qualified its occupancy conditions. **The correction was accurate
and it did not matter.** The probes rebuilt one
crate that fails type-check with eleven errors. **A crate that fails
type-check never reaches LLVM codegen**, so there were no codegen threads for
the jobserver to bound and no CGUs to distribute. Instrumenting it showed this
directly:

    -j 1: 1 rustc process, 4 threads, 960 MB peak, 8s
    -j 3: 1 rustc process, 4 threads, 948 MB peak, 8s

`-j` bound nothing. The two settings were the same measurement twice, and the
bound they were read as establishing compared that against kills taken on a
different, multi-crate shape.

## Why the correction could not reach it

**The false premise was not asserted by either side — it was presupposed by
both.** One seat said `codegen-units` governs the within-crate case; the other
said the jobserver does. Both sentences take for granted that there *is* a
within-crate codegen case in this run. A correction operates on what was
asserted, so it swapped the knob and left the presupposition load-bearing and
unexamined. **The more precisely the two parties disagree, the more firmly the
shared premise is held**, because agreeing on the subject is what makes the
disagreement legible. And it held strongest where it should have been caught:
it walked through the seat consulted *because* the mechanism was in doubt, and
through a reviewer who checked the conditions on the number — and checking a
number's conditions ratifies that it has a subject.

⇒ **A disagreement about mechanism is evidence that both parties accept the
premise the mechanisms range over.** That premise is the thing to measure, and
it is the one nobody will raise, because raising it is not a move in the
argument being had.

## How to apply

- **Establish the phase empirically before reasoning about its knobs.** Cheap
  instruments: does the run emit a `Finished` line, or an `error: could not
  compile`? How many processes and threads exist while it runs, and does that
  change with the flag? Peak RSS at two settings of the flag is a one-line
  discriminator — **if the flag does not move the number, it is not the
  variable.**
- **A flag that binds nothing produces two identical runs, which read as a
  reproduced result.** Sameness across settings is the signature of an inert
  knob and of a robust finding at once; only an instrument that watches the
  mechanism (processes, threads, RSS) separates them. The terminus cannot:
  same endpoint says where a run stopped, never how much it did.
- **When you receive a correction that swaps your mechanism for a better one,
  ask what both versions assumed.** The upgrade in precision is exactly what
  makes the shared premise feel settled.

Related:
[[a-number-sound-as-a-measurement-is-not-sound-as-a-criterion]],
[[mechanism-citation-needs-own-empirical-probe]],
[[repairing-a-census-completeness-does-not-re-aim-its-subject]].
