---
id: RT-JOIN-SCALAR-PAIR-NONSCALAR-RESULT
title: "A dynamic source Match whose specialized result is a constructor with fields is planned NativeScalarPair, because the planner chooses the join representation from phase alone, so object emission refuses with 'Match: dynamic arms must produce scalar Int or Bool values'. Plan such a join from its result shape as well"
status: active
owner: runtime
size: M
gate: architect
tier: T1
depends_on: []
blocks: []
github: null
origin: "Architect evt_2dt20kx3h81pt, the RT-JOIN-PHASE-CASE-BINDER-CARRIED AC-0 result: that WP's witness is gone from main, and the nat row's first refusal is now this shape gap. Also the first refusal of rt_escape ESCAPE_FILE_THEN_READAT (:660), which the Steward placed in L1 (RT-NATIVE-SEQUENTIAL-BRACKETS). Size is provisional and is re-set at AC-0. Steward-filed per COORDINATION section 2."
---

# Join representation follows result shape

## Objective

`nat_fanout_escaped_resource_matches_interpreter` gets past source Match 1289
on `origin/main`, and the planner never plans a scalar pair for a join whose
result has fields.

## Settled inputs (Architect `evt_2dt20kx3h81pt`, at `4441a1422`)

- **The refusal.** `jump_planned_join_arm` calls `merge_scalar_branch`
  (`lowering/joins.rs:400`), which refuses at `:2699`.
- **The join.** Source Match 1289 is planned `NativeScalarPair`. Its arm
  lowered `Specialized(Constructor)`. Its scrutinee, child 1288, is a
  `PrimitiveCall` summarized `SpecializedOnly`, visited once under
  descriptor root 1449 with no var fallback. In the fixture it is `match
  buffer_span_budget span { Zero |-> Ret …; Suc m |-> Ret … }`, and both
  arms return a `Ret` program value.
- **The planner** maps `SpecializedOnly` to `NativeScalarPair`
  (`joins_traps.rs:529-533`). A pair lane holds Int, Bool, a structural
  Nat, an exit status or a nullary Bool constructor (`:2629-2690`), never a
  constructor with fields. The phase plane at 1289 is correct; it does not
  carry scalar-ness (check 5).
- **Repair family.** The planner keys the representation on result shape as
  well as phase, so a specialized non-scalar result plans `CarrierWord`.
  Boxing the arm into a carrier word in lowering is not admissible, because
  the plan stays the single authority.
- **Unmeasured:** whether the runtime IR at a join carries enough to
  classify result shape without lowering; how many joins on main the change
  re-plans; whether `ESCAPE_FILE_THEN_READAT` refuses at the same site.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch; do not build around it.

## Recut (Architect `evt_1xj5yh86d75st`)

The parked two-link plan (`evt_4mp5dtzn0f5rb`) is withdrawn.
`RT-CHECKED-JOIN-SITE-MATCH-POPULATION` closed without repair: no per-Match
Int/Bool site admits any of the 207 Ok merges.

- **Admission**, on main `f1ef79406`. `merge_scalar_operand`
  (`joins.rs:2527`) admits `RecursiveBackedge`, `Int`, `Bool`,
  `StructuralNat`, the zero-arg Bool constructors, `ProcessExitStatus`, and
  `exit_success` and `exit_failure`.
  - The exit constructors count only under the root-exit flag.
  - It strips `ITree::Ret` through `unwrap_terminal_ret` (`calls.rs:2653`)
    only when `has_checked_root_exit_representation()` (`mod.rs:13732`)
    holds, which needs a consumed root ExitCode site.
  - 154 of the Ok merges are `Constructor[args=1]`, logged before the
    unwrap, so they are `Ret(x)` arms.
- **Planner.** `joins_traps.rs:533` chooses `NativeScalarPair` from
  `ResultPhase` alone.
- **Shared predicate.** The representation is chosen on a plane that does
  not carry admission's keys: constructor identity, the scalar kind of the
  `Ret` payload, and the root-exit flag.
- **Closure.** One admission predicate. The planner uses it to choose the
  representation, and `merge_scalar_operand` uses or asserts it, with no
  second rule on either side.
- **Nat.** Under demote-all, Nat passes 1289 and stops at
  `ContinuationSpecialization: the detached-result seat projected 4
  undischarged causal calls onto one unit result`. That is a successor WP,
  not this one. So "Nat passes" is not this WP's acceptance.

## Deliverable

1. **AC-0, measure only, on main.** Run the six default targets plus
   `nat_fanout`. Extend the existing `DASM_C2_SCALAR_MERGE_OBSERVATION` hook
   (`joins.rs:159`) rather than adding a second log. For every
   `merge_scalar_operand` call, log:
   - (a) the constructor symbol before the unwrap;
   - (b) `has_checked_root_exit_representation()`;
   - (c) the lowered value kind after the unwrap;
   - (d) the payload's constructor symbol when (c) is still a constructor;
   - (e) the planner's `ResultPhase` and why it was chosen;
   - (f) ok or err.

   Answer one question: does {(a), (b), (d)} separate the 207 Ok merges from
   Err 1289 with zero overlap, and can the planner compute it statically? On
   1289, tell apart a non-scalar payload from a false root-exit flag that
   leaves `Ret` unstripped, because those are different fixes.

   Single-site experiment: demote only 1289 to `CarrierWord`, and report the
   next stop and the native-tree result.
2. **Then the Architect rules.** If the answer is yes, the repair is the
   shared predicate (size S or M, set at the ruling). If it is no, that is a
   frame stop back to the Architect. Re-point the nat row's ignore
   annotation at its current first refusal.

## Acceptance

- **AC-0** is reported in the WP thread with the (a)-(f) table per target,
  the separation verdict and the 1289 experiment. Each count names its log.
- **AC-1 (repair).**
  - All 207 Ok merges stay Ok, and 1289 no longer refuses at `:400`. Record
    its next first refusal verbatim.
  - Every default target and `rt_parity_native` stay at baseline at 4
    threads.
  - A planner test pins the predicate, and a phase-only plan fails it.
- **AC-2 (control).** Reverting the repair restores the verbatim 1289
  refusal.

## Stop conditions

- Any kernel, `trusted_base()` or spec change (an operator question).
- The predicate is not computable at the planner: stop to the Architect with
  the AC-0 evidence.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40` or
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY`, and never land `a7d46d6f2`.
