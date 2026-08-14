---
id: RT-CONTKEY-REFUSAL-PROFILE-SPLIT
title: "the consuming-occurrence validator's refusal is split into a `cfg(test)` classifier and a `cfg(not(test))` arm, so no test profile can compile the production refusal string -- and the deeper hazard is that under `cfg(test)` the production arm is ABSENT rather than dead, so dropping one `return` makes the validator silently accept a mismatched occurrence in exactly the profile that does the checking"
status: ready
owner: runtime
size: XS
gate: none
depends_on: [RT-CONTKEY-ELIMINATOR-ORIGIN-UNFIRED]
blocks: []
github: null
origin: "Architect non-blocking should-fix recorded in the RT-CONTKEY-ELIMINATOR-ORIGIN-UNFIRED approval (dec_2043sa5rjcyzb, evt_33r91fek6ghjp), plus two additions the Adversary routed to the Steward at evt_2e245r28s3m6n rather than replying to him. Filed by the Steward because a carry recorded only in an approval verdict and a PR body has no node and evaporates. Addition 2 is a code-shape hazard, not a wording point, and it is the reason this is filed rather than folded."
---

## What this is

`RT-CONTKEY-ELIMINATOR-ORIGIN-UNFIRED` landed a test-gated classifier that
distinguishes an `eliminator_origin` failure from a `body_origin` failure. The
shape, as read on the candidate at `49c2bc38`:

```rust
if rederive_consuming_occurrence(plan, &unit.key, claimed)? != Some(claimed) {
    #[cfg(test)]
    {
        let reason = if forward_match_scrutinee(plan, claimed.eliminator_origin)?
            != unit.key.continuation_origin { "…mismatched eliminator_origin…" }
        else { "…mismatched body_origin…" };
        return Err(planner_error(reason));
    }
    #[cfg(not(test))]
    return Err(planner_error(
        "a continuation specialization's consuming occurrence is not the exact outer \
         selected case body derived from its eliminator",
    ));
}
```

**Two defects, and the second is the one that matters.**

**`H1` -- the production refusal string is unpinned.** It lives in exactly one
arm that no test profile compiles, so no assertion can reach it. Anyone may
reword or break it and nothing reds. This is the Architect's should-fix.

**`H2` -- under `cfg(test)` the production arm is ABSENT, not dead.** If that
inner `return` were ever dropped, the `#[cfg(test)]` block evaluates to `()`,
the `if` body completes, the loop **continues to the next unit**, and the
validator **silently accepts a mismatched consuming occurrence.** It compiles
clean.

⇒ **In production the same edit is harmless** -- the `cfg(not(test))` arm still
returns. **The failure exists only in the test profile, and the tests that would
notice it are the ones built with the broken arm.** `H1` loses *cover* for a
correct behaviour; `H2` loses the *behaviour*, in the profile that does the
checking.

## The remedy, and why the obvious one has a residual

**The Architect's proposal is to hoist the common tail into a shared `const`**
referenced by both arms, so the existing test assertion pins the production
wording transitively.

**That works while both arms reference it, and nothing asserts they still do.**
Inline the `const` at the production arm, or rewrite that arm, and the test-side
assertion goes on passing. **The idiom being present is not the property being
covered** -- and it leaves `H2` completely untouched.

**The residual-free repair is to delete the `cfg` split.** The classifier's only
input is `forward_match_scrutinee`, which is **not `cfg`-gated**, so the
classification is a plain plan read available in both profiles:

```rust
let reason = if forward_match_scrutinee(…)? != unit.key.continuation_origin { … } else { … };
return Err(planner_error(reason));
```

One message path, one `return`, production and test seeing the same strings,
and the existing assertions pinning them **directly rather than transitively**.
**It closes `H1` and `H2` together.**

**The one thing it changes is that the production diagnostic becomes more
specific.** That is the direction this work has been going, not a regression --
but if something external freezes the current production wording, that is a
real reason to prefer the `const`. **`D0` settles that by measurement; do not
assume either way.**

## Deliverables

**`D0` -- is the production refusal wording frozen by anything outside this
file?** Grep for the exact sentence across `conformance/`, `spec/`, `library/`,
and the test corpus. **Report the enumeration, not a count.** If nothing
external depends on it, `D1` takes the deletion; if something does, `D1` takes
the `const` and `D2` becomes load-bearing.

**`D1` -- close `H1` and `H2`.** Preferred shape is deleting the split per
above. **If `D0` forces the `const` instead, `H2` is NOT closed by it** -- say
so explicitly and close `H2` separately, because a `const` does nothing about a
missing `return`.

**`D2` -- a control that reds on `H2`'s failure mode.** The mutation is
removing the refusal's `return` (or its equivalent under whichever shape `D1`
landed) and observing that a mismatched occurrence is **refused, not skipped**.
**Under the deletion this is nearly free** -- one `return`, one profile. **A
test that merely asserts the refusal text still exists does not discharge
this**: `H2` is a control-flow defect, and the whole point is that it passes
every text assertion.

## Acceptance criteria

**`AC-1` -- the production refusal string is reachable from a test profile, and
the node names the test that reaches it.** Transitive pinning through a `const`
satisfies this **only** if `D0` forced it, and then the node must state that the
reference itself is unasserted.

**`AC-2` -- `H2` is closed and demonstrated, not argued.** Perform the mutation,
show the validator refuses rather than continuing. **This is the criterion the
node turns on** -- `H2` is invisible to every assertion that inspects text.

**`AC-3` -- no change to the relation, the key, the threading, or
`rederive_consuming_occurrence`'s logic.** This node moves where a refusal is
constructed and nothing else. If a fix appears to need more, stop and report.

**`AC-4` -- the two existing negative controls still pass and still assert their
counterpart phrase is ABSENT.** That mutual-absence assertion is what makes the
arms non-overlapping rather than two spellings of one failure, and it is the
property most easily lost while restructuring the messages.

**`AC-5` -- no-regression, in CI.** `COORDINATION §12` -- the venue is CI, never
a local `--workspace` run. Build targeted, `-p ken-runtime`.

## Sizing

**`XS` under the deletion, `S` if `D0` forces the `const` and `H2` needs its own
close.** The measurement is `D0`; the code is a few lines either way.

## Sequencing

**This is NOT Runtime's next node.** The operator's standing priority is the
`RecursiveDescent` retirement, and the `D2k` probe sequences ahead of it. This
is a fill-in, ready whenever the priority chain yields.

## Adversary disposition, recorded here rather than replied

**`evt_2e245r28s3m6n`. Both additions accepted and they are this node's
substance.** It routed them to the Steward rather than replying to the
Architect, which is the correct edge.

- **Addition 1 -- "the `const` pins the TEXT and not the USE"** -- accepted, and
  it is the reason `D1` prefers the deletion. It also verified the enabling
  fact, that `forward_match_scrutinee` is not `cfg`-gated, which is what makes
  the deletion possible at all.
- **Addition 2 -- the absent-arm control-flow hazard** -- accepted, and it
  outranks the wording point it was appended to. It is `H2`, and `AC-2` exists
  for it.
- **It explicitly declined to gate**, noting the node was approved and CI was
  the remaining gate, and said it would hunt properly on the merge
  notification. **That is the report-only edge working as intended.**
- **Its earlier unfired premise is discharged, by the Architect, not by this
  node.** `evt_7b75nbgqbw04z` listed *"at most one parent has a given occurrence
  at position zero"* as unstated. It is now grounded structurally: `plan_expr`
  plans every source child separately, each visit mints a fresh append-only
  identity, and `semantic_ir.rs` puts origins and nodes in one identity space
  (`StaticOriginId(planned_node.0)`), so there is no second space to slip
  between. **Recorded here so it is not re-listed as unfired.**

## Not this node

- **Not the relation, the key, or the threading.** See `AC-3`.
- **Not a general planner-diagnostic wording pass.** One refusal site.
- **Not the `D2k` successor** -- that is unframed and waits on the Architect's
  probe.
