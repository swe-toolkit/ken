---
id: RT-NATIVE-CONTINUATION-ENV-CARRIAGE
title: "A native function-typed recursive position whose closure escapes through a word-only call result gets its captures from a compile-time side slot, so two constructions of the same continuation cannot be told apart. Carry the environment in the value: the closure word is a handle to the captures its construction wrote"
status: ready
owner: runtime
size: L
tier: T1
gate: architect
depends_on: []
blocks: [RT-NATIVE-SEQUENTIAL-BRACKETS]
github: null
origin: "Architect recut ruling evt_4x84ykwjtmbsh on RT-NATIVE-SEQUENTIAL-BRACKETS (§1a 2), on the research advisory evt_7rqnbfnw80fe: an escaping continuation's environment must travel in the value (closure conversion). Size is provisional and is re-set at AC-0. Steward-filed per COORDINATION section 2."
---

# Continuation environments travel in the value

## Objective

A native continuation closure that escapes through a word-only call result
reaches each consumer with the captures of the construction that produced
it. No compile-time association between construction and consumer remains.

## Settled inputs (T2 `evt_5ktetkd8esyeg`, Architect `evt_4x84ykwjtmbsh`)

- **The escape.** The continuation is emitted as a direct call to a
  separate function, and only words cross, through
  `call_declared_unit_target` (`calls.rs:2067`). The result returns as
  `LoweringOperand::Carried(CarriedBoundaryWord)`. Everything that
  identified the construction is dropped at that projection
  (`core.rs:11732`).
- **The consumers** both run later in the same function, on descendants of
  the carried result:
  - the gate, through `resolve_recursive_unit_body`;
  - the retarget, through `call_declared_recursive_position_unit`.

  Nothing in hand at either reader identifies the construction, and W2 and
  W7 share every static coordinate.
- **Superseded, not repaired.** The `constructed_context_frame` side slot
  is a static association in every form tried: a single slot, a keyed map
  and a scoped stack. So is a runtime "latest slot per key", because W2's
  value can still be pending when W7 executes.
- **The ruled form.**
  - The caller allocates a per-call environment buffer in its own stack
    frame and passes its address across the call.
  - The callee writes the closure's captures into it, and the closure word
    is a handle to that buffer.
  - This holds only while every consumer runs within the caller's frame.
    The heap or arena form is the fallback.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

After an Architect-ruled AC-0, the chosen carriage form for every censused
site, with the side slot retired.

## Acceptance

- **AC-0 (measure and design; no build).**
  - **Census.** Every native site where a function-typed recursive-position
    closure with captures is built inside a value that crosses
    `call_declared_unit_target` or a peer word-only call. Record capture
    counts, and which consumers later eliminate or call the closure.
  - **Lifetime.** For each consumer, whether it runs within the calling
    function's frame, or whether the value can be returned, stored or
    otherwise outlive it. This decides caller buffer against heap or arena.
  - **ABI proposal.** How the buffer address crosses the call, and how the
    closure word encodes the handle. Say how `resolve_recursive_unit_body`
    and the `BoundaryCarrier` guard sit alongside it, and what retires.
  - The Architect rules the form and sets the size and increments. The
    ruling is recorded as an ADR in `docs/adr/`, which lands with the first
    build increment.
- **AC-1.** The `RT-NATIVE-SEQUENTIAL-BRACKETS` two-bracket witness builds
  and runs natively, and its observation matches the interpreter.
- **AC-2 (controls).**
  - Two constructions of one continuation (W2 and W7) reach their
    consumers with their own captures. A mutation that shares one buffer
    turns that row red.
  - The one-bracket control and the landed native census show no verdict
    change.
  - The IR passes the Cranelift verifier.

## Stop conditions

- A consumer that outlives the caller's frame, where the ruled form is the
  caller buffer: stop to the Architect.
- Any kernel, `trusted_base()` or spec change: an operator question.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40` or
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY`.
