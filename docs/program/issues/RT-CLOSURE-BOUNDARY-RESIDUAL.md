---
id: RT-CLOSURE-BOUNDARY-RESIDUAL
title: "Track-1 consumer (M4) — the residual checked-closure population (rt_parity_native:825 + px8f_buffer_native:203 + px8f_write_partition:354), resolved by applying the merged CROSSING-ELIMINATE defunctionalization discipline"
status: merged
owner: runtime
size: M
gate: none
depends_on: [RT-CHECKED-IH-FUNCTIONAL-REPRESENTATION]
blocks: []
github: null
origin: "Steward, 2026-08-22, filing Track-1 seat M4 of [[RT-NATIVE-CARRIED-VALUE]] from the Architect's frame (evt_9kat78d438cb). [[RT-CLOSURE-BOUNDARY-LANE]] and [[RT-CLOSURE-CROSSING-ELIMINATE]] are merged; this is the residual checked-closure population at rt_parity_native:825. Steward-filed per COORDINATION section 2."
---

> # RELEASED 2026-08-24 on the landed M6 close (`011bf2a95`). Frame:
> `docs/program/wp/RT-CLOSURE-BOUNDARY-RESIDUAL.md`.
>
> M6 (`RT-CHECKED-IH-FUNCTIONAL-REPRESENTATION`) merged `79d64a967` / closed
> `011bf2a95`, so this consumer is unblocked. The Architect's post-M6-landing
> shape assessment (`evt_1vcwzkd3g0s1r`) ruled M4 a DISTINCT BUILD (the thinnest
> class — a proven discipline at a new seam), not a collapse-to-re-point.
>
> The population citations below (the title's `rt_parity_native:825` +
> `px8f_write_partition:354`, and the 2026-08-22 Population section) are STALE
> and SUPERSEDED by the frame's entry census (deliverable 1). Every `#[ignore]`
> label in scope predates M6, and M6 changed the escaping functional-IH
> representation, so the post-M6 boundary-refusal population is re-measured at
> `011bf2a95`, not inherited. Verified at `011bf2a95`: only
> `px8f_buffer_native.rs:203` carries the residual label; `rt_parity_native:825`
> is a `BufferFreeze` comment (not a refusal) and `px8f_write_partition:354` has
> no ignore row; six rows carry the origin `RT-CLOSURE-BOUNDARY-LANE` label whose
> per-row disposition the census settles. The seam itself is verified:
> `boundary.rs:1044`, the `Lowered::Closure`/`DeclarationClosure` arm, untouched
> by M6's delta.

> # Track-1 consumer (M4) — gated by the Track-1 D0 representation

## Objective

The residual checked-closure population — the checked continuation
(`lambda response. rec (k response)`) crossing the effect-seat boundary with no
first-class native representation — is resolved by applying the same
defunctionalization discipline the merged [[RT-CLOSURE-CROSSING-ELIMINATE]]
(PR #2327) proved for the source-authored closure population. No new invention.
Draft, gated by the Track-1 D0.

## Population (measured 2026-08-22, broadened per Architect evt_399x8k6mxacwa)

Not just `rt_parity_native.rs:825`. Sweep-first measurement on
[[RT-NATIVE-TRACK0-REARM]] found the checked-write full-program rows
`px8f_buffer_native.rs:203` and `px8f_write_partition.rs:354` also refuse at the
closure-boundary seam (`boundary.rs:1044`, "a closure cannot cross the
boundary") once M1/M2 cleared — they carry the checked continuation. This node
is the LIVE owner those rows' `#[ignore]` labels name (RT-CLOSURE-BOUNDARY-LANE,
where the seam was first named, is merged). The Architect ruled this is the SAME
Track-1 defunctionalization decision, not a distinct member — so these rows'
first-order PX8 witnesses (Wrote/ReadSome) are Track-1-gated, not Track-0.

