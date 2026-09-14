---
id: RT-COMPILE-OUTCOME-RUN-CONFIGURATION-DEPENDENCE
title: "One test's compile takes a DIFFERENT lowering outcome alone than it does in its suite -- same bytes, same source, so this compile's result is not a function of its input alone, and that is a crates question rather than a harness one"
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

> # THIS IS NOT A STACK FINDING AND MUST NOT BE FILED AS ONE.
>
> It was observed in the same session as an unprovisioned-stack overflow and the
> two are easy to conflate. **They are independent.** The stack half is
> [[TEST-STATED-STACK-SITE-RECONCILE]]. This one is underneath it and survives
> the stack being fixed.
>
> **Do not run the experiment below yet.** The runtime ring's priority is
> `RT-CONSTRUCTOR-AUTHORITY-DISCHARGE`; this is filed so the frontier is written
> rather than discovered. Sequencing, not merit.

## What this is

`crates/ken-cli/tests/abi_s6_mapping_file_backed_native.rs`, one test, identical
bytes, measured at `686ffa8ac`:

    A  full 18-test suite, unprovisioned  ->  ok
    C  --exact isolation,  PROVISIONED    ->  lowering REFUSAL:
       "CheckedIhDetachedCallerCut: a post-call consumer receipt is longer than
        the exact local eliminator prefix"

Cell C is past the stack overflow — the provision removed that — so the refusal
is not a headroom artifact. **The same source takes a different compilation
outcome depending on what else ran in the process.**

## Why this is `crates/` and why it is the more consequential of the two

**It means this compile's result is not a function of its input alone.**
Something outside the test's own subject participates in whether lowering accepts
or refuses. For a compiler that is a correctness property, not a test-harness
inconvenience.

**The standing cost, stated rather than smoothed:** nobody knows how many tests
in this file pass only as suite members. **One is now measured; the other
seventeen are unmeasured.** Until someone looks, every green in the file is a
claim about the test *plus* the run configuration.

**It reaches backward into readings already relied on.** Two were taken from this
file with no run configuration recorded:

    owners=4 deferred=1 rows=0 all_rows=4    the mapping fixture SHAREDCAUSE reading
    abi_s6_mapping...:766-788                applications==1, then the constructor-
                                             result-identity expect_err

The Architect used `owners=4` twice, to argue the mapping fixture's promotion
result must not be transported to `rt_read_offset_stage`. **That use happens to
be robust** — it argued *against* carrying a reading across, and a
configuration-dependent reading is less transportable, not more. **The robustness
is luck about direction, not something anyone checked**, and it is recorded that
way on the Architect's own insistence rather than allowed to pass as verified.

## NO MECHANISM IS PROPOSED, DELIBERATELY

**Four mechanisms were proposed on the neighbouring sub-question in one
afternoon and all four were refuted**, one of them the Architect's, each reasoned
from a correlation before the control was run. **Do not open this node by
proposing a fifth.** Run the discriminating experiment first.

## Deliverables

**`D1` — the discriminating experiment: bisect suite membership, do not
theorise.** Run the target test with exactly one companion, then vary which
companion. The three outcomes separate the hypothesis families before anyone
guesses:

    ANY single companion makes it pass   -> a warm-up effect; the state is established
                                            by running almost anything
    a SPECIFIC companion makes it pass   -> that test names the state, and the pair is
                                            the reproduction
    no companion makes it pass           -> it is the suite's aggregate condition, and
                                            the hypothesis space is different again

Cheap, bounded, and it does not require a mechanism first.

**`D2` — the suite-membership census.** For each of the 18 tests in the file:
does it pass in isolation? **Report the count, not a characterization.** This is
the measurement that turns "every green in this file is a claim about the run
configuration" from a hazard into a number.

**`D3` — the mechanism, only after `D1`.** Named with the evidence that
distinguishes it from the families `D1` did not select.

## Acceptance criteria

**`AC-1` — `D1`'s outcome is reported even when it selects no family.** "No
companion makes it pass" is a result, not a failure, and it is the one that most
changes what comes next.

**`AC-2` — the census in `D2` is per-test and complete over the file**, with the
provisioning state of each run stated. A test that could not be run in isolation
for an unrelated reason is named, not omitted.

**`AC-3` — no test's assertions are changed to make it isolation-safe.** If a
test only passes as a suite member, that is the finding. **Repairing the symptom
destroys the evidence**, and a green suite afterwards proves nothing.

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
