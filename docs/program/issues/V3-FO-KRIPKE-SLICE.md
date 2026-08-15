---
id: V3-FO-KRIPKE-SLICE
title: "Build the first route-(a) vertical slice of the FO Kripke embedding, up to the theorem boundary the spec reserves: quotation, embed, Cert, check_cert, and both controls"
status: ready
owner: language
size: L
gate: none
depends_on: [V3-KRIPKE-THEORY-CLOSURE, CONF-PROVER-SEED-KRIPKE-DRIFT]
blocks: []
github: null
origin: "Steward, 2026-08-15. V3-KRIPKE-DECOMPOSITION merged its plan with blocks: [] and no successor filed, so the implementation node has been unwritten since. The theory (V3-KRIPKE-THEORY-CLOSURE) and the conformance seed (CONF-PROVER-SEED-KRIPKE-DRIFT) are both merged, so nothing blocks a start. Every fixed input below was read from spec/20-verification/23-prover.md and crates/ at origin/main 7626e32ce by the Steward. Steward-filed per COORDINATION section 2."
---

## The gap, stated as a fact about the tree

`attempt_fo` (`crates/ken-elaborator/src/prover.rs:524`) is three lines and one
of them is a comment:

```rust
fn attempt_fo(env: &mut GlobalEnv, ctx: &Context, phi: &Term, phi_closed: &Term) -> Verdict {
    // The FO propositional structure can be handled by the IPC tactic for the
    // connective skeleton. The Kripke embedding for quantified FO goals is
    // [placeholder — reifies in V4].
    attempt_ipc(env, ctx, phi, phi_closed)
}
```

**Fragment FO has no Kripke route.** Every Kripke node in the tracker is
`merged`; `V3-KRIPKE-DECOMPOSITION` merged its plan with `blocks: []`. The
theory landed and nothing was built on it.

## The frame is largely the spec's, and that is deliberate

**`23-prover.md §4.5` already specifies this slice.** Do not re-derive it and do
not improve it — the enclave chose these exact bounds, and a wider slice is a
different node.

| the slice, per `§4.5` | |
|---|---|
| object sorts | **one** rigid sort `A` |
| predicates | **one** unary uninterpreted `P : A -> Omega` |
| source forms | `Bottom`, atom, `or`, `imp`, `forall` |
| retained in full | the `World` preorder, possibly-empty `Dom_A` with growth, the `Force_P` domain and persistence axioms |
| emitted target | `bottom`, relation, `and`, `or`, `imp`, `forall` |
| certificate rules needed by the positive | exactly `init`, `imp-right`, `forall-right` |
| `Cert` in the slice theorem | restricted to that constructor subset |

The embedding is fixed at `§4:335`:

```
embed(Sigma, f) := K(Sigma) => forall w : World. w |= f
```

**`K(Sigma)` is INSIDE the target formula** (`:338`), so no frame or forcing
premise exists outside it. That is the merge's largest soundness improvement and
the conformance seed was corrected today to stop denying it — see
[[CONF-PROVER-SEED-KRIPKE-DRIFT]].

## The two controls, and neither is optional

**Positive, end-to-end:** `forall x : A. P x => P x` — the closed intuitionistic
identity.

**Negative, classical-only:** `forall x : A. P x or not (P x)` — **must not**
obtain an accepted certificate or a `proved` verdict merely because the backend
reasons classically.

> **`§4.5` forbids the natural decomposition, in its own words: *"A
> translation-only, checker-only, or solver-only increment is not this slice."***
> Partial increments may still land — `COORDINATION` merge policy is unchanged —
> but **none of them closes this node.** Closure is quotation accepting both
> controls, the positive certificate computing to `True`, and the negative
> remaining honestly not proved.

## THE BOUNDARY THIS NODE STOPS AT, AND IT IS THE SPEC'S, NOT MINE

**`§4.4` reserves the placement decision and gates the `proved` verdict on it:**

> The concrete home of `IForm`, `Form`, `Cert`, `check_cert`,
> `embedding_adequacy`, and `checker_soundness`, and the resulting evaluator/TCB
> boundary, remain an Architect and operator placement decision. **Until both
> theorems are kernel-checked in an approved home, route FO cannot return
> `proved`.** No new kernel primitive or trusted axiom is authorized here.

⇒ **`§4.5`'s full closure condition — *"yields a kernel-checked Ken term through
the two stated theorems"* — cannot be met until that decision is made.** It is
not made, and **no node exists for it.** That is the Steward's framing debt and
it is being routed separately; it is recorded here so this ring does not
discover it at `D5`.

> **This does NOT block the node, and do not read it as a reason to wait.**
> Everything below `D4` is buildable now and needs no home:
> the data definitions, `quote_fo`, `embed`, `check_cert` as a computable
> function, the positive certificate computing to `True`, and **the entire
> negative control** — which is about *failing* to obtain a certificate and
> therefore invokes no theorem at all.
>
> **What is gated is exactly one thing: returning `Proved` from `attempt_fo`.**

## Deliverables

**`D0` — the quoted data.** `IForm`, `Form`, `Cert` for the slice's constructor
subset, and `Carriers`/`AtomEnv`/`denote` for the one-sort one-predicate
signature, per `§4.4`.

**`D1` — `quote_fo`.** Accepts the slice fragment and **refuses everything
outside it**, per `§4.1`'s total-quotation requirement. Both controls quote.

**`D2` — `embed`.** `embed(Sigma, f) := K(Sigma) => forall w : World. w |= f`,
with `K(Sigma)` emitted inside the target. The `World` preorder, possibly-empty
`Dom_A` with growth, and `Force_P` domain and persistence axioms are all
present, not stubbed.

**`D3` — `check_cert`, computable.** Over the `init` / `imp-right` /
`forall-right` subset. The positive control's certificate **computes to `True`**;
demonstrate the computation, not the type.

**`D4` — both controls, wired.** The positive route runs end-to-end to the
theorem boundary and stops there with an honest verdict. **The negative control
returns not-proved and is a complete, closed deliverable** — it does not wait on
anything.

**`D5` — the honest verdict at the boundary.** `attempt_fo` returns `Unknown`
(never `Proved`) for a goal whose certificate is accepted but whose theorems are
not kernel-checked in an approved home. **Write the reason at the site**, so the
next reader does not mistake a reserved decision for an unimplemented branch.

## Acceptance criteria

**`AC-1`.** The negative control does not obtain an accepted certificate or a
`proved` verdict. **Demonstrate it by running it**, not by arguing the calculus
cannot derive it.

**`AC-2`.** The positive control's certificate computes to `True` under `D3`'s
`check_cert`, shown as a computation.

**`AC-3`.** `quote_fo` refuses at least one form outside the slice fragment, and
the refusal is by construction rather than by a fallthrough that happens to fail.

**`AC-4`.** `K(Sigma)` is inside `embed`'s target formula. **No frame or forcing
premise is emitted as an external oracle assumption** — that is the exact drift
the conformance seed was corrected for today.

**`AC-5`.** No new kernel primitive and no trusted axiom, per `§4.4`. `proved` is
not returned for FO under any path in this node.

**`AC-6`.** No placement, artifact-home, evaluator-posture, or trusted-base
decision is made here. **If the work seems to require one, that is the handback**
— it is an Architect and operator call and this ring may not take it.

**`AC-7`.** No-regression, in CI (`COORDINATION §12`).

## Banned scope

- **Deciding the theorem home.** `AC-6`. Hand back instead.
- **Widening the slice.** More sorts, more predicates, more connectives, or more
  `Cert` constructors are a later node. `§4.5` says the full `§4.3` theorem
  "remains owed for the remaining constructors and is not implied by this slice."
- **Route (b).** `§4.4` closes with it being neither specified nor changed here.
- **Fragment D's open-goal search.** `§3.2`, a different route.

## What this node does not settle

**It does not make route FO return `proved`.** It builds everything up to the
reserved boundary and stops honestly. Whether the remaining step is one node or
several depends on the placement decision, which is not this ring's.

**The slice is a first slice, not the contract.** `§4.3`'s theorem for the full
constructor set stays owed, and `§4.5` says so explicitly.

## Provenance

`spec/20-verification/23-prover.md` `§4.1`-`§4.5` as merged by
[[V3-KRIPKE-THEORY-CLOSURE]]; the seed reconciliation in
[[CONF-PROVER-SEED-KRIPKE-DRIFT]]; `OQ-12` (`spec/90-open-decisions.md:151`,
**DECIDED** — route (a) on intrinsic merits). `attempt_fo` read at
`prover.rs:524`. All read from the tree at `7626e32ce` by the Steward.
