---
id: RT-ROUTE-EQUALITY-RECONSTRUCTION-PIN
title: "The route-equality controls' duplicated hand-built reconstruction is the mechanism that detects an added routing disjunct -- record that at both assertion sites and retain the C-arrival mutation, because a routine tidy-up deletes it silently"
status: draft
owner: runtime
size: S
gate: none
depends_on: [RT-ROUTE-EQUALITY-CONTROL-B-ROW]
blocks: []
github: null
origin: "Adversary hunt on merged 0e9d20444 (evt_2xxr83djrtepq), filed by the Steward. Queued behind lane 1 -- nothing regresses while it is open, and the mechanism it protects is present and working today."
---

## What this node is

**A defence of a mechanism that already works, against the specific edit most
likely to remove it.** Nothing is broken. The controls landed at PR #2470 do
what they claim.

## The finding, from the Adversary at `evt_2xxr83djrtepq`

The route-equality controls hand-build the routing decision twice —
`control.rs:16543` (`vec![ordinary_route]`) and `:16590`
(`vec![route_a || route_b]`) — and compare each against the routing site's
**recorded** decision.

**The duplication is the detector.** A hand-built reconstruction **cannot know
about a disjunct added later**, so when a future `C` enters production routing,
the recorded decision goes `true` while the reconstruction stays `false` and the
assertion reds. **That ignorance is the mechanism.**

⇒ **A routine tidy — factoring the two into one shared test helper, or calling a
production helper that computes the route — makes the reconstruction track the
record automatically, and the detector dies silently in both controls at once
with every assertion still passing.**

> ### THIS IS THE INVERSE OF THE USUAL INSTINCT, which is why it will happen
>
> Duplicated logic in a test reads as debt. **Here, removing the duplication
> removes the check.** Nothing in the file says so.

## Two directions, and only one of them is what the tuple assertion catches

| direction | caught by |
|---|---|
| `B` **leaving** | the 4-tuple assertion — `route_b` and the recorded route both flip |
| a disjunct `C` **arriving** | `assert_eq!(observed_routes, vec![route_a \|\| route_b])` |

**These are different assertions, and the shipped mutation exercises only the
first.** The `Construct`-to-scalar mutation proves `B` leaving. The arrival
direction is the one the control is usually cited for.

**A correction to the finding, in the direction that reduces the work.** The
`C`-arrival mutation **has been run**, transiently, twice during the
`D3-narrow` recut. The implementer's handback recorded it — *"future route
disjunct C forced the difference row red: actual route `[true]` versus
constructed current `A || B` `[false]`"* — and QA reproduced it independently
(`evt_5j2pbqncf7pxd`: *"adding route disjunct C fails the difference row at
observed-vs-constructed route equality"*). **So the capability is demonstrated,
not merely argued.** What is missing is that the demonstration was never
retained and the mechanism was never documented.

**Coordinates verified by the Steward** at `main` = `5c5ee5b6c`: both
reconstructions are hand-built, `vec![ordinary_route]` and
`vec![route_a || route_b]`, each compared against
`take_match_scrutinee_producer_route_decisions()`.

## Deliverables

**`D1` — one clause at each of the two assertion sites**, in the file, saying
that the reconstruction is **deliberately** hand-built and that routing it
through a shared or production helper stops it detecting an added disjunct.

**`D2` — retain the `C`-arrival mutation as a recorded proof**, in whatever form
the crate already uses for mutation evidence. It is the same shape as the
mutation already run for `B`-leaving: add a third disjunct to the production
binding, observe the reconstruction-versus-record assertion red, revert.

## Acceptance criteria

**`AC-1`. A reader editing either assertion meets the warning without leaving
the file.** A tracker node does not discharge this — **the person doing the
tidy-up is reading `control.rs`, not this node.**

**`AC-2`. The `C`-arrival red is reproduced and recorded**, so a later reader
does not have to re-derive that the mechanism works.

**`AC-3`. No production change and no control behaviour change.** The
assertions, the rows and the observation domain all stay exactly as they landed.

**`AC-4`. No-regression, in CI** (`COORDINATION §12`).

## Banned scope

- **Factoring the two reconstructions together**, which is the defect.
- **Editing the routing predicate or the retention guard.**
- **Adding rows.** The population question was settled by
  [[RT-ROUTE-EQUALITY-CONTROL-B-ROW]].

## Sequencing

**`draft`, and `draft` here means QUEUED, not unframed.** Dispatchable on sight.
Lane 1 is [[RT-MATCH-SCRUTINEE-PORT]], which is `active` with the ring.

**Flip it `ready` when the runtime ring is between port increments.** It blocks
nothing, and the mechanism it protects works today — the risk is a future edit,
not a present defect.
