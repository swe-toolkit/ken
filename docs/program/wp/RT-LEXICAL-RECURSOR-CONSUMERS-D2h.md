# RT-LEXICAL-RECURSOR-CONSUMERS D2h — the key plane

Owner: runtime. Size: M. Node: [[RT-LEXICAL-RECURSOR-CONSUMERS]] (`#6d`).
Architect rulings `evt_2wwh9yamyhs7p`, `evt_6sk3czsbcr85r`, and
**`evt_2t67rtf6kaw5e` (the checked transport member is required)**.

**Seat tier: T1.** The `#8` suspension does not reach `#6d`.

**RELEASED 2026-08-11. The `D2g` hold is discharged.**
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

**AC-1 — the bijection holds, and the checked coordinate is three classes.**
key↔ID↔descriptor round-trips. Exercise **one member per identity class**
rather than one representative — a bijection test on a single member says
nothing about the others. The Architect specified the set:

- the same complete valid key interns to the same id and round-trips;
- two complete valid keys differing in the **checked frame** member intern
  distinctly;
- likewise for the **selected slot-template** identity and path;
- likewise for the **invocation-template** identity and path;
- the ordinary one-member controls cover every other key identity class;
- **malformed transplants are validator refusals, not "distinct valid key"
  evidence.** Counting a refusal as a distinguished key would make the
  bijection look total while proving nothing.

**AC-2 — fail-closed, and this is the soundness-bearing control.** A producer
**lacking** the exact consuming suffix yields **no fusion and the ordinary
existing refusal** — never a fallback to the unspecialized result-returning
unit.

**Every one of these fires BEFORE any id, descriptor or definition is
created:**

- **strip the checked transport from the twin** — no fusion, ordinary refusal;
- **independently** remove or transplant the **frame**, the **selected slot**,
  and the **invocation** marker/plan relation — each rejects on its own;
- the **missing exact consuming suffix**, **call-identity transplant**, and
  **segment-owner transplant** controls are retained and still reject
  independently.

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
