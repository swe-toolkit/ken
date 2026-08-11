# RT-LEXICAL-RECURSOR-CONSUMERS D2e — the identity plane, planner-only

> ### SCOPE RE-CUT 2026-08-10. This frame is CLOSED; its remaining scope moved.
>
> **`D2e` landed three partials and then stopped short three times. Three
> budget-bound stops on one deliverable means the sizing was the defect, and
> the cut is mine rather than the ring's.** No key or ABI work was ever begun,
> so nothing is half-built.
>
> | landed | where |
> |---|---|
> | `AC-1` binder role, `AC-7` denominator operand | `main` `f37ecd13` |
> | `AC-2`/`AC-3` threading, `AC-9`/`AC-10` | `main` `f2959dfc` |
> | `AC-6` retained, `AC-8` satisfied | across both |
>
> **Remaining scope is re-cut into two deliverables**, on the implementer's own
> recommendation and the Architect's required-transport ruling:
>
> - **[[RT-LEXICAL-RECURSOR-CONSUMERS-D2g]] — the checked transport twin.** The
>   plan-threading capture helper, a complete oriented plan, and the checked
>   `R3`-shaped twin that reaches the same producer to IH-consumer relation.
> - **[[RT-LEXICAL-RECURSOR-CONSUMERS-D2h]] — the key plane.**
>   `StaticContinuationFusionId`, the whole key, interning, the exact
>   re-derivation validator, and `AC-4`/`AC-5`, built against the landed twin.
>
> **`AC-4` and `AC-5` are `D2h`'s and are not discharged here.** `D2f`, the
> emission plane, now gates on `D2h` rather than on this frame.
>
> **Everything below is retained as the durable record** of what landed and why,
> and as the source both successors inherit. The fixed-input reconciliation and
> the Architect's ruling below are binding on both.
>
> **One coordinate statement, sharper than my own wording and taken from the
> implementer's competing frame candidate `5760efcc`:** the unmarked `R3`
> witness's `StaticOriginId(5/12/18/23)`, its owners and its edge are
> coordinates **of that witness** and are **not the coordinates of any fusion**.
> That forecloses reuse everywhere, not only in the twin.

Owner: runtime. Size: M. Node: [[RT-LEXICAL-RECURSOR-CONSUMERS]] (`#6d`).
Architect rulings `evt_2wwh9yamyhs7p` (the mechanism) and `evt_6sk3czsbcr85r`
(the `StaticContinuationFusion` class). Fixed inputs measured at `main`
`3a15b201`. **Re-derive your merge-base — do not reuse that SHA.**

**Seat tier: T1.** The suspension of Runtime's T1 exception covers campaign node
`#8` only. `#6d` is a campaign node and stays T1; do not downgrade this seat.

## Why this is a separate deliverable

`D2d` reached its releasable partial — `AC-6` plus the grounding record, merged
as `main` `ca753171` — and then ended a second turn short with no code written,
at the seam I named. **That is the outcome I asked for.** Two short turns is
evidence the cut is right, and this is the cut: the identity plane here, the
emission plane in [[RT-LEXICAL-RECURSOR-CONSUMERS-D2f]].

**Take `D2d`'s grounding record as a fixed input** for the members it actually
measured — the unique `0 -> 2` producer edge, the two-entry ordinary
`ValueWord` projection, the consuming `Call` at `StaticOriginId(12)`. Read it
at `docs/program/wp/RT-LEXICAL-RECURSOR-CONSUMERS-D2d-GROUNDING.md`. **Do not
re-derive those coordinates; derive the mechanism that produces them.**

> ### FIXED-INPUT RECONCILIATION, 2026-08-10. The `R3` tuple cannot complete a key.
>
> **This section previously said every key member *except* the checked identity
> was already measured on the `R3` before-hole witness. That framing is
> withdrawn, and the correction is not cosmetic.** It invited the next turn to
> read the `R3` tuple as a source from which the full fusion key could be
> completed. It cannot be.
>
> Measured by the implementer at `215bd156` and ruled by the Architect at
> `evt_2t67rtf6kaw5e`: the `R3` before-hole witness carries **zero** checked
> transport markers, as does the two-sibling `AC-9` fixture, while 22 other
> fixtures in the same file construct `CheckedSubcontinuationFrame`. These are
> hand-built IR fixtures and the markers are elaborator-emitted, so the absence
> is a property of the fixture, not of the mechanism.
>
> ⇒ **The `R3` tuple is partial grounding plus an absence control. It is not a
> full-key positive and no positive identity control may be built on it.**
>
> **The implementer stopped before writing any key rather than picking among
> required / optional / re-author. That was the right call** — the member gates
> the key's *shape*, so discovering it afterwards would have meant rewriting the
> discovery, the interning and the validator around it.

## The defect this deliverable removes, and it is measured

`derive_case_producer_fact` threads an environment of `CaseProducerFact`. At the
`ComputationalMatch` arm it pushes `argument_binders + recursive_positions.len()`
entries, **every one of them `CaseProducerFact::open(origin)`**
(`crates/ken-runtime/src/cranelift_backend/planning/static_transition.rs:3414`
and following). `RuntimeExpr::Var(index)` then indexes that environment.

⇒ **The elements are uniform, so no slot carries a role.** An IH slot, an
ordinary constructor child, and a frame value are indistinguishable in the
derived fact. That is why `build_continuation_specialization_plan` falls back to
the syntactic predicate — it requires the argument at a recursive position to be
a `Closure` or `LexicalClosure`, and skips `Var(0)`, which on this witness *is*
the `ComputationalRecursorClosure`.

**The consequence that makes this the first deliverable:** the forbidden IH
identity is derivable without any structural `Var` search, from information the
planner already threads and currently discards.

## Deliverables

**1. The environment carries a slot role.** Replace the uniform `open` push with
a derivation that distinguishes an IH slot — carrying at least its frame origin
and its recursive position — from every other binder.

**Derive the role from the checked binder layout, not from a remembered index
order.** `argument_binders` and `recursive_positions` are two counts summed into
one push; which range occupies which de Bruijn prefix is a property of the
lowering, not of this frame. **Measure it and state the order you measured**, and
make the derivation read the layout rather than assume it. A frame that pins the
order would be pinning a fact I did not verify.

**2. `StaticContinuationFusionId`, its key, and interning.** A new opaque id with
a bijection key↔ID↔descriptor. The whole key is the Architect's, unchanged:
domain-tagged original producer-invocation emission owner and exact edge;
producer owner, result root, construct origin, selected alternative, recursive
position; consumer owner, continuation frame, selected body, and the exact
IH-consuming `Call`; the checked frame/invocation-template identity; and the
complete ordered ABI input projection. **Distinct in any member means a distinct
fusion.**

**The checked transport member is REQUIRED and is never an `Option`**
(`evt_2t67rtf6kaw5e`). Absence does not denote a smaller-but-valid identity.
It is one coordinate resolved from three wrapper authorities:

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

**3. The checked `R3`-shaped twin, and it is a deliverable rather than test
scaffolding.** The positive identity controls need a fixture that carries
checked transport. **Do not rewrite the existing unmarked `R3` witness in
place** — it keeps its `D2d` coordinates and becomes the absence control, with
an exact meaning: no checked transport means **not a fusion candidate**, so no
key, id or descriptor is created and the ordinary refusal stands. The
two-sibling `AC-9` fixture likewise stays an order discriminator and is not
promoted.

The twin must be produced by the checked erasure/wrapper path, **or** carry a
complete matching `OrientedSubcontinuationPlanV1` and pass the existing exact
transport validator. **Hand-wrapping the old `RuntimeExpr` with chosen ids is
forbidden.** Wrappers change the semantic occurrence tree, so **derive and
report the twin's coordinates fresh — do not reuse `StaticOriginId(5/12/18/23)`
or the old owner and edge numbers by assertion.**

**4. The construction order, and it is fail-closed by sequence.**

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

**5. The exact re-derivation validator.** Re-deriving the key from planner facts
yields the same members. This is what converts the grounding tuple into a
mechanism: each member comes from a planner-issued identity, never from a
spelling, a type, a row number, a runtime tag, or *"the only continuation."*

**6. The suppressed-leg denominator repair — a rider, two lines. LANDED** in the
first partial (`f37ecd13`); kept here for the record. Confirmed
Adversary finding `evt_4b1yq03sw9zr6`, measured on `ca753171`. `D2d`'s `AC-6`
made `established_s_arrivals` a `NonZeroUsize`, but on the **suppressed** leg the
value is read **only inside the format string** — the comparison is against the
literal `0`:

```rust
assert_eq!(s_forwards, 0, "…{s_forwards} of {established_s_arrivals} arrivals…");
```

Deleting the binding is `E0425` **in a format argument** today, which is what was
measured. But shortening the message to drop `of {established_s_arrivals}` — an
innocuous tidy of a verbose string — leaves the binding merely `unused`, and
there is no `deny` attribute in `crates/ken-runtime/src/lib.rs` and no
`-D warnings` in the workflows to make that bite. **Two steps instead of one, and
the first step is a message edit.**

**The property: the denominator must be consumed by the assertion, not by its
message.** The repaired leg already satisfies it —
`assert_eq!(forwards, established_arrivals.get(), …)` puts the value on the
right-hand side. Bring the suppressed leg into the same shape. **Choose the
route** — compare against the established count, or state the suppression as a
relation — the Adversary explicitly did not choose and neither do I.

## Acceptance criteria

**AC-1 — the role is derived, and the pair discriminates.** An IH slot
classifies as an IH; an ordinary child binder at the same depth does not. **Both
directions on a shared shape**, because a classifier that answers one way for
everything passes either case alone.

**AC-2 — indirection does not lose the role.** An IH reached through a `Let` or a
nested `Match` rebinding still classifies as an IH. This is the case a
depth-keyed or position-keyed classifier gets wrong, so it must exist by
construction rather than by intent.

**AC-3 — no structural `Var` search.** The identity is derived from the threaded
environment and planner-issued identities. State where it comes from; a
derivation that scans for a `Var` shape is the thing this deliverable replaces.

**AC-4 — the bijection holds.** key↔ID↔descriptor round-trips, and two keys
differing in exactly one member intern to two distinct fusions. Exercise one
member per identity class rather than one representative — a bijection test on a
single member says nothing about the others.

**The Architect specified the control set (`evt_2t67rtf6kaw5e`), and the
checked coordinate counts as three classes, not one:**

- the same complete valid key interns to the same id and round-trips;
- two complete valid keys differing in the **checked frame** member intern
  distinctly;
- likewise for the **selected slot-template** identity and path;
- likewise for the **invocation-template** identity and path;
- the ordinary one-member controls cover every other key identity class;
- **malformed transplants are validator refusals, not "distinct valid key"
  evidence.** Counting a refusal as a distinguished key would make the
  bijection look total while proving nothing.

**AC-5 — fail-closed, and this is the soundness-bearing control.** A producer
**lacking** the exact consuming suffix yields **no fusion and the ordinary
existing refusal** — never a fallback to the unspecialized result-returning unit.
Independently transplant the **call identity**; it **must reject, and reject
before any definition is created.**

> **STRUCK 2026-08-11 — *"(b) the segment owner; both must reject."*** Corrected
> by Architect ruling `evt_cnwn4y5xykg1`, reaffirmed at `evt_1jmng1vr3dw3k`.
> **A coherently re-homed source determines a different complete key and is NOT
> rejected for it**; demanding a refusal would force a second owner-rejection
> mechanism production does not have and should not have. The segment owner is a
> **positive provenance/non-aliasing comparator** — see `D2j`'s `AC-3a` and
> `D2f`'s `AC-6b`.
>
> **This sentence is where the error propagated**, and that is why it is struck
> here rather than only downstream. `D2f`'s `AC-6` inherited it *by citing this
> frame* — "`D2e`'s transplant controls, call identity and segment owner,
> independently, still reject" — and so demanded a control the Architect had
> already forbidden. An implementer caught it at grounding.
>
> ⇒ **A superseded frame is not inert.** Later slices cite it as the source of
> their own controls, so a voided requirement keeps being inherited until it is
> struck at the origin.

**This is `D2d`'s `AC-4` and it is why the ruling forbids "the only
continuation."** An identity that happens to be unique in the measured
population is not an identity — it is the same existential shape that got
`d94ef37e` rejected on `D2b`. The transplant controls are what separate the two.

**Added by the ruling, and every one of these fires BEFORE any id, descriptor
or definition is created:**

- **strip the checked transport from the positive twin** — no fusion, and the
  ordinary refusal;
- **independently** remove or transplant the **frame**, the **selected slot**,
  and the **invocation** marker/plan relation — each rejects on its own;
- the already-required **missing exact consuming suffix** and **call-identity
  transplant** controls are retained. **STRUCK — *"and segment-owner
  transplant"*,** per the strike on `AC-5` above: that one is a positive
  comparator, not a refusal.

**Marker absence does not substitute for those soundness controls.** The
unmarked `R3` witness now refuses for a *transport* reason, which would mask a
missing suffix-identity control that never ran — passing for the wrong reason
is the failure mode here, and it is why the retained controls need the twin.

**AC-6 — `D2b`'s controls are retained** and still prove `Closure` and
`DeclarationClosure` unconditionally non-transferable at every depth, and
`call_declared_unit_target` free of any closure lane.

**AC-7 — the rider is load-bearing.** Show by mutation that the suppressed leg's
denominator can no longer be removed by a message edit: shorten the message and
the leg must still red. Report the mutation and its result. A repair whose
removal is silent is the defect it replaces.

**AC-8 — report a measurement with its population.** Where a claim is measured
on one witness, say so **in the claim, not four paragraphs below it.** `D2d`'s
grounding record was careful in its operative text and slipped in its headings —
*"the exact producer invocation edge is projectable and unique"* reads as a
general property, and a scanner reads headings. One qualifier per claim closes
it.

**AC-9 and AC-10 — added 2026-08-10, after the first partial merged.** The
bounded partial `5add1cb9` landed the checked binder layout and `AC-7` at `main`
`f37ecd13`; confirmed Adversary finding `evt_kjjhmt36kt54` measured two defects
in it, and I verified both. They are not new scope — `AC-9` is discharged by the
occurrence-walk threading this deliverable already owes.

**AC-9 — the layout control must observe production, not itself.**
`CheckedCaseBinderLayout::for_case` **recomputes** the reversal from
`recursive_positions` and `argument_binders` — the same inputs production uses —
and never reads production's assembled environment. **Delete the `.rev()` at
`lowering/core.rs:4939` and `for_case` still reverses, so every layout assertion
stays green.** The one artifact named as owning the order cannot detect the
order changing.

Flip or delete the reversal at a **production** site and a layout control must
**red**. Report the mutation and its result. **A control that recomputes its own
answer from the same inputs is not a control** — it is the claim restated in
executable form, and adding more tests of the same shape does not help.

**AC-10 — the single-owner claim must be true when asserted, or qualified until
it is.** The landed comment says `for_case` performs *"the one `.rev()`"* and
that a lowering change moving the prefix is *"a single-site correction rather
than a hunt."* **Measured at `f37ecd13` there are four production `.rev()` sites
over `recursive_positions` in `lowering/core.rs` — 4939, 5496, 7094, 13708 — and
all four predate the candidate.** `CheckedCaseBinderLayout` carries
`#[cfg_attr(not(test), allow(dead_code))]` and is named only from `mod tests`.

The literal sentence is about consumers *of the layout* and is true of them; the
conclusion drawn from it is not. **"Single-site correction rather than a hunt"
is an instruction to the next implementer not to look, resting on a premise that
is false today.** Either make the four production sites index the layout, or say
plainly that they do not yet and name them, with a pointer at `core.rs:4939`
naming `for_case` as the intended owner so the sites are findable from either
end. **Fix the tense, not the design** — scaffolding for an unbuilt plane is
fine; stating an aspiration in the perfect tense is not.

## Excluded scope

- **No emission, and this is the cut.** No `AbiUnitDefinition` arm, no
  descriptors populated for emission, no `ContinuationEmissionOwner::Fusion`
  arm, no scoped source-body authorities, no generated-definition emitter, and
  **no redirection of the producer invocation edge**. Those are
  [[RT-LEXICAL-RECURSOR-CONSUMERS-D2f]].
- **No `R3` row is claimed green** and `AC-1`..`AC-3` of `D2d` are not
  discharged here — they need emission.
- **The existing continuation-specialization class is unchanged.** No `Option` on
  `worker`, no synthesized `ContinuationWorkerProvenance`, no reuse of
  `ContinuationSpecializationId`. The ruling introduced a separate class rather
  than relaxing this one, and relaxing it would silently widen its authority.
- **Row 5 after-hole stays reported-only.** `R4` belongs to
  [[RT-LEXICAL-ROW2-MISSING-MINT]]. `#6d` keeps its status; `D3` and the
  retirement are untouched.

## Stop conditions — return to the Architect, do not decide

Any need for a runtime continuation or callback; any **inference** of suffix
identity; any eager recursor invocation; any closure carrier or ABI
representation; any weakening of the whole-graph admissibility walk; any case
where the checked binder layout cannot distinguish an IH slot from an ordinary
binder.

**Added by the ruling and it is the live one:** if **no plan-backed checked twin
can reach the same producer to IH-consumer relation**, stop again and hand back
the seam. **Do not manufacture markers and do not make the member optional.**
The first stop on this fork was called correctly; a second one on the twin would
be too.

**Not a stop condition:** the emitter/ABI obligation being undischarged. It is
excluded here by construction and lands in `D2f`.

**Not a stop condition, as of 2026-08-10:** the checked-transport member's
required-versus-optional status. That fork is ruled — **required** — and this
frame carries the ruling. Do not re-escalate it.

## AC ledger, carried from the implementer's handback at `215bd156`

| AC | state |
|---|---|
| `AC-1` binder role | landed, first partial `f37ecd13` |
| `AC-7` denominator operand | landed, first partial |
| `AC-2` / `AC-3` indirection, no `Var` search | landed, threading partial `fe28ac7d` |
| `AC-9` control observes production | landed, threading partial |
| `AC-10` single-owner claim corrected | landed, threading partial |
| `AC-6` `D2b` controls retained | holds, untouched by both partials |
| `AC-8` population stated in the claim | retained and satisfied |
| `AC-4` bijection | **not delivered** |
| `AC-5` fail-closed and transplants | **not delivered** |

**The consuming-`Call` side needs no new mechanism** and this is measured, not
assumed: the consumer's selected case body is the exact suffix iff its callee
`Var` resolves, through the landed IH authority, to `CheckedIhBinding {
frame_origin: continuation_origin, recursive_position }`. That is `AC-5`'s
discrimination, and it is why only the checked coordinate gated the key's shape.

## Contention

Paths are `crates/ken-runtime/src/cranelift_backend/planning/**` and its test
targets, plus the one control file for the rider. Language owns
`crates/ken-elaborator`; Kernel's `D9` is on test targets and a `conformance/`
row. No `spec/` or `conformance/` path here, so no Spec vote on the merge
Decision.

## Sizing and validation

One turn to a releasable increment or a genuine hard stop; both are good
outcomes.

**This deliverable grew on 2026-08-10 and I am saying so rather than leaving the
sizing implicit.** The checked-transport ruling added a fixture twin built
through the checked path, a five-step fail-closed construction order, three
extra `AC-4` identity classes and four extra `AC-5` refusals. **The twin alone
may be a turn.** That is authorized scope — the Architect placed the paired
fixture and control work inside `D2e` — but **a partial that lands the twin and
its absence control, with the key still unbuilt, is a good outcome and I will
merge it.** Do not compress the fail-closed order to fit one turn; the order is
the soundness property.

Targeted only — `-p ken-runtime`, or `--test <name>` for one suite, **never
`--workspace`**. A new id and key add enum variants, so the floor is a full
`-p ken-runtime` test build: a suite-scoped run cannot observe an exhaustive
`match` in a sibling target. "No regression" means green in CI.
