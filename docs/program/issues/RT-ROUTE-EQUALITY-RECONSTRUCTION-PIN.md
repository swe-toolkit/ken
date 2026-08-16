---
id: RT-ROUTE-EQUALITY-RECONSTRUCTION-PIN
title: "The route-equality controls' duplicated hand-built reconstruction is the mechanism that detects an added routing disjunct -- record that at both assertion sites and retain the C-arrival mutation, because a routine tidy-up deletes it silently"
status: merged
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

### D2 mutation record

Reproduced at exact base
`19e0c69a7fed1df3d286cb166eefed109ec498b0`. The population-side mutation added
`matches!(scrutinee.as_ref(), RuntimeExpr::ComputationalMatch { .. })` as a
third disjunct only to the production `producer_route` binding. Both hand-built
`A || B` reconstructions remained unchanged. The targeted invocation

```sh
scripts/ken-cargo test -p ken-runtime \
  msd_d2a_residual_equals_subject_guard_and_route_complement -- \
  --nocapture --test-threads=1
```

executed one selected test and failed at the first
reconstruction-versus-record assertion on the `difference` row: the recorded
route was `[true]` while the hand-built route remained `[false]`. The production
mutation was then reverted byte-for-byte. This moves the production population,
not the detector, and records the exact boundary at which a future disjunct is
caught.

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

**`merged` at PR #2476**, landed squash `b4524ea4c`, candidate exact
`efe867483` on base `19e0c69a7`. One commit, two paths, `+25/-0`. Decision
`dec_87qh5g2zptd` resolved, Architect-approved at the exact SHA
(`evt_301h56fz7scq8`); QA `evt_2t92ftckc7pjx`.

**Both deliverables discharged where they had to be.** `D1`'s clauses are in
`control.rs` at each assertion, not in this file — `AC-1` was written to insist
on exactly that, because the person doing the tidy-up is reading the crate.

**It blocked nothing**, and the mechanism it protects worked the whole time —
the risk was a future edit, not a present defect. **The `C`-arrival red is on
record**, so a later reader does not have to re-derive that it works.

> ### THE ADDRESS WAS WRONG. Successor: [[RT-ROUTE-EQUALITY-PIN-AT-THE-BINDINGS]].
>
> Adversary hunt on the landed squash (`evt_4j3d7523jxh61`): **the clause landed
> on the assertions, and the duplication a tidy-up targets is the BINDINGS** 13
> and 17 lines above. A shared helper introduced at `:16528-16529` leaves
> `vec![ordinary_route]` reading identically ⇒ **the refactor that kills the
> sentinel completes without the warning entering view.**
>
> **`AC-1` here is satisfied as written and that is the point** — *"a reader
> editing either assertion"* is true; **the dangerous edit is not an assertion
> edit.** The clause protects the **use** while the refactor targets the
> **definition**. **The specification was the Adversary's and the Adversary
> caught it.**
>
> **`D2` is untouched by this.** The mutation moves the population, not the
> detector, and the arrival capability is exercised rather than argued.
