---
id: VERIFY-CALLER-OBLIGATION-REPORTING
title: "ken check, ken run, the compiler driver and the REPL elaborate through ID-only wrappers that drop ElabResult.obligations, so a successful declaration's open obligation may reach no report. Make every user-facing caller report its open obligations and fail or flag them"
status: active
owner: verify
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Architect evt_2sb1ehveqk5m0 (from the REFINEMENT review carry evt_53r1rc38nxqgb): the same objective as VERIFY-REUSED-ENV-TRUST-RESIDUE, trust nobody reports, but a different mechanism and seam, so a separate WP on the verify ring. Placed ahead of LANG-ELAB-RECURSIVE-FRAME-BUDGET while REUSED-ENV waits for the operator. Steward-filed per COORDINATION section 2. Measured at origin/main 3ad9a5590."
---

# User-facing callers report open obligations

## Objective

When `ken check`, `ken run`, the compiler driver or the REPL accepts a
declaration whose `ElabResult.obligations` is non-empty, the user sees each
open obligation, and the command does not report a clean success.

## Settled inputs (measured at `3ad9a5590`)

- **The callers** use wrappers that return only `GlobalId`s:
  - `ken check` and `ken run` (`crates/ken-cli/src/main.rs:383-395`) call
    `elaborate_module_from_roots`, `elaborate_ken_md_file` or
    `elaborate_file`;
  - the compiler driver (`crates/ken-elaborator/src/compiler_driver.rs:629`
    and `:1212`) calls `elaborate_ken_md_file` or `elaborate_file`;
  - the REPL `do_def` (`crates/ken-cli/src/repl.rs:137`) calls
    `elaborate_decl`.
- **Reporting APIs.** `elaborate_decl_v1` (`lib.rs:408`) and
  `elaborate_file_v1` (`lib.rs:439`) return obligations. The `.ken.md` and
  module-root paths have no `_v1` counterpart today, so adding them is in
  scope (CHECKS 4).
- **Not this WP.** A failed declaration's orphan hole is
  `VERIFY-REUSED-ENV-TRUST-RESIDUE` (environment rollback).

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

1. **AC-0, at the start of the repair.** For each of the four callers, run
   a file with one open `requires` obligation (a call with no in-scope
   premise) and record the exit status and output. The Architect then rules,
   per caller, whether an open obligation fails the command or is reported
   as a flagged success. If `/spec` does not settle that, the Architect
   routes it to the Spec enclave.
2. **The ruled repair.** Every caller elaborates through an API that returns
   obligations, and applies the ruled behaviour.

## Acceptance

- **AC-1.** For each caller, the open-obligation file shows each obligation
  and gets the ruled exit behaviour. Each row asserts the exit status first.
- **AC-2 (controls).** A file whose obligations are all discharged, and a
  file with none, still exit 0 with unchanged output. The catalog and
  example suites stay green.
- **AC-3 (falsifier).** Switching one caller back to its ID-only wrapper
  turns its row red.

## Stop conditions

- Any kernel, `trusted_base()` or spec change.
- A catalog, example, conformance or test consumer relies on a clean exit
  with open obligations: stop to the Architect with the consumer (CHECKS 3).
