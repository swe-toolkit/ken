---
id: RT-NATIVE-CONTINUATION-ENV-CARRIAGE
title: "A native function-typed recursive position whose closure escapes through a word-only call result gets its captures from a compile-time side slot, so two constructions of the same continuation cannot be told apart. Carry the suffix in the existing value-carried closure environment record"
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
- **The ruled form** (Architect `evt_759pn6f60mmfx`, which withdrew the
  caller-stack buffer). Extend the existing `BoundaryClosureEnvironment`
  record with the context-capture suffix.
  - `emit_boundary_closure_environment` (`aggregates.rs:4032`) already
    emits a per-construction environment record in the value. The planner
    picks its owner lane: `PersistentStore`, or `InvocationArena` when a
    child is arena-referent.
  - The recursive-position continuation's context-capture suffix never
    joined that record, and so it travelled in the side slot.
  - A handle into a stack buffer would outlive its frame: `units.rs:9526`
    returns a carried word unchanged from a non-root unit. The ABI stays
    word-only.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

After an Architect-ruled AC-0, every censused recursive-position closure
carries its context-capture suffix in its `BoundaryClosureEnvironment`
record, and the side slot is retired.

## Acceptance

- **AC-0 (measure and design; no build).**
  1. **Suffix census.** For each field `ConstructedContextFrame` records
     beyond the lexical captures, name the `ir::Value` that holds it at
     that closure's `emit_boundary_closure_environment` call. A field not
     in hand there is a named enabler gap.
  2. **Record coverage.** For W1, W2, W7 and every other producer form,
     state whether the closure goes through a planner-issued record or an
     unplanned or raw path. Unplanned and capture-free raw closures stay
     refused (`boundary.rs:1031-1054`).
  3. **Lane.** State whether appending the suffix changes
     `fixed_node_selected_owner` for any row. The existing lifetime proof
     (planning `aggregates.rs:9002-9023`) must cover the added children as
     written.
  4. **Readers.** Show that the gate and the retarget can decode the
     suffix from the environment word they already hold a descendant of,
     with absence as a refusal, as `checked_ih_captured_environment_record`
     does.

  The Architect rules the form and sets the size and increments. The ruling
  is recorded as an ADR in `docs/adr/`, which lands with the first build
  increment.
- **AC-1.** The `RT-NATIVE-SEQUENTIAL-BRACKETS` two-bracket witness builds
  and runs natively, and its observation matches the interpreter.
- **AC-2 (controls).**
  - Two constructions of one continuation (W2 and W7) reach their
    consumers with their own captures. A mutation that shares one record
    turns that row red.
  - The one-bracket control and the landed native census show no verdict
    change.
  - The IR passes the Cranelift verifier.

## Stop conditions

- A suffix field that is not in hand at the record's emission, or a lane
  the existing lifetime proof does not cover: stop to the Architect with
  the gap. Do not add a second carriage mechanism.
- Any kernel, `trusted_base()` or spec change: an operator question.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40` or
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY`.
