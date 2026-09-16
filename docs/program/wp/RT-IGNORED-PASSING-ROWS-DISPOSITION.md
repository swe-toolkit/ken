# WP frame — `RT-IGNORED-PASSING-ROWS-DISPOSITION`

**Owner:** Team Runtime · **Size:** M · **Risk:** low (test-only diff, but a
wrong disposition installs a fake green) · **Tier:** T2 · **Gate:** none ·
**Deps:** none — `CI-IGNORED-SWEEP` is `merged` and supplies the instrument

**Origin:** operator directive 2026-09-15: *"Tests that pass should be verified
to be functional and desirable and un-ignored... The test to determine that
instant equality is wrong is just expensive documentation. That should be noted
in code comments and/or spec and/or conformance and not in an ignored test."*

## 1. Objective

Twelve `#[ignore]`d rows now **pass**. Give each one a disposition and act on
it, so the ignored set contains only rows that are ignored for a reason that is
still true.

`CI-IGNORED-SWEEP` (merged) exists to route precisely this event — its own
origin record names *"the good-news event this node exists to route, which
nothing reported."* The job emits a `::notice` per passing row, its findings are
non-blocking, and nothing has consumed them. This WP consumes the accumulated
backlog.

## 2. Fixed inputs, measured

Measured at PR #3676 head `0f71ab5b9267781ae1d91bc654011cad42b926af`, from the
`ignored-row sweep` job (GitHub job id `104224384914`), plus a source census
using `scripts/ci-ignored-sweep.py`'s own upward-walk algorithm.

                                      at 0f71ab5b9    on main
    real `#[ignore]` attributes           34            33
      all at column 0, all on `#[test] fn`
    macro-generated                        0             0
    nextest --run-ignored=only population  34            33
    registry run-exemptions                -6            -6
    selected and executed                  28            27

    at 0f71ab5b9  -> 12 PASSED / 16 failed
    on main       -> RE-DERIVE. Not predicted here. See below.

**CORRECTED 2026-09-16 — the earlier table read `33 real + 1 macro-generated`
and pinned `34` and `28` as carrying to `main`. Both pins are refuted by the
tree; section 1a below is the correction and the reason it mattered.**

> **NEITHER THE POPULATION NOR THE SPLIT CARRIES TO `main`. ONLY `-6` DOES.**
> The earlier text claimed the `#[ignore]` attribute-line diff between
> `0f71ab5b9` and `origin/main` was **zero** and pinned `34` and `28` on that
> basis. **The diff is one line, and it was one line when the claim was
> written:**
>
>     git diff 0f71ab5b9267781ae1d91bc654011cad42b926af origin/main -- 'crates/*' \
>       | grep '#\[ignore'
>     -#[ignore = "IGNORED BECAUSE ITS SHAPE IS ABSENT HERE, NOT BECAUSE IT IS
>                  UNFINISHED. ..."
>
> That is `discharge_ledger_refuses_one_word_discharging_two_obligations`, and
> it **never landed on `main`** — `git log -S` over `origin/main` returns only
> the commits that file the frames naming it, never the test.
>
> `.github/ignored-test-exemptions.toml` **is** identical, so **`-6` carries**
> and is the one pin that survives. The corrected population on `main` is `33`
> and the corrected selected figure is `27`.
>
> **`27` is not a prediction — it is a measurement.** Run `34753101365` at
> `7663ad9b924e7b1cb2c0111215b997b085465aa1` (2026-09-13) reported
> `Ignored-row sweep completed: 27 selected; 11 passed.` **The frame's own
> ground-truth instrument has contradicted the frame's pin since 09-13.**
>
> **`12 / 16` is not.** It is a property of the tree the sweep ran on, and that
> tree is PR #3676's head — **36 commits ahead of `origin/main` and 53 behind
> it.** A row that passes there may pass **because of work that is not on
> `main`**, which is the base this WP is routed onto and will be implemented
> from. **Un-ignoring such a row on `main` turns `main` red**, which is exactly
> what `D-READMIT` would do and what `AC-5` would then catch — after the work.
> `px8f_buffer_native.rs` is the one named file that **differs** between the two
> trees, and it holds one of the failing rows.
>
> ⇒ **D0 OF THIS WP IS TO RE-RUN THE SWEEP AT THE IMPLEMENTATION BASE AND
> RE-DERIVE THE SPLIT THERE.** Every count below keyed to "the twelve" or "the
> sixteen" means *the passing set / the failing set as measured at that base*,
> not these numbers. If the implementation base is deliberately `0f71ab5b9`'s
> lineage rather than `main`, say so in the dispatch and these numbers stand as
> written — but that is a decision, not a default.
>
> **This frame's own epistemics already demand it.** *"A label is not evidence
> about current behaviour"* is the same argument: 53 commits is the same kind of
> gap as 2363, and a sweep result is no more a property of a row than a label is.

**A source grep is the wrong instrument and must not be used to re-derive this.**
`grep -c '#\[ignore'` returns 38 on `main`: five of those are doc-comment prose
*about* `#[ignore]` (`recursor_fusion.rs` x2, `r3_c1_source_arrival.rs` x1,
`boundary_value_clif.rs` x2), leaving 33 real attributes.
`scripts/ci-ignored-sweep.py` derives its population from nextest because prose
hits and cfg-excluded rows both defeat a source census.

**The earlier text added "and a grep is simultaneously blind to the
macro-generated row." That rationale is withdrawn — it cited a row the grep sees
fine.** `discharge_ledger`'s attribute is at column 0 on a plain `#[test] fn`
(`crates/ken-cli/tests/px8f_buffer_native.rs`, `#[test]` then `#[ignore = "..."`
then `fn`, at `0f71ab5b9`). The grep finds it; the raw counts show it as
`39 -> 38` across the boundary.

## 1a. Why the misclassification was load-bearing, not cosmetic

**Calling that row macro-generated is what made the carry verification appear to
pass.** A macro-generated row is by construction invisible to a grep for
attribute lines. Labelling it that way let the frame add `+1` to the population
**and** report the attribute-line delta as zero, with no visible contradiction —
the single counterexample had been classified out of the instrument's view.

Relabel it as what the tree shows and every dependent claim moves at once:

    34 real at 0f71ab5b9, not 33 + 1
    attribute-line delta to main is ONE, not zero
    34 does not carry; main's population is 33
    28 does not carry; main's selected is 27

⇒ **`34` was never wrong as a measurement. It was wrong as a PIN.** It is a true
property of PR #3676's head, promoted to a property of `main` by a verification
that could not see the one row responsible for the difference.

**The general shape, which is the part worth keeping:** a classification error
made a verification vacuous, and that verification was the only thing standing
between a tree-local measurement and a pinned cross-tree claim. **A number
measured correctly on tree A becomes a claim about tree B only through a carry
argument.** The measurement was never in question; only its passport was.

Attribution: refuted by the Architect (`evt_2j2dghfrxx0p3`), independently
confirmed by `runtime-implementer`, coordinates re-verified at source by the
Steward before this amendment.

### 2a. The twelve rows, with the ignore reason each currently carries

**A NEARER ROSTER EXISTS AND IS THE BETTER STARTING POINT.** The twelve below
were measured at `0f71ab5b9`, which is 36 commits off `main`'s lineage. Run
`34753101365` at `7663ad9b924e7b1cb2c0111215b997b085465aa1` (2026-09-13) is on
`main`'s own lineage and reported **eleven**:

    ken-cli::px7f_resource_native            linked_public_escape_is_exact_closed
    ken-cli::px8l_recursive_decl_native      dynamic_multistep_seed_preserves_updated_parameter_order
    ken-cli::px8l_recursive_decl_native      dynamic_zero_seed_takes_the_base_case
    ken-cli::px8ta_oriented_subcontinuation  public_one_level_bracket_finishes_and_releases
    ken-cli::px8x_single_schema_observation  linked_route_exposes_real_ordered_bindings_and_filters_reserved_input
    ken-cli::rt_escape_second_resource_native  escape_one_used_matches_interpreter
    ken-cli::rt_escape_second_resource_native  escape_resource_plus_plain_matches_interpreter
    ken-kernel::recursive_head_totality_d0   d0_distinct_recursive_map_child
    ken-runtime  cranelift_backend::lowering::core::tests::constructors::two_same_shape_workers_are_distinguished
    ken-verify   scenario::tests::clock_wall_now_naive_exact_equality_is_wrong_on_correct_real_clocks
    ken-cli::rt_parity_native                buffer_allocate_malformed_capacity_narrows_to_invalid_bounds

**This is still not the bar.** It is a third tree, and `AC-2` governs: the count
is whatever D0 measures at the implementation base. It is offered because it is
nearer than the twelve and because it makes the delta legible.

> **READING THE ROSTER — TWO SURFACES GIVE THE WRONG ANSWER, ONE SILENTLY.**
> Measured on run `34753101365`:
>
>     grep '::notice title='  on the job log        ->  0     WRONG
>     annotations API                               ->  10    WRONG, silent
>     :628 "N selected; M passed"                   ->  11
>     '##[notice]' lines in the job log             ->  11
>     '- <identity>' roster lines in the job log    ->  11
>
> The runner rewrites `::notice title=X::body` to `##[notice]body`, so the
> literal source form never reaches the log. **The annotations API is capped at
> 10 per level per job** and dropped the eleventh row
> (`rt_parity_native buffer_allocate_malformed_capacity_narrows_to_invalid_bounds`)
> reporting no error. **Take the worklist from the job log's `- <identity>`
> lines, which are uncapped, and take the count from `:628`.**

    ken-cli::px7f_resource_native
      linked_public_escape_is_exact_closed
        "RT-CARRIED-RESOURCE-SCALAR: ... fails at base 21fd46dc"
    ken-cli::px8l_recursive_decl_native
      dynamic_zero_seed_takes_the_base_case
      dynamic_multistep_seed_preserves_updated_parameter_order
        "RT-BORROWED-INPUT-CARRIER-DURABILITY: ... traps as
         \"ken native trap: malformed borrowed process input\""
    ken-cli::px8ta_oriented_subcontinuation
      public_one_level_bracket_finishes_and_releases
        "RT-SITEOP-CARRIED-WITNESS D2: ... this row next refuses because ..."
      px8ds_real_same_depth_path_runs_exact_edges
        "focused native resource-cost row; run outside default suite"
    ken-cli::px8x_single_schema_observation
      linked_route_exposes_real_ordered_bindings_and_filters_reserved_input
        "RT-SITEOP-CARRIED-WITNESS D2: ..."
    ken-cli::rt_escape_second_resource_native
      escape_one_used_matches_interpreter
        "RT-CARRIED-RESOURCE-SCALAR: ... fails at base 21fd46dc"
      escape_resource_plus_plain_matches_interpreter
        "RT-SITEOP-CARRIED-WITNESS D2: ..."
    ken-cli::rt_parity_native
      buffer_allocate_malformed_capacity_narrows_to_invalid_bounds
        "RT-SITEOP-CARRIED-WITNESS D2: ..."
    ken-kernel::recursive_head_totality_d0
      d0_distinct_recursive_map_child
        "fixed-2MiB-stack worker; run via d0_..._requires_normal_child_exit"
    ken-runtime  cranelift_backend::lowering::core::tests::constructors
      two_same_shape_workers_are_distinguished
        "RT-WORKER-FIXTURE-DECODE: the worker fixture cannot run, so the AC-5
         comparisons are unreachable; fails at base 21fd46dc"
    ken-verify  scenario::tests
      clock_wall_now_naive_exact_equality_is_wrong_on_correct_real_clocks
        "ABI-A1 D4: demonstrates why exact instant equality is wrong"

### 2b. Nine of the twelve are labelled with a failure that no longer happens

Nine reasons name a **blocking defect** — *"fails at base 21fd46dc"*, *"traps
as ..."*, *"this row next refuses because ..."* — while the row passes. Two
measured facts bear on why:

- **`21fd46dce` is 2338 commits behind `origin/main`.** The labels were written
  against a base far enough back that their claims are not evidence about the
  current tree.
- **`RT-SITEOP-CARRIED-WITNESS` is `merged`.** Four of the nine cite its `D2`.
  A merged node's defect is the ordinary reason a row starts passing.

The other cited nodes: `RT-CARRIED-RESOURCE-SCALAR` (`draft`),
`RT-BORROWED-INPUT-CARRIER-DURABILITY` (`draft`), `RT-WORKER-FIXTURE-DECODE`
(`ready`).

**This is context, not a disposition.** A stale label is one explanation for a
pass; a vacuous test is another, and they are indistinguishable from the label
alone. `AC-1` is what separates them.

## 3. Deliverables

1. A per-row disposition for all twelve, each with the evidence that supports it.
2. The corresponding tree change (un-ignore, register, or relocate-and-delete).
3. A short ledger in the PR body: row, disposition, evidence, owning node.

## 4. The disposition menu — exactly three outcomes, and the row picks one

**D-READMIT — un-ignore.** The defect the label names is closed and the test is
a live control. **Requires `AC-1`.**

**D-REGISTER — keep ignored, add to `.github/ignored-test-exemptions.toml`.**
The row is ignored for a standing reason that is not a defect, so it should
never have been a sweep finding. Two rows are pre-classified here and the
implementer confirms rather than re-decides:

- `px8ds_real_same_depth_path_runs_exact_edges` — `policy-cost`. Ignored for
  runtime cost, exactly like the registered `b2v_ac10_..._at_thirty_thousand`.
- `d0_distinct_recursive_map_child` — a fixed-2MiB-stack **worker** driven by
  its parent `d0_..._requires_normal_child_exit`. Un-ignoring it would run the
  worker directly, which is what the parent exists to orchestrate. Class:
  `deferred-inert-control` or `policy-cost` as the readmission wording fits.

**D-RELOCATE — move the content out and delete the test.** Pre-decided by the
operator for exactly one row: `clock_wall_now_naive_exact_equality_is_wrong_on_
correct_real_clocks`. Operator: *"just expensive documentation ... should be
noted in code comments and/or spec and/or conformance and not in an ignored
test."* The claim must survive the deletion in at least one of those venues;
the implementer picks which and says why.

## 5. Acceptance

**AC-1 (the bar, and it is two-sided). A row may be READMITted only if it is
shown to FAIL when the behaviour it covers is broken.** Perturb the production
path the row asserts over — a deliberate local mutation, reverted before the
diff — and record that the row goes red.

**CORRECTED 2026-09-16. This sentence formerly read: *"A row that stays green
under that mutation is vacuous and must not be readmitted; report it as a
finding with the mutation used."* IT IS REFUTED AND MUST NOT BE APPLIED.** It
required the mutation to be **named**, never to **reach**, so a green produced
by a mutation that never executed was a fully compliant vacuity finding — and
false. Row 1 hit exactly that on its first attempt. **Replaced by `AC-1b`
below, which governs both arms; read it before disposing of any row.**

**Neither arm may be decided on the row's colour, and a green alone decides
nothing.**

This AC exists because a pass is not evidence the defect is closed. Nine of
these rows are labelled with a failure they no longer exhibit, and "the defect
was fixed" and "the assertion stopped reaching the behaviour" produce the same
green. Only the mutation separates them.

### AC-1a. The mutation protocol, and the clean check is a PRECONDITION

**Architect ruling, `evt_3faqxqkxynm57`.** *"Reverted before the diff"* above is
six words with no check attached to it. This is the check, and it belongs to the
**arriving** row rather than the departing one.

    # PRECONDITION of row N's measurement, before touching anything:
    git status --porcelain              # MUST be empty. Non-empty = STOP.

    BEFORE=$(git rev-parse HEAD:<path>)
    ...mutate, run the row, record red/green...

    git checkout -- <path>              # the one file, NAMED. Never `checkout <ref> -- .`
    AFTER=$(git hash-object <path>)
    [ "$AFTER" = "$BEFORE" ] || echo "REVERT DID NOT RESTORE -- STOP"
    git status --porcelain              # empty again

**An UNCOMMITTED leftover does not lose a measurement, it FABRICATES one in the
readmit direction.** Row N's mutation is still live when row N+1 is perturbed,
so row N+1 goes red — and red under mutation is exactly the evidence `AC-1`
accepts for readmission. **A genuinely vacuous row acquires an affirmative,
well-formed `AC-1` record, and the two-sidedness this AC exists to enforce is
defeated on its own terms.** It produces a plausible result rather than a diff,
which is why no downstream gate can see it. A *committed* leftover is the safe
arm: it appears in the candidate's `diff(merge-base, cand)` and `AC-5`'s CI run
almost certainly reds.

**THE PRECONDITION IS WHOLE-TREE, SO THE DISPOSITIONS MUST BE COMMITTED AS THEY
COMPLETE RATHER THAN ACCUMULATED.** Architect, `evt_kkf0ke3e4vf5`. Section 6
lists this WP's own surface — `crates/ken-cli/tests/`, `crates/ken-kernel/
tests/`, `crates/ken-verify/src/scenario.rs`, `.../core/tests/constructors.rs`,
`.github/ignored-test-exemptions.toml` — **and the dispositions are edits to
those paths.** If they pile up uncommitted, `git status --porcelain` fires on
your own work and becomes either a constant halt or a line you learn to skip.
Both lose the check. **A row boundary is a commit boundary**, and then the tree
is clean at every boundary by construction.

**ORDERING, which the clause above does not pin and which is the whole safety
property: the commit falls AFTER the blob-verified revert, never between the
mutation and the revert.** Per row: precondition clean, capture `BEFORE`,
mutate, run, revert, verify `AFTER == BEFORE`, **then** apply the disposition
edit and commit. That is what makes *"never commit a mutated production path"*
mechanical instead of careful — no commit boundary can fall while a mutation is
live.

A path-scoped precondition does **not** substitute for this. The mutations and
the dispositions land on the *same* files, so no path partition separates a leak
from legitimate work. The separation available here is temporal.

### AC-1b. A GREEN UNDER MUTATION IS NOT A VERDICT. It is two facts, one number.

**Added 2026-09-16 by the Steward, after row 1 refuted `AC-1` as written.**

`AC-1` above says a row that stays green under the mutation *"is vacuous and
must not be readmitted; report it as a finding with the mutation used."* **That
is unsound, and row 1 is the counterexample.**

    ken-cli::px7f_resource_native  linked_public_escape_is_exact_closed

    attempt 1  resolve_fs_handle, Retired arm -> MalformedResource     GREEN
    attempt 2  resolve_fs_handle, BOTH Closing and Retired arms        GREEN
    attempt 3  lookup, stale-generation arm -> RightNotHeld            RED

The row's `Closed` comes from the generation check in `lookup`, which **returns
before the state match is ever reached** — so the resolver arms perturbed in
attempts 1 and 2 are not on the row's path at all.

⇒ **Reporting "vacuous, mutation used = the Retired arm" after attempt 1 would
have SATISFIED `AC-1` exactly.** It requires the mutation to be named; it does
not require the mutation to reach. The row is not vacuous, and the finding would
have been confident, well-formed, and false.

**A green under mutation has two causes and they produce one number:**

    the row does not observe the behaviour          -> VACUOUS
    the mutation never executed on the row's path   -> INERT MUTATION

**Only evidence of reach separates them, and a green cannot supply it.**

**THE DISCRIMINATOR, required before any row is reported vacuous:** show the
mutation site is **executed** on the row's path. The cheap mechanical form is a
**reach probe** — replace the semantic mutation with an unconditional
`panic!("reach probe")` at the *same site*, and run the row:

    row REDS on the probe    -> the site IS reached.
                                A green on the semantic mutation then means the
                                row genuinely does not observe it -> VACUOUS,
                                and the finding is sound.

    row GREEN on the probe   -> the site is NOT reached. The mutation was
                                INERT and proves nothing about the row.
                                Find a site on the row's actual path.
                                DO NOT report vacuous.

The probe reverts under `AC-1a`'s protocol like any other mutation — it is a
mutation, and the blob-verified revert and commit ordering apply to it
unchanged.

**No row may be reported vacuous without a reach probe that reds.** A
readmission needs no probe: a red on the semantic mutation already proves reach.
**The probe is owed only by the arm that would otherwise be unfalsifiable.**

#### THE PROBE PROVES EXECUTION. IT DOES NOT PROVE OBSERVABILITY.

**Narrowed 2026-09-16 on the implementer's objection, before any row was
withheld on it.** A `panic!` reds if the site executed **at least once in the
run** — not if it executed **on the path the assertion depends on.** Those come
apart whenever a site serves more than one caller:

    site executed during SETUP but not during the asserted operation
      probe                REDS   (it was reached)
      semantic mutation    GREEN  (the assertion never observes it)
      naive reading        "reach proven + semantic green => VACUOUS"
      truth                the row may be perfectly well coupled elsewhere

**Row 1 is the live example.** `lookup` is called by every resolver, so a probe
there reds on the `FsOpen` at sequence 0 — long before the assertion's
`FsHandleMetadata` at sequence 2. **Reach at `lookup` was never in doubt; which
of its three exits the assertion observes was the entire question.**

⇒ **A red probe kills exactly one hypothesis: "the site never executed."** That
is the hypothesis attempt 1 died of, and it is worth a required check. **It does
not establish that the mutated value was observable by the assertion**, which is
the stronger property a vacuity verdict actually needs, and **no probe can
supply it** — it is an argument about the data path, not a measurement.

**So the record carries the argument explicitly rather than letting a red probe
stand in for it.**

**Record per row — FOUR facts, not one:**

    1  the mutation site
    2  the probe result at that site        MEASUREMENT: did it execute
    3  the semantic-mutation result         MEASUREMENT: did the row notice
    4  the path from the mutated value to the assertion that reads it
                                            ARGUMENT: why it should have

**Item 4 is weaker than items 2 and 3 and is labelled so.** It is required
because it is the piece the probe structurally cannot supply. **A vacuity
finding missing item 4 is incomplete even with a red probe** — the point of
writing it down is that the gap becomes visible instead of hiding behind a
measurement that does not cover it.

#### AC-1b FINAL. Architect ruling `evt_78h2ygj75jeqg`, three amendments.

**The probe proves EXECUTION, not OBSERVABILITY.** A red probe shows the site
runs on the row's path. It does not show the mutated value is *visible to the
assertion* — a site serving setup as well as the asserted operation reds the
probe without ever reaching the assertion. **A vacuity verdict therefore reads
"the row does not observe SITE X", never "the row is vacuous".** To report a row
vacuous outright, state item 4 above and name the site as the one the assertion
depends on.

**Neither arm may be decided on the row's colour.** A red row and a green row
each have several producers, and only one of them is the mutation.
**Readmission** requires the mutated value observed at the assertion (row 1:
`event 2 outcome Error(Resource(RightNotHeld{0,0}))` where `Closed` is
asserted). **Vacuity** requires the probe's own signature in the output — the
literal `reach probe` string, or exit status 101 where the row's own outcomes
are 0 / 91. **A red row without the signature is not a probe result.**

**"The same site" means the same EXPRESSION — the exact match arm replaced,
recorded as `file:line`.** Not the same function and not the same file: a probe
on a neighbouring arm proves reach for code the mutation never touched.

**Record per row, four facts:** the mutation site; the probe result *and its
signature* at that site; the semantic-mutation result; and the path from the
mutated value to the assertion that reads it.

> **Why this needed three passes.** `AC-1` did not require the mutation to
> reach. `AC-1b`'s first draft required reach but licensed a row-level verdict
> from a site-level fact. Its second draft still **read the probe off the row's
> colour** — specifying a one-bit measurement with four producers, inside an
> amendment whose whole rationale is that a one-bit measurement with two
> producers cannot discriminate. **Each pass fixed the previous one's version of
> the same error.** The signature requirement is what finally makes the probe a
> measurement of the probe rather than of the run.
>
> `panic!` is confirmed safe as the instrument: it diverges, so it type-checks
> wherever the mutated expression did, and there is **no `deny(warnings)` and no
> `[workspace.lints]` in the repo**, so its `unreachable_code` warning cannot
> break the build (Architect, verified).

> **Why this was worth stopping for.** The failure `AC-1` exists to prevent is a
> row readmitted on a pass that proves nothing. **This defect is the same shape
> pointed the other way** — a row *withheld* on a green that proves nothing —
> and `AC-1`'s own two-sidedness claim did not cover it. Ten rows remain; the
> implementer found this on the first one, and only because two mutations came
> back green and the third was tried anyway.

**A non-empty precondition is a finding to REPORT, not a state to clean up and
continue from.** Every row measured after the leak began is suspect, and which
rows those were is knowable only if the stop is loud.

**"I ran the revert" and "the file is back" are two claims**, and only the blob
comparison settles the second.

**Why a precondition and not a postcondition:** a postcondition runs only when
something remembers a revert is owed, so it fires exactly in the cases that were
already going to be fine. As a precondition it fires whether or not anything
remembers, and a compaction mid-row is harmless because the check belongs to the
next row. **A check that runs only when you remember the condition it is
checking for is not a check.**

**This hazard does not require a compaction** — an ordering slip inside one turn
does it just as well. Landing `D0` separately and compacting at a seam reduce
the exposure; only the precondition closes it.

**AC-2. Every row in the PASSING SET AS MEASURED AT THE IMPLEMENTATION BASE has
a disposition, and the dispositions partition that set.** No row is left as a
passing ignored row. **The count is whatever D0 measures — twelve is what
`0f71ab5b9` gave and it is not the bar.** An implementer who measures thirteen
must disposition thirteen; one who measures eleven must not manufacture a
twelfth. **An AC that pins a count taken on another tree is unsatisfiable
honestly, and forcing it is the likelier outcome than failing it.**

**AC-3. No row outside the passing set changes disposition.** The failing
rows and the six registry exemptions are out of scope; a diff that touches them
fails this AC. Control: the sweep's own selected count must still reconcile —
`selected + registry == nextest ignored population` — with the registry total
moved only by rows this WP registers.

**AC-4. The relocated clock claim is readable at its new home without the
test.** Cite the file and line in the ledger. Deleting the test without
relocating the claim fails this AC.

**AC-5. No-regression in CI** — workspace-green in CI, not a local
`--workspace` run.

## 6. Contention

Touches `crates/ken-cli/tests/`, `crates/ken-runtime/src/cranelift_backend/
lowering/core/tests/constructors.rs`, `crates/ken-kernel/tests/`,
`crates/ken-verify/src/scenario.rs`, and
`.github/ignored-test-exemptions.toml`.

`crates/ken-verify/src/scenario.rs` is the one file outside Runtime's usual
surface; the change there is a deletion plus a doc relocation. Check for an
open Verify candidate against that file before releasing.

## 7. Proposed successor, not in scope here

Making passing-while-ignored **blocking** rather than `findings non-blocking`
is the anti-recurrence half. It is with the operator and is deliberately not an
AC of this WP: turning the gate red before the backlog is dispositioned would
red CI on twelve known rows.
