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

    33   real `#[ignore]` attributes in crates/, all on `#[test] fn`
    +1   macro-generated (discharge_ledger_refuses_one_word_discharging_two_obligations)
    34   nextest --run-ignored=only population   (GROUND TRUTH, carries to main)
    -6   registry run-exemptions                 (carries to main)
    28   selected and executed                   (carries to main)
         -> 12 PASSED / 16 failed  **AT `0f71ab5b9` ONLY. NOT A FIXED INPUT.
            RE-DERIVE AT THE IMPLEMENTATION BASE — see below.**

> **THE POPULATION CARRIES TO `main`; THE PASS/FAIL SPLIT DOES NOT.** Verified
> between `0f71ab5b9` and `origin/main`: `#[ignore]` attribute lines differing
> across `crates/` is **zero**, `.github/ignored-test-exemptions.toml` is
> **identical**, and ten of the eleven named test files are identical. So
> `34`, `-6` and `28` are properties of the population and are safe to pin.
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
`grep -c '#\[ignore'` returns 38: five of those are doc-comment prose *about*
`#[ignore]` (`recursor_fusion.rs` x2, `r3_c1_source_arrival.rs` x1,
`boundary_value_clif.rs` x2), and a grep is simultaneously blind to the
macro-generated row. `scripts/ci-ignored-sweep.py` derives its population from
nextest for exactly this reason.

### 2a. The twelve rows, with the ignore reason each currently carries

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
diff — and record that the row goes red. A row that stays green under that
mutation is **vacuous** and must not be readmitted; report it as a finding with
the mutation used.

This AC exists because a pass is not evidence the defect is closed. Nine of
these rows are labelled with a failure they no longer exhibit, and "the defect
was fixed" and "the assertion stopped reaching the behaviour" produce the same
green. Only the mutation separates them.

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
