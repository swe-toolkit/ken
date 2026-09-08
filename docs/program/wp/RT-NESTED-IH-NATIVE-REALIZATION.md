# WP frame — RT-NESTED-IH-NATIVE-REALIZATION D3-D5 (realized-recursor EdgeKind reconciliation, §1b structural closure)

> The completing D3-D5 slice of RT-NESTED-IH-NATIVE-REALIZATION (node
> `docs/program/issues/RT-NESTED-IH-NATIVE-REALIZATION.md`). Runtime lane,
> operator kernel-chain priority (`evt_7nkzsy27p7npw`: "Go with A, unblock the
> kernel chain"). Owner: runtime. Size: M. Tier: T1. Gate: none. Architect
> (`evt_1nkx3f30hqp9y` z4027, `evt_2a1gzw40mprz0` HS#2 closure) is the REQUIRED
> reviewer on the candidate (a soundness-bearing planner static-graph fix).
> Builds on RT-REALIZED-BACKEDGE-SOURCE-CONTINUATION (merged `897f1ea6a`) and
> RT-CHECKED-IH-REALIZATION-AUTHORITY (merged `e68ecd79`). On land, this node's
> D3-D5 complete and `KERNEL-NESTED-IND` unblocks.

> ## RECUT 2026-09-08 — HS#2 §1b STRUCTURAL CLOSURE (Architect `evt_2a1gzw40mprz0`)
>
> The z4027 owner-partition mechanism (correct the stale partition with two
> special-cases) is SUPERSEDED by a single edge reconciliation. HS#2 fired one
> edge further on, at the ABI boundary-signature validator, and it shares HS#1's
> ROOT: the realized recursor's edge stays typed `EdgeKind::StaticBody` across
> the entire static graph (~20 consumers, five files) while the elected in-place
> checked-IH realization made it an intra-F CFG backedge. Point-fixing each
> consumer is the RT-NATIVE-FNSPLIT 33-stop chain re-forming. The Architect
> named the predicate at the 2nd entry (§1b sanctions this — the 3rd entry is
> the LATEST you may generalize, not the earliest, and the shared root here is
> grounded, not inferred). z4027 is NOT landed (the node is one still-compiling
> candidate), so folding the closure in throws away no landed code — it SIMPLIFIES
> the partition rather than adding to it. The superseded per-plane WIP is
> checkpointed at `6596d0051` on `wp/RT-NESTED-IH-NATIVE-REALIZATION` (not a
> candidate). Rebuild ONE candidate on the edge reconciliation.

## Mechanism (Architect HS#2 closure `evt_2a1gzw40mprz0`, grounded @ `496d8637b`)

The root is ONE thing: the realized checked-IH recursor edge remains typed
`EdgeKind::StaticBody` across the whole planner static graph, while the
compiler-elected in-place checked-IH realization made it an intra-F CFG backedge
(`Lowered::RecursiveBackedge`, already emitted in F per RT-REALIZED-BACKEDGE).
A `StaticBody` edge asserts a source-level call boundary — a separate
closure-body unit with its own defining occurrence. Post-realization that
assertion is a FICTION: there is no separate unit, no separate frame, no call;
the body is F's own code, lowered INLINE via the `iterative_composition` descent
(`core.rs:3382`), and the tail edge is already in F.

The edge is minted at `construction.rs:771` (`register_static_body`: source ->
`body.entry` as `EdgeKind::StaticBody`) and nothing retires it. z4027 corrected
the partition's TREATMENT of that edge (seed `edge.to` as its own unit at
`semantic_ir.rs:1296`; wall `StaticBody|DeclarationCall` out of the flood at
`semantic_ir.rs:1350`) but left the edge KIND stale — so every other consumer
of `EdgeKind::StaticBody` still reads it as a boundary.

Two hard stops, one predicate ("the static plane trusts a source-level
`StaticBody` boundary"):

1. HS#1 (RULED z4027): `partition_function_units` walls join 50 to the inner
   unit G though it inlined into F — because the recursor entry is a `StaticBody`
   wall.
2. HS#2 (this frontier): `abi.rs:2735` `validate_boundary_layouts` iterates
   `boundary_signatures()` (`abi.rs:2854`), one signature per
   `EdgeKind::StaticBody` edge, with `defining_origin = edge.from` and
   `callee = owner(edge.to)`. For the realized recursor, `edge.to` (the body
   seed) is now owned by F after z4027, so `descriptor(F).closure_shaped_captures()`
   yields F's defining_origin while `signature.defining_origin` is the recursor
   occurrence — they disagree -> `PlannerInvariant` at `abi.rs:2771`
   "boundary signature and callee descriptor disagree on the defining occurrence."

RULING — reconcile the EDGE, once. At the point the plan elects in-place
checked-IH realization for the recursor (the SAME compiler-owned oriented-plan /
checked-IH marking z4027 keys on — kernel-checkable, identity-bound, never
caller-authored), the realized recursor's edge must NOT be an
`EdgeKind::StaticBody` call boundary. Reconcile it once, at the edge, to a
non-boundary control/transfer (backedge) edge that reflects the realized
backedge. The edge is still a real control edge (a tail jump) — it is
RECLASSIFIED, not deleted.

Then, with ZERO per-consumer special-casing:
- `semantic_ir.rs` partition: the edge is no longer `StaticBody`, so `:1296`
  does not seed a `StaticBodyTarget` unit (no dead G) AND `:1350` does not wall
  it (F's flood crosses naturally -> 50 owned by F). z4027's TWO partition
  special-cases fall out for free and become UNNECESSARY (the partition
  simplifies, it does not grow).
- `abi.rs:2861` `boundary_signatures`: filters `EdgeKind::StaticBody` -> skips
  the reconciled edge -> no phantom boundary. HS#2 closed.
- `closure.rs` / `continuations.rs` / `construction.rs` `StaticBody` consumers:
  all skip it. Entries 3..N preempted before they fire.

SOUNDNESS: retiring the boundary makes the graph tell the truth about what was
realized — it does not hide a boundary that exists. Body lowering is unaffected
(it never went through the `StaticBody` call path). Because reconciliation is at
the planner static graph (an `EdgeKind`), it is the SAME lane as z4027 (which
edited the planner partition), NOT a `Lowered`/`LoweringOperand` variant.

## Fixed inputs (Architect-measured @ `496d8637b`; re-measure at your D0 cut)

Line numbers drift — re-measure at D0. The Architect grounded these from
`origin/main 496d8637b` himself.

- THE EDGE-MINT / RECONCILIATION LOCUS: `construction.rs:771`
  (`register_static_body`, source -> `body.entry` as `EdgeKind::StaticBody`);
  reconcile at the realization-election point keyed on the oriented-plan /
  checked-IH marking.
- HS#2 frontier: `abi.rs:2735` (`validate_boundary_layouts`), `abi.rs:2854`
  (`boundary_signatures`), `abi.rs:2771` (the `PlannerInvariant`), `abi.rs:2861`
  (the `EdgeKind::StaticBody` filter).
- z4027 partition sites now SIMPLIFIED (special-cases removed):
  `semantic_ir.rs:1296` (seed), `semantic_ir.rs:1350` (wall).
- THE `EdgeKind::StaticBody` CONSUMER CENSUS (starting set, ~20 sites across five
  files — the ring RE-DERIVES the complete set from `origin/main` at D0):
  - `semantic_ir.rs`: `:1162`, `:1296`, `:1350`, `:2169`
  - `abi.rs`: `:1746`, `:2861`
  - `closure.rs` (~16): `:1270`, `:1508`, `:3282`, `:5405`, `:5769`, `:5858`,
    `:5959`, `:5991`, `:6016`, `:6115`, `:6173`, `:6201`, `:6210`, `:6365`,
    `:6437`, `:6485`
  - `construction.rs`: `:536`, `:563`, `:1725`
  - `continuations.rs`: `:3290`
- `validate_function_units` (`semantic_ir.rs:2395`, recompute-and-compare) — the
  AC-SINGLE-OWNER control.
- `iterative_composition` descent (`core.rs:3382`) — where the body lowers
  inline; unaffected.
- The landed RT-CHECKED-IH D4 positional-ABI vector (the recursor's oriented
  frame / slots / parents / calls) — the AC-ABI-PIN reference, plus the node's
  own deferred positional `#[cfg(test)]` pin.

## Deliverable

Reconcile the realized checked-IH recursor edge at ONE locus — the
realization-election point, keyed on the compiler-owned oriented-plan /
checked-IH marking — from `EdgeKind::StaticBody` to a non-boundary
control/transfer (backedge) `EdgeKind`. Remove z4027's two partition
special-cases as superseded (the partition flood, the no-seed/no-dead-G, and the
ABI boundary filter all follow naturally from the reconciled edge). NO
per-consumer special-casing at any of the ~20 `StaticBody` sites. The
ordinary-`Match` selector, the `source.rs` catch-all, and all values/carriers
stay untouched.

## Acceptance criteria

RETAIN (z4027's proven results, now CONSEQUENCES of the edge reconciliation):

- **AC-COMPLETE-PARITY (the goal):** native execution COMPLETES and native
  result == interpreter == `Nat 3`. Removing the stop is necessary but NOT
  sufficient — parity is the real gate.
- **AC-SINGLE-OWNER:** `validate_function_units` (`semantic_ir.rs:2395`) passes;
  the realized subtree (incl. 50) is owned by EXACTLY F — no double-owner, no
  dead G unit emitted.
- **AC-DISPOSITION-CONSISTENT:** statically-unselected branches still disposition
  correctly; `source_join_origins_in_owner_subtree(F root) ∋ 50`.
- **AC-ABI-PIN (the "wrong answer in the first release that removes the stop"
  hazard the node records):** retain the RT-CHECKED-IH D4 positional-ABI
  `#[cfg(test)]` pin — the recursor's oriented frame / slots / parents / calls
  == the landed D4 vector. Parity at `Nat 3` PLUS an unchanged positional-ABI
  vector proves the completion is the RIGHT one, not an accidental one.

NEW (the closure's guardrail — a proven closure, not a better grep):

- **AC-STATICBODY-CENSUS:** enumerate EVERY `EdgeKind::StaticBody` consumer (the
  ~20 sites above are the STARTING census; re-derive the complete set from
  `origin/main` at D0) and for each confirm it EITHER correctly treats the
  reconciled realized-recursor edge as a non-boundary (skips it) OR carries a
  GROUNDED, STATED reason it must still see the edge — in which case the
  reconciliation is an OVERLAY/attribute checked through a shared helper, not an
  `EdgeKind` change. A reason may NOT be assumed; it must be shown. This is what
  turns "reclassify fixes everything" from a bet into an obligation.

## §1b symptom inventory (seed — extend on each further stop)

Predicate shared by every entry: "the static plane trusts a source-level
`EdgeKind::StaticBody` boundary that in-place checked-IH realization dissolved."

1. **HS#1 — partition (RULED z4027, `evt_1nkx3f30hqp9y`).**
   `partition_function_units` trusts the source unit wall; join 50 walled to
   inner unit G though inlined into F. Point-fix was two partition special-cases.
2. **HS#2 — ABI boundary signatures (RULED, this closure, `evt_2a1gzw40mprz0`).**
   `abi.rs` `boundary_signatures` trusts the `StaticBody` edge as a callee
   definition; the realized subtree inlined into F disagrees ->
   `PlannerInvariant` at `abi.rs:2771`.

CLOSURE VERDICT: these are not two defects but one — the stale `EdgeKind`. The
structural fix is the single edge reconciliation above; entries 3..N (the
`closure.rs` / `continuations.rs` / `construction.rs` consumers) are preempted,
not point-fixed. A genuinely-new mechanism behind the closure returns to the
Architect; the §1a research trigger is at HS#3 and this closure is meant to end
the chain before it reaches it.

## Forbidden boundary (each closed route, inherited from z4022/z4027)

- No selector or catch-all widening: the ordinary-`Match` selector (`core.rs`)
  and the `source.rs` catch-all are UNTOUCHED — the edit is the planner
  `EdgeKind`.
- No second join inventory / no `NativeJoinPlanV1` collection (it stays
  withdrawn).
- No `.residual` inspection; no unchecked / caller-authored plan (reconciliation
  keys on the compiler-owned oriented plan).
- No terminal-`All` / `KERNEL-NESTED-IND` provenance import.
- No new `Lowered` / `LoweringOperand` variant or carrier conversion — an
  `EdgeKind` reconciliation is planner-static-graph (the SAME lane as z4027),
  NOT a `Lowered`/operand variant; `RecursiveBackedge` and all values are
  unchanged.
- No per-consumer special-casing of any `StaticBody` site (that is the 33-stop
  chain this closure exists to prevent).

## Out of scope (Architect flag — do NOT fold in)

`define_continuation_context_bodies` (`units.rs:4183`), emission-owner
`Specialization` but lowering the raw owner's body, does NOT call
`validate_join_plan_consumption`. That is the OTHER realization shape (a
separately-emitted context fn), unreconciled — a DISTINCT latent gap, NOT the
`iterative_composition` witness here. Flag for a separate node if/when a witness
surfaces; do not fold it into this WP.

## Contention check

Touches the planner static graph (the realized-recursor `EdgeKind` at its mint
locus, plus removal of z4027's partition special-cases) — a soundness-bearing
producer of the kernel-checkable required-join set and the ABI boundary
signatures. Runtime is the sole lane on this surface (foundation is on PX9;
language on its own track). Architect REQUIRED reviewer (`evt_2a1gzw40mprz0`) +
Runtime QA. The candidate's TCB classification is assessed at M4 (the edge
classification feeds `required_join_origins` and the ABI boundary validator, so
likely TCB-adjacent); the Architect review is required regardless.

## Sequencing

Releasable now (both predecessors merged, closure ruled, ring holding for this
amended frame). On the candidate: my M1-M4 (Architect REQUIRED reviewer + Runtime
QA). Accepted-partial discipline holds: if native completes but parity fails, or
advances to a NEW named refusal, report it verbatim + site and STOP — a
genuinely-new mechanism is a fresh Architect question, not something to route
around. On land, RT-NESTED-IH-NATIVE-REALIZATION D3-D5 complete and
`KERNEL-NESTED-IND` unblocks; the operator-prioritized runtime kernel chain
advances toward `DS-9`.
