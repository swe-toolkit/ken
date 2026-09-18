# `RT-CONTEXT-CAPTURE-CLAIM-ABSENCE` — frame

**Owner:** runtime. **Size:** M. **Tier:** T1. **Gate:** none.
**Ground SHA:** `origin/main` `e75f1fe2768f1996c582bad6c6832e1e1716fc24`.
Every coordinate in this frame was resolved by the Steward at that SHA **by
symbol**; `file:line` is a HINT and never the anchor.

Node: `docs/program/issues/RT-CONTEXT-CAPTURE-CLAIM-ABSENCE.md`.
Its input: `docs/program/evidence/rt-context-frame-refusal-depth-census.md`.

## §0. Posture

**This node is a DIAGNOSIS. It may land zero `crates/src/` files and still
succeed.** Its deliverable is a ruling between `A` and `B` plus the label
rewrite that ruling implies. `B` — the refusal is correct and permanent — closes
this node green.

**Nothing lands in `crates/` from `D0`-`D2`.** Every forced arm or probe is
applied and reverted inside the turn, recorded beside the result it produced.
Forcing past a deliberate refusal is a **measurement technique, never a repair**,
and a forced arm is never evidence the arm should be relaxed.

`scripts/ken-cargo`, scoped: `-p ken-runtime --lib`, then `-p ken-cli --test
px7l_checked_host_recursive_bind` / `--test px7m_hostresult_computational_match`.
**Never `--workspace`** (`COORDINATION §12`). `--test-threads=1` is load-bearing
for any run that reads printed output: parallel `cargo test` interleaves stderr,
and attributing a printed line to the header above it is not a measurement.

## §1. Fixed inputs, measured at `e75f1fe27`

**The stack, from the census (`0298c51eb`), re-verified here by symbol:**

    L1  recursive_position_captures_all_planner_recoverable   core.rs:13532
    L2  agreeing_recursive_body_unit                          core.rs:1230
        its own two-direction unit test                       core.rs:1271
    L3  resolve_context_capture_claim                         core.rs:9551
        the absence arm, `let Some(claim) = ... else`         core.rs:9557

**The six `context_capture:` producer arms in `planning/`, ALL occurrences:**

    continuations.rs:4625    Some(claim)
    continuations.rs:10491   Some(declared)
    continuations.rs:10541   Some(ContinuationEnvironmentDraft::EntryFrame { .. })
    continuations.rs:10280   None
    continuations.rs:10403   None
    continuations.rs:10423   None

**The four rows** — file and line, function name confirmed:

    px7l_checked_host_recursive_bind.rs:163   delayed_capturing_generic_bind_...
    px7l_checked_host_recursive_bind.rs:241   runtime_selected_non_unit_response_...
    px7m_hostresult_computational_match.rs:163  dynamic_ok_payload_...
    px7m_hostresult_computational_match.rs:206  dynamic_err_payload_...

**Per-row operands from the census** (`L1` admission queries, `L2` unit pairs):

    px7l:163  O(343) then O(322)      px7m:163  O(369) then O(332)
    px7l:241  O(362) then O(347)      px7m:206  O(378) then O(341)

Note the ordering is not uniform: in `px7l` the stored origin is the HIGHER of
the pair, in `px7m` the LOWER. **Any hypothesis explaining the divergence as
"the frame always stores the later body" is refuted by this pair.** The labels
also record a structural difference — `px7l` issues TWO admission queries (one
admitted, one falling through), `px7m` a SOLE query that falls through. The
rows converge at `L3` regardless; they are not identical upstream of it.

## §2. Deliverables

- **`D0` — WHICH ARM.** For each of the four rows, determine which of the six
  `context_capture:` arms the row's coordinate takes, or that it takes none.
  Report per row, never summed.
- **`D1` — WHY THAT ARM.** Read the taken arm's guarding condition. State
  whether the declination is **deliberate with a stated reason** or a
  **fallthrough with none**, quoting the condition.
- **`D2` — RULE `A` OR `B`.** With `D0`+`D1` in hand. If the ruling turns on
  what the planner *ought* to do rather than what it does, that is a mechanism
  question and it goes to the **Architect**, not to this seat.
- **`D3` — DISPOSITION, which follows from `D2` and not before it.**
  - Under `B`: an exemption row plus the `D4` label rewrite. **No repair.**
  - Under `A`: size the repair as a **SUCCESSOR node** and hand it back to the
    Steward. Do not build it here.
- **`D4` — REWRITE THE FOUR `#[ignore]` LABELS.** Required under BOTH outcomes.
  The live labels still say *"READMISSION CONDITION UNKNOWN: ... whether
  `core.rs:1230` is the last layer or the next in a queue is NOT established"*.
  **The census established it: `core.rs:1230` is `L2` of 3, and `L3` is
  `resolve_context_capture_claim`.** Each new label must state the depth-3
  stack, name `L3` as the live stop, and carry this node's `D2` ruling.

## §3. The third possibility `A`/`B` does not cover — name it before you measure

`D0` has three outcomes, not two:

    (i)   the coordinate reaches a `None` arm         -> D1 reads its condition
    (ii)  the coordinate reaches a `Some` arm and the
          claim is lost or overwritten downstream     -> a DIFFERENT defect
    (iii) the coordinate reaches NO arm at all        -> the producer never ran
                                                         for this coordinate

**(iii) is neither `A` nor `B` as the census framed them.** "The planner did not
issue a claim" would then be true for a reason no arm describes, and the
question moves upstream again — to what decides whether the producer runs.
**If `D0` reports (iii), STOP and hand back.** Do not convert it into `A` on the
grounds that a claim is missing; that is the same substitution that put three
refuted nodes in this series.

## §4. Instrument discipline

- **Prefer the number the producer already emits.** Before adding a probe, check
  whether this path is already instrumented — `mod.rs` carries `#[cfg(test)]`
  recorders with take-and-clear semantics (`d2k_owner_trace_take`) whose
  per-compile attribution is structural rather than by convention. A second
  instrument measuring what the first already measures is cost with no signal,
  and only yours needs reverting.
- **Assert a POSITIVE token every run.** A clean run that prints nothing is
  indistinguishable from an instrument that never executed. Each row's
  measurement must emit something naming the arm, so silence is a failure and
  not a pass.
- **A measurement must be able to report (iii).** An instrument keyed on the six
  arms can only ever name one of six; it cannot see "none". Whatever `D0` uses
  must distinguish *"reached arm X"* from *"reached no arm"*, and the frame's
  `AC-2` control exists to prove it can.

## §5. Hard stops — stop and report, do not work around

1. **`D0` reports (iii).** §3.
2. **The ruling requires knowing what the planner OUGHT to issue.** Architect.
3. **Any repair candidate would relax a refusal under `lowering/` outside
   `core/tests/` whose reason is an ABSENT claim or an ABSENT planner-issued
   record.** That is `L3`'s class and it is out of scope for this node under
   every outcome. Escalate; do not write it.
4. **A row stops somewhere the census did not record.** The census is `0298c51eb`
   and main has moved; a changed stack is a finding about the census, and `AC-0`
   is where it surfaces.

## §6. Acceptance criteria

- **`AC-0` — BASELINE AT THIS NODE'S BASE.** Run all four rows `--ignored
  --nocapture --test-threads=1` at the branch base **before any forcing**, and
  record the message per row. **If the stack differs from the census's depth-3
  finding, that is `D0`'s first result and hard stop 4 applies.** The census
  measured at `0298c51eb`; do not inherit its baseline.
- **`AC-1` — `D0` REPORTED PER ROW, NEVER SUMMED.** Four rows, four arm
  attributions (or (iii)). A single aggregate answer fails this AC even if
  correct, because the rows are not identical upstream of `L3` (§1).
- **`AC-2` — THE INSTRUMENT'S NEGATIVE CONTROL.** Demonstrate on a program whose
  arm is known that the instrument names it, **and** demonstrate that it reports
  "no arm" rather than silence when no arm is taken. **An instrument that cannot
  fail cannot report (iii).**
- **`AC-3` — `D1` QUOTES THE CONDITION.** The arm's guard is quoted from the
  tree, not paraphrased, with its symbol named. A paraphrase of a guard is the
  artifact that de-aims when the file moves.
- **`AC-4` — THE `D2` RULING NAMES ITS COUNTER-EVIDENCE.** State which
  measurement would have produced the other outcome and confirm it was not
  observed. A ruling that cites only confirming evidence fails this AC.
- **`AC-5` — EVERY FORCED ARM RECORDED AND REVERTED.** Named beside the result
  it produced; `git status` clean at handback; no row left passing by forcing.
- **`AC-6` — `D4` LANDED ON ALL FOUR ROWS.** Each label states the depth-3
  stack, names `L3`, and carries the `D2` ruling. **Required under `B` as much
  as under `A`** — a terminal refusal with a label that still says "unknown" is
  the defect this node was cut to stop repeating.
- **`AC-7` — NO REPAIR IN THIS NODE.** Zero `crates/src/` changes. If `D2` rules
  `A`, the repair is a successor node handed back to the Steward.
- **`AC-8` — NO REGRESSION.** Green in CI, not a local `--workspace` run
  (`COORDINATION §12`). Under `A` or `B`, `core.rs:1271`'s two-direction unit
  test must be untouched and green.

## §7. Contention

Touches on a read: `lowering/core.rs`, `planning/static_transition/
continuations.rs`. Lands in: the two `ken-cli` test files (labels), this frame,
and the node.

**`RT-CARRIER-PRODUCER-OCCURRENCE` is the runtime ring's active node** and
touches `lowering/aggregates.rs` and `core/tests/constructors.rs`. Path
intersection with this node is empty, **but the fleet is single-threaded and
this node is NOT released** — it is `ready` and queued behind the active L1 work.
Its `BAN 4` — *no refusal whose reason is a missing planner-issued occurrence
may be relaxed anywhere under `lowering/` outside `core/tests/`* — is a
**class**, and `L3` here is a refusal on an ABSENT claim. §5's hard stop 3 keeps
this node on the same side of that line. **Do not read the two as separate
permissions.**
