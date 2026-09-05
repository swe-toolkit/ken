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

> # PARTIAL-MERGE 2026-08-23: D0+D1-structural-half LANDED; target-identity D1 HELD
>
> D0 + the D1 structural half MERGED at `4342ca4cf` (PR #2811, exact
> `00c65711869108c6c43f7d567b32eedd8011516c`): sealed `AbiFamily` enum,
> family-schema by-variant-path generation, fail-closed-on-unassigned
> partition, v2 composed-projection manifest hash (SCHEMA_VERSION 1->2), the
> two-tier enforcement. QA (evt_5k6ac29bq478g) + Architect (evt_1pkmzgx536cca)
> APPROVE on the exact SHA; Decision dec_42hpwdv09r5ts. The remaining D1
> (target-identity family expansion — arch/endianness/scalar-width facts) stays
> OPEN on the branch, held for the runtime implementer's return from the M6
> predecessor; that work MOVES the manifest hash (expected churn, not a
> regression). Fold the symmetric-closure guard (error on a multi-prefix fact
> match, mirroring the fail-closed-on-unassigned assert; Architect non-blocking
> note) into that work. Node stays `ready` for the remaining D1.
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
