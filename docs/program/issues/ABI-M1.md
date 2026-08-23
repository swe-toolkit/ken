---
id: ABI-M1
title: "manifest v2 — family-scoped, versioned, generated from family schemas"
status: ready
owner: runtime
size: L
gate: none
depends_on: [ABI-R3]
blocks: [ABI-M2, ABI-S4, PX10, PX11]
github: null
origin: docs/program/10-linux-abi-completion.md §4 (the ABI-completion program); node filed by the Steward 2026-07-25 on the operator's directive to frame the remaining program. Agents cannot create tracked work (COORDINATION §2).
---

> ## Authority: the WP frame — `docs/program/wp/abi-m1-family-scoped-manifest.md`
>
> Frame authored + released by the Steward 2026-08-23 (fixed inputs at
> `origin/main a12f74158`). It carries the objective, front-loaded design
> judgment (structure-derived family schemas, reuse of the PX2 generator + the
> ABI-R3 derive discipline), the D0 representation probe with an Architect
> return-fork, deliverables D0-D3, acceptance criteria AC-1..AC-7 with controls,
> the zero-TCB note, the contention check, and the named Foundation consult.
> Read the frame, not this node, for the build.

## Objective

A **family-scoped, versioned manifest generated from family schemas** rather
than one growing handwritten list: target identity (arch, pointer width,
endianness, C scalar widths/alignments), constants and record layouts per
enabled family, facility ABI versions, and canonical hashes per family
projection.

⚠ **Runtime + Foundation collaboration.** Recorded with `owner: runtime` because
a WP is owned by a single team (§2); Foundation participation is required and
must be named in the WP frame.

## Explicitly OUT of scope (§3 deferral)

⛔ Cross-target generation · signed or content-addressed manifests · CI
native-builder matrices. **Native-target only.**
