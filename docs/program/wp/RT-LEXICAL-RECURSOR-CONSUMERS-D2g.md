# RT-LEXICAL-RECURSOR-CONSUMERS D2g — the checked transport twin

Owner: runtime. Size: M. Node: [[RT-LEXICAL-RECURSOR-CONSUMERS]] (`#6d`).
Architect rulings `evt_2wwh9yamyhs7p` (the mechanism), `evt_6sk3czsbcr85r` (the
`StaticContinuationFusion` class), and **`evt_2t67rtf6kaw5e` (the checked
transport member is required)**.

**Seat tier: T1.** The `#8` suspension does not reach `#6d`.

**Released 2026-08-10.** Fixed inputs measured at `main` `215bd156`;
**re-derive your merge-base and do not reuse that SHA.**

## Why this is its own deliverable

`D2e` landed three partials and then stopped short three times without ever
starting key work. **Three budget-bound stops on one deliverable means the
compound scope was the defect**, so this is the re-cut: the fixture and plan
here, the key in [[RT-LEXICAL-RECURSOR-CONSUMERS-D2h]], emission in
[[RT-LEXICAL-RECURSOR-CONSUMERS-D2f]].

**Read `D2e` first.** It is closed but retained, and it carries the
fixed-input reconciliation, the landed binder-layout and IH-threading
authorities, and the Architect's required-transport ruling in full. This frame
does not restate them.

## The problem this deliverable exists to remove

The `R3` before-hole witness — the fixture the entire campaign was measured
against — **carries zero checked transport markers**, as does the two-sibling
`AC-9` fixture, while 22 other fixtures in the same file construct
`CheckedSubcontinuationFrame`. These are hand-built IR fixtures and the markers
are elaborator-emitted, so **the absence is a property of the fixture, not of
the mechanism.**

The Architect ruled the checked coordinate **required and never optional**.
⇒ **There is currently no fixture on which a positive fusion identity can be
built at all.** That is what this deliverable supplies, and it is why the key
cannot be written first.

## Deliverables

**1. The plan-threading capture helper.** Thread the checked wrapper
authorities the planner's walk already descends through —
`CheckedSubcontinuationFrame`, `CheckedComputationalIHSlots`,
`CheckedComputationalIHInvocation` — by the **same threading pattern the landed
IH binding already uses**. The implementer established that this derivation is
available at planning time; this deliverable builds it, and nothing here infers
a coordinate from the Runtime shape.

**2. A complete oriented plan for the twin.** `OrientedSubcontinuationPlanV1`
covering all three marker populations, matching the fixture's exact structural
marker locations.

**3. The checked `R3`-shaped twin.** A parallel fixture of the same `R3` shape
that **carries checked transport**, produced by the checked erasure/wrapper
path **or** carrying a complete matching oriented plan and passing the existing
exact transport validator.

**Hand-wrapping the old `RuntimeExpr` with chosen ids is forbidden** — that is
the manufactured-evidence shape, and it would make every downstream control
vacuous.

**Wrappers change the semantic occurrence tree, so derive and report the twin's
coordinates fresh.** Do not reuse `StaticOriginId(5/12/18/23)` or the old owner
and edge numbers by assertion. **Those are coordinates of the unmarked witness
and are not the coordinates of any fusion.**

**4. The unmarked witness is preserved, not rewritten.** The existing `R3`
before-hole witness keeps its `D2d` coordinates and outcome and becomes the
**absence control**. The two-sibling `AC-9` fixture stays an order
discriminator and is not promoted into a fusion-key witness.

## Acceptance criteria

**AC-1 — the twin reaches the same relation, and this is the whole point.** The
twin exhibits **the same producer to IH-consumer relation** as the unmarked
`R3` witness. Establish it through the **landed** `CheckedIhBinding` authority
— the consumer's selected case body is the exact suffix iff its callee `Var`
resolves to `CheckedIhBinding { frame_origin: continuation_origin,
recursive_position }`. **A twin of the same shape that does not carry that
relation is not a twin**, and building the key on it would be worse than having
no twin at all.

**AC-2 — the transport validates exactly.** The oriented plan validates against
all three Runtime marker populations **and their exact structural locations**.
Report the marker census per fixture, as a table, not as a general property.

**AC-3 — the coordinates are fresh and reported as such.** State the twin's
derived coordinates and state that they are the twin's. **Any coordinate
carried over from the unmarked witness by assertion is a defect**, including in
prose.

**AC-4 — the absence control is exercised in the direction that can fail.** The
unmarked `R3` witness carries no transport and the validator says so. **A check
that passes because nothing reached it is the failure mode here**, so state
what would have caught the negative case.

**AC-5 — no key, no id, no descriptor, no interning.** Those are `D2h`'s. A
candidate that begins them has taken the compound scope back.

**AC-6 — `D2b`'s controls are retained** and still prove `Closure` and
`DeclarationClosure` unconditionally non-transferable at every depth, and
`call_declared_unit_target` free of any closure lane.

**AC-7 — measurements carry their population in the claim**, not four
paragraphs below it. Carried from `D2e`'s `AC-8`, which was satisfied and stays
satisfied.

## Excluded scope

- **No key plane.** No `StaticContinuationFusionId`, no key, no interning, no
  re-derivation validator, and none of `D2h`'s `AC-4`/`AC-5`.
- **No emission.** No `AbiUnitDefinition` arm, no
  `ContinuationEmissionOwner::Fusion`, no emitter, no scoped source-body
  authorities, **no redirection of the producer invocation edge**. Those are
  `D2f`.
- **No rewrite of the unmarked `R3` witness**, and no promotion of the
  two-sibling `AC-9` fixture.
- **No widening of the existing continuation-specialization class** — no
  `Option` on `worker`, no synthesized provenance, no
  `ContinuationSpecializationId` reuse.
- **No `R3` row claimed green.** Row 5 after-hole stays reported-only; `R4` is
  [[RT-LEXICAL-ROW2-MISSING-MINT]].

## Stop conditions — return to the Architect, do not decide

**The live one:** if **no plan-backed checked twin can reach the same producer
to IH-consumer relation**, stop and hand back the seam. **Do not manufacture
markers and do not make the member optional.** The stop that produced this
frame was called correctly and a second one here would be too.

Also: any need for a runtime continuation or callback; any **inference** of
suffix identity; any eager recursor invocation; any closure carrier or ABI
representation; any weakening of the whole-graph admissibility walk.

**Not a stop condition:** the checked member's required-versus-optional status.
That is ruled — **required** — and re-escalating it is forbidden.

## Contention

Paths are `crates/ken-runtime/src/cranelift_backend/planning/**` and its test
targets. Language owns `crates/ken-elaborator`; Kernel is on `crates/ken-interp`
and `crates/ken-kernel` under [[KERNEL-NESTED-IND-D10]]. No `spec/` or
`conformance/` path, so **no Spec vote on the merge Decision.**

## Sizing and validation

**One turn, and the scope is deliberately one thing.** `D2e` proved that the
fixture and the key together do not fit; this is the half that has to exist
first. A releasable increment or a genuine hard stop are both good outcomes.

Targeted only — `-p ken-runtime`, or `--test <name>` for one suite, **never
`--workspace`**. If any enum variant is added or changed, the floor is a full
`-p ken-runtime` test build: a suite-scoped run cannot observe an exhaustive
`match` in a sibling target. "No regression" means green in CI.
