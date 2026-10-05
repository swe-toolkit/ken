---
id: RT-JOIN-SCALAR-PAIR-NONSCALAR-RESULT
title: "A dynamic source Match whose specialized result is a constructor with fields is planned NativeScalarPair, because the planner chooses the join representation from phase alone, so object emission refuses with 'Match: dynamic arms must produce scalar Int or Bool values'. Plan such a join from its result shape as well"
status: merged
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
- **The plan stays the single representation authority** on every compile
  attempt. Boxing an arm into a carrier word in lowering is not admissible.

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

## Deliverable: lowering feedback into re-planning (size M, T1)

Architect `evt_16se76agavv64` withdraws the planner leaf rule of
`evt_65ej3g70cs8gs`. Source arm leaves are not the merged operand: lowering
composes eliminator frames into a join before it merges
(`lowering/core.rs:6464-6481` and `:6486-6531`). Each decision gets one
owner: lowering decides admission, the planner decides representation given
the joins lowering refused, and a driver re-plans until no join is refused.
No kernel, spec or `trusted_base()` change.

1. **Revert** the WIP `73947db34`'s `arm_forces_carrier`, its `symbols`
   threading and its three pins.
2. **Planner.** Add a `forced_carrier_joins: &BTreeSet<StaticOriginId>`
   variant of the planner entry; the existing entry delegates with an empty
   set. In `summarize_result_phase`, a forced source join gets
   `ResultPhase::CarrierRequired`, so enclosing joins see a carried result
   through the existing lattice.
3. **Lowering.** Record the refusal by identity, never by error text
   (CHECKS 10). A `scalar_operand_refused` flag is set only in
   `merge_scalar_operand`'s final refusal arm (`joins.rs:~2697`), and
   `jump_planned_join_arm` (`joins.rs:379`) records `join_plan.origin` in a
   `Cell` only when that flag is set. No other refusal records an origin.
4. **Driver** (as corrected by Architect `evt_70vtg3vb66bs9`).
   `compile_with_scalar_join_feedback` retries into a fresh module with the
   refused origin added to the forced set, and stops when a refusal names no
   new origin. It wraps exactly the three production program-backed callers:
   `artifact/mod.rs:86` (JIT program), `:142` (object program) and
   `artifact/api.rs:401` (`emit_bound_process_program_object_with_cranelift`,
   the Nat route).
   - api.rs:401 is rewired from `compile_expr_into_object_module` to
     `compile_program_expr_into_object_module`, which is behaviour-identical
     for that caller under process mode.
   - The seed lane (`mod.rs:119`) and every test caller stay single-attempt,
     with an empty set and a local `Cell`.
   - Assert that the refused origin names the same occurrence in the
     re-plan. A test-support counter records attempts and the final forced
     set per compile.

## Acceptance

- **AC-1 (Nat).** Through `ken_cli::build_native_program`, the api.rs:401
  wrapper records 2 attempts and forced = {1289}, and the 1289 refusal is
  gone. Record the new first refusal verbatim and
  re-point the ignore annotation at it. "Nat passes" is not this WP's
  acceptance.
- **AC-2 (zero collateral).** The six default targets stay at baseline,
  `rt_parity_native` stays 186/186 at 4 threads, and
  `rt_native_tree_match_case_of_case` stays 7/7. Every compile in them
  records 1 attempt and an empty forced set.
- **AC-3 (pins).**
  - Planner: on the `Option::Some(Int)` fixture, forced {root} plans
    `CarrierWord`, and so does a join whose arm returns that join. The empty
    set plans `NativeScalarPair`.
  - Driver: on the bound-process route, a 1289-shaped program compiles in
    exactly 2 attempts, and the `Cell` names that join's origin.
  - The planner forced-child and enclosing-parent pin from `62d8ebc80`
    stays.
  - A non-admission refusal (such as the `RecursiveBackedge` arm) leaves the
    `Cell` empty and does not retry.
- **Mutations.**
  - m1: drop the `CarrierRequired` line; the enclosing-join pin reddens.
  - m2: the driver ignores the `Cell`; Nat restores the verbatim 1289
    refusal.
  - m3: remove the flag set in the final arm; the driver pin reddens.

## Stop conditions

- Any default-target compile records more than one attempt: stop to the
  Architect with the origin.
- Any new `CarrierWord` in `rt_native_tree_match_case_of_case`
  (`RT-CARRIER-ROOT-EXIT-NESTED-MATCH-TRAP` is open).
- Any kernel, `trusted_base()` or spec change (an operator question).
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40` or
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY`, and never land `a7d46d6f2`.

## Hard-stop inventory (§1b)

§1a count: 1 (Architect `evt_16se76agavv64` on stop `evt_6yx4h3eq90esj`;
the entry-point stop `evt_6wya435ge849j` was non-advancing,
`evt_70vtg3vb66bs9`).
The shared predicate is that the planner chooses representation on a plane
that lacks lowering's admission keys.

1. A non-scalar join keyed on source arm-leaf constructor identity; source
   leaves are not the merged operand under eliminator composition
   (`core.rs:6464-6531`).

## Closeout

Merged `3695cd16a` from exact `bb22ef7c6` (PR from
`wp/RT-JOIN-SCALAR-PAIR-BOUND-PROCESS-FEEDBACK`, main push run
37288769802). Runtime QA `evt_5305rr281e701`, Architect `evt_1je587b64j3wh`
(crates only), Decision `dec_3amykkwkf1s39`.

- Lowering's non-scalar operand refusal feeds a bounded re-plan at carrier
  representation on the three program callers (JIT, object,
  bound-process). The seed lane and the expression entries stay
  single-attempt.
- AC-1: the Nat bound-process row runs in 2 attempts with `{1289}`
  forced. Its next refusal, ContinuationSpecialization on a multi-member
  projection, is the successor and starts its own count.
- AC-2 was relaxed for build-lock time (`evt_4mzqmhksbrnx9`): the 186-row
  native parity is CI's, QA did not rerun parity locally, and the default
  recorder summary came from the implementer's log.
- The first QA block (`02611250`) was a 256 MiB test stack with no stated
  peak. The respin states a sampled peak of 2,308 KiB at the site.
