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

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch; do not build around it.

## AC-0 result (runtime `evt_449ksg86zp7xt`, ruled `evt_65ej3g70cs8gs`)

Evidence: `/workspaces/ken/local/rt-scalar-recut-ac0/`, at `18543f8e8`.

- All 269 `merge_scalar_operand` calls pair with planner rows. The 268 Ok
  operands are exit-code constructors (178) or `ProcessExitStatus` (90).
  **None is `ITree::Ret`.**
- 1289 is `Option::Some` with an Int payload, the root-exit flag is true, and
  the unwrap leaves it unchanged. Only constructor identity separates it.
- The root-exit flag and the post-specialization kind exist only at
  lowering, so the planner cannot mirror admission. The earlier "one shared
  admission predicate" closure is withdrawn.
- Demoting 1289 alone moves Nat to the `ContinuationSpecialization` wall (4
  undischarged causal calls onto one unit result), a successor WP. Native
  tree stays 7/7.

## Deliverable: the ruled repair (size M, T1)

Confined to `planning/static_transition/joins_traps.rs`, plus threading
`&NativeProcessSymbols` into it from the planner root
(`static_transition.rs:952`). No change to lowering admission.

- **Predicate.** `arm_forces_carrier(expr, symbols) -> bool` walks each
  source-join arm to its leaves along the structure `summarize_result_phase`
  uses (the `Match` case bodies, the `If` branches, the `Let` body).
  - It returns true iff some leaf is a `Construct` whose constructor is not
    `bool_true`, `bool_false`, `exit_success` or `exit_failure`.
  - It compares whole `RuntimeSymbol` values, never suffixes (CHECKS 10).
  - Every other leaf kind gives no opinion.
- **Rule at `:533-537`.** `SpecializedOnly` plans `NativeScalarPair` only
  when `!forces_carrier`; everything else plans `CarrierWord`. It is computed
  only for source joins. The existing merge (CarrierWord wins) is unchanged.
- **Negative only.** It can demote a join and never promotes one.
  `merge_scalar_operand` stays the fail-closed backstop.
- Re-point the Nat row's ignore annotation at its new first refusal.

## Acceptance

- **AC-1.** 1289 plans `CarrierWord`. Nat passes the dynamic-Match seam and
  stops at the named `ContinuationSpecialization` wall; record it verbatim.
  "Nat passes" is not this WP's acceptance.
- **AC-2 (zero collateral).** Rerun the AC-0 planner FINAL log over the six
  default targets plus Nat, and compare per (target, entry, origin) with the
  AC-0 evidence. Exactly one origin changes representation: 1289.
  `rt_parity_native` stays 186/186 at 4 threads, and
  `rt_native_tree_match_case_of_case` stays 7/7.
- **AC-3 (pins and mutations, in the `joins_traps` tests).**
  - Pins: an arm `Construct(option_some, [Int])` plans `CarrierWord`; a twin
    with `Construct(exit_success)` / `Construct(exit_failure, [code])` plans
    `NativeScalarPair`; a nested `Match` → `Let` → non-scalar `Construct`
    leaf plans `CarrierWord`.
  - m1 (every `Construct` scalar) reddens the Option pin and AC-1.
  - m2 (every `Construct` non-scalar) reddens the exit-code pin, and AC-2
    reports demotions.

## Stop conditions

- Any of the 268 Ok origins demotes: stop to the Architect with its leaf
  constructor.
- Any new `CarrierWord` in `rt_native_tree_match_case_of_case`
  (`RT-CARRIER-ROOT-EXIT-NESTED-MATCH-TRAP` is open).
- Any kernel, `trusted_base()` or spec change (an operator question).
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40` or
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY`, and never land `a7d46d6f2`.
