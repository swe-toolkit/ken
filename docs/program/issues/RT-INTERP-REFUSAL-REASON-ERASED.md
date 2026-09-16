---
id: RT-INTERP-REFUSAL-REASON-ERASED
title: "ambient_dispatch returns Result<EvalVal, ()>, so the reason a host operation was refused is erased at the ken-interp boundary. A Ken program cannot be told whether it hit OperationUnavailable, a capability denial, or a backend fault, and an inverted test there passes for any failure."
status: draft
owner: runtime
size: unsized
gate: none
depends_on: []
blocks: []
github: null
tier: T1
origin: "Surfaced by runtime-implementer while building RT-UNAVAILABLE-OP-UNIFORM-REFUSAL-GATE, 2026-09-16. Architect ruled it OUT OF SCOPE for that WP and directed it be filed separately; Steward cut this node. It is a pre-existing boundary defect that the gate WP makes more visible by adding a refusal, not a defect the gate introduces."
---

> # DRAFT. Not framed, not released. Do not start.

# The defect

    crates/ken-interp/src/eval.rs
    fn ambient_dispatch<H: HostHandler>(...) -> Result<EvalVal, ()>

`ken-host` produces a structured refusal — `TerminalErrorV1`, carrying
`OperationUnavailable(HostOpV1)` among other variants. **`ambient_dispatch`
discards all of it and returns `()`.**

Two consequences, and the second is the one that bites soonest:

1. **A Ken program cannot be told why its operation was refused.** Unavailable
   op, capability denial, and backend fault are one indistinguishable failure at
   the language boundary. Whatever Ken's eventual error surface is, it cannot be
   built on a unit.

2. **No test at that boundary can assert a REASON.** An inverted assertion can
   say *"it refused"* and cannot say *"because `OperationUnavailable`"* — so it
   passes for **any** failure, including one the change under test introduced by
   accident. This is a negative check with nothing to hold it up.

# Why it is filed rather than folded

[[RT-UNAVAILABLE-OP-UNIFORM-REFUSAL-GATE]] adds a refusal on this path and so
raises the cost of the erasure, which is how it was found. It does not cause it
— the unit error type predates the gate. The Architect ruled widening the error
type out of that WP's scope, and the WP mitigates locally instead: every
inverted test it lands must pair the refusal with a **success through the same
helper**, so the assertion is two-sided even though the reason is unavailable.

**That mitigation is a discipline on test authors, not a fix.** It has to be
remembered at every future test on this boundary, and nothing enforces it. The
fix is the error type.

# What a frame for this owes

- The full consumer inventory of `ambient_dispatch`'s `Err(())` — every caller
  that pattern-matches on it, and what each would do with a reason.
- Whether the right shape is `TerminalErrorV1` surfaced through, a
  `ken-interp`-local error enum, or something the language surface can
  eventually name. This is a layering call and belongs to the Architect.
- Whether Ken's error surface is settled enough to receive a reason at all; if
  it is not, that is the real dependency and should be named as one rather than
  discovered mid-build.

# Related

- [[RT-UNAVAILABLE-OP-UNIFORM-REFUSAL-GATE]] §4 point 8 — the local mitigation
  and the positive-control requirement it forces.
