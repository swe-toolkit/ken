---
id: ABI-S2
title: "read_directory returns a whole sorted List DirEntry, which the host builds by reopening the descriptor through /proc/self/fd and collecting every entry into one reply, so a large or concurrently changing directory is either held whole or truncated by an unstated bound. Rule from an AC-0 whether directory reading becomes a handle-based stream read in batches to an end, or keeps the whole-list shape with a stated bound and a typed refusal past it, and land the ruled result on both lanes"
status: active
owner: language
size: M
tier: T1
gate: architect
depends_on: [RT-NATIVE-SCALAR-JOIN-REFERENT-OWNERS]
blocks: [LINUX-ABI-EXIT-SERVICE-SUPERVISOR]
github: null
origin: "docs/program/10-linux-abi-completion.md §4 Track S. Node filed by the Steward 2026-07-25. Framed 2026-11-23 as the third L1b node on the Language ring (operator 2026-11-22: \"Approved. Use the language ring.\"), absorbing the Architect's FSCREATE AC-0 carry on directory access through /proc/self/fd (evt_5zrw3xt08fewp). L1 Linux ABI program, Steward order."
---

# Directory reading with an honest shape

## Objective

A Ken program reads a directory's entries without the host holding an
unbounded list it did not state. Either the program reads entries from an
open directory handle in batches until an end marker, or `read_directory`
keeps its whole-list shape with a stated bound and a typed refusal past
it. Both lanes agree, and spec 38 states the result.

## Settled inputs (main `f09912a10`; anchors re-checked at release)

- **Surface.** `proc read_directory (a : Auth) (cap : Cap a) (path :
  Bytes) : FS a (Result FileError (List DirEntry))` (`prelude.rs`
  ~`:2794`), with `data DirEntry = MkDirEntry Bytes FileKind` (~`:1070`).
  Its package home is `Capability.Filesystem.Authority` (spec 30 taxonomy
  ~`:643`).
- **Host.** `ken_host::read_directory` (`lib.rs` ~`:488`) formats
  `/proc/self/fd/<n>` for the opened handle, then runs `std::fs::read_dir`
  on that path. It collects every entry and sorts the list by name. The
  ABI host (`abi_v1.rs` ~`:723`) opens the leaf with
  `OpenRequest::ReadDirectory` (`RDONLY | DIRECTORY | NOFOLLOW`), and it
  encodes one reply: a u64 count, then the entries (~`:1374`, decoder
  ~`:1399`).
- **Catalog and wire.** `FsReadDirectory` is `0305`, `native`,
  `FsPathRequestV1` (`effect_abi_v1.catalog:89`). Its canonical request
  is tag 9 (`effect_wire.rs` ~`:935`).
- **Interpreter and evidence.** `eval.rs` traces
  `FsTrace::ReadDirectory` (~`:5352`). `ken-verify` compares the
  operation (`canonical.rs` ~`:345`, ~`:393`).
- **The handle pattern exists.** `withResource` over `PrivateFsOpen`, and
  `PrivateFsReadAt` returning `ReadProgress` (`prelude.rs` ~`:2709`,
  ~`:2714`), are the checked template for a batched read with an end.
  PX11-NATIVE Inc2 is folding every native ResourceKind copy into one
  host-owned population (Architect `evt_42ncdsdxj34kk`). A new kind added
  after Inc2 lands is a one-place change.
- **Consumers.** 25 files under `crates/`, `conformance/` and `examples/`
  name `read_directory`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

The directory-read surface, the host read path, the catalog row, wire and
reply, the interpreter hosts, native lowering, `ken-verify` evidence, and
spec 38. No change to path resolution, to root confinement, or to
`FsRemoveDirectory`'s tree walk. That walk's own `/proc/self/fd` use is a
separate carry.

## Acceptance

- **AC-0 (Architect, with the Spec Leader, before code).** Rule stream
  against bounded-whole, against PRINCIPLES' subsume-don't-proliferate
  item. Then:
  - **If stream:** the handle kind (a new ResourceKind, or an existing
    kind with a directory open mode), the batch operation and its end
    marker, whether entry order is specified, and whether
    `read_directory` stays as derived code over the stream or leaves the
    surface. If the ruling needs a new ResourceKind, state whether this
    WP waits for PX11-NATIVE Inc2's single kind population.
  - **If bounded-whole:** the bound, how it is stated, and the
    `FileError` cause past it.
  - **Both:** the host primitive that reads entries, and whether `.` and
    `..` are filtered. A stream keeps its position in the open
    descriptor, so it reads with `getdents64` (through `rustix::fs::Dir`
    or equivalent), not by reopening `/proc/self/fd/<n>` once per batch.
    Sweep every consumer.
- **AC-1.** A differential row, interpreter against native, confirmed by
  `confirm_native_tested_transition`, reads a directory with more entries
  than one batch (stream) or than the bound (bounded-whole). Stream: the
  concatenated batches equal the directory's entries exactly, and the end
  is observed once. Bounded-whole: the bound refuses with the ruled cause
  on both lanes, and a directory one entry under the bound succeeds. The
  negative control drops one entry from one lane's evidence, and
  confirmation fails.
- **AC-2.** Spec 38 states the ruled shape as a Spec co-turn. Full-
  workspace CI is green on the exact candidate.

## Stop conditions

- The ruled shape needs a new authority class or a TCB change.
- A stream ruling that needs a new ResourceKind before PX11-NATIVE Inc2
  lands, and the Architect rules that it must wait. Report this; do not
  add a fourth native kind copy.
- No descriptor-based entry read is available on the CI kernel.

Checks: 4 fired. I read the surface, the host body that formats
`/proc/self/fd`, the ABI host's whole-list reply, the catalog row and
the canonical tag on `f09912a10`. 3 fired. 25 files name `read_directory`,
so AC-0 owns a whole-root sweep. 8 fired. The more-than-one-batch row
fails a host that returns one batch and stops. The dropped-entry control
fails a comparator that checks only the count. I did not pin the
`/proc/self/fd` reopen by itself. It reopens the same directory with no
path concatenation, so no behavioural row separates it from a
descriptor read. Under a stream ruling, AC-1's multi-batch row reaches it
through the batch position.
