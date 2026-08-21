---
id: RT-CAPTURE-CONTEXT-FRAME-EMIT
title: "physically emit the producer-local worker's continuation through the generated context's OWN emitted frame (define_continuation_context_bodies), so its capture-gather reads the context's own (parameters+captures) ABI operand run instead of the enclosing function's 2-operand run -- the sufficient closure for the 10 populated recursive-position witnesses that all six RT-CAPTURE-PROJECTION-GROW findings pointed at. Measured decisive (runtime-leader evt_77tdvcap4067t): seam (i) alone PERSISTS and the context-body defining_abi_operands push executes ZERO times compiling px7f -- the context body is not emitted for this population at all, so the closure is CAUSING that emission plus constructing the context frame at the producer point, substantial new wiring, not existing-path completion."
status: ready
owner: runtime
size: L
gate: none
depends_on: [RT-CAPTURE-PROJECTION-GROW]
blocks: [NATIVE-HANDLE-CARRIER, PX8-F-CAP-41]
github: null
origin: "Steward scope cut on the seam-(i) measurement (runtime-leader evt_77tdvcap4067t), keyed to the Architect's explicit fold-vs-successor discriminator (evt_28pfs66gmth9a) and the Steward's pre-stated disposition (evt_3e0zr8vs8kpav). PERSIST plus substantial-new-wiring triggers the successor cut per COORDINATION section 3 (the WP cut is the Steward's). Physical emission is runtime-backend lowering: gate none, no TCB, no operator gate."
---

# WHY THIS NODE EXISTS

Six consecutive RT-CAPTURE-PROJECTION-GROW findings converged on ONE root,
now decisively measured. The Architect's fully-exposed [[RT-CAPTURE-PROJECTION-GROW]]
section-1b predicate (evt_28pfs66gmth9a): the generated context was retargeted by
LABEL at successively finer grain -- projection site, capture origin, emission
owner, claim frame, declared slot -- but NEVER PHYSICALLY INSTANTIATED as its own
emitted frame with its own ABI operand run. Every label retarget bottomed out
because the producer-local captures are live only AFTER the enclosing function
enters its body, so no worker continuation emitted INLINE in the enclosing
function can ever gather them from that function's `defining_abi_operands`. That
route is structurally closed, not merely mis-slotted.

The seam-(i) increment settled fold-vs-successor by measurement, not by guess.
Seam (i) -- populate `function_local.defining_abi_operands` from the context slot
walk (`units.rs:2820-2918`), mirroring the ordinary-unit push at
`units.rs:6051-6066` -- was built (`c0d0a2451`) and measured (runtime-leader
evt_77tdvcap4067t):

- Seam (i) alone: suite 928/0, a clean existing-path completion.
- Composed with the predecessor's ruled seating + owner bind: the 25/46
  slot-range failures are UNCHANGED, identical shape.
- Decisive: the push seam (i) adds executes ZERO times compiling the px7f
  witness. The context-body path is not taken for this population AT ALL, so
  seam (i) provably cannot affect it -- this is not a partial-fix artifact.

So the sufficient closure is not existing-path completion. It is CAUSING the
context-body emission for these workers (`define_continuation_context_bodies`)
plus constructing the context frame in declared order at the producer point
(where the producer-local captures are live), then routing the worker's
continuation through `call_declared_context` (`calls.rs:776`) so its
capture-gather reads the CONTEXT's own `(parameters + captures)` run. That is
substantial new wiring, and it is the real remaining gate for
[[NATIVE-HANDLE-CARRIER]] and [[PX8-F-CAP-41]] for this population -- which is
why their `blocks` edge moves here from the predecessor.

# WHAT LANDED IN THE PREDECESSOR (necessary, not sufficient)

[[RT-CAPTURE-PROJECTION-GROW]] advanced the chain and closed as a landed
partial; this node carries its closing deliverable.

- `D1` (grow the planner projection / mint the claims) LANDED `9ab12ca97`.
- `D3` (admission gate widened to "captures all planner-recoverable",
  fail-closed, inert until seated) LANDED `38ced327a`.
- The `D2` route was RULED sound (emission-ownership retarget through the
  existing `(GeneratedContext, Specialization)` arm, then homogenize the capture
  run to the single emitting context, seat-first-then-bind) -- but the sufficient
  physical-frame emission is THIS node.
- Seam (i) is BUILT at `c0d0a2451` (branch `wp/RT-CAPTURE-PROJECTION-GROW-D2`,
  parent `6bd280807` = D3 rebased onto `018aa5b44`). It is currently INERT
  (nothing reaches the path it completes) and unexercised. It is preserved as
  this node's `D1` and lands here COMPOSED with `D2` -- where the emission makes
  it live and testable -- never as a standalone inert change read later as
  verified.

# DELIVERABLES

**`D0` (design -- Architect).** Design the creation-site emission: where and how
to CAUSE `define_continuation_context_bodies` (`units.rs:2647+`) to emit the
context body for this population's producer-local workers, and how to construct
the context frame in declared order at the producer point (where the
producer-locals are live), so the worker's continuation routes through
`call_declared_context` (`calls.rs:776`) against the context's own run. This is
component design -- the Architect's call, not the Steward's -- and the
implementer correctly declined to guess it unaided (three measured-and-declined
heavy guesses is exactly what COORDINATION section 1a exists to stop). Front-loaded
by the Architect's ruling (evt_28pfs66gmth9a) and the research advisory
(evt_63pqpmfn3vrez): a complete closure conversion lifts the code AND
materializes/passes the environment at every rewritten caller -- a body
declaration or an owner bind alone is not enough.

**`D1` (seam i -- BUILT, preserved).** Populate `defining_abi_operands` in the
context slot walk (`c0d0a2451`, mirrors `units.rs:6051-6066` in descriptor
order). Lands composed with `D2`, exercised -- not standalone.

**`D2` (seam ii).** Cause the context-body emission plus the creation-site frame
construction per `D0`'s design; route the worker continuation through the
context body. On landing, the 25/46 slot-range failures clear and the ten
witnesses green (composed with the predecessor's `D1`+`D3` and the ruled
seating/owner-bind).

# ACCEPTANCE CRITERIA

**`AC-1` (the moved closing criterion).** The ten populated recursive-position
witnesses from the cardinality-gap D0 green: the worker's continuation resolves
every capture against the context's own ABI operand run, with no remaining
slot-range refusal on this population.

**`AC-2` (soundness -- the inviolable line, doubly proven).**
`verify_entry_frame`'s membership + slot re-derivation guard is UNCHANGED, never
relaxed. A capture-gather that reaches a run which is not the context's own still
refuses loudly -- the fail-closed ordering that proved itself this round
(`verify_entry_frame` accepts `parameters + position`, then the operand FETCH
refuses a foreign run). The route extends membership / routes through existing
arms only.

**`AC-3` (conformance)** -- for the greened witness case.

**`AC-4` (no-regression)** -- CI whole-suite green (section 12).

**`AC-5` (arity re-measurement -- the RT-SRCBODY-BIND-ORDER cross-WP dependency).**
When `D2` routes a worker through `define_continuation_context_bodies`, MEASURE
the worker's arity. `arity >= 2` trips the `RT-SRCBODY-BIND-ORDER` arity-2
sentinel (`units.rs ~2870`, `cfg(test)`:
`assert context_parameter_ordinals.len() <= 1`) -- the deferred cross-host
binding-order equivalence obligation goes LIVE, and the deliverable is then the
equivalence control per the sentinel's own instruction, NOT a wider bound;
surface it to the Steward as a live cross-WP dependency. `arity <= 1` is inert.
Measured inert at seam (i) (the sentinel did not fire, no
`context_parameter_ordinals` assertion in any failure) because arity is only
observable once seam (ii) actually routes a worker through -- hence the
re-measurement here.

**`AC-6` (lifetime/escape residual -- measure-first, report-not-bury).** The
constructed context frame must be live for every recursive/re-entrant worker
invocation, each activation receiving its own capture values (research #2/#3,
safe-for-space closure conversion). The static `(specialization, worker_body)`
key is a sound identity base but does not itself establish runtime frame
freshness. If an escaping or re-entrant invocation surfaces, it is the next
measured finding -- surfaced, not buried -- and it does not defeat this node's
route.

# BANNED SCOPE

- **Relaxing `verify_entry_frame`'s membership + slot re-derivation guard.**
  Inviolable and doubly proven; extend membership / route through existing arms
  only. A gather that reaches a foreign run refuses, loudly.
- **Reading any capture value from the carried word or a producer's lexical
  stack.** Inherited bar from the whole chain.
- **Landing seam (i) as a standalone inert, unexercised change.** It lands
  composed with `D2`, exercised. The implementer explicitly flagged it should
  not be read later as verified.
- **A wider `RT-SRCBODY-BIND-ORDER` bound.** If the arity-2 obligation fires,
  the deliverable is the equivalence control, not a broader change.
- **Reaching the 6 empty-population witnesses** (the cardinality-gap D0's
  reached-not-at-all set). Out of scope; a separate successor only if a
  measurement demands it.

# SEQUENCING

`depends_on: [RT-CAPTURE-PROJECTION-GROW]` (its `D1`+`D3` landed, its `D2` route
ruled sound). Released as the runtime ring's next node. `gate: none` -- runtime
backend lowering: causing a physical context-body emission and constructing the
context frame at the producer point, no TCB or trusted-reduction change, no
operator gate. Tier **T1** (physical emission plus creation-site frame
construction is soundness-adjacent new wiring; the `D0` design and `D2` build are
reasoning-dense; a wrongly-constructed frame is a miscompile, caught fail-closed
by the unchanged guard but it must be designed right). Review: **Architect**
(author is not reviewer) -- the Architect designs `D0` and reviews `D2`. First
actor is the Architect (`D0` design), then the implementer (`D2` build on that
design).
