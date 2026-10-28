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

# A mixed-caller generated context is pinned

## Objective

A test pins what the checked-IH generated-context gate does when its
selected callers disagree: one has a completed return boundary and one does
not. The completed caller keeps its contract. The non-completed caller's
path is refused fail-closed with the FinishedCode cause.

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

1. **The count.** The first act counts the generated contexts in the existing
   native and lib corpora that have two or more selected callers whose
   `checked_ih_static_response_return_boundary` results disagree. Use the
   existing px8-ds diagnostic, and state the membership rule of the corpus
   counted.
2. **The fixture.**
   - If the count is nonzero, the fixture uses one of the counted contexts.
   - If the count is zero, it is hand-authored at plan level, in the same way
     as the row-4 controls.

## Acceptance

- **AC-1.** The fixture asserts that the completed caller's contract exists.
  It also asserts that the non-completed caller's path is refused with the
  FinishedCode cause, and with no other cause.
- **AC-2.** Mutate the gate from "any" to "all", keeping the fixture. The
  fixture compiles and reddens at the AC-1 contract assertion. Restore the
  gate byte-identically.
- **AC-3.** The existing checked-IH pins pass unchanged: increment A's
  presence pin and the restore-k row.

## Stop condition

The fixture shows behavior that diverges from the ruled "any". That is a
precommitted, non-advancing query to the Architect. Do not change the gate.

Checks: 4 fired: both cited functions exist at `:5346` and `:5584` in
`responses.rs` on `a10d37eb3`. 6 fired: whether a disagreeing context exists
is not assumed; it is the deliverable's first act, with its corpus named.
