---
id: RT-NATIVE-COMPILE-RUNS-AT-THE-STACK-WALL
title: "IN A DEBUG BUILD the native compile's stack budget is spent BEFORE lowering begins -- a compile that rejects before native lowering needs (1920, 1952] of 2048 KiB and one that lowers, emits and executes needs (1920, 1984], a difference inside the brackets' own granularity, so lowering is a rounding error on a fixed prefix every compile pays. D4 measured that same row at (320, 352] in RELEASE, and no gate anywhere builds release: zero --release in ci.yml, build-ci-base.yml and scripts/ken-cargo. So the recurrence is CI's rather than the compiler's. Stated the way the decision needs it and with no midpoint invented: ON THE 2048 KiB TEST THREAD, DEBUG HAS ROOM FOR EXACTLY ONE MORE 64 KiB PRELUDE ADDITION AND NOT TWO (headroom is [96, 128) KiB) while release has room for at least 26 -- and NEITHER row describes the product, which carries a stated 4 MiB minimum and on that configuration has room for at least 58. THE REMEDY IS PROPAGATION, NOT MEASUREMENT: the two profiled numbers and that 4 MiB requirement have been in elab.rs:1347 for a month, correctly stated and carefully qualified, in a comment no gate reads and the 30 stack_size sites do not honour. THIRD measured instance; LANG-PRELUDE-ELABORATION-DEPTH named this candidate a month ago and is merged."
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

**The budget is gone before lowering starts, IN A DEBUG BUILD.** Every figure
in this section is from binary search on `RUST_MIN_STACK` against the base
`1dec48f33`, **under `cargo test`, unoptimized**, with no candidate commits
present, and **every probe carries its executed count.** See D4 for the same
row in release, where the picture is different.

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
**In debug, the great majority of the budget is a fixed prefix every compile
pays** -- `(1920, 1952]` of 2048 KiB, so between 93.8% and 95.3%, stated as the
interval the bracket supports -- and lowering is a rounding error on top of it.

The default is bracketed to `(1536, 2048]` by the same instrument and pins to
2048 only via the documented `std::thread` value -- **that last step is
documentation, not measurement.**

## "PRE-LOWERING" IS CONFIRMED. "PRE-LOWERING" IS NOT ONE CONSUMER.

**The row that bounds the pre-lowering claim was selected to drive the kernel's
size-change-termination gate to rejection** (Architect, verified at source,
`evt_526whg4xnpb2m`):

    fn spin (fuel : Nat) : HostIO APartial ExitCode = spin fuel
    .expect_err("the kernel SCT gate must reject a non-decreasing
                 recursive cycle")

⇒ Its `(1920, 1952]` covers prelude registration **plus** program elaboration
**plus** the termination check. At least two candidates live inside that
bracket: **prelude registration, which is unconditional, and termination
checking, which is program-triggered** -- and the row establishing the bound is
one chosen for the second. **Nothing here weakens "the spend is before
lowering." It forbids reading that as "the spend is the prelude."**

**This does NOT touch D4.** D4 compares one row against itself across build
profiles; the confound is present in both arms and cancels. **It bites the
absolute claims -- D0, D3, and the decomposition below -- not the differential
one.** Stated because the reverse reading is available and would retire the
cheapest measurement on the list.

## A DECOMPOSITION THE NUMBERS PERMIT, WHICH IS NOT ASSERTED AND MAY NOT BE

    prelude alone (merged node)      2048 - 115  =  ~1933 KiB
    whole trivial reject row                     <=  1952 KiB
    => everything else that row does             <=  ~19 KiB
       (three declarations elaborated, SCT gate run to rejection)

    heavy rows that fully compile                <=  1984 KiB
    => lowering + emission + execution           <=  ~51 KiB over prelude

If it holds it is far stronger than "94% pre-lowering": **the prelude would be
nearly the whole cost, and program elaboration, termination checking, lowering,
emission and running the binary would together fit in about 2.5% of the
budget** -- which would also bound SCT at ~19 KiB and dissolve the confound
above.

**It is not asserted, and the Architect declined to assert it, on this node's
own AC-7.** The subtraction combines `1933` -- taken from a merged node -- with
tonight's brackets, and **the two have not been shown to share a build
profile.** AC-7 exists precisely to forbid that comparison. ⇒ **A decomposition
to check, not a finding.** D4 is what makes it checkable: if D4 also re-states
the prelude figure under a stated profile, this falls out for free.

## THE PROFILE WAS VARIED ONCE, A MONTH AGO, AND THE RESULT IS IN THE SOURCE

**WITHDRAWN: "the build profile was the uncontrolled variable" and "every stack
figure this project has produced is a debug figure."** Both were this node's,
both are false, and both were pointed at the operator. Verified by the Steward
at `origin/main`, `crates/ken-elaborator/src/elab.rs:1347-1357`:

    REQUIRED MINIMUM (LANG-PRELUDE-ELABORATION-DEPTH D1/D4, measured at
    2ca91a3a): a caller of `ElabEnv::new` + `elaborate_file` -- what `ken
    check`/`ken run` actually do -- must provision at least 4 MiB of stack
    ... worst observed 1,982,464 bytes, ~1.98 MiB ...
    Optimized peak measured the same way: ~280 KiB.

`2ca91a3a` is a real commit (2026-08-14). ⇒ **The profile WAS varied, once, and
a release figure has existed in the elaborator's own source for a month.**

**The census that produced the false inference was itself sound.** Zero
`--release` across `.github/` and `scripts/`, re-verified. That answers *"does
any gate build release?"* -- and it was allowed to answer *"has anyone ever
measured release?"*, a strictly larger question with a different answer. **A
claim inherits the scope of the site you checked, not the scope you stated**
(Architect, `evt_6d1bx8rn8br76`, withdrawing his own sentence).

**THE EXISTENCE IS ESTABLISHED. THE VALUE IS A COMMENT.** `~280 KiB` occurs
**exactly once in the entire tree** -- the Steward grepped it; it is a
transcription with no custodian, which `prelude.rs` warns about in its own
words: *"an in-code figure would need a custodian and would go stale silently at
the next unrelated edit, with nothing red."* Nobody has re-run `2ca91a3a`'s
measurement.

**THE RAW COMPARISON AGAINST D4** (runtime-implementer, read at source,
`evt_1d0gmk3vgmj2r`). **Read the resolution below before drawing anything from
it -- these are not comparable quantities:**

    DEBUG    comment 1,982,464 B = 1936 KiB   vs  D4 (1920, 1952]
             -> lands on the exact midpoint of D4's interval

    RELEASE  comment ~280 KiB                 vs  D4 ( 320,  352]
             -> at least 40 KiB below it, outside the interval

**THERE IS NO ANOMALY. THE TWO PAIRS ARE INCOMPARABLE, AND THE COMMENT SAYS SO
ITSELF.** The premise needed to settle this is stated eleven lines further down
the same block, and the Steward verified it at `origin/main`:

    // `ken-cli`'s own 8 MiB main thread has wide headroom today (~6.1 MiB
    // unoptimized free)

    8192 KiB thread - 1936 KiB peak = 6256 KiB = 6.109 MiB   exact match

⇒ **The 1,982,464-byte peak was taken on an 8 MiB thread with 6.1 MiB of slack
beneath it.** It was never pressed against any 2048 ceiling. With that in hand,
both pairs resolve:

    DEBUG    peak 1936 (8 MiB, uncapped)  vs  D4 min-viable (1920, 1952]
             min-viable would sit BELOW the peak -- impossible for one program,
             since you must provision at least what you use
             => THESE ARE DIFFERENT PROGRAMS; the agreement is COINCIDENCE

    RELEASE  peak 280 (same method)       vs  D4 min-viable ( 320,  352]
             min-viable sits 40-72 KiB ABOVE the peak
             => the EXPECTED ordering; nothing to explain

**And the comment names the two programs**: a bare-prelude four-line program
calling no prelude combinator, against tonight's SCT `nondecreasing_cycle` row.
`elab.rs:1366-68` already forbids swapping peak-usage for minimum-viable-stack.
⇒ **Peak and min-viable across two different programs are not required to agree
in either profile. The implementer's refusal to reconcile them was the correct
read of both halves**, and the debug convergence reported earlier as
corroboration is coincidence.

**WITHDRAWN: the range-restriction inversion**, which this node carried one
commit ago as a live candidate (Architect `evt_evwnx38tea7h`, withdrawn by its
author at `evt_1rsh3qk513qtc`). The argument was that a debug figure exists only
when it fits, so it is evidence about the ceiling first. **The check that
argument itself demands is to read whether the measurement stated its own
slack. It did, in the next paragraph.** A selection-effect argument proposed
without checking the selection.

**A COINCIDENCE FLAGGED SO NOBODY READS IT AS EVIDENCE.** `1,982,464 B` is
exactly `1936.0 KiB` -- the midpoint this node was corrected for inventing two
commits ago. It happens to equal a real measured peak. **That is luck, and it is
the best available argument that inventing it was still wrong: a reader cannot
tell luck from evidence.**

**D4 ANSWERED IT. THE FIRST PRE-COMMITTED ARM FIRED.** Same row, same
instrument, same base `1dec48f33`, both profiles (runtime-implementer,
`evt_2rq86bavmstt8`):

    DEBUG     (1920, 1952] KiB    ~94.5% of a 2048 KiB budget
    RELEASE   ( 320,  352] KiB    ~17%

      1024 pass(1 executed)        384 pass(1 executed)
       512 pass(1 executed)        352 pass(1 executed)
       256 OVERFLOW                320 OVERFLOW
       128 OVERFLOW
        64 OVERFLOW

⇒ **The DEBUG-TO-RELEASE RATIO is in `[5.45, 6.10]`** -- bounded by
`1920/352 = 5.45` and `1952/320 = 6.10` -- and **that interval is the honest
figure**; two seats quoted 5.5x and 5.8x as point values from these same
brackets within minutes of each other. **Do not confuse this with the release
wall multiplier `[5.82, 6.40)` below**: they are different quantities that
happen to overlap, not one number transcribed twice. **The headroom goes from
`[96, 128)` KiB to `[1696, 1728)`** -- both exact, by subtraction from 2048.
The condition is **frame-size dominated**, which was the arm meaning: the remedy is
the elaborator's recursive `check` frame, the severity for shipped compiles is
far lower than for CI, and **this is substantially the SMALLER program of
work.**

**The preconditions were discharged before the number, not after.** `#[ignore]`
screened at the SHA rather than at HEAD; the control re-run inside the release
binary (`1 passed; 0 failed; 0 ignored`, 0.09s) rather than inherited from
debug; and **the release binary shown to be a distinct artifact** --
`px8l_recursive_decl_native-894faa2b7dc8a9b1` at 19,392,776 bytes against the
debug `-3058e1fdeee79953` at 75,598,704. Different path, different hash,
different size: not a debug binary aliased under `--release`. Every probe line
above reports what executed.

It also corroborates the merged node's prose from a direction that node could
not test -- *"in an unoptimized build a new arm's locals in `check` are paid by
every call regardless of which arm runs"* predicted a debug-inflated frame, and
a collapse of `[5.45, 6.10]`x is what that looks like measured.

**D4'S CONTRIBUTION, STATED NARROWLY AND STILL REAL.** It is **not** the first
variation of the build profile -- `2ca91a3a` did that a month ago. It is **the
first release figure on this row, and the first taken under the ignore-screen
and executed-count preconditions**, which is why it can be compared to the
comment at all.

**NOT corroboration.** That the repo's month-old debug figure lands on the exact
midpoint of D4's debug bracket was reported as independent corroboration, by two
seats, and **it is coincidence** -- the two are peak-usage and minimum-viable
stack over two different programs, and a min-viable below a peak is impossible
for one program, which is how we know they are different programs. **The
strongest-looking number in the exchange turned out to be the emptiest**, and it
looked strongest precisely because it agreed to the digit.

**WHAT D4 DOES NOT ESTABLISH, and it bounds how far the result travels:**

- **One row.** The ratio is measured on the pre-lowering row only. The four
  heavy rows that lower, emit and run are **unmeasured in release**, and they
  exercise a different path -- which is the reason this row was chosen.
- **It does not make the condition benign.** 352 KiB of 2048 is comfortable; it
  is still an unstated requirement that nothing measures and nothing bounds, and
  `37 §9`'s queue of prelude additions grows it **in either profile.** The
  condition is smaller, not absent.

## NO GATE ANYWHERE EXERCISES THE RELEASE NUMBER

The implementer left open whether Ken ships through a release-profile binary,
correctly noting a stack bracket cannot answer it. **The repo answers it**
(Architect, `evt_76nhsb73ys3wx`):

    --release in .github/workflows/ci.yml              0
    --release in .github/workflows/build-ci-base.yml   0
    --release in scripts/ken-cargo                     0

⇒ **Nothing in CI and nothing in the sanctioned local build path ever builds
release, so no release stack figure is exercised by any gate.** That is the
whole of what this census supports -- see the withdrawal above for the larger
claim it was allowed to carry and cannot.

**THE CANARY ARITHMETIC, IN THE ONLY FORM THE BRACKETS SUPPORT.** Take the
**subtraction** rather than the division and no midpoint is needed --
`2048 - (1920, 1952] = [96, 128)` is exact:

    DEBUG    headroom  [  96,  128) KiB   wall at [1.049, 1.067)x   4.9% - 6.7%
    RELEASE  headroom  [1696, 1728) KiB   wall at [5.82,  6.40 )x   482% - 540%

**AND THE ABSOLUTE FORM ANSWERS THE OPERATOR'S QUESTION WHERE THE PERCENTAGE
DOES NOT.** The decision is whether the next prelude addition fits, and the
candidate that triggered this arc was **64 KiB**:

- **Debug absorbs exactly one more 64 KiB candidate, and not two.** `64 <= 96`
  holds at every point of the bracket, and `128 > headroom` holds at every
  point because the headroom is strictly below 128. **Both halves are true over
  the whole interval -- no midpoint, no error bar.**
- **Release on that same 2048 KiB test thread absorbs at least 26 of them**
  (`1696 / 64`). **This is not the product figure** -- see below.

⇒ **The recurrence is CI's, not the compiler's**, and `37 §9`'s open queue of
prelude additions is spent against the one, not the twenty-six. *"About 6% of
headroom"* cannot be acted on without knowing 6% of what; *"one more, not two"*
can.

**BOTH ROWS OF THAT TABLE DESCRIBE A 2048 KiB TEST THREAD. NEITHER DESCRIBES
THE PRODUCT.** `ken check` and `ken run` carry a **stated** requirement of
**at least 4 MiB** (`elab.rs:1347`), and the same comment says Rust's 2 MiB
spawned-thread default *"must not be treated as adequate -- it is
`r3_c2_source_mixed_branch.rs`'s `r3_4b` worker's exact failure configuration
(#2144), not a safe answer."* **So "the product has room for 26 more" is a
statement about release code on a 2 MiB thread, which is not the configuration
the product is specified to run in.** Read the table as CI-side only.

**The product figure, computed in the product's own configuration:** release
code on the stated 4 MiB minimum leaves headroom `[3744, 3776)` KiB, **at least
58 more 64 KiB additions** -- not 26. The `26` was computed against 2048, the CI
test thread, and attached to a product claim.

**`1936` and `336` appeared here in the routed version and were never measured
-- they are the midpoints of the two brackets, and every percentage on those
lines was derived from them** (Architect, `evt_4dk1z2ffyef8f`, correcting his
own block). This is the interval defect from 20 lines above, reappearing in the
paragraph the brief quotes, after it had been named.

**SO THE WORKAROUND POPULATION READS BACKWARDS FROM HOW IT WAS BUILT.** 30
`stack_size` occurrences across 21 files, 14 at the same 256 MiB constant, zero
stated rules. **Debug is a ~6x-amplified early-warning instrument for release
stack growth, and 30 sites were spent silencing it** -- then the same condition
was met in a 31st place and filed as a new discovery, three times.

**THE REMEDY SHRANK A SECOND TIME, AND THE GAP IS PROPAGATION, NOT
MEASUREMENT.** The deliverable was placed in `stated-stacks.md` act 2 as *"two
stated numbers with their profiles."* **Those two numbers already exist, with
their profiles** -- plus a stated `REQUIRED MINIMUM` of 4 MiB for `ken
check`/`ken run`, plus the scope qualifier carried inside the requirement
sentence rather than beneath it, where a reader cannot take the number without
the caveat. It was written a month ago in the function everyone has been
reading all night.

⇒ **A correctly stated, carefully qualified requirement lives in a comment that
no gate reads, and the 30 `stack_size` sites do not honour it.** The work is to
make the stated requirement enforceable and propagated, not to measure it.

**This is `a-mechanism-claim-in-a-comment-is-structurally-exempt-from-execution`
at BOTH polarities, in one night, four hours apart.** At `Cargo.toml:61-62` a
comment asserts something **false** and nothing can redden. Here a comment
asserts something **true and carefully qualified** and nothing can enforce it.
**Identical structural cause; one was filed as a finding and the other was read
straight past by the same reader.**

**A THIRD INSTANCE OF THE SAME DEFECT, FOLDED HERE RATHER THAN FILED, BECAUSE
IT IS EVIDENCE FOR THIS FINDING AND NOT A NEW ONE.** The same comment carries an
open obligation:

    // Whether the headline peak-usage figure itself needs revising still
    // requires a peak-usage run on a deep source, which nobody has done.

⇒ **An open obligation, correctly stated, sitting in production source with no
node and no owner.** The tree states the requirement, states its limits, and
states what remains undone -- **and nothing anywhere can read any of it.**

**One caveat before anyone leans on the comment as the bound:** it is a
transcription with no custodian, and **a transcription is not a re-derived
measurement.** `~280 KiB` should be re-run (`D5`), not cited.

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

**BUT THIS FIGURE, UNLIKE THE 1936 PEAK, WAS TAKEN ON A 2 MiB THREAD**, and the
range-restriction caution therefore **does** apply to it: a measurement that
reports 115 KiB of headroom is a measurement that fit, and one that had not fit
would have produced no number at all. **So `1933` landing inside a bracket
bounded above by the same 2048 is weaker evidence than it reads as.** The
`1,982,464`-byte peak was refuted as range-restricted because it ran on an 8 MiB
thread with 6.1 MiB of slack; **`1933` has no such defence, and nobody has
checked whether it needs one.** Do not promote this convergence while `D0` is
open.

**STACK DEPTH IS A MAX OVER THE PRELUDE'S DECLARATIONS, NOT A SUM.**
`register_prelude` makes 177 `elaborate_decl` calls from one call site, and each
returns before the next is made -- so 177 registrations do not accumulate on the
stack. ⇒ **"Elaborate fewer declarations" is aimed wrong**, and so is lazy or
on-demand registration, **unless the declaration skipped is the deep one.** This
sentence is here because it is the remedy a fresh reader proposes first, it is
plausible, it is expensive, and the max-not-sum fact rules it out on its own.

**SETTLED FOR FREE, and it is not an attribution.** The merged node's deep call
-- `register_decimal_char`'s 31-level cascade -- **is on the prelude path**:
`register_prelude` spans `prelude.rs:471-3127` and calls it at `:1267`, verified
by the Steward at `origin/main`. So that node's *"unrelated to any arm here"*
meant unrelated to the **record-literal arms**, not to the prelude, and the
`1933` convergence is a prelude figure. **It still does not attribute tonight's
peak to a phase.**

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

**A NEW CRITERION IS APPLIED FORWARD AND NEVER SWEPT BACKWARD. THREE FOR THREE,
IN ONE HOUR, ON THIS NODE.**

    AC-7 (state the profile)   the Architect restated the cross-node predicate
                               without its profile, one message after proposing
                               AC-7
    AC-7                       this node's TITLE claimed 94% without saying
                               debug -- in the node that cut AC-7
    the interval rule          the canary block re-derived percentages from
                               1936 and 336, midpoints of the two brackets that
                               were never measured, 20 lines after the interval
                               defect was named -- and it reached the title

    "a claim inherits the      a three-site --release census answered "does any
     scope of the site you      gate build release?" and was allowed to answer
     checked"                   "has anyone ever measured release?" -- by the
                                author of that lesson, while auditing the same
                                shape in other people's work all night

Four independent instances, three authors, zero carelessness: **a criterion is
naturally applied to the claims made after it, and to other people's work, and
never to the sentences already standing in the document that states it.**

⇒ **CUTTING AN AC OBLIGES ONE PASS OVER THE CONTAINING DOCUMENT AGAINST THAT
AC BEFORE THE ARTIFACT IS ROUTED.** It is cheap, it is mechanical, and it would
have caught all three of the above.

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
  **The row must elaborate a trivial program TO COMPLETION**, without an SCT
  rejection -- see the confound below. A cheap-row hunt that lands on another
  rejection row re-measures the same mixture.
  **D3 IS A DIRECT TEST OF D0'S HYPOTHESIS, AND BOTH OUTCOMES ARE RECORDED HERE
  BEFORE IT RUNS** (Architect, `evt_526whg4xnpb2m`). `register_prelude` has one
  call site, in `ElabEnv` construction, unconditional -- so every compile pays
  its peak:

      prelude peak really is ~1933 KiB   NO live row can be cheap. D3 comes back
                                         EMPTY and the emptiness is the RESULT.
      D3 finds ONE cheap live row        the prelude-peak hypothesis is REFUTED,
                                         whatever D4 says about profile.

  **Both predictions are DEBUG-PROFILE predictions and D3 must be run in debug
  to test them** -- per `AC-7`, and because D4 measured every row to be cheap in
  release, where the search would return a hit that means nothing.

  **Recorded in advance because an unexpected empty result and a predicted empty
  result get acted on completely differently** -- the first reads as "we did not
  look hard enough" and gets repeated, at cost, forever.
- **D5. Re-measure the optimized peak at HEAD by `2ca91a3a`'s own method.**
  **Reduced to one question.** How the method bounds the run is now answered
  from the tree -- `ken-cli`'s 8 MiB main thread, observed peak, slack stated --
  so all that remains is: **is `~280 KiB` still true at HEAD**, after 28 commits
  and +480 net lines in `prelude.rs`? Both outcomes are fixed in advance:

      ~280 returned        the comment is CURRENT and the release gap is real;
                           different-program / different-quantity is the
                           remaining candidate.
      ~320-352 returned    the comment went stale SILENTLY over a month with
                           nothing red -- precisely what `prelude.rs`'s own doc
                           comment predicts about in-code figures. **This turns
                           the propagation finding from theoretical into
                           demonstrated: the one release number in the tree
                           would have decayed WHILE BEING CITED.**

  **Does not gate the brief**, which holds under either outcome: *no gate builds
  release* and *the requirement is stated where nothing enforces it* are
  untouched by it.
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
  correction is that it points at the smaller one.** That was filed as
  *plausible* pending D4; **D4 has since measured it**, the first arm fired, and
  the remedy's locus is `stated-stacks.md` act 2 plus two stated numbers rather
  than a compiler project. **The size question is answered; the funding question
  is still the operator's.**
- **Not `#3676`.** That candidate cannot be repaired into landing by shaving its
  own increment: the condition it tripped is ~94% occupancy with none of its
  commits present. Routing independent and unchanged.
- **Not `TEST-STATED-STACK-SITE-RECONCILE`.** That owns sites that *state* a
  stack; this owns the population that states nothing.
- **NOT the false `Cargo.toml` comment, which is a SEPARATE FINDING TO FILE.**
  Root `Cargo.toml:61-62` reads *"CI builds the full workspace and runs
  release/conformance ... Release profile is left at defaults (CI owns it)."*
  **Both clauses are false** -- CI has zero `--release` occurrences (measured
  above), and the conformance job echoes *"conformance suite not yet implemented
  (WP F2)"*. **"CI owns it" names an owner that does not exist.** It is recorded
  here rather than folded because it is a false claim about CI sitting in the
  file a reader opens to answer exactly the question being asked of the
  operator, and **a mechanism claim in a comment is structurally exempt from
  execution**, so nothing could ever have gone red on it. Architect
  `evt_76nhsb73ys3wx`; not folded at the Architect's own request.
- **Not the growth curve.** Cancelled -- attributing a 64 KiB straw on a
  96.9%-full stack has no repair attached to either outcome.

## Related

- `LANG-PRELUDE-ELABORATION-DEPTH` -- merged; names this node's leading
  candidate and supplies the independent 1933 KiB figure.
- `LANG-NATIVE-PRODUCTION-STACK-FOOTPRINT` -- merged; first instance, whose
  written prediction this node confirms.
- `TEST-STATED-STACK-SITE-RECONCILE` -- `ready`; the complementary population.
- `LANG-RECORD-STACK-OVERFLOW` -- merged; source of the reduction-not-raise rule.
