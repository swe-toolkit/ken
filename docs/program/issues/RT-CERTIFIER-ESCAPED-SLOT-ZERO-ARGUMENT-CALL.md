---
id: RT-CERTIFIER-ESCAPED-SLOT-ZERO-ARGUMENT-CALL
title: "The finished-CLIF Result certifier keeps a stale Exact state whenever a pointer to the tracked slot is in hand but no consumer's partial evidence shows it: a pointer reloaded from a container slot with stack_load, an escaped slot before a zero-argument call, or a retained address before an unknown store. Close all three with one memory-read rule in the slot walk and one leak predicate that every consumer uses"
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

> **AMENDED 2026-10-29 (Architect `evt_zdyes4xcv8fc`, hard stop
> `evt_3pjmdgmw69zt8`).** Under rules (i) and (ii), AC-3 refused two
> `SP_A_FREEZE` rows in `rt_span_prov_native`. The ruling requires a
> step-1 trace and bisection first. It pre-rules one repair, for one
> outcome only: rule (i) is replaced by (i'), under which `stack_load S`
> depends on the values stored into S, through a new `stack_slot_stores`
> index. Pin P5 is added with it. Every other outcome is a hard stop with
> the trace. Everything else stands.

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
precise alternative, where reads through memory yield the stored values
rather than the slot, is not authorized,
except as (i') under the condition in `evt_zdyes4xcv8fc`.

## Deliverable

Rules (i) and (ii) as ruled, and the pins below. Every pin queries
`reaching_memory_states_before(A, return)`.

- **P1.** Container reload in both spellings, `stack_load C` and
  `load(stack_addr C)`, each followed by a call, a store through the pointer,
  and a heap store. All six rows give `{Unknown}`, and `retaining_calls`
  holds the first call. The three `stack_load` rows are red first.
- **P2.** A copy chain `C → stack_load → D → load(stack_addr D)`, then a
  store through the pointer, gives `{Unknown}`.
- **P3.** Heap-escaped, re-stored, then a zero-argument call gives
  `{Unknown}`. Red first.
- **P4.** `call f(stack_addr A)`, re-store, then a store through a heap
  pointer gives `{Unknown}`. It must be red first. If it is not, drop the
  row and post why.
- **Precision controls.** Each keeps `{Exact}`:
  - P4 without the call;
  - `stack_load E` passed to a call, where `E` never held A's address.

## Acceptance

- **AC-1.** P1 to P4 and both precision controls pass. Each red-first row is
  shown red on the cut base.
- **AC-2.** Each of m1 to m4 from the ruling reddens its own pins, and is
  restored byte-identically:
  - m1: drop the StackLoad insert;
  - m2: move `escaped` back inside the argument loop;
  - m3: drop the predicate from `unknown_write_may_alias`;
  - m4: make the predicate always true.
- **AC-3.** The named native population used in the mixed-caller AC-3
  (`rt_parity_native` named rows, `rt_span_prov_native`,
  `px7f_resource_native`, the lib fixtures) is green on the candidate.

## Stop conditions

- A named plan flips from accepted to refused. Hard-stop with that plan and
  its query named. Do not narrow either rule without a ruling.
- The fix needs a change outside `units.rs`.

Checks: 2 fired: the five walk consumers and the two leak-flag reads are
taken from the Architect's read of `28f3dbb36`. They are cited, not
re-derived. 8 fired: a fix that returns Unknown everywhere passes P1 to P4,
so the precision controls and m4 are kept as the discriminating rows.
