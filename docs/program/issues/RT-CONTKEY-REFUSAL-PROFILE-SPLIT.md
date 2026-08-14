---
id: RT-CONTKEY-REFUSAL-PROFILE-SPLIT
title: "the consuming-occurrence validator refusal has four defects that must land together: the production string sits in a cfg arm no test compiles; under cfg(test) the production arm is ABSENT rather than dead so a dropped return makes the validator silently ACCEPT a mismatched occurrence; and the classifier has two messages for FIVE causes, with the body arm a catch-all over an eliminator-kind defect, an identity-match failure and an ambiguity; and the sibling function 55 lines up has the same defect over four more causes, one of which the D2k probe only just measured"
status: ready
owner: runtime
size: S
gate: none
depends_on: [RT-CONTKEY-ELIMINATOR-ORIGIN-UNFIRED]
blocks: []
github: null
origin: "Architect non-blocking should-fix recorded in the RT-CONTKEY-ELIMINATOR-ORIGIN-UNFIRED approval (dec_2043sa5rjcyzb, evt_33r91fek6ghjp), plus two additions the Adversary routed to the Steward at evt_2e245r28s3m6n rather than replying to him. Filed by the Steward because a carry recorded only in an approval verdict and a PR body has no node and evaporates. AMENDED 2026-08-14 to fold in H3 from the Adversary's post-merge hunt evt_11b9910j85v5v, which asked for exactly that on the argument that D1 would otherwise promote a known-wrong diagnostic to production; the amendment also loosened AC-3, whose original wording banned the only clean discharge of the new D3. AMENDED AGAIN the same day to fold in H4 -- the sibling consuming_occurrence_from_seed has the same unnamed-cause defect, plus a fourth cause the D2k probe measured (evt_76cmre0qvsmmd). The Architect accepted that relocation out of the D2k successor at evt_56dvtaft7ep38 and ruled the new variant must be named for what it OBSERVES, since if the successor lands the absence stops being structural."
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

**Four defects, and they must land together.** `H1` is the wording, `H2` is a
control-flow hazard that outranks it, `H3` decides the *order* (see the box
below), and `H4` is the same defect as `H3` on the sibling function 55 lines
up, cut in here rather than into the `D2k` successor.

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

**`H3` -- TWO MESSAGES FOR FIVE CAUSES, AND THE BODY ARM IS A CATCH-ALL OVER
FOUR OF THEM.** Measured by instrumentation at `49c2bc38`
(`evt_11b9910j85v5v`): a distinct `eprintln!` at each of
`rederive_consuming_occurrence`'s four `None` returns, run against the
controls.

| cause | condition | message it gets | exercised |
|---|---|---|---|
| 1 | step 1: the claimed eliminator's position-zero child is not the continuation | `eliminator_origin` | yes |
| 2 | `planned_occurrence_expr(claimed.eliminator_origin)` is **not a `ComputationalMatch`** | `body_origin` | **no** |
| 3 | `matching` is **empty** -- no alternative's constructor identity is in `produced` | `body_origin` | **no** |
| 4 | `matching` has **two or more** -- ambiguous | `body_origin` | **no** |
| 5 | `Some(only)` but `only != claimed` | `body_origin` | yes |

**Cause 2 is an *eliminator* defect wearing the body name. Cause 3 is an
identity-matching failure where the body may be perfectly correct. Cause 4 is
ambiguity, not a mismatch.**

**This QUALIFIES the landed non-overlap property rather than contradicting
it.** Each control asserting its counterpart phrase absent does establish the
two *messages* are distinct failures, and the `EliminatorOrigin` mutation fired
cause 1 and nothing else. **What it does not establish is that either message
names its cause** -- and for the body arm it does not, because the `else` is a
catch-all. The claim that survives is *"the eliminator arm is 1:1 with step
1"*, which is what the parent node needed.

> ### `H3` IS THE REASON THIS NODE IS URGENT RATHER THAN TIDY
>
> **`D1` prefers deleting the `cfg` split. If the split goes, this two-message
> classifier BECOMES the production diagnostic** -- and the misattribution
> ships to the first person debugging `D2k-1c`'s consumption, who reads
> *"mismatched `body_origin`"* when the fault is the eliminator's kind (cause
> 2) or an ambiguous identity match (cause 4).
>
> **Fold it in here rather than filing it behind this node.** Doing `H1`/`H2`
> first and `H3` second means deliberately promoting a known-wrong diagnostic
> to production and then correcting it.

**`H4` -- THE SIBLING FUNCTION 55 LINES UP HAS THE SAME DEFECT, AND THE `D2k`
PROBE JUST ADDED A CAUSE NOBODY HAD ENUMERATED.**
`consuming_occurrence_from_seed` (`static_transition.rs:10854`) returns `None`
for **three** source-side reasons and keeps no record of which:

1. no outer eliminator scrutinizes the continuation -- structural absence **in
   the source walk**;
2. an outer eliminator exists but no case consumes the produced constructor;
3. `matching.len() >= 2` -- **ambiguity, declined**, which is `D2k` row 1.

**The `D2k` probe (`evt_76cmre0qvsmmd`) measured a FOURTH, and it is a
different kind from all three.** At row 4 depths 2 and 3, every extant target
relation is `Some`; what is missing is a **carried relation on the generated
edge**. Cause 1 is absence in the source walk; this is a generated edge with no
carried relation, reached by a different route and meaning a different thing.

> ### NAME THIS VARIANT FOR WHAT IT OBSERVES, NOT FOR WHAT IT CURRENTLY IMPLIES
>
> **Architect ruling `evt_56dvtaft7ep38`, and it is a correction to this
> frame's own first draft.** Runtime reported the condition as *"structural
> absence from the generated edge"* and this section originally recorded it as
> *"no such relation exists"*.
>
> **If the `D2k` successor lands, that absence stops being structural** -- those
> edges will carry a relation. A variant named for the implication **goes stale
> on the day the successor merges**, and the next reader believes a `None` that
> no longer means what it says.
>
> ⇒ **Name it `no carried relation for this generated edge`** -- what the code
> observed -- **never `no such relation exists`.** The same discipline applies
> to its doc comment and to any message it renders.

⇒ **`None` currently carries four opposite facts.** Row 1's means *"two
candidates, I declined"*; depth 2/3's means *"this generated edge carries no
relation"*. **Anyone diagnosing the second is sent to plan construction when
the truth is that the relation was never carried to that level** -- and that
misdirection is live today on the `RecursiveDescent` critical path.

> **Why this is here and not in the `D2k` successor -- SETTLED, not pending.**
> The Architect had written that the enum *"belongs in the same successor as
> whatever the probe selects."* The Steward cut it here instead, as a WP-cut
> call (`ken-steward §3`), and **he accepted the relocation at
> `evt_56dvtaft7ep38`**: *"your cut is better than mine -- I grouped it by which
> node noticed it; you grouped it by defect class."*
>
> One node now owns *"this file's `Option` returns must name their cause"*,
> covering `:10854`'s three causes and `:10909`'s four, while the `D2k`
> successor stays purely about the relation. It also keeps two nodes out of
> `static_transition.rs` concurrently.
>
> **There is NO blocking dependency in either direction.** Row 1 is not in the
> `D2k` successor's scope, so the relation work does not wait on this enum, and
> this enum does not wait on the relation.

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

**`D3` -- `H3`: each cause names itself.** Causes 2, 3 and 4 stop being
reported as `body_origin`. **Do this in the same landing as `D1`**, per the
box above -- not after it.

**The mechanism choice is yours and both routes are open** (see the amended
`AC-3`): either `rederive_consuming_occurrence` reports *why* it returned
`None` -- a small enum, or `Result<_, Cause>` in place of `Option` -- or the
caller re-derives conditions 2/3/4 from values it already holds. **Prefer the
first if it is not invasive.** The second duplicates the conditions, and two
copies of a five-way condition drift; that is the defect class this node is
already about. **If the first turns out to reach beyond this file, report that
and take the second** -- say which you took and why.

**Causes 2, 3 and 4 are unexercised by anything today.** Their messages are
therefore unpinned in exactly the way `H1` describes, so **`D3` without `AC-4`
is untested text.**

**`D4` -- `H4`: `consuming_occurrence_from_seed`'s `None` names its cause too.**
Same treatment as `D3`, on the sibling function. **Four variants, not three:**
the Architect's three source-side causes plus **absence from the generated
edge**, which the `D2k` probe measured and which is distinct from source-side
structural absence.

**`D3` and `D4` should share one cause type if the two functions' causes
overlap, and must not be forced into one if they do not.** Report which, with
the overlap named. **A shared enum whose variants are half-inapplicable at each
site is worse than two honest types** -- and `subsume-don't-proliferate` is a
reason to check, not a reason to merge regardless.

**Row 1's ambiguity stays unprobed and uncollapsed.** The `D2k` probe
deliberately left it alone. **`D4` makes it *nameable*; it does not resolve
which candidate is right**, and a candidate that silently picks one has widened
into the `D2k` successor's scope. Stop and report instead.

## Acceptance criteria

**`AC-1` -- the production refusal string is reachable from a test profile, and
the node names the test that reaches it.** Transitive pinning through a `const`
satisfies this **only** if `D0` forced it, and then the node must state that the
reference itself is unasserted.

**`AC-2` -- `H2` is closed and demonstrated, not argued.** Perform the mutation,
show the validator refuses rather than continuing. **This is the criterion the
node turns on** -- `H2` is invisible to every assertion that inspects text.

**`AC-3` -- no change to the RELATION, the KEY, or the THREADING.** This node
changes where a refusal is constructed and how it is attributed, and nothing
about what is being validated. **A candidate must not alter which occurrence is
required, how `unit.key` is formed, or how either is threaded.**

> **`AC-3` was amended when `H3` was folded in, and the original wording is the
> reason this note exists.** It read *"no change to the relation, the key, the
> threading, or `rederive_consuming_occurrence`'s logic"*. **That last clause
> forbade the cleanest discharge of `D3`** -- having `rederive` report which of
> its four `None` conditions fired -- and would have forced the duplicated
> re-derivation in the caller, which is the drift-prone shape this node exists
> to remove. **The banned scope was forbidding the only good path to the
> node's own deliverable.** What the ban was protecting is the *validation
> semantics*, and that is what it now says. **`rederive_consuming_occurrence`'s
> return type and its internal reporting are IN scope; which occurrences it
> accepts is NOT** -- and `AC-6` is the control that keeps that honest.

**`AC-4` -- every cause a message can name is reached by a test, or the node
says which are not and why. This covers `D4`'s four causes as well as `D3`'s
five.** Causes 2, 3 and 4 are unexercised today. **If a
cause cannot be constructed, that is a finding worth reporting** -- an
unreachable arm in a validator is either dead code or a condition the plan
makes impossible, and both are worth knowing. **Do not add a message for a
cause you cannot reach and call it covered.**

**`AC-5` -- the two existing negative controls still pass and still assert their
counterpart phrase is ABSENT.** That mutual-absence assertion is what makes the
arms non-overlapping rather than two spellings of one failure, and it is the
property most easily lost while restructuring the messages. **Under `D3` the
absence assertions must be re-derived, not merely kept compiling** -- new
messages change what "counterpart phrase" means.

**`AC-6` -- the accept/reject behaviour is unchanged.** Every input refused
before is refused after, and every input accepted before is accepted after.
**This is the criterion that makes `AC-3`'s loosened wording safe**: the
refusal's *attribution* may change freely, its *incidence* may not.

**`AC-7` -- no-regression, in CI.** `COORDINATION §12` -- the venue is CI, never
a local `--workspace` run. Build targeted, `-p ken-runtime`.

## Sizing

**`S`.** `H1`/`H2` alone are `XS`; `H3` is what makes it `S` -- three causes to
distinguish, each needing a reachability answer under `AC-4`. **The measurement
is `D0` plus `AC-4`'s reachability question; the code is small either way.**

**Re-sized from `XS` when `H3` was folded in.**

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

**Second hunt `evt_11b9910j85v5v`, on the merged `49c2bc38`. Accepted, and
folded into this node rather than filed behind it -- which is what it asked
for.** It is `H3`, and its timing argument is the box above: doing `H1`/`H2`
without it would promote a known-wrong diagnostic to production and then
correct it.

- **It closed its own prior finding by measurement**, instrumenting all four
  `None` returns and observing that the `EliminatorOrigin` mutation fires cause
  1 and nothing else, while the body mutation goes through `Some(only)`. **The
  two arms are reached by disjoint paths, measured rather than asserted.**
- **It qualified the landed non-overlap property instead of overturning it**,
  which is the distinction worth keeping: the controls prove the two *messages*
  are distinct failures, not that either *names its cause*.
- **It also corrected its own earlier report unprompted** -- the arity
  enumeration in `evt_4zx9xp7qkf6rm` "read as a measurement" when it was a
  plausible-mechanism list, and it withdrew the implied doubt about the current
  code. **That correction is why `LANG-WITNESS-ARITY-DERIVED` is framed as an
  absent guard rather than a live defect**, and it is recorded in that node.

## Not this node

- **Not the relation, the key, or the threading.** See `AC-3`.
- **Not a general planner-diagnostic wording pass.** One refusal site.
- **Not the `D2k` successor** -- that is unframed and waits on the Architect's
  probe.
