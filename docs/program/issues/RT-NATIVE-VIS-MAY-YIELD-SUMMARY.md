---
id: RT-NATIVE-VIS-MAY-YIELD-SUMMARY
title: "Each native planning plane (the relay classifier, the census walk and the transport destination) computes which Vis constructs an ITree-valued occurrence may yield by its own lexical walk inside one function owner, and refuses a Call that is not a direct lexical-closure redex. A Vis routed through a named recursive proc, a bracket body closure or a specialization clone is invisible to every walk. Compute one shared per-function may-yield summary, with top at every open terminal, pinned by plan census and switching no consumer"
status: ready
owner: runtime
size: L
tier: T1
gate: architect
depends_on: [PX10-NATIVE-SPAWN-PARITY]
blocks: []
github: null
origin: "Architect ruling evt_146ymmv8mz9py on PX10-NATIVE-SPAWN-PARITY Increment B-closure, after the early Research hold evt_qkp7qcj87b3q and advisory evt_91y97hbf0w06: unit (a), framed as its own later WP; parity does not depend on it. Steward-filed per COORDINATION section 2."
---

# One may-yield summary for every planning plane

## Objective

The native planner has one answer to "which Vis constructs can this
ITree-valued occurrence yield", computed once and read by every plane.
It crosses lexical redexes, calls to named procs (recursion by least
fixpoint) and specialization clones, and it is top, "may yield
anything", at every open terminal. It lands with plan-census pins and
switches no consumer.

## Settled inputs (main `dff99e382`, Architect measurements at `928313aa6`)

- **The per-plane walks.** The relay classifier's source traversal
  (`responses/relay_settlement.rs`, `source_results_in_scope`) accepts a
  Call terminal only when `curried_redex_body` finds a direct application
  of one-parameter lexical closures; otherwise it refuses with "generated
  or unknown Call terminal". The census walk and the transport
  destination make the same one-owner assumption (D0-12, D0-12b, D0-5 to
  D0-7). This is §1b entries 6 to 9 of `PX10-NATIVE-SPAWN-PARITY`.
- **The measured population.** argv's and null's `child_body` binds
  `drain_pipe (Suc (Suc (Suc (Suc Zero)))) pipe buffer`, a named
  self-recursive proc, and their relay 1247 is refused at Call terminal
  1503. live binds `read_pipe` in the same position, and its relay 688
  is admitted with sources 988 and 932 (D0-13, evt_1j5h9g0whjg2m).
- **The higher-order routes.** `child_body` is passed to `withChild` and
  `buffer_body` to `withBuffer`, so a combinator's yield set depends on
  its argument.
- **A reader-less table.** `relay_routes`, built at
  `construction.rs:1594`, has had no reader since `5386dc871`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

The shared summary and its plan-census pins, in one runtime candidate.
No relay classifier, census walk or transport destination reads it in
this WP. Each moves onto it later as its own step.

## Acceptance

- **AC-0 (D0).** The Architect specifies a development on the landed
  base: the patch, and the named-binary commands with a bounded timeout.
  The runtime implementer runs it on scratch and reports. The Architect
  rules from that report, and does not build or test. It rules:
  - the lattice and the fixpoint site, after specialization so that each
    clone has its own summary;
  - the treatment of a body closure passed to a bracket combinator:
    effect-polymorphic or polyvariant;
  - which terminals are open, enumerated from the `RuntimeExpr` call
    forms, not from 1503 alone;
  - whether `relay_routes` is deleted here or left to the consumer step.
- **AC-1.** Plan-census pins, with D0-8 to D0-13's measurements and
  Increment B-closure's live trace as expected values: argv's and null's
  1503 resolves through `drain_pipe`; live's 688 set is the one
  B-closure's trace row measured; the owner member populations on smoke,
  argv, live and null equal D0-12b's.
- **AC-2.** A mutant that returns a smaller set than top at an open
  terminal reddens a named pin. Every native plan output is unchanged,
  because no consumer reads the summary.

## Stop conditions

- A kernel or trust change.
- A pin that can be met only by switching a consumer.

Checks: 2 fired: the guard at `curried_redex_body`'s None arm is
deliberate, so this WP adds the summary that a consumer can later read
instead of the guard, and AC-0 enumerates every open terminal form. 6
fired: the population is the four fixtures' relay lines, one relay each
in argv, null and live, counted by D0-13. 8 fired: AC-2's mutant
produces a smaller set at an open terminal, which the sound summary
does not.
