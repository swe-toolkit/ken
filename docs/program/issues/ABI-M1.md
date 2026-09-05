---
id: ABI-M1
title: "manifest v2 — family-scoped, versioned, generated from family schemas"
status: active
owner: runtime
size: L
gate: none
depends_on: [ABI-R3]
blocks: [ABI-M2, ABI-S4, PX10, PX11]
github: null
origin: docs/program/10-linux-abi-completion.md §4 (the ABI-completion program); node filed by the Steward 2026-07-25 on the operator's directive to frame the remaining program. Agents cannot create tracked work (COORDINATION §2).
---

> # FINAL INCREMENT AS BUILT: target-identity D1 and D2/D3 complete
>
> D0 + the D1 structural half merged earlier at `4342ca4cf` (PR #2811, exact
> `00c65711869108c6c43f7d567b32eedd8011516c`): sealed `AbiFamily`, generated
> by-variant-path join, fail-closed family partition, schema v2, and composed
> projection hashes. The final increment expands target identity from two to
> sixteen layout facts, adds explicit target architecture and endianness, and
> cross-checks every fact against the C probe. Exact target-identity membership
> prevents a broad `C_` prefix from silently absorbing future record-layout
> facts; zero or multiple family claims both abort generation.
>
> D2 independently reconstructs every projection and the top manifest hash,
> proves one-family mutation locality, pins the bounded target-identity vector,
> checks whole-population C-probe disagreement, and preserves the runtime hash
> binding plus ken-verify partition. D3 records the Architect/Steward AC-6
> ruling: unsupported targets fail closed at the native-only guard and produce
> no native manifest. An observable cross-target unavailable manifest remains
> deferred under Program 10 §3/§8 unless a real cross-compilation consumer later
> triggers an operator option-(B) decision. No cross-target generation is added.
>
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
