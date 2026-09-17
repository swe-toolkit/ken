# RT-CONTEXT-FRAME-REFUSAL-DEPTH-CENSUS — work package

**Owner: Team Runtime. Size M. Tier T1. Gate: none.**
**Implementation base: `origin/main` as of whenever you cut your branch. NAME
IT in your first post — AC-0 requires you to re-measure at your own base.**
**Inputs below measured at `b0421afd0816c44347bb3281a5062cc0ada00c8e`.**

**THIS IS A MEASUREMENT NODE. NOTHING LANDS IN `crates/` FROM IT.** The
deliverable is a census and a verdict. It is the gate in front of any fourth
repair node on these four rows.

## 1. Objective

**How many refusals deep are the four rows, and is the stack finite?**

Walk each row's refusal stack: force past the current stop, record the next
stop, repeat. Report the depth per row, the identity of every stop, and
whether the stack bottoms out.

## 2. WHY THIS IS A CENSUS AND NOT A FOURTH REPAIR

**Three nodes in this series each closed one layer and discovered the next.
Each one found that the refusal at its own layer was CORRECT and the cause was
upstream.** That is the same finding three times, arrived at three times by
building a repair first and measuring second.

    RT-CARRIED-RESIDUAL-IH-ARITY     the arity refusal is CORRECT at all four
                                     sites and must not be relaxed; the arity
                                     message is a FALLBACK SYMPTOM. Cause
                                     upstream.
    the label-correction node         the labels asserted a mechanism and a
                                     cardinality comparison that the code does
                                     not make. Cause upstream of the labels.
    RT-CONTEXT-FRAME-SLOT-HOLDS-      REFUTED on its own premise. The slot is
    ONE-PER-FUNCTION                  written EXACTLY ONCE per compile, not
                                     overwritten; and keying it by
                                     worker_body_origin readmits NOTHING.
                                     Cause upstream again.

⇒ **Cutting a fourth repair blind is a fourth cycle to learn the same shape.**
The queue of refusals has never been measured as a queue. **Measure it, then
cut one repair at the layer the measurement names** — or discover the stack
does not bottom out, which is itself the answer and changes the disposition
from "repair" to "exempt".

> ### THE MEASUREMENT WE HAVE, AND EXACTLY WHAT IT DOES NOT SAY
>
> The refuted node forced the admission arm at `core.rs:13577` to return
> `Ok(true)` **unconditionally** — strictly more permissive than any key,
> which is what makes its negative result an **upper bound** rather than a
> failed attempt. The four rows still failed, at `agreeing_recursive_body_unit`
> (`core.rs:1230`):
>
>     plain Match branches declare different recursive body units: N versus M
>
> That refusal is **deliberate and carries its own two-direction unit test**
> (`core.rs:1271`: `[41,41]` agrees, `[41,42]` refuses). It is not a
> fallthrough.
>
> **It was measured to its FIRST STOP AND NO FURTHER.** Whether `core.rs:1230`
> is the last layer or the next entry in a queue is **NOT established**. That
> gap is this node.

## 3. Fixed inputs, measured at `b0421afd0`

**The four rows**, and their stored-versus-queried body origins as the refuted
node measured them:

    px7l_checked_host_recursive_bind.rs
      delayed_capturing_generic_bind_agrees_across_real_executors
        continuation_origin 11, recursive_position 1, stored wbo 343, sibling 322
        3 worker captures, 3 context captures
      runtime_selected_non_unit_response_is_consumed_across_real_executors
        continuation_origin 11, recursive_position 1, stored wbo 362, sibling 347
        3 worker captures, 3 context captures

    px7m_hostresult_computational_match.rs
      dynamic_ok_payload_selects_a_multistep_tree_across_real_executors
        continuation_origin 11, recursive_position 1, stored wbo 332, query 369
        4 worker captures, 3 context captures
      dynamic_err_payload_selects_a_multistep_tree_across_real_executors
        continuation_origin 12, recursive_position 1, stored wbo 341, query 378
        5 worker captures, 4 context captures

**Treat every number here as perishable.** They come from the refuted node's
probe and are re-measurable but not re-measured at your base. AC-0 covers it.

**The probe is reproducible from `main` and the procedure is published** —
`evt_41nd1exp7vbb0` carries five anchors that each resolve exactly once in
`core.rs`, so the patch applies with no line numbers. `--test-threads=1` is
**load-bearing, not hygiene**: parallel `cargo test` interleaves stderr, and
attributing a printed line to the header above it is not a measurement.

## 4. Deliverables

1. **A per-row refusal stack**, ordered, each entry naming: the file and
   function of the stop, its message, and what had to be forced to get past it.
2. **A depth number per row**, and whether the four rows share a stack or
   diverge.
3. **A verdict on finiteness** — §6's outcome menu.
4. **The probe reverted.** Nothing from this node lands in `crates/`.

## 5. THE ONE DISCIPLINE THIS NODE LIVES OR DIES BY

**Forcing past a deliberate refusal is a MEASUREMENT TECHNIQUE. It is never a
repair, and a forced arm is never evidence that the arm should be relaxed.**

Every refusal you force past in this census has, so far, turned out to be
**correct**. `agreeing_recursive_body_unit` has a two-direction unit test
asserting it refuses. **A green row reached by forcing is not a fixed row — it
is a row whose remaining guards you have disabled.**

⇒ **Record what you forced, at every layer, beside every result.** A depth
count with no record of what was disabled is not a census; it is a claim.

> **`scripts/ken-cargo`, scoped. Never `--workspace`** — `-p ken-runtime
> --lib` to materialize, then `-p ken-cli --test <suite>`. The full build runs
> in CI, not here (`COORDINATION §12`).

## 6. OUTCOME MENU — READ THIS BEFORE THE CRITERIA

**Report the outcome you measured. If none of these fits, name the one it is
closest to, say how it differs, and hand it back.** Three Steward outcome menus
in this lane have been exhaustive over the wrong axis, and each time the real
answer sat in the gap.

    A  FINITE, repairable.   The stack bottoms out at a layer that is a defect.
                             Name that layer. It becomes ONE repair node, cut
                             by the Steward. Do not repair it here.

    B  FINITE, terminal.     The stack bottoms out at a refusal that is CORRECT
                             and not going to move. Then these rows are
                             correctly refused, they are not a defect, and the
                             disposition is an exemption row plus a rewritten
                             label -- NOT a repair. This is a real and useful
                             outcome; do not treat it as failure.

    C  NOT BOTTOMED OUT.     You hit the budget (§8) still finding new stops.
                             Report the depth reached and the last stop.
                             "Unbounded so far" is the answer; it still
                             forecloses a fourth blind repair.

    D  THE ROWS DIVERGE.     Different rows bottom out differently. Then the
                             four rows were never one cluster, and that is the
                             finding -- the ledger grouped them by a
                             byte-identical MESSAGE, which is a claim about the
                             symptom, not about the cause.

## 7. Acceptance criteria

**AC-0 — THE FOUR ROWS STILL FAIL THE SAME WAY AT YOUR BASE. RUN THIS FIRST.**
Run the four rows `--ignored` at your own base and paste the failures. **If any
row now passes, or fails with a different message, STOP and return to the
Steward.** Every number in §3 was measured at `b0421afd0` by a probe that has
been reverted; `core.rs` is the lane's hot file, and a repair landing under
this node changes what it is counting.

**AC-1 — every stop is identified by FILE, FUNCTION and MESSAGE.** A line
number alone is not an identity: it resolves a position, and the claim here is
*which guard refused*. Resolve the enclosing function.

**AC-2 — every forced arm is recorded beside the result it produced.** What you
disabled, where, and what the next stop was. §5 is the reason.

**AC-3 — the stack is reported PER ROW, never summed.** Four rows with depths
`{1,1,1,4}` and four rows with depths `{2,2,2,1}` both total seven and mean
entirely different things. **State the split, not the sum.**

**AC-4 — a stop reached by MORE THAN ONE row is stated as shared.** If the four
stacks converge, that convergence is the most useful thing this node can
produce — it is where one repair would serve four rows. Say so explicitly
rather than leaving it to be read off a table.

**AC-5 — the verdict cites the outcome letter from §6**, and if it is `D`, it
says what the ledger's grouping actually grouped.

## 8. Budget, and what to do when you hit it

**Six forced layers per row, or one turn, whichever comes first.** At the cap,
report outcome `C` with the depth reached. **Hitting the cap is a result, not a
failure** — it forecloses the fourth blind repair just as well as a bottom
does, and it does it without spending another cycle.

**Do not carry a half-walked stack into a second turn silently.** Post the
depth you reached and stop.

## 9. What this node is NOT

- **Not a repair, and not a repair's design.** Outcome `A` names a layer; the
  Steward cuts the node.
- **Not a relaxation of any guard.** Every forced arm is reverted.
- **Not the soundness question.** `[[RT-CONTEXT-FRAME-ADMISSION-EVIDENCE-KEY]]`
  asks whether one gate may key on a subset of the frame's identity. That is
  independent of how deep these rows are, and neither node blocks the other.

## 10. Contention

`core.rs` is the lane's hot file, but **this node lands nothing in it.** The
probe is applied and reverted inside the turn. The only durable artifact is the
census document. **Coordinate only on the probe anchors if another node is
editing that region concurrently.**

## 11. Estimated tier: T1

**The hard part is refusing to read a forced-green row as a fixed row**, and
judging whether a given stop is principled or accidental. That is the judgment
that failed three times in this series when the same work was done as a side
effect of building a repair.

## 12. Sizing note

**M.** The mechanics are a published probe applied four times. **The cost is
the per-layer judgment**, and §8's cap is what keeps it inside one turn.
