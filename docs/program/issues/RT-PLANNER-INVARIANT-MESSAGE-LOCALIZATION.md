---
id: RT-PLANNER-INVARIANT-MESSAGE-LOCALIZATION
title: "The PlannerInvariant rendering localizes every failure to the static transition planner, and 16 of its direct producers are resident in lowering -- the same false-localization defect this file already ruled on for its neighbour variant"
status: draft
owner: runtime
size: S
gate: none
depends_on: []
blocks: []
github: null
origin: "Steward, 2026-08-16, on Architect ruling evt_2n4d1pheyw3se. That ruling REJECTED the Adversary's finding against RT-ROOT-AUTHORITY-BLAME-DOMAIN (evt_3x1r2e6b0fzh5) and, in doing so, named a different and real defect in the same string that neither the Adversary nor the Steward had identified. Every coordinate below re-verified by the Steward against origin/main c8fa12c9b; two of the Architect's were corrected in the process."
---

> # QUEUED. THIS IS NOT A THIRD LANE AND DOES NOT GET A RING.
>
> Operator priority, 2026-08-15: lane 1 is `RecursiveDescent` retirement, lane 2
> is the z3 round-trip and the FO Kripke embedding. **This is filed so it is not
> lost, not so it is started.** Runtime has three active lane-1 nodes and stays
> on them.
>
> **It is also not a recut of [[RT-ROOT-AUTHORITY-BLAME-DOMAIN]].** That node is
> correct, merged, and its reclassification was upheld on the merits.

## The defect

`BackendFailure::PlannerInvariant` renders, at
`crates/ken-runtime/src/cranelift_backend/surface.rs:261-264`:

> *"**native static transition planner** invariant failed; please report this
> compiler bug: {msg}"*

**16 direct producers are resident in lowering** — `lowering/core.rs` 13,
`lowering/mod.rs` 3, the last three being the guards
[[RT-ROOT-AUTHORITY-BLAME-DOMAIN]] moved. **For those, the message names a
subsystem the failure did not occur in.** A user meeting *"native static
transition planner invariant failed ... terminal answer has no affine
checked-root authority"* is pointed at the planner for a lowering-resident
guard.

**This node inherited the defect; it did not create it.** The string has been
wrong for the 13 `core.rs` producers since before that node existed, which is
also why reviewing its diff could not have caught it.

## The house rule is already written, one variant below, in this same file

`NativeResultDecode`'s doc at `surface.rs:192-207` records the identical
failure and its resolution:

> *"It said `is not in the result table` and was measured on `nc22` reporting a
> result-table miss for a token that never reached the `Table` arm and against a
> table that was empty. ... **Saying less is the correction — the previous
> wording localized it wrongly, which is worse than not localizing it at all.**"*

⇒ **`PlannerInvariant` breaches a rule this file wrote about its own neighbour.**
Eight sites, false at seven, corrected by saying less — the same shape, the same
file, and the ruling already made.

**And `PlannerInvariant` carries no doc comment at all** (`surface.rs:191`), so
the causal criterion justifying the whole message lives only at one call site in
another file: `planning/static_transition.rs:12753`, *"ambiguity here is a
compiler bug rather than a program the backend cannot handle."*

## Two corrections to the coordinates as first ruled

**The path is `planning/static_transition.rs`, not
`cranelift_backend/static_transition.rs`.** The line number `12753` is exact;
the directory is not. Verified by `git ls-tree`.

> ### THE STRING IS DUPLICATED, AND THE SECOND COPY IS THE PIN
>
> The Architect asked for a pin, noting the file's own doc records six of seven
> format strings were unobservable before its coverage tests were added.
> **The pin already exists** — `surface.rs:326-329`, inside the rendering
> coverage test, carries the message a second time as an `expected` literal.
>
> ⇒ **A repair must change BOTH copies**, and a candidate that changes only the
> `Display` arm goes red. That is the pin working.
>
> ⚠ **But do not mistake what it pins.** It asserts that the rendering equals a
> **hardcoded duplicate of itself**. Change both together and it stays green no
> matter what the new text says. **It is a drift detector between two copies,
> not a check that the message is true** — so it cannot discharge this node's
> `AC-1`, and a candidate that cites it as evidence has cited the wrong thing.

## Deliverables

**`D0` — narrow the message.** Drop `static transition` and, on the Architect's
reading, arguably `planner`. **Keep the causal clause and the instruction to
report a compiler bug** — those were ruled licensed (see acceptance below).
Change both copies. `surface.rs` only.

**`D1` — give `PlannerInvariant` a doc comment** stating its membership
criterion, on the model of `NativeResultDecode`'s. The criterion is causal and
currently recorded only at one call site in another file.

## Acceptance criteria

**`AC-1`. The rendered message is true at every producer.** Enumerate them and
attribute each to its subsystem. **The existing coverage test does not
establish this** — see the pin note above.

**`AC-2`. The causal clause and the report-a-bug instruction SURVIVE.**
Architect `evt_2n4d1pheyw3se` ruled the instruction **licensed**:
`PlannerInvariant` is a causal category, not a description of evidence, so the
message renders a category whose membership criterion is itself causal. Its only
precondition is fault, and fault is what the category establishes. **A candidate
that removes the instruction has reopened a settled question and fails this AC.**

**`AC-3`. No guard condition changes, and no producer is reclassified.** This is
a message repair. Moving producers between variants is option (b) below, which
was considered and not recommended.

**`AC-4`.** No-regression, in CI (`COORDINATION §12`). Local validation targeted
only — `-p ken-runtime`, never `--workspace`.

## Banned scope

- **Renaming the variant or moving lowering-resident producers to a new one.**
  The Architect weighed this as option (b) and rejected it: *"the causal
  criterion is what carries the weight and it is genuinely shared."* Narrowing
  the **message** is option (a) and is the recommendation.
- **Reopening the reclassification** in [[RT-ROOT-AUTHORITY-BLAME-DOMAIN]].
  Upheld on the merits; see that node's post-merge section.
- **Repairing the coverage test's duplicated-literal weakness.** Real, adjacent,
  and a separate cut — note it, do not fold it.

## The observation worth keeping either way

**A variant called `PlannerInvariant` with 16 producers outside the planner is a
category whose name has outgrown its contents.** Whichever remedy is taken, that
is the durable finding, and it is the kind that a diff review structurally
cannot surface: every individual producer is correct, and only the census is
wrong.
