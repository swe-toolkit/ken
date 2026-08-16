---
id: RT-MATCH-SCRUTINEE-PORT
title: "NOT A PORT -- excluding MatchScrutineeRecursor is behaviour-preserving on the scalar-selected rows, measured differentially, so no capability is at stake; the residual obligation is the same differential on the aggregate rows"
status: active
owner: runtime
size: L
gate: none
depends_on: [RT-MATCH-DIFFERENCE-REACHABILITY, RT-DESCENT-RETIRE-PRIOR-ART]
blocks: [RT-DESCENT-RETIRE]
github: null
origin: "Architect ruling evt_nb12nmhd2zzk, 2026-08-16: build the retirement on the uniform-lowering family (GHC), declining the prior-art advisory's global IR invariant as the primary route. Sizing and framing assigned to the Steward in that ruling. Steward-filed per COORDINATION section 2."
---

## D1c RESULT — THE REGRESSION PREMISE IS NOT UNWARRANTED, IT IS FALSE

**Handed back `evt_1e5wbj4af4xbc`, measured at exact `b7f65ad0c`. No candidate,
no diff** — fixtures removed, `git diff --exit-code` and `--check` clean. **This
section is the only durable record.**

**`D1c` ran the differential nobody had run: production versus excluded, on the
same row.** For both scalar-selected rows, pre-exclusion residual was exactly
`{MatchScrutineeRecursor}` and `debug_assert!(was_present)` passed — **so the
exclusion was live, not vacuous.** Both legs returned the **identical**
`Unsupported(UnsupportedLowering { construct: "Match", reason: "scrutinee is not
a constructor value" })`.

⇒ **The scalar cells DO NOT COMPILE TODAY UNDER THE RETAINED LANE EITHER.
Retiring the variant changes nothing for them.**

> ### THIS IS A FOURTH ANSWER. It is not arm 1, 2 or 3, and it is stronger than any.
>
> The three arms all asked **"are these programs well-formed?"** — via the
> interpreter, because the answer was supposed to decide whether a refusal costs
> a capability. **`D1c` makes that question irrelevant to the retirement.** A
> regression is a **difference** between retaining and removing, and **the
> measured difference is zero.**
>
> **I withdrew the `AC-3` premise as UNVERIFIED. It is REFUTED.** *"The scalar
> cells compile today via the retained lane"* is not merely unwarranted — it is
> **false**, and one differential run showed it. **The Architect and I both
> built three arms on top of it.**
>
> ⇒ **`D1b`'s evaluator refusal is correctly NOT carried as evidence.** It was
> answering a question that turned out not to gate anything.

**Soundness note, because the obvious attack is available.** Identical error
text does not prove identical code paths — production may never have reached
`core.rs:17959` by the same route. **That does not weaken the conclusion:** the
regression claim is about **observable behaviour under removal**, and if the
retained lane is not what produced these rows' behaviour, removing it cannot
change it. **Either reading gives a zero differential.**

## THE RESIDUAL OBLIGATION: the differential is complete on 2 of 5 rows

**`D1` measured all five rows under EXCLUSION ONLY** (its own instrument was the
exclusion probe). **`D1c` added the production leg for the two SCALAR rows.**

⇒ **Rows 2, 4 and 5 — the aggregate-selected ones — have NO production leg.**
They execute to `Returned(Int(Small(7)))` **excluded**; nobody has confirmed
they produce the same value **retained**.

> ### THAT IS THE HALF THAT COULD STILL HIDE A REGRESSION, and it is the only half.
>
> A scalar row that refuses both ways is safe. **An aggregate row that executes
> under exclusion is only safe if it executes to the SAME VALUE under
> retention** — a silent value change is exactly what a lane swap can cause and
> exactly what "it compiles and runs" does not detect.
>
> **Do not close this campaign on the scalar half.**

## What this node is

**The retirement's last piece of engineering, and the campaign's own landed
method applied to its last variant.**

Four of the six original residual variants were retired by porting to uniform
lowering. `MatchScrutineeRecursor` is the survivor. **It was framed to go the
same way, and it did not need to** — `D1c` measured that excluding it is
behaviour-preserving on the scalar rows, so there was no port to build.

> ### THIS IS NO LONGER A PORT. The ID still says so and cannot be renamed.
>
> **`D1c` settled it: excluding the variant is behaviour-preserving on the
> measured scalar rows, so there is no capability to port.** `D2`'s
> pre-authorized bound, the three arms, and the interpreter question below are
> all **retained as record, not as live work.**
>
> **The ID `RT-MATCH-SCRUTINEE-PORT` asserts a conclusion this node refuted**
> (Architect, `evt_3v1zp1g315vxz`: an ID cannot carry a disjunction, and the
> churn of renaming exceeds the harm). **Read the title, not the ID.**

> ### THE ROUTE IS SETTLED. Do not re-derive it, and do not re-open what it closed.
>
> **Source-unreachability is dead FOR THE OUTER SHAPE, from both directions.**
> [[RT-MATCH-DIFFERENCE-REACHABILITY]] found no grammar, admission or kernel
> rule that refuses an ordinary `Match` over an active computational scrutinee;
> [[RT-DESCENT-RETIRE-PRIOR-ART]] found that Lean and Agda **both admit it**,
> each with a passing regression case. **No further program-writing or
> rule-reading reopens THAT question.**
>
> **IT DOES NOT EXTEND TO THE SCALAR-SELECTED CELLS, and reading it as though it
> does is the live error** (Architect, `evt_7yrhr0xs81hfc`). Whether a member
> whose **selected case body is a bare scalar** is admissible **was never the
> proposition measured.** ⇒ Those cells are unmeasured on **both** axes — not
> shown well-formed, and not shown source-reachable. **That gap was `D1b`'s,
> and `D1c` made it moot for the retirement** — a question about those cells
> that does not change under removal cannot gate a removal.
>
> **The global-IR-invariant family was declined as the primary route**
> (Architect, `evt_nb12nmhd2zzk`). It survives only as a fallback, boundary-
> scoped, and only on an uncloseable refusal plus a fresh ruling. **Proposing it
> from inside this node is out of scope.**

## Why uniform lowering is not a new argument here

**Ken's ordinary `Match` lowering already has the shape**, in production. At
`core.rs:17734-17756` the producer route is tried first, and when it declines:

```rust
let lowered_scrutinee = self.lower_expr(builder, scrutinee_occurrence, env)?;
```

**That is the GHC sentence — translate the scrutinee with the ordinary
expression translator, then construct the case around the result.** And the
scrutinee form at issue has a general arm in that same translator
(`core.rs:18071-18086` → `lower_computational_match_expr`, `core.rs:3897-3932`),
which builds a `Computational` eliminator frame and hands it to the shared
producer traversal.

⇒ **This node is a totality question about code that exists**, not the
construction of a new global property. Coordinates verified by the Steward at
`main` = `d79392582`; `crates/` is unchanged since the Architect's read at
`999909207`.

## The population, stated exactly

**The difference is the heterogeneous-case-body population.** Retention fires
when the subject is an active-recursion `ComputationalMatch`
(`core.rs:2175-2184`) that `requires_heterogeneous_deforestation` declines — and
it declines because `produces_deforestable_aggregate_with_ih` is **universal
over case bodies** (`mod.rs:16688-16697`, `cases.iter().all(...)`).

⇒ **One case body that does not produce a deforestable aggregate — a bound
scalar, as `D1`'s witness had — takes the whole subject out of the producer
route.** That population is non-empty as backend IR (`D2a` constructed a
member).

**It was assumed to be what the retained lane is carrying. `D1c` REFUTED that
for the scalar rows** — the retained lane refuses them identically. **What the
lane actually carries is an open question, and `D1d` asks it for the aggregate
rows.**

**The two quantifiers do not line up and that is the whole difference:**
retention is **existential** over a per-case field; routing is **universal**
over case bodies. Do not re-derive this; it has been settled twice.

## D1 RESULT — NON-TOTAL, WITH A NAMED REFUSAL. The fallback is NOT triggered.

**Handed back at `evt_5h96qz12red0f`, measured at exact
`5c5ee5b6c37dfc0017afa547f83cd0b9aa5f5111`. There is no candidate and no diff**
— the five-row control was disposable and was fully reverted. **This section is
the only durable record of the measurement; nothing merges to carry it.**

**Coverage (`AC-2`).** The predicate-defined finite quotient, across
selected-case recursive/nonrecursive × aggregate/scalar, including **coincident
and split existential witnesses** — the two "some"s landing on the same case and
on different cases. Not a sample.

**Instrument (`AC-1`) held.** Every row carried exact pre-exclusion
`{MatchScrutineeRecursor}` under the non-short-circuiting enumerator, and the
exclusion assertion held.

| rows | selected case body | outcome |
|---|---|---|
| 2, 4, 5 | aggregate | **lower and execute**, `Returned(Int(Small(7)))` |
| 1, 3 | **scalar** | **refuse:** `UnsupportedLowering { construct: "Match", reason: "scrutinee is not a constructor value" }`, emitted at `core.rs:17959` |

> ### THE REFUSAL IS SELECTED-BODY-SENSITIVE, AND THAT IS WHY `AC-3` IS SATISFIED
>
> **A conservative fail-closed arm would refuse the whole population.** This one
> splits it on the selected body, so it is a **named refusal about a specific
> shape** — the second arm of `AC-3`, not the silent outcome it bars.
>
> **The refusal is a finding, not an acceptable resting state** — but see the
> next paragraph for what that does NOT license.
>
> **`AC-3` calls a refusal here a capability regression on the ground that the
> scalar cells compile today through the retained lane. THAT PREMISE DOES NOT
> HOLD.** Under the operator's 2026-08-15 oracle ruling — **the interpreter, not
> `RecursiveDescent`** — what the retained lane accepts is **not evidence a
> program is well-formed.** `D1` executed only the three aggregate rows.
>
> ⇒ **The regression claim is UNVERIFIED. Do not lean on it**, and do not cite
> `AC-3` as though it settles that the scalar cells must be made to compile.
> **That is exactly what `D1b` measures.**

**What the site is, verified by the Steward at `main` = `70fd2b69f`.**
`core.rs:17959` is the **terminal fallthrough of a `Lowered`-shape dispatch
chain** that runs *after* `lower_expr` has already succeeded on the scrutinee —
`Carried`, `BorrowedNativeValue`, `BorrowedOption`, …, `Bool`, then
`Constructor`, then the `Err`.

⇒ **The refusal is not about the scrutinee being a `ComputationalMatch`.** The
uniform-lowering sentence did its job: the scrutinee lowered. What has no arm is
the **resulting operand shape**. That is a narrower and more tractable gap than
"uniform lowering does not carry this population," and it is the fact the
mechanism ruling turns on.

**`D2` IS NOT AUTHORIZED AND WAS NOT ATTEMPTED.** The ring stopped at the
deliverable boundary as instructed. ⇒ **"Non-total" is established;
"uncloseable" is NOT** — those are different claims and only the first was
measured. `AC-3`'s stop-and-hand-back arm is conditioned on a refusal that
*cannot be closed*, which nobody has tested.

⇒ **The family-A fallback stays out of scope.** It is reached on an *uncloseable*
refusal plus a fresh ruling, and neither exists.

## THE MECHANISM RULING: none yet, and that is the ruling (`evt_7yrhr0xs81hfc`)

**The Architect declined to pick between "closeable" and "uncloseable", because
a third arm is live and neither of the others is answerable until it is
excluded.** Read at `70fd2b69f`.

> ### ARM 2: THE REFUSAL MAY BE CORRECT AND THE SCALAR CELLS ILL-FORMED.
>
> **`produces_deforestable_aggregate_with_ih` (`mod.rs:16667+`) matches only on
> `RuntimeExpr` structure and NEVER CONSULTS A TYPE.** The five rows were
> hand-built `RuntimeExpr` values in a disposable control, so nothing
> type-checked them either.
>
> ⇒ **A quotient defined by that predicate can contain type-incoherent members
> by construction.** *"Non-total over the predicate quotient"* and *"non-total
> over the well-formed population"* are different claims, **and only the second
> is the port's obligation.**

**The site agrees with that reading.** The chain's arms at
`core.rs:17756-17960` are `Carried`, `BorrowedNativeValue`, `BorrowedOption`,
`BoundedNat`, `StructuralNat`, `HostResult`, `DynamicConstructor`, `Bool`, then
the terminal `Lowered::Constructor` requirement. **Every arm carries constructor
identity in some form**, and the outer `Match` eliminates by constructor **name**
(`case.constructor == constructor`). A scrutinee that lowered to a scalar with no
constructor identity **has nothing to match against**.

⇒ **That is the shape of an ill-typed elimination, not of a missing capability.**
And it rules out the near miss: **Ken's scalar-represented data types already
have arms**, so *"it is a scalar"* is not by itself why it refuses.

**A gap that is easy to think is already closed.**
[[RT-MATCH-DIFFERENCE-REACHABILITY]] established that the **outer shape** is
source-admissible. **It says nothing about a member whose selected case body is a
bare scalar** — that was never its proposition. ⇒ **The scalar cells are
unmeasured on both axes: not shown well-formed, and not shown source-reachable.**

> ### IF ARM 2 HOLDS IT IS THE DISCHARGE, NOT A CONSOLATION. Not a defeat.
>
> Rows 2, 4 and 5 lower and execute. **If the scalar rows are ill-formed, the
> port is TOTAL over the well-formed difference and the retirement's obligation
> is MET.** Deleting the retained lane would then remove an **over-acceptance** —
> a lane silently compiling what the type discipline does not admit — rather than
> a capability. **`AC-3`'s regression premise dissolves: there is no capability
> to regress.** On present evidence this is the likeliest route to closing the
> campaign.

## Deliverables

**`D1` — activation measurement. Independently releasable, and size the turn
around it. DELIVERED — see the result section above.**
With the exclusion probe set for `MatchScrutineeRecursor`, take the
heterogeneous-case-body population through the ordinary route and report, per
shape: **lawful lowering, or a refusal quoted in full with its emitting site.**
This is the deliverable that decides whether `D2` is a port or a hard stop.

**`D1b` — SUPERSEDED, and its result is deliberately not carried.** It asked
whether the scalar rows are well-formed. `D1c` showed that question does not
gate the retirement. **`AC-7` through `AC-10` below are record.**

**`D1c` — DELIVERED. The production-versus-excluded differential** on the two
scalar rows. See the result section.

**`D1d` — THE RESIDUAL, AND THE ONLY LIVE DELIVERABLE. Same differential, rows
2, 4 and 5.** For each aggregate-selected row, run **production** and
**excluded** and compare the **decoded result value**, not merely that both
compile. Pre-exclusion residual must be exactly `{MatchScrutineeRecursor}` with
the `debug_assert!(was_present)` in force, as `D1c` had it.

**Report per row: identical decoded value, or the two values.** A difference is
the regression the whole campaign has been trying to rule out, and it would be
found nowhere else.

**Fixtures do not exist** — `D1` and `D1c` both cleaned up. Budget the rebuild.

**`D2` — close what `D1` opens, inside the ordinary route.** Whatever the
ordinary path lacks for this population, supply it there. **Not a new lane, not
a new global invariant, and not a conservative arm** — see `AC-3`.

**`D3` — the executable control.** A witness in the difference population
compiles **and runs** with the variant excluded, and its decoded result agrees
with the interpreter. **The interpreter is the oracle, not `RecursiveDescent`**
(operator, 2026-08-15): what a program does under the retained lane is not
evidence that it should compile.

**`D4` — record the disposition at [[RT-DESCENT-RETIRE]]'s bar**, in the same
place the previous two results were written. A result not written there leaves
the capstone barred regardless of what was built.

## Acceptance criteria

**`AC-1`. The instrument is `enumerate_recursive_descent_residuals` with the
`selector_variant_exclusion` probe (`core.rs:2410-2428`), never the
short-circuiting selector.** Its `Match` arm tests `MatchScrutineeRecursor`
**first** (`core.rs:2118-2127`), so this is the one variant the short-circuit
reads optimistically. **The probe's `debug_assert` must be left in force** — it
is what stops the measurement running on a program that never fired the variant
and reporting an ordinary functionized compile as success.

**`AC-2`. The population is covered by its predicate, not by a sample.** State
what shape space was covered and how; a handful of programs that happen to
compile discharges nothing. This is the third node in this campaign to carry
that criterion.

**`AC-3`. TOTALITY OR A NAMED REFUSAL — and a conservative fail-closed arm is
NOT an acceptable silent outcome.** The prior ports shipped three such arms on
`ProducerMatchCall` alone (`core.rs:2058-2064`). **If the refusal cannot be
closed within the port, stop and hand back.** That is a complete outcome and it
routes to the Steward.

> **THIS AC ORIGINALLY ARGUED that a refusal here is a capability regression
> *because the difference population compiles today via the retained lane*.
> THAT WARRANT IS WITHDRAWN** — the operator's 2026-08-15 oracle ruling makes
> the interpreter the oracle, so **what the retained lane accepts is not
> evidence of well-formedness.** The rest of the AC stands: a **silent**
> conservative arm is still barred, and the outcome is still totality or a
> **named** refusal. **Whether a refusal costs a real capability is `D1b`'s
> question, not an assumption of this AC.**

**`AC-4`. No deletion, and the capstone stays barred by this node's own
result.** Deleting the enum, selector, authority or lane is
[[RT-DESCENT-RETIRE]]'s act, after a ruling on this evidence.

**`AC-5`. The retention guard is untouched.** It landed narrowed at PR #2458 and
is behaviour-preserving; `!A` in production is lawful (Architect,
`evt_292zd309yvkfb`) because `B` is structurally false for the guarded subject.
**Not a proxy — an exact restriction. Do not re-litigate it.**

**`AC-6`. No-regression, in CI** (`COORDINATION §12`). Targeted local validation
only.

### `D1b`'s criteria

**`AC-7`. THREE outcomes, and they must be told apart — two of them look alike
in a log.**

| the interpreter… | means | routes to |
|---|---|---|
| **evaluates them to a value** | meaningful programs; the backend refusal is a genuine capability gap | **arm 1** — `D2` proceeds under the pre-authorized bound |
| **rejects them as ill-formed** — type error, pattern-match failure against a non-constructor, stuck term | the refusal is correct | **arm 2** — recut this node as a disposition |
| **also reports "unsupported"** | **NOTHING. This settles nothing.** | neither |

> ### `D1b` INHERITS A RULED PRECEDENT. It is not inventing a method.
>
> [[RT-DESCENT-RETIRE]] already carries this exact decision procedure, ruled and
> landed, for a **different** population — the governed rows — with the same
> oracle and the same two resolutions:
>
> | interpreter | resolution |
> |---|---|
> | **runs the rows** | the other backend must too — the refusal is a compiler defect and repairing it is convergence |
> | **refuses them** | **`RecursiveDescent` was over-accepting; its behaviour is a bug to drop, not a capability to preserve** |
>
> ⇒ **Arm 1 and arm 2 are that table's two rows.** The campaign has already
> ratified what an arm-2 result *means*, so a `D1b` that lands there is applying
> a settled consequence, **not proposing a new one.** Read `AC-7` as this table
> instantiated at the scalar-selected cells.

**`AC-8`. AN INTERPRETER "UNSUPPORTED" IS NOT A WELL-FORMEDNESS VERDICT, and
must not be recorded as arm 2.** It is an interpreter capability gap. **This is
stated as its own criterion because a refusal reads as a verdict** — it is
exactly the place a negative check passes for the wrong reason.

**`AC-9`. The source-program shortcut is POSITIVE-ONLY.** If anyone can write a
**Ken source program** that lowers to a scalar cell, that settles **arm 1**
immediately and no interpreter argument is needed. ⇒ **A failed search settles
NOTHING.** `AC-2`'s discipline carries forward unchanged, and **a fruitless
attempt may not be written up as evidence for arm 2.** This campaign has paid
twice for that distinction.

**`AC-10`. The reconstructed rows are the same rows.** They are rebuilt from
`D1`'s recorded predicate coverage, not re-derived by taste — a row that is not
the one that refused measures nothing.

### `D2`'s bound — PRE-AUTHORIZED for arm 1, so the ring waits on no second ruling

**Given now by the Architect at `evt_7yrhr0xs81hfc`. It is live only if `D1b`
returns arm 1.**

- **The chain gains a NAMED arm for the observed operand shape. No wildcard, and
  no widening of the terminal `Err`** — that refusal is the chain's fail-closed
  default and **must survive the change**. A `_ =>` here would silently absorb
  every future shape.
- **The producer route, the residual predicate and the retention guard are all
  out of scope.** This node ports; it does not re-narrow.
- **Each added arm carries a positive control PLUS the mutation showing that arm
  is what carries it** — arm removed, the named row reds; restored, green; **both
  exit codes reported. Not an argued red.**
- **A conservative fail-closed arm is not an acceptable substitute for the
  port.** If the shape cannot be lowered, **that is arm 3, not a third
  disposition invented inside `D2`.**

**Arm 3 comes back to the Architect** for the boundary-scoped family-A ruling
`evt_nb12nmhd2zzk` reserved. **Do not attempt family A inside this node, and do
not treat one failed arm as uncloseability** — those are the two claims this
handback correctly kept apart.

## Banned scope

- **The global `RuntimeExpr` invariant (family A).** Declined as the primary
  route. If `D1`/`D2` reach an uncloseable refusal, hand back — the fallback is
  boundary-scoped over what reaches native lowering entry, and it needs a fresh
  ruling before anyone builds it.
- **Re-opening source-unreachability**, by program-writing or rule-reading.
- **Deleting or re-narrowing anything**, and the retirement itself.
- **The `RecursiveDescent`-as-oracle framing** (operator, 2026-08-15).

## Sequencing

**`active`, and `D1` is DONE.** Assigned `evt_38he57edb7bpm`, handed back
`evt_5h96qz12red0f`, measured at exact
`5c5ee5b6c37dfc0017afa547f83cd0b9aa5f5111`. No candidate — see the result
section.

**`D1b` ran and is SUPERSEDED** (`evt_71v2py6b26b2` assigned). **`D1c` ran the
differential and settled the node** (`evt_1e5wbj4af4xbc`, exact `b7f65ad0c`).
Neither produced a candidate.

**`D2` is DEAD, not held.** There is no port to bound. Its pre-authorized bound
stays above as a record of what would have been authorized.

**`D1d` is the only live deliverable** — the same differential on rows 2, 4
and 5.

**The capstone is still barred, and `D1c` did NOT lift it.** The differential
covers **2 of 5 rows**. Nothing here authorizes a deletion; `D1d` completes the
evidence and the ruling on it is the Architect's.

**It blocks [[RT-DESCENT-RETIRE]] as that node's twelfth dependency**, and it is
still **the first link in the chain whose success would license deletion** —
every predecessor could only fail to. **`D1c` is the first result that points
that way, and it is half-measured.**

**Lane 1 under the operator's directive.**
