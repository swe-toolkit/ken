---
id: RT-GRAFTED-SPINE-IR-REPRESENTATION
title: "Represent emission ownership once in the IR instead of re-deriving it per phase. Emission ownership is currently computed by planning phase A, dropped at the DeferredResponseRow record boundary, and re-derived downstream from syntactic occurrence containment via occurrence_subtree_contains -- so how many times an effect is dispatched is a function of which copies happen to contain its occurrence rather than of the grafted object itself. Fork (A) of the RT-PLANNER-KRET-GRAFTED-SPINE (A)/(B) ruling; the Architect frames and prices it."
status: draft
owner: runtime
size: XL
gate: none
tier: T1
depends_on: [RT-PLANNER-KRET-GRAFTED-SPINE]
blocks: []
github: null
origin: "Filed by the Steward on the Architect's signal, 2026-09-19, at origin/main d375c861731a81cff8b30a9ab29e85919bc189dc. The (A)/(B) fork is stated in docs/program/wp/RT-PLANNER-KRET-GRAFTED-SPINE.md section 7; the Architect ruled the (A) trigger MET at M3 (evt_12mfx5rxrp3g4) and specified this node's id, status, owner and depends_on at evt_7tw1rpckksbfd, including the trigger clause reproduced verbatim below. Hard stop 4 (evt_2vv4eedcvpz1a) then supplied a worked instance. Filed draft and NOT released: the Architect owns framing and pricing and is holding that."
---

# `RT-GRAFTED-SPINE-IR-REPRESENTATION` — fork (A)

**The Architect frames and prices this node.** It is filed so that the
obligation exists as a tracked object rather than as a thread commitment; the
contents below are the two things that must not be re-derived.

## THE TRIGGER CLAUSE — Architect, `evt_7tw1rpckksbfd`, verbatim

> **The trigger is MET as of `M3`. It is not contingent on the local repair
> failing.** If `RT-PLANNER-KRET-GRAFTED-SPINE` closes green,
> `RT-GRAFTED-SPINE-IR-REPRESENTATION` is **still owed** — the local repair
> fixes this route; `occurrence_subtree_contains` still keys emission on
> syntactic occurrence containment, and `bind_bind` at `≅` still makes that not
> a function of the grafted object. **A green row is not evidence against this
> node.**

This clause is the reason the node is filed now rather than at the predecessor's
closeout. Without it the sequencing behind `RT-PLANNER-KRET-GRAFTED-SPINE`
becomes the mechanism by which the node lapses once the symptom disappears.

## THE WORKED INSTANCE — hard stop 4, Architect, `evt_2vv4eedcvpz1a`

At `:761-776` of

    crates/ken-runtime/src/cranelift_backend/planning/static_transition/responses.rs

`struct DeferredResponseRow` carries four `StaticOriginId`s, `operation`,
`sub_case`, `capture_count` and `continuation_input_count`. **It carries no
emission-owner field of any kind** — not the base owner, not the caller's.

⇒ Phase A computes the ownership relation, the record drops it, and a later
phase re-derives it from occurrence containment. The Architect's statement of
what that makes this node:

> **(A)'s thesis is *represent the relation once instead of re-deriving it per
> phase*** — so this stop is not merely consistent with (A), **it is a worked
> instance of exactly what (A) removes.**

## WHAT THE PREDECESSOR'S REPAIR DOES AND DOES NOT SETTLE

The scope authorized for `RT-PLANNER-KRET-GRAFTED-SPINE` at hard stop 4 retains
the phase-A owner pair on the row and drives the effect once in its exact caller
owner. **It is neither (A) nor (B).** `§7`'s (B) derives the property from the
object at each site, which is what makes it a function of the object; this
repair instead **retains and propagates a key that is itself
occurrence-derived** — `inline_synthesized_seat_emission_owners`
(`planning/static_transition.rs:743`) builds the owner set by
`occurrence_subtree_contains` and sibling containment tests, so carrying the
owner pair further carries the occurrence derivation further. It makes dispatch
a function of one designated owner instead of every containing copy, which is
strictly better and unblocks the row. **It does not close the class, and it does
not (B)-repair this site** — nothing in it should be read as doing either.

## STANDING CONSTRAINT INHERITED FROM THE PREDECESSOR

No acceptance criterion on this node may quantify over the **absence** of
occurrence-keyed derivations. State what the representation carries and where
the equality is shown, not what no longer exists.

# FRAMING AND PRICE — Architect, `evt_3sftwjey91wjs`, verbatim

**Transcribed by the Steward; authored by the Architect.** The sections above
are deliberately not restated here: the trigger clause, the hard-stop-4 worked
instance, the neither-(A)-nor-(B) paragraph, and the standing constraint are
fixed inputs the Architect did not re-derive, and neither should a reader.

## 1. The subject is one relation. It is not the containment primitive.

The relation is **seat origin -> set of `ContinuationEmissionOwner`**: which
owners emit this seat. It is built in one function, in three arms, and **all
three are occurrence-derived**:

    static_transition.rs:743  inline_synthesized_seat_emission_owners
      arm 1  emittable units    emission_source_origins(fn, body).contains(seat)
      arm 2  continuation units occurrence_subtree_contains(selected case body, seat)
      arm 3  deferred rows      occurrence_subtree_contains(k_body, seat)

**Arm 1 looks object-keyed and is not.** `emission_source_origins`
(`joins_traps.rs:754`) resolves to a walk over `source_occurrences` filtered by
`function_owner`, or to `source_origins_in_owner_subtree(body_occurrence)`, with
worker carve-outs — a source-origin walk with extra steps.

**IN SCOPE:** those three arms, the record that replaces them, and the consumers
that read the result.

**OUT OF SCOPE, named so nobody absorbs them.** `occurrence_subtree_contains` is
a general primitive with other clients asking other questions. These are **not**
ownership, and **(A) does not retire the primitive**:

    occurrences.rs:507   provably_dead_arm_body_containing
    responses.rs:483     checked_ih_caller_completed_required_consumer_edge_index
    responses.rs:3457    deferred_response_handler_prefix_len
    responses.rs:3524    deferred_no_unit_response_in_body
    responses.rs:3993    checked_ih_detached_caller_construct_binds
    aggregates.rs:7157   checked_ih_generated_entry_row

`deferred_response_handler_owner` (`responses.rs:3389`, three containment sites)
**is** ownership and is in scope.

**That split is the Architect's classification, not a measurement.** Confirming
it is `D0b`. **A site mis-sorted is a finding to report, not a scope to absorb.**

## 2. The precedent is already in the tree, and it is the same move

This codebase has **already retired a containment proxy once**, for a
neighbouring relation, and kept the proxy as a negative oracle:

    aggregates.rs:4816   checked_ih_force_emissions -- the live relation.
                         Owners come from iterating plan.continuation_units()
                         and inserting Specialization(unit.id()) for the unit
                         that HAS the edge. Object-keyed. It contains no
                         containment call.
    aggregates.rs:11650  legacy_context_containment_owners -- "The retired
                         context-subtree proxy, retained only as the negative
                         oracle proving the force relation does not
                         accidentally collapse back onto it."
    aggregates.rs:11368  non_inline_host_effect_records_equal_their_actual_
                         emission_owner_set -- asserts the disagreement.

⇒ **(A) is not novel architecture here. It is the generalization of a move this
file family has already made**, with the oracle discipline established. That is
the single largest fact in the price.

**AND THE PRECEDENT'S FIXTURE DOES NOT TRANSFER. Do not reuse it.** Its
discriminator is between the *context* proxy (`context.worker_body_origin`) and
reality. (A)'s arm 2 keys on the *selected case body*, and that test's own
**positive control asserts the selected body DOES contain the seat** — so on
that fixture, containment gets (A)'s relation **right**. **The shape and the
oracle discipline transfer; the witness does not.** (A)'s relation needs its own
discriminating witness, and producing it is `D0a`.

## 3. `D0` — run this first. It sets the shape of the acceptance.

**`D0a` — the witness.** Can two source bracketings related by `bind_bind`
re-association, producing the same grafted object, be expressed in Ken's surface
today, and do they yield **different** occurrence-derived ownership? That pair is
`AC-5`'s witness and the direct instance of the theorem.

**If it cannot be constructed today, say so and stop.** That is a first-class
result, not a failure: it means `AC-5` is not yet observable and acceptance rests
on `AC-4` alone until the surface can express one. **Do not manufacture a
witness** — an AC whose evidence must be manufactured is the one that gets
silently skipped.

**`D0b` — the classification in `§1`.** Confirm which containment sites compute
ownership. Report the residue rather than sorting it by resemblance.

## 4. Acceptance criteria

Each is positive. **None quantifies over the absence of occurrence-keyed
derivations**, per the standing constraint.

- **`AC-1` CARRIED.** The relation is a planner-issued record attached to the
  emission. The candidate **names** the record, its type, and its single
  construction site.
- **`AC-2` BUILT ONCE, READ EVERYWHERE.** Every consumer enumerated in `§1` reads
  that record. The enumeration is `§1`'s as corrected by `D0b` — a closed
  checkable list, not "no other derivation exists".
- **`AC-3` AGREEMENT, WHERE IT MUST HOLD.** On the existing fixture corpus the
  recorded relation agrees with today's derived relation. The no-regression half,
  stated as agreement on named fixtures.
- **`AC-4` NON-COLLAPSE — THE LOAD-BEARING ONE.** Following the precedent, the
  containment derivation is retained as a negative oracle, and a **named fixture
  shows the recorded relation and the proxy disagree, in a stated direction.**
  The test **must fail on the pre-change tree** — run it there and report that it
  does. **Without `AC-4`, `AC-1`–`AC-3` are all satisfiable by a pure rename.**
  This is not an absence claim: it is a measured disagreement on a named fixture,
  and it is executable.
- **`AC-5` OBJECT-INVARIANCE.** On the `D0a` pair, the recorded ownership is **the
  same** for both bracketings and the occurrence-derived one is not. This is the
  property (A) exists for. Governed by `D0a`: if no pair is expressible, `AC-5` is
  **deferred with that finding recorded**, never discharged by a substitute.

## 5. Hard stops — report, do not work around

1. **A second record, or a second relation**, to make the first one work. Same
   tell as the predecessor's fourth site: a "helper" on the first concept.
2. **A disagreement in the wrong direction.** `AC-4` wants the proxy and the
   record to differ. If they differ where **today's behaviour is correct**, the
   new relation is wrong — **stop and report; do not adjust the recording to
   match.**
3. **Retiring `occurrence_subtree_contains`**, or touching the `§1` out-of-scope
   clients.
4. **The predecessor's four-site bound is not this node's budget.** Do not absorb
   work from `RT-PLANNER-KRET-GRAFTED-SPINE`, and do not let this node be used to
   finish it.

## 6. Price

Measured surface, at `origin/main` `d375c861731a81cff8b30a9ab29e85919bc189dc`:

    inline_synthesized_seat_emission_owners     9 call sites
    deferred_response_handler_owner            10
    emission_source_origins                     4
    source_origins_in_owner_subtree             3
    deferred_response_handler_prefix_len        1
    containment sites, non-test                12 (~5 classified ownership)

Six files, two of them very large (`aggregates.rs`; `lowering/core.rs` at 16,487
lines).

**`size: XL`, and it is NOT executable as one candidate.** Sliced:

    D0   XS   the two measurements in §3. Cheap, and it decides whether
              AC-5 is observable at all.
    S1   L    the record + the builder + arm 2 (continuation units) -- the
              arm the trigger was established at.
    S2   M    arm 1, emittable units / emission_source_origins.
    S3   M    arm 3, deferred rows.

Each slice carries its own `AC-4` negative oracle. **`S1` first** — it is where
the trigger was established, and the other two get cheaper once the record
exists.

**The price is reduced by two things, named so the reduction is auditable:** the
precedent supplies the shape and the oracle discipline, and the predecessor's
hard-stop-4 repair **already installs the phase-A owner pair on
`DeferredResponseRow`**, which is `S3`'s first increment arriving under another
node. **So `S3` must be re-priced after that lands, not now** — the number above
is today's, against a tree that does not yet carry it.

## 7. One observation, not a deliverable

The doc comment on `checked_ih_force_emissions` (`aggregates.rs:4807-4810`) is a
collapsed sentence: *"It differs from [`inline_synthesized_seat_emission_owners`]
is the authority for aggregates emitted inline at their own seat"*. **The
sentence stating the difference between the force relation and this node's
relation is corrupted — exactly the distinction (A) turns on.** It sits inside
`S1`'s reading path; repair it there. It is not a separate WP.

## STEWARD NOTE — SEQUENCING, NOT FRAMING

**`§6`'s price was measured at `d375c861731a`, and `§6`'s own last paragraph
says `S3` must be re-priced once the predecessor's hard-stop-4 repair lands.**
That repair is live now under `RT-PLANNER-KRET-GRAFTED-SPINE` at hard stop 5,
inside a four-site bound. **Do not re-price `S3` from this text; re-measure it
when that node closes.**

This node stays `draft` and is NOT released. `depends_on` is the predecessor,
and `§5`'s fourth hard stop forbids using this node to finish it.
