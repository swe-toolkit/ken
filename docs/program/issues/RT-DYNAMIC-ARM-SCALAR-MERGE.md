---
id: RT-DYNAMIC-ARM-SCALAR-MERGE
title: "A carried Match arm carrying a nested-IH result cannot satisfy merge_scalar_operand -- measure what the arm actually produces before bounding the repair"
status: ready
owner: runtime
size: TBD
gate: none
depends_on: []
blocks: []
github: null
origin: Measured by KERNEL-NESTED-IND D5 at WIP 51c482a5 (evt_3evnpax25tckf, 2026-08-09). Kernel reached the native boundary after interpreter Nat-3 and provenance-gated erasure both passed, and stopped without Runtime edits exactly as the durable D5 ruling at main 46c12adb requires. Steward-filed (agents cannot create tracked work per COORDINATION §2). Steward owns the frame and AC/control placement.
---

> # `D0` IS STARTABLE. THE REPAIR IS NOT YET BOUNDED, AND THAT IS DELIBERATE.
>
> Nothing measured says what the refused arm actually produces, so nothing
> bounds the fix. `size: TBD` is honest rather than lazy: a guessed size on
> this campaign has been wrong every time it was guessed. **`D0` closes that
> gap and is a full slice on its own.** Do not size `D1` before `D0` reports.

Treat every anchor below as perishable. If a fixed input turns out false
against the landed code, say so and escalate — do not quietly build around it.

## What it is

`KERNEL-NESTED-IND` `D5` made nested-inductive elimination work through the
elaborator, the interpreter, and checked-artifact erasure. It then reached
native lowering and refused:

```text
NativeLoweringOrExecution: a carried Match arm
  -- dynamic arms must produce scalar Int or Bool values
```

**This is a Runtime-owned capability gap, not a Kernel defect.** Kernel may not
edit `crates/ken-runtime`; the planner/lowering invariant is Runtime's, and a
Steward authorization to the contrary was overruled once already.

## Fixed inputs, measured at `main` `46c12adb`

| fact | value |
|---|---|
| refusal site | `crates/ken-runtime/src/cranelift_backend/lowering/mod.rs:15898-15900`, the `_ =>` arm |
| enclosing function | `merge_scalar_operand`, `:15749` |
| `ScalarMergeKind` | `:14138` — `Int`, `Bool`, `StructuralNat`, `ExitCode`, `RecursiveBackedge` |
| producing WIP | `51c482a5` on `wp/KERNEL-NESTED-IND-D5`, six paths, `crates/ken-runtime` diff **empty** |

What `merge_scalar_operand` **admits** today, read from the arms above the
refusal:

- `Lowered::StructuralNat`;
- `Lowered::Constructor` with **empty** args that is `bool_true` or
  `bool_false`;
- `Lowered::ProcessExitStatus`;
- any `lowered` when `checked_root_exit_representation` holds, via
  `emit_process_exit_status`.

Everything else refuses. **The admitted set is the measurement that matters**
— the gap is the difference between it and what a nested-IH arm yields.

## `D0` — measure the produced value. This is the whole first slice.

**Do not repair anything in `D0`.** Report:

1. The exact `Lowered` variant the refused arm carries, at the refusal, for the
   `LiftRose`/`Bag` Nat-3 case that `KERNEL-NESTED-IND` `D5` drives.
2. Whether that variant is **scalar-representable at all** in
   `NativeScalarPairV1`, or whether it is structurally wider than the pair.
   **These are materially different futures and they bound different repairs**
   — one is a missing arm, the other is a representation question.
3. Whether `RecursiveBackedge` is reachable at this seat. It is a declared
   `ScalarMergeKind` variant that **no arm here produces**, and
   `RT-MATCH-RECURSOR-CONSUMERS` records it as protocol-only and untouched.
   Say which it is; do not assume from the enum.
4. Whether any **other** caller of `merge_scalar_operand` already passes a
   value of the same shape and is therefore already refusing in production, or
   whether `D5` is the first arrival.

**`D0` closes when those four are answered with `file:line` evidence.** It does
not authorize a repair and it does not size one.

## Deliverables beyond `D0` — NOT YET FRAMED

`D1` is the repair and it is deliberately unwritten. **Return to the Steward
with `D0`'s measurement and the frame gets cut against it**, not against the
error string.

## Acceptance

| AC | criterion | control |
|---|---|---|
| `AC-1` | `D0` names the exact `Lowered` variant at the refusal | the variant is read **at the seat**, not inferred from the arm's source expression. A characterization taken upstream of `merge_scalar_operand` does not discharge this |
| `AC-2` | The scalar-representability question is answered **in a direction**, not hedged | state whether it fits `NativeScalarPairV1` or exceeds it, and why. "It depends" is not an answer; if it genuinely depends, name the discriminant |
| `AC-3` | The `RecursiveBackedge` reachability claim carries a witness **or** an explicit "not reachable here, and this is how I established that" | a negative check passes for any reason, so an unreached-variant claim needs a positive control showing the instrument would see it if it fired |
| `AC-4` | Any repair (`D1`, when framed) leaves the four currently-admitted shapes **byte-for-behaviour unchanged** | `StructuralNat`, nullary bool, `ProcessExitStatus`, and the checked-root-exit path each keep their existing arm and result |
| `AC-5` | Any repair keeps the `_ =>` **fail-closed** | widening the admitted set must not convert the catch-all into an accept. A value outside the new admitted set still refuses with a diagnostic that names it |

## Forbidden

- **Blanket relaxation of the scalar contract.** Widening `merge_scalar_operand`
  to accept arbitrary `Lowered` values is not the repair, whatever `D0` finds.
  Same reasoning as [[RT-CARRIER-BYTESPAN-OBSERVE]]: availability is per seat,
  never a blanket phase relaxation.
- **Folding this into [[RT-CARRIED-RESOURCE-SCALAR]].** That node's refusal is
  an effect-seat `ResourceScalar`-in-`CarriedWord` shape — a different need on
  different seats. Its own frame warns against exactly this
  same-shape-different-population fold, and it is `draft` with no frame.
- **Folding this into [[RT-TERMINAL-ALL-ELIM-AUTHORITY]].** Different seat:
  that node owns `lowering/core.rs:6178-6183`, the `ComputationalRecursorClosure`
  remainder arm. This is `lowering/mod.rs:15898`. Checked, not assumed.
- Editing `crates/ken-elaborator`, `crates/ken-kernel`, or `crates/ken-interp`
  to make the arm produce something the existing seat already accepts. That
  moves a Runtime gap into Kernel's landed work.

## Sequencing

**Runtime's next slice after the current `RT-MATCH-RECURSOR-CONSUMERS` work.**
Do not interrupt a slice in flight for it. `D0` is measurement and does not
contend with `D8`'s pin.

### No reverse edge, and the direction is deliberate

`KERNEL-NESTED-IND` `AC-K12` requires native execution, the Cranelift verifier,
and interpreter/native agreement, so that node **cannot close** until this one
lands. **That is an acceptance condition of the Kernel node, not a reverse
implementation dependency**, and `blocks:` stays empty here — the same call
[[RT-TERMINAL-ALL-ELIM-AUTHORITY]] records for the identical shape. Kernel's
`D5` work lands as an accepted partial in the meantime; it does not wait on
this node and this node does not wait on it.
