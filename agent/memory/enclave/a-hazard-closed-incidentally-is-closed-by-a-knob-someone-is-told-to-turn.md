---
scope: enclave
audience: (see scope README)
source: RT-TEST-SCRATCH-RAII fixed-name census, 2026-08-14
---

# A hazard closed incidentally is closed by a knob someone is told to turn

Four test fixtures acquire a fixed `/tmp` root with `create_dir_all` and then
`fs::write` beneath it — `accept-existing`, so a pre-created path redirects the
write. Two dispositions were argued and both were settled by measurement rather
than by shape:

- **Security route — closed.** The exposure needs a second local user to
  pre-create the path; `/workspaces/ken` is a single-user devcontainer and
  GitHub-hosted runners are single-tenant per job. A real mechanism is not a
  reachable threat.
- **Flake route — closed, but by something unrelated.** Concurrent seats across
  worktrees share one `/tmp`, so two `-p ken-cli` runs would meet. They don't:
  `scripts/ken-cargo` holds a `flock` for the **whole invocation** (the shell
  keeps fd 9 across `run "$@"`, so build *and* test execution are serialized).

**The second closure is the interesting one, because the mutex is not for
this.** `ken-cargo`'s own header says it exists *"so that N parallel agents don't
oversubscribe CPU or OOM the box during compilation/linking."* Path exclusion is
a **side effect**. And the relaxation is not hypothetical: `scripts/ken-cargo`
says *"Raise it as hardware grows"* and `docs/ops/compute-budget.md` carries a
**written scaling path** — *"Raise in this order: `KEN_BUILD_SLOTS` (2, 4, …)"*.
So a documented procedure exists whose first step reopens the hazard, and
whoever follows it will have read the mechanism's purpose **correctly** and will
have no way to know it is load-bearing for something else.

**The placement consequence, which is where I was wrong first.** I said the
*fact* belongs in the census (right — the next author adding a fixture reads it)
and that the *trigger* belonged "wherever runner topology is decided; if there is
no such artifact, the census is the best available home." There **was** such an
artifact and I had not looked for it. The trigger line goes beside the knob, on
the scaling-path page — the person raising the cap will never read a test-fixture
census.

**How to apply.**

- **When a hazard turns out closed, ask *by what*, and whether that thing exists
  for this purpose.** Closed-on-purpose stays closed. Closed-incidentally is
  closed until someone optimizes the mechanism for the reason it actually has.
- **Then find the artifact that tells someone to change it.** A knob with a
  documented raise procedure is a scheduled reopening, not a latent one. Grep the
  knob's name repo-wide; the scaling/ops page is usually where it lives.
- **Fact and trigger have different readers, so they get different homes.** Write
  the fact where the person who would *add another instance* stands; write the
  trigger where the person who would *invalidate the closure* stands. Same
  discipline as the `plan_sequence` write-back note — the constraint sits where
  the breaker is standing, and here those were two different people.
- **A closure that rests on an unrelated mechanism is worth one sentence even
  when nothing is owed.** It costs a line and it is the only thing standing
  between a correct local optimization and a silent reopening.

Sibling of [[a-gate-flag-bounds-its-own-path-not-every-consumer-of-the-mechanism]]:
there a guard was narrower than its readers assumed; here a guard is *wider* than
its stated purpose, which is the same mismatch pointed the other way.
