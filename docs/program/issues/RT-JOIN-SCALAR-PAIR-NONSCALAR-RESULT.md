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

## Deliverable

One planner repair, ruled by the Architect at AC-0, after which a join with a
specialized non-scalar result plans a carrier representation. Re-point the
nat row's ignore annotation at its current first refusal and this WP's id
(it still quotes join 1244).

## Acceptance

- **AC-0 (probe, then ruling; no product change).**
  - A census of every source join planned `NativeScalarPair` with an arm
    whose lowered operand is not a scalar-pair kind. It covers the ken-cli
    native suites and runs the nat row and `ESCAPE_FILE_THEN_READAT`
    explicitly. Name each join and its refusal.
  - Say whether the IR at the planning seam carries the result shape. If it
    does not, name the plane where that type is still known.
  - The Architect rules the repair and sets the size.
- **AC-1.** The nat row gets past 1289, and so does `ESCAPE_FILE_THEN_READAT`
  if the census places it here. Record each one's next first refusal
  verbatim. A row that runs natively must match the interpreter, and it
  stays ignored unless it goes green.
- **AC-2 (control).** Reverting the repair restores the verbatim 1289
  refusal. Every join the census named as scalar keeps `NativeScalarPair`,
  pinned by a planner test that a phase-only plan fails. The targeted runtime
  and ken-cli suites stay green.

## Stop conditions

- Any kernel, `trusted_base()` or spec change (an operator question).
- The result shape is known only below the planner, so the repair needs a
  new plane: stop to the Architect with the AC-0 evidence.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40` or
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY`.
