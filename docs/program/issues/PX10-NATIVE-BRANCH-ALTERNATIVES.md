---
id: PX10-NATIVE-BRANCH-ALTERNATIVES
title: "The native static-transition plan assumes every continuation result has exactly one producer construct, so a result that joins two Vis alternatives chosen by a runtime scrutinee refuses at the detached-result seat, the caller cut and the invocation transport. PX10's withChild fixtures hit this at three sites. Discharge and transport such a result per alternative without weakening any single-producer guard for static results, and bring the three fixtures to native build with interpreter parity"
status: ready
owner: runtime
size: L
tier: T1
gate: architect
depends_on: [PX10-NATIVE-SITE-IDENTITY]
blocks: [PX10-NATIVE-SPAWN-PARITY]
github: null
origin: "Architect D0 for PX10-NATIVE-SITE-IDENTITY (evt_5dkq1wcbfvdxm): predicate P2 lies outside that node's objective, since no owner key discriminates which branch ran. Steward recut evt_2teptrm99sf03. L1 objective: the Linux ABI program. Steward-filed per COORDINATION section 2."
---

# A branch-joined result keeps each alternative

## Objective

A continuation result whose producer has N statically known
alternatives, selected at run time, is discharged and transported for
whichever alternative ran. The three PX10 slice-1 fixtures build
natively and match the interpreter. Every guard that requires exactly
one producer still holds for a static result.

## Settled inputs (Architect D0 on `86b7cac5e`, evt_5dkq1wcbfvdxm)

- **The predicate (P2).** Each of the detached-result seat, the caller
  cut and the invocation transport is a function of the result. At a
  branch join over a runtime scrutinee, each therefore refuses. The
  refusal is right for a plan with no runtime selection.
- **The guard population** has six one-producer guards:
  - `core.rs` ~12804 (seat);
  - `responses.rs` ~5566, ~5612 and ~5643 (consumer, boundary, cut);
  - `aggregates.rs` ~4770 and ~4842 (environment substitution).
- **The measured walls:**
  - smoke and live: W6, two causal calls on one result;
  - argv: W7, one caller transport matches two return cuts;
  - live: the authorized-environment transport at `aggregates.rs`
    ~4770.

  In each, the edges share the result and the producer alternative, and
  differ only in the producer construct.
- **A guard bypass is wrong code.** Suppressing only the seat guard
  built smoke natively, and the native run then trapped
  (`UnclassifiedRuntimeTrap`) where the interpreter succeeds.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

1. **AC-0 first (Architect, on the cut base after SITE-IDENTITY).** It
   measures both designs, then rules:
   - a result with two or more alternative producer transports as a
     dynamic boundary, driving its Carried word on the generic path;
   - or per-branch seat duplication.

   It names each of the six guards' disposition.
2. **One runtime branch** carrying the ruled mechanism and its tests.

## Acceptance

- **AC-1.** Smoke, argv_env_pipe and live_child build natively with the
  Process operations admitted in the test configuration. Each native run
  reports the same (request, outcome) pairs as the interpreter.
- **AC-2 (guards intact).** A static result with two producer edges
  still refuses at each of the six guards. A named row pins one.
- **AC-3 (negative, QA).** A two-alternative result consumed with the
  wrong layout refuses or runs correctly, and never traps. A mutant that
  drops the per-alternative discharge reddens a named row.
- **AC-4 (no regression).** The four bracket suites and the return-fold
  equivalence stay green.

## Stop conditions

- A kernel or trust change, or a post-fork callback.
- A third advancing stop on the native-lowering chain (§1a count 2).

Checks: 2 fired: the six cardinality guards are the Architect's
enumeration of the result-to-producer path. 8 fired: native build is a
proxy, since probe 21 built smoke and then trapped. AC-1 compares
traces with the interpreter, and AC-3 requires refusal or a correct run.
