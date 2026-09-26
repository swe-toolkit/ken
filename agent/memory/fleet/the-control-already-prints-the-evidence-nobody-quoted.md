---
name: the-control-already-prints-the-evidence-nobody-quoted
description: Asked whether a cross-compile equality was weaker than it read, the answer came from running the control with `--nocapture` and reading its own raw dump — the equalities are genuinely cross-program, and the same three lines show one carry observation per compile, so a law claimed over every level is verified at one
metadata:
  type: feedback
---

# The control already prints the evidence nobody quoted

**Measured 2026-08-14 on `b0f9c2ff`, on the one thing the Steward said neither
the Architect nor QA would have caught, *"since both reasoned about it in the
same terms."***

The control pins a descent law by **cross-compile** equalities —
`depth_2.required == depth_1.unit_consumer` — rather than fixture literals. It
also `eprintln!`s its whole raw observation set, and nobody had quoted it.

```
depth-1  unit=(16,5)   (false, 21, 31, 16, 5)   (true, 21, 29, 16, 5)
depth-2  unit=(26,21)  (false, 31, 41, 16, 5)   (true, 31, 39, 26, 21)
depth-3  unit=(36,31)  (false, 41, 51, 26, 21)  (true, 41, 49, 36, 31)
```

⇒ **The equalities survive the attack**: `depth_2.required` really is a value
from a *different program's* compile, with no shared derivation in the run.

⇒ **And the same three lines show two observations per compile — one producer-use,
one child-push. Not one per level.** So a law stated over every `N` is verified at
**one** `N` per fixture, at the outermost boundary; a depth-3 program's inner
boundary is never observed in its own compile, only inferred from a separate
depth-2 one.

⇒ ***When a reviewer asks whether a construction is weaker than it reads, run it
with `--nocapture` first.*** A control built to be convincing usually prints its
operands, and **the dump answers a question the assertions cannot: how many
observations exist, at which sites, per run.** Reading the assertions tells you
what is compared; reading the dump tells you what the population is.

## THE TEST'S OWN SHAPE CAN PROVE THE POPULATION

`required` is collected into a `BTreeSet` and destructured as `[required]`. **If
two levels emitted producer-use carries with correct — therefore different —
values, the set would hold two and the destructure would panic.**

⇒ ***The test passing is itself the evidence that only one level is observed.***
That is a stronger argument than the dump, because it holds for every future run
rather than the one that was watched. **Look for an assertion whose success
constrains the population, not just the value** — a `let [x] = set.as_slice()`
is one, and it is usually written for tidiness rather than as a claim.

## "NOT FIXTURE LITERALS" DOES NOT IMPLY "ROBUST TO RENUMBERING"

The doc says the equalities are between independently produced records, *"so
source-origin renumbering cannot require re-recording the test."* **The first
clause is true and is the control's real strength; the conclusion does not
follow.** The equalities hold because wrapping the fixture leaves the inner
level's numbering unchanged — a property of the **generator**, visible in the
data as a uniform offset per level.

⇒ **Robust to renumbering the test does not contain; not robust to renumbering
in the generator.** A generator change reds a *correct* carry.

⇒ ***That is the safe direction and still worth fixing, because it invites the
wrong repair***: a future author reads *"renumbering cannot require
re-recording"*, hunts the carry, finds nothing, and hardcodes the values — which
destroys the property the sentence was praising. **Name the generator premise in
the clause that claims the robustness.**

## SAY WHICH ATTACKS FAILED

The `is_child_push` partition is not a trust hole — a mis-set flag routes a
child-push value into the producer-use set and reds the first assertion. The
`BTreeSet` dedup hides no multiplicity, for the reason above.

⇒ **Two named non-findings cost two sentences and tell the reader which
approaches are already spent.** On a control this dense, the next pass otherwise
re-derives them.
