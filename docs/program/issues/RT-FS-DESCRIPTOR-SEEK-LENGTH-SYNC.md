---
id: RT-FS-DESCRIPTOR-SEEK-LENGTH-SYNC
title: "ABI-S1 merged host implementations of FsSeek, FsSetLength and FsSync, but no Ken source can reach them and all three are RepresentedUnavailable, so a Ken program cannot truncate a file or make a write durable. Rule from an AC-0 whether each operation gains a checked surface and native parity or leaves the catalog, as positional reads and writes may already subsume seek, and land the ruled result"
status: ready
owner: language
size: L
tier: T1
gate: architect
depends_on: []
blocks: [LINUX-ABI-EXIT-SERVICE-SUPERVISOR]
github: null
origin: "Steward 2026-11-22 on operator direction (\"Create WPs and update the campaign docs as necessary to capture the gaps you have identified\"). Program §6 requires every operation NativeTested or probed; these three are unavailable only because nobody surfaced or promoted them. L1 Linux ABI program, Steward order. Released on the Language ring (L1b, operator 2026-11-22: \"Approved. Use the language ring.\") after ABI-S2, anchors re-checked on ABI-S2 98242db31."
---

# Descriptor seek, length and sync

## Objective

Every one of `FsSeek`, `FsSetLength` and `FsSync` ends in one of two
states. Either a checked Ken wrapper reaches it and it is `NativeTested`
with interpreter parity, or it has left the operation catalog with its
host code, wire request and planner arm.

## Settled inputs (ABI-S2 tree `98242db31` over main `3542c4c60`)

- **Host.** `ken-host/src/lib.rs` implements `resource_seek_v1` (`:1061`),
  `resource_set_length_v1` (`:1074`) and `resource_sync_v1` (`:1087`,
  full and data sync). `effect_v1.rs` dispatches their canonical requests
  (from ~`:3648`) and marks all three `RepresentedUnavailable` (`:226` to
  `:228`).
- **No surface.** The prelude has no constructor for any of the three.
  The open-handle operations that do run (`FsHandleMetadata`, `FsReadAt`,
  `FsWriteAt`) enter through private prelude constructors
  (`PrivateFsReadAt`, ~`prelude.rs:2591`) with package wrappers such as
  `writeAt` in `Capability.System.IO` (spec 38 ~`:522`). `ken-interp`
  does not decode any of the three.
- **Seek and positional I/O.** Spec 38 ~`:588` says sequential reads,
  writes and seek-like state are derived code that threads
  `fileOffset + transferred`, and that PX8 adds no seek.
  `FileOperation.OpSeek`, `OpSetLength` and `OpSync` already exist as
  error-context cases (spec 38 ~`:1440`).
- **Native.** The planner names all three among the operations refused
  before any seat is derived (`static_transition/effects.rs` ~`:441`, ~`:697`).

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

The three operations' surface, interpreter decoding, native planning and
lowering, `ken-verify` evidence, and spec 38. Alternatively, their
retirement from `effect_abi_v1.catalog`, `effect_v1.rs`, the wire format
and the planner. No change to `FsReadAt` or `FsWriteAt`, to the
handle-open modes, or to the FS root confinement.

## Acceptance

- **AC-0 (Architect, before code).** For each operation, rule surface or
  retire against PRINCIPLES' subsume-don't-proliferate item and spec 38
  ~`:588`. For each surfaced operation, rule its package home, the
  handle mode it requires, and its `FileError` context. A retired
  operation's `FileOperation` case follows the same ruling. Sweep every
  consumer of the operation identity across `crates/`, `conformance/`
  and `spec/`.
- **AC-1 (surfaced).** One differential row per operation, interpreter
  against native, confirmed by `confirm_native_tested_transition`, each
  with a negative control that perturbs the evidence. Set-length row:
  truncating a 10-byte file to 4 makes a following `readAt` of 10 return
  4 bytes, and extending it to 12 reads 12 with zero fill. Seek row, if
  surfaced: the position it returns is the one the next relative read
  uses.
- **AC-1 (retired).** The operation identity is absent from the catalog
  and from `HostOpV1`, and the build passes on every consumer AC-0
  swept.
- **AC-2.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- A surfaced operation needs a new authority class or a TCB change.
- Sync durability cannot be observed differently from a no-op on the
  CI filesystem. Report that; do not assert it from the call returning.

## Inventory

1. A statically folded guard that selects Ret left the Vis case's
   planned join undisposed. The native join closeout's
   recursive-predecessor union counted a continuation call whose
   producer is in excluded code: it is keyed on planner call membership
   without call liveness (Architect `evt_6k4h2m6hc5249`).
2. The detached-result seat treated the same dead call as owed by the
   unit result, and refused because the result is not its producer
   constructor. It is keyed on planner result-edge membership minus
   emitted, discharged and claimed, with no statically-unselected form.

§1b at entry 2 (Architect `evt_45863hkv60etq`): shared predicate, the
call population lacks a statically-unselected disposition. Steward scope
`evt_6gyt3n9jqxe08`: this WP proceeds with the checked guard, the -1
parity row and an interpreter-only overflow row. The overflow literal's
native build is a named build-time PlannerInvariant refusal, owned by
`RT-STATIC-UNSELECTED-CALL-DISPOSITION`, which carries the native row
and the join fix `8cee29d96`.
