---
id: RT-SELECTED-LIBRARY-DECLARATION-CLOSURE
title: "A selected Library package omits a declaration that a delivered owner's declared type references, so checked core cannot read the owner's sort: theorem proof_match (b : Bool) : Top = match b { True ↦ Proved; False ↦ Proved } is refused as UnsupportedProofOnlyMatch on the Library route, though host preparation of the same source delivers Top and admits it. Deliver the declarations a selected owner's type references"
status: ready
owner: runtime
size: M
tier: T1
gate: architect
depends_on: [RT-MATCH-MOTIVE-ADMISSION-SORT]
blocks: []
github: null
origin: "Measured during RT-MATCH-MOTIVE-ADMISSION-SORT AC-1 under Architect outcome (C), evt_7fv8dg4hen613; measurement evt_1x7a2b7bh5nx5. Fails closed: a valid proof owner is refused on the selected Library route. Steward-filed per COORDINATION section 2."
---

# A selected Library package delivers what its owners' types reference

## Objective

A selected Library package delivers every declaration that a delivered
owner's declared type references, so checked core reads the owner's sort
from delivered bytes on the Library route as it does on the host route.

## Settled inputs (measured `evt_1x7a2b7bh5nx5`)

- **The row.** `theorem proof_match (b : Bool) : Top = match b { True ↦
  Proved; False ↦ Proved }`, selected as a Library declaration and decoded
  through `checked_core_declaration_body_view`, has delivered owner type
  `pi(ind_former Bool, const Top)`.
  - `semantic.declarations` does not contain `decl:rt_motive_proof::Top`,
    so `delivered_sort_kind(owner_type)` is `Ok(None)` and admission refuses
    with `UnsupportedProofOnlyMatch`.
  - Host preparation of the same source delivers `decl:rt_motive_host::Top`
    as `opaque`, declared type `omega(level_zero)`, and admits the owner as
    `ProofOnly`.
- **The refusal is correct for its key.** RT-MATCH reads the sort from
  delivered declarations and refuses a missing one (Architect
  `evt_34xv933a05smz`). The defect is the selection, not the reader. No
  producer fallback.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

1. **D0 (measure only).** Where the selected Library package chooses its
   declarations, and which declarations a selected owner's declared type
   references that the selection omits, counted over the six RT-MATCH
   targets. The Architect rules the closure rule.
2. **The ruled closure.**

## Acceptance

- **AC-1.** `proof_match` on the selected Library route is admitted as
  `ProofOnly`, with `Top` in `semantic.declarations`.
- **AC-2 (controls).** The host route and the RT-MATCH AC-3 arm counts are
  unchanged.
- **AC-3 (mutation, QA).** Removing the closure returns `proof_match` to
  `UnsupportedProofOnlyMatch`.

## Stop conditions

- A closure that delivers a declaration the selection's owners do not
  reference.
- Any kernel, `trusted_base()` or spec change.
