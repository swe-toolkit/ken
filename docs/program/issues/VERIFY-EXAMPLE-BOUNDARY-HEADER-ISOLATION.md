---
id: VERIFY-EXAMPLE-BOUNDARY-HEADER-ISOLATION
title: "A ken example fence with a program capabilities header overwrites the program's own boundary header, so the runner and native admission mint ProgramCaps from the example's authority: an example can raise a program's FS authority. Example elaboration must leave the package's boundary header as the program declared it"
status: ready
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
