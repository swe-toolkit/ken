---
id: RT-CLOSURE-CROSSING-ELIMINATE
title: "Eliminate the closure crossing instead of admitting it: carry the captured environment as an already-admitted Record and dispatch statically to the known body, so no Closure value ever reaches the boundary"
status: merged
owner: runtime
size: M
gate: none
depends_on: [RT-CLOSURE-BOUNDARY-LANE]
blocks: []
github: https://github.com/swe-toolkit/ken/pull/2327
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

> # THAT IS A FACT ABOUT THE TREE, NOT A PROHIBITION — AND THIS FILE READ AS ONE
> # Steward, 2026-08-15, on the operator's challenge to ground each constraint
>
> **The paragraph above is true and stays.** What does not survive is the
> inference drawn from it downstream — in this file's `AC-2`/`AC-4`, in the
> handback prose, and in the Steward's own reporting: that **a live-domain
> closure lane is banned**. The spec says the opposite.
>
> `spec/40-runtime/41-values.md:76-83`, the landed callable boundary:
>
> > **Live-domain invocation only.** Separately compiled artifacts may exchange
> > an ordinary closure only within one live runtime domain, while the defining
> > owner and artifact remain live. The receiver may invoke it at its checked
> > callable type, but may not inspect, serialize, persist, reconstruct, or use
> > it as stable identity.
>
> and `:116-118`, from the same revision:
>
> > This chapter fixes the observable validity boundary, not its
> > implementation. It requires no particular handle or trampoline,
> > owner/lifetime encoding, allocation scheme, GC strategy, or memoization
> > scheme.
>
> ⇒ **Cross-artifact live-domain closure exchange is the SANCTIONED shape, and
> the chapter deliberately declines to constrain its mechanism.** That
> minimum-constraint clause is not incidental: it exists because a
> stronger-than-mission constraint, faithfully implemented, produced six
> consecutive Architect production blocks on `RT-FNSPLIT-B2V` — see
> [[SPEC-CLOSURE-BOUNDARY]] and the operator directive it quotes.
>
> **What IS prohibited is the DURABLE lane, and only that.** `41-values.md:73-76`
> makes a closure transitively non-persistable, with the mission failure named at
> `:103`. The retired pair is `(PersistentClosure, Closure)` — **persistent** —
> and `BOUNDARY_RETIRED_LANES`' own doc states the point is *"a
> recognition/admission split, not a revived capability."* Reviving that pair
> stays banned. A live-domain pair is a different pair and a different question.
>
> **Two supporting claims also need attribution, because both were restated as
> law:**
>
> - **`BOUNDARY_TAG_CLASS_RELATION` is not the authority.** Its own doc: *"NOT
>   the authority, and NOT derived — a hand-written Rust MIRROR"*, reconciled in
>   both directions against the `BoundaryInput → BoundaryOutcome` partition. So
>   "adding a row to the relation" is not the act; changing the partition is, and
>   the mirror follows. This is a **mechanism**, not a gate.
> - **"Do not undo the unit boundary"** is the runtime ring's judgment recorded
>   in a hard stop, not a ruling. Cite it as that.
>
> **The one measured constraint here is narrower than it was quoted as.** The
> eight-call-site argument under `AC-3` says *do not admit at the shared gate*,
> because `boundary_transfer_admissibility` (`lowering/mod.rs:6613`) serves eight
> call sites of which two are classified. **That is an argument about WHERE, not
> about WHETHER.** It survives unchanged and is the only one of the four that was
> ever grounded in a measurement.
>
> ### THE QUESTION THIS OPENS, WHICH IS THE ARCHITECT'S
>
> **Does the generated-unit boundary sit inside "one live runtime domain, while
> the defining owner and artifact remain live"?** If yes, the spec already
> licenses this crossing and the work is building the owner/lifetime encoding and
> the refuse-before-invocation check `:81-83` requires — a real node, but a
> **specified** one rather than an invention. If no, the crossing is a durable
> export wearing a live-domain name and today's refusal is correct on the merits.
>
> **Resolve this BEFORE the [[RT-DESCENT-RETIRE]] product fork is ruled on.** The
> fork was put to the operator as *cover it (requires inventing a representation)
> / accept the narrowing / stop* — and the first option was priced against a
> prohibition that does not exist.

## The finding this node is built on — do not restate it as the predecessor's

**Architect, `evt_2nwtjekh4qtnk`.** The predecessor's disposition text says the
row remains refused until a successor **carrier** exists. The measurement
establishes something narrower than that sentence:

> **The closure *value* cannot cross. That is not the same as: the *capability*
> requires a carrier.**

A true measurement does not entail what the sentence built on it claims. The
predecessor's text forecloses in prose a route the measurement leaves open, and
this node is that route.

## THREE GUESSES HAVE MISSED. THE SHARED PREMISE IS THE UNIT BOUNDARY.
## Steward, 2026-08-15, on runtime's `D2k-2` stop `evt_1rq93nqwe1jtr`.

**Three repair guesses in a row have been refuted on this campaign, all three
mine.** Individually each looked like a different problem. Read together they
are one wall seen from three sides, and **that synthesis is what this node
should be sized against.**

| guess | what refused it |
|---|---|
| admit the closure crossing on clause 2's predicate via `B2F`'s carrier | no admitted live-domain lane for `BoundaryClass::Closure` exists in the ABI |
| recognize the static worker at the computational-producer `Construct` arm | the arm is never reached; recognition already happens earlier |
| route the conservation consumption through `constructor_field_bindings` | the consumer is in a detached ordinary unit; the result crosses as a runtime carrier word, not a compiler-only `ConstructorField` slice |

**The shared premise: that the repair lives on the compiler-side traversal
inside one unit.** All three assumed the static worker and its consumer sit in
the same compilation unit, so a compiler-only representation could carry the
fact between them.

**The measurement says otherwise, and now says it from both ends.** Runtime
measured that for both conservation rows, match-binder descent happens **before**
worker recognition; recognition occurs later in the detached ordinary unit
(`units.rs:6143`), and the non-root result exits **only** via
`transfer_unit_result_into_carrier` (`units.rs:6227-6234`) and
`call_declared_unit_target`, which yield a **runtime carrier word**. Their
conclusion: routing through `constructor_field_bindings` would require **a
cross-unit result edge or representation**, or undoing the unit boundary.

=> *"No admitted `Closure` lane at the boundary"* and *"no lawful
`ConstructorField` input at this consumer"* are **the same unit boundary,
reported from opposite sides.**

### What this means for THIS node, stated as a question and not an assumption

**This node's route is a cross-unit representation** -- carry an admitted
`Record` across the boundary and dispatch statically. That is precisely the
thing the third guess found missing.

=> **The two conservation rows are a CANDIDATE member of this node's
population, not a confirmed one.** If the same cross-unit representation serves
them, this node closes four expressions rather than two and the campaign's
remaining size collapses. If it does not, they need separate treatment.

> **Check this immediately after the signature-machinery joint, and check it as
> a question.** I have **not** verified that the conservation rows' captured
> environment is record-shaped, nor that one carrier serves both shapes.
> **Pooling them for sizing is not assuming one mechanism** -- that error has
> already been made once on this campaign and is called out in
> [[RT-LEXICAL-RECURSOR-CONSUMERS]].
>
> **If the answer is no, do not widen this node to chase them. Hand back.**

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

**`D2a` — the co-population question, answered.** State whether the two
conservation rows (row 4 depth 1, row 5 after-hole) are served by whatever `D1`
produces. **A recorded "no, and here is what refuses them" is a complete
answer** and is worth as much as a yes: it is the input that decides whether the
campaign still has one repair left or two.

**`D2b` — row 1's disposition, which no increment has ever addressed.** Row 1
owned-scope stands at `NativeJoinPlanV1`, a construct unrelated to every wall
this campaign has worked. Under the closure criterion it owes a disposition and
**has neither a repair nor a recorded refusal.** Either the route in `D1`
reaches it — repair it — or **state its wall and what discharging it would
take.**

> **This is a disposition deliverable, not a measurement node.** It is one
> expression; naming its wall is a paragraph, not a turn. It is called out
> separately because it is the expression most likely to be read as closed —
> the other four have all moved, so a reader sweeping the campaign sees motion
> everywhere and row 1 goes quiet.

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

> **This is a SCOPE BOUNDARY ON THIS NODE, not an architectural prohibition.**
> Read the clause after the comma: a candidate needing a new admission **has left
> this node's route**, so the criterion is a handback trigger. It says *"if you
> need this, stop and hand back"* — it does not say the admission may never
> exist, and it is not evidence about what the ABI is permitted to grow. The
> banner above gives the spec position; `AC-2` is silent on it.

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

> **Each of the four clauses here is grounded, and NONE of them generalizes to
> "no live-domain closure lane."** `FrozenClosure` and `StaticCallableRef` are
> the two explicit separate abstractions `41-values.md:88-96` requires be kept
> distinct from an ordinary closure; the retired durable lane is the persistent
> pair the same chapter forbids at `:73-76`. **The generalization was the
> Steward's and it is withdrawn** — see the banner near the top of this file.

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

## The stop that is NOT a failure, named in advance

**A cross-unit result edge or representation is exactly what the third guess
found missing, and building one is a design, not a wiring job.** If `D1` finds
that eliminating the crossing requires inventing that representation rather than
using an admitted one, **that is the stop** — hand back with the mechanism
named.

**Do not undo the unit boundary to make the repair fit.** Runtime named that as
the alternative and correctly refused it without a release; it stays refused.

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

> **The fork has a PRIOR question and cannot be ruled on until it is answered.**
> A narrowing is only a narrowing against what Ken is **specified** to do.
> `41-values.md:76-83` specifies live-domain cross-artifact closure exchange, so
> if the generated-unit boundary sits inside one live runtime domain, these rows
> are refusing something the spec grants and the disposition is a **gap**, not a
> permitted narrowing. If it does not, the refusal is correct on the merits and
> the fork is real as posed. **Architect question; route it before the operator
> is asked to choose.**

## Provenance

Architect verdict and finding: `evt_2nwtjekh4qtnk`, `dec_650dc1x38n4jh`
resolved. Predecessor candidate: exact `5e8907597446ee524b2e8ab11804f1e1a30488ac`
from `wp/RT-CLOSURE-BOUNDARY-LANE`, QA `evt_2th73cw9fsfgt`. All measurements
above are the predecessor's or the Architect's, re-cited to their sites; none is
this frame's own assertion.
