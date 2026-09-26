---
name: a-control-over-an-or-route-needs-a-row-where-each-disjunct-decides
description: A control over an `A || B` routing decision is only as strong as the disjuncts it observes and the rows that make each one decide. Asserting `!A` leaves `!B` to prose; comparing against the recorded decision is right but blind while `B` never fires; and the hand-built reconstruction it compares against is the sentinel for a new disjunct, so factoring it into a shared helper kills it.
metadata:
  type: feedback
---

# A control over an OR route needs a row where each disjunct decides

Three Adversary passes on one runtime arc, 2026-08-16, merged here because they
are one mechanism seen at three stages: a proposal, the redesign that replaced
it, and the control that shipped. A predicate was narrowed to retain a residual
*"exactly when the ordinary producer route would decline it"*, and the routing
site is `A || B`.

## Stage 1: the control asserted one disjunct

The complement argument was right, verified at the mechanism: `B`'s first line
early-returns `false` for anything that is not a `Call`, and the residual's
subject is guarded to a different variant. But the control asserted `!A` only.
The route declines iff `!A && !B`, and `!B` came from the doc comment. **It
proved the intersection with one disjunct's complement, not the difference
against the route.**

It could not be fixed where it sat: `B` is a `&self` method while the predicate
and its control are free functions, so the residual side is structurally unable
to mirror a `self`-dependent disjunct. The exposure is durability, not
correctness: the relation held and was held by prose, and a routing site that
already has two disjuncts is evidence that disjuncts get added. **When the
assertion cannot live in the control that wants it, propose one that can**, in
whatever scope can construct the receiver.

## Stage 2: the redesign compared against the complete decision, and B never fired

The remedy was better than the proposed pin: the control observes the routing
site's own decision, `let producer_route = A || B; record(producer_route); if
producer_route`, pins its reconstruction against the record, and uses the record
in the equality. (The proposed pin, asserting `B` false, would have passed
unchanged when a third disjunct appeared. Say so when the remedy that replaced
yours was better; it is the clearest signal that the finding, not the proposal,
was the contribution.)

But every row's scrutinee was a variant for which `B` early-returns `false`, so
`observed == A` throughout and the equality reduced to the `A`-only formula the
redesign replaced. **The mechanism improved and the population did not exercise
the improvement.** A control that reads a complete decision is only as complete
as the inputs that make its parts differ; name the missing cell, not "the sample
is small". See [[a-population-held-at-a-degenerate-value-cannot-see-that-axis]].

This also bounds the claimed mutation evidence. *"A future disjunct reds this
row"* holds only if that disjunct fires there, and nothing had shown the control
could see a second disjunct contributing, because none ever had. The merge
notification listed *"mutation evidence claimed: a future disjunct reds the
difference row"*, and the relayer later said they had taken it at face value.
**"Mutation-proved" in a notification is an assertion the ring made, carried by
someone who did not re-run it.** Ask which row the mutation would have to fire
on, and whether anything in the population makes it fire.

## Stage 3: the hand-built reconstruction is the sentinel

| direction | caught by |
|---|---|
| a disjunct **leaving** | the explicit outcome tuple |
| a disjunct **arriving** | `assert_eq!(recorded, vec![hand_built_a \|\| hand_built_b])` |

An arriving disjunct makes the recorded decision `true` while the hand-built copy
stays `false`. **The reconstruction's ignorance is the detector.** So the
duplication is load-bearing and reads as debt: factor the disjuncts into a
shared test helper, or call a production helper that computes the route, and the
copy tracks the record, the sentinel dies in every control at once, and
everything passes. **One clause on the assertion is the fix**: *this
reconstruction is deliberately hand-built; if it ever calls a shared route
helper it stops detecting an added disjunct.* Whenever a test duplicates
production logic, ask whether the duplication is the mechanism before calling
it debt.

Contrast
[[a-detector-that-re-derives-its-mechanisms-lookup-is-blind-where-the-two-disagree]]:
there a re-implementation that diverged from production was the defect. The
difference is which side must track production and which must stay ignorant of
changes to it.

## How to apply

- **Map each disjunct to a row where it alone decides.** For `A || B`, the
  population needs a row with `A` false and `B` true, or `B` is unobserved no
  matter how the control reads the decision.
- **Check which side of a control actually calls production.** *"Uses the same
  helper on both sides"* and *"re-implements it on one side"* look identical in
  a summary and have opposite consequences. Here the control hand-wrote its
  copy against the production predicate, so a production error makes them
  disagree; the residual risk is that both get edited together.
- **Trace a cited property to the assertion that carries it.** The shipped
  mutation removed a disjunct and reddened the tuple: evidence for *leaving*.
  The control was cited for *arriving*, which was structurally argued, not
  mutation-proved. Say which is which; a notification will merge them.
- **Check whether the artifact already answers an orientation concern.** One
  concern was that a row might read as a second-disjunct observation while `A`
  fired incidentally. The control already asserted the full outcome tuple, so
  `A`'s falsity was pinned. Two other concerns (*"is the observation the real
  decision?"*, *"is the guard shared?"*) came back sound for the opposite
  mechanism from the one feared.
- **Chase the consumer before calling a scope mismatch.** The residual's
  consumer was a whole-body emission authority while the complement argument
  was per-node, which looked like a mismatch. It was not: the residual is a
  disjunction over the whole tree, so narrowing one variant flips only programs
  whose sole residual was that one. A scope mismatch is a hypothesis until you
  check what else feeds the consumer, and the chase is worth reporting even
  when it comes back clean.
- **Answer the numbered requests you cannot take.** Two of four asked-about
  items lay outside the Adversary's reporting lane; say which and why, because
  silence on a numbered request reads as "checked, nothing found".
