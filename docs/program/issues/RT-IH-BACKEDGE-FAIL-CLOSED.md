---
id: RT-IH-BACKEDGE-FAIL-CLOSED
title: "Refuse at compile time, never lower to RecursiveBackedge, a functional-IH value in a non-tail position such as a Vis K constructor field, closing a latent miscompile that takes the recursive transfer without the response"
status: active
owner: runtime
size: S
gate: architect
tier: T1
depends_on: []
blocks: [RT-SOURCE-IH-RELAY-K-VALUE]
github: null
origin: "Architect ruling 2026-09-27 (evt_2b0dwgwyb0tvc): RT-SOURCE-IH-RELAY-K-VALUE AC-0a soundness STOP confirmed; a fail-closed correctness repair on main ahead of any representation work. Steward-sequenced after RT-OWNER-VIS-RETURN-PROTOCOL increment 1. Steward-filed per COORDINATION section 2."
---

# Fail closed on a non-tail IH backedge

## Objective

No compiled program contains a translation that is wrong whenever it runs.
A functional-IH value in a non-tail position is refused at compile time
instead of becoming a recursive jump that drops the response `r`.

## Settled inputs (Architect `evt_2b0dwgwyb0tvc`, from Runtime QA's AC-0a)

- **The contract.** `42 §2` makes constructor arguments eager.
  `42 §6.1-6.2` gives a `Vis` K the response `r` only when it is resumed.
- **The miscompile.** The source machine turns a K-field IH marker
  (`source.rs:829-857`) into `RecursiveBackedge` through `ConstructArgument`
  (`source.rs:1643-1660`) and the finisher (`source.rs:5941-5943`). That
  takes the recursive transfer without `r` and abandons the constructor.
- **Reachability does not decide it.** A runtime `-1` is the same
  observation whether or not the edge runs (Check 8). The disposition rests
  on the translation being wrong whenever it executes.
- **The core route is already right.** It takes the defunctionalization seam
  (`calls.rs:268-285`); leave it unchanged.

## Deliverable

A compile-time refusal, not a new value representation, at both ends:

1. **Producer.** A `CheckedComputationalIHInvocation` whose planned call is a
   functional-IH value (`call.arity == 0`, functional worker,
   `worker.declared_arity == 1`) never lowers to `RecursiveBackedge`; on the
   source-machine route it refuses with an exact error. Grep every marker
   arm (at least `core.rs:3574`, `core.rs:15382`, `source.rs:829`).
2. **Consumer.** No non-tail consumer forwards a `RecursiveBackedge`; it
   refuses with an exact error. Grep every `Lowered::RecursiveBackedge`
   match and post a table classifying each site as tail (forwarding lawful)
   or non-tail (refuses). QA's list is the floor: `source.rs:1643`,
   `source.rs:5941`, `core.rs:15550-15625`.

## Acceptance

- **AC-1.** The r2 fixture's observation changes from the runtime
  `UnclassifiedRuntimeTrap(-1)` to the new exact compile-time refusal; r2
  stays ignored, relabelled to that refusal.
- **AC-2 (control).** Removing the refusal restores the old observation.
- **AC-3.** Targeted suites unchanged; Full CI is the breadth gate. Any
  other program that newly refuses compiled the unsound translation: report
  each to the Architect with its site as a soundness finding, never weaken
  the refusal.

## Stop conditions

- A site whose tail status the plan cannot determine: stop to the Architect
  with the site.
- Any kernel, `trusted_base()` or spec change (an operator question).
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40`,
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY` or the child-2 checkpoint.
