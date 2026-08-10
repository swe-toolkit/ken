# KERNEL-NESTED-IND D8 — ungate `nested-size-uses-lift`

Owner: kernel. Size: S. Node: [[KERNEL-NESTED-IND]] (`active`).
Fixed inputs measured at `main` `2c0f4c03`.

## The constraint, and where it is grounded

`conformance/kernel/inductive/seed-nested.md` carries the row
`kernel/inductive/nested-size-uses-lift`, marked `[KERNEL-NESTED-IND]` and
stating *"blocked on `KERNEL-RECURSIVE-RESULT-SURFACE`. The current
implementation cannot yet elaborate or execute the specified selector."*

**That blocker is discharged.** `KERNEL-RECURSIVE-RESULT-SURFACE` delivered the
spec and conformance contract only; its implementation successor
`LANG-STRUCTURAL-RESULT-ELAB` merged at `2c0f4c03` (exact `41c28de7`). The
gating sentence therefore describes a state that no longer holds, and the row
is a landed conformance obligation guarding nothing.

This is grounded in the conformance corpus itself, not in frame prose. The row
also writes its own acceptance, which is why this frame is short: it names the
two required executing witnesses, forbids three degenerate shapes, and states
the discriminating value flip.

## What is already measured, so nobody re-derives it

| stage | status at `2c0f4c03` | evidence |
|---|---|---|
| surface elaboration | available | `lang_structural_result_elab.rs::validated_nested_results_elaborate_and_kernel_check` elaborates `STRUCTURAL_SIZE_SOURCE`, whose `Bag.join xs ys` arm uses `structural result of xs`/`ys` |
| kernel checking | available | same test; the completed eliminator is kernel-checked |
| erasure | landed | `82918b6a` — checked-artifact erasure admits the generated support `Elim` gated on `all_support_origins` |
| interpretation | landed | `82918b6a` — the interpreter evaluates the `LiftRose`/`Bag` Nat-3 case |

**The existing elaboration witness is not the row's witness.** It exercises no
erasure and no interpretation, so it cannot discharge a `reduces-to` row. The
four stages are individually present; whether they compose end to end is the
thing this deliverable measures.

## Deliverable

Measure whether `size (node (join (one leaf) (one (node empty))))` reduces to
`3` through the full pipeline — surface elaboration, kernel checking, erasure,
interpretation — at `2c0f4c03`.

**Two outcomes, both acceptable, and the second is not a failure.**

**(a) It reduces to `3`.** Bind the row to executing witnesses named by the row
itself and remove the gate marker and the `blocked on` status line.

**(b) Some stage refuses.** Deliver the attribution: which stage, the exact
refusal, and the `file:line`. **Do not repair it.** A refusal in
`crates/ken-runtime` is a Runtime attribution and Kernel stops. A refusal
elsewhere comes back to me for a fresh frame.

## Acceptance criteria

**AC-1.** The row's two named executing witnesses exist and pass:
`nested_recursive_bag_rose_elaborates_checks_erases_and_interprets_at_nat_three`
and `nested_recursive_bag_join_residual_folds_all_leaves_at_nat_three`. Both
must drive the selector through the **full** pipeline — the first for the Nat-3
`Bag`/`Rose` computation, the second for a deeper residual `Bag.join` topology.

**AC-2.** The row's three named degenerate shapes are excluded by construction,
not by intent: a finite unroll, a depth-three snapshot that never consumes a
residual recursive result, and a header naming only one side of the `join` each
fail to bind the row. State for each which property of the witness rules it out.

**AC-3.** The kernel structured-IH/iota witness
`production_nested_lift_is_consumed_and_iota_computes` is named in the binding,
per the row's executing-binding requirement.

**AC-4 — the discriminating control.** The row states the value flip and it is
the whole point of the case: with the correct lifted IH the result is `3`; an
implementation that supplies a lift but drops or ignores its leaves computes
`1`; a guard-deletion-only implementation cannot type-check the definition at
all. **A witness that only asserts `3` does not discriminate.** Show that the
`1` outcome is reachable under a mutation of the fold and that the witness reds
on it. One mutation, reported with its result.

**AC-5.** The gate marker and the `blocked on KERNEL-RECURSIVE-RESULT-SURFACE`
status are removed from this row only, and the file's summary paragraph is
updated so it no longer claims this row is gated. **Line 21's summary and the
line-541 residual list both say so and are separate sites.**

## Excluded scope

- **`nested-dependent-motive-uses-lift` is OUT.** Kernel measured it as
  unestablished: the test file contains no `Omega`, `Proof`, `motive`, or
  dependent-motive residual-Bag witness, so no source witness exists for that
  row. It keeps its gate. Do not update its status line on the strength of this
  row's outcome — the rows share a blocker sentence, not a witness.
- **`crates/ken-runtime` planner and lowering stay OUT**, unchanged from the
  standing ruling on this node. `AC-K12`'s native stage remains Runtime-owned.
- **No new capability.** If the pipeline cannot express the row, that is
  outcome (b). Do not invent the missing surface.
- `wp/KERNEL-NESTED-IND-D5` and `wp/KERNEL-NESTED-IND-D6` are squash-merge
  leftovers, verified by blob. Do not reopen either.

## Contention

None. The paths are `conformance/kernel/inductive/seed-nested.md` and
`crates/ken-kernel` / `crates/ken-elaborator` test targets. Runtime is on
`RT-LEXICAL-RECURSOR-CONSUMERS` in `crates/ken-runtime`; Language is on a
bounded read in `crates/ken-elaborator/src`. A `conformance/` path pulls a Spec
vote on the merge Decision.

## Validation

Targeted only. `-p <crate>` or `--test <name>`, never `--workspace`. This
deliverable touches a public surface only if outcome (b) forces it, so the
public-enum rule does not bind by default — but if any enum variant is added or
changed, the floor is a full `-p <crate>` test build, not one suite.

"No regression" means green in CI.
