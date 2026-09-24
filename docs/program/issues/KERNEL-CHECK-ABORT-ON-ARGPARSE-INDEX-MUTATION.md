---
id: KERNEL-CHECK-ABORT-ON-ARGPARSE-INDEX-MUTATION
title: "Checker totality finding: a one-token mutation of argparse_parse_tokens makes ken check abort with a process stack overflow (rc=134) instead of returning a KernelRejected at the failing obligation; locate the diverging query and make it return a verdict"
status: draft
owner: kernel
size: M
gate: architect
tier: T1
depends_on: []
blocks: []
github: null
origin: "Research evt_579nbhknhvcnn measured the abort while advising CAT-ARGPARSE-LAWS HS3; Architect evt_4xhp1k4y4w8sq directed a finding node ('a resource blowup on small code is a checker bug'). Not on an authorized lane; unscheduled. Steward-filed per COORDINATION section 2."
---

# A wrong proof aborts the checker instead of failing

## Fixed inputs -- measured by Research at `acbebbfa9`

- Checkpoint `acbebbfa99530277b8dd57c51e012832d3b63b5c` (base
  `d7f6c2bcf`), `catalog/packages/Application/CommandLine/ArgParse.ken.md`,
  with both `ken ignore` fences still ignored. The unmodified file checks.
- Changing `Suc (Suc index)` to `Suc index` after a value option in
  `argparse_parse_tokens` makes `ken check` abort with a stack overflow,
  `rc=134`. On an 8x stack it is SIGKILLed, `rc=137`.
- A proof-side `Suc index` to `index` in the Flag arm does the same (fence
  state not recorded; D0 re-measures).
- **Second reproducer, outside the `Cons` bridge** (Architect
  `evt_58hr8a3vegsvc`): on WIP `f7e83e2178c834b0afc2764b5c3b3c3b129a5b8d`,
  changing the `Nil` arm's `index` to `Suc index` in `argparse_parse_tokens`
  (`:363`) also aborts. Log `/tmp/argparse-law2-nil-index-mutation.log`
  (perishable).
- It fails closed: nothing wrong is accepted. But the check names no
  obligation, and a small file exhausts the process.
- The Architect's hypothesis, unmeasured: conversion diverges on a
  non-convertible `Refl` between recursive definitions. The landed
  `KERNEL-CONV-RECURSIVE-HEAD-TOTALITY` bounded one such case; whether this
  is a second case of that boundary or a different cause is unknown. With
  the fences ignored, the one checked obligation relating
  `argparse_parse_tokens` to a copy of itself is
  `full_validation_cons_bridge`, proved by `Refl`. That is one candidate,
  not the lead: the `Nil`-arm reproducer lies outside the bridge.

Treat anchors as perishable. If a fixed input is false on the landed base,
stop and report the mismatch; do not build around it.

## Deliverable

First, reduce the reproducer and name the diverging query (conversion,
whnf, evaluation or elaboration) and its cause. Then make that query return
a verdict, so `ken check` reports `KernelRejected` at the obligation.

## Acceptance

- **AC-1 (red first).** The reduced reproducer aborts at base and gives a
  `KernelRejected` naming its obligation on the candidate.
- **AC-2 (no false accept).** The unmutated ArgParse checkpoint still
  checks, and the reproducer is still rejected. Existing conversion verdicts,
  including the `KERNEL-CONV-RECURSIVE-HEAD-TOTALITY` matrix, are unchanged.
- **AC-3.** Targeted builds only, through `scripts/ken-cargo`. No-regression
  means green in CI.

## Stop conditions

Stop if the repair needs a new reduction rule, an equality certificate or
any trust change, or if the reduced cause is not in the checker.
