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

> **BOTH OUTCOME SENTENCES ARE WEAK INSTRUMENTS AND THE FRAME SAYS SO.**
> *"Every `static_origin` is visited exactly once"* is close to unfailable as
> written, and (1) as written cannot distinguish a genuine second visit from
> two origins that merely look alike. **What carried `D0` in practice was the
> `S6` control, not either sentence.** `D0` is behind this node now, so this
> costs the current run nothing — but **do not let outcome (2) be reached by
> the weak reading and then treated as a licence for `D1'`.** `D1'` is gated
> in §3a below regardless of how (2) was arrived at.

**A third layer is an ANSWER, not a rung.** If the census shows something
neither (1) nor (2) describes, that is the deliverable — report it and stop.
Do not invent a third reading mid-turn and follow it; bring it back.

**The constant delta is a symptom, not a diagnosis.** 365 and 317 came from
counting deltas, not from reading the producer. D0 exists precisely because
"duplicated block" is the phrasing the symptom suggested, and nobody has yet
read the code path that would make it true.

> **EXPLAIN THE CONSTANT, NOT JUST THE DUPLICATE. Restored in the fold.** A
> cause that predicts a duplicate but not a *uniform offset* has not explained
> what was measured.
>
> **And the delta differs BETWEEN programs (`365` vs `317`) while being
> constant WITHIN each.** That is a live constraint on the cause: whatever
> duplicates the block does so at a point whose origin-id distance is
> **program-relative**. ⇒ **A cause that predicts a fixed constant across
> programs is refuted by this before you write it down.**

## 3a. THE DESIGN FORK — A CITED RULING SETTLES IT, YOU DO NOT

> **RESTORED 2026-09-18. This section and `AC-6` were lost in the fold and are
> the reason the fold was not complete.** They stood in the frame of the
> duplicate node that was closed into this one. The duplicate's *node* text
> moved; **its frame's gate did not**, and a gate is what a frame is for. See
> the banner in §0 of the issue node. Architect, `evt_261zdt47mgr9p`.

**Is the invariant `one host-operation constructor ⇒ one response-handling
site in the whole program` too strong?**

- **Arm A — NO, the invariant is right.** Every second entry is a
  materialization artefact; no admissible Ken program has two distinct
  response-handling sites for one constructor. Then the repair is the
  de-duplication found in `D0`, the invariant is untouched, and **this node
  closes here.**
- **Arm B — YES, it is too strong.** Some admissible program legitimately has N
  handling sites. Then the repair is to make routing occurrence-aware — pairing
  N `Vis` sites to N handlers — **which is planner work and is a SUCCESSOR
  NODE, not this one.** This node closes by delivering the ruling and the `D0`,
  and the Steward cuts the successor.

**Do not pick an arm to get started.** The two have disjoint deliverables. The
predecessor diagnosis's surviving *"too strong"* verdict **is not a ruling** —
its only supporting case was measured false and no replacement was offered.
See the node's *"THE CONCLUSION THAT OUTLIVED ITS PREMISE"*.

**This matches what the issue node already says and the frame previously did
not:** *"This node does not pick. The fork is a design question about
admissible Ken programs and it routes to the Architect with a measurement in
hand."*

## 4. Deliverables

- **D0** — the census above, its instrument, and a verdict of (1), (2), or a
  named third answer. Lands as evidence; no production change.
- **D1** — on (1): the repair at the site D0 names, with the four rows
  re-measured individually and each given a named disposition.
- **Dr — THE RULING ON §3a's FORK, cited by `evt_`/`dec_` id, with the arm
  recorded.** This is a deliverable, not a courtesy. It is required before any
  edit to `host_response_routes`' key or loop shape, and before any retirement
  or relocation of the uniqueness assertion.
- **D1'** — on (2): **GATED BEHIND `Dr`, and only if the ruling returns Arm
  A.** If the ruling returns Arm B, the repair is a successor node and **this
  node closes on `Dr` + `D0` without touching the check.** The relaxation is
  not this node's to perform on its own judgment.
- **D2** — the four `#[ignore]` labels rewritten to whatever D0/D1 establish,
  including for any row that stays ignored. A row that stays ignored gets its
  reason restated in current terms, not left carrying a prediction this node
  has since tested.
- **D3** — the three-program census table in the issue node re-measured at
  this base and corrected in place if it has moved.
- **D4 — a registry row in `.github/ignored-test-exemptions.toml`** for any row
  readmitted with an accepted red. **Also lost in the fold; restored.**

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
  **This AC is INPUT TO `Dr`, not a substitute for it.** Discharging it is what
  gives the Architect a measurement to rule on; it does not authorize the
  relaxation by itself. An implementer who satisfies `AC-4` and proceeds has
  taken the decision §3a reserves.
- **AC-6. THE FORK IS ANSWERED BY A CITED RULING BEFORE ANY PLANNER EDIT.**
  Record the `evt_`/`dec_` id and the arm.
  **Control, and it is mechanical:** an edit to `host_response_routes`' key or
  loop shape, or a retirement or relocation of the uniqueness assertion, that
  lands without that citation is **out of scope by construction** — not a
  judgment call, not a close one. Check the diff for those three shapes before
  handing off.
- **AC-5. `[[RT-FRAME-MARKER-ONCE]]` is untouched and still `draft`**, and the
  two `px7n` rows' continued failure on it after a successful D1 is reported as
  the expected outcome rather than as this node's failure.

> ### AC-7 THROUGH AC-10 ALSO FAILED TO TRAVEL IN THE FOLD. RESTORED 2026-09-18.
>
> **The Architect found the fork gate. Enumerating the closed frame's
> obligations BY POSITION found four more** — which is the rule earning its
> keep in the same hour it was written: checking by recall found one, checking
> by position found five. **Two of these are the controls that catch a guard
> which has been deleted rather than satisfied.**

- **AC-7 (control, REQUIRED). The repair is shown to be REACHING THE SITE.**
  Revert the `D0`/`D1` repair and confirm the rows you readmitted go RED again
  with the collision message.
  **If they stay green without your change, the rows were not gated on what you
  fixed** — and that is a finding to report, not a quiet pass.
- **AC-8 (control, REQUIRED). The invariant STILL REFUSES SOMETHING.** If the
  repair removes the duplication, the uniqueness guard now fires on nothing in
  these programs. **Name one case it must still refuse and show that it still
  refuses** — a construction, a targeted unit test, or a stated argument from
  the plan shape.
  **A check that refuses nothing has been deleted rather than satisfied**, and
  a silent deletion is exactly how this cluster's labels went wrong the first
  time.
- **AC-9. The crate set is DERIVED, not named.** Compute the reverse-dependency
  closure over your touched set to a fixpoint and test that set. **Never
  `--workspace`** (`COORDINATION §12`) — the workspace build and the
  conformance suite run in CI. **State your target selection beside the
  claim:** `cargo check` does not compile `#[cfg(test)]`, so a green `check` is
  **not** evidence that any test built.
- **AC-10. The second blockers are checked and NOT pulled in.** For each row
  that does not readmit, say which blocker it now stops at, **measured**.
  Specifically: does the `px7n` pair reach `[[RT-FRAME-MARKER-ONCE]]`'s
  frame-marker message, and does anything under the `rt_escape` pair confirm or
  refute `[[RT-CLOSURE-BOUNDARY-LANE]]`? **Both answers are findings for the
  Steward to re-cut on. Do not absorb either into this node, and do not record
  "undetermined" as "ruled out".**

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
