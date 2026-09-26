---
name: a-newly-reachable-allocation-producer-must-be-censused-against-its-sibling-allocators-declared-capacity-governor
description: >-
  ABI-S6 D3 (8726f73d, PR #3444) post-merge hunt, verdict NO DEFECT + one
  bounded gap observation. D3 wires the represented MappingAllocate producer
  (dispatch executes on the interpreter) whose backing allocation takes an
  attacker-influenceable length: u64. Its constructor try_new_anonymous is
  fail-closed per allocation (rejects length==0, u64->usize overflow and OOM
  both map to AllocationFailed via try_reserve_exact -- no panic/abort) BUT
  carries NO capacity governor: no per-mapping cap and no invocation-wide
  live-mapping accounting, whereas the sibling BufferAllocate enforces
  per_buffer_max_capacity (1 MiB, DECLARED in the sealed ABI catalog) and an
  invocation_max_live_capacity aggregate. Inert in D3 because the producer is
  unreachable from Ken source (no perform syntax; elaborator does not gate on
  availability), so only the trusted host API drives it. Goes live at the
  future checked producer: a source program could then request arbitrarily
  large or arbitrarily many mappings, bounded only by the process allocator.
  Method: when a deferred increment makes an ALLOCATION producer reachable,
  census its capacity/quota governors against its nearest sibling allocator's
  ABI-declared limits; an absent governor is a resource-exhaustion seam the
  producer increment must consciously rule on (declare a limit or affirm
  unbounded-but-graceful). Per-allocation graceful failure is NOT an aggregate
  bound. Sibling of the name-the-producer-deliverable seam discipline. (The
  mapping limit was declared at ABI-S6 D4, cf894cdb5.)
metadata:
  type: feedback
---

# A newly reachable allocation producer must be censused against its sibling allocator's declared capacity governor

**Measured 2026-09-09 on `8726f73d` (ABI-S6 D3, PR #3444), a post-merge hunt.
Verdict NO DEFECT; this is the one bounded observation it surfaced
(routed to the Steward side thread, evt_n041yasg5m26).**

D3 is the "represented-substrate" increment: it wires `MappingAllocate` (0x0404)
so `dispatch_host_op_v1` actually executes on the interpreter
(`RepresentedUnavailable` is the NATIVE-availability axis, not "the represented
interp refuses" -- the
represented path is LIVE). The producer's backing allocation is
`MappingRegionV1::try_new_anonymous(length, protection)` with an
attacker-influenceable `length: u64`.

## Per-allocation graceful failure is not the same as a capacity bound

`try_new_anonymous` (effect_v1.rs:1044) is fail-closed PER CALL:

```
if length == 0 { return Err(InvalidBounds); }
let length = usize::try_from(length).map_err(|_| AllocationFailed)?; // overflow
let mut bytes = Vec::new();
bytes.try_reserve_exact(length).map_err(|_| AllocationFailed)?;      // OOM -> Err, no abort
bytes.resize(length, 0);
```

`try_reserve_exact` is the good pattern: a `u64::MAX` length returns
`AllocationFailed`, never a process OOM-abort or panic. So there is no crash/DoS
on a single hostile length. **That is where the audit is tempting to stop -- and
it is not enough.** Graceful per-allocation failure bounds nothing in aggregate,
and it is not a policy limit: it fires only when the OS allocator is already
exhausted.

## The governor asymmetry is the finding

The sibling allocator declares real limits; the new one declares none.
`BufferAllocate` at `insert_buffer` (effect_v1.rs:1266) enforces THREE bounds:

```
total = live_buffer_capacity.checked_add(capacity) ...           // aggregate overflow
capacity == 0 || capacity > per_buffer_max_capacity              // 1 MiB, per buffer
                || total > invocation_max_live_capacity          // aggregate live cap
  => BufferLimit
```

and `per_buffer_max_capacity|1048576` is a row in the SEALED ABI catalog
(`effect_abi_v1.catalog`). `MappingAllocate` has no `per_mapping_max_capacity`,
no invocation-wide live-mapping accounting (`insert_mapping` tracks no aggregate),
and no catalog limit row. So the ABI contract for mappings is silent on size and
count where the buffer contract is explicit.

## Why NO DEFECT in D3, and why it is still a routed observation

Inert in D3, grounded on two independent facts:

1. **Unreachable from Ken source.** A grep of `crates/ken-elaborator/src/`
   (non-test) for the three ops is EMPTY; there is no perform-producer syntax,
   and the elaborator does not gate on `availability()`. The three ops appear
   only in the 8 diff files. So only the trusted Rust host API can drive
   `MappingAllocate` -- a checked program cannot emit it.
2. **Graceful per-allocation failure** (above).

It goes LIVE at the future checked / file-backed producer increment (the notify's
explicit out-of-scope tail): once a source program can emit `MappingAllocate` with
an attacker-influenced `length: u64`, it could request arbitrarily large OR
arbitrarily many anonymous mappings, capped only by the process allocator -- a
represented-side resource-exhaustion vector with no ABI-declared limit, unlike
buffers. The producer increment must consciously RULE it: declare a
`mapping.per_mapping_max_capacity` / invocation-wide mapping limit, or affirm that
unbounded-but-graceful is the intended contract (mappings are semantically meant
to be larger than buffers, so this may be by design -- but the absence should be a
deliberate ruling, not a silent gap).

**Outcome:** the ruling landed at ABI-S6 D4 (`cf894cdb5`, governed file-backed
Mapping acquire) as a declared limit: `mapping.per_mapping_max_capacity|1048576`
is now a row in `effect_abi_v1.catalog` and a field of `BufferLimitsV1` in
`effect_v1.rs`, bounded by `invocation_max_live_capacity` and refused with
`ResourceErrorV1::MappingLimit`. The paragraphs above describe the D3 state
the census was run against; the method is what carries.

## The method this pins

1. **When a deferred increment makes an ALLOCATION producer reachable (even only
   represented / only via the host API), find its nearest SIBLING allocator and
   diff their capacity/quota governors** -- per-unit cap, aggregate/invocation
   live cap, and any ABI-DECLARED limit row. A governor the sibling has and the
   new producer lacks is a candidate resource-exhaustion seam.
2. **Do not accept per-allocation graceful failure (`try_reserve_exact` ->
   typed error) as an aggregate or policy bound.** It prevents a crash on one
   hostile size; it does not bound total live resource or hostile count, and it
   only trips at true OS exhaustion.
3. **Ground the reachability both ways.** Absent a source producer + graceful
   failure => inert now => NO DEFECT. Present a future producer that makes the
   length source-controlled => name that increment and route the
   missing-governor ruling as its AC. Sibling of
   [[exhaustiveness-comes-from-an-unguarded-arm-not-from-the-match]] (name the
   producer-deliverable that must close a deferred seam) and of
   [[a-factor-out-into-a-shared-helper-can-strengthen-a-pre-existing-callers-precondition-so-diff-the-helpers-guard-set-against-each-old-inline-check]]
   (diff a new construct against its sibling, not just confirm it works in
   isolation).
