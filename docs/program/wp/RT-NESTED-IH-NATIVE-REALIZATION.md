# WP frame — RT-NESTED-IH-NATIVE-REALIZATION D3-D5 (source-join owner-partition correction)

> The completing D3-D5 slice of RT-NESTED-IH-NATIVE-REALIZATION (node
> `docs/program/issues/RT-NESTED-IH-NATIVE-REALIZATION.md`). Runtime lane,
> operator kernel-chain priority (`evt_7nkzsy27p7npw`: "Go with A, unblock the
> kernel chain"). Owner: runtime. Size: M. Tier: T1. Gate: none. Architect
> (`evt_1nkx3f30hqp9y`) is the REQUIRED reviewer on the candidate (a
> soundness-bearing producer fix to the owner partition). Builds on
> RT-REALIZED-BACKEDGE-SOURCE-CONTINUATION (merged `897f1ea6a`) and
> RT-CHECKED-IH-REALIZATION-AUTHORITY (merged `e68ecd79`). On land, this node's
> D3-D5 complete and `KERNEL-NESTED-IND` unblocks.

## Mechanism (Architect ruling `evt_1nkx3f30hqp9y`, grounded @ `4c8f44066`)

The parent frontier RT-REALIZED-BACKEDGE advanced to is `joins.rs:2140`
`source join StaticOriginId(50) was classified outside its owning function`.
This is a PRODUCER fix — correct the owner partition — NOT a consumption patch
and NOT any widening.

Root: two authorities disagree, and one is stale.
- OWNERSHIP (authoritative, static): `required_join_origins(function)`
  (`joins_traps.rs:675`) admits an origin iff
  `semantic.function_owner(origin) == Some(function)` (`semantic_ir.rs:2097`);
  `function_owner` reads the owner partition computed by
  `partition_function_units` (`semantic_ir.rs:1231-1414`) — a flood-fill over
  transfer/continuation edges from function-unit seeds, with call / `StaticBody`
  / `DeclarationCall` edges as partition WALLS. Its docstring: "a projection of
  the already-validated occurrence population + semantic owner partition; cannot
  add or omit a join by maintaining a second caller inventory."
- CONSUMPTION (no owner check): `consume_join_plan` (`joins.rs:1798`), reached
  via `enter_source_occurrence_plan` (`mod.rs:7060`), inserts whatever origin
  the LIVE traversal reaches into `function_local.consumed_join_origins`.
- The realized checked-IH recursor edge is a CFG TAIL-JUMP
  (`Lowered::RecursiveBackedge` = "control already departed"), NOT a call.
  Realization is IN-PLACE at the source `Match` seat (the RT-REALIZED-BACKEDGE
  forward), so post-realization join 50 physically lowers into the ENCLOSING
  specialization F's CFG via the `iterative_composition` descent
  (`core.rs:3382-3459`, gated on `Specialization` + checked-IH environment
  transports), and F finalizes against `required_join_origins(F)`
  (`units.rs:8069`). But the partition still WALLS 50 to the inner recursor unit
  G, because the recursor entry is classified as a call/body boundary rather
  than the transfer edge it became. The partition is STALE w.r.t. the
  realization the PLANNER ITSELF elected (the `OrientedSubcontinuationPlanV1` is
  derived at planning/erasure time — planning knows the capsule will be
  realized).

RULING — the correct classification: StaticOriginId(50)'s owning function IS F
(the enclosing specialization), because that is where its merge block physically
lowers after in-place realization. Consumption in F is correct; the partition
must be corrected to agree. Concretely, in `partition_function_units` / the
owner descriptor:

1. The realized checked-IH recursor ENTRY edge must be classified as a TRANSFER
   edge (so F's seed floods ownership across it to 50 and the realized inner
   subtree), NOT a `StaticBody` / `DeclarationCall` wall.
2. The realized inner recursor must NOT seed a separate competing function unit
   (no dead G emitted) — otherwise the double-owner invariant
   (`semantic_ir.rs:1385` "planned node is owned by more than one function
   unit") would fire.

Both decisions key on the SAME compiler-owned fact that authorized the
realization (the oriented plan / checked-IH realization marking) — so ownership
stays compiler-derived and kernel-checkable, never caller-authored. Then
`required_join_origins(F) ∋ 50` = its consumption site, and disposition stays
consistent: `source_join_origins_in_owner_subtree` (`joins_traps.rs:709`, which
HONORS owner walls at `:724`) now walks 50 under F's subtree, so
statically-unselected-branch disposition is unaffected.

Why NOT the consumption direction: 50 does not live in a separately-emitted
function — the `iterative_composition` witness lowers the nested scrutinees
INLINE in F's own body pass, and `RecursiveBackedge` means the tail edge was
already emitted in F. Making F "disposition 50 across the boundary to G" would
be fiction (G's CFG has no 50 block) and would merely relocate the stop. The
partition is the single truth feeding BOTH the `required` set and disposition;
correct it once and both agree.

## Fixed inputs (Architect-measured @ `4c8f44066`; re-confirm at your D0 cut)

Line numbers drift — re-measure at D0. The Architect grounded these himself.

- The frontier refusal: `joins.rs:2140`, verbatim
  `source join StaticOriginId(50) was classified outside its owning function`.
- THE EDIT SITE: `partition_function_units` (`semantic_ir.rs:1231-1414`) / the
  owner descriptor — the edge-classification of the realized-recursor entry.
- `required_join_origins` (`joins_traps.rs:675`); `function_owner`
  (`semantic_ir.rs:2097`).
- double-owner invariant (`semantic_ir.rs:1385`).
- `consume_join_plan` (`joins.rs:1798`); `enter_source_occurrence_plan`
  (`mod.rs:7060`).
- `iterative_composition` descent (`core.rs:3382-3459`, `Specialization` +
  checked-IH env transports); F finalize (`units.rs:8069`).
- `source_join_origins_in_owner_subtree` (`joins_traps.rs:709`, owner walls
  `:724`).
- `validate_function_units` (`semantic_ir.rs:2395`, recompute-and-compare).
- The landed RT-CHECKED-IH D4 positional-ABI vector (the recursor's oriented
  frame / slots / parents / calls) — the AC-ABI-PIN reference, plus the node's
  own deferred positional `#[cfg(test)]` pin.

## Deliverable

Correct the owner partition so the realized checked-IH recursor entry is a
transfer edge and `StaticOriginId(50)` (with the realized inner subtree) is
owned by EXACTLY the enclosing specialization F — no dead competing G unit. The
edit is in the planner's owner partition (`partition_function_units` / the owner
descriptor), keyed on the compiler-owned oriented-plan / checked-IH realization
marking. The ordinary-`Match` selector, the `source.rs` catch-all, and all
values/carriers stay untouched.

## Acceptance criteria (Architect-pinned)

- **AC-COMPLETE-PARITY (the goal):** native execution COMPLETES and the native
  result == interpreter == `Nat 3`. Removing the stop is necessary but NOT
  sufficient — parity is the real gate.
- **AC-SINGLE-OWNER:** `validate_function_units` (`semantic_ir.rs:2395`,
  recompute-and-compare) passes; the realized subtree (incl. 50) is owned by
  EXACTLY F — assert no double-owner fires and no dead G unit is emitted.
- **AC-DISPOSITION-CONSISTENT:** statically-unselected branches still disposition
  correctly; `source_join_origins_in_owner_subtree(F root) ∋ 50`.
- **AC-ABI-PIN (the deferred "wrong answer in the first release that removes the
  stop" hazard the node records):** retain the RT-CHECKED-IH D4 positional-ABI
  `#[cfg(test)]` pin — the recursor's oriented frame / slots / parents / calls
  == the landed D4 vector. Parity at `Nat 3` PLUS an unchanged positional-ABI
  vector is what proves the completion is the RIGHT one, not an accidental one.

## Forbidden boundary (each closed route, inherited)

- No selector or catch-all widening: the ordinary-`Match` selector (`core.rs`)
  and the `source.rs` catch-all are UNTOUCHED — the edit is in the owner
  partition.
- No second join inventory / no `NativeJoinPlanV1` collection (it stays
  withdrawn) — correct the EXISTING owner partition that
  `required_join_origins` already projects (its own docstring forbids a second
  inventory; this honors it).
- No `.residual` inspection; no unchecked / caller-authored plan (ownership
  derives from the compiler-owned oriented plan).
- No terminal-`All` / `KERNEL-NESTED-IND` provenance import.
- No new `Lowered` / `LoweringOperand` variant or carrier conversion —
  `RecursiveBackedge` and all values are unchanged; ONLY the partition's
  edge-classification of the realized-recursor entry changes.

## Out of scope (Architect flag — do NOT fold in)

`define_continuation_context_bodies` (`units.rs:4183`), emission-owner
`Specialization` but lowering the raw owner's body, does NOT call
`validate_join_plan_consumption`. That is the OTHER realization shape (a
separately-emitted context fn), unreconciled — a DISTINCT latent gap, NOT the
`iterative_composition` witness here. Flag for a separate node if/when a witness
surfaces; do not fold it into this WP.

## Contention check

Touches the planner owner partition (`partition_function_units` in
`semantic_ir.rs`) — a soundness-bearing producer of the kernel-checkable
required-join set. Runtime is the sole lane on this surface (foundation is on
PX9; language on its own track). Architect REQUIRED reviewer
(`evt_1nkx3f30hqp9y`) + Runtime QA. The candidate's TCB classification is
assessed at M4 (the owner partition feeds `required_join_origins`, so likely
TCB-adjacent); the Architect review is required regardless.

## Sequencing

Releasable now (both predecessors merged, mechanism ruled). On the candidate: my
M1-M4 (Architect REQUIRED reviewer + Runtime QA). Accepted-partial discipline
holds: if native completes but parity fails, or advances to a NEW named refusal,
report it verbatim + site and STOP — a genuinely-new mechanism is a fresh
Architect question, not something to route around. On land,
RT-NESTED-IH-NATIVE-REALIZATION D3-D5 complete and `KERNEL-NESTED-IND` unblocks;
the operator-prioritized runtime kernel chain advances toward `DS-9`.
