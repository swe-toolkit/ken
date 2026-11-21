---
id: LANG-NATIVE-CLOSURE-DEAD-BINDER-CAPTURE
title: "A native host-computation continuation captures every in-scope runtime value, not its free variables, and each closure record also inlines a full copy of that environment, so every binder a straight-line host chain introduces is carried by every later closure even when no later code reads it, and an N-step chain costs Θ(N²) invocation words. Capture only what a continuation can read, so a chain's per-step arena cost is constant"
status: ready
owner: language
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Architect census attribution evt_68ym86q2hxtss on RT-NATIVE-INVOCATION-ARENA-COST-OBSERVATION (arm C1, classification (ii)); replaces the Θ(N²) carry from Architect evt_3cqsj6ncrym8t. Steward-filed per COORDINATION section 2."
---

# Native continuations capture only live binders

## Objective

In a native host-computation chain, a continuation's captured environment
holds only the runtime values its body can read. A write's result and its
pattern-bound reply values that no later code reads are not carried
forward. A straight-line chain of N writes then costs a constant number of
invocation words per write, not a cost that grows with the write's depth.

## Settled inputs (main `4e6c4e62b`)

- **The capture.** A host_bind continuation's environment E holds every
  in-scope runtime value. `LexicalClosure` captures `(0..runtime_depth)`
  (`crates/ken-elaborator/src/erasure.rs:3240`-`:3243`, `:3630`,
  `:6112`), not the body's free variables.
- **The measurement.** In the F1 to F4 census
  (`/workspaces/ken/tmp/rt-arenacost-census/`, Architect
  `evt_68ym86q2hxtss`), each chain level prepends exactly 3 fields to
  each later E: one InvocationHostResult and two PersistentGround values.
  Every closure record R carries E's fields again. That gives the second
  difference of 6 words per level.
- **The liveness.** In the F4 fixture, both payloads of
  `Ok (WriteProgress.Wrote _)` are wildcards, and each `written` is read
  only by the next proc's match. Once the next level begins, all three
  fields are dead. This is read from source, not measured.
- **The second width source.** Each R holds a reference to its E, a full
  inline copy of E's fields and a repeated 20-field tail, which is
  E + 22 at every F4 level. No layer has been attributed yet. The
  candidates are erasure, the planner record, or lowering. Under
  `crates/ken-runtime/src/cranelift_backend/`, those are
  `planning/static_transition/aggregates.rs:4556`-`:4588` and
  `lowering/aggregates.rs:4272`.
- **The pinned consumer.** RT-NATIVE-INVOCATION-ARENA-COST-OBSERVATION
  pins D1 and states this defect as its GAP. A fix lowers D1 and re-rules
  that literal; it is not a write regression.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

The runtime context a native host continuation captures, and R's inline
copy if AC-0 attributes it to the same change. No change to the
interpreter, the kernel, the effect ABI or any availability label.

## Deliverable

One Language-ring candidate, with Runtime review of the lowered plan if
AC-0 places R's layout in Runtime's code.

## Acceptance

- **AC-0 (D0, Architect, before code).**
  - Attribute R's inline copy to a layer.
  - Rule the fix family: free-variable capture, dropping dead and
    wildcard pattern binders from the runtime context, removing R's
    inline copy, or a combination.
  - Rule the new D1 the ARENA cost row will pin.
- **AC-1.** On the F1 to F4 write chain, the second difference of the
  invocation word count is zero, and D1 equals the D0's literal.
- **AC-2.** A chain whose later level does read an earlier write's result
  still reads the right value natively, and its output matches a fresh
  interpreter run.
- **AC-3.** Full-workspace CI is green on the exact candidate, with the
  ARENA cost row re-ruled in the same candidate.

## Stop conditions

- The fix needs a kernel, trust or effect-ABI change.
- A continuation that reads an earlier binder loses it, or a parity row
  changes output.

Checks: 1 fired. The counts come from the census and the liveness from
the source. These are independent inputs, and the frame states which one
is measured. 8 fired. AC-2's live-read chain separates dropping dead
binders from dropping every earlier binder. AC-1 alone would pass under
both.
