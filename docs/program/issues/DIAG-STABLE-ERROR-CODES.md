---
id: DIAG-STABLE-ERROR-CODES
title: "Errors and refusals are identified only by message text, so tests and tools pin prose and a reworded message breaks them. Give every error the CLI can return a stable code plus a context-sensitive message: deterministic systems check the code, intelligent actors read the message"
status: merged
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

## Settled inputs (Architect D0 `evt_1yhr30fekfyyv`, on `473ccf9b4`)

- **No code exists.** No crate defines an error code or a `code()`
  accessor.
- **Front-end enums.** `ElabError` has 61 variants, `KernelError` 14 (the
  last a free-text `Msg`), `RunError` (ken-cli/src/lib.rs:47) 11, plus
  `CompilerDriverError` (18), `NativeProgramBuildError` (5) and
  `RunIoError` (3).
- **Native refusal carriers**, word-bounded counts in ken-runtime/src:
  `planner_error(` 1,259, `unsupported(` 1,013 (plus 30
  `eval_unsupported(` and 23 `preflight_unsupported(`), `backend_module(`
  501, `planner_capacity_error(` 235, `PendingRefusal` 45,
  `NativeTrustError` 28, `NativeAuthorityError` 12. ken-host has 23
  `unsupported(`; ken-interp has none.
- **The motivating row** is `owner_vis_pending_verifier_rejects_unchecked_k_result`
  at crates/ken-cli/tests/px7f_resource_native.rs:741. Its reason is a
  `backend_module(` text emitted at lowering/units.rs:3605 and :3838,
  which test one predicate.
- **Control flow keyed on the carrier.** artifact/api.rs:1095 and :1142
  turn `CraneliftBackendError::Unsupported` into the `Unsupported`
  verdict, so moving a site between carrier variants changes control
  flow.
- **The TCB stays untouched.** ken-kernel and ken-host gain no
  dependency; their errors are coded by projection in the crates that
  wrap them.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The D0's design, as three straight-ancestor increments, each one accepted
work (operator 2026-08-06):

1. **ken-diagnostic and the front end.**
   - A new dependency-free leaf crate `ken-diagnostic` holds `Code`, the
     append-only ledger `codes.tsv` and its test.
   - The front-end enums get a `code()` that is one exhaustive `match`
     with no `_ =>` arm, a `TABLE`, and a ledger-match test.
   - Wrapper variants delegate. `KernelError` is coded by projection in
     ken-elaborator. Nothing is rendered yet.
2. **The runtime.**
   - `UnsupportedLowering` gains `cause: LoweringCause` over the seven
     families N0001-N0007, with no `_` arm.
   - Every `unsupported(` and `backend_module(` site names one family by
     the D0's membership rule; `planner_error` maps to N0003 and
     `planner_capacity_error` to N0002.
   - No site changes carrier variant.
3. **CLI rendering**, only after `SPEC-DIAGNOSTIC-CODE-CONTRACT` lands,
   and as that section states it (spec D0 `evt_5gjsaxr0t1xvg`).
   - Every coded failure prints a whole stderr line
     `error[<code>]: <message>`. Its position on stderr is not pinned.
   - The population with no carrier yet, measured at `74dd59d20`:
     - 21 C sites in ken-cli `main.rs`, coded through one `CliError`.
       These are 17 exit-path diagnostics and the 4 `fmt` per-path
       diagnostics.
     - 3 H sites in ken-cli `lib.rs` on the `ken run` path, coded by
       projection from ken-host's `TerminalErrorV1` with no ken-host
       change. Two of them exit 1 with no message today.
     - The `native-build` build error and the `fmt` formatting error stay
       E. The dead `run` arm at `main.rs:409` is deleted.
   - Coded failures exit 1. `main.rs:502` and `:517` change from 2.
   - Tests that identify a failure migrate to codes.
   - px7f:741 asserts N0004.

## Acceptance

- **AC-1.** The ledger test fails on a reused code or label, a gap, a
  malformed row and an unregistered code. Each owning crate's table test
  fails on an active ledger row it does not produce. Every `RunError`
  path and every native refusal reaching the CLI carries a code.
- **AC-2 (the motivating row).** px7f:741 asserts N0004. Rewording its
  reason leaves it green, and changing the code reddens it.
- **AC-3.**
  - (a) The units.rs:3605/:3838 pair, one predicate, carries one code.
  - (b) Two rows of different families carry different codes.
  - (c) A test-only rewording of either message leaves the code
    assertion green.
- **AC-4 (no behaviour change).** Interpreter and native results, exit
  statuses and `trusted_base()` are unchanged, and ken-kernel and
  ken-host sources are byte-identical. CI is green on each increment.
  Increment 3 is the one exception, and only as the spec section rules:
  the coded line, exit status 1 at `main.rs:502` and `:517`, and messages
  on the two silent H sites.

## Stop conditions

- Any change to ken-kernel, ken-host, a check rule or `trusted_base()`:
  that is a TCB change and returns to the operator.
- One site serves two families and splitting it changes control flow:
  stop to the Architect.
- A user-visible contract beyond the D0 (a JSON form, several diagnostics
  per run, warnings, a code-to-exit-status map) before the spec section.

## Residuals (named by the D0, not this WP)

- R4: capacity and invariant refusals carried as `Unsupported` are
  reported as an Unsupported verdict rather than a failure. That is a
  behavioural node.
- R5: `KernelError::Msg`, `ElabError::Internal` and forwarded
  `to_string()` reasons get one catch-all code until their text is typed.
- S1: deleting a domain's last row with its table entry is not caught
  in-tree. The Architect's merge review checks the ledger diff.
