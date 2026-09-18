---
id: RT-NATIVE-ELIDED-READ-TRACE-DIVERGENCE
title: "Native's host-effect trace has no `FsReadAt` where the interpreter's has one, on the same program, in six measured rows -- every one a two-level bracket nest whose inner body makes exactly one host call. On four of the six the exit codes AGREE, so every exit-status-keyed test in the repository is green on them and the native/interpreter effect-trace comparison is the only instrument in the tree that can see this class. No cause is named: the read may not have happened, or it happened and the artifact's emitter omitted the event, and nothing buildable separates those."
status: draft
owner: runtime
size: M
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "Measured by runtime-implementer during RT-BRACKET-RELEASE-ORDER-PARITY D0c, 2026-09-18, at origin/main ad29bd81b86431519b33f32a63a46af16ec8478f. The cell was reached by accident: D0c needed a plain successful two-level read to separate two counting hypotheses, the Architect observed that the rt_parity fixture family contains no such row because all three cr-read-* rows are guard-FAILURE variants and every dispatching row carries a third bracket or a second call, and the first fixture built into that cell diverged. Steward directed the filing (evt_6z9brnymd784e) after gating on a complete run; node written by runtime-implementer, reviewed by runtime-leader. Every fixture named here was uncommitted scratch, reverted after measurement; the diff is archived as d0c_probes_final.patch and fixtures are rebuildable from it. Agents cannot create tracked work (COORDINATION section 2): this node exists on the Steward's direction."
---

> ## THE COVERAGE FINDING IS THE PART THAT SHOULD ALARM THE NEXT READER
>
> Four of the six diverging rows have **identical exit codes on both
> engines** -- 51/51, 51/51, 0/0, 0/0. A test that asserts on exit status
> passes on all four. The divergence is visible only in the ordered
> host-effect trace, and it was found the first time a fixture entered a
> cell this family had never contained.
>
> **No cause is named anywhere in this node.** Two dispositions remain and
> they predict the identical observation.

## 1. METHOD BOUNDARY: FIVE SEPARATING CELLS COULD NOT BE REACHED

This is a limit on the investigation, stated first so that nobody re-runs it
expecting a different result. Five distinct cells, each of which would have
discriminated between two live hypotheses, could not be built:

    identical-projection control   two same-operation calls whose consumer
                                   projections are EQUAL. Every surface
                                   spelling forces an unused binder, which is
                                   itself the refusing shape.
    fixH  three pure buffer nests  "unsupported runtime-IR lowering:
                                   ContinuationSpecialization: the claimed
                                   continuation target was not declared into
                                   this function"
    fixI  four pure buffer nests   "unsupported runtime-IR lowering:
                                   ComputationalMatch: recursive field expects
                                   1 args but call provides 0"
    byte-value witness             `spanBytes`/`freeze` is a HOST OPERATION
      in the one-call body         (38-ffi-io.md:423-426), so observing the
                                   bytes puts a second host call in the body,
                                   and a second host call is the condition
                                   under which native dispatches. The
                                   observation destroys the phenomenon. This
                                   cell was specified by the Steward.
    fixK8  unconsumed second       "unsupported runtime-IR lowering:
      effect                       ContinuationSpecialization: the claimed
                                   continuation target was not declared into
                                   this function"

Four compiler refusals and one design-level impossibility. **The discriminator
is not empirically separable by this method in the current lowering.**

`fixG` builds at the same depth as `fixK8` with a control-dependent nested
bracket, while `fixK8` refuses when the same bracket is made unconditional.
That is a difference and not a measurement.

## 2. THE BOARD

    ELISION CELL       6 rows   fixK0, fixK1, fixK2, fixK4, fixK5, fixK6
    COVERAGE CLAIM     4 rows   fixK0, fixK1, fixK2, fixK4 -- exits AGREE
                                fixK5/fixK6 exit native 1 / interp 0, so an
                                exit-keyed instrument WOULD catch those two

    (i-a) the runtime's POST-PROCESSING lost the event     EXCLUDED
          An independently captured trace, from a spawn replicating
          `run_bound_process_effect_observation` at every line except that the
          observation file survives, also has no `FsReadAt`.
    (i-b) the ARTIFACT emits a trace missing an event
          that happened                                    OPEN
          Narrowed by `fixK7`: native DOES record `FsReadAt` there, so the
          emitter is capable of it. (i-b) can no longer be "the emitter is
          lossy"; it must be SELECTIVE omission in exactly the six rows.
    (ii)  the read did not happen                          OPEN
    (iii) the data arrived by another path                 REFUTED, MEASURED
          `FsOpen` and `BufferAllocate` replies are
          `Success(ResourceAcquired { schema_version, resource_kind,
          identity })` -- three bookkeeping fields, no data, no span. No
          mapping operation occurs in any of these programs.

(i-b) and (ii) differ on one thing only -- whether the read happened -- and
both predict the identical trace.

### The elision is reproduced; the trap is not

`fixK1` reproduced on four separate builds, `fixK2` on three, `fixK0`,
`fixK4` and `fixK7` on two, with identical event lists and exits each time.
Separate builds, separate temp directories, separate processes. **That
evidence covers rows that elide and survive; it does not cover the trapping
rows**, which have run once each.

## 3. RELEASE ORDER: THE DIVERGENCE IS WIDER THAN THE ELISION CELL

Both engines, all sixteen rows, release order read off the trace:

    case                    native rel   interp rel   native dispatched
    cr-read-offset            [1,2]        [2,1]            no
    cr-read-window            [1,2]        [2,1]            no
    cr-read-norights          [1,2]        [2,1]            no
    fixA                      [1,2]        [2,1]            no
    fixB                      [1,2]        [2,1]            no
    fixK0                     [1,2]        [2,1]            no
    fixK1                     [1,2]        [2,1]            no
    fixK2                     [1,2]        [2,1]            no
    fixK4                     [1,2]        [2,1]            no
    fixK5                     [1,2]        [2,1]            no
    fixK6                     [1,2]        [2,1]            no
    cr-write-readonly         [2,1]        [2,1]            YES
    cr-write-writable       [3,2,1]      [3,2,1]            YES
    fixG                    [3,2,1]      [3,2,1]            YES
    fixJ                    [3,2,1]      [3,2,1]            YES
    fixK7                     [2,1]        [2,1]            YES

**The interpreter releases strict LIFO in all sixteen. Native releases LIFO in
exactly the five rows where it dispatched a host call, and in acquisition
order in the other eleven.**

    release-order divergence      11 rows
    read-event divergence          6 rows      (the elision cell)

⇒ **The release-order divergence is WIDER than the elision.** Five rows --
`cr-read-offset`, `cr-read-window`, `cr-read-norights`, `fixA`, `fixB` --
release in acquisition order on native while both engines agree the read was
narrowed at lowering and no read event is owed. **The read is entirely correct
in those rows and the release order still diverges.**

An earlier draft of this node offered those five as a control showing that
acquisition-order release "is not itself the defect". They cannot serve as
that control: they were defined by a two-engine property (neither engine
dispatches) and read on one engine (native's release order), and once
interp's column is measured they turn out to be divergent on the release axis
themselves. What they do establish is narrower and still useful:
**acquisition-order release is not CAUSED BY the elision**, because it occurs
where there is no elision.

**Eleven rows raises both dispositions together and discriminates between
neither.** Under (ii) native genuinely releases in acquisition order in eleven
rows, a product defect wider than the elision cell. Under (i-b) native's
emitter misrepresents order in eleven rows, an instrument defect wider than
the elision cell. The blast radius grew on both sides and the discriminator
did not move; "eleven" is not evidence for the product reading.

**No shared cause is proposed.** The split is perfect across sixteen rows --
native releases LIFO if and only if native dispatched -- and a perfect split
is exactly the shape this campaign has twice found to be one partition read
off two variables.

## 4. DECLINED OUT LOUD: THE READING THIS NODE DOES NOT MAKE

The account that assembles itself from two adjacent facts:

> the read was elided, so the buffer was never filled, so the value carried
> out was malformed, so the entrypoint returned `-1`.

**It is not asserted here.** Its status is: supported by both rows that can
exhibit it and separated from nothing.

- `fixK5` and `fixK6` are the only payload-READING rows in the elision cell.
  Both trap, which this reading predicts.
- A rival predicts the same trap. The `-1` sentinel is a verdict on the
  entrypoint's RETURN VALUE, at the boundary where it becomes an `ExitCode`
  and downstream of the destructuring -- so the field read produced something
  rather than dying.
- **`fixK2` and `fixK4` cannot test it.** Their continuations return constants
  and carry no payload-derived value out, so a clean exit is what the reading
  PREDICTS for them. They are tag-only rows and the mechanism operates on
  payload-reading ones.

The trap's stderr string, `ken native trap: malformed borrowed process input`,
is **not a diagnosis**. The generated C shim prints it because `value == -1`
(`object_linker_packaging.rs:2294-2310`), from a fixed ladder of six cases. It
is a label on the sentinel, not a report about what happened.

## 5. DOWNSTREAM

**`RT-SUBCONTINUATION-LIFO-RELEASE-ORDER` shares this emitter, and the
release-order question is wider than these rows.**
Its evidence is two filters over `observation.effect_trace`
(`px8ta_oriented_subcontinuation.rs:508-520`), obtained from
`run_bound_process_effect_observation`
(`object_linker_packaging.rs:284`) -- the same read-decode-delete path. Verified
independently by runtime-implementer and runtime-leader.

**And the repository's strict-LIFO ORDER assertion runs in CI at only one
depth, where order is degenerate.** `assert_depth_finishes_and_releases_lifo`
has two call sites:

    depth 1        RUNS in CI. `releases == opens.rev()` on a one-element
                   vector is satisfied by any correct single release, so the
                   ORDER half is vacuous there; what the row's readmission
                   note shows it actually pins is release IDENTITY.
    depths 2-3     `#[ignore]`d. Depth 2 EXECUTES when run and the order
                   assertion fails with content -- observed `[1,2]`, expected
                   `[2,1]`, deterministic across 20 fresh processes. Depth 3
                   refuses at object emission (`ContinuationSpecialization`)
                   and never executes.

So the order assertion is vacuous at the depth CI runs, has real content at
depth 2 but only under a manual run of an ignored row, and cannot execute at
depth 3. Its `opens` filter also matches `FsOpen` only, so a nest formed by
`BufferAllocate` -- `fixA`'s shape, and five of the eleven divergent rows --
is invisible to it even when it does run.

The claim that node can still make is *"the trace shows releases in acquisition
order."* The claim it can no longer make for free is *"this trace is a faithful
record of what depth 2 acquired and released."* **(i-b) is about OMISSION, and
an emitter that omits a read is not thereby one that misorders releases** --
the finding is not compromised, its burden has moved.

**`RT-BRACKET-RELEASE-ORDER-PARITY`'s carried lead did not survive section 3,
and that node is `ready`.** This campaign was conducted under it. Its title on
current main states, flagging the pair as carried from 2026-09-03 and un-rerun:

    interp releases outer-then-inner on the composed-return fixtures, where
    native is CORRECT

Section 3 is that re-measurement, and both halves invert:

    claimed   interp releases outer-then-inner on the composed-return nests
    measured  interp [2,1], strict LIFO, on ALL FIVE

    claimed   native is CORRECT on those
    measured  native [1,2] on cr-read-offset, cr-read-window,
              cr-read-norights; correct only on cr-write-readonly and
              cr-write-writable

So *"both engines violate it, on different nests"* does not hold: only native
violates, on eleven of sixteen rows, and the interpreter violates nowhere.
**The contrast that node calls "the lead", and the six properties said to
co-vary across it, have no subject left** -- which is the outcome that node's
own text anticipated when it said that if the native half no longer holds
there is no contrast left to explain.

*Fixture identity, checked by the ring:* the five rows measured here are the
five `withResource`-rooted nests in `rt_parity_native.rs` outside the `cap41`
family, and the two coordinates that node cites for the composed-return shape
-- `:184` and `:336` -- fall inside two of them. Residual, stated: the `cap41`
stages also form `withResource`/`withBuffer` nests and were not measured here,
so if that node's "five" included any of them the sets are not identical.

Nothing is asked of that node here. This is a cross-reference so that a
refuted lead is not handed to whoever picks it up.

**The decoder implements a strict subset of its own ABI.** The C shim
enumerates six cases (`-1` through `-4` reserved entrypoint sentinels, the
tagged trap-token space, and other negatives); `decode_signed_root_trap`
understands one. Four named reserved sentinels and genuine garbage collapse
into a single `UnclassifiedRuntimeTrap`, and the error path discards both
stderr and the already-decoded trace. A named diagnostic the process actually
produced is replaced by a sentinel. Queued, not opened.

## 6. WHAT WOULD DECIDE IT

**An instrument that is not native's emitter.** Both open dispositions predict
the same native host-effect trace, so no fixture written against that trace
can separate them. The planner's own event set, or an observation of the
process's syscalls, would.

One half of that requirement is already met and is worth stating precisely,
because it is easy to overclaim: **native and interp traces do not come from
the same emitter.** Native's is the linked artifact writing the wire format
that `decode_linked_effect_trace` reads; interp's comes from
`run_program_effect_observation` over a `PosixHost`. So the cross-engine
comparison is NOT self-referential, and the divergence is not an artifact of
one instrument disagreeing with itself. That establishes the comparison, not
the content: it does not show either trace is faithful, and it does not
separate (i-b) from (ii), because both are claims about the native side
alone.

Until then this node records a measurement and a coverage gap, and names no
cause.
