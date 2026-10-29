---
id: RT-CERTIFIER-CALLEE-SLOT-RETENTION
title: "The finished-CLIF Result certifier assumes a call that receives a slot address does not publish it into memory the caller later reloads and writes through, and consults retention only for later calls, not for later unknown-destination stores. Decide and certify the property that makes a Result producer non-retaining for its out-parameter, then reinstate retention for unknown stores after every other retaining call"
status: ready
owner: runtime
size: M
tier: T1
gate: architect
depends_on: [RT-CERTIFIER-ESCAPED-SLOT-ZERO-ARGUMENT-CALL]
blocks: []
github: null
origin: "Architect ruling evt_15rkc0pjxhm1z on the RT-CERTIFIER-ESCAPED-SLOT-ZERO-ARGUMENT-CALL double-cause stop (evt_3ts5d91sq7amt): gap (c) withdrawn from that WP and filed as its own design node, gate architect, before ABI-S6-HS18-MAIN-BASED-CLOSURE increment C. Steward-filed per COORDINATION section 2."
---

# A Result producer is certified not to retain its out-parameter

## Objective

After a call that received a slot's address, an unknown-destination store
clobbers the slot, unless the call is a Result producer certified not to
retain that address. Ordinary native Results stay accepted.

## Fixed inputs (Architect `evt_15rkc0pjxhm1z`)

All in `crates/ken-runtime/src/cranelift_backend/lowering/units.rs`.

- **The gap.** `unknown_write_may_alias` checks `escaped` and the slot walk,
  never `call_follows_retention`. After `call f(stack_addr A)` returns and A
  is re-stored, `p = load(heap); store 9, p` keeps `Exact`, although `f` may
  have published A's address.
- **Why plain retention refuses real plans.** In `SP_A_FREEZE`
  (`rt_span_prov_native`), producer `inst2935` receives `&ss36` and writes
  the Result there. A pointer field is reloaded (`v1924 = stack_load ss36`),
  and `inst3010` stores through it into the Result's heap payload. With
  retention consulted for unknown stores, every such store clobbers the
  Result: 34 similar stores in that plan. Restricting rule (ii) to opaque
  addresses does not help, because `v1924` is itself a load.
- **The predecessor's residual.** RT-CERTIFIER-ESCAPED-SLOT-ZERO-ARGUMENT-CALL
  states this gap. Its rule (i') rests on the same assumption for slot
  contents written by a callee.

Treat anchors as perishable. If a fixed input is false on the landed base,
stop and report the mismatch; do not build around it.

## Deliverable

1. **AC-0 first (D0, Architect).** It rules which property makes a Result
   producer non-retaining for its out-parameter, and how that property is
   certified on the callee's finished body. An example is that the
   out-parameter's address is never stored and never passed on.
2. **The repair, as ruled.**
   - Retention exempts certified producers.
   - `location_leaked_before` is reinstated in `unknown_write_may_alias` for
     the remaining retaining calls.

## Acceptance

- **AC-1.** P4 from `evt_2tsqhgywfxdb` gives `{Unknown}`, and is shown red
  first on the cut base. P4 is `a = stack_addr A; call f(a); stack_store 7,
  A; p = load(iconst 0x1000); store 9, p`, where `f` is not a certified
  producer.
- **AC-2.** m3, which drops the predicate from `unknown_write_may_alias`,
  reddens P4. A mutant that certifies every call as non-retaining also
  reddens P4. Each is restored byte-identically.
- **AC-3.** Both `SP_A_FREEZE` rows in `rt_span_prov_native` pass, as does
  the rest of the named native population of the predecessor's AC-3. A
  named plan that flips to refused is a stop, not a pin update.

## Stop conditions

- The property cannot be certified on the callee's finished body without a
  change outside `ken-runtime`.
- A named plan flips from accepted to refused. Hard-stop with the plan and
  its trace, as in `evt_1q2wcn4d9feze`.

Checks: 8 fired: P4 alone passes under a rule that refuses every unknown
store after any call, so AC-3's `SP_A_FREEZE` rows are kept as the
discriminating measurement, and the always-certified mutant is in AC-2. 2
fired: the producer call `inst2935` and its 34 payload stores are taken
from the implementer's trace (`evt_1q2wcn4d9feze`), as cited by the
Architect, not re-derived.
