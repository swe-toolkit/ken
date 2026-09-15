---
id: RT-NATIVE-COMPILE-RUNS-AT-THE-STACK-WALL
title: "Roughly 94 percent of the native compile's 2048 KiB stack budget is consumed BEFORE lowering begins: a compile that rejects before native lowering needs (1920, 1952] KiB and one that lowers, emits and executes needs (1920, 1984], a difference inside the brackets' own granularity. Lowering is a rounding error on a fixed prefix every compile pays. So a candidate adding 64 KiB aborts and looks guilty while not being the cause. THIRD measured instance; LANG-PRELUDE-ELABORATION-DEPTH named this candidate a month ago and is merged."
status: draft
owner: runtime
size: M
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "Measured by runtime-implementer 2026-09-15 while diagnosing the class-1 SIGABRT on PR #3676. This node was FILED WITH THE OPPOSITE CONCLUSION at 3603991f2 and is rewritten, not amended -- its floor-side evidence was two #[ignore]d tests that never executed (harness: '0 passed; 1 ignored; 0.00s'). Architect caught the ignore census (evt_3zv3c7ya7jkzr chain); implementer withdrew the refutation and re-measured on a verified-live row. Funding is an operator scope call and is NOT taken here."
---

## The finding

**The budget is gone before lowering starts.** Every figure is from binary
search on `RUST_MIN_STACK` against the base `1dec48f33`, with no candidate
commits present, and **every probe carries its executed count.**

    REJECTS BEFORE LOWERING -- verified live, "1 passed; 0 failed;
    0 ignored", 0.23s at default
      nondecreasing_cycle_is_rejected_before_native_lowering   (1920, 1952]

    LOWERS, EMITS AND EXECUTES
      complete_carried_mapping_access_matrix_matches_the...    (1952, 1984]
      window_direct_map_bytes_executes_natively                (1920, 1984]
      nested_former_fold_executes_natively                     (1920, 1984]
      multiple_steps_preserve_the_recursive_payload            (1920, 1984]

⇒ **Doing no lowering at all and doing all of it differ by less than the
granularity of the brackets** -- tens of KiB out of nearly two megabytes.
**Roughly 94% of the budget is a fixed prefix every compile pays**, and lowering
is a rounding error on top of it.

The default is bracketed to `(1536, 2048]` by the same instrument and pins to
2048 only via the documented `std::thread` value -- **that last step is
documentation, not measurement.**

## EVERY NUMBER ABOVE IS A DEBUG BUILD. RELEASE IS UNMEASURED.

**Every stack measurement in this investigation, and in both merged precedent
nodes, was taken under `cargo test` -- an unoptimized build.** The variable was
never varied. `LANG-PRELUDE-ELABORATION-DEPTH` says in its own body:

> in an unoptimized build a new arm's locals in `check` are paid by every call
> regardless of which arm runs

⇒ **The severity of this condition for shipped compiles is unmeasured**, and
that is a property of the measurement configuration being read as a property of
the subject -- the same class as the `#[ignore]` confound, one axis over. It is
**not** an attribution claim and does not weaken `AC-5`; it names a condition
nobody controlled.

**The discriminator, with both readings fixed in advance** (Architect,
`evt_7kxt44edjhjwq`). Subject: the `nondecreasing_cycle` row -- live, executed,
never lowers, already bracketed in debug.

    requirement COLLAPSES in release   frame-size dominated. A CI and test-
                                       infrastructure condition carrying a
                                       stated-bound obligation. The SMALLER
                                       program of work.

    requirement HOLDS in release       depth dominated and live for users. The
                                       2 MiB default is a product constraint,
                                       not a test artifact. The LARGER one.

**This is the question the funding call turns on**, and it is two probes on an
instrument that already exists.

## What this explains, and what it costs

**The cluster.** Four unrelated programs landing inside one 64 KiB window at the
top of the budget is exactly what a fixed shared prefix produces: everyone pays
~1920, everyone lands just under 2048, and the program's own contribution is too
small to separate them. **The cluster was reported twice as awkward for the
then-current thesis, and the awkwardness was the signal.**

**The candidate is already filed and merged.** `LANG-PRELUDE-ELABORATION-DEPTH`
states that *every compilation elaborates the whole prelude* and measures
`elab.rs:997` at ~115 KiB of headroom out of 2 MiB. That implies a requirement
of **1933 KiB, which falls inside `(1920, 1952]`** -- an independent measurement
from a different subsystem a month earlier, landing inside tonight's bracket to
within its own resolution. **The two nodes are plausibly one finding.**

**STACK DEPTH IS A MAX OVER THE PRELUDE'S DECLARATIONS, NOT A SUM.**
`register_prelude` makes 177 `elaborate_decl` calls from one call site, and each
returns before the next is made -- so 177 registrations do not accumulate on the
stack. ⇒ **"Elaborate fewer declarations" is aimed wrong**, and so is lazy or
on-demand registration, **unless the declaration skipped is the deep one.** This
sentence is here because it is the remedy a fresh reader proposes first, it is
plausible, it is expensive, and the max-not-sum fact rules it out on its own.

**NOT ESTABLISHED: that the prefix IS prelude elaboration.** What is measured is
that ~1920 KiB is spent **before lowering**. It is not attributed to a phase.
The row's name and its 0.23-second runtime say it stops early -- strong, but a
name and a duration are not a profile. **Do not name a function nobody has
instrumented.**

## WITHDRAWN: this node's original conclusion, and why

**This node was filed at `3603991f2` claiming the opposite** -- a thirtyfold
program-dependent spread with no shared component. Every clause is refuted:

    thirty-fold range          its low end was an UNEXECUTED test
    no shared component        the shared component is ~94% of the budget
    bound consumption broadly  find ONE expensive shared path
    the larger program of work plausibly the SMALLER one

**The floor-side evidence was two `#[ignore]`d rows.** Harness output at the
probe base:

    RUST_MIN_STACK=64KiB  px8l dynamic_multistep_seed_...
    test result: ok. 0 passed; 0 failed; 1 ignored; 0 measured;
                 2 filtered out; finished in 0.00s

Both floor rows carry `#[ignore]` at `1dec48f33` (`px8l` under
`RT-BORROWED-INPUT-CARRIER-DURABILITY`, `px7m` under
`RT-SITEOP-CARRIED-WITNESS D2`), and both annotations say the row **fails** if
it runs. Nine rows across seven files; of the eight stack-measured rows, the
**2 ignored were the only two cheap ones** and all **6 live ones are
expensive**. Zero exceptions: the ignored/live split predicted the
cheap/expensive split exactly.

## METHOD: why the rule that should have caught this did not

**`rc=0` is one number covering two facts: ran-and-passed, and never-ran.**
libtest defines a run's success as `state.failed == 0`
(`formatters/pretty.rs:270`, `terse.rs:259`), **which zero executed tests
satisfies by construction.** So an exit code can never witness execution.

The standing rule -- *"a floor measured on a compile that did not happen is
worth nothing"* -- was applied to `dasm_c2` on `rc=101` and missed these two.
**It caught the row that failed loudly and missed the two that succeeded
silently**, and `dasm_c2` was the control that proved the mechanism: live,
executed, real non-zero code, and it was the one discarded.

⇒ **Screening `#[ignore]` status is a precondition of this instrument, not a
nicety, and every probe states its executed count.** A pass is reported as
`1 passed`, never as "no overflow" and never as an exit code.

**Provenance, recorded because it crossed three seats.** Execution was asserted
from a **call-path read** rather than an execution check; the Steward inherited
the word "verified" and filed it as a measurement without asking how execution
had been established; the implementer's discounting rule could not see it. **One
unexamined word, three seats, into a durable artifact.** It is the Architect's
own D5b ruling of the same night -- *an inert instrument is green-vs-green in
the limit* -- arriving on this node's own evidence. A census tally inherited the
same way inside the commit that documented the mechanism.

## This is the third instance, and the first one predicted it

`LANG-NATIVE-PRODUCTION-STACK-FOOTPRINT` (merged) recorded the condition on
`px4b_native_production`, and its body says:

> So the path is armed for whatever lands next, and the next candidate to trip
> it will look equally guilty and equally not be the cause.

**That came true on a different path.** `LANG-PRELUDE-ELABORATION-DEPTH`
(merged) is the second, and named this node's leading candidate a month early.
**Per-instance repairs were applied twice and the condition recurred.**

## Deliverables

- **D0. Attribute the prefix to a phase.** ~1920 KiB is spent before lowering;
  which phase spends it is unmeasured. This is the gating question and it needs
  a profile, not a name.
  **A first target, offered as a HYPOTHESIS and not a finding** (Architect,
  `evt_3mwrt8gj994q1`): `register_prelude` is a single function spanning
  `prelude.rs:471-3127` -- 2657 lines, 177 call expressions, 4 top-level
  conditionals -- reached from exactly one call site, and **in an unoptimized
  build its own frame is a candidate for a large fixed cost**, since debug
  builds allocate slots for a function's locals and temporaries with little
  reuse. It would account for a cost that is fixed, near-constant across
  programs, not a depth phenomenon, and largely gone in release -- and it
  **predicts D4's outcome**, which makes it refutable rather than decorative.
  Measure it first; `AC-5` still forbids the node asserting it.
- **D1. Reconcile with `LANG-PRELUDE-ELABORATION-DEPTH`.** If the prefix is
  prelude elaboration, these are one finding and that node's merged status is
  itself a result -- a repair that did not hold. **Establish it; do not assume
  it from the arithmetic.**
- **D2. A stated bound TOGETHER WITH THE INPUT IT IS A FUNCTION OF**, over a
  named population. Not "the compiler needs N KiB": a number without its input
  is satisfiable by measuring the cheapest member. (Architect's third rescoping
  of this criterion, `evt_79fyfvanbk896`.)
- **D3. A live cheap row, or the finding that none exists.** The floor side is
  currently **empty**. Whether any native compile is cheap is unmeasured, and
  "none exists" would be a stronger result than the one this node first claimed.
- **D4. Separate the build profile. RUN THIS FIRST.** Re-bracket
  `nondecreasing_cycle_is_rejected_before_native_lowering` in release against
  its debug bracket, both readings pre-committed above. Last in the list and
  first in time: it is two probes, it bounds the severity of everything else
  here, and **the funding call should not be made without it.**

## Acceptance criteria

- **AC-1.** Every probe reports its **executed count**. No claim rests on an
  exit code or on "no overflow".
- **AC-2.** Every measured row's `#[ignore]` status is stated at the measured
  SHA, not at `origin/main`.
- **AC-3.** No `RUST_MIN_STACK` raise and no `stack_size` added to buy headroom.
  The fix is a **reduction** -- inherited from `LANG-RECORD-STACK-OVERFLOW`.
  **The criterion most likely to be reached for under pressure.**
- **AC-4.** Any repair lands carrying its resulting headroom as a number. **A
  green check is not evidence**; it cannot distinguish 3% from 22%.
- **AC-5.** No deliverable names the expensive phase until D0 measures it.
- **AC-7.** Every stack figure states the **build profile** it was taken under.
  A number without its profile is not comparable to one taken under the other,
  and this node was written with every number in it taken under only one.
- **AC-6.** No-regression, green in CI (`COORDINATION §12`, never a local
  `--workspace` run).

## Not this node

- **Not the remedy's funding.** Whether this is worked, and when, is an operator
  scope call under `steward/lanes.md` §0. **`draft` is what stops it being
  pulled before that call.** The Steward told the operator this pointed at the
  *larger* program of work; that rested on the withdrawn evidence and **the
  correction is that it plausibly points at the smaller one.** *Plausibly* is
  load-bearing: **which of the two it is, is exactly what D4 measures**, and the
  brief carries that it is unmeasured rather than asserting either.
- **Not `#3676`.** That candidate cannot be repaired into landing by shaving its
  own increment: the condition it tripped is ~94% occupancy with none of its
  commits present. Routing independent and unchanged.
- **Not `TEST-STATED-STACK-SITE-RECONCILE`.** That owns sites that *state* a
  stack; this owns the population that states nothing.
- **Not the growth curve.** Cancelled -- attributing a 64 KiB straw on a
  96.9%-full stack has no repair attached to either outcome.

## Related

- `LANG-PRELUDE-ELABORATION-DEPTH` -- merged; names this node's leading
  candidate and supplies the independent 1933 KiB figure.
- `LANG-NATIVE-PRODUCTION-STACK-FOOTPRINT` -- merged; first instance, whose
  written prediction this node confirms.
- `TEST-STATED-STACK-SITE-RECONCILE` -- `ready`; the complementary population.
- `LANG-RECORD-STACK-OVERFLOW` -- merged; source of the reduction-not-raise rule.
