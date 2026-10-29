---
id: ABI-S6-D5B-COW-DISCHARGE
title: "ABI-S6's last unmet criterion is seed case 1: a process-local write to a file-backed mapping must not reach the file. No fixture writes through a file-backed mapping, and on main no open mode can acquire a writable one, because file-mapping admission demands the destination's write right of the source handle. Admit the source on READ, run the case on both engines with a write-through control, lift the seed markers, and add the two missing typed-refusal witnesses"
status: ready
owner: runtime
size: S
tier: T1
gate: architect
depends_on: [RT-D5B-DEAD-FILE-MMAP-ROUTE]
blocks: [ABI-S6]
github: null
origin: "Architect AC-0 for RT-D5B-DEAD-FILE-MMAP-ROUTE, evt_2rm267s3ye5sa §2-§3: ABI-S6 is not met on main because seed case 1 (D5b's MAP_PRIVATE discharge) is executed nowhere, and AC-REFUSAL-TYPED lacks two witnesses. Amended in place by this WP's AC-0, evt_4jn3kvg87gqnp: the no-runtime-change premise is refuted, and the repair is the already-ruled D4 source-admission correction (evt_cf7sk8pss0dk). L1 objective: the Linux ABI program, ABI-S6. Steward-filed per COORDINATION section 2."
---

# A write to a file-backed mapping never reaches the file

## Objective

On both engines, a program that writes through a `FileBacked` mapping
sees its write inside the mapping. After the bracket settles, it reads
the original bytes back through the ordinary file API. A write-through
implementation fails this. With the two missing refusal witnesses added,
every ABI-S6 acceptance criterion has a witness, and ABI-S6 closes.

## Settled inputs (main `37f38afc7`, AC-0 `evt_4jn3kvg87gqnp`)

- **The case.** `conformance/surface/ffi-io/seed-mapping.md`, case
  `mapping-file-writes-are-copy-on-write` (`:40`), spec `38 §1.9` and
  `§1.3`. It is marked RED and `BLOCKED-ON-ABI-S6-D5b` (`:50`, `:53`).
  Cases 2 and 3 still carry `BLOCKED-ON-ABI-S6-D5a` (`:77`, `:104`),
  although tests witness them (Architect `evt_2rm267s3ye5sa` §2).
- **The rights wall (AC-0 §1, both engines).** No surface open mode
  mints READ|WRITE: `ResourceRead` gives READ, and `ResourceWriteCreate`
  gives WRITE|CREATE. A `ReadWrite` file mapping over a `ResourceRead`
  handle is refused with `RightNotHeld{required: 3, held: 1}`, exit 91.
  The read-only control acquires, exit 60.
- **The cause is one line.** `effect_v1.rs:3231`, in the
  `MappingAcquireFile` arm, admits the SOURCE handle with the
  DESTINATION's `protection.rights()`. Under 38 §1.9 file mappings are
  private, so the source's governing right is READ for both protections.
  This repair was ruled before (`evt_cf7sk8pss0dk`, recorded in
  `ABI-S6.md`) and lived only in WIP `37390dfcf`, which never landed.
- **Nothing else writes the file.** File-backed storage is
  `InProcess(Vec)` only (`try_new_file_backed`, `:1144`), no handle is
  retained after copy-in, and ken-host has no `msync` or `MAP_SHARED`.
  The minted Mapping's own rights stay protection-derived (`:1459`).
- **The only pin of the old rule** is D4's first assertion
  (`effect_v1.rs:6498`, `required: READ|WRITE, held: READ`), in
  `abi_s6_d4_file_acquire_enforces_rights_exact_length_lineage_and_capacity`
  (`:6448`). It reds under R1.
- **Out of scope.** Spec 38's question about external writes after
  acquisition stays with the Spec Leader. This WP pins only that
  process-local writes do not reach the file.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

One branch, from the kickoff base. The Spec co-turn owns the seed text,
and CV votes on `conformance/`.

- **R1.** Change exactly `effect_v1.rs:3231` to admit the source with
  `crate::RightSet::READ`. Do not change `MappingProtectionV1::rights`
  (`:1086`), `insert_mapping`, view admission, `mapWrite`, the wire, the
  prelude or the ABI.
- **R2.** Replace D4's first assertion with a non-degenerate pair:
  - a READ-only source with `Writable` protection succeeds, and is
    released inside the test;
  - a source holding WRITE|CREATE without READ refuses with exact
    `RightNotHeld{required: READ, held: WRITE|CREATE}` and
    `read_calls == 0`.

  A full-rights source may stay as a neighbour, but not as the only
  success.
- **R3.** A natural-site mutation, `UseProtectionRightsForSource`,
  restores `protection.rights()` at `:3231`. On the AC-1 witness it
  restores exactly the `required: 3, held: 1` refusal.
- **The AC-1 fixture and native test**, opening the file with
  `ResourceRead`.
- **The three AC-3 tests** as written in AC-0 §5, plus the extracted
  `fn map_anonymous_error_v1(IoErrorIdentityV1) -> SemanticErrorV1`
  from `ProcessHost`'s `map_err` composition (`abi_v1.rs:735-746`), unit
  tested directly. This closes the composition residual AC-0 §5 names.
- **Seed text (Spec co-turn).** Case 1's "given" says the file is opened
  with `ResourceRead`. The markers on cases 1 to 3 are lifted.

## Acceptance

- **AC-1.** Both engines run the program in parity:
  - open a file with known bytes with `ResourceRead`, and acquire
    `FileBacked` over it with `ReadWrite`;
  - `mapWrite` different bytes in range;
  - `mapBytes` reads the written bytes;
  - after the bracket settles, the §1.3 file read returns the original
    bytes.

  The two engines' traces and exit statuses are equal.
- **AC-2 (mutation, QA).** A write-through mutant writes the region back
  to the file on `mapWrite` or on release, for example by reopening the
  backing file read-write at host level. It must be measured to change
  the bytes on disk, and that change must redden AC-1's file-read
  assertion on both engines. A mutant that only errors, at acquire or at
  write on the read-only fd, does not satisfy AC-2.
- **AC-3.** Each refusal has its own exact-variant assertion:
  - a malformed (future-generation) Mapping token gives
    `MalformedResource`, on the reply and in the trace;
  - a backend mapping failure gives exactly `AllocationFailed` or
    `Io(Other(5))`, and mints no capacity;
  - `map_anonymous_error_v1` maps ENOMEM to `AllocationFailed` and any
    other identity to `Io`.
- **AC-4.** Cases 1 to 3 of `seed-mapping.md` name their witnessing
  tests and carry no `BLOCKED-ON` marker.
- **AC-5.** R2's success and refusal rows pass, and R3's mutant
  restores the exact `required: 3, held: 1` refusal on AC-1.

## Stop conditions

- A change beyond R1 is needed to acquire the mapping or to keep writes
  off the file.
- A new open mode, right bit, `MAP_SHARED`, weakened mapping rights or
  test-only handle injection.
- A new carrier, Need, kernel change or trust growth, which is an
  operator stop.

Checks: 4 fired: the AC-1 obligation, a `FileBacked ReadWrite` acquire
from a surface handle, is measured unconstructible on main and
constructible under R1 (AC-0 §1, §3). 8 fired: on the read-only fd an
erroring write-through mutant reddens AC-1 without writing the file, so
AC-2 requires the bytes on disk to change. R2's pair has a READ-only
success and a READ-less refusal, so the old READ|WRITE rule fails one of
them.
