---
id: RT-4B-UNIQUENESS-GATE-REACH
title: "Count whether any candidate reaches the twelfth of thirteen elimination exits before building anything that classifies what happens there -- a call-site counter at `fusion_unique_static_body_triple`, changing no signature, no control flow and no plan, which decides whether the attribution increment has a subject at all"
status: ready
owner: runtime
size: S
gate: none
depends_on: [RT-4B-OBSERVATION-FEATURE-GATE]
blocks: [RT-4B-UNIQUENESS-GATE-ATTRIBUTION]
github: null
origin: Architect evt_5gck3qg72xe37 ruling the Steward's single attribution node be SPLIT, on the Steward's own sizing caveat (evt_24j7cjr1bhvzr) that exit 12 may never be reached. Reach is pure recording inside the already-authorized 4b observation gate and needs no exception; attribution is a builder change and stays conditional. Split taken by the Steward 2026-08-13.
---

> **RE-POINTED AND RETURNED TO `ready` 2026-08-13, after
> `RT-4B-OBSERVATION-FEATURE-GATE` removed the blocker named at the bottom of
> this note. Read the frame; three of its fixed inputs changed.**
>
> **What the predecessor did and did not deliver.** It made the D2f observation
> reachable from `ken-elaborator` behind a default-off feature, and proved the
> feature inert across two Cargo compilations. **It did not measure C2** — its
> identity control drives a purpose-built `R3_4B_IDENTITY_SOURCE`, and its own
> doc comment says so. So C2's count remains exactly as unmeasured as it was;
> what changed is that it is now *possible* to measure.
>
> **A correction the Steward measured while re-pointing, which the original
> note did not catch:** `fusion_unique_static_body_triple` has **two**
> production call sites, not one. `:10367` in the enumeration loop is the
> elimination; `:8927` in `rederive_fusion_key` turns the same `None` into a
> hard planner error on an already-formed key. A counter inside the function
> body sums two different populations and looks exactly like a clean answer.
>
> **The C2 witness also cannot supply artifact bytes** — it reaches an
> immutable pre-object preparation and emits no native object — so the old
> AC-2 was unsatisfiable on the witness the node exists to measure.
>
> ---
>
> **The original pull-back, retained because its lesson is the reusable part.
> Architect `evt_6hfw027f43cgg`, verified by the Steward.**
>
> **The measurement this node was built on is about the wrong population.**
> `(4, 2, 0, 2, 1)` is asserted on `arrived_empty` in
> `d2f_0_the_applied_root_production_path_gate`, which iterates exactly
> `[D2jCause::ExactSuffix, D2jCause::CallIdentity]` under the test's own comment
> **`// AC-6a phase B: arrived once, resolved nothing`.**
>
> Those are **deliberate perturbations, authored so fusion does NOT form** —
> `ExactSuffix` = *"the selected case body is no longer the IH-consuming
> `Call`"*; `CallIdentity` = *"the consuming `Call` calls the ordinary child
> instead of the hypothesis."* Four candidates walk in and nothing survives
> **because the fixture was perturbed to make nothing survive.**
>
> **The same assertion shows the three unperturbed rows resolve exactly one key
> and one descriptor each. This planner FUSES.** And the artifact-identity
> control drives `d2j_checked_fixture_under(D2jCause::Exact)` — a D2j fixture,
> **not `C2_MIXED_SOURCE`.**
>
> ⇒ **`C2`'s walked count remains UNMEASURED**, exactly where 4b stood before.
> Pointed as written, this node would faithfully count the same comparators and
> the successor would be spent on a fixture built to fail.
>
> **THE FRAME DEFECT, because it is the reusable part:** the C2 requirement was
> written as **prose in D2 and never carried into an AC**. AC-1 to AC-5
> constrained where the count came from, what stayed identical, which channel
> carried it, that a mutation redded it, and that the artifact stated its limit
> — **not one said which witness.** An in-crate D2j implementation satisfies
> every acceptance criterion while answering a different question. **A
> deliverable stated in prose and not carried into an AC is one the frame cannot
> check.**
>
> **AND THE DRIFT WAS NOT CARELESSNESS.** The observation is `#[cfg(test)]`
> inside `ken-runtime`; C2 drives through `ken-elaborator`, which links a build
> where those calls do not exist. **The cross-crate gate expressibility problem
> that 4b opened with is still unsolved and is the actual blocker** — this frame
> asked for something its own fixed inputs made impossible. Re-pointing at C2 is
> a scope question, not an edit.
>
> **(That last paragraph was true when written and is now discharged:
> `RT-4B-OBSERVATION-FEATURE-GATE` solved the cross-crate expressibility
> problem, and the scope question it called for was taken by the Steward
> above. Read it as the record of why this node waited, not as current
> status.)**

## What this is, and why it is its own node

`RT-4B-ENUMERATION-INPUT-SIZE` measured `(4, 2, 0, 2, 1)`: four admitted
discoveries enter `enumerate_live_fusion_candidates` and `keys = []` comes out.
All four were eliminated somewhere among fourteen indistinguishable routes.

`fusion_unique_static_body_triple` (`planning/static_transition.rs:10099`) is
the most informative of them — the only elimination with documented
lawful-refusal semantics. **But it is the twelfth of thirteen exits.** A
candidate reaches it only after surviving eleven earlier ones, and **nothing
measured says the four get that far.**

⇒ **Count the population before building the thing that classifies it.**

## Why it split from the attribution increment

The two halves have different costs and different lawfulness statuses:

| half | what it needs | status |
|---|---|---|
| **reach** — was exit 12 reached, and how often | a **counter at the call site**. No signature, no control flow, no plan change | **pure recording, inside the 4b observation gate already authorized.** No exception needed |
| **attribution** — which arm fired | widening `fusion_unique_static_body_triple`'s return type | a **builder change**, outside the observation gate; the one-function exception, **conditional on reach > 0** |

**If reach is zero, the attribution increment has nothing to attribute** — it
would be a widened return type observing a branch no candidate enters. That is
the "working instrument nothing reaches" artifact, built deliberately.

## Pre-stated licensing — read BEFORE reporting

| outcome | what it licenses |
|---|---|
| **reach = 0** | **Exactly one thing: the eliminations on this witness are upstream of the twelfth exit.** It narrows fourteen routes to eleven. It does **not** attribute any of them, does **not** reopen the fourteen-exit census (that stays out), and does **not** change 4b's status — still exhausted, now with one route excluded rather than zero. |
| **reach > 0** | The attribution increment has a subject. `RT-4B-UNIQUENESS-GATE-ATTRIBUTION` becomes lawful and is built against its five criteria. |

**Neither outcome licenses a finding against the planner.**

## Not this node

- **Widening any function's return type**, including
  `fusion_unique_static_body_triple`. That is the successor and it is
  conditional.
- **Attributing absence versus multiplicity.** This node cannot see the
  difference and must not claim to.
- **The fourteen-exit census.** Ruled out and staying out.
- Enumeration, classifier, checker, marker, fusion-candidate, representation,
  ledger or closure-boundary repair. Gates 5 and 6 held; production unarmed.
