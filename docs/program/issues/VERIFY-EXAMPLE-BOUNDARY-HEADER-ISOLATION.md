---
id: VERIFY-EXAMPLE-BOUNDARY-HEADER-ISOLATION
title: "A ken example fence with a program capabilities header overwrites the program's own boundary header, so the runner and native admission mint ProgramCaps from the example's authority: an example can raise a program's FS authority. Example elaboration must leave the package's boundary header as the program declared it"
status: merged
owner: verify
size: S
tier: T2
gate: architect
depends_on: [VERIFY-NATIVE-PROGRAM-EXAMPLE-EXCLUSION]
blocks: []
github: null
origin: "Adversary finding evt_4t57ymq37xyj7 on ce4e72b33 (pre-existing): violates spec/40-runtime/62-authority section 3.2 (Ken source cannot raise authority) and 33-declarations section 3.2.1 (the runner mints ProgramCaps from the program's own header). Steward-filed per COORDINATION section 2."
---

# An example cannot change a program's authority

## Objective

A `ken example` fence is checked, not shipped. Its boundary header, if
any, never reaches `ProgramCaps`, on the runner or on native admission.

## Settled inputs (on `ce4e72b33`)

- **The write.** `expand_and_elaborate` sets `direct_call` when the
  parsed declarations carry a boundary and no package is current
  (`modules.rs:5295`), and on that path writes
  `module_state.boundary_header` (`:5364-5365`). The class-env admits
  state is saved and restored around the call (`:5296-5331`); the
  boundary header is not.
- **The examples.** The example executor elaborates each fence through
  `elaborate_file_v1` (`lib.rs:701-708`), a direct call, so the last
  headed fence wins. A header-less example does not write.
- **The readers.** Native admission reads `env.boundary_header()`
  (`program_admission.rs:57-60`); `ken run` mints `ProgramCaps` from
  the same seam.
- **Repro** (Adversary): a plain fence `program capabilities FS ANone`, a
  `main` typed `ProgramCaps AFull` that writes a file, and a trailing
  example fence with only `program capabilities FS AFull`. `ken run`
  exits 0 and writes the file; without the example it is refused with
  "invalid entrypoint 'main': expected ... ProgramCaps ANone ...". The
  native route behaves the same in preparation, including a header only
  in a dependency's example admitting a program that has none
  (`MissingProgramBoundary` without it).

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

Example elaboration leaves `boundary_header` exactly as the program's
own sources set it. The Architect rules the seam at D0: restore around
the example executor, or refuse a boundary header inside a `ken example`
fence. Enumerate every other `module_state` or `class_env` field an
example's direct call can write and leave behind, and close each one
that reaches admission or the runner the same way.

## Acceptance

- **AC-1.** The Adversary's four rows (run with the AFull example;
  native with the AFull example; a valid ANone program plus the AFull
  example; the dependency-only example header) each end as the same
  program without the example does: refused, or admitted with the
  program's own authority.
- **AC-2 (control).** A header-less example leaves results unchanged,
  and literate programs in the catalog and corpus keep their results.
- **AC-3 (mutation, QA).** Restoring the unguarded write reddens AC-1.

## Stop conditions

- A catalog or corpus program relies on an example's header: stop with
  the list.
- The repair needs a kernel, trust or spec change.

## Closeout

Merged `f63764273` from exact `6b83e1d23` (PR #4662). Verify QA
`evt_7bmbfc48qdvnj`, Architect `evt_tphteqveavm7`, Decision
`dec_4bzq55z97z0np`. The checked-fence executor saves the program's
boundary header, runs the reject and example loops through a private
helper, and restores the header on both `Ok` and `Err`, the seam ruled at
D0 (`evt_2k1ctycw0444f`). An example's header no longer reaches
`ProgramCaps` on `ken run`, native admission, a dependency-only header or
the roots-loaded entry; each fence is still checked against its own
header. A no-op-restore mutant reddened 7 of 8 rows, including an actual
forbidden write. Zero headed example fences among 73 tracked `.ken.md`
files, and nine real-file outputs are byte-identical. §1a: 0.

Residuals, disposed (`evt_qn2e1q8bca92`): R1 declined; R2 is
`VERIFY-PACKAGE-BOUNDARY-HEADER-ORDER`; R3 is
`VERIFY-EXAMPLE-MAIN-NOT-ENTRYPOINT`.
