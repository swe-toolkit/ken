---
name: a-recorder-witnesses-the-line-it-sits-on-not-the-mechanism
description: A test-only recorder observes that control reached its own line, nothing more. A push after a `?` counts Ok-returns, not calls; a row whose fields are written only on the failure path cannot certify success; an arrival bit at the head of a mechanism partitions mutants by which side of the line their early return sits. Ask which field the SUCCESS path writes, and locate every mutant relative to the observation.
metadata:
  type: feedback
---

# A recorder witnesses the line it sits on, not the mechanism

The fleet's controls lean on `#[cfg(test)]` recorders: a thread-local vector
pushed at some site in production code, read back by a test. Whatever the doc
says the recorder counts, **it counts arrivals at its own line** under the
control flow above that line. Three measured failures of the same kind, then
the review disciplines they produced.

## 1. An "unconditional push" after a `?` counts successes, not calls

**Measured 2026-08-12 on `d5fab77e` (`D2k` `AC-1a`).** A control gate split five
refusal causes into two tiers — *never reached the builder* (arrivals `0`)
versus *reached and resolved nothing* (arrivals `1`, keys `0`) — and its doc
said `d2f_gate_note_arrival` *"is an **unconditional push** — executed once per
builder call with **no predicate in front of it**. So the vector's length counts
arrivals exactly."* The production site:

```rust
let plan = build_static_continuation_fusion_plan(...)?;   // four error exits
#[cfg(test)] d2f_note_production_fusion_plane(plan.len());
#[cfg(test)] d2f_gate_note_arrival(D2fGateArrival { ... });
```

The push has no predicate in front of it and it does have an early return in
front of it. It counts builder calls that returned `Ok`, so `0` means *never
reached* **or** *reached and errored*.

⇒ **"Unconditional" is a claim about the control flow above a statement, and a
reader checks it by looking for an `if`.** Enumerate the other ways a statement
is skipped: `?`, `return`, `let ... else`, `continue`, a diverging `match` arm,
a panic in a prior expression. The `?` reads as straight-line code, which is why
it survives review.

**The label was true anyway, for an unwritten reason.** The tier-3 causes do
refuse before the builder in production because
`validate_oriented_subcontinuation_transport` is called **twice with identical
arguments** — once in the compile path above the builder, once as the builder's
opening statement — and the outer copy propagates first. The property the
assertion depends on is an **ordering between two copies of one operation**,
stated nowhere, while a false mechanism claim is stated in its place; a
reordering, removal or bypass takes the tier's meaning with it while every row
still reads `0`. Check the motive honestly even when it cuts against you: the
outer call's `is_ok()` feeds a census row, but that consumer is
`#[cfg(any(test, feature = ...))]`, so in a non-test build the outer call is
exactly redundant, and a pass stripping test-only machinery reaches this block.
Say both halves ([[preventive-findings-are-unfalsifiable-so-keep-them-cheap]]).

## 2. A row whose fields are written only on the failure path cannot certify success

**Measured 2026-08-17 on `ca639b5ef`.** A port's `AC-3` was accepted on
`entered >= 1 AND route1 == 0`, from a recorder deliberately strengthened to push
a row at **function entry**, because a bare `route1 == 0` had been ruled equally
consistent with the code never being reached. Correct, and still not what the
criterion claims.

**Make the mechanism vacuous, not absent.** Deleting the new branch brings back
the old refusal and reddens everything, which measures nothing. Leave the
branch and empty its body:

| | entered | bad-route bit | the AC | the test |
|---|---|---|---|---|
| landed | 1 | 0 | satisfied | pass |
| branch walks nothing | 1 | 0 | **satisfied** | FAIL |

⇒ **The criterion is satisfied by a no-op.** What caught the mutation was a
different assertion — an error-string check the recorded criterion never
mentions. The gate that discriminates and the gate that was written down were
not the same gate. Kin of
[[an-acceptance-criterion-must-name-an-observation-the-failing-configuration-does-not-also-produce]].

**The structural tell:** the row's four descriptive fields are filled in at the
bad-route site and left `false`/`0` by the entry push, so on the advancing run
the row is all zeros. `entered` counts arrivals, `route1` denies one exit, and
nothing is evidence the new code did work. A two-clause AC over one negative bit
reads as covering presence and absence.

⇒ **Ask of any observer: which field does the SUCCESS path write?** If none, it
is a negative-only instrument, and an AC phrased as "advanced" rather than "did
not refuse" over-claims. The remedy is one line and belongs to the owning ring:
a bit the new branch sets, or a count it records.

**The same change can silently redefine the observer's unit.** Before: one row
per bad-route return, so `rows.len()` was the bad-route count. After: one row
per entry, tagged. The number stayed `1` and meant something different on each
side, while three doc sites, including the witness file's own `MEASURED:` line,
still described the old unit. On any observer edit, diff what one row
**means**, not just what the assertion says.

## 3. An arrival bit partitions mutants by line position, not by behaviour

**Measured 2026-08-17 on `b7e2cf8f8`**, the repair for finding 2. It added a
`match_descent` bit set at the head of the branch, and a witness assertion.
Re-running the original mutant one line lower passed the repaired criterion.

| mutant | early return placed | new bit | verdict |
|---|---|---|---|
| the node's own committed hook | **above** the recorder | false | caught |
| the Adversary's mutant | **below** the recorder | **true** | **passes** |

Both produce the **byte-identical compiler error** — proved by instrumenting the
node's own control test to print the output it suppresses and comparing strings.
⇒ The instrument separates two behaviourally identical programs on the position
of one line: **the recorder's line is the blind-region boundary.**

**A hook installed above the recorder cannot fail its own test.** The repair
shipped a permanent test-only kill switch — read, then `return`, before the
record call — and cited the test over it as mutation evidence. That switch
ablates the *recording*, so any bit written at that site is false under it: a
positive control that the recorder fires when reached, and nothing about the
mechanism. An ablation cannot test a conjunct the ablation itself forces.
⇒ **When a node commits its own mutant as evidence, locate the mutant relative
to the observation, not relative to the mechanism.** If nothing between them
runs, the demonstration is circular.

**Rank mutants by the first assertion that fails, not red versus green.** The
mutant the new bit does catch (mechanism removed, falls to the old route) fails
on a pre-existing assertion several lines earlier, so the new bit is never
evaluated. Net new discriminating power: one mutant, the node's own hook. A
repair that reddens is not a repair that discriminates.

**"Record work, not arrival" is necessary and not sufficient** — a counter
bumped inside the loop body. Check where the mechanism exits first: on this
witness the loop leaves through `?` on the first arm, so a counter after the
loop would be equally blind. **The remedy landed on `8f09b122b` and still did
not discriminate:** the counter went in one statement below the old recorder,
the same early return one statement lower passed it again, and ablating the new
assertion showed a pre-existing behavioural assertion had been catching the
mutant all along. ⇒ The fact has to sit **below the work the deliverable is
about**, not one statement below where the last fact sat. Two-run technique:
[[measure-what-each-assertion-adds-by-removing-only-it]].

**The printed evidence line is a separate artifact from the assertion.** The
witness asserted the new bit and printed the old pair unchanged, so every
handback quoted the pair that had been ruled insufficient. On any observer edit,
diff the `eprintln!` as well as the `assert!`
([[the-control-already-prints-the-evidence-nobody-quoted]]).

## Review disciplines from the same arc

- **A correction can add the interpretation without adding the evidence.** An
  earlier draft of the `d5fab77e` gate asserted a uniform *"must fail to
  resolve"*; the repair split the tiers correctly and upgraded the message to
  *"never reach it **AT ALL**"* plus a paragraph on what an arrivals-zero row
  says — over the same three counts. When a correction sharpens what a
  measurement MEANS, ask what it added on the evidence side
  ([[the-operative-artifact-must-carry-the-claim-whichever-pass-wrote-it]]).
- **Strength should track what the row has left.** Two landed siblings state the
  standard (*"a count alone could not tell 'the validator refused' from 'the
  compile failed earlier for an unrelated reason'"*;
  *"the refusals are attributed, not merely counted"*). The gate's tier-3 rows
  take `compile_cause(cause, ...).0` — `.0` discards the error the helper
  returns — while the sentence-asserting helper is used on the positives,
  already anchored by `keys.len() == 1`. The tier-3 rows cannot have a positive
  comparator, so the sentence was their sole discriminator and the one place it
  was omitted. Ask of any control mixing strengths: which row has no other
  anchor? (Second increment where the standard needed no argument;
  [[the-unlawed-link-is-the-join-between-two-maps]].)
- **Report the attacks that died, and rank the survivor against them.** Refuted:
  a forced zero from `any()`/`.sum()` over a possibly-empty vector (the two
  recorders are adjacent and unconditional *relative to each other*, so a plane
  vector of length one entails one arrival); cross-test contamination (both
  `thread_local!`, drained before and taken after on one thread); live
  misclassification (the outer validator propagates with `?` above the
  builder). The identical "two things at one seat" argument is sound for the
  plane/arrival pair and unsound for the tiers: **measure the distance, do not
  classify the shape.** A report listing only the survivor cannot be told from
  a lucky first draw
  ([[a-confirming-first-instance-is-when-the-sample-size-matters-most]]).
- **"Independent of X" and "asserts nothing about X" are the same sentence.**
  The repair `D2k-1d` (`1f578a70`) corrected the doc (the length counts
  `Ok`-returns) and replaced the tier-3 discriminator with each cause's refusal
  sentence, *"identical under either copy, which makes these rows independent of
  the ordering without legislating it."* True, and it is the admission: both
  worlds produce arrivals `0` and a byte-identical construct and reason, so the
  row passes either way. When a repair reports robustness to a variable, ask
  whether it measured the variable or dropped the claim that mentioned it.
- **A correction's blast radius is every sentence the corrected reason was
  supporting.** Three unchanged sentences still stated the phase, including a
  table column headed `phase reached` and *"The phase is asserted per row."* —
  weakly true before the repair and false after it. Grep the doc for the
  conclusion, not the premise. Credit where the qualifier did land: the new
  assertion message says *"without ever reaching the builder's **Ok return**"*,
  exactly right, so the operative text was repaired and the header block was
  not — the pass asymmetry running the other way from usual.
- **Name the property, offer both closures, prescribe neither**: wire the
  already-recorded discriminator (`with_match_recursor_census` exposes
  `validator_admitted`, fed from the outer call, unused in the test file), or
  delete the three phase claims
  ([[surface-the-seam-need-not-your-preferred-mechanism]]).
- **A flagged risk confirmed absent, carrying its reason, retires the
  question.** Both risks the Steward flagged were absent: the three reason
  substrings are each raised at exactly one production site, the two
  near-twins differ at `slot`/`call` so no message holds both, `construct` is
  compared by equality, and the requested disclosure covers rows 1-3, the whole
  synthetic set (row 0 is the positive control). "I checked and it's fine"
  retires nothing.

The same recorder shape also makes a refusal-pin's inversion unreadable when the
port lands:
[[a-test-pinning-a-missing-port-refusal-inverts-its-signal-when-the-port-lands]].
