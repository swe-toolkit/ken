---
id: RT-GRAFTED-SPINE-IR-REPRESENTATION
title: "Represent emission ownership once in the IR instead of re-deriving it per phase. Emission ownership is currently computed by planning phase A, dropped at the DeferredResponseRow record boundary, and re-derived downstream from syntactic occurrence containment via occurrence_subtree_contains -- so how many times an effect is dispatched is a function of which copies happen to contain its occurrence rather than of the grafted object itself. Fork (A) of the RT-PLANNER-KRET-GRAFTED-SPINE (A)/(B) ruling; the Architect frames and prices it."
status: draft
owner: runtime
size: TBD
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
