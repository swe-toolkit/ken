---
id: RT-PRODUCER-MATCH-BORROWED-OPTION
title: "A tree-producing Match whose scrutinee lowers to a BorrowedOption (an Option match on bytes_at inside read_byte) dispatches natively, as the ordinary Match chain already does, instead of refusing in the producer operand chain for want of a BorrowedOption arm"
status: merged
owner: runtime
size: S
gate: architect
tier: T1
depends_on: []
blocks: []
github: null
origin: "Architect carry 2026-09-28 (evt_1mz68b0assf2d) on the RT-NATIVE-TREE-MATCH O1 observation (evt_4ahm641a7q49h): a measured, fail-closed gap outside that WP. Steward-filed per COORDINATION section 2."
---

# BorrowedOption in the producer Match chain

## Objective

A producer Match on a borrowed `bytes_at` Option builds and runs natively, and
the interpreter agrees.

## Settled inputs (Architect `evt_1mz68b0assf2d`, read at WIP `26fcaa31e`)

- **The refusal.** The FS direct-exit fixture's helper `read_byte` has an
  Option match on `bytes_at` (Match origin 367). It refuses at the producer
  chain's fall-through (`core.rs:6610` at `26fcaa31e`) with `BorrowedOption`
  selected.
- **The gap.** The producer Match's operand dispatch (`core.rs` ~6395-6605 at
  `26fcaa31e`) has arms for Carried, Bool, HostResult, DynamicConstructor,
  BoundedNat, StructuralNat and Constructor. It has no BorrowedOption arm.
  The ordinary Match chain has one (`core.rs:14944` on `origin/main`).
- **The witness.** The FS try2 fixture was written on the TREE-MATCH WIP and
  dropped from that WP. This node recreates it.
- **Two populations share `:6610`.** ProcessExitStatus (the rt_escape rows at
  their D0 origins) and BorrowedOption (origin 367) both refuse there, so a
  refusal-text match cannot tell them apart.
- **Unmeasured:** whether the fixture reaches origin 367 on `origin/main`
  without the TREE-MATCH route, and which other programs do.

## Deliverable

One repair, ruled by the Architect. With it, the producer chain handles a
BorrowedOption scrutinee with the same meaning as the ordinary chain, and every
other operand class keeps its current dispatch or refusal.

## Acceptance

- **AC-0 (probe, then ruling; no product change).** Recreate the witness and
  measure its first refusal on the current base, keyed on origin plus selected
  kind. Say whether the ordinary chain's BorrowedOption handling can be reused
  or needs a producer form. The Architect rules before any edit.
- **AC-1.** The witness runs natively on both of its Option arms, with inputs
  that select each arm, and matches the interpreter's output and exit code.
- **AC-2 (control).** Removing the new arm brings back the AC-0 refusal at
  origin 371 with BorrowedOption selected (re-keyed from the discarded WIP
  fixture's 367, Architect `evt_6w6dn563hddtx`). The new arm matches only
  `Lowered::BorrowedOption`. The two rt_escape ProcessExitStatus rows and the
  targeted runtime and ken-cli suites stay green. On the current base no
  ProcessExitStatus population reaches the producer fall-through (both rows
  select HostResult), so that refusal is not a control here (Steward
  `evt_xj1pg26866sx` reply).

## Stop conditions

- Any kernel, `trusted_base()` or spec change (an operator question).
- A witness that needs the TREE-MATCH route to reach origin 367 waits for that
  WP to land: report it and stop.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40` or
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY`.

## Closeout

Merged `cc0e6f2b6` from exact `054932299` (PR #4602). Runtime QA
`evt_6k3sn999hevmb` (AC-2 as amended `evt_1baz5z4ebt7vf`), Architect
`evt_2fj5q8ta83mnf`, Decision `dec_4nsfd82jkxzyc`. A dedicated
BorrowedOption arm in the producer Match dispatch lowers each selected arm
through the computational producer path. The `read_byte` Option witness
runs natively on both arms at interpreter parity in
`rt_producer_match_borrowed_option.rs`. The full parity population was gated
by CI.
