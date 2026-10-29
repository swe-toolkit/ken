---
id: ABI-S6-D5B-COW-DISCHARGE
title: "ABI-S6's last unmet criterion is seed case 1: a process-local write to a file-backed mapping must not reach the file. No fixture writes through a file-backed mapping, so a write-through implementation passes every current test. Run the case on both engines with a write-through control, lift the seed markers, and add the two missing typed-refusal witnesses"
status: ready
owner: runtime
size: M
tier: T1
gate: architect
depends_on: [RT-D5B-DEAD-FILE-MMAP-ROUTE]
blocks: [ABI-S6]
github: null
origin: "Architect AC-0 for RT-D5B-DEAD-FILE-MMAP-ROUTE, evt_2rm267s3ye5sa §2-§3: ABI-S6 is not met on main because seed case 1 (D5b's MAP_PRIVATE discharge) is executed nowhere, and AC-REFUSAL-TYPED lacks two witnesses. L1 objective: the Linux ABI program, ABI-S6. Steward-filed per COORDINATION section 2."
---

# A write to a file-backed mapping never reaches the file

## Objective

On both engines, a program that writes through a `FileBacked` mapping
sees its write inside the mapping. After the bracket settles, it reads
the original bytes back through the ordinary file API. A write-through
implementation fails this. With the two missing refusal witnesses added,
every ABI-S6 acceptance criterion has a witness, and ABI-S6 closes.

## Settled inputs (main after the route deletion)

- **The case.** `conformance/surface/ffi-io/seed-mapping.md`, case
  `mapping-file-writes-are-copy-on-write` (`:40`), spec `38 §1.9` and
  `§1.3`. It is marked RED and `BLOCKED-ON-ABI-S6-D5b` (`:50`, `:53`).
  Cases 2 and 3 still carry `BLOCKED-ON-ABI-S6-D5a` (`:77`, `:104`),
  although tests witness them (Architect `evt_2rm267s3ye5sa` §2).
- **Why nothing catches write-through today.** The only file-backed
  fixture, `abi_s6_mapping_file_backed.ken`, never calls `mapWrite` or
  uses `ReadWrite`. `abi_s6_mapping_surface_native.rs:383` says that
  MAP_PRIVATE isolation is not asserted.
- **The enablers exist.**
  - `MappingAcquireFile` is NativeTested (flip `f84e35e15`).
  - `mapWrite` is prelude surface, and the anonymous-mapping rows of
    `abi_s6_mapping_surface_native.rs` run it natively.
  - File copy-in writes only owned `InProcess` bytes, and release drops
    them (`release_native`, HS-1 `evt_7wme5eytyz2bk`). So the Architect
    expects no runtime code to change. That is a premise for AC-0 to
    probe, not a measurement.
- **The rights premise.** A `ReadWrite` mapping over a read-only source
  handle is refused with `RightNotHeld`, exit 91 (Runtime QA probe,
  `evt_3bhx30w0hjj72`). The fixture therefore opens its file with a
  handle that holds the write right.
- **Out of scope.** Spec 38's question about external writes after
  acquisition stays with the Spec Leader. This WP pins only that
  process-local writes do not reach the file.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

1. **AC-0 first (Architect, on the kickoff base).** Confirm or refute
   the no-runtime-change premise. Name the injection seam for a backend
   mapping failure and the construction of a malformed Mapping token.
2. **One branch.** It adds the fixture and its native test, the two
   refusal witnesses, and the seed-text change that lifts the markers on
   cases 1 to 3. The Spec co-turn owns the seed text, and CV votes on
   `conformance/`.

## Acceptance

- **AC-1.** Both engines run the program in parity:
  - acquire `FileBacked` over a file with known bytes, `ReadWrite`;
  - `mapWrite` different bytes in range;
  - `mapBytes` reads the written bytes;
  - after the bracket settles, the §1.3 file read returns the original
    bytes.

  The two engines' traces and exit statuses are equal.
- **AC-2 (mutation, QA).** A write-through mutant, which writes the
  region back to the file on `mapWrite` or on release, reddens AC-1's
  file-read assertion on both engines.
- **AC-3.** A backend mapping failure surfaces as `AllocationFailed` or
  `Io`, and a malformed Mapping token surfaces as its typed refusal. Each
  has its own exact-variant assertion.
- **AC-4.** Cases 1 to 3 of `seed-mapping.md` name their witnessing
  tests and carry no `BLOCKED-ON` marker.

## Stop conditions

- AC-0 finds that a runtime or host change is needed to keep writes off
  the file. That is a design finding for the Architect, and the frame is
  re-sized.
- A new carrier, Need, kernel change or trust growth, which is an
  operator stop.

Checks: 4 fired: the obligation was written against main's surface. The
fixture needs `withMapping (FileBacked h n) ReadWrite`, `mapWrite`,
`mapBytes` and a §1.3 read. Every one exists, and the source handle
needs the write right, per QA's `RightNotHeld` probe. 8 fired: the
current read-only fixture with an empty filesystem delta gives the same
observation under write-through, which is why AC-2's mutant must redden
the file-read assertion itself.
