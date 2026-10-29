---
id: RT-CERTIFIER-ESCAPED-SLOT-ZERO-ARGUMENT-CALL
title: "The finished-CLIF Result certifier keeps a stale Exact state whenever a pointer to the tracked slot is in hand but no consumer's partial evidence shows it: a pointer reloaded from a container slot with stack_load, or an escaped slot before a zero-argument call. Close both with a stored-values read rule in the slot walk and one leak predicate for calls; retention before an unknown store is a stated residual"
status: ready
owner: runtime
size: M
tier: T1
gate: architect
depends_on: [RT-CERTIFIER-DERIVED-STACK-POINTER-CLOBBER]
blocks: []
github: null
origin: "Architect review evt_4em6knbncena7 of RT-CERTIFIER-DERIVED-STACK-POINTER-CLOBBER (zero-argument call after escape; requested by the Runtime Leader evt_7jbmssk0bvjtw). Widened by Architect ruling evt_2tsqhgywfxdb to fold in Adversary finding evt_3v8e7x9w7ar4f (stack_load reload, on 28f3dbb36) and a third gap read by the Architect (retention before an unknown store). Should-fix before ABI-S6-HS18-MAIN-BASED-CLOSURE increment C. Steward-filed per COORDINATION section 2."
---

> **AMENDED 2026-10-29 (Architect `evt_zdyes4xcv8fc`, `evt_15rkc0pjxhm1z`,
> then `evt_5gty25zxqwndr` after a QA block of `9288e1902`; hard stops
> `evt_3pjmdgmw69zt8`, `evt_3ts5d91sq7amt`;
> §1a count 2).** Rules (i) and (ii) refused two `SP_A_FREEZE` rows in
> `rt_span_prov_native`, and the bisect showed each rule refuses on its own.
> The WP is split:
> - rule (i) is replaced by (i');
> - rule (ii) applies to calls only;
> - gap (c), retention before an unknown store, leaves this WP for
>   `RT-CERTIFIER-CALLEE-SLOT-RETENTION`, together with P4 and m3;
> - each `stack_slot_stores` writer arm gets its own pin (P6, P7) and
>   mutant (m5a, m5b).
>
> The Deliverable and Acceptance below are the amended ones.

# A pointer to the slot that is in hand may write the slot

## Objective

The certifier gives one answer to "can a pointer to the tracked location be
in hand at this instruction?" for every consumer. A stale `Exact` state no
longer survives any operation that can write the location through such a
pointer. Precision is kept where no pointer can be in hand.

## Fixed inputs (main `28f3dbb36`, Architect `evt_2tsqhgywfxdb`)

All in `crates/ken-runtime/src/cranelift_backend/lowering/units.rs`. The
Architect read these and did not compile them.

- **(a) The walk.** `address_dependent_slots` (`:12223`) follows value
  operands only. `stack_load C` has none, so a pointer reloaded from
  container `C` depends on nothing. `load(stack_addr C)` reaches `C` through
  its operand (Adversary `evt_3v8e7x9w7ar4f`, measured).
- **(b) Escape.** `call_may_access_location` (`:12280`) consults `escaped`
  only inside its argument loop, so a zero-argument call never sees it.
- **(c) Retention.** `unknown_write_may_alias` (`:14037`) checks `escaped`
  and the walk, but never `call_follows_retention`. This was read by the
  Architect, not measured.
- **The closure, verbatim from the ruling:**
  - (i) The walk's `Result` arm inserts `S` for `stack_load S`. This is
    complete for StackSlot readers in cranelift 0.113.1.
  - (ii) One `location_leaked_before(func, inst, access)`, equal to
    `access.escaped || call_follows_retention(...)`. It is used at the top
    of `call_may_access_location` and in the last line of
    `unknown_write_may_alias`.
  - All five walk consumers inherit (i): `:11882`, `:12091`, `:12099`,
    `:12280` and `:14052`.

Treat anchors as perishable. If a fixed input is false on the landed base,
stop and report the mismatch; do not build around it.

## Scope

`units.rs` only, production plus `#[cfg(test)]` pins in
`generated_result_protocol_verifier`, extending `slot_pointer_case`. The
alternative where `load(stack_addr S)` also yields S's stored values is not
authorized.

## Deliverable

The rules as amended in `evt_15rkc0pjxhm1z`, and the pins below. Every pin
queries `reaching_memory_states_before(A, return)`.

- **(i').** A new cached `stack_slot_stores` index on `IndexedFunction`
  records every value stored into each slot at an exact stack address. The
  walk's `StackLoad` arm pushes the values stored into S, not S itself.
- **(ii), calls only.** `location_leaked_before` is
  `access.escaped || call_follows_retention(...)`, used first in
  `call_may_access_location`. `unknown_write_may_alias` keeps its
  `28f3dbb36` last line, with no retention.


- **P1.** Container reload in both spellings, `stack_load C` and
  `load(stack_addr C)`, each followed by a call, a store through the pointer,
  and a heap store. All six rows give `{Unknown}`, and `retaining_calls`
  holds the first call. The three `stack_load` rows are red first.
- **P2.** A copy chain `C → stack_load → D → load(stack_addr D)`, then a
  store through the pointer, gives `{Unknown}`.
- **P3.** Heap-escaped, re-stored, then a zero-argument call gives
  `{Unknown}`. Red first.
- **P5.** `v = stack_load A` (the scalar 7), store v to heap `0x1000`, then
  a zero-argument call gives `{Exact}`. Red first against rule (i).
- **P6.** `store stack_addr(A), stack_addr(C)`; `p = stack_load C`;
  `store 9, p` gives `{Unknown}`. It pins the `Store` format arm, which
  also covers `istore8/16/32`; those get no separate rows.
- **P7.** The same shape with a direct `atomic_store stack_addr(A),
  stack_addr(C)` gives `{Unknown}`. It pins the `StoreNoOffset` arm, whose
  only opcode is `atomic_store`. Do not use the `StoreForm::Atomic` helper,
  which emits `atomic_rmw`.
- **Precision controls.** Each keeps `{Exact}`:
  - `stack_store 7, A; p = load(iconst 0x1000); store 9, p`, a heap store
    with no leak, followed by an unrelated zero-argument call;
  - `stack_load E` passed to a call, where `E` never held A's address.

## Acceptance

- **AC-1.** P1, P2, P3, P5, P6, P7 and both precision controls pass. Each
  red-first row is shown red on the cut base.
- **AC-2.** Each mutant reddens its pins and is restored byte-identically:
  - m1, drop the `StackLoad` arm: P1's `stack_load` rows and P2;
  - m2, move `escaped` back inside the argument loop: P3;
  - m4, make `location_leaked_before` always true: both precision controls;
  - m5a, remove only `Store` from the writer or-pattern: P6 red, P1 and P7
    green;
  - m5b, remove only `StoreNoOffset`: P7 red, P1 and P6 green.
- **AC-3.** The named native population used in the mixed-caller AC-3
  (`rt_parity_native` named rows, `rt_span_prov_native`,
  `px7f_resource_native`, the lib fixtures) is green on the candidate.
  Both `SP_A_FREEZE` rows pass. The four-name parity run reports its build
  time and test time separately.

## Residual (stated, owned elsewhere)

The certifier assumes a call that receives a slot address does not publish
that address into memory the caller later reloads and writes through.
Retention is consulted for later calls, not for later unknown-destination
stores. This gap predates this WP. (i')'s soundness for slot contents
written by a callee rests on the same assumption. Owner:
`RT-CERTIFIER-CALLEE-SLOT-RETENTION`.

## Stop conditions

- A named plan flips from accepted to refused. Hard-stop with that plan and
  its query named, in the step-1 trace format. If `escaped` is still true
  under (i'), name the store into the slot whose stored value walks to a
  slot. Do not narrow either rule without a ruling.
- The fix needs a change outside `units.rs`.

Checks: 2 fired: the five walk consumers and the two leak-flag reads are
taken from the Architect's read of `28f3dbb36`. They are cited, not
re-derived. 8 fired: a fix that returns Unknown everywhere passes P1 to P3,
so the precision controls and m4 are kept as the discriminating rows, and
P5 separates (i') from (i).
