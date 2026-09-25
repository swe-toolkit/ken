---
id: RT-PLANNER-SEED-BINDING-ORDER
title: "Planner lexical-walk seed must follow the owner's source-body binding order: for a converting owner (CallableDeclaration, ClosureBody) with two or more entry parameters, every EntryAbi coordinate the continuation walk attaches names the mirror parameter, so coordinate-keyed consumers load the wrong ABI slot and copy the wrong carrier and lifetime metadata; seed the walk with the emitter's own ordering function, keep ABI-run consumers in ABI order, and prove it with a two-sided two-parameter witness"
status: merged
owner: runtime
size: M
gate: architect
tier: T1
depends_on: []
blocks: [RT-SELECTED-PENDING-CALL-BUILD]
github: null
origin: "RT-SELECTED-PENDING-CALL-BUILD AC-0(e) STOP (runtime-implementer evt_4dbywywkr59a5, e2 != e3 under D1 amendment 2 evt_6yjef2cy4nv1e). Architect ruling evt_2eqe033bxff6f set the defect, D0, witness and fix direction and asked the Steward to frame the node. Serves the L1 objective. Steward-filed per COORDINATION section 2."
---

# The planner labels a converting owner's parameters in mirror order

## Objective

For every owner, each `EntryAbi` coordinate the planner attaches to a
lexical index names the parameter that actually sits at that index, so a
consumer keyed on the coordinate reads the same value as a consumer keyed on
the index.

## Fixed inputs -- Architect `evt_2eqe033bxff6f`, measured at `5a3f3dad0`

- **Defect.** The planner seeds every lexical walk in ABI order and never
  applies the owner's source-body binding order.
  - `continuations.rs::continuation_emission_seat_environment` (`:4153`)
    seeds `walk_continuation_value_environment` with
    `continuation_owner_entry_sources`, which is ABI-ascending by
    construction (the `:3094` sort).
  - The continuation's own input environment is built from the same seed
    (`:3895-3907`); its inputs are the walked environment's prefix, so C_k is
    `reached[k]`.
  - The emitter reverses the parameter run for `CallableDeclaration` and
    `ClosureBody` (`units.rs::source_body_binding_order`, `:7748`;
    `generated_context_source_environment`, `:4251`). The planner reads
    neither.
  - Forward and reversed coincide at length one (`continuations.rs:5281`
    names this for IH prefixes), so the defect needs a converting owner with
    two or more entry parameters.
- **Measured instance** (`evt_4dbywywkr59a5`, px7l delayed row, owner
  `Predeclared(3)` body 359): the planner seat is `[ProducerLocal(358),
  EntryAbi(P0), EntryAbi(P1)]`; the emitter's arm environment is `[Bool,
  v11, v10]`, where index 1 loads ABI offset 8 (ordinal 1, `_caps`) and index
  2 loads offset 0 (ordinal 0, `_input`).
- **Prediction, not yet measured: positions are right and labels are wrong.**
  Source de Bruijn indices are compiled against the emitter's reversed
  environment. Consumers keyed on index, including the direct-emission route
  through `nearest_exact_alias`, get the right value, so sibling 343's
  `v11, v10` is probably correct. Consumers keyed on the coordinate get the
  mirror parameter: for example the capture view's
  `predeclared_entry_frame_slot(.., input.coordinate)` (`:4340`), which loads
  the ABI slot the coordinate names, and every carrier, ownership,
  storage-owner and referent-affinity record copied from that slot. The last
  is lifetime metadata, so a mislabel there is a soundness concern.

Treat anchors as perishable. If a fixed input is false on the landed base,
stop and report the mismatch; do not build around it.

## Scope

`ken-runtime` planner (`cranelift_backend/planning/static_transition/`) and a
witness fixture under `crates/ken-cli/tests/`. Consumers of the **ABI run**
(the `:3094` exactness check, the entry-frame declared slot as a slot) keep
ABI order; only the lexical seed changes. `StaticContinuationFusion` keeps
its refusal. Out of scope: the pending-call package, its D1 and D2.

## Deliverable

Seed the planner's lexical walk in the owner's source-body binding order,
produced by **the same function the emitter uses**, not a restatement.
`generated_context_source_environment` is already generic over `T` and
handles the raw-capture and context-capture suffix.

## Acceptance

- **AC-0 (D0, measure before repairing; post to the WP thread).** Use a
  predicate, not a list.
  - (i) Take every call of `walk_continuation_value_environment` whose seed
    derives from `continuation_owner_entry_sources` (at base: `:3583`,
    `:3902`, `:4173`, `:8562`), and every consumer that reads an `EntryAbi`
    coordinate or its slot contract off the result. Classify each consumer
    as index-keyed or coordinate-keyed.
  - (ii) For each coordinate-keyed consumer, find which landed rows reach it
    with a converting owner of two or more parameters, and whether any of
    them reads the affected value. A read on a landed green row is a live
    wrong value; no read means latent.
  - (iii) Confirm or refute the prediction that 343's direct-emission values
    are correct.
  Proceed to the repair without review unless (iii) refutes it.
- **AC-1 (witness, two-sided).** A fixture in which a continuation of a
  converting owner with two entry parameters (for example after an effectful
  bind in `proc main (input) (caps)`) reads both parameters, with
  distinguishable values of the **same** type and, separately, of
  **different** types. No pending selection is involved. Reach it both
  through direct emission and through a context-capture (entry-frame) route.
  It is red on base in every coordinate-keyed route and green after the fix.
  A single-parameter fixture is not a witness.
  - A coordinate-keyed consumer that no checked-source row reaches, in a
    published trace-selected population with a live positive sensor, is
    named as **unreached in that population** (Architect `evt_6nq0hrfx576fs`).
    That is not unreachable: a shape outside the population is not excluded.
    It is covered by a planner-level control instead, and its source witness
    stays owed (see After landing).
    - **At base `50966beb2`:** the context-capture gather (`calls.rs`
      `call_declared_context`) is unreached. It is reachable only from five
      `Some(coordinates)` arms: `source.rs:4991`, `:5085`, and
      `core.rs:6097`, `:16204`, `:16288`.
    - **Population:** ten targeted rows across `rt_planner_seed_binding_order`,
      px8ta, px7p, px7l and px7m. One resolves a context, px8ta public, and it
      takes the constructed frame. There are zero counted gather reads.
    - **Positive sensor:** synthetic px8tr, at `source.rs:4991` to the gather.

    For a two-parameter converting owner, the control asserts that the
    `EntryAbi` label at each seat index names the ABI slot the emitter loads
    at that index. It compares against the emitter's load offset, not
    against another planner record.
- **AC-2 (controls).** Direct-emission operands come out byte-identical
  before and after the fix, asserted by the node. Reverting only the seed
  change reddens AC-1. Targeted builds only, through `scripts/ken-cargo`.
  No-regression means green in CI.
  - Byte-identity is asserted over the AC-0(i) direct-emission population.
    Any landed row whose planner accept/refuse outcome changes is listed
    with the record that changed it. A row that newly accepts is a STOP for
    review, because a looser lifetime label is the soundness direction.

## Stop conditions

- AC-0(iii) finds 343's direct-emission values wrong: STOP for a ruling,
  because the byte-identical control assumes they are right.
- The fix needs a second ordering function, or changes an ABI-run consumer.
- A row newly accepts (AC-2).
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40`,
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY` or the child-2 checkpoint.

## After landing

`RT-SELECTED-PENDING-CALL-BUILD` rebases onto this and re-runs AC-0(e) as
amended. The Architect expects e1 = e2 = e3 (C1 = `_caps`, C2 = `_input`),
after which its AC-1 proceeds without review. C1 and C2's backing classes
swap labels (C1 the invocation-arena handle, C2 the ingress-borrowed
pointee); both are admitted classes, so no D1 amendment is needed.

`RT-SELECTED-PENDING-CALL-BUILD` AC-1 owes the checked-source witness for the
context-capture gather on a two-parameter converting owner. It reddens under a
seed-only rollback of this node's fix. Until then the gather is covered only by
this node's planner-level control.

## Symptom inventory

Append one line per hard stop; never rewrite history.

1. E2 unreachability was read off a refusal on the `recursive_unit_body =
   None` arm, while five `Some(coordinates)` producer arms bypass it -- keyed
   on one guard's arm, not on the producer set (Architect
   `evt_tfcq2stzw5y9`).
