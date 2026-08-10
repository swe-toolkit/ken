# RT-LEXICAL-RECURSOR-CONSUMERS D2e — the identity plane, planner-only

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

**Take `D2d`'s grounding record as a fixed input.** Every key member except the
checked frame/invocation-template identity is already a measured coordinate on
the `R3` before-hole witness — the unique `0 -> 2` producer edge, the two-entry
ordinary `ValueWord` projection, the consuming `Call` at `StaticOriginId(12)`.
Read it at `docs/program/wp/RT-LEXICAL-RECURSOR-CONSUMERS-D2d-GROUNDING.md`.
**Do not re-derive those coordinates; derive the mechanism that produces them.**

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

**3. The exact re-derivation validator.** Re-deriving the key from planner facts
yields the same members. This is what converts the grounding tuple into a
mechanism: each member comes from a planner-issued identity, never from a
spelling, a type, a row number, a runtime tag, or *"the only continuation."*

**4. The suppressed-leg denominator repair — a rider, two lines.** Confirmed
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

**AC-5 — fail-closed, and this is the soundness-bearing control.** A producer
**lacking** the exact consuming suffix yields **no fusion and the ordinary
existing refusal** — never a fallback to the unspecialized result-returning unit.
Independently transplant (a) the call identity and (b) the segment owner; **both
must reject, and reject before any definition is created.**

**This is `D2d`'s `AC-4` and it is why the ruling forbids "the only
continuation."** An identity that happens to be unique in the measured
population is not an identity — it is the same existential shape that got
`d94ef37e` rejected on `D2b`. The transplant controls are what separate the two.

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

**Not a stop condition:** the emitter/ABI obligation being undischarged. It is
excluded here by construction and lands in `D2f`.

## Contention

Paths are `crates/ken-runtime/src/cranelift_backend/planning/**` and its test
targets, plus the one control file for the rider. Language owns
`crates/ken-elaborator`; Kernel's `D9` is on test targets and a `conformance/`
row. No `spec/` or `conformance/` path here, so no Spec vote on the merge
Decision.

## Sizing and validation

One turn to a releasable increment or a genuine hard stop; both are good
outcomes. **The implementer reported context budget as the binding constraint on
the last turn — this deliverable starts on a fresh one.**

Targeted only — `-p ken-runtime`, or `--test <name>` for one suite, **never
`--workspace`**. A new id and key add enum variants, so the floor is a full
`-p ken-runtime` test build: a suite-scoped run cannot observe an exhaustive
`match` in a sibling target. "No regression" means green in CI.
