# WP frame — RT-REALIZED-BACKEDGE-SOURCE-CONTINUATION

> Runtime lane (operator priority A — the kernel chain). Direct successor to the
> landed [[RT-CHECKED-IH-REALIZATION-AUTHORITY]] (merged `d29cc8ad0`), which mints
> the checked-IH authority and honestly hard-stops on one edge. Sole live frontier
> gating [[RT-NESTED-IH-NATIVE-REALIZATION]] -> [[KERNEL-NESTED-IND]] -> DS-9.
> Owning team: runtime. Size: S. Capability tier: T1 (one forwarding arm, but the
> review turns on the placement/route-carry soundness argument). Gate: none.
> Architect is the REQUIRED reviewer on the candidate (mechanism ruling
> `evt_47mfww0ecbf5y`).

## Objective

The landed authority realizes both checked-IH calls under the 1-frame/2-slot/
2-call plan; the seat then refuses one edge on. The realized IH result is a
`Lowered::RecursiveBackedge` — a protocol marker, not a value — and it arrives as
the scrutinee of an enclosing ordinary source-machine Match. The ordinary
`SourceContinuation::MatchScrutinee` seat asks that marker to be a constructor
value and refuses with the verbatim `Match: scrutinee is not a constructor
value`. Forward the realized `RecursiveBackedge` at this seat, exactly as the
sibling `ComputationalMatchScrutinee` seat already forwards it — making the
ordinary Match seat propagate the marker instead of eliminating it, and letting
the checked native pipeline advance past the residual the authority partial
pinned.

## Design judgment (front-loaded; Architect ruling `evt_47mfww0ecbf5y`)

This is NOT new machinery: it is the one remaining seat missing an already-ruled,
already-uniform protocol. `Lowered::RecursiveBackedge` says the tail-recursive
edge has ALREADY been emitted as a CFG jump and the current block is
predecessor-free (`resume_active_continuation`, RT-RECURSOR-TRANSPORT D2,
`evt_bqg3gjwkp350`). Every ENCLOSING combinator must PROPAGATE the marker, not
consume it — the generic `lower_expr` Match, the If seat, the Let-binding seat,
and the arg-vector seat all already forward it, and the source-machine
`ComputationalMatchScrutinee` seat forwards it too (RT-LEXICAL-RECURSOR-CONSUMERS
D2a). The ordinary `MatchScrutinee` seat is the LAST one missing it.

- CORRECT CONTINUATION = FORWARD (propagation), never a re-entry / resume / join /
  selection. The OWNING recursor (e.g. `lower_source_bounded_nat_match`) resumes
  and joins its OWN edge at its merge; an ENCLOSING consumer of a realized
  recursive result hands the marker onward unchanged. Mirror the D2a arm.
- CHECKED PROVENANCE = NONE new. The marker is self-describing: the forward mints
  no authority, consumes no occurrence plan, selects/dispositions no case,
  constructs no value.
- PLACEMENT INVARIANT (load-bearing, straight from D2a): the forward MUST be taken
  BEFORE `self.enter_source_occurrence_plan(static_origin)`, or it consumes an
  occurrence plan the forward must not. Carry the incoming route; do NOT reset it
  to `DirectScrutinee` (D6a: a reset is a silent erasure — green compile, wrong
  closed default).

REUSE, do not reinvent: this is a near-verbatim port of the `ComputationalMatch
Scrutinee` D2a arm (`source.rs:1999-2027`) to the `MatchScrutinee` seat. Do not
stand up a parallel forwarding mechanism.

## Fixed inputs (measured @ origin/main `e68ecd79`)

Coordinates measured at `e68ecd79` — the landed-authority SHA the Architect
grounded from. Line numbers drift; re-confirm at your D0 branch cut.

- The refusing seat (MISSING the forward):
  `crates/ken-runtime/src/cranelift_backend/lowering/source.rs`,
  `SourceContinuation::MatchScrutinee { cases, default, env, static_origin, next }`
  arm, opens at `:1740`. It runs `self.enter_source_occurrence_plan(static_origin)?`
  at `:1747` and `control.continuation = *next` at `:1748` UNCONDITIONALLY at the
  top of the arm, then computes the cfg-observation `refusal_operand_kind`
  (`:1752`) and does `match value { ... }` (`:1757`) — a
  `Specialized(RecursiveBackedge)` value falls through that dispatch to the
  `Match: scrutinee is not a constructor value` refusal. The `RecursiveBackedge`
  forward must be HOISTED above `:1747`.
- The TEMPLATE to mirror (already forwards): the sibling
  `SourceContinuation::ComputationalMatchScrutinee` arm, opens at `:1986` (labeled
  block `'computational_scrutinee`). Its D2a forward at `:1999-2027`:
  `if matches!(&value, LoweringOperand::Specialized(Lowered::RecursiveBackedge)) {
  control.continuation = *next; break 'computational_scrutinee
  SourceMachineState::Value { value: RoutedAnswer { value, route: incoming_route,
  role: EliminatorRole::Scrutinee }, control }; }` — taken BEFORE that arm's own
  `enter_source_occurrence_plan` (`:2095`), route carried not reset. (Ignore the
  D2f `take_fused_region_at` takeover below it at `:2071` — a separate concern, not
  part of the backedge forward.)
- SHAPE ADAPTATION (this is D0 measurement, not invention — Architect): the
  `ComputationalMatchScrutinee` arm is a labeled block whose value is a
  `RoutedAnswer`; the `MatchScrutinee` arm is a plain `=> { ... }` whose value is a
  plain `LoweringOperand` and whose case arms `return self.lower_...()` directly.
  Produce the `RecursiveBackedge` marker as the machine's forwarded value with the
  continuation set to `next`, in the `MatchScrutinee` arm's own return/value shape.
- The discriminating INSTRUMENT is already in-tree: the authority partial's
  `refusal_operand_kind` cfg-observation (`source.rs:1752`, under
  `feature = "checked-ih-realization-observation"`) is what pinned the residual
  operand `RecursiveBackedge` at this seat. Reuse it for the positive/negative
  controls; no new observation machinery.

## Deliverables

1. The forwarding arm at the ordinary `MatchScrutinee` seat: a
   `Specialized(RecursiveBackedge)` value is forwarded (continuation set to `next`,
   incoming route carried, no `DirectScrutinee` reset), HOISTED above
   `enter_source_occurrence_plan(static_origin)`. A near-verbatim port of the D2a
   arm, adapted to this seat's value/return shape.
2. The discriminating same-seat pair (below) + the parent-frontier measurement.

## Acceptance criteria

- AC-FORWARD (the object): a realized `RecursiveBackedge` reaching the ordinary
  source-machine `MatchScrutinee` seat is FORWARDED — the `source.rs`
  `SourceMachineSelector` refusal with operand `RecursiveBackedge` (the exact
  residual the authority partial pinned via the `checked-ih-realization-observation`
  instrument) is GONE / the pipeline advances past it.
- AC-GUARD-INTACT (negative control, non-degenerate same-seat pair — load-bearing):
  a genuine non-constructor scrutinee STILL refuses at the SAME seat with the
  verbatim `Match: scrutinee is not a constructor value`. Neither guard is
  weakened; only `RecursiveBackedge`, which was never a scrutinee value, is routed
  past selection. "Realize before selection; never accept AT selection" holds.
- AC-PLACEMENT (reviewer-checkable at the diff): the forward is taken BEFORE
  `enter_source_occurrence_plan(static_origin)` and carries `incoming_route`
  unchanged (no `DirectScrutinee` reset). A control that REDS if the forward is
  placed after `enter_source_occurrence_plan` (double-consuming the occurrence
  plan) or resets the route.
- AC-PARENT-FRONTIER (measure and report; both outcomes first-class): with the
  forward in place, does native execution complete to interpreter agreement at
  Nat 3 — discharging [[RT-NESTED-IH-NATIVE-REALIZATION]] D3-D5 — or advance to a
  new NAMED refusal? If it completes, say so and un-ignore the parity row. If it
  advances, report the refusal VERBATIM + site and STOP (accepted-partial
  discipline, exactly as the authority node did); do not press past a new wall. A
  genuinely NEW mechanism (not merely a further-along refusal) is a fresh mechanism
  question for the Architect, not something to route around (mirrors the authority
  node's AC-5).

## Boundary — the routes that stay closed

Inherited from the authority node, restated by the ruling.

No selector widening (the `Specialized(_)` refusal stays for genuine
non-constructor scrutinees); no join collection (this is a FORWARD, not a join —
NativeJoinPlanV1 stays withdrawn; the distinction from the bounded-Nat resume,
which uses a join plan for its OWN edge, is exactly that an enclosing consumer
propagates rather than merges); no unchecked/caller plan; no `.residual`
inspection; no terminal-All / KERNEL-NESTED-IND provenance (the marker is
self-describing); no new `Lowered`/`LoweringOperand` variant (`RecursiveBackedge`
exists; the forward returns the same operand); no carrier conversion; no
RT-TERMINAL-ALL scope edit.

## Contention check

Touches one arm of
`crates/ken-runtime/src/cranelift_backend/lowering/source.rs` (the ordinary
`MatchScrutinee` seat) plus the parity test row that measures the parent frontier.
Runtime is the sole priority lane touching this surface (language/foundation are
on their own tracks). No spec/, kernel/, or conformance/ touch. TCB-neutral (the
elaborator/kernel are not touched; this is runtime backend lowering).

## Sequencing

Releasable on the node's land. The Architect is the required reviewer at the
candidate for the placement/route-carry soundness and the guard-intact control.
On land, this discharges the frontier and [[RT-NESTED-IH-NATIVE-REALIZATION]] can
re-release (its D3-D5 gate on this node); when the runtime chain clears
(RT-NESTED-IH + [[KERNEL-NESTED-IND]]) runtime returns to the ABI program.
