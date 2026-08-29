---
id: RT-COMPOSED-RETURN-PRODUCED-TRANSFER
title: "Carry the closed predecessor's FULL route-specific objective: add Direct's ruled declared call whose local return is fresh R2, replace the general composed-return protocol with one exhaustive affine state machine carrying Tail's produced result to the exact Tail consume, and bind the delivered R2 from BOTH variants through each independently derived Ret capture. Atomic over Direct application + Tail produced transfer + both-arm D3B + products."
status: draft
owner: runtime
size: L
gate: none
tier: T1
depends_on: [RT-COMPOSED-RETURN-TRAMPOLINE-EXHAUSTIVE]
blocks: []
github: null
origin: "Architect HS14 component-design ruling evt_7gnw8s9k7rh6, 2026-08-29, answering Steward disposition request evt_6dvw8j96w2sdx. Arm 1 selected: widen the composed return protocol. Arm 2 collapses into arm 1 (an earlier coupling still needs a carrier, and the only candidates are the already-rejected runtime data / capture / control stores, or arm 1). Arm 3 is true only of the predecessor's scope, not of the objective. This node is the Steward's recut of that ruling; the behaviour-preserving trampoline prefix is split out as RT-COMPOSED-RETURN-TRAMPOLINE-EXHAUSTIVE. Steward-owned cut per COORDINATION section 2."
---

> # ATOMIC FROM THE FIRST PRODUCTION MINT. NO PARTIAL LANDING.
>
> Architect, verbatim: *"From the first production mint onward, producer ->
> construction -> composed propagation -> exact Tail consume -> D3B Ret-input
> binding -> read/write `InvalidOffset` controls is one semantic candidate.
> **Nothing may land with a live `Produced` state dropped or merely reported.**"*
>
> **A dropped-or-reported `Produced` is the precise failure this node exists to
> prevent**, so it can never be an acceptable intermediate state. There is no
> application-only checkpoint and no evidence-only partial.
>
> ### THE ATOMIC OBJECT IS FOUR THINGS, NOT TWO (Architect `evt_3vpt967507xw3`)
>
> **Direct application + Tail produced transfer + BOTH-ARM D3B + products.**
>
> **This corrects a Steward omission in the first recut.** That version scoped
> the node to the Tail transfer and D3B alone, and said of Direct only that
> `DirectInvocationReturn` "remains an explicit ordinary/direct arm" — which is
> the ruled PROTOCOL SHAPE, not a deliverable. **No production Direct call ever
> landed: Direct D3A was FROZEN with Tail, never discharged.** So in this tree
> "Direct remains ordinary" **preserves the MISSING Direct application** instead
> of carrying the objective forward. **A protocol-shape constraint is not a
> substitute for a scope item, and a sentence that reads as preservation can
> authorize an omission.**
>
> **`draft` — NOT RELEASED.** A landing discharges a dependency; only an
> explicit Steward release starts a turn. Flip `draft` -> `ready` -> `active` on
> release.

## What this preserves, and what was refuted

[[RT-RESULT-CONTINUATION-BINDING-PROVENANCE]] is **closed as structurally
refuted at HS14** — not abandoned. **The semantic objective is NOT refuted:**
`spec/40-runtime/42-evaluation.md §6.2` still requires perform -> observe ->
single tail resume, and the two admitted products still distinguish the wrong
seed from the fresh `R2`. What was refuted is delivering that edge **while
forbidden to change the general composed-return protocol.** This node's scope
explicitly includes changing it.

**Do not create another predecessor.** The Architect ruled it directly: no HS15
and no further predecessor on this axis. Fourteen stops established the
obstruction; the remaining work is construction, not localization.

## Required protocol shape (Architect-ruled — do NOT redesign it)

**One exhaustive affine state machine, NOT `Option` beside `LoweringOperand`.**

1. **`Ordinary`** owns one ordinary/direct current result.
2. **`Produced`** is minted **only** at Tail's exact
   `call_checked_ih_transport_from_case_environment` return
   (`source.rs:4369-4374`). It owns the returned operand, the exact
   `ContinuationCallIdentity`, and an **opaque Tail-destination certificate**
   narrowed from the already-selected, already-validated governed projection.
   **Retain that projection locally through call dispatch; do NOT perform a
   later map lookup.**
3. At `ConstructArgument`, a `Produced` field is **borrowed** to emit/store the
   constructor field, then the **same Rust owner is moved out** after
   construction — **no operand clone**. The transition becomes
   `ConstructedWithProduced { current: constructed, produced }`. **A field-state
   enum (`Ordinary` vs `Produced`) is preferable to a parallel vector/slot
   because it cannot desynchronize.** A second live `Produced` **fails closed**
   unless a census proves multiplicity and a later design supports it — **do not
   smuggle in a Vec/stack.**
4. Every composed/source return boundary propagates `Ordinary` or
   `ConstructedWithProduced` **exhaustively**. **No helper may call `into_value`
   on a pending state.** At a boundary that genuinely accepts only ordinary
   output, extraction is an exhaustive `Ordinary` match and **any pending state
   is a compiler error.**
5. At the exact later Tail route, match the current destination coordinates
   against the carried opaque destination certificate, consume `Produced` to
   form `Routed`, and have the active jump consume `Routed` **exactly once**.
   The certificate uses the landed Tail route for **retained topology/destination
   facts only — it must NOT resurrect `TailResumedRetInput` as value
   authority.**
6. **`DirectInvocationReturn` remains an explicit separate ordinary/direct arm**
   in the protocol. **That is a shape constraint and NOT the Direct deliverable
   — see D-DIRECT below, which is what actually discharges the obligation.**

## Deliverables

- **D0 — mint-point and boundary census** at the release SHA: the exact producer,
  every composed/source return boundary the state crosses, the complete nominal
  caller closure, and the governed projection retained through dispatch.
  **Compile every named caller, `cfg(test)` included.**
- **D1 — the affine state machine**, points 1-6 above.
- **D2 — the exact Tail consume**, point 5, consumed exactly once.
- **D-DIRECT — add Direct's ruled declared call.** Retain the body-refined
  `CheckedIhEnvironmentTransport`; **validate its source record and capture
  projection**; resolve **only**
  `continuation_calls[transport.source_call_identity()]`; emit **exactly one**
  ruled declared call; **treat its local return as fresh `R2`**; and pair it to
  the exact governed Direct arrival. **This is the
  predecessor's still-live D3A obligation for the Direct variant** — it was
  frozen alongside Tail and never discharged, so it is added here, not
  preserved by leaving Direct "ordinary".
- **D3 — the Ret-input binding** (the surviving D3B): bind the delivered fresh
  `R2` **from BOTH variants** through ordinary Ret-case/capture semantics, with
  the read and write analogues **derived independently** from their own planner
  facts. **Both arms, not Tail alone.**
- **D4 — the read/write `InvalidOffset` products** and the fresh-`R2`/seed
  inequality proof.

**D1-D4 are ONE candidate.** D0 may be reported before the rest.

## Acceptance criteria, each with its control

- **AC-EXHAUSTIVE-PROPAGATION.** Removing any propagation arm is a **compile
  error** or a named fail-closed error. Control: delete an arm, show the exact
  failure, byte-restore.
- **AC-NO-PENDING-ESCAPE.** No helper calls `into_value` on a pending state, and
  a pending state at an ordinary-only boundary is a compiler error. Control:
  suppress only the final consume while **preserving** the mint, and observe a
  **pending-state escape error — NOT `Unavailable`**, and not a silent default.
- **AC-CERTIFICATE-DISCRIMINATES.** Mutate the carried destination certificate
  and the source identity **independently**, and observe refusal **before** the
  active jump. Keep an ordinary/direct positive control. **Vary each
  independently — a stated arm count is both wrong and narrowing**, since it
  turns an open family into a checklist somebody can finish.
- **AC-SUPPRESSION-RETAINS-OLD-PRODUCTS.** Suppress **only** the `Produced`
  propagation while **preserving construction**, and observe both products
  retain the **old wrong seed/default**. **This is the two-sided proof the whole
  chain turns on** — a suppression that reds for any other reason proves
  nothing.
- **AC-DIRECT-APPLICATION.** Exactly ONE ruled declared call is emitted per
  governed Direct arrival, resolved **only** through
  `continuation_calls[transport.source_call_identity()]`, with its source record
  and capture projection validated and its local return treated as fresh `R2`.
  Control: mutate the resolved identity and observe refusal; a Direct arrival
  that silently emits zero calls must FAIL, not pass quietly. **Zero emitted
  calls is the pre-existing state, so an AC that cannot distinguish zero from
  one certifies the defect.**
- **AC-ROUTE-SUPPRESSION-INDEPENDENT.** Direct application must be
  mutable/suppressible **independently** of Tail production, Tail propagation,
  Tail consume, and D3B binding — five separately movable axes, and the parent's
  route-variant discriminator is preserved. Control: suppress each alone and
  observe a distinct, named failure. **A shared suppression proves only that
  something in the union matters**, which is how a two-arm defect hides behind a
  one-arm control.
- **AC-BOTH-ARM-BINDING.** D3B binds the delivered fresh `R2` for **both**
  variants, each through its own independently derived Ret capture. Control: an
  arm-specific mutation reds only its own arm; keep ordinary/direct positive
  controls alongside.
- **AC-AFFINE-ONCE.** Duplicate mint and duplicate consume must **fail closed**.
  Control: attempt each and show the named refusal.
- **AC-NO-CLONE.** The constructor field is emitted from a **borrow** and the
  same owner is moved out — no operand clone anywhere on the path. Control:
  assert at the natural producer, not at the consumer.
- **AC-PRODUCTS-EXACT.** The successful candidate proves fresh-`R2`/seed
  **inequality** and the independently derived read and write Ret bindings, with
  the exact `InvalidOffset` products — **exact text, never merely a changed
  default.**
- **AC-CALLER-CLOSURE-COMPILES.** The complete nominal caller closure compiles,
  `cfg(test)` callers included. Control: name the closure; show the build log
  covering every named caller. **A `-p <crate> --lib` build does not compile a
  `cfg(test)` caller.**
- **AC-AFFECTED-CLOSURE.** Cover every target that loads any module whose
  closure this increment changes, whether or not it touches that target's file.
  **Scope by which PATHS changed, never which VALUES changed.** Targeted via
  `scripts/ken-cargo`, never `--workspace`.
- **AC-NO-REGRESSION.** Whole-suite green in CI.

## FORBIDDEN

No empty Tail state, `Unavailable`, or fallback. No cursor proximity or any
recovery by liveness/number/proximity. No runtime carrier, ABI/header field,
capture write, side table, post-emission rewrite, or `answer_route` promotion.
**No resurrection of `TailResumedRetInput` as value authority.** No `Option`
beside `LoweringOperand` in place of the exhaustive state. No Vec/stack for
multiplicity.

## Trust boundary — settled, no operator escalation

Architect, on the Steward's TCB question: this does **not** add to the proof TCB
defined by `spec/60-security/64-trust-model.md §1` — no kernel code, primitive
declaration, postulate, FFI signature, or ABI surface is added. It changes code
inside the **already-existing** native-runtime correctness assumption (§4.3) and
its audit surface, and replacing the clone/out-parameter escape with an
exhaustive affine enum makes that surface **more** explicit. **There is no
operator escalation on TCB-growth grounds** — do not re-raise it.

## Capability

**T1, size L.** Novel design realization with a soundness-bearing affine
discipline, an open family of discriminating mutations, and a two-sided
suppression proof. This is the axis on which the predecessor chain took fourteen
hard stops.

## Contention check

Touches `source.rs`, `core.rs`, and the planner route surface.
[[RT-COMPOSED-RETURN-TRAMPOLINE-EXHAUSTIVE]] touches `core.rs` and **must land
first** — that is why it is `depends_on` and not a parallel node.
[[RT-RESULT-CLOSURE-LIFETIME-CONTAINMENT-CONTROL]] is `active` over
`aggregates.rs`; **it yields to this node only on an explicit Steward release,
never automatically.** Re-measure at release.

## Sequencing

`draft`, queued behind the prefix. **No Runtime release follows from the HS14
ruling itself** — the ruling settles the architecture, and starting a turn is a
separate Steward act.
