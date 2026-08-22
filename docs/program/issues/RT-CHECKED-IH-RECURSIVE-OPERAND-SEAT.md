---
id: RT-CHECKED-IH-RECURSIVE-OPERAND-SEAT
title: "Seat the checked-IH recursive-position ENVIRONMENT operand on the ported (checked-IH) route so the static worker call arrives saturated -- the structural closure of section-1b entries 2+3 (Architect ruling evt_3tspjkw7dhh6x: CLOSURE A). The checked-IH marker is a bound, already-specialized computational IH -- a saturated NULLARY closure-style invocation -- so explicit template arity 0 is CORRECT and the recursive value is an environment operand at the plan's recursive_position, NOT an explicit application argument. In the ported-route lowering (consume_checked_ih_marker_at_static_worker_call and the worker-call emission it feeds), BEFORE the declared-arity gate at core.rs:14306, materialize the recursive-position operand from the resolved checked slot template and place it in the worker-call inputs, reading the SAME recursive_position authority and using the SAME seating discipline the composed route already uses (composed_recursive_argument_binding, or an equivalent reading that authority). Do NOT weaken or bypass the core.rs:14306 declared-arity gate, do NOT relax the marker-seam checks, do NOT change the marker's meaning, and do NOT relabel any count -- post-fix, supplied == worker.declared_arity must hold HONESTLY because the operand is now supplied. Producer, template, marker seam, and the join half stay untouched. Carries the terminal AC-REENUM gate for the checked-program family (shared with RT-IH-MARKER-PRODUCER-COMPLETE)."
status: closed
owner: runtime
size: M
gate: none
depends_on: [RT-IH-MARKER-PRODUCER-COMPLETE]
blocks: []
github: null
origin: "Successor cut by the Steward from the AC-REENUM STOP (runtime-implementer evt_6p7vfbadg863p) after the Language producer fix (64019430c) cleared the marker seam and exposed a deeper refusal one layer below it: the static-worker ABI supply layer (template arity 0 vs worker.declared_arity 1, calls.rs:220; 2/2 checked-family, 8/11 rt_parity). Architect ruled the closure fork CLOSURE A (evt_3tspjkw7dhh6x), grounded on the research advisory (evt_n38ptc08a1sc) and Ken's own facts; owner returns to Runtime; the Language producer fix stays a closed deliverable. HS=4 (Steward-confirmed evt_423sg9t98rrf4) -- a genuine new mechanism, not a mandatory 3/6/9 re-trigger. Steward-filed per COORDINATION section 2."
---

# WHAT THIS NODE IS

> SUPERSEDED + CLOSED 2026-08-22 (never merged; mechanism and name both refuted).
> Successor: [[RT-CHECKED-IH-FUNCTIONAL-REPRESENTATION]]. Steward scoping ruling
> evt_5pmk273zg5paa, on the Architect's pre-committed conditional ruling
> evt_2f4bbmt7qfde1 and the decisive ESCAPE measurement evt_79jd1nxamqd95.
>
> Why it is closed rather than re-framed: the whole arc refuted this node twice
> over. (1) Closure A WITHDRAWN (Architect evt_hftfnn4mh8jk) on the
> runtime-implementer's grounded refutation (evt_6tzrt1xndpx1e, measured on
> `64019430c`): the recursive value at the firing seam is a LIVE nested-recursor
> StaticWorker, and `LoweringOperand` = `{ Specialized(Lowered),
> Carried(CarriedBoundaryWord) }` with `worker.captures` itself
> `Vec<LoweringOperand>` -- so the live worker fits NEITHER `inputs` NOR captures,
> and the only conversion is the forbidden closure-boundary crossing. Nothing can
> be "seated as an operand". (2) The reframed ruling then established the marker is
> a NULLARY_FORCE (Ken's own elaborator concept, `compiler_driver.rs:1718-1743`):
> template arity 0 means "realize the specialized IH function VALUE, unapplied,"
> not "call with zero args." (3) The decisive ESCAPE measurement came back
> ESCAPING: the realized IH value's immediate parent is a `Construct` on both
> checked-family programs -- it is stored straight into a constructor field, so no
> in-frame use-site specialization can apply. This is a CAPABILITY GAP (Ken has no
> first-class functional-IH value), not an operand seat and not a seam fix.
>
> Disposition of the surrounding work: the marker's nullary_force semantic, the
> producer fix (`64019430c`, closed Language deliverable), and the join half
> (`6a45ae1a7`, APPROVED) all STAND. The co-land set lands WITHOUT the functional-IH
> piece; the two checked-family programs are re-pointed to documented ADVANCING
> REFUSALS (they now refuse at the correct deeper capability-gap point, not the
> shallow marker seam). The terminal "green both checked-family programs" goal and
> the AC-REENUM rerun move to the successor. HS=5 (Steward of record); the ESCAPING
> outcome is a scoping decision, not a hard-stop increment.
>
> Everything below is the REFUTED Closure-A mechanism, retained only as the
> reasoning record. Do NOT build it.

Seat the checked-IH recursive-position ENVIRONMENT operand on the ported
(checked-IH) route so the static worker call arrives saturated at the
declared-arity gate. This is the STRUCTURAL closure of the shared predicate
behind section-1b inventory entries 2 and 3 (Architect ruling evt_3tspjkw7dhh6x):
the recursive-hypothesis invocation's argument supply was asserted independently
at producer, marker template, and worker ABI, with no single authority actually
supplying the recursive argument on the ported route. Closure A makes the ported
route read the SAME `recursive_position` authority and seat through the SAME
discipline the composed route already uses -- not a point-fix, not a count
relabel, not a gate weakening.

# THE RULING (Architect evt_3tspjkw7dhh6x -- CLOSURE A; B REJECTED)

The checked-IH marker is a bound, already-specialized computational IH -- a
SATURATED NULLARY closure-style invocation. Explicit template arity 0 is CORRECT
and truthful; the recursive value is an ENVIRONMENT operand at the plan's
`recursive_position`, not an explicit application argument. "Explicit arity 0"
and "physical ABI arity 1" are contradictory only if both count the same
direct-call parameter run -- here they do not (direct application arity vs a
closure/environment operand).

Grounded on Ken's own facts plus prior art:

- `CheckedComputationalIHSlotTemplateV1` holds `recursive_position` AND
  `method_binder_ordinal` as SEPARATE fields (the constructor source coordinate
  and the lexical one). Marker consumption already resolves that slot template to
  check the binder, so it is ALREADY holding the object whose `recursive_position`
  identifies the needed constructor field (confirmed against the consume-side D5a
  doc in `lowering/mod.rs`, not only the advisory).
- `composed_recursive_argument_binding` already seats the recursive value from
  the case's selected `recursive_position` into the worker binding with its
  declared arity -- an EXISTING instance of the position-seating discipline in
  this same backend, not an analogy. Closure A makes the ported route participate
  in that one authority.
- Prior art uniformly separates explicit application arity from
  closure/environment operands (research advisory evt_n38ptc08a1sc): Lean's
  recursor contract hands the method the recursive field plus a specialized IH it
  does not re-supply per use; typed closure conversion (Minamide/Morrisett/Harper,
  POPL 1996) excludes captures from source arity; GHC join arity counts only
  explicit params; Lean IR `fap` (direct full app) vs `ap` (closure app, env not
  in the explicit arg list).

Closure B REJECTED: it is coherent only if the marker denoted a DIRECT
raw-recursor-worker call with the recursive subterm as an explicit operand. It
does not -- the marker denotes use/forcing of a bound already-specialized IH slot
(D5a: "the marker denotes the application, not the applied value"). B would fold
a hidden environment input into the explicit arity the call template deliberately
keeps separate.

# THE DELIVERABLE (Runtime-owned)

- In the ported-route lowering (`consume_checked_ih_marker_at_static_worker_call`
  and the worker-call emission it feeds), BEFORE the declared-arity gate at
  `core.rs:14306`, materialize the checked recursive-position environment operand
  and place it in the worker-call inputs -- using the `recursive_position` carried
  by the resolved checked slot template and the composed route's existing seating
  discipline (`composed_recursive_argument_binding`, or an equivalent that reads
  the same authority).
- Producer, template, marker seam, and the join half (`6a45ae1a7`) stay
  byte-untouched.

# ACCEPTANCE

- **AC-1 (operand seated from the resolved authority).** The ported route seats
  the recursive operand from the resolved checked slot template's
  `recursive_position` via the shared composed-route discipline -- not a fresh
  ad-hoc derivation, not a hard-coded value.
- **AC-2 (gate honest, not moved).** The `core.rs:14306` declared-arity gate is
  NOT weakened or bypassed and the marker-seam checks are NOT relaxed. Post-fix,
  `supplied == worker.declared_arity` holds because the operand is now SUPPLIED,
  not because any count was relabeled or any gate moved. The fail-closed gate
  stays a real defence: a ported-route call that reaches it without the seated
  operand must STILL trip it. Ship one durable test in that shape.
- **AC-REENUM (terminal gate; shared with RT-IH-MARKER-PRODUCER-COMPLETE).**
  Rerun report-2's checked-family runner end-to-end (it already exists -- a rerun,
  not authoring). Both checked-family programs green ⇒ the family is bounded and
  both runner tables re-point green. Any FURTHER refusal ⇒ STOP and report a
  genuinely new deeper mechanism back to the Architect + Steward -- do not proceed
  silently. `rt_parity`: expect the 8 (the 288/301-then-IH-marker class) to
  advance; the other 3 stay out of scope (`rt_allocate_stage` BoundaryCarrier,
  `rt_write_pair_source` TypeMismatch, `rt_write_writable_stage` closure boundary).
- **AC-NO-REGRESSION.** No lowering on `main` regresses; whole-suite green in CI
  (`COORDINATION §12`). Local targeted `-p` only, never `--workspace`; runtime
  respin gate `-p ken-runtime` all-binaries + `-p ken-cli` + `-p ken-verify`.
- **Required reviewer.** Architect (soundness): verify the operand is seated from
  the resolved slot's `recursive_position` via the shared discipline; the gate is
  not weakened; `supplied == declared_arity` holds by supply not relabel; marker
  seam / producer / template / join stay untouched; AC-REENUM green. The Adversary
  hunts the over-accept shapes: a weakened/bypassed arity gate, a relabeled count,
  an ad-hoc operand that does not read the resolved `recursive_position`.

# CO-LANDING

Part of the RT-FSREADAT co-landing set (§8) on
`wp/RT-FSREADAT-REPLY-BUFFER-GATE-REMOVAL` (thread `thr_3j5ew8rhy35nh`), current
tip `64019430c` (carries Language's producer fix). The whole set lands as ONE
green candidate once AC-REENUM greens and the `cap41_*`/terminal rows go
all-green: the join half (`6a45ae1a7`, APPROVED) + the Language producer fix
(`64019430c`, closed deliverable) + this runtime seat + the built projection /
disposition + the join reconcile. It does NOT land alone. On green, `D-final`
closes NATIVE-HANDLE-CARRIER + PX8-F-CAP-41 Phase 2.

# NOT IN SCOPE

- Weakening or bypassing the `core.rs:14306` declared-arity gate, or relaxing the
  marker-seam / D5a checks -- all stay the fail-closed boundaries.
- Changing the marker's meaning, the template arity (0 is correct), the producer
  emission ([[RT-IH-MARKER-PRODUCER-COMPLETE]], closed deliverable), or the join
  half ([[RT-MATERIALIZED-DEAD-JOIN-RECONCILE]]).
- The opposite-direction BoundaryCarrier observation (`rt_allocate_stage`) -- a
  SEPARATE mechanism, not reopened. The advisory's separation of direct-call
  arity vs environment operands is exactly why the two routes answer 0-vs-1 in
  opposite directions without being the same defect. The `rt_write_pair_source`
  TypeMismatch and the `rt_write_writable_stage` closure boundary stay out of
  scope as before.
- Any kernel / TCB edit. `ken-runtime` cranelift lowering; no operator
  authorization is in play.

# SEQUENCING / CONTENTION / CAPABILITY TIER

`depends_on: [RT-IH-MARKER-PRODUCER-COMPLETE]` -- the producer fix is already on
the co-land branch tip (`64019430c`); this seat builds on top of it in the same
branch, ring, and thread. `ken-runtime` cranelift lowering; no other lane touches
it.

Tier T1: a soundness-bearing lowering change on the recursor ABI supply layer,
reviewed on the argument that the operand is seated from the resolved authority
and the gate stays honest -- not a differential diff. It carries the terminal
AC-REENUM gate that decides whether the checked family is bounded. Size M.
