---
id: RT-FS-DESCRIPTOR-INHERITANCE-DUPLICATE
title: "ABI-S1 merged host implementations of FsGetInheritance, FsSetInheritance and FsDuplicate, but no Ken source can reach them and all three are RepresentedUnavailable, while PX10's spawn plan already builds child descriptor maps under deny-by-default inheritance. Rule from an AC-0 whether each operation gains a checked surface and native parity or leaves the catalog because the spawn plan subsumes it, and land the ruled result"
status: ready
owner: language
size: L
tier: T1
gate: architect
depends_on: [RT-FS-DESCRIPTOR-SEEK-LENGTH-SYNC]
blocks: [LINUX-ABI-EXIT-SERVICE-SUPERVISOR]
github: null
origin: "Steward 2026-11-22 on operator direction (\"Create WPs and update the campaign docs as necessary to capture the gaps you have identified\"). Program §6 requires every operation NativeTested or probed; these three are unavailable only because nobody surfaced or promoted them. Sequenced after RT-FS-DESCRIPTOR-SEEK-LENGTH-SYNC, whose AC-0 sets the descriptor surface pattern. L1 Linux ABI program, Steward order. Released on the Language ring (L1b, operator 2026-11-22: \"Approved. Use the language ring.\") as the DESCRIPTOR successor; program §4 places these three before ABI-M2, whose probes are for operations the kernel lacks. Anchors re-checked on b36f84e03."
---

# Descriptor inheritance and duplication

## Objective

Every one of `FsGetInheritance`, `FsSetInheritance` and `FsDuplicate`
ends in one of two states. Either a checked Ken wrapper reaches it and it
is `NativeTested` with interpreter parity, or it has left the operation
catalog with its host code, wire request and planner arm.

## Settled inputs (main `b36f84e03`; anchors re-checked at release)

- **Host.** `ken-host/src/lib.rs` implements
  `resource_get_inheritance_v1` (`:1100`), `resource_set_inheritance_v1`
  (`:1113`) and `resource_duplicate_v1` (`:1129`) over
  `FdInheritancePolicyV1`. `effect_v1.rs` marks all three
  `RepresentedUnavailable` (`:229` to `:231`), and
  `effect_abi_v1.catalog` lists them `unavailable` (`:108` to `:110`).
- **No surface.** The prelude has no constructor for any of the three,
  and `ken-interp` does not decode them. `FileOperation.OpGetInheritance`,
  `OpSetInheritance` and `OpDuplicate` exist as error-context cases
  (spec 38 ~`:1441`). DESCRIPTOR's AC-0 carries the FileOperation
  handle-case family (`evt_fzy51fc52v1x` §4); a retirement here feeds
  that carry rather than editing the family.
- **The spawn plan.** PX10's declarative plan constructs the child's
  descriptor map with close-on-exec and deny-by-default inheritance
  (program §4 Track T, spec 38 §1.11). Spec 38 ~`:1105` states that
  the child inherits no other parent descriptor, even one without
  close-on-exec, so its descriptor set at start is exactly `{0, 1, 2}`.
  A Ken program has no post-`fork` callback, so the plan is the only
  point where a child inherits a descriptor.
- **Native.** The planner names all three among the operations refused
  before any seat is derived (`static_transition/effects.rs` ~`:700`).
- **The descriptor pattern.** `RT-FS-DESCRIPTOR-SEEK-LENGTH-SYNC`'s AC-0
  (`evt_fzy51fc52v1x`) retired `FsSeek` and surfaced `setLength` and
  `sync` in `Capability.System.IO` on `ResourceWriteCreate` handles.
  Follow its landed shape; re-check these anchors on its merge.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

The three operations' surface, interpreter decoding, native planning and
lowering, `ken-verify` evidence and spec 38, or their retirement. No
change to the spawn plan's inheritance default or to its descriptor-map
construction.

## Acceptance

- **AC-0 (Architect, before code).** For each operation, rule surface or
  retire, against PRINCIPLES' subsume-don't-proliferate item and the spawn
  plan's descriptor map. For a surfaced inheritance setter, rule how it
  composes with deny-by-default: whether it can widen what a child
  inherits outside the plan. If it can, it amplifies authority, and the
  ruling must say why that is admissible. Sweep every consumer of the
  operation identity across `crates/`, `conformance/` and `spec/`.
- **AC-1 (surfaced).** One differential row per operation, interpreter
  against native, confirmed by `confirm_native_tested_transition`, each
  with a negative control that perturbs the evidence. A spawned child
  observes a descriptor exactly when the plan grants it, whatever the
  parent's inheritance flag.
- **AC-1 (retired).** The operation identity is absent from the catalog
  and from `HostOpV1`, and the build passes on every consumer AC-0
  swept.
- **AC-2.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- A surfaced setter lets a child inherit a descriptor the plan did not
  grant.
- The ruling needs a new authority class or a TCB change.

Checks: 2 fired. The guarded property is "a child inherits only what the
plan grants". Its fan-in is the spawn plan's map, the descriptor's
close-on-exec flag, and any duplicate that clears it. AC-1's child row
covers all three. 4 fired. I read the three host functions, the
catalog rows, the planner refusal, spec 38's child descriptor sentence and
the absence of a prelude constructor or interpreter decoder on
`b36f84e03`.
