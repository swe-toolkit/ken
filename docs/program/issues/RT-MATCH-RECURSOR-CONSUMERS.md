---
id: RT-MATCH-RECURSOR-CONSUMERS
title: "Complete the MatchScrutineeRecursor consumer repair in Position A — the D2 increment closed one witness, not the population"
status: active
owner: runtime
size: M
gate: none
depends_on: [RT-CARRIED-CONTINUATION-RESUME, RT-CARRIED-ORDINARY-COMPOSITION, RT-SPECIALIZED-ACTIVE-RESUME, RT-CONTINUATION-CALL-DISCHARGE, RT-CONTINUATION-EDGE-DISPOSITION]
blocks: [RT-RECURSOR-TRANSPORT]
github: null
origin: Architect re-rule evt_3r4j14fv1jtj2 (2026-08-08) on the nine-expression census evt_16cmej481q7ns, partitioning RT-RECURSOR-TRANSPORT hard stop 4 by measured residual population. Row 6 (d8d) is a D2 completeness defect in Position A, not a lexical-successor row. Campaign docs/program/16-recursive-descent-retirement.md node #6d. Steward-filed (agents cannot create tracked work per COORDINATION §2).
---

> # `D9` — ADDED 2026-08-09. `D8`'s PIN READS 3 OF 8 WORKSPACE MEMBERS.
>
> **Confirmed Adversary finding `evt_1wxdxmpkxsakc` on the merged `D8`
> (`74c60c5d`), triaged by the Steward and folded here rather than filed as a
> node** — the repair is the same reader plus one loop, in the same file, on the
> same premise (`steward.md §4c`).
>
> **`D8`'s header says the premise "rests on exactly three facts". It rests on
> four.** The fourth: *no other workspace member enables `px8-ds-test-support`
> on a normal `[dependencies]` edge to `ken-runtime`.* The workspace declares
> **eight** members (`Cargo.toml:3-12`); the pin opens **three**
> (`mrc_d8_manifest_premise.rs:305-307`). Three it never opens already hold a
> normal edge to `ken-runtime`, each one `features = [...]` token away:
>
> | member | edge |
> |---|---|
> | `crates/ken-elaborator/Cargo.toml` | `:14` |
> | `crates/ken-interp/Cargo.toml` | `:13` |
> | `crates/ken-verify/Cargo.toml` | `:16` |
>
> ⭐⭐ **Resolver 2 withholds unification for DEV-dependency edges — which is
> exactly what fact 3 buys — but NOT across NORMAL edges of sibling members in
> one invocation.** CI runs `cargo build --workspace --locked`
> (`.github/workflows/ci.yml:58`), which is that invocation.
>
> **Measured, with a positive control.** Mutation: add
> `features = ["px8-ds-test-support"]` to the existing normal edge at
> `ken-verify/Cargo.toml:16`. Probe: `cargo tree --offline --workspace -e
> features,no-dev -p ken-runtime`.
>
> | leg | baseline | mutated |
> |---|---|---|
> | workspace normal-edge resolution | **0** | **1** — feature ON for `ken-runtime`'s normal build |
> | `-p ken-cli` normal graph (what `--bin ken` selects) | 0 | **0 — structurally blind** |
> | `mrc_d8_manifest_premise` | 10 passed | **10 passed** — the pin does not notice |
>
> ⛔ **THE TWO PINS DO NOT COMPOSE, AND THIS RETIRES THE `D8` SCOPE RULING'S
> SAFETY ARGUMENT.** `D8` was closed on manifest facts because
> `mrc_4a1_feature_gate_holds_at_the_artifact` was held to carry the
> artifact-level link. But 4a.1 builds `--bin ken`, selecting only `ken-cli`'s
> graph — **an invocation in which cross-member unification structurally cannot
> appear.** Both pins are keyed on `ken-cli`'s graph while the hazard lives in a
> sibling's. ⚠ **The `D8` ruling was still correct** — it declined to re-prove
> *Cargo resolver semantics*, which is not this. This is an unchecked fact about
> *this repository*, which is squarely what `D8` exists to pin.
>
> **Severity, stated in the honest direction: NOT a present defect.** No member
> enables it today; the baseline `0` is measured. This is a blind spot in the
> instrument whose entire purpose is catching the future manifest edit — and the
> natural future edit is precisely a sibling wanting test support and writing it
> into `[dependencies]`.
>
> ### `D9` — extend the pin to the workspace's own declared population
>
> ⛔ **Iterate `[workspace] members` from the root manifest. Do not enumerate
> the three members named above.** The declared member list is the artifact's
> own population, so iterating it closes the class **by construction** and stays
> correct when a ninth member arrives. An enumeration is correct today and
> silently wrong at the next `cargo new`.
>
> | AC | criterion | control |
> |---|---|---|
> | `AC-9a` | the pin reads **every** member in `[workspace] members` and asserts none enables `px8-ds-test-support` on a normal edge to `ken-runtime` | mutate the `ken-verify` normal edge as above → the pin **reds**; revert → green. ⚠ `D8`'s current pin stays **green** under that mutation, so this is a genuine new discrimination and not a restatement |
> | `AC-9b` | the member list is **read**, not transcribed | add a synthetic ninth member with the offending edge → the pin reds **without** editing the test. ⛔ A test naming eight paths fails this row |
> | `AC-9c` | the dev-dependency edge stays **accepted** | `ken-cli`'s existing featured `[dev-dependencies]` edge must not red — the non-degenerate pair. Without it, `D9` could pass by rejecting every featured edge anywhere |
>
> ⚠ **One thing the Adversary explicitly did NOT measure, and I am not
> inferring it:** which invocation cuts a release artifact. The hazard is
> established for `cargo build --workspace`, the invocation **CI** runs. If
> releases are cut `-p ken-cli` or `--bin ken`, the exposure is **CI-only** and
> this is instrument coverage alone. ⛔ Do not report `D9` as closing a
> shipped-binary exposure unless someone measures that separately. The repair is
> cheap under either answer, which is why `D9` does not wait on it.
>
> **Sequencing: FIRST in Runtime's queue. The "after
> `RT-DYNAMIC-ARM-SCALAR-MERGE` `D1`" note is WITHDRAWN** (operator, 2026-08-10:
> *"retiring RecursiveDescent is the priority"*).
>
> That note was a Steward sequencing preference, never a mechanism dependency —
> `D9` extends a manifest pin and touches none of the lowering surface `D1`
> touches. Its effect was to park this campaign's next actionable deliverable
> behind the ABI lane, which is how Runtime spent 08-09 22:12 onward on
> non-campaign nodes while the retirement directive stood. The premise still
> holds today, so `D9` is not urgent by severity — it is first because it is the
> cheapest thing standing between here and `#6d`.

> # `D10` — ADDED 2026-08-09. STRIP THE RELEASE GATE OUT OF A CODE COMMENT.
>
> **Confirmed Adversary finding `evt_37y39vcj7y695` on `82918b6a` (`D5`),
> triaged by the Steward and folded here rather than filed as a node** — it is a
> comment edit in a file this node's campaign already owns (`steward.md §4c`).
>
> `planning/static_transition.rs` carries an `#[ignore]`d control,
> `liftrose_synthetic_witness_closes_owner_two_required_joins`, authored by the
> now-**closed** `RT-BODY-OCCURRENCE-PROVENANCE`. Its doc comment states a
> release condition — *"nested-inductive admission is on `main`"* — and then
> gives the reader exactly one concrete way to decide it, at `:16518-16522`:
> `ken-elaborator/src/{compiler_driver,elab}.rs`,
> `ken-elaborator/tests/nc14_data_match_lowering.rs` and `ken-interp/src/eval.rs`
> *"each remain at their pre-change state."*
>
> **All four moved in `82918b6a`**, measured `afb38934..44c0ceab`:
> `compiler_driver.rs` `+8/-1`, `elab.rs` `+703/-217`,
> `nc14_data_match_lowering.rs` `+149/-2`, `eval.rs` `+233/-66`.
>
> ⇒ **A reader applying the comment's own test concludes the capability arrived
> and un-ignores the control** — verbatim the harm the same paragraph predicts
> three lines later. The sentence is grammatically scoped to `afb38934`, so a
> careful reader parses it as historical; the defect is that **the comment
> offers no current test and the one it offers reads GO.**
>
> **Direction, stated honestly: this is not a soundness defect and not urgent by
> severity.** The body is `panic!`, so un-ignoring yields a RED, not a vacuous
> green. The cost is a wasted red and a misdirected reader. What makes it worth
> doing now is *proximity*: Runtime is working in these exact files, so the
> reader is imminent rather than hypothetical.
>
> ### The repair is a DELETION plus a pointer. It is deliberately not a rewording.
>
> This condition has needed correction three times — `afb38934` pulled the
> predicates apart, `D7` re-keyed the gate from a merge event to a capability,
> and `D5` falsified the re-keyed version's evidence clause. **A gate re-keyed
> from event to capability but operationalized by a path-state snapshot is still
> event-keyed**; `D7` moved the problem one level down rather than removing it.
> A paragraph needing the same correction three times is tracking state that
> lives somewhere else.
>
> **The owner is now `KERNEL-NESTED-IND` `AC-K12`**, recorded in that node the
> same day. `AC-K12` *is* the capability — a nested-IH constructor lowers and
> evaluates — so the condition finally sits somewhere a status check can see it.
>
> | AC | criterion | control |
> |---|---|---|
> | `AC-10a` | the four-path snapshot at `:16518-16522` is **deleted**, not re-worded and not re-dated | grep the doc comment for `nc14_data_match_lowering` and for `pre-change state`: both absent. A fourth wording of the same test fails this row |
> | `AC-10b` | the comment names `KERNEL-NESTED-IND` `AC-K12` as the tracked owner, and offers the reader **no** local test for the release condition | the comment states where the condition is tracked and stops. It must not substitute a different snapshot, path list, or SHA |
> | `AC-10c` | the control stays **carried and fail-closed** — `#[ignore]` retained, `panic!` body and its four asserted properties byte-unchanged | the test still does not run, and removing `#[ignore]` still reds. This deliverable changes prose only |
>
> ⛔ **Do not discharge `AC-5` here and do not delete the control.** `D10`
> relocates the *condition*; the *obligation* is untouched and remains carried.
> Making the witness runnable requires the capability, which is not this node's.
>
> **Scope: one doc comment.** No production change, no test-body change, no
> `#[ignore]` removal. Prose-only, so no new suite is owed beyond the file still
> compiling.
>
> **Sequencing: alongside `D9`**, whenever Runtime's lane next turns. Both are
> small instrument-hygiene items in this campaign's own files and neither depends
> on the other.

> # THIS NODE EXISTS BECAUSE `D2`'s RECORD OVERCLAIMED — NOT BECAUSE `D3`
> # BROKE SOMETHING
>
> **The distinction changes what you are looking for, so read it before `D0`.**
> Row 6's refusal reproduces at exact `D2` `8efdfdb3`, with the `D2` repair
> active and production still selecting `RecursiveDescent`. **It is not
> downstream of `D3`'s retirement** — `D3` merely made it unavoidable.
>
> ⇒ `D2` is a **completeness/scope defect**, not a later sizing discovery. The
> production change at `8efdfdb3` is **sound and stays** — it correctly closes
> the exact `D1` A witness at the exact `resume_active_continuation` seat. What
> was false is its record's claim that *"position A closes"* and that *"both
> lanes now agree on position A."*
>
> **Do not revert the `D2` mechanism.** This node completes it.

## What it is

**Row 6, `d8d`**, the composed binding-site fixture. It enumerates exactly
`{MatchScrutineeRecursor}` — it was never in the `LexicalCallArgumentRecursor`
population, and B-only exclusion is not merely weak for it but **inapplicable**:
the hook's own `debug_assert` refuses an exclusion of a variant that is not in
the set.

Under **A-only exclusion** at `8efdfdb3` it reaches `FunctionizedUnits` and
refuses:

```
Unsupported(UnsupportedLowering {
  construct: "RecursiveBackedge",
  reason: "protocol machinery is never a source value at a boundary" })
```

## The population is the production predicate; `d8d` is a floor

**`D0` closes the population from the production `MatchScrutineeRecursor`
predicate — not from `d8d`'s spelling.** One fixture is a witness, never a
perimeter, and this node was created precisely because a one-witness result was
read as a class-wide property.

## `D1` — activate and attribute before any repair

**A-only exclusion is the activation seam**, proven at `8efdfdb3`. Use the
existing one-variant hook exactly as designed.

- Reproduce each row's **exact first refusal**.
- The ordinary retained run stays green.
- Record **exact activation denominators**, so a refusal cannot be credited when
  the selector or harness never reached the path.
- **Trace each red to the first missing or mis-consumed static fact**, and
  attribute the owner. A rendered refusal string is a symptom, not a cause.

**Banned:** simultaneous exclusion of both variants, a generalized hook, any
`#[ignore]`, and reinterpreting a retained `RecursiveDescent` run as activation.

## The guard that may not be weakened

**`RecursiveBackedge` remains protocol-only and may never become an accepted
source boundary value.** The lawful repair makes the protocol get **consumed or
represented at its owner before the value boundary**; it does not teach the
downstream guard to accept the forbidden state, and it does not make the marker
boundary-transferable.

Also banned as mechanisms: any fallback to `RecursiveDescent`, `BoundaryUse`,
`PlannedEffectSeat` widening, a lowering-minted token, and invocation-local
activation/resume/return-hole state in ABI data.

## Why it goes FIRST, ahead of [[RT-LEXICAL-RECURSOR-CONSUMERS]]

**It closes the claim the `D2` record correction is in the act of narrowing.**
That correction says the A population is *still open at `d8d`*; this node is
what closes it. Landing B first would leave that open statement standing longer
for no gain.

**Do not fold the two nodes together.** The exact residual producer, activation
hook, observed boundary and completion owner all differ. If the two `D1` causal
partitions later prove one exact shared production root, **route a subsumption
proposal before coding** — Runtime may not infer it from shared retirement
timing or shared syntax. Conversely, **materially distinct authorities are a
hard stop** for either node's provisional size.

## Size

**`M`, provisional**, and the provisional part is real: `d8d` is one measured
expression and `D0` may find the A population materially wider. **Return the
partition before coding** if it does.

## Sequence

1. `10369776252861e8b15e613576256a3682c70066` stays **held evidence only**.
2. **DONE** — the bounded `D2` record correction landed at `89aa1550`.
3. **DONE** — this node's `D0`/`D1` closed the A population at two rows, one
   root; its `D2` repaired `carried_join_arm` at `50808c11`. That repair is
   **correct and lands as an accepted partial**; it is not this node's
   completion, because it advanced the refusal to a sibling authority.
4. **[[RT-CARRIED-CONTINUATION-RESUME]] releases and merges** — inserted here
   2026-08-08. It gates this node's `AC-1`.
5. **This node completes and merges.**
6. [[RT-LEXICAL-RECURSOR-CONSUMERS]] releases and merges.
7. [[RT-RECURSOR-TRANSPORT]] `D3` resumes from the resulting `main`, reapplies
   the retirement and the `AC-2b` dispositions, and proves all six old-green
   rows green **without exclusion**.

Both successor nodes block `D3`. [[RT-DESCENT-RETIRE]] remains downstream.

## The edges, and the one that is deliberately absent

`depends_on: [RT-CARRIED-CONTINUATION-RESUME, RT-CARRIED-ORDINARY-COMPOSITION]`
— **two successors, added 2026-08-08 in that order** as each was measured.

The first was added on the Architect's sibling-authority ruling
`evt_2pt95vbja6447`. This node's `D2` repair at `carried_join_arm` is built and
correct, and it advanced both A rows to a **new** owner,
`lower_computational_match_value_composed`.

The second was added on the Architect's fourth-wall ruling `evt_63ae56tttz9pq`.
[[RT-CARRIED-CONTINUATION-RESUME]]'s `D2` was **also** correct and advanced both
rows **again**, to the `Carried x Ordinary` pre-delegation guard family left
unported by [[RT-PRODUCER-MATCH-PORT]].

⇒ **`AC-1` cannot close here until both successors land.** Each addition
followed a repair that worked, not one that failed — the refusal is walking
outward through a fail-closed chain, and this node's `AC-1` is the thing that
closes when it runs out of walls. **Expect the possibility of a third
successor**; if one appears, it joins this list rather than reopening anything
above it.

**[[RT-RECURSOR-TRANSPORT]] is still not in `depends_on`, and that is
deliberate.** This node's base is post-`D2`-correction `main`, and that
correction is a *partial* merge of that node, not its completion. Naming it
would be a **cycle**, since its `D3` is blocked on this one. The base is stated
here and in the frame; the machine-checked edge is
`blocks: [RT-RECURSOR-TRANSPORT]`.
