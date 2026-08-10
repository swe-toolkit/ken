---
id: LANG-NESTED-MATCH-LIFT-ALIGNMENT
title: "the generated-All aligned check path is lost when the lifted match is nested under an outer contribution, so a residual-Bag fold cannot type-check"
status: closed
owner: language
size: M
gate: none
depends_on: []
blocks: []
github: null
origin: Measured by the Kernel ring on KERNEL-NESTED-IND D8, 2026-08-10, at base 5756ff74. D8 was framed with two outcomes and reached outcome (b) -- an attribution rather than a repair. Kernel stopped at its lane boundary and returned this to the Steward, per the frame. Steward-filed (agents cannot create tracked work per COORDINATION section 2).
---

> # WITHDRAWN — THE ATTRIBUTION BELOW IS FALSE. DO NOT IMPLEMENT AGAINST IT.
>
> **There is no aligned-check propagation defect.** Measured by Language against
> exact `02070073` on the recovered D8 source: the outer-`Suc` selector form
> **elaborates and completes final kernel checking**. `scripts/ken-cargo test -p
> ken-elaborator --test nc14_data_match_lowering
> nested_inductive_elaboration_preserves_trusted_base_set -- --exact` is 1/0 and
> the trusted-base comparator holds (`evt_4zdhnrdpf0dw`).
>
> **The refusal was a fixture-migration artifact.** D8 changed `Join : a -> a ->
> Bag a` to `Join : Bag a -> Bag a -> Bag a` and did not migrate the reaching
> constant, which went on supplying bare `LiftRose` operands where `Bag LiftRose`
> was then required. That is the whole of `TypeMismatch { expected: (Dg570
> Dg582), found: Dg582 }` — `Dg570` is `Bag` and `Dg582` is `LiftRose`. Migrating
> the operands to `LiftNode (Join LiftRose (One LiftRose LiftLeaf) (One LiftRose
> LiftLeaf))` clears it. `One : a -> Bag a` is unchanged and that asymmetry is
> load-bearing.
>
> **Two signals said so before the measurement did**, and both are worth keeping:
> the Architect read the landed control flow and found it contradicted the bypass
> this node asserts (`evt_3xyc3cbkk39k9`), and the explicitly-typed `let`
> variant failed *identically* — which a defect in expected-type propagation
> cannot explain, because the broken constant is downstream of how the node
> method is spelled.
>
> The D8 probe was restored without ever being committed, so the failing source
> existed only in the Kernel seat's scrollback. **An attribution built on a
> source nobody can re-read is an attribution nobody can check.**
>
> Everything below is the original filing, kept for the record.

## What it is

`structural result of x` type-checks when the lifted `match` is the **direct
head** being checked. It does not when that match sits **under an outer
contribution** — which is the shape the conformance corpus actually requires.

Measured refusal, at the final kernel re-check, before erasure and before
interpretation:

```
KernelRejected(TypeMismatch { expected: (Dg570 Dg582), found: Dg582 })
```

on

```
Join xs ys |-> liftAdd (structural result of xs) (structural result of ys)
```

when the enclosing `node` method is `Suc (match b { ... })` (span 467..568). An
explicitly typed `let folded : Nat = match b { ... } in Suc folded` fails
identically (span 494..595), so this is not a syntactic artifact of the `Suc`
application.

## Attribution

`crates/ken-elaborator/src/elab.rs`.

Direct `RMatch` checking dispatches through `check_match_dependent` (808-850)
to `check_match_with_lift` (1576-1595). When the required outer contribution
**wraps** that match, `check` falls through to its generic inference arm
(863-865) — `infer` then `unify_types` — and inference routes the nested match
to ordinary `infer_match` (3009-3014). **The generated-All aligned check path is
simply not reached.**

This is **not** Runtime-owned. Kernel confirmed the direct-head selector control
stays green (1/1) while the recursive-`Bag` with outer-`Suc` full-pipeline probe
is 0/1 at this refusal.

## Why it matters

`conformance/kernel/inductive/seed-nested.md` row
`kernel/inductive/nested-size-uses-lift` specifies that the `node` method *folds
the supplied `All^Type_{Bag,0} (λ_. Nat) b` inhabitant and adds `1`*. **Adding
one is an outer contribution**, so the row's own required shape is exactly the
failing case. The row cannot be ungated until this is repaired, and
`KERNEL-NESTED-IND` `D8` therefore produced no candidate.

The landed selector is not wrong — it is reachable and correct at the direct
head, and its identity and wildcard controls hold. What is missing is that the
aligned checking path does not survive being nested.

## Scope note for whoever frames the fix

The mechanism question is **how the aligned check path should propagate through
an enclosing expression**, and it is a genuine design fork rather than a
mechanical patch. Route it to the Architect from inside the work; do not
pre-empt it here.

Two things this node does **not** authorize: widening `check`'s generic
inference arm into a second topology rule, and reconstructing a nested lift
anywhere outside the kernel `method_type`/`recursive_shapes` telescope. Both are
standing prohibitions on this mechanism.

`crates/ken-runtime` is out of scope. `nested-dependent-motive-uses-lift`
remains gated on its own separate, unmeasured blocker and is not this node's
work.
