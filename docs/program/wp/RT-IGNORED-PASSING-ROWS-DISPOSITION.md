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
         SOURCE census -- cfg-BLIND, counts text
    +1   macro-generated (discharge_ledger_refuses_one_word_discharging_two_obligations)
    34   nextest --run-ignored=only population
         the COMPILED population under this sweep's cfg + feature set
    -6   registry run-exemptions
    28   selected and executed  ->  12 PASSED, 16 failed

**`34` is not ground truth for "what ignored tests exist", and an earlier
revision of this block called it that.** The two populations answer different
questions and move in opposite directions:

    macro-generated rows   push nextest UP    (compiled, absent from source text)
    cfg-excluded rows      push nextest DOWN  (in source text, never compiled)

A test that is both `#[ignore]`d and cfg-excluded is invisible to
`--run-ignored=only`, which is the instrument people reach for to ask *"what are
we not running?"*. **A gated-out item is ABSENT from the analysis, not EXEMPT
from it.**

**What that arithmetic already bounds, and it is the cheapest finding in this
frame.** Both numbers above were taken at the same SHA, so the subtraction is
legal. If `k` is the cfg-excluded population, nextest should read `34 - k`. It
reads exactly `34`, so **`k = 0` at `0f71ab5b9` under that sweep's profile.**

Two limits, both load-bearing:

- **Evidence, not proof.** The identity also holds if one cfg-excluded row
  (`-1`) coincides with one uncounted macro-generated row (`+1`). A total cannot
  separate them — which is why the control below is required rather than nice to
  have.
- **One SHA, one profile.** This is *measured zero at `0f71ab5b9`, unverified
  since*, not *structurally zero*. A re-check is one subtraction off any later
  `mode=full` sweep; no local build, no `--workspace`.

**REQUIRED CONTROL — one expected member per profile.** Name, in this block, one
test already known to be gated each way (platform cfg; feature not enabled by
the sweep). Each named member must appear in the gap. Without this a zero result
is indistinguishable from *"the instrument could not see any of them"*, and on
this instrument that is a live possibility rather than a pedantic one — the whole
finding is that nextest's view is narrower than the source census's.

**Report the gap ATTRIBUTED TO CFG PROFILE, never as a total.** A row excluded by
a feature the sweep does not enable and a row excluded by a platform cfg land in
the gap identically and are different findings: the first is includable by
flipping a feature, the second is not. A bare count says *how many* are
invisible; the frame needs *which, and what would include them*. Only the second
can be acted on.

**A source grep alone is still the wrong instrument for the executed population
and must not be used to re-derive `28`.** `grep -c '#\[ignore'` returns 38: five
are doc-comment prose *about* `#[ignore]` (`recursor_fusion.rs` x2,
`r3_c1_source_arrival.rs` x1, `boundary_value_clif.rs` x2), and a grep is blind
to the macro-generated row. `scripts/ci-ignored-sweep.py` derives its population
from nextest for exactly that reason. **The source census earns its place beside
nextest, not instead of it** — it is the only side that can see a cfg-excluded
row, and that is precisely why the two are compared rather than one being
preferred.

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

**AC-2. Every one of the twelve rows has a disposition, and the dispositions
partition the twelve.** No row is left as a passing ignored row.

**AC-3. No row outside the twelve changes disposition.** The sixteen failing
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
