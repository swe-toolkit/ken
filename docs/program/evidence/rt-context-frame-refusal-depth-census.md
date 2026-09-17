# `RT-CONTEXT-FRAME-REFUSAL-DEPTH-CENSUS` — the per-row refusal stacks

**Base: `origin/main` at `0298c51eb08df955c502a4fa2bfd17e315745e22`.**
Nothing here lands in `crates/`; every forced arm was reverted inside the turn
and the working tree is clean.

## `AC-0` — the four rows still fail the same way at this base

Run `--ignored --nocapture --test-threads=1` at `0298c51eb`, before any
forcing. All four fail, all four with the identical message:

    unsupported runtime-IR lowering: BoundaryCarrier: a carried recursive
    hypothesis is an eliminated value, not a callable, so it takes no
    arguments, but the call provides 1

§3's inputs were measured at `b0421afd0`; this base is past it, and `#3878` is
in flight against both test files. **The labels move, the behaviour does not.**

## The stacks

**Depth 3 on every row. All four converge, same stops in the same order.**

    row                                          L1        L2        L3
    px7l delayed_capturing_generic_bind          arity  ->  branch ->  claim
    px7l runtime_selected_non_unit_response      arity  ->  branch ->  claim
    px7m dynamic_ok_payload                      arity  ->  branch ->  claim
    px7m dynamic_err_payload                     arity  ->  branch ->  claim

Per-row operands differ; the stops do not.

### L1 — `recursive_position_captures_all_planner_recoverable`, `core.rs:13532`

    BoundaryCarrier: a carried recursive hypothesis is an eliminated value,
    not a callable, so it takes no arguments, but the call provides 1

Reached via `resolve_recursive_unit_body` returning `Ok(None)`, which drops
control to the zero-argument route; the arity text is that route's report, not
this gate's.

**FORCED BY** replacing the three-condition frame arm with an unconditional
`return Ok(true)` — strictly more permissive than any key.

    row   admission queried for
    px7l  O(343) then O(322)     |  px7m  O(369) then O(332)
    px7l  O(362) then O(347)     |  px7m  O(378) then O(341)

### L2 — `agreeing_recursive_body_unit`, `core.rs:1230`

    ComputationalMatch: plain Match branches declare different recursive
    body units: O(343) versus O(322)        (per row: 362/347, 369/332, 378/341)

**FORCED BY** replacing the `expected != unit` refusal with a print.

**This guard carries its own two-direction unit test at `core.rs:1271`** —
`[41,41]` agrees, `[41,42]` refuses on the exact message. **Forcing it reddens
that test**, which is not a side effect to apologise for: it is direct evidence
the guard is deliberate and covered, measured rather than assumed.

### L3 — `resolve_context_capture_claim`, `core.rs:9551`

    ContinuationSpecialization: a generated context capture carries no
    context-capture availability claim, so nothing says where this frame
    holds ProducerLocal { ... }; RT-CONTSRC-PRODUCER-LOCAL D3b refuses
    rather than reading the direct-emission claim, whose index counts
    binders in a lexical environment this consumer does not hold

**NOT FORCED. The technique terminates here, and that is the finding.**

## Why L3 is a different KIND of stop, and why the census stops at it

    L1  guard REJECTS data that is PRESENT   (a frame exists; its key mismatches)
    L2  guard REJECTS data that is PRESENT   (two units exist; they differ)
    L3  the data is ABSENT                   (`views.context_capture` is None)

`L3` is `let Some(claim) = views.context_capture else { return Err(...) }`.
**Nothing is being rejected.** Forcing `L1` and `L2` meant *removing a
rejection of data the compiler already held*. Forcing `L3` would mean
**fabricating a `ContinuationEnvironmentClaim` the planner never issued** — and
every stop found after that would be a fact about my fabrication, not about the
program.

**That is a principled stop, not a budget stop.** The six-layer budget was not
reached; the census reached the depth at which forcing stops measuring
anything.

⇒ **The claim is absent because the planner did not issue one.** A producer
site exists (`continuations.rs:4625`, `context_capture: Some(claim)`), so this
is not an unimplemented field — it is a coordinate for which no claim was
produced.

## Verdict — `AC-5`

**The stack is FINITE at depth 3 and shared by all four rows. The outcome is
`A` or `B`, and this node cannot discriminate between them.**

    A  finite, repairable   iff the planner SHOULD have issued a claim for
                            this coordinate and did not
    B  finite, terminal     iff no claim can exist here, and the refusal is
                            correct and permanent

**Both fit every measurement in this document.** The discriminator is a
planner-side question — *is `RT-CONTSRC-PRODUCER-LOCAL D3b`'s missing claim a
gap or a correct absence?* — and answering it means reading the producer's
conditions, which is a different node's work.

**Not `C`:** the budget was not reached. **Not `D`:** the rows do not diverge;
they converge completely, which is `AC-4`'s shared-stop case at every layer.

## What this changes

**One repair, not four, and not sixteen.** All four rows stop at the same
place for the same reason, so whatever disposition `L3` receives serves all
four. The ledger grouped these rows by a byte-identical L1 message and fenced
that as a claim about the symptom rather than the cause; **walking the stack
confirms the grouping survives to the bottom** — which the ledger could not
say and deliberately did not.

## Stated limits

- **Depth 3 is where FORCING stops being a measurement, not proof that nothing
  lies behind `L3`.** If the planner question resolves to `A`, a repair there
  may expose an `L4` this census never saw.
- **Every layer's result is conditional on the layers forced beneath it.** `L3`
  was reached only with `L1` and `L2` disabled; it is what these rows meet when
  two correct guards are removed, not what they meet today.
- **No row was made to pass and none should be read as passing.** Every forced
  arm was reverted; the four rows fail at `L1` on `main` exactly as `AC-0`
  records.
