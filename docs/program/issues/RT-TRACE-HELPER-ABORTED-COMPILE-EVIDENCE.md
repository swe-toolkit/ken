---
id: RT-TRACE-HELPER-ABORTED-COMPILE-EVIDENCE
title: "Two control.rs trace helpers assert over ABORTED compiles -- owner and multiplicity each run five expressions, EVERY functionized compile aborts, and their trace-event assertions stay green, so zero completed functionized runs back any claim built on them; live on main today and independent of the retirement"
status: merged
owner: runtime
size: S
gate: none
depends_on: []
blocks: []
github: null
origin: "Architect ruling evt_3bkkjpps1bcpe, 2026-08-16: cut this SEPARATELY and do not fold it into RT-DESCENT-RETIRE -- folded in it rides a gated node and stays unfixed while being green. Measured by runtime-leader as D4 of RT-DESCENT-LANE-COMPLETENESS (evt_2fmjv69z5bg2g) at 3c9b8bbd5. Steward-filed per COORDINATION section 2."
---

Frame: `docs/program/wp/RT-TRACE-HELPER-ABORTED-COMPILE-EVIDENCE.md`.

## The defect, measured

In `crates/ken-runtime/src/cranelift_backend/lowering/core/tests/control.rs`,
two helpers — `owner(...)` and `multiplicity(...)` — run a functionized compile
purely to harvest trace events and **never confirm it succeeded**. Both discard
the result with `let (_result, _trace) = px8j_capture_source_trace(expression,
false, symbol);` and then read `d2k_owner_trace_take()`.

**Measured, and it is worse than partial contamination.** Each helper runs five
expressions and **every functionized compile aborts** — row 1
`PlannerInvariant`, rows 4 and 5 `StaticWorkerBinding` — **while their
trace-event assertions stay green. Zero completed functionized runs.**

⇒ **These tests sit on `main` today asserting over aborted compiles.** Whatever
they establish about owner and recognition structure is a **prefix of a compile
that never finished**, and no claim of the form *"the functionized lane produces
owner structure X"* is established by them at all.

## Why this is its own node

**Architect `evt_3bkkjpps1bcpe`, explicitly: cut it separately, do not fold it
into the retirement.** The defect is **live on `main` now** and **independent of
whether the `RecursiveDescent` lane is ever deleted**. Folded into
[[RT-DESCENT-RETIRE]] it would ride a gated node and **stay unfixed while being
green** — which is the same shape as the defect itself.

## Provenance, and why it matters more than the count

Found as `D4` of [[RT-DESCENT-LANE-COMPLETENESS]], which existed because **this
same shape concealed the whole campaign's blocking finding**: the sentinel
`recursive_descent_recursors_compile_without_a_boundary_crossing` captured
`_excluded_result` and discarded it, holding the answer the campaign spent
nineteen days approaching.

**Three of eighteen `set_selector_variant_exclusion(Some(...))` sites discard
the compile result** (Architect census). The sentinel is the third and is
handled under the retirement. **Three of eighteen is narrow, not systemic — the
census was run precisely so nobody has to assume either way. Do not widen the
sweep.**

## Scope

- **In:** the two helpers, their callers, and whatever those callers' assertions
  actually establish once an aborted compile can no longer pass silently.
- **Out:** the sentinel (owned by [[RT-DESCENT-RETIRE]]'s `D6`), the other
  fifteen exclusion sites, and any change to the functionized lane's behaviour.
