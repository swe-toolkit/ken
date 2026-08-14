---
id: LANG-PRELUDE-COMBINATOR-BLOCK-DELTA
title: "`AC-6`'s doc says a live differential is impossible because `ElabEnv::new()` has no 'before' env -- true of an ENV-level bracket and false of a BLOCK-level one, which is the established idiom four sites away in the same crate, so the instrument the comment says does not exist is writable, id-keyed, and fails in production"
status: merged
owner: language
size: S
gate: none
depends_on: []
blocks: []
github: 2189
origin: "Adversary hunt evt_6e1n4b6xq347y on the landed squash 225876a4, second finding, with a compiled-and-run probe that was reverted. The refuted claim is one the Adversary itself confirmed two nodes ago, and it is in landed test doc text. Every load-bearing fact below re-measured by the Steward against main 225876a4 before filing."
---

## The defect

`crates/ken-elaborator/tests/lang_prelude_collections.rs:236-238` carries, in
landed doc text:

> *"not a count and not a differential (a live differential is separately
> impossible here: the four combinators are registered inside `ElabEnv::new()`
> itself, so there is no "before" env to diff against)"*

**That is exact about an env-level differential and false about a block-level
one.** The four combinators are four consecutive `elaborate_decl` calls at
`src/prelude.rs:452-471`. A bracket around *those four lines* has a "before"
even though the environment does not.

**Measured by the Adversary, not argued.** They bracketed exactly those four
lines with the established idiom, compiled it, ran it, and got
`block-level delta over the four combinators = 0 entries`, then reverted. The
Steward re-verified the surrounding facts at `225876a4`.

**The idiom is not novel — it is four sites away in the same crate**, id-keyed
and returning `Err(ElabError::Internal)` from production:

```
crates/ken-elaborator/src/bytes.rs:223 / :294
crates/ken-elaborator/src/conversions.rs:173 / :201
crates/ken-elaborator/src/conversions.rs:303 / :364
crates/ken-elaborator/src/numbers.rs:351 / :356
```

⇒ **"No instrument exists" was "no instrument at the granularity I looked at."**
Nobody asked whether the thing bracketed had to be the whole env.

## Why this is worth a node rather than a comment fix

**Because the comment is load-bearing in the direction that prevents the
repair.** It does not merely describe the current control; it tells the next
reader that a better one is *impossible*. A false impossibility claim is the
one kind of stale comment that stops work rather than merely misinforming — and
this arc has now hit that shape three times.

The instrument it forecloses is strictly better on four axes: **id-keyed** (no
label collisions), **no 107-entry list to maintain** when unrelated prelude
entries are added, **fails in production** rather than only under test, and
**scoped to the property being claimed** rather than to the whole trusted base.

## The design call, front-loaded — this does NOT replace `AC-6`

**Taken by the Steward. It is a test-topology call; the Architect reviews it on
the merge Decision.**

**`AC-6`'s 107-entry enumeration STAYS and must not be deleted or shrunk.** It
and the block delta cover **different populations** and it is easy to read them
as redundant:

| | population | keyed on |
|---|---|---|
| `AC-6` enumeration | the **whole** trusted base, 107 entries | labels |
| block delta | **only** the four combinators' contribution | `GlobalId` |

`AC-6` also carries a value the block delta does not: its own doc calls it
*"large enough to be a finding about the shape of the trusted base in its own
right"* — it is a **census**, not only a control. **Deleting a census because a
control arrived is the failure this arc keeps retiring in the other direction.**

**What the block delta does replace is the per-name reasoning about the four
combinators specifically** — that is the claim it proves directly.

## Deliverables

**`D1` — correct the false claim.** Rewrite the parenthetical at
`lang_prelude_collections.rs:236-238`. **State what is actually true**: an
*env-level* differential is impossible because `ElabEnv::new()` has no before;
a *block-level* bracket is the established idiom and is what `D2` adds. **Do
not simply delete the sentence** — the next reader needs the distinction, which
is the whole content of this node.

**`D2` — the block-level delta over the four combinators**, at
`src/prelude.rs:452-471`, following the existing idiom: `BTreeSet` of
`trusted_base()` before and after, difference, compared against the expected
set, returning `Err(ElabError::Internal)` on mismatch with a message naming the
combinators. **Match the shape at `conversions.rs:303/364`** rather than
inventing a variant.

**`D3` — record why `AC-6` survives**, in one paragraph at the enumeration, per
the design call above. A future reader who sees an id-keyed delta land next to a
107-entry label list will otherwise reasonably try to retire the list.

## Acceptance criteria

**`AC-1` — `D2` is exercised under a mutation, with the failing text reported.**
Add a fifth `elaborate_decl` to the bracketed block that *does* touch the
trusted base (a postulate is the obvious choice), confirm `D2` **reds** naming
the unexpected id, and restore. **A delta assertion that has never seen a
non-zero delta is not known to be a control** — that is precisely the shape this
arc has retired twice, and this node must not add a third instance while closing
one.

**`AC-2` — `D2` fires in production, not only under test.** Show it running via
an ordinary `ElabEnv::new()` path rather than a `#[cfg(test)]` one. Report how
you established that.

**`AC-3` — the expected delta is stated independently, not read back from the
env.** The expected set is *"the four combinators contribute nothing to the
trusted base"* — write that, do not compute it from `trusted_base()` and compare
it to itself. **An assertion whose expected value is computed from the thing
under test is a theorem, not a check**, which is the lesson this same file
produced two nodes ago.

**`AC-4` — `AC-6`'s enumeration is unchanged and still 107.** Membership, count
and kind tags as they landed at `225876a4`. **If the count moves, stop and
report it** rather than updating the list.

**`AC-5` — `D1`'s replacement text distinguishes the two granularities.** Grep
the file for `impossible` and report what the text now says. A correction that
leaves a reader thinking no differential is available has not discharged `D1`.

**`AC-6` — no-regression, in CI.** `COORDINATION §12` — the venue is CI, never a
local `--workspace` run.

## Sizing

**`S`.** `D2` is a transcription of an idiom that exists four times in the same
crate, and the Adversary has already run it. `D1` plus `D2` with its mutation is
a releasable increment; `D3` is a paragraph.

**If `D2` turns out to need a production shape the existing idiom does not
cover — in particular if it would change what the trusted base MEANS rather than
assert a property of it — stop and report.** That is a TCB question and it is
not this node's to answer.

## Not this node

- **No change to `AC-6`'s membership, count, or kind tags.** See the design
  call.
- **No new trusted-base entry, and no change to what `trusted_base()` returns.**
  This node adds an assertion about the existing behaviour.
- **No change to the three `Primitive(<unregistered>)` entries.** Their residue
  is recorded on [[LANG-TRUSTED-BASE-LABEL-KIND-TAG]] and is measured
  unexploitable; it is not a defect to repair here.
- No change to `conversions.rs`, `bytes.rs` or `numbers.rs` — they are the
  model, not the target.

## Contention

**None.** Runtime is on `RT-NESTED-IH-NATIVE-REALIZATION` and
`RT-C2-OBSERVATION-ARTIFACT-IDENTITY`, scoped to `ken-runtime` plus
`tests/nc14_data_match_lowering.rs`. This node touches `src/prelude.rs` and
`tests/lang_prelude_collections.rs`. Intersection empty.
