---
id: RT-FS-FILE-OPERATION-HANDLE-CASES
title: "FileOperation still carries six handle-operation cases, OpSeek through OpDuplicate, that spec 38 says no producer selects. Four of them name operations the descriptor WPs retired from HostOpV1 (FsSeek, FsGetInheritance, FsSetInheritance, FsDuplicate). Yet the prelude, the constructor-ID spine, both lanes and System.Error's idempotence table still carry all six, so a Ken program can construct and match an operation context that no host path can produce. Rule from an AC-0 whether only the path cases stay, and remove what it retires across every consumer"
status: ready
owner: language
size: L
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "L1b filesystem objective (operator 2026-11-22: \"Approved. Use the language ring.\"). The Architect's DESCRIPTOR AC-0 carried this family (evt_fzy51fc52v1x §4), and INHERITANCE-DUPLICATE retired its three operations into that carry. PRINCIPLES: subsume-don't-proliferate; prelude names are fixed surface and must be minimal (operator 2026-09-25). Steward-filed per COORDINATION section 2."
---

# FileOperation names only operations that exist

## Objective

Every `FileOperation` case names an operation that a producer can report.
A case that no producer selects is either retired from the family, together
with its constructor ID and every consumer, or AC-0 names the producer that
will select it.

## Settled inputs (main `b8bc69953`; anchors re-checked at release)

- **The family.** `prelude.rs:1079` declares `data FileOperation` with
  sixteen cases. The last six are `OpSeek`, `OpSetLength`, `OpSync`,
  `OpGetInheritance`, `OpSetInheritance` and `OpDuplicate`. Their spine
  roles are at `prelude.rs:164-169`.
- **The spec admits that they are dead.** Spec 38 (`38-ffi-io.md:1466-1470`)
  says the six "are not selected by any current producer; handle
  operations report through ResourceError".
- **Four name retired operations.** `effect_v1.rs:6734` asserts that
  `FsSeek`, `FsGetInheritance`, `FsSetInheritance` and `FsDuplicate` are
  absent from the catalog. `FsSetLength` and `FsSync` are live
  (`effect_v1.rs:220-221`), but they report through `ResourceError`, not
  `FileOperation`.
- **The consumers.** A grep of the six case names and their
  `file_operation_*` roles on main finds 20 files:
  - elaborator: `compiler_driver.rs`, `erasure.rs`,
    `erasure/selected_host_binder_depth_tests.rs` and `prelude.rs`;
  - native lane: `ken-runtime-support/src/ir.rs`, `lowering/effects.rs`,
    `static_transition/semantic_ir.rs`, `native_process_authority.rs` and
    `native_process_entrypoint.rs`;
  - interpreter and verify: `ken-interp/src/eval.rs` and
    `ken-verify/src/scenario.rs`;
  - catalog: `Capability/System/Error.ken.md:206-211` (the
    `operation_idempotence` arms);
  - spec: 38 and 30-intrinsic-ledger;
  - conformance: `seed-strict-package-homes.md` and `taxonomy/minimality.md`;
  - tests: four elaborator test files.
- **The floor.** Spec 38 says only the five I/O family names enter the
  counted floor, and no constructor leaf joins bare protected B. So
  retiring cases should not move the floor count. AC-0 confirms this.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

The `FileOperation` family, its constructor IDs and spine roles, every
consumer that AC-0's sweep names, spec 38 and the intrinsic ledger, and
the conformance rows. Out of scope: the path cases, `ResourceError`, and
any change to what an operation reports.

## Acceptance

- **AC-0 (Architect, before code).** Rule:
  - for each of the six cases, retire it or name its producer;
  - whether the native constructor IDs of the remaining cases renumber or
    keep their values, and what that means for artifact compatibility;
  - the consumer sweep, across `crates/`, `catalog/`, `conformance/`,
    `spec/`, `library/`, `examples/` and the CLI fixtures.
- **AC-1.** A Strict client that constructs or matches a retired case
  refuses at elaboration with the unbound-constructor error. The same
  client with a path case accepts. The paired rows differ only in the
  case name.
- **AC-2.** Both lanes still report every path-operation error with its
  `FileOperation` context unchanged. One existing interpreter row and one
  native row per lane run green on the candidate.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- A retired case turns out to have a producer on main.
- Retirement needs a change to the counted floor or to `ResourceError`.
- The native ID ruling needs a target ABI or manifest hash change that
  AC-0 did not rule.

Checks: 4 fired. I read the prelude family, the spec 38 paragraph, the
catalog absence assertion and the live availability lines on `b8bc69953`.
3 fired. I grepped the six case names and their spine roles across
`crates/`, `catalog/`, `conformance/`, `spec/` and `library/`, and found 20
files. 8 fired. A candidate that drops the case from the prelude but leaves
its spine role or idempotence arm fails to build, and one that leaves the
case in place passes AC-1's accept row but fails its refuse row.
