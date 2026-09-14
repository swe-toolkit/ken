---
id: RT-COMPILE-OUTCOME-RUN-CONFIGURATION-DEPENDENCE
title: "A live CheckedIhDetachedCallerCut lowering refusal causes 8 of 11 failures in abi_s6_mapping_file_backed_native and gates the detached-caller family -- which is the population RT-CONSTRUCTOR-AUTHORITY-DISCHARGE D2 needs, so a refusal nobody has adjudicated is emptying an in-flight node's population. RE-SCOPED: the original run-configuration-dependence claim in this node's id is REFUTED"
status: draft
owner: runtime
size: M
gate: architect
depends_on: []
blocks: []
github: null
tier: T1
origin: "Routed to the Steward by the Architect at evt_11ry996w02kfh as finding (ii) of a two-part finding, explicitly split from the stack half so the two do not travel together: 'a compile whose outcome depends on what ran before it is a crates question and does not belong under a harness heading.' Observed by the runtime implementer as cell C of the four-cell table (evt_6tk9xskk67jza) while measuring something else; durable at target/D2-ASSIGNMENT.md."
---

> # THE HEADLINE IN THIS NODE'S OWN TITLE IS REFUTED. THE ID IS NOW A MISNOMER
> # AND IS KEPT DELIBERATELY — READ THIS BLOCK BEFORE ANYTHING BELOW IT.
>
> **Re-scoped 2026-09-14 by the Steward, one day after filing, from the ring's
> measurement at `evt_2q1ggxnpemwyn`.** This node was filed on the claim that one
> test takes a *different compilation outcome* alone than it does in its suite.
> **That claim's entire evidence base is gone:**
>
> - **The refusal is NOT isolation-only.** Nine occurrences in the FULL suite
>   once provisioned. It was invisible before only because the unprovisioned run
>   died earlier.
> - **Cell A (`full suite, unprovisioned -> ok`) does not reproduce** on a
>   byte-identical tree. It carried no SHA when recorded.
>
> ⇒ **With both cells of the contrast withdrawn, there is no measured
> configuration-dependence left.** A compile whose result is not a function of
> its input alone would be a serious correctness property; **nothing currently
> shows this one is that.** Do not open this node by defending the title.
>
> **WHAT THE NODE IS NOW — the refusal itself, which is the real finding and is
> larger than the one it replaces.** At `686ffa8ac`,
> `abi_s6_mapping_file_backed_native` runs **7 passed / 11 failed** provisioned,
> and **8 of those 11 are one lowering refusal**:
>
>     CheckedIhDetachedCallerCut: a post-call consumer receipt is longer than
>     the exact local eliminator prefix
>
> **That refusal gates the detached-caller family — which is exactly the
> population `RT-CONSTRUCTOR-AUTHORITY-DISCHARGE` `D2` needs.** `required` is
> non-empty only when a generated context carries a declared result contract,
> gated on `has_detached_return`; the programs that would populate `D2` are the
> programs whose compile aborts on this refusal. **A live refusal is emptying the
> population of a node already in flight.** No other node owns it (checked), so
> it is folded here rather than proliferated.
>
> **The id stays.** Renaming costs a file move, a tracker regen and every
> inbound link, and buys a tidier name; this block is cheaper and louder.
> Interrogate that trade before reversing it — the constraint demanding a new id
> is aesthetic (`ken-steward` §4c).
>
> **Still not the ring's next node.** `RT-CONSTRUCTOR-AUTHORITY-DISCHARGE` is the
> priority. Sequencing, not merit.

## What this is

**A live lowering refusal causing 8 of 11 failures in a base fixture, gating the
detached-caller family.** Measured at `686ffa8ac` by the runtime ring.

**Why it matters beyond its own reds:** it is upstream of `D2`'s population. Any
node that measures the generated-Result proof path at this fixture measures
**zero**, because the programs never survive lowering to get there — which is
precisely what cost four instrument passes on 2026-09-14 before anyone looked at
the call graph.

## Why this is `crates/`

**A lowering refusal is the compiler declining to compile a valid-looking
program.** Whether `CheckedIhDetachedCallerCut` is correct to fire on these
eleven is a question about the checked-IH consumer rules, not about a harness.
Either the programs are genuinely ill-formed — and the fixture encodes an
expectation the language no longer honours — or the refusal is over-strict and is
rejecting the detached-caller family wholesale. **Both answers are
consequential and neither is currently established.**

**State the DIRECTION when you answer it:** over-strict (rejects well-formed
programs, a completeness bug) versus under-strict (admits ill-formed ones, a
soundness bug). They have opposite repairs, and "the refusal is wrong" does not
say which.

**It reaches backward into readings already relied on.** Two were taken from this
file with no run configuration recorded — and that concern SURVIVES the re-scope,
because the file is now known to be 11-of-18 red at base, which is a stronger
reason to distrust a reading taken from it than configuration-dependence ever
was:

    owners=4 deferred=1 rows=0 all_rows=4    the mapping fixture SHAREDCAUSE reading
    abi_s6_mapping...:766-788                applications==1, then the constructor-
                                             result-identity expect_err

The Architect used `owners=4` twice, to argue the mapping fixture's promotion
result must not be transported to `rt_read_offset_stage`. **That use happens to
be robust** — it argued *against* carrying a reading across, and a
configuration-dependent reading is less transportable, not more. **The robustness
is luck about direction, not something anyone checked**, and it is recorded that
way on the Architect's own insistence rather than allowed to pass as verified.

## Deliverables

**REPLACED 2026-09-14 with the re-scope. The former `D1` was a suite-membership
bisect and the former `D3` a mechanism for configuration-dependence; both
answered the refuted question and are deleted rather than kept beside these.**

**`D1` — the refusal census, under `RUST_MIN_STACK=268435456`.** For each of the
18 tests: passes, or fails and with which error. **Report counts, not a
characterization.** The env var removes the stack variable with **zero source
edits** — no fixture change, no contention with R1 at `:565` or red #2 at `:719`.
The ring's reading is 7 passed / 11 failed with 8 sharing this refusal;
**re-derive it at your own base rather than inheriting those numbers.**

**`D2` — is the refusal correct on these programs?** For a representative failing
case, decide whether the post-call consumer receipt genuinely exceeds the exact
local eliminator prefix, or whether the check is over-strict. **Answer with the
DIRECTION named** (see above). One worked case beats a survey.

**`D3` — the reach, which is why this is not just a red fixture.** Does this
refusal gate the whole detached-caller family, or only these programs? **This is
the deliverable that matters to `RT-CONSTRUCTOR-AUTHORITY-DISCHARGE`** — that
node's `D2` population is gated on `has_detached_return`, so a refusal covering
the family empties it. Report as: which programs can reach a non-empty
`required` at this base, and which cannot.

**Do not propose a mechanism before `D1`.** Five mechanisms were proposed across
this arc on 2026-09-14 and every one was refuted, each reasoned from a
correlation before a control was run — one of them the Architect's, one the
Steward's frame. **The population-is-zero finding was reached by reading the call
graph, which nobody had done.** Read the call graph first.

## Acceptance criteria

**`AC-1` — `D1`'s census is reported as counts, with the error text for each
failure.** A failure recorded as "fails" without its error cannot be attributed
to this refusal rather than to a neighbour, and the whole point of `D1` is the
attribution. **A count of 18 passing is a result too** — it would mean the base
red is configuration-bound after all, and that is the one outcome that most
changes what comes next.

**`AC-2` — the census in `D1` is per-test and complete over the file**, with the
provisioning state of each run stated (`RUST_MIN_STACK` value, or none). A test
that could not be run for an unrelated reason is named, not omitted. **State
whether you re-derived the 7/11 split or inherited it** — it is the ring's
reading, not yet re-measured.

**`AC-3` — no test's assertions are changed to make it pass.** If a program is
refused, that is the finding. **Repairing the symptom destroys the evidence**,
and a green suite afterwards proves nothing. This node measures and attributes;
it does not repair the refusal (see "Not this node").

**`AC-4` — the two backward-reaching readings above are re-taken with the run
configuration recorded**, or explicitly declared still-unverified. **Do not
retroactively bless them from a mechanism.**

**`AC-5` — no-regression, in CI.** `COORDINATION §12` — the venue is CI, never a
local `--workspace` run.

## Sizing

**`M`**, and the one-hour target applies to `D1` alone. If the bisect runs long,
hand back what it selected and stop — `D2` and `D3` both depend on it.

## Contention

`crates/ken-cli/tests/abi_s6_mapping_file_backed_native.rs` is the file
`RT-CONSTRUCTOR-AUTHORITY-DISCHARGE` R1 and the red-#2 repair both edit, at
`:565` and `:719`. **Re-derive the intersection at candidate time.** `D1` and
`D2` are read-only over that file and should not need to touch it at all; if a
deliverable here starts editing it, that is a scope escalation to the Steward.

## Not this node

- **Not the stack provisioning.** See [[TEST-STATED-STACK-SITE-RECONCILE]].
- **Not the satisfier-disjointness measurement.** See
  [[RT-FORWARDING-PROOF-SATISFIER-DISJOINTNESS]].
- **Not a repair of the refusal itself.** Whether
  `CheckedIhDetachedCallerCut` is correct to fire is a separate question from why
  it fires in one configuration and not another.
