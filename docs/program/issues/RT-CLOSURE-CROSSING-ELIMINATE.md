---
id: RT-CLOSURE-CROSSING-ELIMINATE
title: "Eliminate the closure crossing instead of admitting it: carry the captured environment as an already-admitted Record and dispatch statically to the known body, so no Closure value ever reaches the boundary"
status: ready
owner: runtime
size: M
gate: none
depends_on: [RT-CLOSURE-BOUNDARY-LANE]
blocks: []
github: null
origin: "Steward, 2026-08-15, from the Architect's finding on the RT-CLOSURE-BOUNDARY-LANE failed repair attempt (evt_2nwtjekh4qtnk, dec_650dc1x38n4jh). The Architect named the route and routed the framing to me explicitly: the successor question is whether the crossing can be ELIMINATED, not only which carrier admits it. Steward-filed per COORDINATION section 2."
---

## Why this node exists

[[RT-CLOSURE-BOUNDARY-LANE]] made an a priori repair guess, attempted it, and
the guess was refuted from the inside by its own second constraint. That is the
node working as framed, and the refutation is what this node is built on.

**What the predecessor established, at `crates/ken-runtime/src/boundary_value.rs`:**

- `BOUNDARY_TAG_CLASS_RELATION` (`:670`) is mechanically reconciled to the
  partition-derived relation over the full `BoundaryTag::ALL × BoundaryClass::ALL`
  product in both directions, and drift either way reddens. The language is
  **enforced-closed**, not merely written down.
- `BoundaryClass::Closure` appears in **exactly one** row: `(PersistentClosure,
  Closure)` at `:683`.
- That pair is the **sole** entry in `BOUNDARY_RETIRED_LANES` (`:723-724`), and
  `boundary_relation_admits` returns `false` for it **before reading the schema
  at all** (`:769+`).
- `InvocationAggregate` admits **exactly** `Constructor` and `Record`
  (`:706-711`).

⇒ There is no admitted live-domain lane for `BoundaryClass::Closure` anywhere
in the ABI, and carrying a first-class closure across the generated-unit
boundary requires a **new `(tag, class)` admission**.

## The finding this node is built on — do not restate it as the predecessor's

**Architect, `evt_2nwtjekh4qtnk`.** The predecessor's disposition text says the
row remains refused until a successor **carrier** exists. The measurement
establishes something narrower than that sentence:

> **The closure *value* cannot cross. That is not the same as: the *capability*
> requires a carrier.**

A true measurement does not entail what the sentence built on it claims. The
predecessor's text forecloses in prose a route the measurement leaves open, and
this node is that route.

## The a priori best guess — build this

**Operator ruling, 2026-08-15: frame the repair, state the guess as an
attackable claim, attempt it. Do not open with a measurement.**

> **Carry the captured environment across as an already-admitted `Record`, and
> dispatch statically to the known callee body. No `Closure` value is
> constructed at the boundary, so no new `(tag, class)` admission is needed and
> the enforced-closed relation is untouched.**

Two of the three legs are already measured, in the predecessor:

1. **`InvocationAggregate` already admits `Record`** (`boundary_value.rs:706-711`)
   — a record-shaped captured environment crosses on an **existing admitted
   lane**.
2. **The attempt's own disposition states that `B2F` directly calls a
   statically selected closure body**, at
   `crates/ken-runtime/src/cranelift_backend/lowering/core.rs:18334-18410`, with
   its inputs crossing as the closed one-word `AbiCarrier::ValueWord` at
   `crates/ken-runtime/src/cranelift_backend/planning/static_transition/abi.rs:66-89`.
3. **`D1` measured the callee bodies here as statically known** — exact origins
   49 and 59.

## The joint that is NOT measured, and it is the first thing the attempt hits

**Stated plainly because the Architect stated it plainly: he did not measure
this, and is not asserting it is available.**

> **Whether `B2F`'s generated-unit signature machinery permits splitting a
> closure-typed parameter into (environment record, statically selected body) is
> unknown.**

**Attack it first, in code, not in a survey.** If the signature machinery cannot
express the split, that is the handback and it is a real deliverable — name the
exact mechanism that refuses and what it would take, and stop. Do not design a
replacement signature machinery inside this node.

> **This joint is unhunted, and it will stay unhunted until this node merges.**
> The Adversary offered to attack it **ahead of** the merge rather than behind
> one (`evt_6z5scnv9h7x6j`), and noted that its own posture is to stay behind
> unless routed. **`COORDINATION §10⁻a` makes the edge report-only — the merge
> notification is the Steward's sole outbound message — so there is no lawful
> way for me to route that hunt.** Recording it here rather than declining by
> reply, which the same rule forbids.
>
> ⇒ **The ring is the only party who will attack this joint before it is built
> on.** Size the attempt accordingly and do not treat the joint as
> independently reviewed.

## This route is NOT the spec-forbidden one. Know why before you start.

`spec/40-runtime/41-values.md:84-91` forbids **silently converting an ordinary
closure into a `StaticCallableRef`-class value** as an empty-capture
optimization. That prohibition is live and binds here.

**It does not bind this route.** The prohibition is on producing a
`StaticCallableRef`-**class value** — a stable, serializable callable identity
qualified by package/artifact, callable unit/export, and ABI signature. This
route produces **no such value and nothing serializable**: the environment
crosses as a `Record` and the body selection is a compile-time dispatch
decision that never becomes a boundary value of any class.

**If the attempt finds itself constructing a callable identity that outlives the
call, it has left this route and entered the forbidden one. Stop there and hand
back.**

## Deliverables

**`D0` — anchor the `D0` control's non-vacuity. Adversary finding
`evt_6z5scnv9h7x6j`, CONFIRMED and accepted.**

`recursive_descent_recursors_compile_without_a_boundary_crossing` runs with
`set_selector_variant_exclusion(None)` and asserts `crossings.is_empty()`.

⇒ **That assertion is satisfied both by *"`RecursiveDescent` performs no
crossing"* and by *"the recorder did not fire in this configuration, for any
reason at all."*** `assert!(result.is_ok())` establishes that the fixture
compiled; it does **not** establish that the rig would have recorded a crossing
had one occurred.

**The same commit contains the discipline this control omits.** Its sibling
asserts `!baseline_callees.is_empty()`, and the `D7` control states the reason
outright — *"NON-VACUITY: ... or the substitution below is inert for the trivial
reason that nothing was there."* **This is the only one of the three that
measures an absence without first proving the instrument is live.**

**The repair is small and the rig already exists.**
`required_consumer_route_manufactures_...` drives the **same** expression,
`host_result_closure_match(px8j_scope_chain_observation_result(depth, 0))`, with
`set_selector_variant_exclusion(Some(LexicalCallArgumentRecursor))` and observes
crossings. Run that arm inside `D0`'s own loop and assert it is **non-empty**.

> **Why this is worth a deliverable at low severity.** A dead recorder would red
> the neighbouring control, so the corpus catches the failure even though this
> test would not. **What is at stake is attribution.** `D0` is the row that will
> be cited when anyone argues that retiring `RecursiveDescent` removes a
> capability — including the product fork recorded at the foot of this node —
> and it currently offers an **unanchored empty set** as that evidence. Turn
> *"we saw nothing"* into *"we saw nothing where this same rig, in this same
> process, sees something."*

**`D1` — the repair, per the guess.** The closure crossing is eliminated for
the predecessor's owned rows: recursor row 4 depths 2 and 3, and
`rt_escape_second_resource_native`
`escaped_resource_used_by_fanning_host_op_matches_interpreter`.

**`D2` — the disposition record.** Whatever `D1` produces, every expression in
the population carries a recorded disposition: repaired, or refused with its
spec clause cited and its pre-retirement behaviour accounted for. This is the
ratified closure criterion for [[RT-LEXICAL-RECURSOR-CONSUMERS]] and it is
**not** compile-green.

**`D3` — the un-skip, or the exact reason it cannot happen.** The escape row is
skipped. If `D1` succeeds, un-skip it and demonstrate the native and interpreter
paths agree. If `D1` stops, state exactly what the un-skip still waits on.

## Acceptance criteria

**`AC-1`.** `D1` attempts the stated guess directly. A handback that reports the
signature machinery refuses the split, naming the exact mechanism and site,
**satisfies this criterion** — a refuted guess is the deliverable when it is
refuted by an attempt.

**`AC-2`.** No new `(tag, class)` admission is added to
`BOUNDARY_TAG_CLASS_RELATION`, and `BOUNDARY_RETIRED_LANES` is unchanged. The
relation is mechanically reconciled in both directions, so this is enforced
rather than reviewed — but state it, because a candidate that needed to touch it
has left this node's route.

**`AC-3`.** **No admission is installed at the shared gate.** This carries
forward unchanged from the predecessor's `AC-3a` and the reason has not
weakened: `transfer_into_carrier` has **eight** non-test call sites all funnelling
through **one** `boundary_transfer_admissibility` call at
`crates/ken-runtime/src/cranelift_backend/lowering/mod.rs:6613`, and only **two**
of the eight are classified. Admitting at the gate refuses less on six
unclassified routes. Demonstrate the six reach today's refusal unchanged, rather
than asserting it.

**`AC-4`.** No `FrozenClosure`-class value, no `StaticCallableRef` conversion,
no revival of the retired durable lane, and no weakening of the refusal or its
message. If the repair succeeds, the refusal arm still refuses everything it
refuses today — the crossing is gone, not permitted.

**`AC-5`.** Nothing added to a production build surface that was not there
before, verified by a targeted control rather than by inspection.

**`AC-6a`.** `D0`'s non-vacuity anchor is demonstrated, not asserted: the
excluded-variant arm inside the same loop, in the same process, records a
**non-empty** crossing set while the unexcluded arm records an empty one.
**A green that does not show both halves does not satisfy this.**

**`AC-6`.** No-regression, in CI (`COORDINATION §12`).

## Banned scope

- **A new boundary carrier.** That is the route the predecessor proved
  unavailable without a new admission. If this node concludes a carrier is the
  only way, that is a handback to the Architect, not a design to start here.
- **Designing `B2F`'s signature machinery.** Attack whether it permits the
  split; do not rebuild it.
- **Retiring `RecursiveDescent`.** [[RT-DESCENT-RETIRE]] is downstream and is
  not this node.

## The fork this node does not settle, recorded so it is not lost

`D0` of the predecessor measured that the descent lane **does not** perform an
equivalent crossing, so retiring `RecursiveDescent` **removes a presently
compiling capability** unless this crossing exists by some route.

The ratified closure criterion decoupled [[RT-DESCENT-RETIRE]] from this lane's
**sizing** — the recursor rows close on a recorded disposition, and "conservative
clause-2 refusal" is a recorded disposition. **So retirement can proceed with
these rows refused, and would then ship a narrowing.**

**Whether that narrowing is acceptable is a product call, not a runtime one, and
it is not this node's to make.** It changes this node's priority and nothing
about its content, which is why it is recorded here rather than blocking.

## Provenance

Architect verdict and finding: `evt_2nwtjekh4qtnk`, `dec_650dc1x38n4jh`
resolved. Predecessor candidate: exact `5e8907597446ee524b2e8ab11804f1e1a30488ac`
from `wp/RT-CLOSURE-BOUNDARY-LANE`, QA `evt_2th73cw9fsfgt`. All measurements
above are the predecessor's or the Architect's, re-cited to their sites; none is
this frame's own assertion.
