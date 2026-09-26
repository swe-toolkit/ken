# Authoring a short product frame

Use this only when an authorized lane needs its next product WP. Governing
playbook: `../steward.md`. Release mechanics:
`release-and-handoff.md`.

## Default frame

A normal frame contains six sections and stays under about 120 lines.

1. **Objective.** One observable product change.
2. **Fixed inputs.** The controlling spec, operator or Architect ruling, and the
   base SHA at which current-code facts were measured.
3. **Scope.** Files or components expected to change, plus the one or two
   boundaries that must not move.
4. **Deliverable.** One implementable result. Investigation needed to build it
   is its first step, not a separate node.
5. **Acceptance.** Two or three controls that can fail on a wrong
   implementation.
6. **Stop condition.** The concrete observation that requires a handback rather
   than a workaround or scope expansion.

## Perishable anchors

Include this sentence once:

> Treat anchors as perishable. If a fixed input is false on the landed base,
> stop and report the mismatch; do not build around it.

Do not copy the spec, component design, review protocol, federation law, or
incident history into the frame. Cite the source.

## Deliverable test

The deliverable must change `crates/`, `catalog/`, `spec/`, or `conformance/`,
or directly enable a named change in one of those paths.

Do not create a WP whose output is only:

- a report, census, inventory, attribution, label, or status correction;
- a workflow, tracker, detector, diary, or playbook change;
- proof that an already-owned item has an owner;
- wording cleanup or a list of possible future repairs.

When a repair needs discovery, state the best current repair hypothesis and ask
the ring to try it. The attempt may hard-stop with evidence. Split discovery
from repair only when the attempt would be destructive, unbounded, or an order
of magnitude larger under one outcome.

## Acceptance criteria

Each criterion names a property and the observation that distinguishes a good
implementation from a bad one. Prefer one positive control and one
property-specific negative or differential control.

Do not use:

- counts where identity or pairing is the property;
- absence of an error without proof the command ran;
- a detector derived from the implementation it checks;
- repository-text assertions as a substitute for product behavior;
- a control that the failing configuration also passes;
- an exhaustive checklist when the population has no proved closure.

Load `pin-a-property` only when the WP actually introduces a mechanical pin.
Load `mutation-prove-a-pin` only when the frame requires a mutation proof. Those
specialized skills replace, rather than supplement, a long generic audit here.

## Conditional checks

Run an extra check only when its trigger is present:

| Trigger | Extra question |
|---|---|
| New or changed ABI/seam | Can every required value cross the full seam? |
| Concrete path becomes generic | Does the trait expose every operation used? |
| Opaque primitive involved | Can the promised value or proof be constructed? |
| Corpus-wide claim | What producer closes the population? |
| Trust or proof-status change | Which exact trusted-base entry or verdict moves? |
| Multi-piece atomic change | Can every piece be assembled on one branch? |

If none of these triggers is present, do not run the checks. A rare past failure
is not a reason to tax every future frame.

## Revisions

Edit the operative section. Delete text invalidated by a ruling. Never append a
correction or prior-state block beside the old instruction. Git and the WP
thread preserve the prior state.

A revision that changes acceptance criteria gets the review required by
`COORDINATION.md §14a`. A deliverable or evidence update that leaves the
criteria unchanged does not acquire an extra review merely because it touches a
frame.

## Release checklist

Before marking the frame ready, verify:

1. the lane is authorized and the dependency is on `origin/main`;
2. the objective is a product change, not an artifact about work;
3. there is one deliverable and no deferred cleanup tail;
4. there are at most three normal acceptance criteria;
5. every criterion can fail on a plausible wrong implementation;
6. current-code facts name the measured SHA;
7. stale or superseded prose was deleted rather than annotated;
8. every `agent/memory/CHECKS.md` check the frame's settled inputs trigger was
   run, and its measured result is on the frame's `Checks:` line (a consumer list is grepped across
   every root, an assumed enabler is written once, an exempted axis is
   measured).

Then release. Do not request a separate review of the frame unless an existing
rule requires it.