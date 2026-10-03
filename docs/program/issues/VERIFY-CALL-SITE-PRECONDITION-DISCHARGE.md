---
id: VERIFY-CALL-SITE-PRECONDITION-DISCHARGE
title: "No caller of a function with requires can be elaborated, because nothing supplies the premise proofs at a call site (spec 22 §2.3). Insert the proofs at each call, recognised from the caller's premises or raised as call-site obligations, and stage a spec'd declaration at its full type so self-calls see it"
status: active
owner: verify
size: M
tier: T1
gate: architect
depends_on: [VERIFY-CONTRACT-LOWERING-MULTI-REQUIRES]
blocks: [LANG-REFINED-PARAM-REQUIRES-DESUGAR]
github: null
origin: "Verify QA evt_25tbd7ab2ytxt (recursive requires repro); Architect ruling evt_4qb4fh6bbnyhg: recursion is not the defect, call-site discharge is missing for every caller. Steward-filed per COORDINATION section 2."
---

# Callers of a function with requires elaborate

## Objective

A call to a function whose declaration has `requires` elaborates. Each
premise proof is either recognised from a premise in scope or raised as a
call-site obligation (spec 22 §2.3). A recursive call goes through the same
mechanism.

## Settled inputs (Architect `evt_4qb4fh6bbnyhg`, measured on `775d721c3`)

**Probes.**
- P1: `fn f (n:Int):Int requires Equal Int n n = n` with `fn g (m:Int):Int
  = f m`. `g` is KernelRejected `TypeMismatch { expected: Int, found: Π
  (Equal Int @0 @0). Int }`.
- P2: P1 with `g` also carrying `requires Equal Int m m`. Same rejection.
- P3: `fn recurse (n:Int):Int requires Equal Int n n = recurse n` gives
  `NotAFunction`.
- P4: the same declaration without `requires` gives `NotTerminating`.
- P6/P7: mutual groups with spec clauses fail closed with `Internal`. They
  stay out of scope.

**Code.**
- Nothing in `elab.rs` reads a callee's premises.
- Premises are elaborator-only `Assumption`s, not kernel binders.
  - The body is elaborated without them, then weakened past them in
    Phase 4.
  - `elab.rs:15204` assumes `full_ty` is the carrier Π-chain.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable (the Architect's shape)

1. **Staging.** Build the declaration's full `Π(Δ). Π(φ̄). B` before
   `stage_placeholders`, using MULTI-REQUIRES's Phase 4 construction. Stage
   and admit against that term, and delete the `:15204` assumption.
2. **Premises as binders.** Elaborate the body in `Δ ⊕ φ̄` (spec 21 §6.3)
   and drop the Phase 4 `weaken`.
   - `Assumption { prop, depth }` stays as the recognition index, with
     `depth` pointing at the premise binder.
   - The `/`/`%` site's `close_goal(&cx.ctx, &cx.assumptions, …)` (`:10097`)
     becomes `close_goal(&cx.ctx, &[], …)`.
   - Ensures goals close over the premises.
3. **Arity by checked identity (check 10).** Add `preconditions:
   HashMap<GlobalId, (|Δ|, |φ̄|)>`. Write it when a spec'd declaration is
   staged or declared. Never infer a premise from an Ω-typed domain: an
   explicit proof parameter stays an argument the caller writes.
4. **Insertion.** At an application whose head is `Const(id)`, where `id`
   has an entry and the spine supplies exactly |Δ| explicit arguments,
   append |φ̄| proofs in clause order. Instantiate each `φᵢ` at the
   arguments and at the earlier proofs.
   - A premise binder in scope that converts to the instance supplies the
     proof.
   - Otherwise the elaborator raises an `ObligationKind::Requires` hole
     closed over `ctx`, applied to the context (the Architect's
     `precondition_proof`).
   - A partial spine or a non-`Const` head gets no insertion. It is refused
     where a plain function type is expected.

## Acceptance

- **AC-1.** P1 elaborates with 1 open Requires obligation in `g`. P2
  elaborates with 0.
- **AC-2.** `fn r (n:Nat):Nat requires Equal Nat n n = match n { Zero ↦
  Zero; Suc k ↦ r k }` admits with 1 open Requires obligation. P3 moves to
  `NotTerminating`.
- **AC-3.** An explicit-proof-parameter fixture (`fn h (n:Int) (p : Equal
  Int n n) : Int`) gets no inserted argument.
- **AC-4.** One backend run (interpreter or eval) of a caller shows that
  the proof arguments are erased.
- **AC-5.** MULTI-REQUIRES's bijection still holds: `T1 \ T0` equals the
  reported obligation holes, Requires included.
- **AC-6 (falsifiers; each must redden).**
  - M1: skip insertion, and P1 reddens with `TypeMismatch`.
  - M2: stage at the carrier, and P3 returns to `NotAFunction`.
  - M3: key on an Ω-typed domain instead of the table, and AC-3 reddens.
  - M4: close the call-site goal without the premise binders, and P2 goes
    from 0 to 1.

## Stop conditions

- A backend cannot erase an Ω argument: stop to the Architect.
- A failed admission does not roll back the call-site holes declared during
  the body. Check this with a fixture whose admission fails after a
  self-call hole is declared.
- A kernel change or a fresh-env `trusted_base()` change.
