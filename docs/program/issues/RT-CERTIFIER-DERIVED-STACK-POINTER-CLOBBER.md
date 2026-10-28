---
id: RT-CERTIFIER-DERIVED-STACK-POINTER-CLOBBER
title: "The finished-CLIF Result certifier treats a call as unable to write a stack slot when the call receives a pointer derived from that slot by a dynamic offset, a select or a block-param merge, and an earlier store then counts as the slot's exact current contents. The store path already catches the same pointers. Classify call arguments, retained-call arguments and stored values by slot dependency, not exact resolution"
status: merged
owner: runtime
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Adversary finding evt_3c4jqmy7jxx7w on ABI-S6-HS18-MAIN-BASED-CLOSURE increment A (bd5c64ed1), accepted as a should-fix and ordered before increment C by the Architect's increment B D0 (evt_43ekkkz2adr43, item d). Steward-filed per COORDINATION section 2."
---

# A call that receives a derived slot pointer may write the slot

## Objective

When a call receives a pointer derived from a stack slot, or such a pointer
is stored to memory and a later call runs, the certifier treats the slot as
possibly written. A stale `Exact` state no longer satisfies
`verify_definite_stored_value` after such a call.

## Fixed inputs (main `a10d37eb3`)

- **The gap.** In `crates/ken-runtime/src/cranelift_backend/lowering/units.rs`,
  three sites resolve stack addresses exactly. A derived pointer resolves to
  `None` there, and `None` counts only when `access.escaped` is set:
  - `call_may_access_location` (`:12265`);
  - `call_address_arguments` (`:11867`), so no retaining call is recorded;
  - the stored-value arm of `stored_slots` (`:12066`), so `escaped` is never
    set.
- **The sibling that is right.** `unknown_write_may_alias` (`:14030`) uses
  `address_depends_on_slots` (`:12222`), which walks operands, block params
  and unions. A store through the same pointers is caught.
- **The ruling.** The certifier must not depend on what lowering currently
  emits, so "unreachable from current lowering" is not a disposition
  (Architect `evt_43ekkkz2adr43`, item d).
- **Fix shape (Architect).** Classify call arguments, retained-call arguments
  and stored values with `address_depends_on_slots`, or one shared walk.
  Keep exact resolution only where an exact offset is consumed.
- **The cases.** The Adversary's seven, in `evt_3c4jqmy7jxx7w`. They use one
  function with a 16-byte slot `ss0` and an `i64` parameter `idx`, and they
  query `reaching_memory_states_before` at `ss0+0`, 8 bytes, before the
  return. Four cases are wrong today:
  - the `iadd` pointer passed to `g`;
  - the `select` pointer passed to `g`;
  - the block-param pointer passed to `g`;
  - the `iadd` pointer stored to the heap before `h()`.

  Three are right today and are controls:
  - the exact `stack_addr` passed to `g`;
  - a store through the `iadd` pointer;
  - the exact pointer stored to the heap before `h()`.

  The D0 says "five wrong, two controls". The Adversary's list, counted case
  by case, is four and three, and the list governs.

Treat anchors as perishable. If a fixed input is false on the landed base,
stop and report the mismatch; do not build around it.

## Scope

`units.rs` only, production plus its `#[cfg(test)]` pins. Out of scope:
- every other file under `lowering/`, including the lowering that emits
  pointers;
- `source.rs` and `core.rs`, which increment B touches;
- the separate "escaped plus zero-argument call" residual that increment A
  disclosed.

## Deliverable

The three sites classify by slot dependency, as ruled, with the seven cases
landed as unit pins.

## Acceptance

- **AC-1.** The four wrong cases yield `{Unknown}`, and the heap-store case
  also sets `escaped = true`. The three controls are unchanged.
- **AC-2.** Reverting each of the three sites alone, with the pins kept,
  reddens that site's own pin. QA reproduces each revert, and each is
  restored byte-identically.
- **AC-3.** The certified populations A used do not regress:
  - `rt_span_prov_native`;
  - `px7f_resource_native`;
  - the named `rt_parity_native` rows.

  A newly refused certified row is a stop, not a pin update.

## Stop conditions

- A certified row in AC-3 is refused after the fix, which means a current
  lowering emits a derived stack pointer to a call. Report it to the
  Architect.
- The fix needs a change outside `units.rs`.

Checks: 6 fired: the case population was counted from the Adversary's list,
which has seven cases, four wrong and three right. That differs from the
D0's five and two, as recorded above. 4 fired: the four cited functions
exist at the cited lines on `a10d37eb3`, and line `:14045` shows the
dependency walk in use.
