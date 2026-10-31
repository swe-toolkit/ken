---
id: PX10-NATIVE-BRANCH-ALTERNATIVES
title: "The native static-transition plan assumes every continuation result has exactly one producer construct, so a result that joins two Vis alternatives chosen by a runtime scrutinee refuses at the detached-result seat, the caller cut and the invocation transport. A producer call whose transport sources mix a predeclared and a specialization owner refuses as a mixed-owner relay. PX10's withChild fixtures hit one or the other. Lift both refusals for these shapes without weakening any guard for static results, and bring the three fixtures to native build with interpreter parity"
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
whichever alternative ran. A producer call whose continuation sources
mix owners is transported as well. The three PX10 slice-1 fixtures build
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
- **The measured walls** (Architect evt_2vjqrxcmee6ra, on SITE-IDENTITY
  without the W3 key):
  - smoke: P2, two causal calls on one result. The edges share the
    result and the producer alternative and differ only in the producer
    construct.
  - argv and live: W3, `responses.rs:3825`, a relay source in a
    mixed-owner transport stage (sources 1724 and 1228). Each stage
    mixes predeclared and specialization owners at one producer call.
    `rt_performing_mixed_owner` pins that refusal on a fixture of the
    same shape.
  - W7 (argv) and `aggregates.rs` ~4770 (live) appeared only after an
    over-admitting W3 key, so they are unverified as real walls.
- **A guard bypass is wrong code.** Suppressing only the seat guard
  built smoke natively, and the native run then trapped
  (`UnclassifiedRuntimeTrap`) where the interpreter succeeds.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

1. **AC-0 first (Architect, on the cut base after SITE-IDENTITY).** It
   measures whether one mechanism closes both P2 and W3, the case of
   continuation sources that are not uniform at one result or one call.
   If not, it names the split. For P2 it rules between:
   - a result with two or more alternative producer transports as a
     dynamic boundary, driving its Carried word on the generic path;
   - or per-branch seat duplication.

   It names each of the six guards' disposition, and whether
   `rt_performing_mixed_owner`'s shape stays refused.
2. **One runtime branch** carrying the ruled mechanism and its tests.

## Acceptance

- **AC-1.** Smoke, argv_env_pipe and live_child build natively with the
  Process operations admitted in the test configuration. Each native run
  reports the same (request, outcome) pairs as the interpreter.
- **AC-2 (guards intact).** A static result with two producer edges
  still refuses at each of the six guards. A named row pins one.
  `rt_performing_mixed_owner` stays green, unless AC-0 rules its shape
  lawful and replaces the pin with a parity row.
- **AC-3 (negative, QA).** A two-alternative result consumed with the
  wrong layout refuses or runs correctly, and never traps. A mutant that
  drops the per-alternative discharge reddens a named row.
- **AC-4 (no regression).** The four bracket suites and the return-fold
  equivalence stay green.

## Stop conditions

- A kernel or trust change, or a post-fork callback.
- A third advancing stop on the native-lowering chain (§1a count 2).

Checks: 7 fired: W3's static-owner key reddened
`rt_performing_mixed_owner`, so AC-2 keeps that pin unless AC-0 rules
otherwise. 2 fired: the six cardinality guards are the Architect's
enumeration of the result-to-producer path. 8 fired: native build is a
proxy, since probe 21 built smoke and then trapped. AC-1 compares
traces with the interpreter, and AC-3 requires refusal or a correct run.
