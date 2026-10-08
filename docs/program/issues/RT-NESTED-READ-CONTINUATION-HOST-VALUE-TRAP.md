---
id: RT-NESTED-READ-CONTINUATION-HOST-VALUE-TRAP
title: "A second readFile inside a selected arm, whose continuation inspects a host value (bytes_at or bytes_length of a read payload, or a match on the first read's captured response), builds natively and traps UnclassifiedRuntimeTrap { terminal_value: -1 } at a carrier-class or carrier-tag check, while the interpreter returns a value. Since cc0e6f2b6 the outer-Option rows build instead of refusing. Make each row run at parity or refuse at build"
status: ready
owner: runtime
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Adversary M8 finding evt_5zgtvn9jb0gkb on cc0e6f2b6 (RT-PRODUCER-MATCH-BORROWED-OPTION). Spec 42 section 5 and 45 section 4: a native build that succeeds runs at interpreter parity, or the program refuses at compile time. Fails closed (abort, no wrong value). Not the RT-BORROWED-INPUT-CARRIER-DURABILITY population (captured process input). Steward-filed per COORDINATION section 2."
---

# A nested read's continuation runs at parity or refuses

## Objective

Every row below either runs natively at interpreter parity on exit code,
stdout and effect trace, or is refused at build with a typed diagnostic.
No row traps `-1`.

## Settled inputs (Adversary `evt_5zgtvn9jb0gkb`, at `cc0e6f2b6`)

- **The minimal shape.** Inside `read_byte`, a second read in a selected
  arm: `Ok b2 |-> bind .. (readFile ..) (\again. match again { Err _ |->
  Ret 50; Ok more |-> BODY })`. Files `[]` and `[9]`, with both arms
  selected.
- **Trapping BODY forms and their sites** (sentinel per `require_i64`
  call site):
  - `bytes_at more 0` or `bytes_at b2 0`: `primitive.rs:130`, the
    `BorrowedOpaque` carrier-class check on `bytes_at` argument 0;
  - `eq_int (bytes_length more) 1`: `primitive.rs:111`, the same check on
    `bytes_length`;
  - a match on the first read's captured `read`: `mod.rs:8724`, where the
    `emit_carrier_tag` status is not `BOUNDARY_OK`.
  The interpreter returns 51, 9, 52, 53/54 or 56 on the same rows.
- **Controls that pass:** the same nested bind whose continuation only
  matches `again` and returns a captured value; the landed single-read
  shape.
- **Base behaviour.** With `core.rs` at `cc0e6f2b6^`, the minimal row
  traps too. The rows with an outer `bytes_at` match refused at build
  ("tree-producing match scrutinee is not Bool or a constructor"), so
  `cc0e6f2b6` moved them from refusal to trap.
- The Adversary's probe is `zz_adv_bopt.rs` (ken-cli), in its scratchpad.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

1. **D0 (measure only).** For each trapping row, report the carrier class
   and tag the lowering assigns to the second read's payload and to the
   captured first response at the site that later checks it, and where
   the class is lost. The Architect rules the repair from the D0: a
   carrier that keeps its class across the nested continuation, or a
   build-time refusal of the shape.
2. **The ruled repair.**

## Acceptance

- **AC-1.** Each trapping row, as a new test on files `[]`, `[0]`, `[9]`
  and `[255]`, runs at interpreter parity, or is refused at build with a
  typed diagnostic if the ruling chose refusal. No row exits through
  `UnclassifiedRuntimeTrap`.
- **AC-2 (control).** The two passing controls,
  `rt_producer_match_borrowed_option`, and the `rt_parity_native`
  population stay green.
- **AC-3 (mutation, QA).** Reverting the repair returns at least one AC-1
  row to the `-1` trap.

## Stop conditions

- The repair needs a held-ref move (`4b4c8565c`, `21c039918`,
  `7f1a04a40`, `wp/RT-BRACKET-PRODUCER-AUTHENTICITY`) or bracket-tree
  work.
- Any kernel, `trusted_base()` or spec change.
