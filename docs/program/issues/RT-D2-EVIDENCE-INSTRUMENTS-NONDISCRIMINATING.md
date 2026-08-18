---
id: RT-D2-EVIDENCE-INSTRUMENTS-NONDISCRIMINATING
title: "Three instruments that discharged RT-BRANCHED-SCRUTINEE-UNIT-BODY-PORT cannot detect the failures they were chosen for -- AC-3's recorder is satisfied by a no-op D2, AC-4's control reaches its mechanism only from its own unit test, and the cfg(test) pin that cost a review round annotates a census compiled out 19 days earlier"
status: active
owner: runtime
size: S
gate: none
depends_on: []
blocks: [RT-CAVEAT-GUARD-SPELLING-DOMAIN]
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
  and correct the three doc sites above. **MERGED 2026-08-17**, landed squash
  `b7e2cf8f885bc103364975c64058dba6887872b4`, four paths, `+96/-15`, all four
  blob-verified on `main`. The observer row carries a `match_descent` bit set
  only after the plain Match branch is entered; the later route-1 replacement
  preserves that bit rather than overwriting it; the three doc sites now
  distinguish resolver entry, plain-Match descent, and direct non-`Construct`
  route-1 returns. `AC-1` was discharged by two mutations with the exact
  candidate restored after each — a committed no-descent mutant that leaves the
  former entry/route observation satisfied while `match_descent` goes false, and
  a temporary un-preserving read that reddens the new co-occurrence control with
  `route1: true, match_descent: false`.

  > ### `D1` DID NOT CLOSE FINDING 1. See `D4`, filed 2026-08-17 after the merge.
  >
  > **The `AC-1` discharge above is weaker than it reads.** Of the two mutations,
  > the committed no-descent mutant is `D1`'s own ablation switch, which returns
  > *before* the recorder and so cannot be passed by any resolver that reaches
  > it. Against the actual Finding-1 mutant — a `return` one line *below* the
  > recorder — the repaired triad **passes**. `match_descent` records arrival,
  > not descent. Do not cite `D1` as certifying that lowering walks an arm.

  > The census caveat was re-derived here rather than bumped, moving to **324**
  > with an inventory naming the observation scope, the three record helpers and
  > `RuntimeExpr::Var(0)`. **That re-derivation is not `D3`** — it keeps the pin
  > honest at the new magnitude; deciding what the pin should protect is still
  > `D3`'s, and the Adversary's second hole on it (it counts exactly
  > `#[cfg(test)]` while `any(test, feature = ...)` regions move invisibly)
  > remains open inside `D3`'s option set.
- **`D2`** — reach `agreeing_recursive_body_unit` from lowering with a real
  witness, or record why the in-situ path cannot be reached and what that means
  for `AC-4`'s claim (Finding 2). **DISCHARGED 2026-08-17 through its
  record-why arm**, at exact base `b7e2cf8f8`, no candidate and no retained
  change (`evt_2p6c34eg8shfm`).

  > **This deliverable used to end "a mixed-arm `Some`/`None` witness is the
  > first thing to try." That instruction was impossible to satisfy, and
  > Finding 2 above already said so** — the agreement check is reached only when
  > *every* arm returns `Some`, because the first `None` or `Err` short-circuits
  > past it at `:15838-15840`. The sentence is deleted rather than annotated,
  > because an implementer reads the deliverable, not the finding. The ring
  > re-derived the short-circuit independently and spent a turn on it.
  >
  > **What `D2` established:** the mixed-arm route cannot reach the seam, by
  > construction. **What it did NOT establish, and must not be read as:** that
  > the seam is unreachable. The only shape that could reach it is an
  > **all-`Some`** lowering witness, and `D2` was pointed at the mixed shape, so
  > it never searched for one. Reachability is **unresolved, not refuted.**
  >
  > **Named residual, unowned and not authorized as work:** does any Ken source
  > program drive `resolve_recursive_unit_body` to return `Some` on *every* arm?
  > If one exists, the seam is live production behaviour and `AC-4` can be
  > re-widened against it. If a bounded search finds none, that is a much
  > stronger statement than `D2` made and belongs in whatever node makes it.
  > Nothing in the tree answers this today: the Adversary's whole-suite probe
  > (923 passed / 1 failed, the one failure being `AC-4`'s own unit test) shows
  > only that no *existing test* reaches it.
- **`D4`** — **record work, not arrival.** `D1`'s `match_descent` bit does not
  catch the mutant that produced Finding 1. Filed from the Adversary's M8 hunt on
  `b7e2cf8f8` (`evt_13qerjefkkdpj`), **Steward-verified structurally against the
  tree** at `core.rs:15884-15901`.

  > **The mechanism.** `record_branched_scrutinee_unit_body_match_descent()`
  > fires at `:15890`, on **arrival** inside the `if let RuntimeExpr::Match`
  > branch. Everything the witness claims to observe — `declared_units`, the
  > `for` loop, `case_body_occurrence`, the recursive
  > `resolve_recursive_unit_body`, and the `agreeing_recursive_body_unit` call —
  > is at `:15891-15899`, **below the recorder and inside the bit's blind
  > region.** A `return Ok(None)` placed one line *below* the recorder yields
  > `entered=1, route1=false, match_descent=TRUE` while walking no arm: the
  > triad is satisfied, `:96` passes, and the test reds at `:104` on the
  > **pre-existing** error-string assertion, exactly as it did before `D1`
  > landed.
  >
  > **Why `D1`'s committed control cannot fail, which is the sharp part.**
  > `BRANCHED_SCRUTINEE_UNIT_BODY_SKIP_MATCH_DESCENT` is read at `:15886` and
  > returns at `:15887`, **before** the record call. It ablates the recording
  > itself, so any bit written at that site is false under it. It is a sound
  > positive control that the recorder fires when reached, and it is **not** a
  > discrimination test for descent, because no resolver that reaches the
  > recorder can pass it.
  >
  > ⇒ **Net new discriminating power on this witness is one mutant, and it is
  > the hook `D1` installed for itself.** The branch-deletion mutant is caught
  > at `:92` by the pre-existing `route1` assertion, before `:96` is evaluated.
  > Two mutants with **byte-identical compiler output** get opposite verdicts,
  > decided by which side of one recorder line the `return` sits on.
  >
  > **This is an evidence defect, not a behaviour defect.** The commit message's
  > own wording is accurate — *"set only after the plain Match branch is actually
  > entered"*. The overreach is downstream: the witness asserts *"D2 must descend
  > into the plain Match"*, the module doc claims it *"enters and descends
  > through the carried child's owning plain `Match`"*, and this node treated the
  > triad as certifying descent. **Entry is measured; descent is claimed.**

  **Remedy shape, the ring's to choose.** Bump a per-arm counter **inside the
  loop body**, after `case_body_occurrence` succeeds, so the row separates
  *arrived at the Match branch* from *walked into an arm* — 0 under the mutant,
  1 under the landed tree. **A counter recorded after the loop does not work on
  this witness**: the loop exits through `?` on arm 0, so `declared_units` never
  completes.

  **`AC-4a` — the discriminating mutant is the Finding-1 mutant**, a `return
  Ok(None)` placed *below* the recorder. It must red on the descent claim
  itself, not on the error string at `:104`. State the before/after and a
  positive control. **`D1`'s ablation switch does not qualify** — it is the
  control this deliverable exists because of.

  **`AC-4b`** — the printed evidence at
  `rt_branched_scrutinee_unit_body_port.rs:97-101` still prints only `entered=`
  and `route1=`, so a handback quoting the test's own output reproduces exactly
  the pair already ruled insufficient. Print whatever the repaired claim rests
  on.

  > **Not a defect, recorded so it is not re-filed:** the bit is per-entry, not
  > per-site — the recorder writes `rows.last_mut()`, so a Match descended at any
  > depth under one entry sets it. This witness has exactly one entry, so nothing
  > is wrong today; the assertion text says "the plain Match" while the bit means
  > "some Match under this entry". The retained-bit unit test at `core.rs:1143`
  > is **sound and not under attack** — it certifies that the route-1 replacement
  > composes with the descent bit, which is what it claims.

- **`D3` MERGED** at `b430d73e0` (2026-08-18), taking the **retire** option.
  Dispose of the pin (Finding 3): either revive the census it
  annotates, or retire the pin with its caveat, or re-key it to all six
  test-gating `cfg` spellings. **Do not just widen the count** — decide first
  what compiled thing it protects.

  > ### WHAT LANDED, AND WHAT IT SETTLES
  >
  > The candidate deletes only the live
  > `identifier_census_caveat_tracks_inline_cfg_test_region_count` test, the
  > caveat sentence it pinned, and the local `DOCUMENTED_INLINE_CFG_TEST_REGIONS`
  > constant. One path, `+0/-25`, no production code.
  >
  > **The ground is that the pin protected nothing compiled.** The census it
  > annotates sits under `#[cfg(any())]`, so widening its predicate — the re-key
  > option, and the whole of the measured input below — would have bought a
  > sharper guard over disabled commentary. The trim-normalized own-line count is
  > **324** and is now deliberately *not* asserted, rather than asserted at a
  > number no reader can act on.
  >
  > **The two open sub-questions in the blocks below die with the pin, they are
  > not answered.** The `19 to 23` invisible-drift measurement and the
  > `#[cfg(test)]` 327-vs-324 comment reconciliation were both inputs to a re-key
  > that will not happen. If a future node revives a census here, it starts from
  > the tree, not from these figures.
  >
  > **The post-merge Adversary hunt (`evt_6npaybf8cznp8`) confirmed every claim
  > this deliverable made** — one hunk, `+0/-25`, own-line count 324, zero
  > surviving references under `crates/`, and all three `RETIRED` successors
  > present and substantive. **It also strengthened the fork's ground:** it
  > flipped `#[cfg(any())]` to `#[test]` on all three retired censuses in
  > `control.rs` and **all three fail**, so *revive* was not merely the wrong
  > option here, it was not an available one. Filed separately as
  > [[RT-RETIRED-CENSUS-ROT]] — the rot is in the retirement convention, not in
  > this node, and it is not urgent.

  > **Measured input for the re-key option** (same hunt, quantified only): across
  > `ca639b5ef..b7e2cf8f8`, `core.rs` moved own-line `#[cfg(test)]` 323 to 324,
  > which the pin tracks, and `#[cfg(any(test, feature = "px8-ds-test-support"))]`
  > **19 to 23**, which it does not. **Four test-gated regions landed invisibly
  > in the same commit that re-derived the pin, two of them inside
  > `resolve_recursive_unit_body` itself.** A dropped suspicion, so it is not
  > re-checked: `#[cfg(test)]` occurs 327 times against 324 own-line matches, and
  > all three extras are inside comments — the pin's population is correct on its
  > own criterion.

  > **`D3` GATED [[RT-CAVEAT-GUARD-SPELLING-DOMAIN]], AND HAS NOW CLOSED IT.**
  > That node widens this same guard from one spelling to the full test-gating
  > domain, and its opening clause asserts the census *"errs toward a false red,
  > never a false green"* — which Finding 3 refutes, because a census that does
  > not compile errs toward nothing. **Only the re-key outcome left that node
  > with work.** `D3` retired instead, so the guard it would widen no longer
  > exists, and the node is `closed` — resolved without landing, not abandoned.
  > Its measured spelling table is preserved there as the record of what the
  > file held at `be25ea6a2`: `#[cfg(test)]` 322,
  > `any(test, feature = "px8-ds-test-support")` 12,
  > `any(test, feature = "r3-4b-observation")` 6.

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
