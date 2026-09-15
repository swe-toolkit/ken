---
id: RT-OBSERVATION-EXIT-STATUS-LAUNDERS-SIGNAL-DEATH
title: "run_bound_process_effect_observation's exit_status is output.status.code().unwrap_or(1), so a child killed by a signal and a child that genuinely exited 1 become the same integer at the point of capture. The substitute value is chosen, not measured, and the only reason the laundering was ever detectable is that 1 happens to be outside the exit vocabulary of the one program that hit it."
status: ready
owner: runtime
size: S
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "Read out of the producers at 0f71ab5b9 by the Architect at evt_625fkk2ecck7d (2026-09-15), while correcting the Steward's exit-route hypothesis during the D5b M5a escalation. The Architect ran nothing and explicitly did not establish which arm fires on that failure; this node is about the site, not about that failure. Architect asked for it filed rather than folded into the candidate. Steward-filed per COORDINATION section 2."
---

## The defect

`crates/ken-runtime/src/object_linker_packaging.rs:346`, in
`run_bound_process_effect_observation_with_stdin`:

```rust
let exit_status = output.status.code().unwrap_or(1);
```

`std::process::ExitStatus::code()` returns `None` when the child was terminated
by a **signal**. `unwrap_or(1)` turns that into `1`. At this call site a
SIGSEGV and a well-formed `exit(1)` are the same integer, by construction, and
the observation carries the substitute onward as though it were measured.

**The substituted value is a choice, not a measurement, and nothing downstream
marks it as one.**

## Why it has been survivable, and why that is not reassuring

Ten lines below, the same function computes a field that *does* distinguish
them:

```rust
} else if output.status.code().is_none() {
    Some(ken_host::TerminalErrorV1::DriverFailure)
```

So the information is not lost from the observation as a whole — it is lost
from `exit_status` specifically. Any consumer that reads `exit_status` without
also reading `terminal_error` is reading a value that may be fabricated, and
the type does not say so.

The laundering became visible on 2026-09-15 only because the program that hit
it has the exit vocabulary `{0, 81, 82, 83, 84}` — so `1` was impossible and
therefore conspicuous. **A program whose vocabulary includes 1 would have
produced a plausible, wrong, unremarkable number.** Detectability here was a
property of the victim, not of the instrument. That is the whole finding.

## What this node is NOT about, measured the same day it was filed

This was found while diagnosing the D5b CI red, and **the laundering was not
what caused that red.** runtime-implementer answered it at both trees with
fresh artifacts and reported the arm directly: `code()` was **not** `None`, the
child was **not** signal-killed, and `terminal_error` came back
`RuntimeTrap(PatternMatchFailure)` — a Ken-level trap, exiting 1 through the C
starter's `if (value < 0) return 1;` path, which is a real and producible 1.

That does not weaken this node and it must not be read as refuting it. The
finding was always about the **site**, not about that failure — the Architect
said so when raising it, having run nothing. The site still substitutes a
chosen value for absent information, and a signal-killed child still arrives
downstream as an ordinary `1`. What the measurement removes is any temptation
to treat this node as the D5b repair, or to close it by fixing D5b.

**Recorded here because the opposite mistake was made on this arc once
already**: a node filed mid-hunt was read as the hunt's cause, and stayed
`ready` on a premise its own author had retracted. A node filed during a hunt
needs its relationship to that hunt written down while both are fresh.

## The design fork, which is the Architect's call and is deliberately not ruled here

Two shapes, with different blast radii:

- **Make the absence representable.** `exit_status` stops being a bare `i32`
  that can hold a fabricated value. This is the honest shape and it ripples to
  every consumer of `ProcessEffectObservation`.
- **Keep the integer, remove the fabrication.** Leave the field's type alone
  but stop inventing a value — and make it impossible to read `exit_status`
  meaningfully without `terminal_error`, rather than merely conventional.

The first is correct and costs a sweep; the second is cheap and leaves a
convention where an invariant should be. **Route this to the Architect before
building either.** Do not pick the cheap one because the node is sized S.

## Deliverables

- **D0 — the fork ruled**, by the Architect, with the ruling recorded at the
  site rather than only in a thread.
- **D1 — the repair**, shaped by D0. Whatever the shape: no caller may read a
  fabricated exit status while believing it measured one.
- **D2 — a control.** A child killed by a signal and a child exiting with the
  same integer must be distinguishable through the observation API, and the
  control must be demonstrated to **red against the pre-repair tree**. A
  control that has only ever been seen green is not yet known to test
  anything.

## The sibling deliverable: assertion order destroys the diagnostic

`crates/ken-verify/tests/px8f_write_partition.rs:315-316`:

```rust
assert_eq!(result.observation.exit_status, expected_exit);   // fails here
assert_eq!(result.observation.terminal_error, None);         // never evaluated
```

The least informative assertion runs first, so the one that would name the
cause never runs. The 2026-09-15 CI failure therefore reported `left: 1, right:
0` and nothing else, in a native run that costs ~240-310s and had already been
paid for.

- **D3 — order the assertions so the naming one runs first**, here and wherever
  else this pair appears. This is not a style preference: an exit-status
  equality is a *symptom* check and `terminal_error` is the *cause* check, and
  a failing symptom check that preempts the cause check converts a diagnosis
  into a re-run. Census the pairing rather than fixing only this row.

  **This row is already done, in the D5b respin.** The Steward ruled that the
  three-line diagnostic that produced the `RuntimeTrap` reading stays in that
  candidate rather than being reverted and refiled here. So D3's remaining
  scope is **every OTHER occurrence of the pairing**, and its census must be
  taken against the tree *after* the respin lands. Do not re-fix
  `px8f_write_partition.rs:315-316`; do use it as the worked example of the
  shape you are looking for.

## Acceptance criteria

- **AC-1.** With a child killed by a signal, a consumer of the observation can
  tell that from a child that exited with the same integer. Demonstrate with a
  real signalled child, not a unit test of the mapping.
- **AC-2 (positive control).** D2's control is shown red on the pre-repair tree
  and green after. Both directions, both shown.
- **AC-3.** D3's reordering is applied across the censused population, and the
  census states how it was enumerated so the denominator is checkable.
- **AC-4.** No regression, green in CI (never a local `--workspace` run;
  COORDINATION §12).

## Sequencing and contention

**Queued behind the active lane.** Touches `object_linker_packaging.rs`, which
the D5b candidate does **not** modify — `git diff --numstat` against
`1dec48f33` is empty for it. That independence is exactly why this must not be
folded into the D5b respin: same reasoning that kept
`RT-EFFECT-CATALOG-GLOBAL-ARTIFACT-COUPLING` out of it. Two unrelated
attributions in one diff make the candidate unreviewable.

**D3 contends with the D5b respin** if that respin edits the same test file.
Sequence D3 after it lands.

## Related

- `CI-WRITE-PARTITION-JOB-COMMENT-STALE` — same class, filed the same day: a
  site that substitutes a plausible claim for the true state, where the
  substitution is invisible at the point of reading.
- `RT-TEST-HARNESS-STATICLIB-CURRENCY` — the arc's third instrument defect, and
  the same underlying shape: an instrument whose own inputs are unverified.
