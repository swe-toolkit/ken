---
id: RT-CAPTURE-PROJECTION-GROW
title: "green the ten populated recursive-position witnesses by growing the planner's capture projection to cover the closure's declared, body-referenced captures (D1) and seating the resulting producer-local claims in the generated context's entry-source enumeration (D2, the revived RT-CONTSRC-ENTRY-FRAME-WIDEN widening). The cardinality-gap D0 measured all-H1: every unclaimed capture is a genuine value the projection drops because continuations.rs:6075 clones the context's capture set from the enclosing specialization's continuation_inputs, a different population than the closure's declared set. This is the closing deliverable for the population -- the two steps compose, D2 is non-inert only because D1 supplies the claims."
status: active
owner: runtime
size: L
gate: none
depends_on: [RT-CAPTURE-CARDINALITY-GAP]
blocks: [NATIVE-HANDLE-CARRIER, PX8-F-CAP-41]
github: null
origin: "Steward disposition of RT-CAPTURE-CARDINALITY-GAP's all-H1 D0 (43044dbcd, Architect-approved evt_20d4h0xvd5wya). The D0 classified all 22 unclaimed captures across the 10 populated witnesses as H1 (planner under-projection), zero H2 (elaborator over-capture): every capture is referenced by the body, none is spurious. Per the cardinality-gap frame's own routing, all-H1 means the fix grows the projection and revives the parked RT-CONTSRC-ENTRY-FRAME-WIDEN widening, and the two compose. Node scoping is the Steward's (COORDINATION section 3); the D0's disposition drives the cut and this is that cut."
---

# WHY THIS NODE EXISTS

Four consecutive results on this chain were each necessary but not sufficient
and each greened zero witnesses:

- [[RT-BRANCH-LOCAL-DECLARED-CALLABLE]] fixed the branch-local partition.
- [[RT-CAPTURE-SUPPLY-DECLARED-INPUTS]] measured that 25 of 30 planner claims are
  `ProducerLocal`, refused by `resolve_context_capture_claim`.
- [[RT-CONTSRC-ENTRY-FRAME-WIDEN]] measured that widening every one of those
  producer-local claims into the generated context's entry-source enumeration is
  sound and open -- and still greens zero, because the claims are the wrong
  population: the planner produces at most 2 claims against a 3-5 capture set.
- [[RT-CAPTURE-CARDINALITY-GAP]]'s D0 measured the CAUSE of that cardinality gap
  and classified it: **all H1, zero H2.** Every one of the 22 unclaimed captures
  across the 10 populated witnesses is a genuine, body-referenced value the
  planner never projects a claim for.

The D0 named the exact projection step (`AC-0`'s required half):
`continuations.rs:6075`, where a generated context is interned with

```rust
captures: enclosing_unit.key.continuation_inputs.clone(),
```

The generated context's capture set is **cloned from the enclosing
specialization's `continuation_inputs`** -- a projection of a different
population than the worker closure's own declared capture set. The `<=2` versus
`3-5` mismatch is not a projection that loses entries; it is a projection of the
wrong set. That is why the arithmetic held uniformly across all 25 closures in
the predecessor's D0.

This node closes the gap for the 10 populated witnesses. It is the real
remaining gate for [[NATIVE-HANDLE-CARRIER]] and [[PX8-F-CAP-41]] for this
population, which is why their `blocks` edge sits here.

# THE COMPOSED FIX -- two steps, D2 non-inert only because D1 supplies its input

The all-H1 disposition fixes the direction the three predecessors could not: the
fix **grows** the projection (it does not shrink the declared set -- there is
nothing spurious to prune here; that is what zero-H2 means). The added claims are
`ProducerLocal`, so they then need seating -- which is the widening
RT-CONTSRC-ENTRY-FRAME-WIDEN measured sound but witness-inert. It was inert
because no claims existed to seat; D1 supplies them, so D2 is now load-bearing.
The D0-first discipline paid off exactly here: the widening was correctly
declined when it greened nothing, and is built now that its input exists.

# DELIVERABLES

**`D1` -- grow the planner's capture projection.** At the projection that feeds
`continuations.rs:6075`, mint a planner claim for each of the closure's declared,
body-referenced captures that currently carries none -- so the generated
context's capture set covers the closure's declared set rather than the
enclosing specialization's `continuation_inputs`. The added claims are expected
to be `ProducerLocal` (the D0's grounding); D1 alone therefore still greens zero
(the claims are minted but not yet resolvable) -- its acceptance is a seam
property that the projection now emits a claim per referenced capture, measured
against the D0's per-witness capture counts, not a greened witness. The
Architect reviews the exact growth mechanism (derive-from-declared vs
union-with-declared) on the D1 candidate.

**`D1` AS BUILT (landed `9ab12ca97`, candidate `d722d4c79`, Architect APPROVE
evt_2hsm7j7stx2dp).** The growth does **not** land at `continuations.rs:6075`:
that clone site is downstream of the `ContinuationSpecializationKey` interning
key, whose `continuation_inputs` is identity-bearing and forbids post-assignment
widening. The real knob is **upstream** at `required_input_count` (a `max` over
the consumer's entry sources and eliminator case bodies via
`required_surrounding_environment_prefix`, which never consulted the worker
closure). D1 conditionally joins the recursive-position worker's required prefix
into that `max`, **fit-only** -- joined when `demand <= reached.len()`, never
forcing `required_input_count > reached.len()` (that count gates a fail-closed
whole-continuation refusal, so an over-demanding edge must not veto the fitting
edges in the same continuation, per the Architect's z34 (b) ruling). Every skip
is recorded as a `WorkerPrefixDeferral{depth,demand,reached}` (feature-gated
observability, production semantics unconditional) and classified (a) depth
mis-derived -> fix the walk, vs (b) genuine producer-local demand -> defer to D2.
`producer_binder_depth` treats a closure as a leaf (returns `None` = do-not-join,
never a wrong-small depth), arities mirrored from `shift_runtime_vars`. A found
constructor-identity bug in the join was fixed and affirmed. Effectiveness
(measured): `planner_claims` rose from `<=2` to `captures-1` across all ten
populated witnesses; px7f edge 2 recorded as a genuine (b) deferral (7 captures,
depth 1, demand 6, reached 4, excess 2).

**`D2` -- seat the producer-local claims via the entry-frame widening.**
Implement the RT-CONTSRC-ENTRY-FRAME-WIDEN widening to its measured spec: seat
each producer-local claim D1 mints as a real member of the **generated context's
own entry-source enumeration**. Per the Architect's soundness ruling
(evt_37mt6t65vvw39 / evt_75k8cydbj5127): the generated context is a distinct
frame with its own ABI arenas, so seating a producer-local as a member of the
CONTEXT's entry-source enumeration is **extend-membership, not relax-guard** --
`verify_entry_frame`'s membership check is **never relaxed**; the widening
extends the set the guard checks against. D1+D2 compose: the ten populated
witnesses green because every declared capture now carries a claim (D1) that
resolves against the context's entry frame (D2). D2 uses **D1's landed
`WorkerPrefixDeferral` ledger as its work-list**: every deferred edge the ledger
records is a producer-local value D2 must seat, and D2's AC (AC-6) asserts each
one greens once seated -- the ledger makes the D1->D2 handoff concrete rather
than a re-measurement.

# ACCEPTANCE CRITERIA

**`AC-1` (composed)** -- the ten populated witnesses from the cardinality-gap D0
green: the recursive-position lowering mints `Some(body)` where all captures
resolve, with no remaining word-only refusal on this population. D1 and D2 may
land as separate candidates; the greened-witness AC is met by the composition.

**`AC-2` (D1 seam property) -- MET at `9ab12ca97`.** The planner projection emits
a claim for each declared, body-referenced **outer-environment** capture, so
claim-count reaches the outer-environment capture count `captures - depth` where
the predecessor measured `<=2` (measured: `<=2 -> captures-1`, i.e. `5->4`,
`4->3`, uniformly across the ten witnesses). The target is `captures - depth`,
**not raw capture-count**: the one residual capture is the producer arm's own
local binder, not an outer-environment position, so it structurally cannot carry
a claim in this population -- `captures - depth` is the same depth-1 accounting
the whole fix rests on, and reading `4 of 5` as a shortfall would be a
misreading. The discriminating control is the committed
`rt_capture_projection_grow` test (mutation-proven: the unconditional-join
mutation reddens the planner-veto assertion), which shows D1 grew the projection
rather than the surrounding provenance.

**`AC-3` (D2 soundness control)** -- a producer-local claim that is NOT a live,
identity-preserving member of the generated context's entry run is still
**refused** by `verify_entry_frame` (fail-loud), and the widening extends
membership only for claims D1 mints from genuinely-referenced captures. A claim
seated for a value the context's entry run does not actually carry must fail
loudly, never silently miscompile.

**`AC-4`** -- conformance for the greened witness case.

**`AC-5`** -- no-regression in CI.

**`AC-6` (D2 composed-green over both edge classes -- the Architect's carried D1
obligation, evt_2hsm7j7stx2dp).** D2's acceptance locks the composed green for
**both** edge classes, using D1's landed `WorkerPrefixDeferral` ledger as the
checklist: (a) the **fitting** edges (D1 joined them into `required_input_count`
but D1 alone greens nothing) green once D2 seats -- this closes D1's
effectiveness half, which landed measured-not-asserted (no claim-count observer
was built, acceptable for D1 because the growth is inert until D2 seats); (b) the
**deferred** edges the ledger records (px7f edge 2 and every other row) green
once the widening seats their producer-local values; (c) a deferred edge that
**survives D2 still word-only** must be made **visible** (a named residual), never
silently accepted. This turns the ledger from D1 observability into D2's concrete
acceptance work-list.

# THE SEPARATE POPULATION -- 6 of 16, not this node's burden

The cardinality-gap D0 recorded 6 of the 16 witnesses as **reached-not-at-all**:
no generated context owns their closure body, so growing the projection does not
reach them. They are neither greened nor regressed by this node. If they need a
mechanism at all it is a separate successor, framed only if a measurement
demands it -- not carried here.

# BANNED SCOPE

- **Relaxing `verify_entry_frame`'s membership guard.** Inherited and load-bearing
  for D2: the widening extends the membership set, it never opens the guard. A
  claim that is not a real member of the context's entry run is refused, loudly.
- **Reading any capture value from the carried word.** Inherited from
  RT-CAPTURE-SUPPLY / RT-BRANCH / the cardinality-gap D0; still barred. The fix is
  in the projection and the entry-source enumeration, never in a carried-word read.
- **Elaborator free-variable pruning (the H2 route).** The D0 measured zero H2 for
  this population; pruning would green none of these witnesses. The
  over-capture at `erasure.rs:2210` remains a real latent defect and a fair
  standalone cleanup, but it is not this node's fix and is not on this critical
  path.
- **Reaching the 6 empty-population witnesses.** Out of scope; a separate
  successor if ever demanded.

# SEQUENCING

`depends_on: [RT-CAPTURE-CARDINALITY-GAP]` (closed at its all-H1 D0). Released as
the runtime ring's next node. `gate: none` -- runtime lowering: growing a planner
projection and extending a generated context's entry-source membership, no TCB or
trusted-reduction change. Tier **T1** (correctness-sensitive: D1 mints new
authority claims and D2 extends an ABI membership set that a soundness guard
checks against; a wrongly-seated claim is a miscompile). Review: **Architect**
(author is not reviewer), who reviews the D1 growth mechanism and the D2
soundness discipline on their candidates.
