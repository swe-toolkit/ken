---
id: RT-COMPOSED-RETURN-TRAMPOLINE-EXHAUSTIVE
title: "Replace the producer-trampoline clone/out-parameter return protocol with an exhaustive Continue/Complete step result — core.rs:3144-3164 clones answer.value only to return an ignored bare value while ProducerTrampolineWork owns the real answer, and a pending affine result cannot cross that shape. Behaviour-preserving PREFIX to the composed-return protocol replacement; mints no Produced state and changes no product."
status: active
owner: runtime
size: M
gate: none
tier: T1
depends_on: []
blocks: [RT-COMPOSED-RETURN-PRODUCED-TRANSFER]
github: null
origin: "Architect HS14 component-design ruling evt_7gnw8s9k7rh6, 2026-08-29, on Steward disposition request evt_6dvw8j96w2sdx. The ruling selected arm 1 (widen the composed return protocol) and named the trampoline as a mandatory part of the replacement: 'A behavior-preserving trampoline/ordinary-return refactor may be a separately landable prefix only if it mints no Produced state and every existing product stays unchanged.' Steward cut this as a separate node under that permission; the atomic remainder is RT-COMPOSED-RETURN-PRODUCED-TRANSFER. Steward-owned cut per COORDINATION section 2 and steward.md section 4."
---

> # THIS NODE IS THE INERT HALF. IT MINTS NO `Produced` STATE.
>
> The Architect permitted this prefix **conditionally**, and the condition is the
> whole reason it may land alone: *"only if it mints no `Produced` state and
> every existing product stays unchanged."* **The moment this node mints a
> production `Produced`, it stops being a prefix and becomes part of the atomic
> candidate**, which is [[RT-COMPOSED-RETURN-PRODUCED-TRANSFER]] and is not this
> node's to start.
>
> **`draft` — NOT RELEASED.** `blocks:` is a dependency record, not
> authorization. Flip `draft` -> `ready` -> `active` on an explicit Steward
> release, because a dispatched node left at `ready` is invisible to the
> per-node watchdog sweep.

## Why this exists

[[RT-RESULT-CONTINUATION-BINDING-PROVENANCE]] is **closed as structurally
refuted** at hard stop 14. Its corrected D0 returned NO: the produced operand
becomes constructor-field material at `ConstructArgument`, and while its SSA
survives as a constructor field, its **top-level compiler-control state does
not**. The obstruction is the general composed-return protocol, which that node
was forbidden to change.

The Architect ruled the successor architecture is to **replace that protocol**.
This node is the behaviour-preserving half of the replacement, cut out so the
semantic candidate does not also carry a mechanical refactor.

## Fixed inputs, from the ruling (bound blobs)

**Re-measure at the release SHA. These are the ruling's coordinates, not a
survey of the tree.**

- Trampoline defect: `core.rs:3144-3164` **clones `answer.value` only to return
  an ignored bare value**, while `ProducerTrampolineWork` owns the real answer.
- Drivers that must loop on the new step result: `core.rs:3100-3142` and
  `core.rs:3905-3947`.
- Bound blobs at the ruling: `source.rs` `88fcc401b0e078f78298a0998d09364b22e64a27`,
  `core.rs` `68f9394ce4d75f68bcfbaaeff7b294040a4fd50b`,
  `mod.rs` `2ee945bc07c2facbbe016b505f8a8ab449862c44`, planner route
  `9eb2c118e227c3a7db2849e03046db02d93a48eb`.

## Deliverables

- **D0 — enumerate the boundary, before editing.** Every caller and every return
  boundary that the step result crosses, plus the complete nominal caller
  closure. **Compile every named caller, including `cfg(test)` callers** — a
  `-p <crate> --lib` build does not compile a `cfg(test)` caller, and that
  omission has already cost this chain one rejected D0.
- **D1 — the exhaustive step result.** Replace the out-parameter-plus-ignored-
  clone protocol with an exhaustive result such as
  `Continue(ProducerTrampolineWork)` versus `Complete(ComposedReturn)`. Both
  drivers loop on `Continue`; **only `Complete(Ordinary)` may exit through an
  ordinary public boundary.**
- **D2 — the inertness differential.** Show every existing product unchanged.

## Acceptance criteria, each with its control

- **AC-NO-PRODUCED.** This node mints **no** production `Produced` state and
  introduces no Tail side channel. Control: a census of production mint sites
  returning zero. **This is the condition the prefix permission rests on — if it
  fails, the work belongs in [[RT-COMPOSED-RETURN-PRODUCED-TRANSFER]] and this
  node does not land.**
- **AC-PRODUCTS-UNCHANGED.** Every existing product is byte-identical across the
  increment. Control: a differential over the products, not a statement that the
  suite is green. **"The suite still passes" is not evidence of inertness** —
  it is consistent with a changed product no test pins.
- **AC-EXHAUSTIVE-NOT-OPTIONAL.** The step result is an exhaustive enum, and
  removing any propagation arm is a **compile error** or a named fail-closed
  error — never a silent fallthrough. Control: delete one arm and show the
  build fails with the exact error; restore and show it byte-restored.
- **AC-NO-IGNORED-CLONE.** The clone-to-return-an-ignored-value shape at
  `core.rs:3144-3164` is GONE, not merely bypassed. Control: assert its absence
  at the natural producer, and show the replacement carries the real answer
  rather than a copy of it.
- **AC-CALLER-CLOSURE-COMPILES.** The complete nominal caller closure compiles,
  `cfg(test)` callers included. Control: name the closure and show the build
  log covering every named caller.
- **AC-AFFECTED-CLOSURE.** Re-run the COMPLETE AFFECTED-TARGET CLOSURE, not the
  diff-touched set: every target that loads any module whose closure this
  increment changes, whether or not the increment touches its file. **Scope by
  which PATHS changed, never by which VALUES changed** — the parent chain's CI
  red had every production value byte-identical on both sides and still broke an
  untouched consumer. Targeted via `scripts/ken-cargo`, never `--workspace`.
- **AC-NO-REGRESSION.** Whole-suite green in CI, which is where the workspace
  gate runs. Local builds stay targeted.

## HARD STOP

**If the trampoline cannot be made exhaustive without minting a `Produced`
state, STOP and return that.** It means the prefix is not separable and the
whole replacement is one candidate. **That is a finding and a complete
deliverable, not a failure** — and it is strictly cheaper to learn here than
inside the semantic candidate.

## FORBIDDEN

No `Produced` mint. No Tail side channel. No `Unavailable` or empty state, no
fallback, no cursor proximity, no runtime carrier, no ABI/header field, no
capture write, no side table, no post-emission rewrite, no `answer_route`
promotion. Do not touch the D3B Ret-input binding or the exact Tail consume —
those are the successor's.

## Capability

**T1, though the diff is largely mechanical.** Size and capability are
independent axes (`steward.md` §4h). The deliverable is not the edit but the
**inertness argument**: proving a general return protocol changed shape while no
product moved. This chain has twice shipped a control that passed for the wrong
reason, and "behaviour-preserving" is exactly the claim that is easy to assert
and hard to earn.

## Contention check

Touches `core.rs` in the runtime backend.
[[RT-RESULT-CONTINUATION-BINDING-PROVENANCE]] is CLOSED and holds nothing.
[[RT-RESULT-CLOSURE-LIFETIME-CONTAINMENT-CONTROL]]
is `active` over `aggregates.rs` — a different file, but **re-measure at release
rather than inheriting this sentence.**

## Sequencing

`draft`, queued. It blocks [[RT-COMPOSED-RETURN-PRODUCED-TRANSFER]], which
cannot start until either this lands or its HARD STOP proves the two are
inseparable. Release when the runtime seat frees.
