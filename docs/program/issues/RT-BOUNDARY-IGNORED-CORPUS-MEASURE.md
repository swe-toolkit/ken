---
id: RT-BOUNDARY-IGNORED-CORPUS-MEASURE
title: "Read unit_boundary_environment_fields on the six ignored closure-at-boundary tests, the population the merged measurement's own selection rule excluded"
status: ready
owner: runtime
size: S
gate: none
depends_on: [RT-SYNTHESIZED-ENV-RECORD-OCCURRENCE]
blocks: []
github: null
origin: "Steward, 2026-08-15, on Adversary hunt evt_71wmpee00vt3j against the merged range de551a4dd..4eec77390 (PR #2352, squash a1c064d5f). The six #[ignore] attributes and the 33-attribute total were verified against the tree by the Steward before filing. Steward-filed per COORDINATION section 2."
---

## The defect, and it is in an argument rather than in code

`RT-SYNTHESIZED-ENV-RECORD-OCCURRENCE` merged on a two-part argument. **The
first part stands on its own and is not reopened here:** the substitution fails
closed on every absence, the producer-class gate refuses outright when a child
has no producer occurrence, and the `continue` skips only the source-only
per-position lookup with the generic ownership record checked before it. That
was traced independently by the Adversary rather than taken from the comment.

**The second part is the one carrying the weight.**
`boundary_transfer_admissibility`'s closure refusal is defeated by construction
for the substituted case — `unit_boundary_environment_record` runs in
`carry_call_input`'s `Specialized` arm, before `transfer_into_carrier`, and
replaces a `Closure`'s environment with a `Record`, which is admissible where a
`Closure` is not. **"Defeats a refusal" is tolerable only if nothing reaches
it**, so the emptiness claim is what licenses the merge.

**The corpus that produced "empty" excludes the population by its own selection
rule.** It is the **non-ignored** `ken-cli --tests` paths.
`crates/ken-cli/tests` carries 33 `#[ignore]` attributes; six read verbatim:

```
#[ignore = "RT-CLOSURE-BOUNDARY-LANE: a runtime-local closure has no
            durable lane across the boundary; fails at base 21fd46dc"]
```

in `px8l_recursive_decl_native.rs` (two), `rt_escape_second_resource_native.rs`
(two), `px8ta_oriented_subcontinuation.rs`, and `rt_parity_native.rs`.

⇒ **The sample is "programs that currently compile," and the mechanism under
measurement exists to change what compiles.** The exclusion criterion is the
negation of the thing being measured, so the empty set is **guaranteed by
construction, not observed.**

> **What this node does NOT claim, stated first because it is the easy misread.**
> It does **not** claim the six would show a non-empty set. Whether any of them
> produces a directly carried empty lexical environment with a planner-issued
> synthesized record is exactly what a measurement would have to determine, and
> **cannot**, because they sit outside the one that was run. The claim is
> narrower and harder to dispute: **the corpus is structurally incapable of
> answering the question it was used to answer.**

## Fixed inputs, measured at `origin/main` `a1c064d5f`

- `crates/ken-cli/tests`: **33 `#[ignore]` attributes total**, of which **six**
  carry the `RT-CLOSURE-BOUNDARY-LANE` reason string above.
- The landed measurement comment sits in
  `crates/ken-runtime/src/cranelift_backend/lowering/core/tests/control.rs`,
  above `required_consumer_route_manufactures_the_depth_two_plus_closure_crossing`.
  It already says "non-ignored" and already says *"This is a scoped corpus
  measurement, not a universal property of Ken programs."* **What it does not
  say is that the excluded set is precisely the population of interest.**
- [[RT-CLOSURE-BOUNDARY-LANE]] is `merged` (PR #2322) and owns those six tests.
  It is not a fold target — this is a measurement its closure did not take.

## Deliverables

**`D0` — run the six with `--ignored` and read the instrument.** For each,
report `unit_boundary_environment_fields`. **They will fail; that is why they
are ignored, and a failing run is not a failed deliverable.** The instrument
reports on completed returns along the way, and those returns are the result.

**`D1` — the disposition, and both answers are real outcomes.**

| `D0` returns | what it means |
|---|---|
| empty on all six | the emptiness claim survives a corpus that can actually contain the shape, and the comment's scope sentence becomes defensible as written |
| non-empty on any | **this converts a scope defect into a live one** — the substitution defeats a refusal on a population that exists, and that is a finding about the merged mechanism, not about the comment |

**`D2` — amend the landed comment to state the exclusion, whichever way `D0`
goes.** One clause: the corpus excludes the closure-at-boundary tests by an
`#[ignore]` whose stated reason is that condition. **A reader takes
"non-ignored" as incidental scoping**, and this is the sentence that stops the
seam being cited as cleared.

> **`D2` is worth landing even if `D0` cannot be run.** The Adversary's own
> recommendation was that stating the exclusion is *cheaper and strictly better*
> than re-measuring. If `D0` turns out to be expensive or the harness cannot be
> driven that way, **land `D2` alone and report why** — do not hold the
> truthfulness repair behind the measurement.

## Acceptance criteria

**`AC-1`.** `D0` names each of the six tests and its individual result. **A
single aggregate line does not discharge it** — the interesting case is one test
disagreeing with five, and an aggregate hides exactly that.

**`AC-2`.** If a test cannot be driven to a completed return at all, **say so as
a result** rather than dropping it from the table. A silently shortened
population is the defect this node exists to correct, and reproducing it here
would be the whole failure repeated one level up.

**`AC-3`.** `D2`'s clause is checked against what a reader would conclude, not
against what is literally true. The existing sentence is already literally true
and still misleads.

**`AC-4`.** No production logic change. This node measures and documents. **If
`D0` comes back non-empty, that is a handback to the Steward, not a repair to
attempt here** — the repair would be a different node with the population as its
subject.

**`AC-5`.** No-regression, in CI (`COORDINATION §12`).

## Banned scope

- **Un-ignoring the six tests.** They fail for a reason
  [[RT-CLOSURE-BOUNDARY-LANE]] recorded; this node reads them, it does not
  repair them.
- **Repairing the substitution.** See `AC-4`.
- **Re-litigating the merge.** `a1c064d5f` stands. This is a measurement the
  merge argument needed and did not have.

## Sequencing

**Not on the `RecursiveDescent` retirement critical path**, and it must not
displace [[RT-RECURSOR-TRANSPORT]] `D0`-`D2`, which is the runtime ring's
released work. Size `S`. Take it at the next seam in that node, or when the ring
would otherwise be idle.

## Provenance

Adversary hunt `evt_71wmpee00vt3j`, read-only at `9620bf9f4`, on the merged
range `de551a4dd..4eec77390`. **The Steward verified the six `#[ignore]`
attributes and the 33-attribute total against the tree before filing**, per the
standing rule that a report's claims are confirmed at the point of use.

**The Steward's own overread is recorded rather than quietly dropped.** The
merge notification said *"the source-reachable population is measured empty,
which is why it merged"* — the landed comment is narrower than that and the
error was in the broadcast, not the artifact. Corrected at `evt_5hremk2yx49kc`.
