---
id: RT-LEXICAL-CALL-ARG-WITNESS-OR-PORT
title: "LexicalCallArgumentRecursor's twelve fixed renderings are fixture-only -- no port is owed and the variant is re-described with the retained lane"
status: active
owner: runtime
size: M
gate: none
depends_on: []
blocks: [RT-DESCENT-RETIRE]
github: null
origin: "Architect ruling evt_620806vfy5kwm (2026-08-16) on RT-DESCENT-RETIRE's D1 hard stop: the disposition axis is the RecursiveDescentResidual variant, not the expression; LexicalCallArgumentRecursor is an incomplete port with capability owed; the cut is one node keyed on the variant whose first deliverable is a per-expression fixture-or-production triage. Population fixed by runtime measurement A/B at exact 3523868afe7cd84b47c7b07281ff7df7c3202d61 (runtime-implementer evt_4v0frfza70d2m, runtime-leader evt_1d5wb0t98jadx). Steward-filed per COORDINATION section 2."
---

## What this node is

**`RT-DESCENT-RETIRE`'s `D1` found this variant live only in fixtures.**
Production selects `BodyEmissionAuthority::RecursiveDescent` 31 times across
20 tests, and 27 of those selections carry
`LexicalCallArgumentRecursor`. This node's corpus-independent `D1` then
measured all twelve fixed renderings as fixture-only under the current
production source path: source reachability is `0/12`, accounting for `0/27`
measured compiles.

**No lexical port is owed.** This is a measured conclusion, not a capability
decision: kernel definition admission rejects the required source shape before
native preparation or Runtime lowering, so there is no source-reachable
population for the missing port to serve. `LexicalCallArgumentRecursor` is
therefore re-described as fixture-only with the retained lane and retires when
[[RT-DESCENT-RETIRE]] removes that lane. This node changes no production code.

**The variant's own doc correctly records that the port was never built** —
`lowering/core.rs:2005-2011`, verbatim:

> The recursive result still carries invocation-local scope/return-hole state.
> Passing it through a separately declared lexical unit is **not one of the
> completed functionized ports**, so the established recursive descent lane
> retains the whole call.

**That text identified a missing mechanism, not a conservation law or an
invariant.** Before `D1`, it correctly opened a capability question. `D1` closes
the population question without pretending the mechanism exists: all twelve
renderings are fixtures, so the unbuilt port has no current user-program
surface. This disposition remains separate from `MatchScrutineeRecursor`, whose
three renderings belong only to [[RT-MATCH-SCRUTINEE-DISPOSITION]].

## The population, measured, and it is a true partition

**Fixed at exact `3523868afe7cd84b47c7b07281ff7df7c3202d61`**; the complete
`crates/ken-runtime` tree is identical at `dc98f6f84`, so this table is current.
`L` = `LexicalCallArgumentRecursor`, `M` = `MatchScrutineeRecursor`.

| # | hash | exact rendering | compiles | set | provenance |
|---|---|---|---:|---|---|
| 1 | `7433055269044ce8` | row1 `host_result_closure_match(px8j_layered_recursive_result(1, 1))` | 3 | {L} | `#6d` live |
| 2 | `c1294e143381564e` | row2 `host_result_closure_match(recursive_computational_result_depth(2, Result::Ok(Unit)))` | 3 | {L} | [[RT-LEXICAL-ROW2-MISSING-MINT]] |
| 3 | `23fad2ab9d295856` | row3 `host_result_closure_match(px8j_recursive_sibling_result(1, 2, px8j_aggregate_result()))` | 2 | {L} | `#6d` live |
| 4 | `a26749baed91331f` | row4 depth1 `host_result_closure_match(px8j_scope_chain_observation_result(1, 0))` | 2 | {L} | `#6d` live |
| 5 | `de31e8ed184a5754` | row4 depth2 equivalent | 3 | {L} | `#6d` live |
| 6 | `e365f91d10c12a7c` | row4 depth3 equivalent | 3 | {L} | `#6d` live |
| 7 | `25c3d81c8054e552` | row5 before-hole `host_result_closure_match(px8j_equal_payload_hole_placement(BeforeReturnHole))` | 4 | {L} | [[RT-LEXICAL-R3-FUSION-EMITTER]] |
| 8 | `3db7ba503cbf472e` | row5 after-hole equivalent | 2 | {L} | `#6d` live |
| 9 | `e8da0476e3b56008` | ordinary-frame aggregate `host_result_closure_match(recursive_computational_result(Result::Ok(Unit)))` | 1 | {L} | **outside `#6d`** |
| 10 | `4dc485d6d0951b49` | dynamic HostResult `host_result_closure_match(recursive_computational_result(console ...))` | 1 | {L} | **outside `#6d`** |
| 11 | `aa5333756917f356` | `d8_recursive_computational_revisit_with_join()` | 2 | {L} | **outside `#6d`** |
| 12 | `b6d242da59f49ca2` | `rt_lexical_call_argument_recursor_executable()` | 1 | {L} | **outside `#6d`**, known `#[cfg(test)]` |

**Twelve renderings, 27 compiles. Six of them are `#6d`'s live population** —
rows 1, 3, 4 at all three depths, and row 5's after-hole. **Rows 2 and 7 spun
out to their own nodes and are named separately above**, per
`RT-LEXICAL-R3-FUSION-EMITTER.md:228`. **All four owner nodes are `merged`**, so
no live node claims any of these twelve.

### The partition is measured, and the instrument was the wrong one first

**Measurement `A` found the original `D1` probe read the SHORT-CIRCUITING
selector.** It printed after `select_body_emission_authority` returned, and
production selection at `core.rs:2409-2415` calls `recursive_descent_residual`
plus a declaration `find_map`, both short-circuiting. In the `Match` arm
`MatchScrutineeRecursor` is tested first and `.or_else`s the rest, **so a
program retained by both variants reports only `M` and lexical retention is
masked.** The hazard was real and the tree names it:
`ResidualEnumerationMutation::ShortCircuitLikeTheSelector` (`core.rs:2186-2190`).

**The re-read against the non-short-circuiting enumerator
(`enumerate_recursive_descent_residuals`, `core.rs:2180`, recorded at
`core.rs:2625-2628`) found ZERO dual-retained renderings.** Removing `L` leaves
exactly the three `M` renderings; removing `M` leaves exactly these twelve.
**So 27/4 is a true partition and 27 is not a lower bound.**

> **Do not read that as "the selector was fine."** It was the wrong instrument
> and the number happened to be tight. The set-valued observation is the
> established currency — `#6d` stated its own population in set language — and
> the exact-set control `d3_the_exact_set_control_still_reds_under_short_circuiting`
> (`lowering/core/tests/control.rs:16959`) exists to keep it that way. **A future
> measurement that reaches for the selector is repeating the defect, whatever
> this one's outcome was.**

### What measurement `B` bought, stated precisely because it is easy to over-read

**Every one of the fifteen renderings is singly retained**, so for each of these
twelve the excluded-variant probe's `FunctionizedUnits` result **is
sole-retainer evidence.** That closes the silent blindness the Architect
flagged: the probe's `debug_assert` checks only that the excluded variant was
*present*, not that it was the *sole* retainer.

⇒ **Sole-retention is necessary, and it is not sufficient.** An exclusion
returning `FunctionizedUnits` says the classifier no longer retains the program;
it does **not** say the functionized lane can emit it. **It is not capability
evidence** and no deliverable here may treat it as such.

## The method gate. Read this before designing `D1`.

**`D1` is the operator's own test**, already landed once as
[[RT-REFUSAL-SOURCE-WITNESS-OR-INVARIANT]] (PR #2440) — operator, 2026-08-16:
*"look for a ken source program that could generate that state, or declare that
such a program did not exist."* That node applied it to **refusals**. This one
applies it to **selections**. Same method, same evidentiary bar, new population.

> ### A NEGATIVE EXISTENCE CLAIM IS NOT ESTABLISHED BY FAILED ATTEMPTS.
>
> *"We wrote N programs and none reached it"* is consistent with the N+1th
> reaching it. **A "no source program reaches this" disposition must be argued
> from the surface grammar, the elaborator's admission rules, and the kernel
> gates** — the things that decide what a user can write at all — not from a
> sample of attempts, however large. This is
> [[RT-REFUSAL-SOURCE-WITNESS-OR-INVARIANT]]'s method gate, restated because it
> governs here identically.
>
> **A positive answer is the easy direction and needs no such care** — one
> `.ken` file reaching the shape settles that row outright.

**Finding witnesses is the better outcome.** A witness converts an open question
into a sized capability gap; "no witness" is the expensive claim to establish.
**Do not treat "these are all fixtures" as the target.**

## What is already known about the triage, and what it does not settle

**Row 12 is settled.** `rt_lexical_call_argument_recursor_executable()` is
`#[cfg(test)]` at `lowering/core/tests/control.rs:16166-16167`, doc-commented
*"`D1` position B: a lexical unit call whose argument is an active recursor."*
**A fixture authored to occupy the position. No production capability is owed
for it.** Verified by the Architect and independently by the Steward.

**Nothing else is settled, and one fixture out of twelve does not make the
others fixtures.**

> **Ten of the twelve share the outer shape `host_result_closure_match(<recursive
> producer>)`** — rows 1-10. **That is an observation about syntax, and it does
> not predict the triage answer.** Whether a user can write the shape is decided
> by the grammar and the admission rules, not by how many fixtures share a
> constructor. **Do not let the shared shape collapse ten rows into one
> judgement** — and equally, do not re-derive ten separate grammar arguments if
> one gate genuinely governs them all. Establish which, and say so.

## D1 result: all twelve exact renderings are fixtures

**Measured at exact
`ddd29db514c4eedf2eb48f50581fe9371964e5e6`.** The disposition is over every
Ken program admitted through the production native-build path, not over a
searched source corpus. One compiler/kernel gate governs all twelve hashes:

1. `LexicalCallArgumentRecursor` requires one immediate `RuntimeExpr::Call`
   whose callee is `RuntimeExpr::LexicalClosure` and whose argument is an
   immediate recursive `RuntimeExpr::ComputationalMatch`
   (`lowering/core.rs:2131-2142`).
2. The direct structural route is an immediate checked-core `Application`
   whose function is a `Lambda`: plan-aware erasure maps those two arms to
   `Call` and `LexicalClosure` (`erasure.rs:2505-2535`).
3. A bare source-written lambda cannot occupy that application head: both the
   elaborator and kernel require an expected type for a lambda. Ascription does
   let surface elaboration construct the checked-core `Application(Lambda,
   ...)`; it is not the refusal. The exact spelling
   `((\value. Success) : Nat -> ExitCode) Zero`, embedded as `host_exit`'s
   argument in an otherwise ordinary `fn main`, was run through `ken
   native-build`. Native build refused it during kernel definition admission as
   `elaboration failed: KernelRejected` with the kernel's non-inferable-lambda
   message. `elaborate_v0` produces the checked body before calling
   `declare_def` (`elab.rs:7087-7117`); `declare_def` re-checks that body
   (`check.rs:1082-1111`), where application inference first infers its function
   and the immediate `Lambda` is deliberately non-inferable
   (`check.rs:251-256`, `:358-365`). This is after surface elaboration and before
   native preparation or Runtime lowering. No Runtime compilation row exists,
   so the observed triple is `residuals=<not reached>`, `selector=<not
   reached>`, `authority=<not reached>`.
4. The only apparent indirect route is an application whose function is a
   direct declaration call that erasure could inline to a lambda. Naming the
   lambda does not preserve that route. Before checked-core selection, native
   preparation replaces recursive members with opaque barriers and normalizes
   every executable body
   (`compiler_driver.rs:2179-2183`, `:2245-2252`). Acyclic lambda helpers beta
   reduce into their callers; recursive heads remain declaration references.
   Neither can reach erasure as the immediate `Application(Lambda, ...)`
   required above.

The fourth clause is load-bearing. A valid source probe combined a named
lambda with a real nested-result computational match. Preparation normalized
the call to an ordinary `Match`; the production census reported
`residuals=[]`, `selector=true`, `authority=FunctionizedUnits`. This is a
discriminator for the normalization account, not the basis of the negative
existence claim.

| row | hash | D1 disposition | governing gate |
|---:|---|---|---|
| 1 | `7433055269044ce8` | fixture | kernel definition admission rejects `App(Lam, ...)` |
| 2 | `c1294e143381564e` | fixture | kernel definition admission rejects `App(Lam, ...)` |
| 3 | `23fad2ab9d295856` | fixture | kernel definition admission rejects `App(Lam, ...)` |
| 4 | `a26749baed91331f` | fixture | kernel definition admission rejects `App(Lam, ...)` |
| 5 | `de31e8ed184a5754` | fixture | kernel definition admission rejects `App(Lam, ...)` |
| 6 | `e365f91d10c12a7c` | fixture | kernel definition admission rejects `App(Lam, ...)` |
| 7 | `25c3d81c8054e552` | fixture | kernel definition admission rejects `App(Lam, ...)` |
| 8 | `3db7ba503cbf472e` | fixture | kernel definition admission rejects `App(Lam, ...)` |
| 9 | `e8da0476e3b56008` | fixture | kernel definition admission rejects `App(Lam, ...)` |
| 10 | `4dc485d6d0951b49` | fixture | kernel definition admission rejects `App(Lam, ...)` |
| 11 | `aa5333756917f356` | fixture | kernel definition admission rejects `App(Lam, ...)` |
| 12 | `b6d242da59f49ca2` | fixture | same gate; additionally authored under `#[cfg(test)]` |

The source-reachable D1 population is therefore zero of twelve exact
renderings, accounting for zero of the 27 measured compiles.

**Hash/rendering control.** The complete `crates/ken-runtime` tree is
`17246cb8615e04fd520d646eed60079ea28d06f0` at both the fixed measurement base
`3523868afe7cd84b47c7b07281ff7df7c3202d61` and this D1 base. The normalized
`hash<TAB>rendering` population is therefore byte-identical: the twelve hashes
above are the same twelve expressions, not reconstructed analogues. The three
`MatchScrutineeRecursor` renderings remain outside this disposition.

## D2/D3 closeout: measured fixture population, repaired retirement gate

**`D2` takes the all-fixture branch.** All twelve fixed lexical renderings are
fixture-only under the current production source path: source reachability is
`0/12`, accounting for `0/27` measured compiles. This is a **measured
conclusion, not a capability decision**. There is no source-reachable
population for the missing lexical port to serve, so no port is owed and
`LexicalCallArgumentRecursor` is re-described as fixture-only with the retained
lane for [[RT-DESCENT-RETIRE]]. The lane, selector, residual enum, and production
code remain untouched here; the retirement node owns their deletion.

The conclusion is conditional on the measured kernel-definition-admission
gate. A future kernel completeness change that admits or normalizes the
ascribed `Application(Lambda, ...)` shape re-opens this twelve-row disposition.

**`D3` records that `#6d` was deliberately scoped, not refuted.**
[[RT-LEXICAL-RECURSOR-CONSUMERS]] accurately governed its rows-1-5 population;
it never claimed the complete `LexicalCallArgumentRecursor` population. The
defect was the retirement gate consuming that scoped claim as variant-wide.
That gate is repaired by [[RT-DESCENT-RETIRE]]'s published `depends_on` edge to
this node and this variant-wide, hash-preserved measured population. No
deliverable remains delegated to `#6d` for this disposition.

`MatchScrutineeRecursor` remains outside this result. Its separate dependency
[[RT-MATCH-SCRUTINEE-DISPOSITION]] is the only authority for that variant; this
fixture-only result must not be carried across by symmetry.

## Deliverables

**`D1` — the per-expression triage. This is the node's first deliverable and
the Architect ruled it so.** For each of the twelve, record **fixture** or
**source-reachable**:

- **Source-reachable** — name the `.ken` file in the tree that reaches it and
  the exact behaviour observed. This is a real capability gap; size it, do not
  repair it here.
- **Fixture** — argue from the grammar, the elaborator's admission rules or a
  kernel gate that no source program reaches the shape, and **state the
  population the argument is over.**

**Group the rows where one gate governs several**, and say which rows it
governs. A single gate argument covering ten rows is a better result than ten
arguments; a single argument *asserted* to cover ten is worse than none.

**`D2` — the disposition of the variant, written where the retirement's gate
reads it.** From `D1`'s twelve answers:

- **If every row is a fixture**, the variant is retired or re-described with the
  lane, exactly the disposition class from [[RT-RECURSOR-TRANSPORT]], and no
  port is owed. Record it and hand back.
- **If any row is source-reachable**, the port is owed for a population `D1` has
  now measured. **Report the sized gap and STOP** — see the stop condition.

**`D3` — the `#6d` gate repair, and this node's existence is most of it.** The
retirement gate made `#6d` closure the precondition for retiring
`LexicalCallArgumentRecursor`. **`#6d`'s "rows 1-5 only" was a deliberate
narrowing from a known nine-expression census, accurate about the scope it
named** (`RT-LEXICAL-RECURSOR-CONSUMERS.md:11`) — **so `#6d` is not refuted; the
gate that consumed a scoped claim as if it were variant-wide is the defect.**
Repair it by naming the population the gate actually governs: **these twelve
renderings, this node, and the `M` variant's separate disposition.**

## Acceptance criteria

**`AC-1`. All twelve rows carry a disposition.** A partial answer is reportable
and is a real result; the variant is not dispositioned until all twelve are
recorded.

**`AC-2`. Every "no source program reaches this" carries a gate argument, not an
attempt count.** Name the grammar production, admission rule or kernel gate.
**A disposition resting on "we tried and could not" is not accepted.**

**`AC-3`. Every negative claim states the population it is over** — the measured
corpus, a superset, or a corpus-independent argument. **The claim and its
population travel together**; this is the walk-back that produced
[[RT-REFUSAL-SOURCE-WITNESS-OR-INVARIANT]].

**`AC-4`. Every witness is an actual `.ken` file in the tree**, not a described
program, and the handback names the file and the exact observed behaviour.

**`AC-5`. No exclusion result is cited as capability evidence.** Measurement `B`
established sole-retention for all twelve. **Sole-retention is not the ability
to emit**, and an AC discharged by an exclusion probe returning
`FunctionizedUnits` is discharged on the wrong measurement.

**`AC-6`. The twelve exact renderings are preserved by hash**, as measurement
`A` preserved them: normalize `hash<TAB>rendering` and diff against this table.
**A row re-identified by description rather than by hash is a reconstructed
analogue**, and the population stops being comparable across nodes.

**`AC-7`. The `M` variant is untouched.** Its three renderings are
[[RT-MATCH-SCRUTINEE-DISPOSITION]]'s and it is **unmeasured** — the Architect
declined to rule it by symmetry with this one. **Do not disposition it here, and
do not cite this node's answer as evidence about it.**

**`AC-8`. No-regression, in CI** (`COORDINATION §12`). Targeted local validation
only. **This node may land measurements and dispositions and no production
change at all** — that is a success, not a thin result.

## Banned scope

- **Building the port.** Whether it is worth building is **scope — the Steward's
  and the operator's — and it is explicitly not ruled** (Architect,
  `evt_620806vfy5kwm`). It is decided after `D1` says how many rows are
  production-reachable at all.
- **Retiring anything.** [[RT-DESCENT-RETIRE]] deletes the selector, the enum,
  the authority and the lane.
- **Dispositioning `MatchScrutineeRecursor`**, by symmetry or otherwise.
- **Repairing a witnessed gap in the same candidate that finds it.** A witness
  changes the disposition from fixture to capability gap; the repair is sized
  and routed on its own.
- **The `RecursiveDescent`-as-oracle framing.** Operator, 2026-08-15:
  *"`RecursiveDescent` should not be taken as de facto spec... The key oracle is
  not `RecursiveDescent`, but the interpreter."* **What a program does under
  `RecursiveDescent` is not evidence that it should compile.** For this node the
  question is narrower still: can a user write it at all.

## Stop condition — return to the Steward

**If `D1` finds any row source-reachable, report the sized gap and stop.**
Building the port is a scope call the ring does not hold. The Architect ruled
the disposition; he deliberately did not rule whether the capability is worth
building, and that decision needs `D1`'s count in front of it.

## Sequencing

**`ready` at filing, `depends_on: []`.** Nothing gates it — the population is
measured, fixed by hash, and named above.

**It blocks [[RT-DESCENT-RETIRE]]**, whose `D1` hard-stopped on exactly this
variant. The capstone also needs [[RT-MATCH-SCRUTINEE-DISPOSITION]]; the two are
independent and the retirement needs both.

**Lane 1 under the operator's 2026-08-15 two-lane directive**, and the campaign
`docs/program/16-recursive-descent-retirement.md` remains the priority path.
