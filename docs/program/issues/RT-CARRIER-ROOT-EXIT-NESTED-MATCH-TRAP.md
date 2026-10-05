---
id: RT-CARRIER-ROOT-EXIT-NESTED-MATCH-TRAP
title: "A root-exit nested Match lowered as CarrierWord builds but traps natively with UnclassifiedRuntimeTrap { terminal_value: -1 }, where the NativeScalarPair route passes 7/7. Find whether the carrier word reaching emit_result has the wrong boundary tag, and fix the CarrierWord route"
status: merged
owner: runtime
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Architect carry evt_1xj5yh86d75st, a possible wrong-code defect found by the RT-CHECKED-JOIN-SITE-MATCH-POPULATION M3 demote-all experiment (evt_5337gyhq3akdz, evidence /workspaces/ken/local/rt-checked-join-match-m123/). A runtime-correctness question independent of SCALAR-PAIR's representation choice. Steward-filed per COORDINATION section 2. Measured at origin/main f1ef79406."
---

# CarrierWord at a root-exit nested Match keeps parity

## Objective

A root-exit nested Match whose join is planned `CarrierWord` runs natively
with the same result as the interpreter and as its `NativeScalarPair`
lowering.

## Settled inputs (Architect `evt_1xj5yh86d75st`)

- **The witness.** Under demote-all (`joins_traps.rs:533` plans
  `CarrierWord` for every `SpecializedOnly` Match, with no other change),
  `console_direct_exit_nested_match_uses_existing_route`
  (`crates/ken-cli/tests/rt_native_tree_match_case_of_case.rs:204`) builds.
  Its native run then traps with `UnclassifiedRuntimeTrap { terminal_value:
  -1 }`. Baseline on main is 7/7.
- **Why it matters.** `CarrierWord` is the production route for
  `CarrierRequired` joins, and SCALAR-PAIR's repair may move joins onto it.
  A root-exit join on that route that traps where the scalar route passes is
  a parity break.
- **Admission context.** At a root exit, `merge_scalar_operand`
  (`joins.rs:2527`) strips `Ret` only under
  `has_checked_root_exit_representation()` (`mod.rs:13732`). The carrier
  route reaches `emit_result` (`calls.rs:2709`) without that unwrap.
- **Unmeasured:** whether the trap needs every join demoted or only one;
  which site; and what boundary tag the carrier word carries at
  `emit_result`.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

1. **AC-0, at the start of the repair.**
   - Minimize the witness to the single demoted site that reproduces the
     trap (demote one join at a time, on main).
   - At that site, log the carrier word reaching `emit_result`: its boundary
     tag and payload, against the `NativeScalarPair` lowering of the same
     program.
   - The Architect rules the repair.
2. **The ruled repair** in the `CarrierWord` route. The planner stays the
   single authority on representation, so the repair does not re-plan at
   lowering.

## Acceptance

- **AC-1.** With the minimized site forced to `CarrierWord`, the test passes
  natively and matches the interpreter. A focused native test pins that
  configuration. The pin must fail on main before the repair (CHECKS 8).
- **AC-2.** `rt_native_tree_match_case_of_case` stays 7/7 on the default
  plan. The six default targets stay at baseline at 4 threads. The full
  `rt_parity_native` suite is CI's: it is not run locally (Steward
  `evt_6fh5hhabpjyvx`), and any parity rows the repair touches are run by
  name.
- **AC-3 (falsifier).** Reverting the repair brings back the verbatim trap
  on the pin.

## Stop conditions

- The trap needs more than one demoted site to reproduce, and they share no
  route: stop to the Architect with the minimization.
- The fix needs a kernel, trust or spec change.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40` or
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY`, and never land `a7d46d6f2`.

## Closeout

Merged `77c6b0041` from exact `ecb9c26be` (PR #4523 from
`wp/RT-CARRIER-ROOT-EXIT-NESTED-MATCH-TRAP`, main push run 37339510488).
Runtime QA `evt_37dpe5s7rb12t`, Architect `evt_7tnmah846h86a`, Decision
`dec_vq7zgm4srvyk`.

- The trap was the checked root's ExitCode decoder in `units.rs`. It
  required the carrier's boundary tag to be `PersistentGround`, but the root
  may read invocation-owned constructor nodes. It now requires the node class
  `Constructor` (Architect `evt_4wvxm7657qvgw`).
- A scoped `cfg(test)` planner hook forces the minimized site to
  `CarrierWord`. The new pin in `rt_native_tree_match_case_of_case.rs` is a
  durable parity invariant for that route, 8/8 with the default rows.
  Restoring the old tag check traps on byte 2.
- The c91 unit-IR fixture was recaptured; its only semantic hunk is the
  class check.
- The L1 successor is `RT-NAT-FANOUT-DETACHED-MULTI-MEMBER`.
