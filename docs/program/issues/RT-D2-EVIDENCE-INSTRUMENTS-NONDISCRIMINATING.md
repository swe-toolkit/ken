---
id: RT-D2-EVIDENCE-INSTRUMENTS-NONDISCRIMINATING
title: "Three instruments that discharged RT-BRANCHED-SCRUTINEE-UNIT-BODY-PORT cannot detect the failures they were chosen for -- AC-3's recorder is satisfied by a no-op D2, AC-4's control reaches its mechanism only from its own unit test, and the cfg(test) pin that cost a review round annotates a census compiled out 19 days earlier"
status: active
owner: runtime
size: S
gate: none
depends_on: []
blocks: []
github: null
origin: "Adversary M8 hunt on ca639b5ef, evt_12x7wnwfbfbr (thread thr_5x0fypvv6rzhb), three findings measured by mutation with positive controls, five attacks refuted. Steward-verified the structural half of each before filing. AC-3's amended wording was the Steward's own. Steward-filed per COORDINATION section 2."
---

> # THE CODE IS NOT KNOWN TO BE WRONG. THE EVIDENCE IS KNOWN NOT TO SHOW IT RIGHT.
>
> [[RT-BRANCHED-SCRUTINEE-UNIT-BODY-PORT]] is `merged` and stays merged. **No
> finding here says the landed behaviour is incorrect**, and the Adversary
> refuted five attacks that would have. What they say is that three of the
> instruments the node's acceptance rested on **cannot distinguish the failure
> they were chosen to detect.** That is a different defect and it is not
> repaired by re-reading the code.
>
> **`AC-3`'s wording was mine.** I amended it from a bare `route1 == 0` to
> `entered >= 1 AND route1 == 0` precisely to close this class, and the amended
> form is still satisfied by a `D2` that does nothing. Recording that plainly is
> the point of the node.

## Finding 1 — `AC-3`'s criterion is satisfied by a `D2` that walks no arm

Adversary mutation at `core.rs:15833`: return `Ok(None)` without walking a single
arm — mechanism present, doing nothing.

| | `entered` | `route1` | `AC-3` | CLI test |
|---|---|---|---|---|
| landed | 1 | 0 | satisfied | pass |
| descent walks no arm | 1 | 0 | satisfied | **FAIL** |

**Identical on both recorder axes.** The mutated build also refuses at
`BoundaryCarrier: ... takes no arguments, but the call provides 1` — the
**pre-`D2`** outcome — so the criterion cannot separate `D2` from `D1` either.

**What actually caught the mutation is the CLI test's error-string assertion**
(`rt_branched_scrutinee_unit_body_port.rs:103-106`), which is **not part of the
criterion the resolution recorded.** The node is protected; the AC is not what
protects it.

**Why the recorder cannot see it.** It is a negative-only instrument.
`route1 == 0` proves route 1 was not taken. **Nothing in it proves the Match
branch was entered**, because the row's four descriptive fields are populated
only on the route-1 path and read `false`/`0` on the advance. So `entered` and
`advanced` are one bit read twice, not two observations.

**Remedy (the ring's, and cheap):** give the entry row a tag the descent sets —
a `descended` bit, or record `cases.len()` on the Match branch — so the two
facts are independently observable.

## Finding 2 — the headline mechanism is reached only by its own unit test

`agreeing_recursive_body_unit` (`core.rs:1106`) is `D2`'s stated contract:
*"requires the arms to agree; disagreement refuses rather than picking one."*

Adversary made it `panic!` unconditionally, positive control first:

- positive control: the `AC-4` unit test panics ⇒ the probe is live;
- `-p ken-runtime --lib` whole suite: **923 passed, 1 failed** — the one failure
  is the `AC-4` unit test itself;
- the `D2` CLI witness: **passes unchanged**, same error string.

**Steward-verified statically.** Exactly one production call site,
`core.rs:15842`, plus the two test calls at `:1137`/`:1140`. And the loop above
it carries `let Some(unit) = self.resolve_recursive_unit_body(...)? else { return
Ok(None) };` at `:15838-15840` ⇒ **the agreement check is reached only when every
arm returns `Some`**; the first `None` or `Err` short-circuits past it.

⇒ `AC-4` is discharged by a `#[cfg(test)]` unit test calling the generic helper
directly with `[41_u8, 41]` and `[41_u8, 42]`. **It is never reached through
`resolve_recursive_unit_body`, never with a `StaticOriginId`, never from
lowering.**

**Related, read from the code and NOT measured — say so when acting on it.**
`Some(a)` versus `None` across arms **is** a disagreement, and the loop picks
`None` at `:15837-15839` without consulting the check. `None` is not a refusal:
`recursive_position_unit_body`'s own doc (`core.rs:15805-15808`) gives it the
positive meaning *"structural-data positions return `None`; they resume the
eliminator directly and take no arguments"*, and it reaches
`lower_recursor_residual_call` (`core.rs:3098-3124`), which is fail-closed only
when the call passes arguments — `reject_carried_residual_arguments(0)` returns
`Ok` (`core.rs:3073-3076`). **No mixed-arm witness was built.** What *is*
measured: the Finding-1 mutation drove exactly this `None` path and it refused.

## Finding 3 — the pin that cost a review round annotates a census compiled out

The `#[cfg(test)]`-count pin reddened CI on `D2` candidate 1 and bought the
entire second review round. The census its caveat annotates —
`the_lower_expr_call_population_is_dispositioned_by_owner_not_by_site`
(`control.rs:9234`) — carries **`#[cfg(any())]` at `control.rs:9233`**, which is
always false. **Steward-verified by reading the attribute.**

Adversary measured it, positive control first:

- subject: `compile_error!` planted in that function body, `-p ken-runtime
  --tests` **compiles clean**;
- positive control: the same macro in the live caveat test at `control.rs:9310`
  errors.

**The file is compiled; the census is not.** It is the only site applying
`identifier_occurrences` to `core.rs`'s text — the other live user
(`control.rs:8586`) feeds hand-written literals.

**The ordering is the part worth keeping.** `6a451b456` (2026-07-29,
`RT-FNSPLIT-RECUR-PORT`) retired the census to `#[cfg(any())]`. `be25ea6a2`
(2026-08-17, `RT-CENSUS-CAVEAT-GUARD`) landed the count-keyed pin, and
`git merge-base --is-ancestor` confirms the retirement **precedes** it. ⇒ **The
census was already compiled out when the pin was built.** The caveat's stated
hazard — *"a call added inside an inline region would be counted as production,
which errs toward a FALSE RED"* — cannot occur, because no compiled assertion
counts anything.

**Second, independent hole in the same pin.** It counts lines whose trimmed text
is **exactly** `#[cfg(test)]`. `core.rs` carries six test-gating `cfg` spellings,
and `D2` also moved `#[cfg(any(test, feature = "px8-ds-test-support"))]` from 23
to 25 lines — `record_branched_scrutinee_unit_body_entry` and its call site.
**That is test-only text added to the production file that the sentinel does not
see**, and the caveat's own wording does not cover it. Had a `lower_expr` call
landed there instead, nothing would have moved.

## Also owed, cheap — three doc sites state the recorder's OLD meaning

Under `D1`, `route1.len()` was the count of route-1 returns. Under `D2` it is the
count of **resolver entries**, with route-1 a per-row bit. Still describing the
old meaning:

- `core.rs:1034-1035` — *"counting direct route-1 returns"*;
- `core.rs:1014-1016` — *"records the early return in `recursive_position_unit_body`
  directly"*; it now records inside `resolve_recursive_unit_body`, at arbitrary
  recursion depth, overwriting the entry row;
- `rt_branched_scrutinee_unit_body_port.rs:3-4` — the witness file's own MEASURED
  line.

**A future reader checking `entered >= 1` against that doc concludes a nonzero
`len()` means route 1 was taken, which is the inversion.**

## Deliverables

- **`D1`** — make `entered` and `advanced` independently observable (Finding 1),
  and correct the three doc sites above.
- **`D2`** — reach `agreeing_recursive_body_unit` from lowering with a real
  witness, or record why the in-situ path cannot be reached and what that means
  for `AC-4`'s claim (Finding 2). **A mixed-arm `Some`/`None` witness is the
  first thing to try.**
- **`D3`** — dispose of the pin (Finding 3): either revive the census it
  annotates, or retire the pin with its caveat, or re-key it to all six
  test-gating `cfg` spellings. **Do not just widen the count** — decide first
  what compiled thing it protects.

  > **`D3` GATES [[RT-CAVEAT-GUARD-SPELLING-DOMAIN]], WHICH IS `ready` AND MUST
  > NOT BE STARTED.** That node widens this same guard from one spelling to the
  > full test-gating domain, and its opening clause asserts the census *"errs
  > toward a false red, never a false green"* — which Finding 3 refutes, because
  > a census that does not compile errs toward nothing. **Only the re-key outcome
  > leaves that node with work, and then it is likely subsumed here rather than
  > run separately.** Its measured spelling table stays valid either way:
  > `#[cfg(test)]` 322, `any(test, feature = "px8-ds-test-support")` 12,
  > `any(test, feature = "r3-4b-observation")` 6, at `be25ea6a2`.

## Acceptance criteria

- **`AC-1` — every repair is demonstrated by the mutation it now catches.**
  State the mutation, the before/after, and the positive control. **A green run
  is not evidence here; the whole node exists because green runs were.**
- **`AC-2`** — no behaviour change to the landed `D2` mechanism. This node
  repairs evidence, not semantics.
- **`AC-3` (no-regression).** Workspace green **in CI**, never a local
  `--workspace` run (`COORDINATION §12`).

## Sequencing

**Not urgent, and say so when picking it up.** Nothing here says the landed
behaviour is wrong, and the CLI error-string assertion does catch the Finding-1
mutation today. This is evidence quality, and it queues behind
`RT-PLANNER-GRAPH-FOUNDATION-SPLIT`'s pivot.
