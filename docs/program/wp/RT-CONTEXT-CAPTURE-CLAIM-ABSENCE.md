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

> **`L1` AND `L3` ARE THE SAME PRESENCE PREDICATE ON THE SAME FIELD** — §1a.A.
> Read "depth 3" as **two distinct predicates plus a convergence result**, not
> as three independent layers. **Convergence is the measured claim and the one
> this node rests on; finiteness is NOT.**

**The `context_capture` writers in `planning/`, ALL occurrences, with the
production/test split — CORRECTED 2026-09-18, see the amendment at §1a:**

    continuations.rs:830     draft.context_capture.map(..).transpose()?  PROD  pass-through
    continuations.rs:1001    the field declaration                       PROD
    continuations.rs:4560    context_capture: capture,                   PROD  a variable
    continuations.rs:4625    context_capture: Some(claim),               PROD  only literal Some
    continuations.rs:10280   context_capture: None,                      TEST
    continuations.rs:10403   context_capture: None,                      TEST
    continuations.rs:10423   context_capture: None,                      TEST
    continuations.rs:10491   context_capture: Some(declared),            TEST
    continuations.rs:10541   context_capture: Some(EntryFrame { .. }),   TEST

`#[cfg(test)] pub(in crate::cranelift_backend) mod tests` spans `8324`-`11079`
(brace-depth and column-0 scans agree). **There is NOT ONE production
`context_capture: None` literal in `planning/`.** Closure on the population:
**zero** assignment-form writes (`.context_capture =`) anywhere in
`crates/ken-runtime/src/`, so the struct-literal census is the complete set of
writers.

**Production `None` is decided ONE HOP ABOVE the arm.** `:4560`'s `capture` is a
forward, not a decision:

    let capture = predeclared_entry_frame_slot(plan, *emission_owner, input.coordinate)?
        .map(|declared_slot| ContinuationEnvironmentDraft::EntryFrame { .. });

Production `None` arises exactly when `predeclared_entry_frame_slot` returns
`Ok(None)`. Its own comment (`continuations.rs:4546-4548`): *"`None` when the
frame declares no member: fails closed. Nothing invents a position, and no
fallback reads the direct-emission index as a frame slot."*

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

## §1a. AMENDMENT 2026-09-18 (Steward) — four Architect findings, all verified

Raised by the Architect at `evt_4hzm4praxvdrq` against this frame while it was
`ready` and unreleased. **Every claim below re-measured by the Steward at
`e75f1fe27` before folding.** The census's own base `0298c51eb` is an ancestor
of main, 23 commits behind, with `continuations.rs` and `core.rs` UNCHANGED
since — **its coordinates are live and need no re-derivation.**

**A. `L1` AND `L3` READ THE SAME FIELD, AND `L1`'S DOC SAYS SO.** `core.rs:13594`
is `Ok(claims.iter().all(|claim| claim.availability.context_capture.is_some()))`
— the same presence test `L3` unwraps at `core.rs:9557`. `L1`'s doc comment,
`core.rs:13527-13531`, verbatim:

> *"`context_capture` is the field `resolve_context_capture_claim` consumes; a
> `None` there is precisely the refusal this gate must respect rather than route
> around."*

**The census forced `L1` to an unconditional `Ok(true)` — the "route around"
that comment names — and reported meeting `L3` as a discovery about the
program.** ⇒ **DEPTH 3 OVER-COUNTS: two distinct predicates were traversed, not
three.** Not a claim of set identity: `L1` quantifies over the context's
`Capture` run, `L3` resolves one coordinate's views, and whether the second is a
member of the first is a further measurement.

**B. FINITENESS DOES NOT SURVIVE; CONVERGENCE DOES.** The census's verdict says
*"FINITE at depth 3"* while its own limits section says *"not proof that nothing
lies behind `L3`... a repair there may expose an `L4`."* Both cannot hold.
**Under `B` finite-at-3 is right; under `A` a repair issues a real claim, `L3`
passes for real, and what is behind it is UNMEASURED.** ⇒ **`A`'s correct label
is "repairable, depth beyond `L3` UNKNOWN".** A single document at a single base
disagreeing with itself, and the half that reached this frame is the half
phrased as a result — **a limits section is written in the register of caveats,
so it reads as hedging rather than as a finding and does not get relayed.**

**"One disposition serves all four" rests on CONVERGENCE, not on finiteness**,
and convergence is measured and solid. Ground it there.

**C. THE PRODUCTION POPULATION IS NOT SIX ARMS.** §1 as first written named six
literal arms; **five are `#[cfg(test)]` fixtures.** The Steward's error was
treating a PATH (`planning/`) as a production filter — `continuations.rs`
carries an inline `#[cfg(test)] mod tests`. ⇒ **`D0` as first written was
unanswerable: no production compile reaches `:10280`.** The direction was right
— the census's single-arm evidence base was a real defect, and banning *"the
producer"* as singular stands. It is the replacement population that needed
deriving.

**D. A HAZARD CLOSED RATHER THAN HANDED ON.** A claim cannot be produced and
lost in transit (which would make the consumer's `None` underdetermine the
producer — the `occurrence: None` shape from RT-CARRIER). `:830` is
`draft.context_capture.map(finalize).transpose()?`: `None` in gives `None` out,
`Some` in gives `Some` out or a propagated `Err`. **No `Some`-to-`None` collapse.
The transport is FAITHFUL, so `D0` may reason from producer to consumer.** One
fewer thing for the ring to establish.

**E. THREE READERS, AND ONE SELECTS RATHER THAN REFUSES.** Every read in
`crates/ken-runtime/src/`:

    core.rs:13594   .all(.. .is_some())    L1, ADMISSION
    core.rs:9557    let Some(claim) ..     L3, REFUSAL
    calls.rs:982    .any(.. .is_none())    gather_cannot_serve, ROUTE SELECTION

**`calls.rs:982` does not refuse, it SELECTS.** Issuing a claim where there was
none flips `gather_cannot_serve` and changes which route `calls.rs` takes —
**silently, and outside the four rows this node fences.** ⇒ **Outcome `A` is not
a local repair**, and if `A` is taken `calls.rs:982` needs a named disposition
in the same node. This is the measured reason behind §0's *"do not arrive at `A`
because `A` produces code."*

> **THE METHOD FINDING, worth more than any single correction above.** Forcing a
> guard demands surgical precision about WHERE it is and zero understanding of
> WHY it exists: you must locate the exact arm to replace, and nothing in that
> operation makes you read the paragraph above it. **A forcing census
> systematically under-reads the guards it is most intimately engaged with.**
> Here the doc comment that answered the census's own open question sat four
> lines above the line it edited. **Whenever you force an arm, read its
> enclosing doc comment first and quote it beside the result.**

**What the Architect explicitly did NOT rule: `A` versus `B`.** `A.` and the
`predeclared_entry_frame_slot` comment are two statements of design **intent**,
at the gate and at the producer, both pointing at `B`. **Comments are evidence
about intent, never proof of correctness.** They do not rule `B`; they make the
question cheaper and better posed.

## §2. Deliverables

- **`D0` — DOES THE PREDECLARED ENTRY FRAME DECLARE A MEMBER?** RE-AIMED by
  §1a.C/F3; the original *"which of the six arms"* was unanswerable because five
  are test fixtures. Production `None` has exactly one source: for each of the
  four rows' coordinates, does `predeclared_entry_frame_slot` return `Ok(None)`,
  and on which condition? **Report per row, never summed.** §1a.D licenses
  reasoning from producer to consumer — the transport is faithful.
- **`D1` — IS THE DECLINATION DELIBERATE?** Read the condition `D0` lands on.
  State whether it is **deliberate with a stated reason** or a **fallthrough
  with none**, quoting the condition from the tree. **Read the enclosing doc
  comment and quote it too** — §1a's method finding is that a forcing pass
  under-reads exactly these paragraphs.
- **`D2` — RULE `A` OR `B`.** With `D0`+`D1` in hand. If the ruling turns on
  what the planner *ought* to do rather than what it does, that is a mechanism
  question and it goes to the **Architect**, not to this seat.
- **`D3` — DISPOSITION, which follows from `D2` and not before it.**
  - Under `B`: an exemption row plus the `D4` label rewrite. **No repair.**
  - Under `A`: size the repair as a **SUCCESSOR node** and hand it back to the
    Steward. Do not build it here. **`A`'s disposition is NOT "repair, relabel,
    un-ignore."** Two things follow from §1a and a ring will otherwise infer
    neither: (1) **a repair RE-OPENS the census** — `A` means "repairable, depth
    beyond `L3` UNKNOWN", so the successor must re-walk, and a row that reaches
    `L3` legitimately may meet an `L4` nobody has seen; (2) **`calls.rs:982`
    needs a named disposition in that same successor** — it SELECTS on this
    field rather than refusing, so issuing a claim changes a route silently,
    outside the four rows this node fences.
- **`D4` — REWRITE THE FOUR `#[ignore]` LABELS.** Required under BOTH outcomes.
  The live labels still say *"READMISSION CONDITION UNKNOWN: ... whether
  `core.rs:1230` is the last layer or the next in a queue is NOT established"*.
  **The census established it: `core.rs:1230` is `L2` of 3, and `L3` is
  `resolve_context_capture_claim`.** Each new label must state the depth-3
  stack, name `L3` as the live stop, and carry this node's `D2` ruling.

## §3. The third possibility `A`/`B` does not cover — name it before you measure

`D0` has three outcomes, not two. **Re-based by §1a onto
`predeclared_entry_frame_slot` rather than onto the arm list:**

    (i)   it returns Ok(None) on a stated condition  -> D1 reads that condition
    (ii)  it returns Ok(Some(..)) and the row still
          meets L3 with None                         -> CLOSED, see below
    (iii) it is never called for this coordinate     -> the producer never ran

**(ii) IS ALREADY CLOSED — do not spend a measurement on it.** §1a.D establishes
the transport is faithful (`:830` is `map(..).transpose()?`, no `Some`-to-`None`
collapse) and there are zero assignment-form writes to the field. If `D0`
somehow observes (ii) anyway, that refutes §1a.D and is a finding about this
frame, not a branch to explore.

**(iii) is neither `A` nor `B` as the census framed them.** "The planner did not
issue a claim" would then be true for a reason no condition describes, and the
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
- **`AC-9` — CLOSE OBLIGATION. This node does not reach `merged` until each of
  its four rows either CLEARS or carries a LIVE owner at the START of its own
  `#[ignore]` string.** The four are `px7l_checked_host_recursive_bind.rs:163`
  and `:241`, `px7m_hostresult_computational_match.rs:163` and `:206`. If the
  answer for a row is that no live node owns its next step, the attribute says
  so in those words, naming this node as the one that established it.
  *(Control: each of the four satisfies exactly one branch at the candidate's
  base. **(a) CLEARED** — the row is no longer `#[ignore]`d; there is no reason
  string and no token to resolve, and this satisfies the AC. **(b) LIVE OWNER**
  — the first node token in the reason string resolves to a node whose status is
  `ready`, `active` or `draft`; `merged` and `closed` fail, and
  present-and-terminal fails exactly as absent does. **(c) NO LIVE OWNER** — the
  reason string opens by stating in words that no live node owns the next step
  and names this node as having established that, which leaves a reader a record
  to go to rather than a dead end; the reviewer's check is that branch (b) was
  unavailable, not a token resolution. A row that names a terminal node without
  taking branch (c)'s words fails. Because it touches `crates/`, the candidate
  carrying this edit is **`full` CI, never doc-only**.)*

  > **Added by Steward amendment 2026-09-19. These four rows are why the check
  > exists, and today they fail it.** All four currently open
  > `#[ignore = "RT-CONTEXT-FRAME-LABEL-CORRECTION …` — `merged` — and every
  > later token they name (`RT-CARRIED-RESIDUAL-IH-ARITY` `closed`,
  > `RT-CONTSRC-PRODUCER-LOCAL` and `RT-SITEOP-CARRIED-WITNESS` both `merged`)
  > is terminal too. **This node is their live owner and its frame is the only
  > place that says so**, by `file:line` at §1 — so a reader who starts at the
  > row, which is where anyone clearing it starts, sees a merged node and a dead
  > end. Measured at `evt_73fa9v0ca03ve`: 8 of the 14 selected rows are in this
  > state and four of them are these.
  >
  > **`AC-6` above is not this AC.** It requires the `D4` label to state the
  > depth-3 stack, name `L3`, and carry the `D2` ruling — all of which is
  > *mechanism*, and a row can satisfy every word of it while still naming a
  > dead owner. This AC is about **who to go to next**, which is the axis `AC-6`
  > does not constrain.
  >
  > **Why an AC when `M7a` arm 2 already reaches these rows at close.**
  > `M7a` is a **gate**, run by the merge seats when the node closes. An AC is a
  > **design input**, read by the implementer before the work and by QA during
  > review. **Reachability by the gate is not visibility to the author** — the
  > distinction the runtime-implementer drew at `evt_719q9chqy9dwz` after
  > measuring arm 2's reach and retracting the redundancy claim it seemed to
  > license. The same reasoning is why
  > `RT-SITEOP-ORPHANED-ROWS-REFUSAL-CENSUS` carries `AC-11` despite arm 2.
  >
  > **This paragraph defends the EXECUTION-time reading only, and the teeth are
  > not in the paragraph.** What stops this AC being skipped as ceremony is that
  > **its control is already RED**: all four rows fail it as of `338a46e05`, and
  > an AC that is already failing cannot be satisfied by doing nothing. The
  > prose explains why the AC exists; the red control is what makes it
  > unignorable. Noted because the reverse — a close obligation whose control
  > passes on the tree that motivated it — would be ceremony no wording could
  > rescue.
  >
  > The **proposal**-time failure is a different axis and this text does not
  > close it: concluding an AC should not exist, before one is written, never
  > reaches the paragraph that would have answered it. That belongs with `M7a`'s
  > own general-form box, not here (runtime-implementer, `evt_5fj3d9ez285na`).
  >
  > **What this AC does NOT license.** The census's `AC-6` proposal will name
  > this node as the expected applier for these four rows. That is a **routing**
  > statement and it is backed by measurement. The **mechanism content** of the
  > successor string is this node's own finding to make — the census did not
  > measure these rows and may not write their labels. For `:206` in particular
  > the live possibility is that the refusal is **correct** (its two arms
  > measured `DIFFER, 1164 vs 1168 bytes, in exactly one leaf`), in which case
  > the row CLEARS and this AC is satisfied by the first branch, not the second.

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
