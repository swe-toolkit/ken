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
invocation — the single `StaticBody` edge `0 -> 2` on the `R3` witness. Not
every edge to that callee, and not a search for a plausible one.

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

**AC-1.** On the exact `R3` before-hole compile under B-only exclusion
(`RecursiveDescentResidual::LexicalCallArgumentRecursor`, fixture
`px8j_equal_payload_hole_placement(BeforeReturnHole)`), the inner `Node` recursor
closure and its exact activation and cursor are produced and consumed by the
downstream call **in the same specialized unit**, with **zero**
`transfer_into_carrier` attempts for that intermediate
`Node[ComputationalRecursorClosure]`.

**AC-2 — the A/B witness, and its suppressor is load-bearing by construction.**
Suppressing **only** the continuation fusion restores the measured origin-23
refusal. Suppressing anything else does not count.

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

**AC-6 — the fail-closed boundary survives emission.** `D2e`'s transplant
controls — call identity and segment owner, independently — still reject, and
still reject **before** any definition is created. Emission must not introduce a
path that reaches a generated definition for a key that failed to resolve.

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
