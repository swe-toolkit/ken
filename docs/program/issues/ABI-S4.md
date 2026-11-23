---
id: ABI-S4
title: "A Ken program's file metadata is MkFileMetadata Int FileKind, so it cannot read a modification time, a link count or a mode, and nothing tells it which fields the kernel actually supplied. Rule from an AC-0 the statx-shaped field set and how an unsupplied field is represented, then carry it through both lanes so a missing field is never a plausible zero"
status: active
owner: language
size: M
tier: T1
gate: architect
depends_on: [ABI-M1]
blocks: [LINUX-ABI-EXIT-SERVICE-SUPERVISOR]
github: null
origin: "docs/program/10-linux-abi-completion.md §4 Track S. Node filed by the Steward 2026-07-25. Framed 2026-11-23 as the second L1b node on the Language ring (operator 2026-11-22: \"Approved. Use the language ring.\"). L1 Linux ABI program, Steward order."
---

# statx-shaped metadata with field availability

## Objective

`file_metadata` and `resourceMetadata` return a record whose fields come
from one `statx` call. Each field the kernel did not fill is observably
absent. It is never zero, and never copied from another field. Both lanes
agree.

## Settled inputs (main `8c6d5b37c`; anchors re-checked at release)

- **Surface.** `data FileMetadata = MkFileMetadata Int FileKind`
  (`prelude.rs` ~`:1068`) carries size and kind only. `file_metadata`
  (~`:2788`) and `resourceMetadata` (~`:2904`) return it. Spec 38
  ~`:1420` names the carrier exactly, and native replies "select their
  checked constructors".
- **Host.** `ken_host::metadata` and `resource_metadata` (`lib.rs`
  ~`:320`, ~`:351`) use std `metadata()`, which is fstat with no mask.
  The host `Metadata` already holds mode and device and inode identity
  (`:674`). The ABI host drops both when it builds `FileMetadataV1 { size,
  kind }` (`abi_v1.rs` ~`:655`, `effect_v1.rs` ~`:4684`).
- **Wire and evidence.** Reply tag 6 is size then kind
  (`effect_wire.rs` ~`:409`, ~`:1118`). `ken-verify` compares
  `FileMetadataSize` and `FileMetadataKind` (`canonical.rs` ~`:101`).
- **Interpreter.** `HostFileMetadata` (`eval.rs` ~`:3866`), on the real
  and virtual hosts (~`:4570`, ~`:5338`).
- **Native.** The metadata reply is built in `lowering/effects.rs`
  (~`:4957`, the `file_metadata` process symbol).
- **Layout source.** ABI-M1 (merged) generates per-family record layouts.
  A new reply record comes from that generator and is not restated by
  hand.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

The metadata carrier, both metadata operations, the host `statx` call,
the reply record and wire tag, the interpreter hosts, native reply
construction, `ken-verify` evidence and comparators, and spec 38. No
change to path resolution, to the root confinement, or to the open mode
that metadata uses.

## Acceptance

- **AC-0 (Architect, with the Spec Leader, before code).** Rule:
  - the field set, starting from size, kind, mode, link count, device and
    inode identity, and the modification, status-change, access and birth
    times;
  - how an unsupplied field appears in Ken: a per-field optional, or a
    mask beside plain fields;
  - whether `FileMetadata` grows in place or a new carrier replaces it;
  - the time representation;
  - whether native lowering changes. If it does, name the lines, because
    the PX11-NATIVE chain also edits `lowering/effects.rs`.

  Sweep every consumer of `FileMetadata` and `MkFileMetadata` across
  `crates/`, `catalog/`, `conformance/` and `spec/`.
- **AC-1.** A differential row, interpreter against native, confirmed by
  `confirm_native_tested_transition`, reads a regular file and a
  directory and observes the ruled fields. A row forces one field absent
  from the host mask and observes it absent, not zero, on both lanes. The
  negative control changes the absent field to zero, and confirmation
  fails.
- **AC-2.** A modification-time row writes a file, reads its time, writes
  again after a deadline, and observes a strictly later time. A host that
  reports a constant fails it.
- **AC-3.** Spec 38 states the carrier and the absence rule as a Spec
  co-turn. Full-workspace CI is green on the exact candidate.

## Stop conditions

- The ruled carrier needs a new authority class or a TCB change.
- `statx` is unavailable on the CI kernel. Report that; do not fall back
  to fstat while claiming a mask.

Checks: 4 fired. I read the surface carrier, both host metadata bodies,
the ABI host's dropped fields, the reply tag and the native symbol on
`8c6d5b37c`. 2 fired. The property is that an unsupplied field is never a
value. Its fan-in is the host mask, the reply record, both interpreter
hosts and native construction, and AC-1's forced-absent row reaches every
one. 8 fired. The zero-substitution control fails a binding that drops
the mask, and AC-2's later time fails a constant.
