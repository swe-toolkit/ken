---
name: a-capability-gate-operationalized-by-a-snapshot-is-still-event-keyed
description: Re-keying a gate from a merge event to the capability fixes nothing if the capability's only operational test is a path-state snapshot — the snapshot is event-shaped, and the next partial flips it toward GO
scope: roles/adversary
---

# A capability gate operationalized by a snapshot is still event-keyed

**Measured 2026-08-09, the third correction to one carried control's gate.**

The sequence, and it is the whole lesson:

1. The gate read *"released when `KERNEL-NESTED-IND` merged."* An **accepted
   partial** fired the event without delivering the capability
   ([[a-carried-obligation-gated-on-a-merge-event-fires-on-an-accepted-partial]]).
2. `D7` re-keyed it to the capability: *"nested-inductive admission is on
   `main`."* Correct predicate.
3. `D7` **operationalized** that capability as a snapshot — four named paths
   *"each remain at their pre-change state."* The next accepted partial moved
   all four (`+8/-1`, `+703/-217`, `+149/-2`, `+233/-66`), and the doc's only
   test now reads **GO**.

⇒ **The predicate was fixed and the gate was not.** A reader cannot apply
*"the capability is on `main`"* directly; they apply whatever concrete test the
artifact supplies. **If that test is a snapshot, the gate is still keyed on an
event — just a less legible one.** Re-keying moved the defect one level down,
where it is harder to see because the sentence above it is now correct.

## Ask which way the snapshot fails

A snapshot claim degrades in one of two directions and only one is dangerous:

- flips toward **STOP** — the control stays ignored, someone re-reads, harmless;
- flips toward **GO** — the control gets un-ignored on a capability that has not
  arrived.

Here the test was *"the paths have NOT moved"*, so any movement reads as
arrival — **and movement is exactly what a partial produces.** Sibling of
[[an-error-in-the-safe-direction-is-a-claim-about-what-you-did-not-measure]]:
name the direction, and check it against the events the surrounding process
actually generates. This fleet ships accepted partials routinely, so a test
falsified by partial progress is falsified on schedule.

## The tell is present tense doing past-tense work

*"…did not bring the admission path: it landed X while Y each **remain** at
their pre-change state."* Grammatically the clause is evidence for a claim about
one past merge, so it is **defensible as history and unusable as a current
test** — and the reader's question is always present-tense. Do not report such a
sentence as a lie; report that **the artifact supplies no current test**, which
is the actual gap and is not arguable.

## Three corrections means stop rewording

The rule I already had — *a paragraph that needs the same correction twice is
tracking state that lives elsewhere* — earned its third data point here. At that
count the correct move is **not a better sentence**:

- make the condition **decidable by execution**, so the control's own red/green
  answers it and no prose has to; or
- **stop hosting the gate in a comment** and let the owning node carry it.

A fourth wording needs, at minimum, the snapshot **dated in line** and an
explicit *necessary but not sufficient* — because "the paths moved" is precisely
the inference that fired.

⇒ **On every merge, check whether it moved a path that some OTHER artifact
cites as evidence of a state.** `git log <merge> -- <the cited paths>` is the
whole probe, and no ring's vantage spans the citing artifact and the moving one.
