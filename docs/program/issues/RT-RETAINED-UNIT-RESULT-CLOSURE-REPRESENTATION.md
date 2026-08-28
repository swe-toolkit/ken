---
id: RT-RETAINED-UNIT-RESULT-CLOSURE-REPRESENTATION
title: "Post-call retained-unit RESULT closure-representation route at constructor composition — the distinct object behind the retained-unit target derivation. After the derived declared-unit call returns (lower_computational_producer_call, core.rs:5221-5227), the carrier is handed to lower_computational_match_value_composed (core.rs:5228-5232); later constructor composition sees a carried sibling, enters transfer_constructor_operands, and its whole-child preflight (core.rs:11478-11485) meets a specialized raw closure and refuses via boundary_transfer_admissibility. This consumer reads value-class / aggregate-representation facts, NOT call-target facts, which is why it is a different component object. NOT framed as generic Closure transport: the same total refusal arm is shared by multiple production roots, and a repeated refusal string proves the shared GATE, not a shared PRODUCER."
status: draft
owner: runtime
size: M
gate: none
tier: T1
depends_on: [RT-RETAINED-UNIT-CALL-TARGET-DERIVATION]
blocks: []
github: null
origin: "Architect component ruling evt_2jdfsv6w8nh19, 2026-08-28 (thr_4vk7ks9gk3bmn), on exact base 077286dbcaf166af4c3bd8b8910c113871825690 and exact WIP 6282f149545c973f801c1cf8e715212d9ba77d99 / tree 3993a2653d5812fa97c6318261ccd73c7f88b112. Runtime raised the component-design request evt_5gytgyp26b3sp after its completed target derivation advanced the trigger row to a new refusal. The Steward split the question at evt_6mmj97njjpq3e: the WP cut and partial-landing call are the Steward's, the distinctness call the Architect's, and the Steward asked specifically whether this chain had begun repeating the HS7 last-gap-decomposition defect (evt_1z1p9t4tdyd2v). The Architect ruled it does NOT: HS7's predicate was 'each stop supplies another missing input to the same generated-entry accessor at the same final consumer', and no such predicate holds here. Steward-owned framing per the ruling's disposition."
---

> # OPERATIVE — Architect component ruling `evt_2jdfsv6w8nh19` (2026-08-28)
>
> **DO NOT FRAME THIS AS GENERIC `Closure` TRANSPORT.** This exact px8f row
> already crossed the earlier M4 checked-continuation seam and then advanced to
> M3. The same total refusal arm is shared by multiple production roots. **A
> repeated refusal string proves the shared GATE, not a shared PRODUCER** — that
> inference is the trap this frame exists to prevent.
>
> **WHY THIS IS A SEPARATE OBJECT FROM THE TARGET DERIVATION**, in the ruling's
> own terms: target derivation is static graph ownership and function declaration
> BEFORE the call; this is post-call result composition and runtime-value
> representation. Suppressing the graph claim prevents the call and restores the
> earlier miss; supplying the exact graph claim completes that lookup and exposes
> this later value boundary. **That is causal sequencing, not one accessor
> missing another field.**
>
> **This chain does NOT share the HS7 defect predicate.** HS7's predicate was
> "each stop supplies another missing input to the same generated-entry accessor
> at the same final consumer." The only common description here is "the same
> end-to-end fixture advances to the next compiler layer," which is not a
> component boundary and cannot justify folding unrelated authority into one
> mechanism.

## Objective

Establish, from planner/declaration authority, the exact post-call retained-unit
RESULT closure-representation route at constructor composition, so the refused
occurrence is authorized by a typed relation rather than by the refusal itself.

## Fixed inputs (Architect `evt_2jdfsv6w8nh19`, exact base `077286dbc`)

- The prior object's authority ENDS at `core.rs:5221-5227`, where
  `lower_computational_producer_call` consumes the declared target.
- The returned carrier is handed to `lower_computational_match_value_composed` at
  `core.rs:5228-5232`.
- Constructor composition then sees a carried sibling and enters
  `transfer_constructor_operands`; the whole-child preflight at
  `core.rs:11478-11485` encounters a specialized raw closure and applies
  `boundary_transfer_admissibility`, which refuses it. Localized refusal site:
  `core.rs:11483`.
- Trigger row: `crates/ken-cli/tests/px8f_buffer_native.rs:201`, whose `#[ignore]`
  is re-pointed to THIS node by the predecessor.

## Deliverables

- **D0 — identify the exact occurrence, before proposing anything.** From
  planner/declaration authority establish: the exact refused closure occurrence,
  its body, its capture environment, the result constructor owner and field, and
  the generated-unit endpoints. **This is identification, not repair.**
- **D1 — reconcile against the LANDED discipline, and take exactly one branch.**
  Reconcile the D0 row against `RT-CLOSURE-CROSSING-ELIMINATE` / M4:
  - **If an existing planner-owned representation row already covers this exact
    occurrence: repair its REACH/WIRING. Do NOT mint a parallel representation.**
  - **If the planner can prove one exact static body plus its exact positional
    captured-environment record:** eliminate the raw closure crossing through the
    existing admitted `Record` plus static-dispatch discipline. **Ordinary
    Ret/constructor fields not carrying that exact authority remain unchanged.**
  - **If NO existing typed relation can authorize the exact post-call result
    occurrence: STOP and return the design gap.** That is a correct outcome of
    the turn, not a failure of it.

## Acceptance criteria, each with its control

- **AC-EXACT-OCCURRENCE.** D0's row is established from planner/declaration
  authority. Control: it must be derivable without reading the refusal.
- **AC-NO-PARALLEL-REPRESENTATION.** If an existing row covers the occurrence,
  the change repairs reach/wiring and mints nothing parallel. Control: a census
  showing no second representation for the same occurrence.
- **AC-REFUSE-MALFORMED.** Missing, duplicate, wrong-owner, wrong-body,
  wrong-field, and wrong-capture relations must REFUSE. **Controls must
  substitute REAL NEIGHBORING LEGAL ROWS — never numeric origins and never
  emission-local identity.** Each mutation needs its own same-shape positive.
- **AC-UNRELATED-FIELDS-UNCHANGED.** Ordinary Ret/constructor fields not carrying
  the exact authority are byte-unchanged. Control: a differential over those
  fields.
- **AC-GATE-INTACT.** `boundary_transfer_admissibility` still refuses everything
  it refused before, for every root that is not this authorized occurrence.
  Control: the shared refusal arm still fires for a neighbouring production root.

## FORBIDDEN — any of these means the turn stopped wrong

Do NOT weaken `boundary_transfer_admissibility`; do NOT revive a
persistent/durable closure lane; do NOT add a `(tag,class)` admission; do NOT
silently convert to `StaticCallableRef`; do NOT scan for a closure; do NOT choose
first-match; **and do NOT derive authority from this refusal.**

## Sequencing

**`draft`, and deliberately not `ready` yet — the frame is COMPLETE, the status
is the honest part.** This node is genuinely gated on
[[RT-RETAINED-UNIT-CALL-TARGET-DERIVATION]] LANDING as an accepted partial: the
refused occurrence only appears once the derived declared-unit call succeeds, so
nobody can start here before that. `ready` would claim any team may pull it,
which is false today.

**Flip it `ready` and release the moment the partial lands** — the frame needs no
further work. The Architect reviews the WP at release; a candidate takes fresh
Architect then Runtime QA gates. Tier T1 — the work turns on which authority may
license the occurrence, not on a mechanical diff.
