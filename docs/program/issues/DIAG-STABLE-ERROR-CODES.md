---
id: DIAG-STABLE-ERROR-CODES
title: "Errors and refusals are identified only by message text, so tests and tools pin prose and a reworded message breaks them. Give every error the CLI can return a stable code plus a context-sensitive message: deterministic systems check the code, intelligent actors read the message"
status: active
owner: runtime
size: L
tier: T2
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-12: \"Stable error codes + context sensitive messaging is the preferred design, which allows deterministic systems to check the stable code and intelligent actors to read the message. This is probably a large T2 change. Scope it and create a WP. Sequence it after the ignored tests work has finished.\" Research advisory evt_6ps86q1jcnghm (typed guard identity, the RELAY 18th stop). Steward-filed per COORDINATION section 2."
---

# Stable error codes with context-sensitive messages

## Objective

Every error or refusal the CLI can return carries a stable code that
never changes meaning, beside a message written for the reader. A test or
tool that asks "which failure is this" compares the code, never the
prose.

## Settled inputs (measured on `811ca8ee3`)

- **No code exists.** No crate defines an error code or a `code()`
  accessor.
- **Typed enums already exist in the front end:**
  - `ElabError` (ken-elaborator/src/error.rs:77) has 61 variants.
  - `KernelError` (ken-kernel/src/error.rs:14) has 13.
  - `RunError` (ken-cli/src/lib.rs:47) wraps them.
  - About 30 other `*Error` enums sit in the elaborator, host, interp,
    runtime and verify crates.
- **The native back end identifies refusals by free text.**
  `UnsupportedLowering { construct: &'static str, reason: String }`
  (ken-runtime/src/cranelift_backend/surface.rs) is built through
  `unsupported(` at about 1,066 sites in ken-runtime. Calls named
  `unsupported(` number about 30 in ken-interp and 23 in ken-host.
- **Tests pin the prose.** Error values are rendered with `{…:?}` and
  matched by substring:
  - 133 sites in ken-runtime's in-source tests;
  - 39 sites across 17 integration-test files;
  - the `.contains(` population in the test roots is 1,104 lines, of
    which these are a part.
- **The motivating case.** `px7f_resource_native.rs:733` pins
  `stage: ObjectEmission`, `checked_process_object` and a reason string.
  Re-pinning it after a reworded reason was the RELAY 18th stop.
- **The kernel is TCB.** `ken-kernel` has no dependencies. A code for a
  `KernelError` variant can be assigned where the error is projected,
  outside the kernel.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

D0 first: the Architect rules the design, and the WP then lands as
straight-ancestor increments, each one accepted work (operator
2026-08-06).

1. **D0 (Architect).** Rule on:
   - the code type and its namespace, and where it lives so that every
     crate's errors can carry one without a dependency into the kernel;
   - the stability rule: a code is never reused and never changes
     meaning, in an append-only registry that a test enforces;
   - how the CLI renders the code and the message for a person and for a
     machine;
   - the grouping of the back end's `unsupported(` sites into codes, by
     cause rather than by call site;
   - whether the code set is a user-facing contract that needs a spec
     section. If it does, that section is a Spec enclave node, framed then.
2. **Front end.** The code type and registry, codes for `ElabError`,
   `KernelError` (by projection), `RunError` and the other front-end
   enums, and CLI rendering.
3. **Native back end.** `UnsupportedLowering` and the other runtime
   refusals carry a code. The message keeps the site's context.
4. **Tests.** A test that identifies a failure asserts its code. A
   message is asserted only where its wording is itself the subject.

## Acceptance

- **AC-1.** Every `RunError` path and every native refusal reaching the
  CLI carries a code. The registry test fails on a duplicate code and on
  a code removed from the registry.
- **AC-2 (the motivating row).** `px7f_resource_native.rs:733` asserts
  the code. Rewording its reason leaves it green, and changing the code
  reddens it.
- **AC-3 (discriminating).** Two refusals that share message text but
  differ in cause carry different codes, and one row shows each.
- **AC-4 (no behaviour change).** Interpreter and native results, exit
  statuses and `trusted_base()` are unchanged, and `ken-kernel` source is
  byte-identical. CI green on each increment.

## Stop conditions

- Any change to `ken-kernel`, a check rule or `trusted_base()`: that is a
  TCB change and returns to the operator.
- One refusal site serves two causes that need two codes, and splitting
  it changes control flow: stop to the Architect.
- A user-visible contract beyond the code and message, such as a new CLI
  flag or output format, is not in the D0 ruling.
