---
id: RT-NATIVE-COMPILE-RUNS-AT-THE-STACK-WALL
title: "The native compile's stack consumption SCALES WITH THE PROGRAM, unmeasured and unbounded: one real complete compile fits in 64 KiB while three others need ~1950 KiB of a 2048 KiB default, a thirtyfold spread, and nothing states which programs are which or what the limit is. So a candidate adding 64 KiB aborts one of them and looks guilty while not being the cause. This is the THIRD measured instance of that condition -- LANG-NATIVE-PRODUCTION-STACK-FOOTPRINT and LANG-PRELUDE-ELABORATION-DEPTH are both merged, both single-digit-percent headroom, both diagnosed trigger-not-cause -- and the first predicted this recurrence IN WRITING."
status: draft
owner: runtime
size: L
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "Measured by runtime-implementer 2026-09-15 while diagnosing the class-1 SIGABRT on PR #3676 (evt_3c8a5tzt0ksmx, evt_1bk3sktcha9vr, evt_5xtnkrqadf69d, evt_1yh3m79r5wdjb). Architect ruled the shave-the-increment arm is not a coherent stopping point and named the base-bracket narrowing as the deciding measurement (evt_29ph3z4a7661t), then withdrew a minimal-compile probe when the screen's fourth row answered it better. Steward found the merged precedents and filed. Funding is an operator scope call and is NOT taken here."
---

## SUSPENDED: THE FLOOR-SIDE EVIDENCE MAY BE VACUOUS. READ THIS FIRST.

**Both floor rows carry `#[ignore]` at the probe base `1dec48f33`** -- verified
by the Steward directly against that tree, and by the Architect against
`origin/main` (`evt_3zr83trkqgt3h`):

    px8l_recursive_decl_native.rs      #[ignore = "RT-BORROWED-INPUT-CARRIER-
                                        DURABILITY: ... traps ..."]
    px7m_hostresult_computational...   #[ignore = "RT-SITEOP-CARRIED-WITNESS
                                        D2: ... refuses ..."]

**Every heavy row is live `#[test]`. Seven rows, six files, zero exceptions: the
ignored/live split predicts the cheap/expensive split exactly.**

⇒ **If those rows never executed, every number on the floor side of this node is
vacuous**, and with it the thirtyfold spread, the refutation of a shared
component, and this node's central claim. On live rows only the lowest
measurement is `rt_branched` at `>512 KiB` and everything else is `>1024` --
which is the **uniform** picture, the opposite of what the body below argues.

**The one measurement that settles it** is the harness `running N tests ... X
passed; Y failed; Z ignored` summary from any floor probe. `Z` counting the
floor row means it never ran.

**Why this was not caught by the rule that should have caught it.** The
implementer's own standard -- *"a floor measured on a compile that did not happen
is worth nothing"* -- was applied to `dasm_c2` on `rc=101` and missed these,
because **`rc=0` is one number covering two facts: ran and passed, and never
ran.** The rule caught the row that failed loudly and missed the two that
succeed silently. `dasm_c2` is the control that proves the mechanism: live,
executed, real non-zero code -- and it was the one discarded.

**Provenance of the error, recorded because it crossed three seats.** The
Architect asserted the floor row was a verified real compile from a **call-path
read**, not an execution check; the Steward inherited that word and filed it as
a measurement without asking how execution had been established; the
implementer's discounting rule could not see it. **One unexamined word, three
seats.** It is the Architect's own D5b ruling of the same night -- *an inert
instrument is green-vs-green in the limit* -- arriving on this node's evidence.

**Until the harness line is read, treat everything below the next heading as
UNDER CHALLENGE.** If the rows never ran, this node is rewritten rather than
amended: a claim whose evidence is withdrawn should not be patched.

## The finding, measured

**The compile's own stack requirement is ambient, unmeasured, and discovered
only by SIGABRT.** All figures come from binary search on `RUST_MIN_STACK`
against the base `1dec48f33`, with **no candidate commits present**. The
instrument brackets the requirement directly rather than inferring it from
struct sizes or a backtrace.

    px8l_recursive_decl_native
      dynamic_multistep_seed_preserves_updated_parameter_order    <= 64 KiB
    abi_s6_mapping_surface_native
      window_direct_map_bytes_executes_natively                (1920, 1984]
      complete_carried_mapping_access_matrix_matches_the...    (1952, 1984]
    lang_nested_former_recursion_native                            > 1024
    px8h_heterogeneous_continuation                                > 1024

Against a 2048 KiB default, the heavy rows sit at **95.3% - 96.9% occupancy**,
leaving 64 - 96 KiB. The default is bracketed to `(1536, 2048]` by the same
measurements and pins to 2048 only via the documented `std::thread` value --
**that last step is documentation, not measurement**, stated so a later reader
does not inherit it as a derived figure.

## The claim is NOT that every Ken compile runs near the wall

**It is the opposite, and this is the node's central point.** A real, complete
native compile -- prelude elaborated, lowered, emitted, executed -- fits in
**64 KiB**. The implementer verified that row is a genuine compile before
trusting it as a floor, tracing `assert_agreement` through
`ken_cli::build_native_program` (`px8l_recursive_decl_native.rs:122`) to the
same `compile_native_program_sources` entry the heavy rows use.

⇒ **A thirtyfold spread, driven by the program.** Consumption scales with what
is being compiled; nothing states which programs are expensive, and nothing
bounds the scaling. Some compiles sit at 96.9% and others at 3%.

**This is sharper than, and contrary to, what the screen alone supported.** A
3-of-4 overflow at 1024 KiB reads as "the compiler is uniformly deep". The floor
row refutes that. **Do not restate this node as "every native compile is near
the wall"**, and do not present `3/4` as a rate -- three independent failures
refute "one pathological row" outright, but four rows cannot estimate what
fraction of the surface is affected. That is a wider census and a different
instrument.

## How the comparison class was chosen, since that is what could invalidate it

Not by name and not by "looks heavy". `compile_native_program_sources` was on
the stack at the fault per the lldb backtrace; it is defined once at
`crates/ken-elaborator/src/compiler_driver.rs:2699` and `ken-cli`'s tests reach
it through one wrapper, so **"drives a comparable in-process Ken compile" holds
by construction**. The failing files are otherwise unlike each other: a second
row in the *same* file as the original (separating row from file), nested former
recursion, heterogeneous continuation. Three independent programs, three
independent test binaries.

## METHOD: the near-miss that decided this, recorded for the next bracketer

When the first probe overflowed at 128 and 64 KiB, the implementer wrote that
*"at 128 KiB essentially anything overflows, so those low probes carry no
information."* **A full native compile then passed at 64 KiB.**

⇒ **An assumption about which probes carry information is itself a claim, and
here it was the load-bearing one.** Acting on it would have stopped the downward
bracket at 512, reported the floor as `<= 512 KiB` rather than `<= 64`, and
weakened the refutation of cause B **eightfold**. The probes dismissed as
uninformative were the ones that mattered.

This is in the node beside the numbers because the next person bracketing
anything will have the same instinct.

## Cause B is refuted; cause A survives only in part, and the anomaly says why

    CAUSE B  one shared prefix (e.g. prelude elaboration) consumes most of the
             budget before lowering starts.  Remedy: fix that one floor.
    CAUSE A  consumption is in the lowering recursion and scales with the
             program.  Remedy: per-level frame reduction.

**B is refuted by measurement.** If a shared prefix cost ~1900 KiB, the floor
row would pay it too; it completes in 64 KiB. The Architect proposed
constructing a minimal-compile probe to separate these and **withdrew it when
the screen's fourth row turned out to be that probe already**, and a real row
rather than a constructed one.

**A's LOCATION HALF IS NOT ESTABLISHED, and the anomaly is why.**
`nondecreasing_cycle_is_rejected_before_native_lowering` -- a row that by its own
name stops *before* lowering -- needs **more than 1024 KiB**, overflowing at
1024, 512, 256 and 128. **A row that rejects before lowering costs more than
sixteen times a row that lowers, emits and runs.** If the expensive recursion
were in lowering, that row should be cheap. It is not.

⇒ **Either the expensive path is not lowering, or there is more than one
expensive path. Both readings are live.**

**KEEP:** consumption scales with the program, unmeasured and unbounded, so some
compiles sit at 96.9% and others at 3%, and nothing states which is which or
what the limit is. **DROP, until something measures it:** that the consumption
is in the lowering recursion. The measurement establishes **program-dependence,
not location** (Architect, `evt_79fyfvanbk896`).

⇒ **"Per-level frame reduction" is therefore a CANDIDATE remedy, not the implied
one** -- remedy shape follows location, and location is unattributed. The
anomaly is in this node as the reason for that, not as an untidy loose end.

## This is the third instance, and the first one predicted it

`LANG-NATIVE-PRODUCTION-STACK-FOOTPRINT` (merged) recorded the same condition on
`px4b_native_production`, and its body says:

> So the path is armed for whatever lands next, and the next candidate to trip
> it will look equally guilty and equally not be the cause.

**That came true on a different path.** That instance overflowed in
`register_prelude` (`ken-elaborator`); this one in `lower_expr` (`ken-runtime`
cranelift lowering). Same double structure: an A/B attributes the abort to the
candidate *and* the candidate's additions are not the deficit.

`LANG-PRELUDE-ELABORATION-DEPTH` (merged) is the second: `elab.rs:997` measured
~115 KiB of headroom out of 2 MiB, and **thirteen sites across four crates
independently bumped their thread to 256 MiB without any stated rule.**

⇒ **Per-instance repairs were applied twice and the condition recurred.** That
is the evidence bearing on whether to repair instances or the scaling, and it is
historical rather than predicted.

## The criterion and the remedy shape both already exist

**The criterion.** `LANG-NATIVE-PRODUCTION-STACK-FOOTPRINT`'s `D3` reads *"the
margin stated as a number ... The node's real product is that this number
exists."* The Architect independently re-derived it tonight as *"it lands with a
stated headroom number ... and not with a green check as its evidence."* **Two
independent routes to one criterion, weeks apart, from different evidence.**

**The remedy shape.** `crates/ken-verify/tests/px8f_write_partition.rs:368-374`
already implements it:

    WRITE_ALL_PARTITION_STACK_MEASURED_PEAK_BYTES
    WRITE_ALL_PARTITION_STACK_HEADROOM_BYTES = 252 * 1024 * 1024
    // measured headroom without depending on ambient RUST_MIN_STACK

⇒ **The population that has a stated margin is not the population that needs
one.** Sites that state a stack are covered by `TEST-STATED-STACK-SITE-RECONCILE`
(15 sites). **Nothing covers the population that states nothing and inherits the
ambient default**, which is where all three SIGABRTs happened.

## Deliverables

- **D0. The requirement stated as a number** for each measured row, with its
  budget in the same place. Brackets for the remaining heavy rows are in flight
  at +/-64 KiB; they are corroboration, not a dependency, since the cause fork
  is already settled.
- **D1. What separates cheap from expensive.** The floor row and the heavy rows
  differ by 30x through the same entry point. **That property is what a remedy
  has to address** -- a per-level frame reduction on the common path may not
  reach whatever is expensive about the three. This is the real next question.
- **D2. The population enumerated:** which in-process compiles inherit the
  ambient default rather than stating a stack. State the method, so a zero for
  any search term is readable as evidence about that term.
- **D3. A stated bound TOGETHER WITH THE INPUT IT IS A FUNCTION OF.** Not "the
  compiler needs N KiB" but "the requirement is a function of the program, here
  is the measured worst case over a **named population**, and here is what
  bounds it."

  **Why the program-wide form is wrong, which is not obvious.** If consumption
  is a function of the program, there is no single number that is "the
  compiler's stack requirement." A stated program-wide number measured on the
  floor row would read **64 KiB**, be perfectly true, satisfy a program-wide
  criterion, and be useless. **A number without its input is the same defect one
  level down** -- satisfiable by measuring the cheapest thing in the class.

  This is the Architect's third rescoping of its own criterion tonight: per-row,
  then program-wide, now bound-with-input. It is recorded as a rescoping rather
  than presented as the original, because each earlier form **could be satisfied
  without the property being true** -- the same defect being caught elsewhere in
  this arc, in the author's own criteria.

## Acceptance criteria

- **AC-1.** Every number is measured, not inferred. This margin arrived
  unnoticed precisely because nobody had probed it.
- **AC-2.** No `RUST_MIN_STACK` raise and no `stack_size` added to buy headroom.
  The fix is a **reduction** of what frames hold -- inherited from
  `LANG-RECORD-STACK-OVERFLOW`, restated by the Architect. **This is the
  criterion most likely to be reached for under pressure**, because raising the
  limit converts the measurement into a workaround for the condition it
  uncovered.
- **AC-3.** Any repair lands carrying its resulting headroom as a number. **A
  green check is not evidence** -- it cannot distinguish 3% from 22%, which is
  how this survived until an arc tipped it.
- **AC-4.** The `3/4` screen is never restated as a rate, and the node is never
  restated as "every native compile is near the wall".
- **AC-5.** The rejection-path anomaly is either explained or carried forward as
  an open observation. It is not absorbed into the cause-A story by silence, and
  no deliverable asserts a location for the consumption until one is measured.
- **AC-6.** Any stated bound names **the input it is a function of and the
  population it is a worst case over.** A bare number satisfies nothing here: a
  true program-wide figure measured on the cheapest member is the failure mode
  this criterion exists to exclude.
- **AC-7.** No-regression, green in CI (`COORDINATION §12`, never a local
  `--workspace` run).

## OPEN, and decision-relevant

**Did the earlier per-instance repair hold?** `px4b_native_production` was the
first instance and still provisions no stack today. Whether it is *back* at the
wall is **unmeasured**. One screen at 1024 KiB answers it, and it distinguishes
"targeted fixes hold" from "targeted fixes get re-crossed" -- the crux of the
funding question. Requested and not yet run.

## Not this node

- **Not the remedy.** Bounding the scaling across the ambient population is
  program-sized. **Whether it is funded, and when, is an operator scope call
  under `steward/lanes.md` §0** -- no measurement here adds or re-scopes a lane.
  This node is `draft` for exactly that reason: it must not be pulled before
  that call.
- **Not `#3676`.** That candidate cannot be repaired into landing by shaving its
  own increment, because the condition it tripped is 96.9% occupancy with none
  of its commits present. Its routing is independent and unchanged.
- **Not `TEST-STATED-STACK-SITE-RECONCILE`.** That owns sites that *state* a
  stack; this owns the population that states nothing. Do not merge them.
- **Not the growth curve.** Attributing which arc commit supplied a 64 KiB straw
  on a 96.9%-full stack has no repair attached to either outcome. Cancelled.

## Related

- `LANG-NATIVE-PRODUCTION-STACK-FOOTPRINT` -- merged; first instance, whose
  written prediction this node confirms.
- `LANG-PRELUDE-ELABORATION-DEPTH` -- merged; second instance, and the source of
  the thirteen-sites-at-256-MiB census.
- `TEST-STATED-STACK-SITE-RECONCILE` -- `ready`; the complementary population.
- `TEST-NATIVE-STACK-PROVISIONING-STANDARD` -- merged; the statedness standard,
  governing a test's stack rather than a compile's requirement.
- `LANG-RECORD-STACK-OVERFLOW` -- merged; source of the reduction-not-raise rule.
