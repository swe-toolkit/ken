---
id: ABI-FSCREATE-RECURSIVE-CONTRACT
title: "create_directory takes a recursive Bool that the surface, the wire, the interpreter, native lowering and ken-verify all carry, but both host implementations drop it, so recursive=True silently creates one leaf and refuses a missing parent. Rule from an AC-0 whether the flag gains parent-chain meaning under the filesystem root confinement or leaves the operation, and land the ruled result on both lanes"
status: ready
owner: language
size: M
tier: T1
gate: architect
depends_on: []
blocks: [LINUX-ABI-EXIT-SERVICE-SUPERVISOR]
github: null
origin: "Registered by the Steward 2026-09-07 from ABI-A3's D0 hard stop (runtime-implementer evt_4k04typ5a45mg) and the Architect's §6 contract ruling (evt_3hfxk94v9f9n7), which blessed the inert behaviour as A3's NativeTested contract but not the inert field. Framed 2026-11-22 for the L1b track on the Language ring (operator: \"Approved. Use the language ring.\"), the first of the filesystem nodes that run alongside the native chain. L1 Linux ABI program, Steward order."
---

# create_directory's recursive flag

## Objective

`create_directory` either honours `recursive = True` by creating the
missing parent chain inside the capability's root, or no longer takes the
flag. In both cases the interpreter and a native build agree, and spec 38
states the result. No caller can pass a flag that changes nothing.

## Settled inputs (main `7e8e1cc38`; anchors re-checked at release)

- **The surface carries it.** The prelude declares `proc create_directory
  (a : Auth) (cap : Cap a) (recursive : Bool) (path : Bytes)`
  (`prelude.rs` ~`:2800`), whose package home is
  `Capability.Filesystem.Authority` (spec 38 ~`:1445`).
- **The wire carries it.** `FsCreateDirectory` (`0x0306`) uses
  `FsRecursivePathRequestV1 { recursive: u64, path }` (`abi_v1.rs:197`),
  decoded at ~`:1836`. `FsRemoveDirectory` shares the struct.
- **Both hosts drop it.** The ABI host takes `_recursive` and calls one
  `create_directory(parent, leaf)` (`abi_v1.rs:696`). The interpreter's
  real host does the same (`eval.rs` ~`:4615`). `ken_host::create_directory`
  is a single `mkdirat` (`lib.rs:513`). The sibling remove path does
  branch: `recursive` selects `remove_directory_tree` (`abi_v1.rs:719`).
- **Evidence records it.** `ken-verify` records the flag
  (`host.rs` ~`:467`, `scenario.rs` ~`:3158`), and native lowering
  marshals it (`lowering/effects.rs`). The operation is `NativeTested` on
  the inert behaviour (ABI-A3).
- **The spec is silent.** Spec 38 names `create_directory` only in the
  package-home list. No sentence gives `recursive` a meaning.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

The flag's meaning or removal across the prelude, `ken-host` (wire, ABI
host, `lib.rs`), `ken-interp`, native lowering, `ken-verify`, spec 38,
and the consumers AC-0 sweeps. No change to `FsRemoveDirectory`'s
recursive removal, or to the root confinement rules themselves.

## Deliverable

One candidate with the ruled contract, both lanes agreeing, and the spec
38 sentence as a Spec co-turn.

## Acceptance

- **AC-0 (Architect, with the Spec Leader, before code).** Rule meaning or
  retirement against PRINCIPLES' subsume-don't-proliferate item. A client
  can already create a chain by calling `create_directory` once per
  prefix. The ruling weighs that against what one host call buys:
  atomicity against a concurrent rename, and confinement checked once per
  component. If it rules meaning, it states:
  - the error for an existing non-directory component;
  - that an existing target directory is either `AlreadyExists` or success;
  - how each created component stays beneath the root, with no symlink
    followed out of it.
  If it rules retirement, it states the wire change, a new request struct
  or the shared struct with a fixed field, and whether `0x0306`'s arity
  moves. Either way, sweep every consumer across `crates/`, `catalog/`,
  `conformance/`, `examples/` and the CLI fixtures.
- **AC-1 (meaning).** A row creates `R/a/b/c` with `recursive = True`
  where only `R` exists, and all three directories exist afterwards, on
  both lanes. The same call with `recursive = False` refuses `NotFound`
  and creates nothing. A row whose chain passes through a symlink to a
  directory outside `R` refuses, and nothing is created outside `R`.
- **AC-1 (retirement).** No surface, wire, host or evidence path carries
  the flag. A source that passes it fails to elaborate, and
  `create_directory` keeps its single-leaf behaviour on both lanes.
- **AC-2.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- Meaning needs a confinement primitive the host does not have, such as
  creating beneath a root without following symlinks. Report it, and do
  not create through `/proc/self/fd` path concatenation.
- Retirement needs a change to `FsRemoveDirectory`'s request.

Checks: 3 fired. I grepped `crates/`, `catalog/`, `conformance/`,
`examples/` and the CLI tests for `create_directory` and
`FsCreateDirectory` on `7e8e1cc38`. Seventeen crate source files and at
least seven test files name the operation, so AC-0 owns a whole-root
sweep. 2 fired. The guarded property is "nothing is created outside the
root". Its fan-in under meaning is every component the chain creates, not
only the leaf. AC-1's symlink row covers an intermediate component.
8 fired. A host that still ignores the flag fails AC-1's three-level row.
A retirement that drops the flag from the surface but leaves it on the
wire passes an elaboration row alone, so AC-1's retirement row names the
surface, the wire, the host and the evidence paths.
