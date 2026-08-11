# RT-LEXICAL-RECURSOR-CONSUMERS D2h — the key plane

Owner: runtime. Size: M. Node: [[RT-LEXICAL-RECURSOR-CONSUMERS]] (`#6d`).
Architect rulings `evt_2wwh9yamyhs7p`, `evt_6sk3czsbcr85r`, and
**`evt_2t67rtf6kaw5e` (the checked transport member is required)**.

**Seat tier: T1.** The `#8` suspension does not reach `#6d`.

> ## SCOPE RULING — Steward, 2026-08-11. THIS FRAME IS NOW SPLIT.
>
> Steward ruling `evt_2vfgg71s847ns`; Architect correction `evt_4psbpktt6tv75`.
>
> **Everything below this block that says "scope is unchanged" is superseded by
> this block.** Those sentences were true when written, and both are corrected
> in place where they appear.
>
> Architect block `evt_1ef33yr2xxdad` rejected candidate `a77ba94a` on four
> grounds. Three are bounded repairs. The fourth — `AC-1`'s demand for two
> complete **planner-valid** keys per distinguishable identity class, including
> a non-empty ordered input projection — is fixture construction, not a control
> edit, and it resizes the node rather than the candidate.
>
> ### What `D2h` now is
>
> The **production key plane**, and only that:
>
> 1. `StaticContinuationFusionId`, the complete immutable seven-fact key, the
>    descriptor, the interner, and `build_static_continuation_fusion_plan` as
>    **production planner state**. Not `#[cfg(test)]`. This is the half that
>    makes it `D2f`'s fixed input, and a plane erased from a non-test build is
>    not that input.
> 2. An **independent exact re-derivation** of the whole key from planner facts
>    before interning. **`a77ba94a`'s defect was not a weak check — it was a
>    check that measured nothing**: two identical runs of one enumerator agree
>    by construction, so the comparison could only detect nondeterminism. The
>    control that earns the word *independent* is a mutation of the primary
>    derivation that the validator catches, not a second call to it.
> 3. The key↔ID↔descriptor bijection round-trip.
> 4. The `AC-2` refusals the landed witness already expresses: stripped
>    transport, suppressed descent, duplicated actual `StaticBody` edge.
>
> #### ARCHITECT RULING `evt_4psbpktt6tv75` — `AC-1` carried TWO obligations
>
> The Architect answered the proportionality question and **corrected the
> proportional part of the block.** The blanket *"two planner-valid keys per
> member"* conflated an interner-algebra property with a derivation property,
> and the two take different instruments. An earlier draft of this ruling
> relocated the whole per-class matrix; that was superseded before this frame
> was published, and the split below is the ruling as issued.
>
> **Collision is interner algebra, and a synthetic mutation IS a valid net for
> it** — provided the mutated key is **actually submitted to the interner and
> receives a distinct ID.** `a77ba94a`'s defect was asking a one-element lookup
> and getting `None`, which is not an interning test at all. **Planner validity
> is an upstream precondition here**, so demanding a real program per field is
> disproportionate to the map property.
>
> **A production requirement follows, and it is structural, not conventional:**
> the map is keyed by the **complete key type under its full derived
> equality/order** — no hand-written projection, no subset comparator. Then
> same-key reuse, unequal-key distinct mint, and both round-trips are the whole
> collision net.
>
> **Label it for what it is.** That per-member matrix is an **interner-unit**
> matrix. It must not be reported as planner-derivation evidence. Conflating
> the two is what produced the block.
>
> ### What stays here after the correction
>
> `AC-1`'s per-member mutation matrix stays in `D2h` as an interner-unit
> matrix, labelled as such, with every mutation **submitted to the interner**
> rather than looked up.
>
> ### What relocates to a successor
>
> **Per-member derivation correctness** — a provenance matrix, **not** twenty
> pairwise planner-valid programs. For every key member the successor states
> the exact authoritative planner fact it comes from; a reaching positive
> witness on which that fact is **non-degenerate**; an independent source-side
> mutation or transplant that either changes the re-derived member or refuses
> before interning; and agreement between the primary derivation and the
> independently authored re-derivation.
>
> **One real witness may discharge many rows.** Additional `d2g_declaration`
> knob variants are owed only where the existing witness plus the
> production-mutation harness cannot make a member's source causal. A **pair**
> of planner-valid programs is owed only for a member whose derivation could
> otherwise alias or normalize two genuinely distinct planner facts — never as
> a blanket condition on every field.
>
> **The non-empty ordered-input witness is mandatory**, not optional scope: the
> current witness cannot discharge ordered-input derivation at all, because its
> vector is empty.
>
> **RELOCATION IS NOT RETIREMENT.** The `#6d` node does not close on this
> frame's merge. A reader who takes the narrowed merge as the whole obligation
> discharged has read it wrong, and this sentence exists so that reading is not
> available.
>
> ### The six `AC-2` refusals — measure, do not assume
>
> The frame, selected slot, invocation, exact suffix, call identity, and
> segment-owner refusals relocate **only where the landed
> `CONTINUATION_PRODUCTION_MUTATION` harness cannot express them on the current
> witness.** That harness exists and already carries transplant-shaped variants.
> **Report a verdict per cause.** Whatever the harness reaches stays here. Six
> refusals must not leave this node silently because one of them is expensive.
>
> ### Two sizing facts, because the estimate that produced the stop conflates
> ### two different jobs
>
> **The empty projection has a locatable cause, and it is the program, not the
> mechanism.** `intrinsic_environment_floor` is `entry_sources.len()`
> (`crates/ken-runtime/src/cranelift_backend/planning/static_transition.rs:6947`),
> and `required_input_count` rises above it only when a case body needs a longer
> surrounding prefix (`:6948` onward). This consumer has neither. So the
> non-empty-inputs witness is a consumer function that takes entry values —
> **one structurally different program**, not one of twenty.
>
> **The fixture builder is already parameterized** — `d2g_declaration(true)` at
> `:14874`, fed through a real `plan_static_transition_graph` run. Nineteen knob
> variants on an existing builder plus one new program shape is a different
> estimate from *"roughly twenty fixtures"*, and it may change how the successor
> is cut.
>
> ### Open with the Architect; the plane does not wait on it
>
> Whether `AC-1` guards **collision** or **the correct derivation of each key
> member from planner facts**. A clone mutation proves the map is keyed on a
> field; it cannot prove the planner can *produce* two fusions differing only
> there — but if the planner cannot, including that member is harmless
> over-specification rather than a collision hazard. The second reading is the
> one this plane visibly lacks: **the ordered-inputs member's derivation has
> never run non-trivially on any witness**, so nothing has checked it is derived
> correctly, independent of whether two keys can differ in it. If the answer is
> derivation correctness, the successor is framed that way rather than as twenty
> discrimination pairs.
>
> The closed seven facts are unchanged, `continuation_result_origins` is
> unchanged, and no eighth fact is licensed. If a gate needs one, stop.

> ## RELEASED 2026-08-11 — the gate below is discharged. `D2i` is on `main`.
>
> **`D2i` merged at `f01e63a1`** (`c1725317`, the live enumerator reading the
> admitted ledger; the ledger half landed earlier at `b7142fe5`). The gate at
> the foot of this block said *"resume unchanged once `D2i` is on `main`"*, and
> it is. ~~Scope is unchanged. Resume.~~ **Scope WAS unchanged when this block
> was written and is no longer** — the resumed turn produced `a77ba94a`, which
> the Architect blocked, and the SCOPE RULING above splits the frame. Read that
> block for the current scope; this one is retained for the `D2i` gate history.
>
> **But read how the gate was satisfied, because it is not how it was written.**
> The gate expected a *new* productive checked twin. The `D2i` scope ruling
> retired that artifact: making the enumerator live measured the **landed `D2g`
> twin itself** as productive on a production-issued ledger root, so no
> replacement fixture was built. The twin did not change — **the instrument
> did.** Zero result-flow pair under seed enumeration, one under ledger
> enumeration.
>
> ### YOUR FIRST STEP IS A MEASUREMENT, NOT THE PLANE
>
> **Before building anything, confirm the enumerator presents exactly one
> pre-interning candidate on your fixture.** State the count.
>
> - **One candidate ⇒ proceed**, and the zero-fusion condition that held this
>   frame twice is gone.
> - **Zero ⇒ STOP AND COME BACK TO ME IMMEDIATELY.** That is the third hold on
>   this frame, and a third would mean the gate itself is mis-specified rather
>   than unmet. **Do not build the plane to find out**, and do not manufacture a
>   fixture to make the count non-zero — that is the green-by-construction shape
>   Runtime correctly refused twice.
>
> `continuation_result_origins` **must not be widened** (Architect
> `evt_1dgwdvxhnabg4`). If presenting the pair appears to require widening it,
> that is the stop above, not a licence.
>
> ~~**This frame's scope is unchanged and still correct. It was blocked, not
> mis-scoped**, so it was respun rather than split~~ — **that held for the two
> earlier holds and no longer holds.** The 2026-08-11 block found the frame
> genuinely mis-sized on `AC-1`, and the SCOPE RULING above splits it. **The
> existing thread `thr_2htr4r28a64c1` still stays its spine** — `D2h` narrows
> rather than dissolving, so there is still no fresh `D2h` kick; the relocated
> successor gets its own kick and its own thread when I release it.
>
> **The key plane interns zero fusions on the landed `D2g` twin.** Production
> discovery walks `continuation_result_origins` by result positions and treats
> `Construct` and `LexicalClosure` as terminal; the twin's shape is
> `Construct[LexicalClosure[inner match]]`, so no producer/consumer pair is ever
> presented. Runtime built the whole plane, measured zero, and **discarded it
> rather than land a mechanism green by construction** — that was correct, and
> a plane with zero positive candidates must not land.
>
> **`D2g`'s `AC-1` is true and was the wrong criterion.** It pinned binder
> resolution through `CheckedIhBinding`, which still passes on the twin.
> Result-flow membership is a different relation, and nothing in `D2g` consumed
> the pair, so nothing in `D2g` could have caught it. **Steward framing defect**,
> not an implementation or review miss.
>
> **Gate: [[RT-LEXICAL-RECURSOR-CONSUMERS-D2i]] lands the productive checked
> twin.** Architect ruling `evt_1dgwdvxhnabg4` settled the mechanism —
> `continuation_result_origins` is semantically right and **must not be
> widened**. Resume unchanged once `D2i` is on `main`; **do not manufacture the
> replacement fixture under this frame.**

**Superseded release note, retained because its `D2g` half still holds.**
[[RT-LEXICAL-RECURSOR-CONSUMERS-D2g]] merged as `main` `ae9db606` (candidate
`027a0674`, declared merge-base `72fe6714`, PR #1844, three commits, one path,
`+984/-0`, blob-verified). Re-derive your merge-base from `origin/main`; **do
not take a SHA from this frame.**

**The gate was stated as a property, and I checked the property rather than the
merge.** The condition was `D2g`'s `AC-1`..`AC-4` discharged and the checked
twin on `main`, not "D2g merged". Both reviewers verified those ACs on exact
`027a0674`: `AC-1`'s same-builder binding to one consumer-frame/position-0
`CheckedIhBinding`; `AC-2`'s causal, runtime-only discriminator, where the plan
is held byte-fixed and equality-guarded while only the Runtime declaration moves
the outer slot wrapper to a real sibling case; the per-slot constructor pins on
outer `D2gOut::Node` and inner `D2gIn::Node` with their inequality asserted; and
the plan-backed triple resolution with relation-severance yielding no coordinate.

**Why that distinction was worth keeping.** Under the accepted-partial policy a
node merges repeatedly by construction — `RT-LEXICAL-RECURSOR-CONSUMERS` has
landed eight times (`D2b`, `D2c`, `D2d` twice, `D2e` three times, and now `D2g`)
— so a release condition keyed on "the WP merged" names an event that recurs and
fires early by default. That already happened once on this node, which is why
this gate reads as a property instead.

`D2g` itself merged once, at `ae9db606`. Its two earlier candidates were
**rejected in review, never merged** — the first for a non-causal `AC-2`
discriminator, the second for a single constructor answering for both slot
frames. Both defects are closed in what landed.

## What this deliverable is

`D2g` supplies the checked `R3`-shaped twin. **This deliverable builds the
identity on it.** The twin is a fixed input here: if you find yourself
re-opening the plan threading, the twin's construction, or its coordinates,
that is `D2g` work and it comes back to me rather than being redone under this
frame.

**Read `D2e` first** — closed but retained, it carries the fixed-input
reconciliation, the landed binder-layout and IH-threading authorities, and the
required-transport ruling in full.

## Deliverables

**1. `StaticContinuationFusionId`, its key, and interning.** A new opaque id
with a bijection key↔ID↔descriptor. The whole key is the Architect's,
unchanged: domain-tagged original producer-invocation emission owner and exact
edge; producer owner, result root, construct origin, selected alternative,
recursive position; consumer owner, continuation frame, selected body, and the
exact IH-consuming `Call`; **the checked transport coordinate**; and the
complete ordered ABI input projection. **Distinct in any member means a
distinct fusion.**

**The checked transport member is REQUIRED and is never an `Option`.** Absence
does not denote a smaller-but-valid identity. It is one coordinate resolved
from three wrapper authorities:

| wrapper authority | resolved against |
|---|---|
| `CheckedSubcontinuationFrame.frame_id` | its exact `OrientedSubcontinuationFramePlanV1` |
| the selected `CheckedComputationalIHSlots` entry, with slot-template id and checked occurrence path | its exact `CheckedComputationalIHSlotTemplateV1` |
| `CheckedComputationalIHInvocation`, with call-template id and checked occurrence path | its exact `CheckedComputationalIHCallTemplateV1` |

**Record the exact resolved identity. Do not infer it from the Runtime shape,
do not select "the only" marker, and do not accept raw wrapper numbers without
the matching oriented plan and exact marker-location validation.** "The only
marker" is the same existential shape as "the only continuation", and it is
rejected for the same reason.

**2. The construction order, and it is fail-closed by sequence.**

1. Validate the oriented plan against all three Runtime marker populations and
   their exact structural locations.
2. Resolve the checked frame, the selected IH slot, and the exact IH
   invocation template.
3. Derive every other whole-key member, including the landed exact consuming
   `Call`.
4. Exact-re-derive and compare the complete key.
5. **Only then** intern and mint key↔ID↔descriptor.

**Any absence, multiplicity, marker/plan disagreement, wrong selected slot, or
transplanted frame/slot/call template yields no candidate before interning.
There is no optional-key equivalence class and no fallback identity.**

**Do not compress this order to fit a turn. The order is the soundness
property**, not an implementation convenience.

**3. The exact re-derivation validator.** Re-deriving the key from planner
facts yields the same members. This is what converts a grounding tuple into a
mechanism: each member comes from a planner-issued identity, never from a
spelling, a type, a row number, a runtime tag, "the only continuation", or
"the only marker".

## Acceptance criteria

**AC-1 — SPLIT by the 2026-08-11 scope ruling. Read this before executing it.**

**The half that stays here** is the bijection on the production plane: the same
complete valid key interns to the same id, `key→ID→key` and `ID→descriptor`
round-trip, and exactly one identity is minted from production planner facts on
the landed witness.

**AC-1a — the independent re-derivation, and this is now the load-bearing
control of this node.** The whole key is re-derived from planner facts by a
second route and compared before interning. **A second call to the same
enumerator does not satisfy this** — that is what `a77ba94a` shipped, and two
identical runs of one function agree by construction, so it detects only
nondeterminism. The evidence is a **mutation of the primary derivation that the
validator catches**; a passing validator with no such mutation is assertionless.

**The half that RELOCATED to the successor** — do not attempt it here, and do
not report the node complete on the strength of the half above:

- two complete valid keys differing in the **checked frame** member interning
  distinctly, and likewise per remaining identity class;
- the **non-empty ordered input projection** witness. The landed witness's
  projection is empty for a program reason, not a mechanism reason
  (`static_transition.rs:6947` — the floor is `entry_sources.len()`), so **no
  test-side work produces it.**

**Both halves keep this rule:** malformed transplants are validator refusals,
not "distinct valid key" evidence. Counting a refusal as a distinguished key
would make the bijection look total while proving nothing.

**And the failure that made the split necessary is worth carrying forward:**
`a77ba94a`'s `AC-1` mutated **clones** of the interned key and called `id_for`,
which is a lookup in a one-element vector. A miss proves the map is keyed on
that field. It does not prove the planner can issue two valid keys that differ
there — which is the claim the criterion was written to establish.

**AC-2 — fail-closed, and this is the soundness-bearing control.** A producer
**lacking** the exact consuming suffix yields **no fusion and the ordinary
existing refusal** — never a fallback to the unspecialized result-returning
unit.

**Every one of these fires BEFORE any id, descriptor or definition is
created.** The split below is settled by measurement, not estimate — Runtime
enumerated `ContinuationProductionMutation` on exact `1139e0be` and found its
complete variant set to be `Exact`, `ResultLifetimeProxy`,
`ConstructorFieldCountPrefix`, `DescriptorOrdinalSources`, and
`DescriptorInputCountTruncation`. **It expresses none of the six member
transplants**, so the per-cause verdict is unanimous rather than mixed:

**Retained in `D2h`** — the landed witness expresses all three:

- **strip the checked transport from the twin** — no fusion, ordinary refusal;
- **suppressed descent** — no pair presented, nothing minted;
- **a second matching `StaticBody` edge** — refuses rather than selecting.

**RELOCATED to the successor**, all six, because no landed harness reaches
them and each needs a planner-valid transplant fixture:

- the **frame**, the **selected slot**, and the **invocation** marker/plan
  relation, each rejecting on its own;
- the **missing exact consuming suffix**, **call-identity transplant**, and
  **segment-owner transplant** controls.

**These six are the `D2b`/`D2d` inheritance and they are the reason the
successor is not optional.** They are what separates an identity from a value
that happens to be unique in the measured population.

**Marker absence must not substitute for those soundness controls.** The
unmarked `R3` witness now refuses for a **transport** reason, which would mask
a missing suffix-identity control that never ran. **Passing for the wrong
reason is the failure mode here**, and it is exactly why `D2g` builds the twin:
the retained controls need a fixture that gets *past* the transport gate.

**This is `D2d`'s `AC-4` and it is why the ruling forbids "the only
continuation."** An identity that happens to be unique in the measured
population is not an identity — it is the existential shape that got
`d94ef37e` rejected on `D2b`. The transplant controls separate the two.

**AC-3 — no structural `Var` search.** The identity is derived from the
threaded environment and planner-issued identities. State where each member
comes from.

**AC-4 — `D2b`'s controls are retained** and still prove `Closure` and
`DeclarationClosure` unconditionally non-transferable at every depth, and
`call_declared_unit_target` free of any closure lane.

**AC-5 — measurements carry their population in the claim**, not four
paragraphs below it.

## Excluded scope

- **No emission, and this is the cut.** No `AbiUnitDefinition` arm, no
  descriptors populated for emission, no `ContinuationEmissionOwner::Fusion`,
  no scoped source-body authorities, no generated-definition emitter, and **no
  redirection of the producer invocation edge**. Those are `D2f`.
- **No re-opening of the twin, the plan threading, or the twin's coordinates.**
  Those are `D2g`'s and land before this.
- **No `R3` row claimed green** and `D2d`'s `AC-1`..`AC-3` are not discharged
  here — they need emission.
- **The existing continuation-specialization class is unchanged.** No `Option`
  on `worker`, no synthesized `ContinuationWorkerProvenance`, no reuse of
  `ContinuationSpecializationId`. The ruling introduced a separate class rather
  than relaxing this one, and relaxing it would silently widen its authority.

## Stop conditions — return to the Architect, do not decide

Any need for a runtime continuation or callback; any **inference** of suffix
identity; any eager recursor invocation; any closure carrier or ABI
representation; any weakening of the whole-graph admissibility walk; any case
where the checked binder layout cannot distinguish an IH slot from an ordinary
binder.

**Not a stop condition:** the emitter/ABI obligation being undischarged — it is
excluded here by construction and lands in `D2f`. Nor the checked member's
required-versus-optional status, which is ruled and closed.

## Contention

Paths are `crates/ken-runtime/src/cranelift_backend/planning/**` and its test
targets. Language owns `crates/ken-elaborator`; Kernel is on `crates/ken-interp`
and `crates/ken-kernel`. No `spec/` or `conformance/` path, so **no Spec vote on
the merge Decision.**

## Sizing and validation

One turn to a releasable increment or a genuine hard stop; both are good
outcomes. **`D2e` ended three turns short under a compound scope and produced
this cut — if this one also ends short, hand back the seam rather than leaving
a half-built key.**

Targeted only — `-p ken-runtime`, or `--test <name>` for one suite, **never
`--workspace`**. A new id and key add enum variants, so the floor is a full
`-p ken-runtime` test build: a suite-scoped run cannot observe an exhaustive
`match` in a sibling target. "No regression" means green in CI.
