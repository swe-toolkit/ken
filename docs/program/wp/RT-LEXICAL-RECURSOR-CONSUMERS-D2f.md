# RT-LEXICAL-RECURSOR-CONSUMERS D2f — the emission plane

Owner: runtime. Size: M. Node: [[RT-LEXICAL-RECURSOR-CONSUMERS]] (`#6d`).
Architect rulings `evt_2wwh9yamyhs7p` (the mechanism) and `evt_6sk3czsbcr85r`
(the `StaticContinuationFusion` class).

**Held pending [[RT-LEXICAL-RECURSOR-CONSUMERS-D2e]] — see Contention.** Fixed
inputs will be measured at whatever `main` carries `D2e`; **do not take a SHA
from this frame**, and do not start before I release it.

**Seat tier: T1.** The `#8` suspension does not reach `#6d`.

## What this deliverable is

The second half of the cut. `D2e` builds the identity plane — the slot-role
derivation, `StaticContinuationFusionId`, its key, interning, and the exact
re-derivation validator. **This deliverable emits from it**, and it carries
`D2d`'s original `AC-1`..`AC-3`, which no planner-only work can discharge.

**The identity is a fixed input here.** If you find yourself re-opening the key,
the slot-role derivation, or the fail-closed refusal, that is `D2e` work and it
comes back to me rather than being redone under this frame.

## Deliverables

**0. The production-path gate. ADDED 2026-08-11 (Architect `evt_6vf66hmwv52y6`).
Commit this BEFORE any emitter definition is written.** It is a deliverable, not
a preflight, because it is the thing that establishes the emitter has a
non-vacuous subject at all.

One committed control carrying these four results **side by side**:

| input | required result |
|---|---|
| old exact `px8j` seed witness | exactly one builder arrival, `oriented_present=false`, resolved plane `0`, and **no** key / ID / descriptor / definition |
| checked exact `D2j` positive | exactly one builder arrival, `oriented_present=true`, resolved plane `1`, exactly **one** key / ID / descriptor, closed seven facts matching that fixture's independently re-derived planner facts |
| strip or relocate **one** checked marker, plan held fixed | exact **validator refusal**, before any key / ID / descriptor / definition |

> ### THE POSITIVE IS THE ABI-APPLIED ENTRY, NOT THE BARE REFERENCE
>
> **Architect ruling `evt_6907h4rv5kq1a`, 2026-08-11.** The gate exact
> `e4531318` is a **sound identity-plane partial and merges**, but **its
> bare-root observation does not carry across this correction.**
>
> **The governing invariant is one end-to-end PROGRAM, not one declaration
> body.** The complete fusion key contains positional and provenance facts
> derived from the **planned program**, so *"the applied form was derived from
> the same bare declaration"* **does not** prove the emitter observed the
> identity the planner controls established.
>
> **The bare `DeclarationRef` cannot state the emission contract at all** —
> root projection stops at `Unsupported(Closure)`. **A shape that cannot reach
> the claimed definition movement cannot be the canonical positive.**
>
> **Fix at the sole fixture authority, never at callers.** For the canonical
> `Exact` positive, `d2j_entry()` itself returns the applied root (**the root is
> cause-aware — see the per-cause family below**):
>
> ```rust
> RuntimeExpr::Call {
>     callee: Box::new(RuntimeExpr::DeclarationRef {
>         symbol: D2J_DECLARATION.to_string(),
>     }),
>     args: vec![unit(), unit()],
> }
> ```
>
> The two arguments correspond exactly to the declaration's `params: ["a","b"]`;
> they are closed ordinary `Unit` values and the witness body takes no authority
> from them. `d2j_checked_fixture_under(cause)` remains the **only** exported
> constructor. **No caller may re-wrap it, construct a second entry, or retain a
> planner-only bare-entry helper.**

### The root family is PER CAUSE. One constructor, not one identical root.

**Architect `evt_4trsqtkxtghjx`, correcting the uniform-root wording above as
over-broad — including the Steward's restatement of it.** **One fixture means
one cause-aware constructor, not one identical root across causes that
deliberately change callable arity.**

| cause | entry |
|---|---|
| `Exact`, `Frame`, `SelectedSlot`, `Invocation`, `ExactSuffix`, `CallIdentity`, `ProducerArity` | `Call(DeclarationRef(D2J), [Unit, Unit])` |
| **`ReHomed`** | **bare `DeclarationRef(D2J)`** |

**`ReHomed` needs its own branch because that cause itself removes the outer
`LexicalClosure` and therefore has ZERO ABI inputs.** Applying two `Unit`s to it
manufactures an **ill-typed program**, and the resulting `Unsupported(Call,
callee is not a closure)` would then be evidence **about the test harness, not
about fusion.**

**Use an explicit `D2jCause::ReHomed` branch inside the one constructor.** Do not
add a second fixture constructor, do not re-wrap the re-homed declaration, and
**do not infer the entry by inspecting the mutated body** — an explicit branch
keeps the entry contract independently reviewable and stops it drifting along
with a malformed source mutation.

**The sharing invariant is PER CAUSE**: planner, production-gate, and emitter
controls for `Exact` all consume the same applied exact object; those for
`ReHomed` all consume the same bare re-homed object. **Different causes are not
required to share an outer root.**

> **STRUCK — *"A bare `DeclarationRef` may remain only as an explicitly named
> root-projection NEGATIVE comparator. It is not the `D2f` positive and it
> discharges no emitter AC."*** **The Steward wrote that while relaying the
> uniform-root ruling, and it is wrong for `ReHomed`.** The bare re-homed root
> **is a positive** — `AC-6b`'s provenance / non-aliasing positive. What remains
> true is narrower: **the canonical `D2f` positive is applied `Exact`**, and a
> bare root is not *that*.

**REBASELINE; DO NOT TRANSPORT LITERALS.** The outer `Call` can shift planned
origins, so **every `D2j` planner control runs on its own cause-selected root**
and **freshly re-derives** the closed-seven coordinates, complete key,
descriptor, and refusal coordinates. **If any seven-fact member or
non-degeneracy control stops holding, STOP and report** — do not copy the
bare-root key and do not add an origin translation.

### The causal chain, committed in this order

1. The shared constructor yields **one entry / declaration / plan triple for the
   cause under test** — applied for `Exact`, **bare for `ReHomed`**. The sharing
   invariant is **per cause**, not one root across causes.
2. The **planner** control independently derives exactly one complete key and
   descriptor from that triple.
3. `compile_expr_into_object_module` on the **same triple** independently
   reaches one production key and descriptor, and **exact equality with the
   planner observations is asserted.**
4. The future emitter consumes **only that production plane in the same
   compile**, bound to that compilation-local key ↔ ID ↔ descriptor. **It never
   reads the planner comparator.**
5. **Before emission**, that same cause-selected root reaches the twin's exact
   ordinary `ComputationalMatch` refusal with plane `1` and definition count
   `0`. **This holds for `ReHomed` on its bare root too**, with its own
   populations — see `AC-6b`.
6. **With emission enabled**, the same root retains plane `1` and moves the
   independently observed generated-definition / redirection population
   **`0 → 1`**.
7. **Fusion-only suppression** leaves builder arrival and plane `1` intact,
   moves definition/redirection **`1 → 0`**, and restores that same ordinary
   refusal. **The plane and definition populations must be operands of ONE
   assertion**, so suppression cannot pass on a resting zero.

**No emitter AC may be credited until the positive row is non-zero.** That is the
whole point of ordering this first: every emitter criterion in this frame
discharges **vacuously** against a permanently empty plane, and a no-activation
proof over nothing emitted passes for free.

**Once emission is added**, the same positive must move definition count
**`0 → 1`**; and fusion-only suppression must keep builder arrival and the
resolved plane **non-zero** while moving definition/redirection count
**`1 → 0`** and restoring the checked twin's own ordinary refusal.

**The existing `planes=[0], oriented_present=[false]` result is accepted as the
old witness's negative evidence** and does not need re-measuring. It says nothing
about the gates below `oriented`, which **remain unmeasured** until the checked
positive reaches them.

**1. The ABI arm.** A separate exhaustive `AbiUnitDefinition::StaticContinuation`
`Fusion` definition with its descriptors. **Only ordinary tagged inputs and
normal outputs.** Activation, cursor, selection and unwind state must never enter
a descriptor, slot, carrier, capture, parameter, tag, or target lane.

**2. `ContinuationEmissionOwner::Fusion`.** Generated ownership is `Fusion(id)`,
not `PredeclaredFunctionId(0)` or `(2)`. The measured owners in `D2d`'s grounding
record are the *original* consumer and producer owners; the fused region is a
third thing and must own itself.

**3. Scoped source-body authorities.** Every source origin lowered into the
generated definition is validated to its planned source-body owner. The suffix
is lowered under a **separately validated** authority — not under the producer's.

**4. The generated-definition emitter.** The generated function receives the
ordinary producer operands plus the projected suffix inputs, executes the
producer **once, at the original CBV point**, keeps the activation local, and
exposes **only the closure-free final result**. The suffix `Call` is the sole
consumer of that activation.

**5. Exact edge redirection.** Redirect **only** the exact original producer
invocation. Not every edge to that callee, and not a search for a plausible one.

**Derive that edge from the complete production key's invocation identity —
caller and callee — and validate it before redirecting.** Redirect only after
the full key and the local descriptor agree. The frame states the derivation;
it does not state the coordinate.

> **This deliverable previously named a literal `0 -> 2` edge, and that edge
> does not exist on the checked `Exact` twin.** `0 -> 2` was measured on the
> retired `px8j` witness, and **this frame is where it lost its scope.** Its two
> upstream sources were both correct: `D2d-GROUNDING` records it under "measured
> coordinate **on this witness**", and `D2e` says in as many words *"do not
> re-derive those coordinates; derive the mechanism that produces them."* `D2f`
> restated it as a bare requirement with the witness qualifier dropped — which
> is the whole defect, since dropping a measurement's scope always strengthens
> the claim. On the
> checked twin the invocation is caller **3**, callee **2**; unit **0** is a
> `SchedulingEntry` and invokes nothing — so the old text prescribed an absent
> edge. Struck on `runtime-leader`'s measurement (`evt_4xktmhfam8gyy`,
> 2026-08-11).
>
> **`3 -> 2` is this fixture's observed control coordinate, not production
> authority.** Do not hard-code it in place of the struck one. An implementation
> that reads the coordinate from the key may *assert* the derivation yields
> `3 -> 2` on this witness; one that hard-codes `3 -> 2` has reproduced the same
> defect one witness later.
>
> The general lesson, since this frame is not the last to inherit a coordinate:
> **pin a frame against the derivation, never against the number it produced on
> the witness that happened to be current.** A number survives the witness it
> was measured on and stays syntactically valid after it stops being true.

## The obligation this deliverable inherits, and it is the live one

**The emitter/ABI stop condition is undischarged, and `D2d` says so in its own
words.** Its §4.3 established only that no activation appears in the *projected
inputs* of the `R3` witness. **A projection cannot prove that a generated
definition nobody had written would never export activation state.** That
sentence was the Architect's block on the first `D2d` candidate, and the record
was recut to keep it live.

⇒ **You are the turn that discharges it.** Treat "the projection contained no
activation" as grounding for where to look, never as evidence that the emitter is
clean.

## Acceptance criteria

`D2d`'s `AC-1`..`AC-3` and `AC-5`, carried forward unchanged, plus the emitter
obligation.

> # AC-1 AND AC-2 REBOUND 2026-08-11 — the old fixture binding was UNSATISFIABLE
>
> **Architect ruling `evt_6vf66hmwv52y6`**, on a Runtime hard stop. **The
> correction is required, not wording cleanup, and the defect is the Steward's.**
>
> **STRUCK — *"On the exact `R3` before-hole compile under B-only exclusion
> (fixture `px8j_equal_payload_hole_placement(BeforeReturnHole)`)…"* as the
> POSITIVE.** That witness was deliberately preserved as the **unmarked
> negative**: it carries no checked frame, no selected-IH-slot, and no
> checked-IH-invocation marker, so **it is not a fusion candidate and cannot be
> made into one.** `validate_oriented_subcontinuation_transport` makes that
> boundary structural — unmarked IR with `None` is lawful seed IR producing no
> fusion; unmarked IR with a non-empty plan is a marker/plan mismatch that must
> **reject**; an empty supplied plan carries no checked transport coordinate;
> and adding wrappers changes the occurrence tree, so it is no longer the exact
> fixture.
>
> ⇒ **The frame pinned acceptance to a witness that cannot carry the mechanism's
> required input.** This follows from the landed required-member ruling and was
> true when the frame was written — today's `planes=[0]` measurement revealed it,
> it did not cause it.
>
> **The old witness is NOT deleted from the frame. It is the absence /
> ordinary-refusal comparator**, and it must never again be described as the
> fusion-positive.

**AC-1 — the positive full-pipeline baseline is the CHECKED `R3`-shaped `D2j`
exact witness, in its ABI-APPLIED form.** Reuse the landed `D2g`/`D2j` checked
fixture and its complete, independently authored `OrientedSubcontinuationPlanV1`.
**Do not duplicate it and do not re-hand-wrap it** — `d2j_checked_fixture_under`
`(cause)` is the one exported constructor, and for this criterion's `Exact`
cause it returns the **applied root** (`Call { callee:
DeclarationRef(D2J_DECLARATION), args: [unit(), unit()] }`), so the planner tests
and this full-compile control consume the **same** entry, transparent
declaration, and plan. **The root is cause-aware**: `ReHomed` takes the bare
reference, and that is `AC-6b`'s positive, not this one's.

**The bare `DeclarationRef` is NOT this baseline** (Architect
`evt_6907h4rv5kq1a`) — root projection stops at `Unsupported(Closure)`, so it
cannot state the emission contract or reach the claimed definition movement. It
survives only as an explicitly named root-projection negative comparator and
**discharges no emitter AC.**

On that witness, the inner `Node` recursor closure and its exact activation and
cursor are produced and consumed by the downstream call **in the same specialized
unit**, with **zero** `transfer_into_carrier` attempts for that intermediate
`Node[ComputationalRecursorClosure]`.

**The control must enter `compile_expr_into_object_module`** with that
declaration map and `Some(oriented)`, so it traverses
`compile_expr_into_module_with_root_projection`, production static planning,
exact transport validation, candidate enumeration, full-key re-derivation,
interning, and later emission. **It must not call the fusion builder or the
emitter directly** — a control that bypasses the planner cannot see the path
these criteria are about.

**AC-2 — the A/B witness, bound to the APPLIED ROOT's own coordinates.**
Suppressing **only** the continuation fusion restores the applied root's **own
ordinary `ComputationalMatch` refusal**, at its **freshly derived** planner
coordinates. Suppressing anything else does not count.

**Re-derive those coordinates; do not transport them.** The outer `Call` can
shift planned origins, so every coordinate in this criterion comes from the
applied root's own planning run.

> **STRUCK — *"restores the measured origin-23 refusal."*** Origin 23 is the old
> `px8j` coordinate. **The checked twin's origins, owners, and edge are its own
> and are never copied from `px8j`** — its fresh planner coordinates and complete
> seven-fact key are authoritative.

**The denominator must be an operand of the assertion, never an argument to its
message.** This is the standing form of the finding `D2e` repairs: a value read
only inside a format string is removable by a message edit, and `unused_variable`
is advisory in this crate — no `deny` in `crates/ken-runtime/src/lib.rs`, no
`-D warnings` in the workflows. **Do not restate the guarantee in a comment**;
make the clause that establishes the suppression worked inseparable from what
establishes the run was non-empty. Then prove it: shorten the message and the
control must still red.

**AC-3.** A closure-free sibling result continues through the ordinary carried
join **unchanged**.

**AC-4 — the emitter exports no activation.** Not "the projection contained
none" — a property of the emitted definition. State how you established it and
what would have caught the negative case. **A check that passes because nothing
reached it is the failure mode here**, so it needs a positive control: a shape
that *would* export activation must be refused.

**AC-5 — `D2b`'s controls are retained** and still prove `Closure` and
`DeclarationClosure` unconditionally non-transferable at every depth, and
`call_declared_unit_target` free of any closure lane.

**AC-6 — the fail-closed boundary survives emission.** Emission must not
introduce a path that reaches a generated definition for a key that failed to
resolve.

> **AMENDED IN PLACE 2026-08-11 by the Steward**, on Architect ruling
> `evt_1jmng1vr3dw3k`, which confirms this AC inherits the landed `D2j`
> disposition `evt_cnwn4y5xykg1`.
>
> **STRUCK — *"`D2e`'s transplant controls — call identity and segment owner,
> independently — still reject."*** **Segment owner must NOT reject.** A
> coherently re-homed source determines a *different complete key* and is not
> refused for it; building that rejection would add a topology restriction the
> production key model neither has nor needs. **This frame predates the ruling
> that corrected it, and the criterion as written could not be discharged.**
> Found at grounding by the implementer, not at acceptance.
>
> An *incoherent* owner exists only as a mutated key or an inconsistent plan
> artifact, and the landed `D2h` interner/re-derivation and oriented-plan
> validators already own that refusal. **`D2f` must not add a second
> owner-rejection path.**

**AC-6a — the five causes still reach nothing, and they DO NOT all refuse at the
same phase.** Checked frame, selected slot, invocation template, exact suffix,
and **call identity**. Start each from the **same positive full-pipeline
baseline** that resolves one key/ID/descriptor and creates exactly one `Fusion`
definition, then apply one source-side cause independently, with the matching
independently authored oriented plan where applicable. Each negative must yield
**no key, no `Fusion` ID, no descriptor, no generated definition, and no
redirected edge.**

> **THE PHASE SPLIT IS MEASURED AND MUST BE PRESERVED.** Architect
> `evt_6907h4rv5kq1a`, correcting wording the Steward introduced earlier the
> same day.
>
> | cause | where it stops |
> |---|---|
> | checked frame, selected slot, invocation template | **refuse BEFORE builder arrival** |
> | exact suffix, call identity | **reach the builder and resolve ZERO** |
>
> **STRUCK — *"the five refusals"* as a single class.** Calling all five
> "validator refusals" asserts a phase that two of them do not have, so a
> control written to that description would look for a refusal where the
> measured behaviour is arrival-then-empty — and would either fail on correct
> code or be weakened until it passed.
>
> **All five still owe the same end state** — no key, ID, descriptor,
> definition, or redirection. **What differs is where each stops**, and the
> frame must not flatten that.
>
> **Each row asserts its OWN phase evidence, beside a non-empty positive**
> (Architect `evt_4trsqtkxtghjx`), so that none of the zeros is vacuous:
>
> | population | required phase evidence |
> |---|---|
> | frame, selected slot, invocation | **zero arrivals** |
> | exact suffix, call identity | **one arrival, zero keys / descriptors** |
>
> **The applied `Exact` positive must be present and non-empty in the same
> control.** A row asserting "zero arrivals" proves nothing on its own — it
> holds identically when the mechanism never ran, which is the failure this
> node has now filed against itself repeatedly.

- **The definition/edge population must be an operand of the assertion**, with
  the positive baseline present. **A refusal control asserted against a resting
  zero proves nothing** — it holds whether or not the mechanism ran.
- **Enter through the ordinary planner-to-emission path.** Do not mutate a key
  struct and do not call the emitter directly. The control's job is to prove
  **emission has no fallback around a failed resolution**, and a control that
  bypasses the planner cannot see that path.

**AC-6b — segment owner is a POSITIVE comparator, not a refusal.** Build the
exact and coherently re-homed sources against their own plans **and their own
cause-selected roots** — applied for `Exact`, **bare for `ReHomed`**; **each must
still resolve one complete key and emit exactly one generated definition.**

> **THE RE-HOMED DISPOSITION. Architect `evt_4trsqtkxtghjx`.**
>
> **The lawful bare re-homed root must remain POSITIVE.** It reaches production
> builder **plane `1`** and independently agrees with its own planner key and
> descriptor. **Before** fusion emission it proceeds to the ordinary
> `ComputationalMatch` lowering refusal with **definition count `0`**; **after**
> emission it mints **exactly its own one definition and redirection edge**
> before reaching that same ordinary refusal.
>
> **It retains the already-established ZERO-input ABI projection**, while applied
> `Exact` retains **two ordered inputs**. Their complete keys and owner pairs
> stay distinct, and **raw IDs from their separately numbered planes are never
> compared** (see `AC-6c`).
>
> **If the bare re-homed root fails to reach that ordinary seat: STOP and
> measure the new boundary. Do NOT repair the comparator by wrapping or applying
> it.** The applied `Unsupported(Call)` outcome is **a required negative sanity
> check at most — never an emission witness and never AC credit.**

Then:

- re-assert each key's producer and consumer owners against **its own plan's**
  `occurrence_authority`, preserving `producer_owner != consumer_owner`;
- require the **owner pairs and the complete keys to differ**, and preserve the
  constant checked-transport relation;
- bind each emitted definition to **its own** locally resolved
  descriptor/key — owner `ContinuationEmissionOwner::Fusion(local_id)`, its own
  validated source-body authorities, and exactly its own redirected producer
  edge;
- require the emitted ordinary-ABI projection to reflect the discriminating
  source facts `D2j` already established — **two continuation inputs on the
  exact side, zero on the re-homed side** — so that reusing one side's
  descriptor or definition for the other **reds**.

**AC-6c — do NOT compare bare numeric `Fusion` IDs across the two planes.** They
are **independent interners and both may lawfully issue local ID `0`.** An
assertion that two such IDs differ is `0 != 0` — it fails on correct code, and
manufacturing a shared namespace to make it pass would invent a cross-plane
relation production does not have.

⇒ **Non-aliasing is established by the distinct complete keys, by each
compilation-local key ↔ ID ↔ descriptor ↔ definition binding, and by the
different emitted ABI projection.** Never by an ID inequality.

## Excluded scope

- **No re-opening of the identity plane.** The key, the slot-role derivation,
  interning, and the re-derivation validator are `D2e`'s and land before this.
- **No closure representation, at any depth.** No carrier tag or class, no ABI
  slot, no `Closure`/`DeclarationClosure` representation, no
  `call_declared_unit_target` escape.
- **No widening of the existing continuation-specialization class** — no
  `Option` on `worker`, no synthesized provenance, no `ContinuationSpecialization`
  `Id` reuse.
- **Row 5 after-hole stays reported-only.** `R4` is
  [[RT-LEXICAL-ROW2-MISSING-MINT]]. `#6d` keeps its status; `D3` and the
  retirement are untouched.

### Explicitly forbidden routes to a positive. Architect `evt_6vf66hmwv52y6`.

These are named because each is a plausible way to make the old witness look
like a candidate, and every one of them is a way of **fabricating the input the
key re-derives against**:

- **Do not pass `Some(plan)` to `px8j_capture_source_trace.`**
- **Do not synthesize a default plan**, and do not infer markers from the
  Runtime shape.
- **Do not weaken the required checked-transport key member.**
- **Route (2) — making fusion independent of `oriented` — is REJECTED
  outright.** It would reopen `D2h`'s soundness-bearing identity and contradict
  the required-member ruling. It is not a fallback if the checked route proves
  awkward; it is closed.

### The authority split, so the fixture plan is not mistaken for a production one

1. **In real package-backed production the only authority remains canonical
   checked-package metadata** decoded by `oriented_subcontinuation_plan_for_`
   `program`. **Runtime synthesizes nothing.**
2. **In the `#[cfg(test)]` control**, the already-authorized `D2g`/`D2j` plan is
   lawful **fixture** authority — and only because it is complete,
   independently authored from the marker collector, and passes exact validation
   against all three marker populations and structural locations. **Passing it
   explicitly in that test does not create a production fallback.**
3. **The old `px8j` origins, owners, and edge are never copied into the checked
   twin.** The twin's fresh planner coordinates and complete seven-fact key
   remain authoritative.

## Stop conditions — return to the Architect, do not decide

Any need for a runtime continuation or callback; any eager recursor invocation;
any weakening of the whole-graph admissibility walk; **any case where activation
state would cross the ABI or descriptor boundary.** That last one is the ruling's
own stop condition and it is the one most likely to fire here.

## Contention

Paths are `crates/ken-runtime/src/cranelift_backend/**` — the emission side —
and its test targets. Language owns `crates/ken-elaborator`. No `spec/` or
`conformance/` path, so no Spec vote on the merge Decision.

**The sequencing constraint, and its gate is CORRECTED.** This deliverable
consumes `D2e`'s id, key, and validator as fixed inputs. Starting it first would
mean building an emitter against an identity that does not exist yet, and the
identity is the soundness-bearing half.

**The gate is the identity plane landing, NOT `D2e` merging.** This frame
originally read *"do not start until `D2e` has merged."* **That wording fires
early and did.** `D2e`'s first candidate merged as a bounded partial at `main`
`f37ecd13` carrying only the checked binder layout and `AC-7`, while
`StaticContinuationFusionId`, the key, interning, the occurrence-walk threading
and the exact re-derivation validator were all still unbuilt. Under the operator's
accepted-partial policy a WP branch merges **repeatedly** by construction, so a
release condition keyed on a merge *event* is early by default, not by accident.

⇒ **Do not start until the identity plane is landed.** I will release this
then; if you receive it earlier the kickoff is my error.

**RE-CUT 2026-08-10 — the gate now names `D2h`, not `D2e`.** `D2e` stopped
short three times without starting key work, so its remaining scope was re-cut
into [[RT-LEXICAL-RECURSOR-CONSUMERS-D2g]] (the checked transport twin) and
[[RT-LEXICAL-RECURSOR-CONSUMERS-D2h]] (the key plane). **The identity plane is
`D2h`'s `AC-1`..`AC-5` discharged and landed**, on the twin `D2g` supplies.
`D2e` is closed and retained as the record of what landed.

The substance of the gate is unchanged: **the identity must exist before an
emitter is built against it**, and the identity is the soundness-bearing half.

## Sizing and validation

One turn to a releasable increment or a genuine hard stop. **`D2d` ended two
turns short and produced this cut; if this one also ends short, hand back the
seam rather than leaving a half-built ABI class.**

Targeted only — `-p ken-runtime`, or `--test <name>` for one suite, **never
`--workspace`**. A new `AbiUnitDefinition` arm changes an enum, so the floor is a
full `-p ken-runtime` test build: a suite-scoped run cannot observe an exhaustive
`match` in a sibling target. "No regression" means green in CI.
