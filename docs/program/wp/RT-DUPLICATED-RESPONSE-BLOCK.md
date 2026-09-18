# RT-DUPLICATED-RESPONSE-BLOCK — work package

**Owner: Team Runtime. Size M. Tier T1. Gate: none.**
**Implementation base: `origin/main` at
`60df2cfd2ff4273ffdf5002d92d27d05c8a51391`.**

Operator priority, 2026-09-15: *"The other tests should be fixed."* Operator,
2026-09-17: *"Is L1 still working on clearing the ignored tests? That is the top
priority until it is done."* Third repair node from the
`RT-IGNORED-FAILING-ROWS-INVENTORY` ledger.

> **Every coordinate, count, line number and status in this frame is
> PERISHABLE and was measured at the base SHA above.** Re-ground each one
> against `origin/main` before you act on it — including the prose. A frame is
> not a status source and a line number is not a binding. If a re-measurement
> disagrees with this document, the tree wins and the disagreement is a
> finding worth reporting, not a discrepancy to reconcile silently.

## 1. Objective

Decide **where the duplicated host-response block enters the plan**, and on
that answer either repair it or establish that the construction-time collision
check is asserting a property the plan was never required to have.

**Both outcomes close this node.** A duplication that turns out to be
legitimate, with the check retired or moved as a result, is as good a result as
a planner defect found and fixed, and it is reported the same way. What does
not close this node is a repair that relaxes the collision check without
deciding which of the two readings holds — that is the move
`RT-HOST-RESPONSE-ROUTE-KEY-COLLISION` already built, measured, and reverted.

**Out of scope, explicitly.** `[[RT-FRAME-MARKER-ONCE]]` is the second refusal
underneath the two `px7n` rows. It is a separate node at `draft`. Do not
absorb it, do not repair it here, and do not let a row's continued failure
after this node's repair read as this node failing.

## 2. Fixed inputs, measured at `60df2cfd2`

**The producer:**

    crates/ken-runtime/src/cranelift_backend/planning/
      static_transition/responses.rs

    fn host_response_routes(...)               the construction-time map
      routes.insert(case.constructor.clone(), route).is_some()
        -> planner_error("two host response cases claim one operation
                          constructor")                        line 1281
    fn selected_host_response_route(...)       the point-of-use selector,
                                               which carries its own
                                               `selected.is_some()` refusal

**The four rows, by FILE and LINE:**

    crates/ken-cli/tests/px7n_nested_computational_eliminator.rs:149
    crates/ken-cli/tests/px7n_nested_computational_eliminator.rs:170
    crates/ken-cli/tests/rt_escape_second_resource_native.rs:653
    crates/ken-cli/tests/rt_escape_second_resource_native.rs:713

The two `px7n` rows build the **same** program (`const PROGRAM`, reached via
`assert_case`), so one census covers both. The two `rt_escape` rows build
**different** programs and **no census value transfers between them.**

**The controls, already measured and already in the tree:**

    S6  escaped_buffer_used_by_fanning_host_op_matches_interpreter
        same file as :653/:713, gets PAST response-route construction,
        fails on typed consumer projection instead
    S8  sp_a_foreign_span_freeze_rejects_own_span_succeeds_on_both_engines
        emits no collision at all

**Predecessor evidence, landed at `ef11485dd` (annotation-only, zero
production change), sections 9.5 and 9.8 of its frame.** Its section 3
branches — *fix the key* and *the tests are wrong* — are BOTH refuted. Do not
re-open either.

**Baseline to reproduce before changing anything:**

    ken-cargo test -p ken-runtime --lib

The predecessor measured **1035 passed / 0 failed / 2 ignored** at its own
base. Re-measure at THIS base rather than carrying that number forward; if it
differs, that difference is the first finding of the turn.

**Local build discipline (`COORDINATION §12`).** Targeted only, through
`scripts/ken-cargo`, scoped `-p ken-runtime` or `--test <name>`. Never
`--workspace` — the full build, the `--locked` gate and the conformance suite
run in CI.

## 3. D0 — the forcing step, with its terminating observation written first

**D0 is the whole first turn and it lands no production change.**

Instrument `host_response_routes` to report, for one program, every
`(case.constructor, occurrence.static_origin, body, effect_origin)` it visits
— every insert, not only the colliding second one. Run it on
`px7n-nested-computational-eliminator`.

**Write the answer before you look.** The observation that ends D0 is a
decision between exactly these, and both are terminating:

- **(1) EMITTED TWICE.** The same `occurrence.static_origin` is visited more
  than once, or two distinct `static_origin` values carry byte-identical case
  rosters at a constant offset. ⇒ The duplication is upstream of the check,
  the check is faithful, and the repair site is whatever populates
  `plan.source_occurrences`.
- **(2) VISITED ONCE, SHAPE IS REAL.** Every `static_origin` is visited
  exactly once and the 29 collisions come from genuinely distinct occurrences
  that legitimately share constructors. ⇒ The duplication is not a defect, and
  the construction-time uniqueness assertion is the wrong instrument.

**A third layer is an ANSWER, not a rung.** If the census shows something
neither (1) nor (2) describes, that is the deliverable — report it and stop.
Do not invent a third reading mid-turn and follow it; bring it back.

**The constant delta is a symptom, not a diagnosis.** 365 and 317 came from
counting deltas, not from reading the producer. D0 exists precisely because
"duplicated block" is the phrasing the symptom suggested, and nobody has yet
read the code path that would make it true.

## 4. Deliverables

- **D0** — the census above, its instrument, and a verdict of (1), (2), or a
  named third answer. Lands as evidence; no production change.
- **D1** — on (1): the repair at the site D0 names, with the four rows
  re-measured individually and each given a named disposition.
- **D1'** — on (2): the check retired or relocated to the point of use, with
  AC-4 below discharged — the thing the predecessor could not reach.
- **D2** — the four `#[ignore]` labels rewritten to whatever D0/D1 establish,
  including for any row that stays ignored. A row that stays ignored gets its
  reason restated in current terms, not left carrying a prediction this node
  has since tested.
- **D3** — the three-program census table in the issue node re-measured at
  this base and corrected in place if it has moved.

## 5. Acceptance criteria, each with its control

- **AC-0. The census is counted by the instrument, not by grep.** The probe
  emits its own totals and the run reaches completion.
  **Control:** a grep-derived count of the same quantity is recorded alongside
  it. If the two agree, say so; if they disagree, the instrument's number is
  the measurement and the disagreement is reported.
- **AC-1. Each of the four rows gets an individually named disposition** —
  readmitted, or staying ignored with a current reason. No row is dispositioned
  by inheriting its file-mate's result.
  **Control:** `rt_escape:653` and `rt_escape:713` are different programs.
  A disposition that cites one census for both fails this AC.
- **AC-2. `S6` does not acquire a collision.** Whatever account D0 reaches must
  not predict one in `S6`.
  **Control:** run `S6` against the instrumented build and record that it still
  gets past response-route construction. An account that would collide there is
  refuted on arrival.
- **AC-3. No regression.** `ken-cargo test -p ken-runtime --lib` at least
  matches the baseline re-measured in section 2. Workspace-green means **green
  in CI**, never a local `--workspace` run.
- **AC-4. On outcome (2) only — the relaxation is justified against PURPOSE.**
  State what the uniqueness assertion was protecting, and why that protection
  either survives at the point of use or was never real. This is the AC the
  predecessor recorded as "not reached"; reaching it is what makes outcome (2)
  a result rather than a gate being switched off.
- **AC-5. `[[RT-FRAME-MARKER-ONCE]]` is untouched and still `draft`**, and the
  two `px7n` rows' continued failure on it after a successful D1 is reported as
  the expected outcome rather than as this node's failure.

## 6. Contention

**Paths this node expects to touch:**

    crates/ken-runtime/src/cranelift_backend/planning/static_transition/
      responses.rs
    crates/ken-cli/tests/px7n_nested_computational_eliminator.rs
    crates/ken-cli/tests/rt_escape_second_resource_native.rs
    docs/program/issues/RT-DUPLICATED-RESPONSE-BLOCK.md
    docs/program/wp/RT-DUPLICATED-RESPONSE-BLOCK.md

**Checked at framing against the routed-and-unlanded set — SIX candidates,
each measured as `diff(merge-base, candidate)` because that is what a squash
lands:**

    f5a195abb  e3bfa8bbe  ed8d01af8  c65263f17  4f123f3f9  d8a8719655

**The intersection with this node's three code paths is EMPTY.** Not one of
the six touches `static_transition/responses.rs`,
`px7n_nested_computational_eliminator.rs`, or
`rt_escape_second_resource_native.rs`.
`RT-HOST-RESULT-ARM-SHAPE-DISAGREEMENT` (`d8a8719655`) is the one worth naming
because it is the re-homed residue of `RT-CARRIER-PRODUCER-OCCURRENCE` and
concerns row 684 of that same test file — but it is **docs-only** and edits no
test. Its row 684 is outside this node's population either way.

**Path-disjoint is not behaviour-disjoint, and this check expires.** Full CI
at M4 is the instrument for the behavioural half; a red there is a finding, not
a rebase trigger. Re-take this intersection against `origin/main` before
cutting — the routed set moves.

## 7. Tier

**T1.** The work turns on deciding between two readings of one measurement and
on justifying a fail-closed gate against its purpose — a diff whose review
turns on an argument, not on byte-faithfulness. The predecessor reached a
correct answer by building a repair and then declining to land it, which is the
judgment this tier buys.
