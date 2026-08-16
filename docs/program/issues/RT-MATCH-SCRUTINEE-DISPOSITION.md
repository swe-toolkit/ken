---
id: RT-MATCH-SCRUTINEE-DISPOSITION
title: "MatchScrutineeRecursor's retention guard was broader than the capability boundary it stood in for -- NARROWED to retain exactly when the ordinary producer route declines; the difference is non-empty, so the variant survives load-bearing"
status: merged
owner: runtime
size: M
gate: none
depends_on: []
blocks: [RT-DESCENT-RETIRE]
github: https://github.com/swe-toolkit/ken/pull/2458
origin: "Architect ruling evt_620806vfy5kwm (2026-08-16) on RT-DESCENT-RETIRE's D1 hard stop, verbatim: 'MatchScrutineeRecursor is UNMEASURED, and symmetry is not an argument for it. Its entire doc is one line. No reason is recorded. Nothing in the tree says whether it is an unbuilt port or a shape the functionized lane would correctly refuse. I will not rule it by analogy to its neighbour.' Population fixed by runtime measurement A/B at exact 3523868afe7cd84b47c7b07281ff7df7c3202d61 (runtime-implementer evt_4v0frfza70d2m). RECUT 2026-08-16 on Architect ruling evt_29rrwtbh48n8z: D1 returned a third disposition that neither authorized outcome covers; deliverables re-split to measure-the-difference then narrow-or-delete. Steward-filed per COORDINATION section 2."
---

> # MERGED at PR #2458. THE OUTCOME, so nobody reads the deliverables as open.
>
> **`D1`** returned a third disposition — the port is not missing and the rule is
> not a correct refusal; the guard was simply broader than the boundary it stood
> in for.
>
> **`D2a`** constructed an expression **in the difference**: a one-case
> `ComputationalMatch` with `recursive_positions=[0]` whose scalar body fails
> `produces_deforestable_aggregate_with_ih`. Retained today; with the residual
> excluded it reaches the ordinary route's exact *"scrutinee is not a constructor
> value"* refusal. **The difference is non-empty.**
>
> **`D2b`** attempted source-reachability and **honestly stopped short** — the
> program normalized before the runtime classifier, and a failed search is not
> the method gate's argument. **No unreachability or deletion claim was made**,
> and that restraint is what kept the record sound.
>
> **`D3-narrow`** landed: one shared predicate drives both the selector and the
> enumerator, retaining exactly when the ordinary producer route declines.
> Behaviour-preserving — the three intersection renderings left the residual set
> and still compile, and the executable row still returns
> `Returned(Int(Small(7)))`. The Architect additionally required a **differential
> equality control** against a constructed `Lowering`, which reds on `A` drift,
> `B` drift, **and a future third routing disjunct** — a `B == false` pin would
> not have.
>
> ## `D3-delete` WAS NOT TAKEN, and it is no longer this node's
>
> **`MatchScrutineeRecursor` survives in reduced, load-bearing form**, so
> [[RT-DESCENT-RETIRE]] may delete nothing on present evidence. **The discharge
> moved to [[RT-MATCH-DIFFERENCE-REACHABILITY]]**, filed because a bar that cites
> an un-taken branch of a *merged* node names no owner and no dispatchable work.
> **Route any future source-reachable finding there, not against this node.**

## What this node is

**`D1` is answered, and its answer is a third thing.** The frame authorized two
outcomes — an unbuilt port or a correct refusal — and the ring measured that
**neither holds**, then stopped rather than forcing a fit. That was right, and
the finding is the reason this node still exists.

**The port is not missing. All three governed renderings compile on the
functionized lane.** Measured at exact `f24ad5242f46fd36a345b7b20a46256d936caf79`
by `runtime-implementer` in thread `thr_2qv29nypwrnq0`, by one-variant exclusion
on each exact expression, read through `observed_recursive_descent_residuals()`:

| hash | observed set | forced `FunctionizedUnits` |
|---|---|---|
| `e1183de5e21770cc` | `Some({MatchScrutineeRecursor})` | compile `Ok(())` |
| `cc234d5c5f826979` | `Some({MatchScrutineeRecursor})` | compile `Ok(())` |
| `55fc4ee2ca985f82` | `Some({MatchScrutineeRecursor})` | executes `Returned(Int(Small(7)))` |

**The disposition, named by the Architect at `evt_29rrwtbh48n8z`:**

> **A retention guard whose condition is broader than the capability boundary it
> was standing in for.**

**This is a third class, distinct from both siblings, and the campaign should
stop expecting only two answers.** `LexicalCallArgumentRecursor` was an *unbuilt
port* — its own doc said the mechanism does not exist. The four
`FunctionizedUnits` refusals ruled at `evt_5h7vzc27mc11j` were *correct
refusals* — conservation laws and invariants, over-strict and sound. **Here the
mechanism exists and the guard that routes around it does not match its shape.**
The four retirements so far were all *"build the port, then delete the
variant."* This one is *"the variant was never load-bearing across its full
extent"* — a different act, with a different proof obligation.

## The two guards, and why they cannot coincide by construction

**Verified by the Steward against the objects at `f24ad5242`, not taken on
cite.**

**The RETENTION guard is EXISTENTIAL** — `lowering/core.rs:2104-2118`, the
`Match` arm of `recursive_descent_residual`:

```rust
matches!(
    scrutinee.as_ref(),
    RuntimeExpr::ComputationalMatch { cases, .. }
        if cases.iter().any(|case| !case.recursive_positions.is_empty())
)
.then_some(RecursiveDescentResidual::MatchScrutineeRecursor)
```

**The ROUTING guard is UNIVERSAL.** `lowering/core.rs:17708-17715` fires the
ordinary route on `requires_heterogeneous_deforestation(scrutinee) ||
self.declaration_call_produces_deforestable_aggregate(scrutinee)`.
`requires_heterogeneous_deforestation` (`lowering/mod.rs:16629-16636`) is a
shape test conjoined with `produces_deforestable_aggregate_with_ih`, whose
`ComputationalMatch` arm (`lowering/mod.rs:16688-16697`) reads:

```rust
RuntimeExpr::ComputationalMatch { cases, .. } => {
    !cases.is_empty()
        && cases.iter().all(|case| { /* ... */ })
}
```

**The second disjunct is dead on this population, by construction.**
`declaration_call_produces_deforestable_aggregate`
(`lowering/mod.rs:18724-18726`) opens `let RuntimeExpr::Call { callee, .. } =
expr else { return false; }` and then requires a `DeclarationRef` callee. **The
retention guard's subject is an *immediate* `ComputationalMatch`, never a
`Call`**, so that disjunct contributes nothing to any program this rule retains.

⇒ **Retention asks *"does SOME case carry recursive positions?"* Routing asks
*"does EVERY case body produce a deforestable aggregate?"* Different quantifier,
and a different subject — per-case positions versus per-case bodies. An
existential over one property cannot imply a universal over another.** The two
guards can coincide only *contingently*, if the difference happens to be
unreachable. **That measurement was attempted at `D2b`, did not settle, and is
now [[RT-MATCH-DIFFERENCE-REACHABILITY]]'s.**

### The difference, stated exactly

A program is **in the difference** when the retention guard fires and the
routing guard declines:

- the `Match` scrutinee is a `ComputationalMatch` with **non-empty** `cases`; and
- **some** case has non-empty `recursive_positions`; and
- **some** case body fails `produces_deforestable_aggregate_with_ih` under that
  case's `case_ihs`.

**The two "some"s need not be the same case, and the minimal witness is a single
case that satisfies both.** Empty `cases` is *not* in the difference: `any` over
an empty slice is false, so retention does not fire either.

**Note the asymmetric IH environments** — `requires_heterogeneous_deforestation`
enters with an empty `BTreeSet`, and the `ComputationalMatch` arm rebuilds
`case_ihs` from `recursive_positions.len()`. **A case with no recursive
positions therefore has no IHs available to its body**, which is a strong
requirement on that body and is the most likely source of a difference.

## Why the three witnesses cannot carry a deletion

**All three necessarily sit in the intersection — that is exactly why
`FunctionizedUnits` succeeded for them.** Two `Ok(())` and one executing
`Returned(Int(Small(7)))` is real evidence, and it is evidence about programs
satisfying **both** guards.

**A program in the difference is the one that decides this, and none of the three
is one.** Today the classifier retains such a program. Delete the rule and it
goes functionized, the routing guard declines it, and it falls through to
`lower_expr(scrutinee)` and the carried-arm chain that the code itself says
*"ends in 'scrutinee is not a constructor value'."*

> **Three green results in the intersection are consistent with the rule being
> redundant and equally consistent with it being load-bearing on the difference.
> They discriminate neither.** More witnesses in the intersection add nothing.

**The Architect is not ruling that the retention rule is wrong** — it may be
exactly redundant. **The ruling is that three intersection witnesses do not
establish it, and that the quantifier mismatch is a positive reason to expect a
difference rather than an absence of evidence.**

## The population, measured

**Fixed at exact `3523868afe7cd84b47c7b07281ff7df7c3202d61`**; the complete
`crates/ken-runtime` tree is identical at `dc98f6f84` and at `f24ad5242`
(tree `17246cb8615e04fd520d646eed60079ea28d06f0`), so the population is
unchanged across the whole campaign.

| # | hash | exact rendering | compiles | set |
|---|---|---|---:|---|
| 1 | `e1183de5e21770cc` | `D8d` `Match { scrutinee: px8j_deferred_recursive_field_fixture(), ... }` | 1 | {M} |
| 2 | `cc234d5c5f826979` | distinct deferred `Match` in `px8j_all_three_producer_paths...` | 1 | {M} |
| 3 | `55fc4ee2ca985f82` | `rt_match_scrutinee_recursor_executable()` | 2 | {M} |

**Row 3 is `#[cfg(test)]`** at `lowering/core/tests/control.rs:16150`, with
`scrutinee: rt_closed_active_recursor()`. Verified by the Steward.

> **The per-rendering fixture-or-source triage that the old `D2` asked for is
> DISSOLVED, not deferred.** Both lawful repairs — narrowing and deletion —
> treat the intersection identically, and all three rows are in the
> intersection. **Their provenance cannot change the act**, so triaging them
> would buy nothing. If a later finding makes it load-bearing again, it comes
> back to the Steward as a fresh deliverable rather than being reconstructed
> from this paragraph.

> **The masking hazard has not gone away.** `recursive_descent_residual`
> short-circuits, and its `Match` arm tests `MatchScrutineeRecursor` **first**,
> `.or_else`ing the rest — so under the selector a doubly-retained program
> reports **only** `M`. **This is the one variant the short-circuit reads
> optimistically. Use `observed_recursive_descent_residuals()`, never the
> selector**, including when constructing the witness below.

## Deliverables

> **ALL DISCHARGED — this section is the record of what was ASKED, in the
> imperative it was written in. The answers are in the outcome block at the top.**
> `D1` answered, `D2a` proved the difference non-empty, `D2b` stopped short
> without a claim, `D3-narrow` landed and `D3-delete` was not taken. **Nothing
> below is open work.**

**`D1` — ANSWERED. Do not re-run it.** The mechanism question is settled: the
functionized lane *does* take all three renderings, and the disposition is the
third class above. Nothing in `D1` is wasted — it establishes the intersection
cleanly and it is the reason the third disposition is visible at all.

**`D2` — MEASURE THE DIFFERENCE.** This is the measurement that unblocks the
node, and it is **not** more of the same. Two parts:

- **`D2a` — the shape.** Construct a `RuntimeExpr` in the difference as defined
  above, and establish two things about it against the tree: that the retention
  guard **does** retain it today, and that with the retention rule removed the
  ordinary route **declines** it and it reaches the carried-arm chain. This is a
  disposable probe unless a control is warranted; it settles whether the
  difference is empty *as a shape*.
- **`D2b` — the reachability.** Determine whether a source program elaborates
  into the difference **and reaches this classifier call**. This is the load
  bearing half, and the method gate below governs it in full.

**`D3` — NARROW OR DELETE, on `D2`'s result.**

- **`D3-narrow` is PRE-AUTHORIZED and needs no further ruling.** Change the
  retention condition to exactly the complement of the routing condition —
  **retain if and only if the ordinary route would not take it.** The Architect
  ruled this lawful now, with no measurement, because it is behaviour-preserving
  by construction: every program the ordinary route already handles stops being
  retained, every program it declines stays retained, and **no reachability
  claim is needed**. It also makes the guard state its actual contract instead
  of a syntactic proxy for it.
- **`D3-delete` requires a FRESH Architect ruling on `D2b`'s gate argument.**
  Deletion is lawful only on a measurement that the difference is empty or
  unreachable. Report `D2b`'s negative and stop; do not delete on it
  unilaterally.
- **Record the outcome where [[RT-DESCENT-RETIRE]]'s gate reads it**, in that
  node's terms. A finding that is measured but not recorded where the criterion
  looks leaves the lane exactly where it is.

> **If `D2b` proves hard, the narrowing is always available and always lawful.**
> It unblocks this node without the reachability argument. Prefer it over
> spending the turn on an unbounded search — a variant surviving in reduced form
> is a good outcome here and does not block the capstone differently from today.

## The method gate

> **A NEGATIVE EXISTENCE CLAIM IS NOT ESTABLISHED BY FAILED ATTEMPTS.** *"We
> wrote N programs and none reached the difference"* is consistent with the
> N+1th reaching it. **Argue from the surface grammar, the elaborator's
> admission rules and the kernel gates** — what a user can write at all — never
> from a sample of attempts. This is
> [[RT-REFUSAL-SOURCE-WITNESS-OR-INVARIANT]]'s gate, which the operator's
> 2026-08-16 instruction established and which governs `D2b` identically.
>
> **A witness is the easy direction** — one `.ken` file reaching the difference
> settles `D2` outright and routes straight to `D3-narrow`.
>
> **READ ANY REFUSAL YOU CITE FOR AN ESCAPE HATCH.** The lexical node's first
> `D1` was blocked for citing a real kernel refusal and then naming *that
> refusal's own recommended workaround* as the thing closing the route — the
> error text read *"(use ascription)"* verbatim. **A diagnostic that tells the
> user how to proceed is naming a route your claim must then close separately.**
> Name the stage that **actually** refuses, from an observed run.

## Acceptance criteria

**`AC-1`. `D2a` exhibits a concrete difference shape, or argues it is empty from
the two guards' definitions.** *"Like `LexicalCallArgumentRecursor`"* and *"like
the `FunctionizedUnits` refusals"* remain rejected. Cite by file and line.

**`AC-2`. `D2a` establishes BOTH halves on the same shape** — retained today,
and declined by the ordinary route once the rule is removed. **One half alone
does not distinguish the difference from the intersection.**

**`AC-3`. `D2b`'s reachability claim carries a gate argument and states its
population**, per the method gate. **An honest negative is an acceptable
outcome; an unstated one is not.**

**`AC-4`. No exclusion result is cited as capability evidence, and no
intersection witness is cited as evidence about the difference.** Measurement
`B` established sole-retention for all three; that is necessary and **not
sufficient**. This is the criterion the Architect's ruling turns on.

**`AC-5`. The `L` variant is untouched.** Its twelve renderings are
[[RT-LEXICAL-CALL-ARG-WITNESS-OR-PORT]]'s, and that node is `merged`. **Do not
disposition it here and do not cite this node's answer as evidence about it** —
the asymmetry runs both ways, and it runs in this direction now too.

**`AC-6`. If `D3-narrow` lands, the three hash-pinned renderings still compile
and row 3 still executes to `Returned(Int(Small(7)))`.** The narrowing is
behaviour-preserving by construction; this is the control that says so.

**`AC-7`. No-regression, in CI** (`COORDINATION §12`). Targeted local validation
only.

## Banned scope

- **Deleting the `RecursiveDescent` LANE, the selector, the authority, or the
  enum.** That is [[RT-DESCENT-RETIRE]]'s act and it is not this node's.
  **Narrowing the `MatchScrutineeRecursor` retention rule is NOT that**, and is
  authorized above; deleting the `MatchScrutineeRecursor` *variant* is in scope
  only via `D3-delete`, which needs the fresh ruling.
- **Dispositioning `LexicalCallArgumentRecursor`.**
- **More intersection witnesses.** They add nothing and the Architect said so in
  terms.
- **The `RecursiveDescent`-as-oracle framing** (operator, 2026-08-15). What a
  program does under `RecursiveDescent` is not evidence that it should compile.
  **This bites hardest on `D2a`:** a difference program's current behaviour under
  the retained lane is not an argument that the lane must keep taking it.

## Sequencing

**`merged` at PR #2458.** Published from exact
`b7bb88c6dab8b4b65886b59159943aa07d3cc9aa`, base `d44482f06`, two non-merge
commits, three paths, `+346/-157`. QA `evt_5j2pbqncf7pxd`, Decision
`dec_1g7ef2t0ddd7k` resolved by the Architect on that exact SHA.

**It was NOT the last remaining condition on [[RT-DESCENT-RETIRE]], though it
was filed as one.** The narrowing left the variant load-bearing, so the capstone
is still barred — now by [[RT-MATCH-DIFFERENCE-REACHABILITY]], which carries the
sole discharge. **The capstone stays `draft`, and `draft` there means blocked,
not unframed** — but the reason changed from *"a dependency has not landed"* to
*"the retained difference is unmeasured for source-reachability."*

**This node touched `crates/`** — the classifier change — so its merge took the
full Adversary pass, which the lexical node never owed.

**Lane 1 under the operator's 2026-08-15 two-lane directive.**
