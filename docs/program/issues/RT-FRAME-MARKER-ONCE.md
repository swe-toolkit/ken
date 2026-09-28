---
id: RT-FRAME-MARKER-ONCE
title: "Native execution of a host-effect tree that one checked frame marker serves more than once: the two px7n nested-eliminator rows and the checked double bind build and run natively and agree with the interpreter, instead of refusing in object emission because the oriented subcontinuation plan consumes one checked Runtime frame marker more than once"
status: active
owner: runtime
size: M
gate: architect
tier: T1
depends_on: [RT-IGNORED-ROWS-NEXT-GROUP]
blocks: []
github: null
origin: "Measured at 21fd46dc by the RT-SRCBODY-BIND-ORDER D10 differential (evt_2jc88hbzfskpm) as pre-existing base debt. RT-IGNORED-ROWS-NEXT-GROUP's occurrence-key repair moves both px7n rows to this refusal (AC-0 ruling evt_49fjtm9sen7bh), and its CI respin adds the checked double bind as a third witness (Architect evt_6ne90bptkxg33). Operator L1 directive 2026-09-17 (clear the ignored tests). Steward-filed per COORDINATION section 2."
---

# One checked frame marker, consumed more than once

## Objective

The two `px7n` rows run green and un-ignored natively, and the checked
double bind runs natively with the interpreter's observation.

## Settled inputs (on the NEXT-GROUP repair, `07eb1971e` and its respin)

- **The refusal**, verbatim:
  `ObjectEmission/checked_process_object: unsupported runtime-IR lowering:
  OrientedSubcontinuationPlanV1: checked Runtime frame marker was consumed
  more than once`.
- **The rows**, in `crates/ken-cli/tests/px7n_nested_computational_eliminator.rs`:
  `nested_ok_payload_reaches_both_real_executors` (`:150`) and
  `nested_err_payload_reaches_both_real_executors` (`:171`). Both are
  relabelled to this node by NEXT-GROUP.
- **The smallest witness**, in
  `crates/ken-cli/tests/rt_selected_pending_call_admission.rs`: the checked
  double bind, `let p = body MkUnit in bind p (λ_. bind p (λ_. exit))`.
  - It runs one HostIO tree value twice. That is a legitimate program: a
    tree is a description, so its effects run twice.
  - On the respin, planning yields exactly 2 admission rows. Object emission
    then refuses as above, and no native artifact is produced
    (runtime-implementer `evt_5sa7g29th903g`).
  - Interpreter target: effect trace `ConsoleIsTerminal, ConsoleWrite,
    ConsoleWrite`, stdout `captured\ncaptured\n`, exit 0.
  - The test is a transition sentinel (renamed on the respin to
    `checked_double_bind_admits_then_refuses_at_frame_marker`). This node's
    repair turns it red, and it becomes a native-versus-interpreter parity
    pin.
- **A fourth witness** (Steward `evt_qej118yh57f8`). The TREE-MATCH
  shared-bind ExitCode pin (WIP `4f101bba0`) reaches the same refusal after the
  TREE-MATCH route fires (Architect `evt_1mz68b0assf2d`). AC-0 re-measures it.
  This node does not repair TREE-MATCH. That WP runs its own parity pin once
  this node lands.
- **Unmeasured:** which plan element consumes the marker twice, from which
  source occurrence, and whether a marker is owed once per run of a shared
  tree value or once per tree. Also unmeasured: whether another blocker sits
  behind it for any of the three programs.

## Symptom inventory

1. The exactly-once rule admitted a same-path duplicate activation. It was
   keyed on the marker key, not on activation-event identity, and it carried
   an invented re-entry positive with no reachable shape (Architect
   `evt_11hdhkc9zp2wg`, on hard stop `evt_1yd2ay3s2d7r`).
2. Terminal classification by emitter registration assumed ledger access at
   every abort emitter (the static `require_*` family, about 146 callers). It
   was keyed on emitter provenance instead of the ABI status the Function
   returns (Architect `evt_1chmsz4k1se05`, on hard stop `evt_1hy1fsf51wkrc`).
3. Mutation acceptance assumed one rule per control. Rules (a), (b) and (d)
   overlap on `control.rs:807`, so the rule set was never executed against
   its own controls (Architect `evt_1ew4w7jzdz1cm`, on hard stop
   `evt_750pw6w085aev`). The shared predicate: the rulings specified rules and
   acceptance without running them against the concrete controls and
   populations they govern. Research advisory requested at stop 3.

## Deliverable

One repair, ruled by the Architect. With it, the three programs build and
run natively, the interpreter agrees on each, and a marker that truly has a
second consumer still refuses.

## Acceptance

- **AC-0 (probe, then D0; no build).**
  - Re-measure the three programs on the landed NEXT-GROUP respin (Check 4).
  - At the refusal, report the marker's identity, both consuming sites, and
    the source occurrence each serves.
  - Propose the repair. Name what the interpreter does on the same term, and
    which duplicate consumption the fixed plan must still refuse.
  - The Architect rules before any build.
- **AC-1.** Both `px7n` rows pass un-ignored on both engines. The double-bind
  sentinel is replaced by a parity pin: native stdout, exit code and effect
  count equal the interpreter's (two writes).
- **AC-2 (control).** Reverting the repair returns each program to the AC-0
  refusal. A plan that consumes one marker at two sites serving the same
  occurrence still refuses. Rows outside the three keep their current
  refusal.

## Stop conditions

- Any kernel, `trusted_base()` or spec change (an operator question).
- A program needs a second, independent repair behind this one: land what
  the one repair clears, relabel the rest, and report.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40` or
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY`.
