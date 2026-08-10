# KERNEL-NESTED-IND D10 — make the residual recursive `Bag` topology interpret

Owner: kernel. Size: M. Node: [[KERNEL-NESTED-IND]] (`active`).
**Released 2026-08-10**, on the hard stop [[KERNEL-NESTED-IND-D9]] returned.

Fixed inputs are the durable repro commit `2c165823b102287101eba5a238b650aaa2b2f3fc`
on `wp/KERNEL-NESTED-IND-D9`, cut directly on `main` `81a78dec` — one path,
`crates/ken-elaborator/tests/nc14_data_match_lowering.rs`, `+98/-26`.
**Re-derive your merge-base; do not reuse that SHA.**

## Why this is a separate deliverable and not a widening of D9

`D9` is binding-only by construction: bind
`conformance/kernel/inductive/nested-size-uses-lift` to executing witnesses and
remove its gate. Its frame says that if the pipeline cannot express the row,
that is a hard stop that comes back to me, and forbids inventing the missing
surface. **Kernel hit exactly that condition and escalated exactly as framed.**
The escalation is correct and the clause did its job; this frame is the ruling.

**The witnesses cannot execute, so `D9`'s deliverable is currently
unachievable.** `D10` makes them execute; `D9` then binds the row to them. That
is the same cut shape as Runtime's `D2e`/`D2f` — build the thing, then bind to
it — and it adds **no tracker node**: both are deliverables of the one `active`
`KERNEL-NESTED-IND` node.

## The defect, as Kernel measured it

The repaired fixture **elaborates, kernel-checks, and erases**. The Nat-3
witness then panics before producing a value:

```
scripts/ken-cargo test -p ken-elaborator --test nc14_data_match_lowering \
  nested_recursive_bag_rose_elaborates_checks_erases_and_interprets_at_nat_three \
  -- --exact
0 passed; 1 failed -- panic at ken-kernel/src/subst.rs:245
```

Causal path, reported by Kernel and confirmed by its leader:

```
subst_outer <- method_type <- elim_reduce
            <- lift_recursive_value(Direct)
            <- lift_recursive_value(Former) <- elim_reduce
```

**What I read in the code, as grounding rather than as a prescription.**
`lift_recursive_value`'s `Former` arm (`crates/ken-interp/src/eval.rs:633` and
following) recurses into each recursive field passing `fam`, `level_args` and
`params` **unchanged** — the outer eliminated family's coordinates. On this
topology the field's value is still a `Bag` constructor while `fam` is the
unparameterized `LiftRose`, so `params` is empty. The `Direct` arm then calls
`elim_reduce` directly; `elim_reduce` resolves the constructor's host as the
parameterized `Bag` and calls `method_type` with one parameter missing, and
`subst_outer` indexes `params[p_idx]` with no bounds check.

**The shape of the omission is worth naming: every other mismatch in that
function is guarded to `EvalVal::Neutral`** — a wrong host, a missing support
decl, a missing ordinal, a short field list. The `Direct` arm alone proceeds
with no check that the value's constructor host is `fam`.

## The two properties, and they are not the same property

**P1 — robustness. A checked, erased artifact must not panic the interpreter.**
The artifact passed kernel checking and erasure; an index-out-of-range in
`subst_outer` on well-typed input is a defect independent of this row.

**P2 — capability. This topology must actually compute `Nat 3`.**

⇒ **A repair that converts the panic into `EvalVal::Neutral` satisfies P1 and
not P2, and therefore does not discharge this deliverable or unblock `D9`.**
Say which of the two any intermediate result reaches. If P2 turns out to need
genuine new lifting capability rather than a corrected recursion, **that is a
hard stop and it comes back to me** — do not grow the surface to reach it.

## The mechanism is yours and the Architect's, not mine

I am naming the seam, not the repair. At least three layers could carry it —
the `Former` arm carrying the nested host's coordinates; `elim_reduce`
declining when the constructor host and `fam` disagree; a bounds-check in
`subst_outer`. **I am not choosing, and I do not want the cheapest one chosen
because it is nearest to the panic.** Repair at the layer where the wrong
information is *introduced*, and say why that layer is the right one.

**`crates/ken-kernel` is TCB.** `subst_outer` and `method_type` are kernel
surface: a change to either one's **contract** is a stop condition, not a
judgment call — see below.

## Deliverables

**1. The topology interprets.** The residual recursive `Bag`/`Rose` witness
runs the full pipeline through interpretation and produces `Nat 3`.

**2. The guard the `Direct` arm lacks.** Whatever layer you choose, a
constructor host that disagrees with the eliminated family must not reach a
raw index. State what it does instead and why that is the correct behaviour
rather than the convenient one.

**3. The fixture lands here, not in `D9`.** Carry `2c165823` into this branch
so the migration and the repair land together and CI is green on the result.
`D9` must not also land it — I have said so in its frame.

## Acceptance criteria

**AC-1.** The row's Nat-3 witness —
`nested_recursive_bag_rose_elaborates_checks_erases_and_interprets_at_nat_three`
— passes and produces `Nat 3`. Not "does not panic": the value.

**AC-2 — the panic is unreachable by construction, with a positive control.**
A shape that would previously have reached the raw index is exercised and is
refused or handled. **A check that passes because nothing reached it is the
failure mode here**, so state what would have caught the negative case.

**AC-3 — P1 and P2 are reported separately.** Say plainly which the candidate
discharges. If the panic is gone and the value is not `Nat 3`, that is a
partial and it is a good outcome — hand back the seam rather than forcing it.

**AC-4 — the fix is at the layer where the wrong coordinates originate.**
Name the layer and say why. If the repair is a bounds check in `subst_outer`,
justify why the caller passing empty `params` for a parameterized host is
correct behaviour that the kernel should tolerate — that is a real position,
but it must be argued, not defaulted into.

**AC-5 — `One : a -> Bag a` stays unchanged.** Only `Join` is recursive. That
asymmetry is load-bearing and is what the row's topology specifies.

**AC-6 — carried from `D9` unchanged.** Any fixture edit changing a
constructor's type migrates every reaching term **in the same commit**, and no
reported measurement may rest on a source that exists only in a restored
working tree.

## Excluded scope

- **The conformance row binding is `D9`'s and stays there.** No
  `conformance/` path in this candidate, no gate-marker removal, no
  `blocked on` status edit, no `AC-4` fold mutation. **So no Spec vote on this
  merge Decision** — check that before you open it.
- **`nested-dependent-motive-uses-lift` stays gated and untouched.**
- **No `crates/ken-runtime`.** Runtime owns it and is mid-flight on `D2e`.
- **No new capability to reach P2** if a corrected recursion will not.

## Stop conditions — return to me, do not decide

- The repair requires changing the **contract** of kernel `method_type`,
  `subst_outer`, or `elim_reduce` rather than their defensive behaviour.
- The repair **grows the TCB**. That is the operator's call and I forward it.
- P2 needs genuine new lifting capability rather than corrected coordinates.

## Contention

Paths are `crates/ken-interp`, possibly `crates/ken-kernel`, and the
`crates/ken-elaborator` test fixture. Runtime is on
`crates/ken-runtime/src/cranelift_backend`; Language is idle and its next node
is `crates/ken-elaborator/src`, not its tests. The intersection is empty, but
check before you write.

## Validation

Targeted only, **never `--workspace`**. **An `eval` change makes the floor a
full `-p ken-interp` run** — a suite-scoped run does not cover the reifier. Add
a full `-p ken-kernel` run if you touch the kernel, and the named
`-p ken-elaborator --test nc14_data_match_lowering` witness either way. "No
regression" means green in CI.

## Sizing

One turn to a releasable increment or a genuine hard stop. Both are good
outcomes, and this deliverable exists because the last hard stop was called
correctly.
