# RT-CONTINUATION-CALL-DISCHARGE — `D0` trace and `D1` classification

> ## READ THIS FIRST — THIS DOCUMENT IS A DATED RECORD, NOT A CURRENT STATE
>
> ### THE CLASSIFICATION DID NOT CLOSE. NONE OF THE THREE OPTIONS IS A RESOLUTION.
>
> **Added 2026-08-08 by the Runtime implementer, on the Steward's instruction
> (`evt_3ktfbtkehh3gk`) and the Architect's hard-stop ruling
> (`evt_dakdkqk4wbg6`), both relayed by the Runtime leader.**
>
> **Everything below this block is the `D0`/`D1` handback exactly as it was
> returned at exact `9f0a4e41`, before any ruling on it.** It is preserved
> verbatim because it is the measurement every later ruling was argued from.
> This block does not correct it. It records what became of its open questions,
> and the short answer is that they were answered and the answer did not hold.
>
> **The chronology, in full, because no single step of it is the current
> state:**
>
> 1. **The Architect ruled option 3** (relayed `evt_4ebpfvfrvv8qy`): the planner
>    over-issues the causal edge, because the heterogeneous-deforestation bridge
>    performs the selected case inline and the producer's call seat is never
>    reached. Option 1 was refuted there; option 2 is refuted below, in section
>    4, and that refutation still stands.
> 2. **`D2` built that ruling's mechanism — a planner-side source-structural
>    classification that excludes the edge — and measured that it does not
>    hold.** Classifying every bridge-taken edge deferred-inline reds nineteen
>    committed controls, because thirty-four bridge-taken edges are genuinely
>    compositionally consumed. Narrowing to the ordinary bridge arm still
>    conflates the ruled witness with `d8e`'s, whose planner coordinates are
>    **identical** to it; the two differ only in the de Bruijn callee the arm
>    body resolves against the materialized environment. Excluding the edge
>    before interning loses the binding and lets `d8e` compile in a shifted
>    environment where it previously refused. Excluding only the call leaves an
>    interned unit with no caller.
> 3. **The Architect withdrew that mechanism** (`evt_dakdkqk4wbg6`), on the
>    ground the measurement established: **one planner edge carries two roles** —
>    binding projection, which the deferred constructor environment needs to
>    install IH and static-worker bindings at recursive positions, and causal
>    call obligation, which only a real direct or verified composed call owes.
>    **Bridge selection cannot distinguish them.** Both failures above are real,
>    not missing predicates.
>
> **Where that leaves this record.** The exact-witness finding is unaffected:
> for this identity, no call occurred. **But option 3 is NOT implementable as
> planner-side edge exclusion, and the three-way frame below is incomplete at
> the component boundary.** Nothing in sections 5 through 8 should be read as
> naming an implementable resolution or as work to pick up — in particular,
> section 5's *"options 1 and 3 are both live"* and section 6's separating
> measurement are both historical.
>
> **The successor is a distinct seventh authority, and it is not this node.**
> Phase-spanning continuation-edge disposition: the planner mints an opaque
> binding candidate, and lowering settles each candidate exactly once, from
> events only it can observe. It is **deliberately not a third discharge** —
> the causal-call partition stays exactly as it is. Do not start it; it is
> released separately.
>
> **This document's own standing is unchanged by all of that.** It is accepted
> `D0`/`D1` attribution evidence, `crates/`-identical, and it said at the time —
> correctly — that the classification did not close.
>
> **Held and never published:** the `D2` code, its four controls and its
> committed red control are exact `a15a3e934766a1d075386ba561a9469e51a448b7`.
> The red is load-bearing: it is what established the two roles, and a control
> edited until it passes stops being evidence of anything.
>
> **Sections 1 through 8 below are unchanged and are not to be rewritten.** One
> note on their coordinates: the base line immediately below names `cc4621d0`,
> which was correct when this was written. The commit was later carried
> unchanged onto `9f4a44d6`; no measurement in the record is affected by that
> move.

Base: `origin/main` `cc4621d03a13b7a47cadb1b8ecf035b7ae85a378`. Frame blob
`59fb2325`, node blob `f56b4641`, both read from the worktree at that base.
Predecessor `2137ee7d` confirmed an ancestor.

Attribution only. **No production code changed**; the census instrument was
reverted before this record was written, and every coordinate is cited by
grep-able phrase rather than by line.

> ## THE HEADLINE, BECAUSE IT CHANGES WHAT `D1` CAN BE ASKED FOR
>
> **`D1` does not close, and this record says so rather than choosing.** Option 2
> is refuted decisively. Options 1 and 3 are both live, and the single fact that
> appeared to settle them — `declared = 1` — turns out to prove nothing, by the
> ledger's own documented design. The frame's section 5 anticipates exactly this
> outcome: *route it, do not absorb it.*

## 1. Method

One census at the top of `ContinuationClaimLedger::close`, **above the
`resolved`/`declared` guards**, so an identity is recorded before any refusal can
return. A second at `ContinuationClaimLedger::open`, which answers a question
`close` structurally cannot: whether a lane **plans** the token at all. A lane
that never opens a ledger and a lane that opens one with an empty planned set
are different facts, and neither is visible from `close`.

Environment-gated, not `#[cfg(test)]`, and one `write_all` per batch — the two
disciplines the predecessor node validated. Records carry the pid and the
libtest thread name, so identities are attributable to fixtures and committed
controls can be excluded by name rather than by guesswork.

## 2. `D0` — the missing identity, all seven fields

Activated lane, on the held lane-pair witness (`65639a13`, read and not
published):

| field | value |
|---|---|
| construct origin | `StaticOriginId(40)` |
| continuation origin (`producer_result_origin`) | `StaticOriginId(40)` |
| alternative | `0` |
| recursive position | `0` |
| call-site sequence | `0` |
| target | `ContinuationSpecializationId(0)` |
| emission owner | `Predeclared(PredeclaredFunctionId(0))` |

Provenance carried with it: `producer_owner = PredeclaredFunctionId(0)`; worker
`parent_origin = 10`, `producer_origin = 40`, `sibling_position = 0`,
`closure_origin = 39`, `body_origin = 38`, `declared_arity = 1`, `captures = []`.

Ledger at close: **`planned=1 resolved=1 declared=1 emitted=0 composed=0`**,
disposition `UNDISCHARGED`, and `claims` holds the key with value `None` — the
slot exists and was never claimed by either form.

### The retained lane does not discharge it, because it does not have it

The frame expects the retained lane to close the same program and offers its
disposition as *"the single most informative fact available, and free because
both lanes already run"*.

**Measured: the retained lane never opens a claim ledger for this program at
all.** No `OPEN` record, no `LEDGER` record, and the compile returns `Ok`. There
is exactly one `.close()` call site and the instrument sits at its top, so a
missing record means `close` never ran, not that it ran quietly.

⇒ **The free control does not exist.** The identity is not discharged directly,
not discharged compositionally, and not planned. This is a correction to the
frame's premise, and it matters: any argument of the form *"the retained lane
discharges it as X, so the activated lane should too"* has no evidence under it.

It is also **not** evidence that the activated plan is wrong. The two lanes lower
by different strategies, so planning different causal populations is expected
rather than anomalous.

## 3. `D0` — the population, with committed controls excluded and named

Whole lib suite, retained configuration, 820 passed / 0 failed / 4 ignored with
the instrument in place.

**602 ledgers opened, 427 closed** — 175 opened without closing, which is
compilations that fail or return before closeout. **213 planned identities**
reached `close` and each produced exactly one disposition record; zero orphans,
zero duplicate keys.

| disposition | all | committed controls | **independent** |
|---|---|---|---|
| `DIRECT` | 170 | 0 | **170** |
| `COMPOSED` | 34 | 0 | **34** |
| `UNDISCHARGED` | 7 | 7 | **0** |
| `BOTH` | 2 | 2 | **0** |
| total | 213 | 9 | **204** |

**Every undischarged and every double-claimed identity is a committed control**,
named in full:

- `coc_d3_the_trailing_suffix_is_continued_...` — this chain's control
- `ccr_d3_the_active_carried_route_is_taken_...` — this chain's control
- `sar_d3_the_ordinary_live_cell_is_routed_...` — this chain's control
- `d4_failing_to_accumulate_emissions_reds_the_closeout_set_equality` — a
  deliberate mutation of the closeout law
- `d5a_the_one_causal_ledger_closes_over_the_generalized_emission_owner_domain`
- `d8k_the_causal_population_is_a_disjoint_partition_of_direct_and_composed` —
  supplies both `BOTH` rows and one `UNDISCHARGED`, by design

⇒ **Excluding committed controls, no independent program in this corpus reaches
`close` undischarged.** Both discharge forms are well populated — 170 direct and
34 composed — so neither mechanism is exotic and neither is the thing that is
broken in general.

The three chain controls all show the exact failing shape
`planned=1 declared=1 emitted=0 composed=0`, which is consistent: each arms
A-only exclusion internally, so each *is* the activated lane.

**The two independent A rows remain the floor, not the perimeter.**

## 4. `D1` — option 2 is refuted, and the reason is clean

**Option 2 — a real composed consumption occurred but its evidence was lost.**

A composed discharge requires a recorded raw-worker call that is **found in the
finished CLIF**, whose decoded callee matches the target, whose operand run
matches the target's declared run, and whose result is shown to return
downstream into the unchanged continuation.

**Measured: `emitted = 0` and `composed = 0` together, with no call instruction
of any kind for this identity.** The activated path reaches an `Active` resume
with `pending_len = 0`, where `resume_active_continuation` returns its operand
unchanged — it writes no call.

⇒ There is no lost evidence because **there is no evidence to lose.** Option 2
requires a consumption to have happened; nothing happened. Refuted.

## 5. `D1` — options 1 and 3 are both live, and the fact that looked decisive is not

**What I expected to settle it, and why it does not.** `declared = 1` reads as
lowering having *intended* to call: a `FuncRef` was minted for this exact
identity. If declaration were per-decision, that would be direct evidence of a
real obligation and would carry option 1.

**It is not per-decision.** `close`'s own note says so verbatim:

> `DECLARATION may remain over the full planned set. An unused declaration is a
> `FuncRef` nobody called, not an emitted call, so the declared population stays
> equal to planned even where the discharge took the composed form.`

⇒ Declaration is **bulk over the planned set**. `declared = 1` restates
`planned = 1` and carries no information about intent. Recorded because it is
exactly the inference a reader of this trace will reach for, and it is wrong.

**What remains for each option, stated symmetrically.**

*Option 1 — a real direct obligation was skipped.* The plan projects a causal
call from the producer at origin 40 to specialization 0; the continuation worker
exists with `declared_arity = 1` and a body at origin 38; nothing in the
activated path discharges it. But the whole of this is *"the plan says the
obligation exists"*, which assumes what option 3 denies.

*Option 3 — the activated path has no causal obligation.* `pending_len = 0` says
the `Active` frame has no pending eliminators, so there is semantically nothing
for a continuation to do. **The frame states plainly that `pending_len == 0`
alone does not establish this, and it does not.** Worse, option 3 is not a free
relabelling: `open` records that `planned == resolved` is **structural today**
because `resolve_continuation_targets` walks the same projection, so a
projection-level correction **moves the set `close` checks against**. That has to
be argued at planner authority and measured.

**Neither is refuted.** Choosing between them on what I have would be a
preference dressed as a result, which is the failure mode the frame names.

## 6. The measurement that would separate them

**Does the continuation specialization at `body_origin = 38` perform work the
activated program otherwise omits?**

- If its body does work that nothing else in the activated lowering performs,
  the obligation is real and the call seat skipped it — **option 1**, and the
  repair is at the producer/call seat with finished-CLIF verification retained.
- If its body is semantically inert for this shape — the identity on the
  producer's result — then the projection over-issued a call for a path that
  cannot need one, and the correction is **option 3**, at planner authority.

This is a question about what the planner projects and what that unit's body
means. It sits at planner/component-design authority, which is precisely where
the frame forbids me to absorb it.

## 7. Hard stop, and what is deliberately not done

Section 5 of the frame: *"If the classification does not close — route it, do not
absorb it."* It does not close, and this is that route.

**Not done, so it does not read as done:** no repair attempted; no discharge
ownership assigned; the token is not discharged in the empty resume; the
set-equality law, the both-sets refusal, and `composed` being fed only from
`function_local.composed_discharges` are all untouched and unexamined-for-change;
nothing is bulk-claimed; no composed discharge is manufactured and no identity
return is treated as a call.

Held evidence `65639a13` was **read and not published**. The cross-crate census
was **not re-run** — that question is retired. Rows 1-5, `issues/`, the five
landed repairs, and the ignore inventory are untouched.

## 8. Re-size input

`D0` is complete and its population is closed with controls excluded. `D1`
delivers one refutation of three and a named discriminator for the remaining
fork, which needs a ruling this seat cannot make.

The remaining work after that ruling is bounded and different in each direction —
a producer/call-seat repair with CLIF verification, or a planner-authority
projection correction whose blast radius includes the set `close` checks against.
**Sizing the remainder before the fork is ruled would be sizing two different
nodes at once.**
