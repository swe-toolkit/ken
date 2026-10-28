---
id: RT-CERTIFIER-ESCAPED-SLOT-ZERO-ARGUMENT-CALL
title: "After a pointer to a stack slot is stored to the heap, the finished-CLIF Result certifier treats a later zero-argument call as unable to write the slot, because call_may_access_location checks access.escaped only inside its argument loop. Return true for an escaped location before the loop"
status: ready
owner: runtime
size: S
tier: T1
gate: architect
depends_on: [RT-CERTIFIER-DERIVED-STACK-POINTER-CLOBBER]
blocks: []
github: null
origin: "Architect review evt_4em6knbncena7 of RT-CERTIFIER-DERIVED-STACK-POINTER-CLOBBER (candidate 819dabd1): pre-existing on main, out of that candidate's scope, should-fix before ABI-S6-HS18-MAIN-BASED-CLOSURE increment C. Requested by the Runtime Leader in evt_7jbmssk0bvjtw. It is the 'escaped plus zero-argument call' residual that increment A disclosed. Steward-filed per COORDINATION section 2."
---

# A call after a slot escapes may write the slot

## Objective

Once a location has escaped, every later call is treated as possibly
writing it, including a call that takes no arguments. A stale `Exact` state
no longer survives such a call.

## Fixed inputs (main `afc5e0fae`)

- **The gap.** `call_may_access_location` in
  `crates/ken-runtime/src/cranelift_backend/lowering/units.rs` (`:12265`)
  tests `access.escaped` only on an argument that resolves to `None`, inside
  `for argument in func.dfg.inst_args(call)`. A zero-argument call never
  enters the loop. Storing a pointer to the heap creates no retaining call,
  so `call_follows_retention` returns false too.
- **The directed fix (Architect `evt_4em6knbncena7`).** Return `Ok(true)` at
  the top of `call_may_access_location` when `access.escaped`, before the
  loop. The function's other arms are unchanged.
- **Sequencing.** RT-CERTIFIER-DERIVED-STACK-POINTER-CLOBBER (candidate
  `819dabd1`) changes the same function. Cut this node from main after that
  candidate lands, and do not stack it on the unlanded branch.

Treat anchors as perishable. If a fixed input is false on the landed base,
stop and report the mismatch; do not build around it.

## Deliverable

1. **The pin, first.**
   - In a single function: store the slot's `stack_addr` to the heap, then
     make a fresh exact store to the slot.
   - Then call `later_call()` with zero arguments, then load the slot.
   - Query `reaching_memory_states_before` at the load. The pin requires
     `{Unknown}`.
   - Show it red on the cut base, with the falsely retained `Exact`, before
     the fix.
2. **The fix.** The directed early return, in `units.rs` only.

## Acceptance

- **AC-1.** The pin is red on the cut base and green with the fix. The seven
  RT-CERTIFIER-DERIVED-STACK-POINTER-CLOBBER pins stay green.
- **AC-2.** A control in the same shape, without the heap store, keeps
  `Exact` after the zero-argument call. This shows the fix keys on
  `escaped`, not on every call.
- **AC-3.** `rt_span_prov_native`, `px7f_resource_native` and the named
  `rt_parity_native` rows do not regress. A newly refused certified row is a
  stop, not a pin update.

## Stop conditions

- A certified row in AC-3 is refused after the fix. Report it to the
  Architect.
- The fix needs a change outside `units.rs`, or beyond the early return.

Checks: 4 fired: the loop-only `access.escaped` test was read at
`units.rs:12270` to `:12277` on `afc5e0fae`. 8 fired: AC-2's no-escape
control would also go `Unknown` under a fix that treated every call as a
clobber, so the AC-1 pin alone is not evidence.
