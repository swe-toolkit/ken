---
id: RT-DYNAMIC-CONSTRUCTOR-DISPATCH-PROVENANCE
title: "M3 successor (recut) — bind px8ta's causal residual (emit_carrier_dynamic_constructor's direct return_(-3) at StaticOriginId(34)) to one actual generated function/owner/SSA discriminator/compare/successor, then repair only the proven layer; the equality chain over discriminator 1 with a declared tag-1 alternative reaches its residual, which the hard-stop #3 research advisory places most plausibly in value-handle provenance, not integer equality"
status: merged
owner: runtime
size: M
gate: none
depends_on: [RT-CARRIED-IH-DISPATCH-SITEOP]
blocks: []
github: null
origin: "Steward, 2026-08-25, from the Architect hard-stop #3 ruling after the research advisory (evt_1vhmndq7fscd1, thr_305pn5gzx37h). RECUT of RT-EXITCODE-FAILURE-PAYLOAD-TRANSPORT, which the Architect FALSIFIED as a product object: the exact-Int carrier already admits every valid exit code and the two named process consumers are not missing transport; the causal defect is a dynamic-constructor dispatch residual, not an ExitCode payload gap. The three consecutive hard stops shared one predicate (a downstream semantic classification used as upstream producer/provenance authority), so this replaces rather than amends. Steward framing call per COORDINATION section 2."
---

> # MERGED 2026-08-25 — D1 landed at squash `c7541df21`
>
> D1 (branch-before-transfer, one active `HostResult` payload) merged: candidate
> `0295a7f71` squashed to `c7541df21` (7 files, +305/-88); Steward blob-audit
> confirms all 7 paths byte-identical to the candidate, no D0 remnants. px8ta
> HALF B advanced — the eager inactive-error residual is gone; execution now
> reaches `ConsoleIsTerminal`, then a distinct new obstruction `ControlledTrap
> RuntimeTrap(4)` at the explicit entry trap (grounded in the landed test's
> ignore reason). That is the next lane-1 object; it awaits the Architect's
> object-distinctness read before framing (do not pre-frame).
> [[RT-UNIT-FAILURE-STATUS-PROVENANCE]] (the separate `-3` reporter alias) is now
> dependency-unblocked; Steward owns lane order.

> # AMENDED 2026-08-25 — D0 done; D1 authorized (Architect hard-stop #1)
>
> D0 selected a SIXTH class beyond the frame's five-class closure: an inactive
> `HostResult` template eagerly materialized before the runtime sum choice
> (`aggregates.rs::emit_carrier_transfer` transfers ok then error BEFORE the
> discriminant). The Architect authorized D1 in place — branch-before-transfer,
> one active payload (evt_6h546ckyzsgtf, thr_1b16f1grspdq8). New-chain hard-stop
> count is 1; runtime held until the amendment released. Mechanism + controls are
> in the frame's "D1 — authorized mechanism" + AC-3.

> # M3 successor, recut — replaces the falsified ExitCode object
>
> The Architect falsified [[RT-EXITCODE-FAILURE-PAYLOAD-TRANSPORT]] as a product
> object (hard-stop #3 ruling evt_1vhmndq7fscd1). D1 on that WP is NOT resumed and
> its production refactor in `34ab178ac` is NOT shipped. This node is the
> replacement: an owner-bound probe of the causal dynamic-constructor dispatch
> residual. Full frame:
> `docs/program/wp/RT-DYNAMIC-CONSTRUCTOR-DISPATCH-PROVENANCE.md`.
>
> The `-3` reporter alias (an independently proven honesty defect) is tracked
> SEPARATELY as [[RT-UNIT-FAILURE-STATUS-PROVENANCE]] and must NOT be folded in.
> [[RT-RETAINED-UNIT-CALL-TARGET-DERIVATION]] remains distinct with no dependency
> or sequencing change. The Architect's symptom inventory through entry 3 is on
> `architect/rt-exit-inventory @ 967618b3f`.

## Objective (Architect ruling evt_1vhmndq7fscd1)

Bind px8ta's causal residual — the direct `return_(-3)` at the bottom of
`emit_carrier_dynamic_constructor`'s alternative chain — to ONE actual generated
function, owner, SSA discriminator, compare instruction, and taken successor,
then repair only the proven layer. No production mechanism is authorized before
the probe ladder selects a class.

Durable facts: the px8ta path carries `Lowered::DynamicConstructor` at
`StaticOriginId(34)`; the reported discriminator is `1`; the emitted alternative
list contains tag `1` (`ResourceError::Closed`); the equality chain nonetheless
reaches its residual. Mutating only the residual `-3 -> 73` makes the process
status exactly `73`, forwarded unchanged by `call_declared_unit_target` — the
status bypasses the result slot, carrier decode, root result arms, and both
process-exit consumers. But the same-site `1`/tag-1 contradiction is NOT yet
proven at one emitted function: `StaticOriginId(34)` is plan-local, markers 100
and 106 are duplicate `ResourceError` sites with identical inventories, and the
high/low observations came from separate builds. The advisory
(evt_5yxw7qypv4w4q) places the plausible class in value-handle provenance
(Cranelift `Value` is a bare per-function `u32` index with no owner in its type),
not integer equality. D0 selects the actual class before any repair.

## Anchor

`34ab178acc65b4f6d165e1b2d40f5809d1c475d2` on
`wp/RT-EXITCODE-FAILURE-PAYLOAD-TRANSPORT` is the READ-ONLY load-bearing probe
checkpoint. Its consumer-refactor production delta is the falsified ExitCode
object and is NOT the base or candidate. Base the successor branch on current
`main`.

## Sequencing

First lane-1 object after the ExitCode recut. [[RT-RETAINED-UNIT-CALL-TARGET-DERIVATION]]
remains distinct (draft, unchanged). [[RT-UNIT-FAILURE-STATUS-PROVENANCE]] is
sequenced AFTER this causal-dispatch object and does not block its D0-P0/P1.
Steward owns lane order.
