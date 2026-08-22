---
id: RT-CARRIED-IH-DISPATCH-SITEOP
title: "Track-1 consumer (M3) — the live successor at core.rs:2962 reject_carried_residual_arguments becomes a defunctionalized dispatch once the checked-IH representation lands"
status: draft
owner: runtime
size: M
gate: none
depends_on: [RT-CHECKED-IH-FUNCTIONAL-REPRESENTATION]
blocks: []
github: null
origin: "Steward, 2026-08-22, filing Track-1 seat M3 of [[RT-NATIVE-CARRIED-VALUE]] from the Architect's frame (evt_9kat78d438cb). The SiteOperand PORT already landed ([[RT-SITEOP-CARRIED-WITNESS]] merged); the seam advanced to core.rs:2962, where a carried recursive hypothesis is currently rejected as 'an eliminated value, not a callable'. Steward-filed per COORDINATION section 2."
---

> # Track-1 consumer (M3) — gated by the Track-1 D0 representation

## Objective

At `core.rs:2962` (`reject_carried_residual_arguments`) a carried recursive
hypothesis is refused as "an eliminated value, not a callable." Once the
defunctionalized checked-IH representation lands (the Track-1 D0), this seat
becomes a defunctionalized dispatch (code id + env Record + finite static apply)
rather than a refusal.

## Sequencing and caution

Draft, gated by the D0. The Architect's arity caution
(from [[RT-NATIVE-CARRIED-VALUE]]): M3 and M6 fail in OPPOSITE arity directions
(M3 one too many, M6 one too few) at DIFFERENT families (BoundaryCarrier vs
Call). They share the representation DECISION, not a seam or a defect — do not
collapse them because the numbers rhyme.
