---
id: RT-NATIVE-SCALAR-JOIN-REFERENT-OWNERS
title: "A source aggregate whose child is a specialized scalar join is planned as owning no referent, although every stored scalar except Bool has a persistent spill arm, so a withResource bracket body that returns a computed ExitCode refuses at native-build with 'child 0 is held ... can be owned by PersistentStore ... (planned Persistent over [NoReferent])'. Key the child's planned owners on the stored scalar's spill class, carried as a planner fact beside the join's representation"
status: ready
owner: language
size: M
tier: T1
gate: architect
depends_on: []
blocks: [ABI-S2]
github: null
origin: "Architect closure ruling evt_29p8rbjhbz7mh on ABI-S2's computed-bracket-exit wall (I-probe evt_3va3hpc27g78q, V2/V4), on the Steward's default (2) evt_46qr4y9vsmn85 after the Y-probe fixture was stopped by the provider filter. The cause is in the planner, not the directory surface. L1b Linux ABI program, Language ring, ahead of ABI-S2's consuming fixture. Steward order."
---

# Scalar join children own their spill

## Objective

A natively built program whose bracket body returns a computed `ExitCode`
or `Int` as a constructor field builds and runs, and agrees with the
interpreter. A `Bool` field stays planned as owning nothing.

## Settled inputs (main `67128ba0f`; read by code, `evt_29p8rbjhbz7mh`)

- **The witnesses.** ABI-S2's V2 and V4 (I-probe `evt_3va3hpc27g78q`)
  refuse at `reconcile_source_aggregate` for occurrence 99, child 0, the
  `ResourceBodyOk Unit ExitCode (listing_exit ...)` arm. V3, with a constant
  exit, builds and runs.
- **The arm.** `aggregate_child_referent_owners`
  (`planning/static_transition/aggregates.rs` ~`:3361`) answers
  `[NoReferent]` for a `NativeScalarPair` child that cannot take the
  process carrier. `NativeScalarPair` is the emission phase
  (`summarize_result_phase`, `joins_traps.rs` ~`:613`), not the stored
  form. A stored `ExitCode` is `ProcessExitStatus`, with spill `Some(Int)`
  (`boundary.rs` ~`:1145`).
- **The only witness so far** of the narrowing is D7
  (`lowering/aggregates.rs` ~`:6644`), a `Bool`.
- **The closure.** The ruling's §§(1) to (3) are the design:
  - `JoinScalarSpill { SpillFree, MaySpill }` on `PlannedJoinResult` and
    `ResultPhaseSummary`, with the stated join, sequence and form rules;
  - `semantic_ir.rs`'s typed `is_bool_constructor`;
  - the replaced arm and doc comment.

  Build it as written. The containment gate in `reconcile_source_aggregate`
  does not change.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

The planner fact, the typed Bool check, the owner arm, and the rows below.
No change to the containment gate, the lane deciders, the boundary format,
ABI-S2's surface, or any fixture rewritten to avoid the computed exit.

## Acceptance

- **AC-1.** The ruling's rows:
  - (a) Int and ExitCode `If` children give owners `{NoReferent,
    PersistentStore}` and reach the allocator;
  - (b) and (b'): D7 is preserved byte-identically, over `Bool` literal and
    `Bool`-constructor arms;
  - (c) the narrowing's two-call counterexample never gains
    `InvocationArena`;
  - (d) the native file-bracket row with a computed exit, with its
    `SourceAggregateChildObservation` and transport-domain report.
- **AC-2.** The ruling's mutations m1 to m4, each restored byte-identically.
  m1 must make (a) and (d) refuse with the containment text.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- N-b and N-d go to the Architect: no single-step form reaches the arm
  under m1, or (a) does not refuse under m1.
- **N-c goes to the Steward.** If (d)'s fixture trips the provider safety
  flag while it is being written, stop. No seat retries it with other
  wording. (a) to (c) and m1 to m4 continue.

Checks: 2 fired. The singleton `[NoReferent]` has one producing arm, and
the ruling enumerates the other two and the residual override. 8 fired.
m1 restores the base answer and must refuse (a) and (d), while m2 collapses
D7's population, so both directions fail on a wrong fix. 3 fired. The
in-flight siblings (RESULT-JOIN-PHASE-r1, METADATA-WIDE-INT,
ORIENT-TRANSLATE) touch nothing under `planning/` or
`lowering/aggregates.rs`.
