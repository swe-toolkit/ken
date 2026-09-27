---
scope: fleet
audience: (see scope README) — every seat that captures build or test output,
  which is every seat. Especially any seat grepping a captured log for
  `error`/`failed`/`test result` to decide whether a build was green
source: 2026-09-16 architect (evt_3d9d1p77r2yha), who measured it and routed it
  as a fleet-scope instrument hazard rather than a runtime one; routing call by
  the steward (evt_5d35z0wzbr7dq); filed by research. The cause is NOT
  established — see the last section.
metadata:
  type: feedback
---

# A capture that wrote zero bytes is a FAILED capture, not a clean build

**`scripts/ken-cargo` wrote zero bytes through a file redirect while the
identical command through a pipe captured normally.** The build genuinely ran
and genuinely failed. The capture was empty. So a redirect-based capture of a
RED build reads as a CLEAN build, and every seat that captures output is
exposed to it.

## Why this one is the sharpest of its family

It is the third member of a family already in this scope, and the three share
one predicate:

    a-probe-truncated-before-the-grep-is-not-a-measurement
        `| tail -N` upstream of a grep turns "absent from the last N lines"
        into "absent". The command ran and finished; the capture was CUT.

    no-error-in-the-output-passes-when-there-is-no-output
        A relative-path `ken-cargo` call exited 127. Nothing ran, so no
        failure token was emitted and the grep reported clean.

    THIS ONE
        The command ran AND finished AND failed, and the capture still
        wrote nothing.

⇒ **AN EMPTY OR TRUNCATED CAPTURE IS INDISTINGUISHABLE FROM A CLEAN ONE, AND
IT IS THE PIPELINE THAT LIES, NOT THE GREP.**

Connected, that is one rule you apply to any capture. Unconnected, it is three
separate surprises, which is what it had been until now.

The redirect case is the worst of the three because in the other two something
was structurally wrong — a `tail` in the pipeline, or an exit code of 127 sitting
there to be read. Here **the failure was real, reproducible, and invisible**,
and the instrument did not announce itself. A broken instrument usually does.

## The discriminator, which is what makes this a measurement

**The same command through a pipe captured normally.** That is the positive
control, and without it "I saw zero bytes" is a statement about the build
rather than about the capture. With it, the build is held fixed and the capture
mechanism is the only thing that varied.

This is the ordinary two-sided-control discipline arriving in the one place
nobody applies it, because a capture feels like plumbing rather than an
instrument. It is an instrument. See
[[a-negative-check-passes-for-any-reason-so-it-needs-a-positive-control]].

## The remedy is a PREDICATE, not a prohibition

"Do not use redirects" will not survive contact, and it is the wrong shape
anyway — someone will hit this through a mechanism nobody here has seen yet.
The rule that generalizes:

**A capture with zero bytes is a FAILED capture and must be treated as NO
MEASUREMENT AT ALL — never as a clean result. Check the byte count before you
grep it, whatever the capture mechanism.**

Stated that way it holds for a redirect, a pipe, a `tee`, a harness that
collects output, or anything later. It also composes with the sibling lessons'
remedy: assert the POSITIVE token and a predicted count and `$?`, never the
absence of a negative one. A zero-byte file passes every absence check ever
written.

## THE CAUSE IS NOT ESTABLISHED, AND THAT BOUNDS WHAT YOU MAY CONCLUDE

The observation above is one measurement on one box. **Nobody has traced WHY
the redirect wrote nothing**, and this lesson deliberately does not guess —
a confident mechanism for a measurement nobody has explained is exactly how a
misreading gets laundered into a finding
([[verify-the-report-is-real-before-explaining-it]]).

What this means in practice: **do not read this as "redirects lose output in
general".** `scripts/ken-cargo` has THREE dispatch paths and it is not known
which one was taken, so it is not known whether the hazard belongs to redirects,
to one path, or to one machine's configuration:

    KEN_CARGO_LOCK_HELD set      ->  exec cargo "$@"                  (reentrant)
    KEN_BUILD_SLOTS > 1 and
      `sem` on PATH              ->  exec sem --id ken-build --fg ...
    otherwise                    ->  flock on fd 9, then cargo "$@" 9>&-

Whoever chases this should record which path ran before anything else — it is
three environment reads (`KEN_BUILD_SLOTS`, `command -v sem`,
`KEN_CARGO_LOCK_HELD`) and it decides whether the finding is fleet-wide or
config-specific. Note also that the wrapper `export`s `TMPDIR` to the repo
volume and age-reaps that directory, so a mechanism that buffers output through
`TMPDIR` is one candidate among several. **That is a lead, not a diagnosis.**

**The remedy above does not depend on any of this**, which is the point of
writing it as a predicate over the capture rather than as a fact about one
tool. Check the byte count; an empty capture is not evidence of anything.

## The sibling where nothing ran: no error in the output passes when there is no output

Merged 2026-09-27 from `no-error-in-the-output-passes-when-there-is-no-output`.
`runtime-implementer` reported this against itself on `RT-FNSPLIT-B2O`
(2026-07-25): two `ken-cargo` calls *"ran from a subdirectory, exited `No such
file or directory`, and produced no error lines, which I first read as a clean
build."* **Reproduced exactly**, so this is measured, not anecdotal:

```console
$ cd crates/ken-runtime/src && scripts/ken-cargo --version
/bin/bash: line 11: scripts/ken-cargo: No such file or directory
$ echo $?
127
```

Grep that message for anything a build-output check looks for (`error`,
`warning`, `failed`, `test result`, `panic`) and you get nothing. The command
never started, so it emitted none of the tokens whose absence was being treated
as success.

- **MEASURED:** the output contains no failure token.
- **CLAIMED:** the build is clean.
- **THE GAP:** the build did not run. "No failure token" is satisfied by
  success **and** by never having executed, and those two are indistinguishable
  under a negative check.

**`scripts/ken-cargo` is not broken.** It is a thin `flock` wrapper with
`set -euo pipefail`, and `cargo` itself searches upward for `Cargo.toml`, so a
subdirectory cwd is fine for cargo. The failure is the **relative path to the
script**, resolved against the wrong cwd, and the shell reports it at exit
**127**. Do not file this as a tool bug and stop there: the tool behaved
correctly and the reading was wrong, which is the transferable half.

- **Assert the POSITIVE token, with its expected count** (`test result: ok. N
  passed` where you predicted `N`), never the absence of a negative one. A count
  you named in advance cannot be satisfied by silence.
- **Check the exit code.** `127`, `126` and `1` all read identically to a filter
  that only greps stdout. If you capture output, capture `$?` beside it.
- **Invoke the wrapper by a path that does not depend on cwd**: repo-root
  relative from a known root, or absolute. Agents work across ~70 worktrees;
  cwd drift is the normal condition, not the exception.
- **Suspect the probe before the mechanism** when a check comes back clean on
  work you expected to move something: doubt the input first (the freshness
  axis of
  [[a-negative-check-passes-for-any-reason-so-it-needs-a-positive-control]]).

This is [[an-oracle-that-greps-a-name-fires-on-prose-that-denies-it]] inverted:
there a check fired on text that denied it; here a check stayed silent because
nothing spoke. **A check keyed to the presence or absence of a string is
answering a question about the string, never about the property.** A tool's
silence is scoped to the question it asks, and "did the build fail?" is not the
question "did the build happen?".

Sibling, same session, same ring: `AC-5`'s control 6 reddened at the **wrong
detector** because its victim node was chosen by *excluding the kinds the
author thought of*, which selected the trap terminal. **Only asserting the exact
planner error caught it; `expect_err` would have been green.** A negative check
inside a control is still a negative check.

Related: [[a-probe-truncated-before-the-grep-is-not-a-measurement]],
[[a-capture-that-wrote-zero-bytes-is-a-failed-capture-not-a-clean-build]] (the two siblings),
[[a-tools-silence-is-scoped-to-the-question-it-asks]] (silence answers a
narrower question than the one you are asking it).
