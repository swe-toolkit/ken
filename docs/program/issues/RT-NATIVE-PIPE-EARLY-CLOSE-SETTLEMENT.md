---
id: RT-NATIVE-PIPE-EARLY-CLOSE-SETTLEMENT
title: "Closing a child pipe end in the body of withChild and then waiting runs on the interpreter but fails native-build with N0003, a planner invariant the compiler calls its own bug: the relay selector finds two ResourceRelease positions for one forwarded pipe and cannot tell them apart. Name the two positions from a D0, key the selector on the position identity the planner carries, and give native parity for early close on both pipe ends"
status: ready
owner: runtime
size: M
tier: T1
gate: architect
depends_on: []
blocks: [RT-NATIVE-RESOURCE-STREAM-WRITE-PROMOTION]
github: null
origin: "Adversary post-merge finding F1 on LANG-PROCESS-CHILD-PIPE-WRITE-HOME 943d14a06 (evt_4sqs1d163es91); Architect split ruling evt_4ry3fw55kaxhz names this node. L1 Linux ABI program. Steward-filed per COORDINATION section 2."
---

# Early close of a child pipe end on the native backend

## Objective

A Strict Ken program that takes a child pipe end, closes it in the body of
`withChild`, then waits for the child, builds natively and produces the
same terminal result and the same ordered canonical requests, outcomes and
bindings as the interpreter. This holds for both ends: `close_pipe_read`
on a `take_stdout` end, and `close_pipe_write` on a `take_stdin` end.

## Settled inputs (main `943d14a06`)

- **The failure.** `take_stdout`, `close_pipe_read`, then `wait_child`
  inside `withChild`, with the child running `printf x`, gives `ken run`
  exit 0. `native-build` instead fails with "error[N0003] ... native
  static transition planner invariant failed ... relay: forwarded op
  reaches more than one protocol position of its minting instance". The
  no-close control (`take_stdout` then `wait_child`) builds.
- **The guard.** The refusal is a deliberate fail-closed guard in the
  relay selector (`planning/static_transition/responses.rs:1585`-`:1594`).
  Its fan-in is the owner's response row plus each non-relay pending-Vis
  member's successor (`:1560`-`:1583`). It keeps the members whose
  `(effect_origin, operation)` equals the relay pair's, and it refuses
  when more than one matches.
- **The hypothesis, withdrawn.** It said the body's close and the
  bracket's own settlement of the same end are two `ResourceRelease`
  positions of one forwarded pipe. The D0 counted three. Spec 38
  §1.11.2 requires the bracket to settle an end the body closed early.
- **The precedent to avoid.** `RT-BRACKET-SETTLEMENT-PLANE` (merged)
  warns against "a first-match find_map keyed on effect_origin", which
  projects one ID for a shared static origin. This fix must not fall back
  to a first-match choice.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

The relay selector's key and whatever position identity it needs from the
planner, plus native parity rows for early close. No spec, catalog,
authority, trust or availability-label change. The interpreter is
unchanged.

## Deliverable

One runtime candidate.

## Acceptance

- **AC-0 (D0, before any planner code).** On `take_stdout` +
  `close_pipe_read` + `wait_child` inside `withChild`, name every
  matching member: owner, row ids, `effect_origin`, operation and the
  source site of each. Give the exact call or return context that the
  planner carries to separate them. The Architect rules the key on that
  D0. The measured population is three members (`evt_21j9nb6g4k6he`),
  and the rest of AC-0 is the Architect's D0-b, M1 to M4
  (`evt_63r00kec09cjx`).
- **AC-1.** Native builds and interpreter parity for early close on the
  read end, and, once `RT-NATIVE-RESOURCE-STREAM-WRITE-PROMOTION`
  Increment 1 has landed, on the write end.
- **AC-2.** The bracket's settlement of an already-closed end has the
  outcome spec 38 §1.11.2 states, and both engines agree on it exactly.
- **AC-3.** The no-close controls stay green, and reverting the new key
  to the old `(effect_origin, operation)` match reddens the AC-1 row with
  the N0003 refusal.
- **AC-4.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The members are ones the planner's carried context cannot separate.
  Stop to the Architect.
- The engines disagree on any canonical request or outcome. Report the
  first differing index with both events.

Checks: 2 fired. The guard and its fan-in are the Architect's read at
`943d14a06`, and AC-0 measures which members arrive before any key is
chosen. 8 fired: the AC-3 revert restores exactly the measured refusal,
so the row tells the two keys apart. 9 fired: the read-end reproduction
issues neither `take_stdin` nor `ResourceStreamWrite`, so this node
starts on `943d14a06` without waiting for the stream-write increment.
