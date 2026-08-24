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

## Measured trigger layer (runtime-leader, `evt_52yj0cddb1ffc`)

Carried from the M6 respin's full default-stack parity run, as a fixed input for
M3 framing (not re-measured by the Steward): the non-eight control row
`rt_write_writable_stage` advances past its stale permanent closure-boundary
refusal to the exact new refusal

    Effect: seat Argument(1) of FsOpen needs ConstructorTag,
    which it cannot observe in CarriedWord

and the runtime implementer re-pointed ONLY that row to this layer with
retirement owner `RT-CARRIED-IH-DISPATCH-SITEOP` (M3). The ruled eight
`rt_cold_lowering_path_enumeration` rows are untouched and remain
`Disposition::Completes` (M6's AC-REENUM, Architect `evt_6eztb270x0067` part 3).

This measured refusal — a `CarriedWord` that cannot present the `ConstructorTag`
an `FsOpen` argument seat demands — is the concrete layer M3 resolves, and it
supersedes the vaguer closure-boundary characterization as M3's trigger. It is
consistent with the arity caution below (the `CarriedWord` / BoundaryCarrier
family, not the `Call` family). Recording the layer does NOT prejudge M3's
shape: whether M3 collapses to re-measure/re-point work or is a distinct build
remains the Architect's post-M6-landing assessment (`evt_4sp2xftkmc1mz`).

## Sequencing and caution

Draft, gated by the D0. The Architect's arity caution
(from [[RT-NATIVE-CARRIED-VALUE]]): M3 and M6 fail in OPPOSITE arity directions
(M3 one too many, M6 one too few) at DIFFERENT families (BoundaryCarrier vs
Call). They share the representation DECISION, not a seam or a defect — do not
collapse them because the numbers rhyme.
