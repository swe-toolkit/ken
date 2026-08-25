---
id: RT-CARRIED-IH-DISPATCH-SITEOP
title: "Track-1 consumer (M3) — a boundary-carried CarriedWord value cannot present the ConstructorTag an effect seat demands (effects.rs:548), so object emission refuses; give the carried value a finite compile-time constructor discriminant it can present, reusing M4's defunctionalization"
status: merged
owner: runtime
size: M
gate: none
depends_on: [RT-CHECKED-IH-FUNCTIONAL-REPRESENTATION]
blocks: []
github: null
origin: "Steward, 2026-08-22, filing Track-1 seat M3 of [[RT-NATIVE-CARRIED-VALUE]] from the Architect's frame (evt_9kat78d438cb). The SiteOperand PORT already landed ([[RT-SITEOP-CARRIED-WITNESS]] merged); the seam advanced to core.rs:2962, where a carried recursive hypothesis is currently rejected as 'an eliminated value, not a callable'. Steward-filed per COORDINATION section 2."
---

> # MERGED — squash `5fff430db` (2026-08-25), tree `8e72fd8f`
>
> Landed via the CI-red respin (parity fixture flipped: `rt_write_writable_stage`
> from the M3-retired `CARRIED_SITEOP_CONSTRUCTOR_TAG` refusal to `Completes`).
> Steward tree-audit: landed tree byte-identical to the approved candidate
> `2dd8fe7c`; all 10 cold-lowering path blobs match; zero `trusted_base()` delta.
> Route A proven (finite compile-time constructor discriminant, runtime carries
> the tag only; guarded finite dispatcher; one-sided — traps before host commit).
> The ken-CI auto-close did not flip this node; reconciled here.

> # RELEASED 2026-08-25 on the landed M4 close (`f02922221`). Frame:
> `docs/program/wp/RT-CARRIED-IH-DISPATCH-SITEOP.md`.
>
> M6 (D0) and M4 are both merged, so this consumer is unblocked. The Architect's
> post-landing shape ruling (`evt_3r7fhkcd3e`, resolving the deferred fork
> `evt_4sp2xftkmc1mz`) ruled M3 a DISTINCT BUILD — same defunctionalization
> family as M4, applied to DATA-constructor discrimination for host marshalling,
> not closure-code identity — and CORRECTED the scope: frame M3 against the
> effect-seat claim routine (`effects.rs:548`, the seam px8f:200 and px8ta HALF
> B:372 actually hit post-M4), NOT `reject_carried_residual_arguments`
> (core.rs:2935), which is [[RT-SITEOP-CARRIED-WITNESS]] D2's. The Objective
> below is the STALE filing-origin characterization, superseded by the frame; see
> the frame for the released deliverables and ACs.

> # Track-1 consumer (M3) — gated by the Track-1 D0 representation

## Objective (STALE — superseded by the frame; see the released banner)

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
