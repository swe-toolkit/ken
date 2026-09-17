---
name: attributing-interleaved-output-to-the-adjacent-test-is-not-a-measurement
description: "Parallel `cargo test` interleaves stderr, so attributing a printed line to the test header above it is not a measurement - the same probe gave 110 producing tests with the wrong top producer under the default harness and 11 with the right ones under `--test-threads=1`. Both runs were real and reproducible; only one was true. Carries the prior half too: a probe whose output the harness CAPTURES reads zero from live code, and only a positive control in a function the suite certainly runs distinguishes a dead channel from dead code."
metadata:
  type: feedback
---

# Attributing interleaved output to the adjacent test is not a measurement

**Measured 2026-09-16 while pinning a planner invariant.** Two instrument
defects in one sitting, both on the same probe, both caught by controls rather
than by care.

## One: a captured probe reads zero from live code

`eprintln!` probes in a planner path reported, across a 1034-test suite:

    detached proofs minted     0
    worker_return set          0
    mint guard evaluated       0
    plan builder entered       0

**`cargo test` captures stdout and stderr for passing tests and prints neither.**
Every zero was an artifact. The conclusion one step away — *"this function is
never entered by 1034 tests"* — is finding-shaped, alarming, and would have been
believed, and every number in it was honestly measured.

**What separated "the code never runs" from "the channel is dead" was a positive
control**: the same probe placed in a function the suite certainly runs. It also
read `0`. Under `--nocapture` the same build reported `9557` for the control and
`5185` for the path claimed never to execute.

⇒ **A probe channel needs its own control, every time, in the same run.** Not
the instrument's reach into the code — the instrument's reach into your eyes.

## Two: adjacency is not attribution under parallelism

With `--nocapture` the output is real, so the natural next step is to attribute
each line to the `test <name> ... ` header above it. **libtest runs tests in
parallel by default, so one test's stderr lands inside another's output.**

    default harness       110 producing tests, top producer d8d_the_composed_...
    --test-threads=1       11 producing tests, top producers contspec_*

**Both runs were real and reproducible. Only one was true.** A pin written
against the parallel run's "heaviest producer" asserted over a fixture that
produces none of the population — caught in seconds by the pin's vacuity
control, which existed only because a pin that finds nothing passes.

⇒ **If you are attributing output to a unit of work, serialize first.** The
parallel number is not a smaller-than-true measurement; it is a measurement of
a different thing wearing the same shape.

## The trap sitting next to this

The corpus carries
[[exact-and-single-threaded-do-not-give-a-test-more-stack-libtest-always-spawns]]
— `--test-threads=1` does **not** move a test onto the main thread's larger
stack, because libtest always spawns. That is true and it is about **stack
headroom**.

**It is easy to come away from it believing the flag is inert. It is not.** It
serializes execution, and therefore serializes output — which is exactly the
property you need for attribution. **A lesson of the form "flag X does not do Y"
is remembered as "flag X does nothing."** Read what a lesson's subject actually
was before concluding a tool is useless.

## How to apply

- **Probing a code path under `cargo test`: pass `--nocapture`, and put a probe
  in a function you know runs.** If the control reads zero, your measurement is
  of the channel, not the code.
- **Attributing output to tests: `--test-threads=1`.** Adjacency under
  parallelism is not evidence.
- **Give every pin a vacuity control** — assert the population is non-empty
  before asserting anything about its members. It is what catches a subject
  chosen from a bad attribution, and it costs one line.
- **Prefer a counter you assert on inside a test to a line you read out of a
  log.** A counter cannot be captured, interleaved, or misattributed.

Related:
[[a-negative-check-passes-for-any-reason-so-it-needs-a-positive-control]],
[[a-probe-truncated-before-the-grep-is-not-a-measurement]],
[[exact-and-single-threaded-do-not-give-a-test-more-stack-libtest-always-spawns]].
