---
id: ABI-S6-HS18-CHECKED-IH-MIXED-CALLER-FIXTURE
title: "The checked-IH generated-context gate gives every selected caller a result contract when ANY of them has a completed return boundary, but no fixture has callers that disagree. Count the disagreeing contexts, then pin the ruled any behavior together with a fail-closed FinishedCode refusal on the non-completed caller's path"
status: ready
owner: runtime
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Carried follow-on of ABI-S6-HS18-MAIN-BASED-CLOSURE increment A (bd5c64ed1), split out of increment B as its own node by the Architect's increment B D0 (evt_43ekkkz2adr43, item c). Steward-filed per COORDINATION section 2."
---

> **AMENDED 2026-10-28 (Architect `evt_65wp3btz7m0c3`).** The census found
> no context with two or more selected callers among 34 named plans
> (`evt_7hbdq2rpegrrc`). A forged plan-level caller is refused by owner and
> target validation before the gate, so it cannot tell "any" from "all".
> The Deliverable, Acceptance and Residual sections below replace the
> original plan-level fixture. Everything else stands.

# A mixed-caller generated context is pinned

## Objective

A test pins what the checked-IH generated-context gate does when its
selected callers disagree: one has a completed return boundary and one does
not. The context keeps its contract. That rule is extracted into a pure
helper and pinned there, as amended. The FinishedCode half is a stated
residual.

## Fixed inputs (main `a10d37eb3`)

- **The gate.**
  - `checked_ih_generated_context_result_contract` is in
    `crates/ken-runtime/src/cranelift_backend/planning/static_transition/responses.rs`
    at `:5584`.
  - It gives every caller a contract when ANY selected caller has a completed
    boundary, as `checked_ih_static_response_return_boundary` (`:5346`)
    reports it.
  - "Any" is the ruled behavior. This node pins it and does not change it.
- **Why it is separate from B.** B touches `source.rs`. This gate is
  increment A's plane, in another file, so folding it into B would mix two
  causes into B's mutation story (Architect `evt_43ekkkz2adr43`, item c).

Treat anchors as perishable. If a fixed input is false on the landed base,
stop and report the mismatch; do not build around it.

## Deliverable

1. **The count (done).** In 34 named plans there were 0 contexts with two
   or more selected callers (`evt_7hbdq2rpegrrc`):
   - 26 native: the four named `rt_parity_native` rows, all of
     `rt_span_prov_native` and all of `px7f_resource_native`;
   - 8 lib: six `static_transition::responses::tests` fixtures.
2. **A pure extraction, with no change in behavior.**
   - Extract the "any" rule from `checked_ih_generated_context_result_contract`
     into the private `generated_context_contract_from_boundaries(completed,
     identities)`, verbatim from the ruling.
   - The gate still evaluates every selected caller's boundary first, so its
     errors propagate as before.
3. **A unit fixture** on the helper, in `responses.rs`'s `mod tests`. It
   takes two distinct `ConstructorIdentity` values, `a` and `b`.

Scope: `responses.rs` only.

## Acceptance

- **AC-1.** The four helper rows pass:

  | Completed flags | Identities | Expected |
  |---|---|---|
  | `[true, false]` | `[a]` | `Ok(Some(a))` |
  | `[false, true]` | `[a]` | `Ok(Some(a))` |
  | `[false, false]` | `[a]` | `Ok(None)` |
  | `[true]` | `[a, b]` | `Err`, "disagreeing" |

- **AC-2.** Change `.any(` to `.all(` in the helper only. It compiles, and
  both mixed rows go red at their `Some(a)` assertion. Restore the file
  byte-identically.
- **AC-3.** Behavior is preserved:
  - increment A's presence pin and the restore-k row pass;
  - the px8-ds generated-context diagnostic over the same 34 named plans
    records the same `(context, contract)` pairs before and after the
    extraction.

## Residual (stated, not measured)

No pin shows the downstream FinishedCode refusal on a non-completed caller
that holds a contract. Its population is 0 of 34 named plans. The first WP
whose plan produces a context with two or more distinct selected callers
owes the real-plan fixture for that half.

## Stop condition

The fixture shows behavior that diverges from the ruled "any". That is a
precommitted, non-advancing query to the Architect. Do not change the gate.

Checks: 4 fired: both cited functions exist at `:5346` and `:5584` in
`responses.rs` on `a10d37eb3`. 6 fired: whether a disagreeing context exists
is not assumed; it is the deliverable's first act, with its corpus named.
