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

## 8. Measured outcome

**Everything below was measured at `origin/main`
`9dfa6978ebf6c86e5d4ebafe8f2ecf6a565a1158`** unless a sentence names another
revision. `D0`, `D1` and the `AC-4` measurement were first taken at the framing
base `60df2cfd2`; between that base and `9dfa6978e` the only crate file that
moved is `ken-elaborator/src/parser.rs` (+108/-25), and `ken-runtime` and
`ken-cli` are byte-identical across the range. The parser was therefore the
single route by which a census number could have moved, and `D3` measured that
it did not. Every probe used here was environment-gated, is reverted, and
`grep -c RTPROBE` over the touched files returns zero.

### 8.1 `D0` — the census, and why NEITHER section-3 branch could decide it

**The observable signature of `(1)` holds: the block is presented twice.** The
census, counted by the instrument, is in section 8.5.

**But the diagnosis attached to `(1)` does not follow, and `(2)`'s
discriminating clause could not have come out false.** That is the finding of
`D0`, and it is the "named third answer" section 3 asks for rather than either
listed branch:

- `(2)` says *every `static_origin` is visited exactly once*. That is
  **guaranteed by construction**, not measured: `plan.source_occurrences` is
  dense by origin ordinal, a node's origin **is** its index in it, and
  `record_source_occurrence` refuses a second write at one origin as a
  `PlannerInvariant`. A clause that cannot be false cannot discriminate.
- `(1)` says two distinct origins carry byte-identical case rosters at a
  constant offset. That is **exactly what correct code produces** whenever two
  call sites of one proc are inlined. Observing it does not distinguish a
  planner defect from a planner working correctly.

⇒ **Both branches were written over the same observation and neither
separates the readings it was supposed to separate.** The census is sound; the
dichotomy it was commissioned to settle was not a dichotomy.

### 8.2 `D1` — where the second presentation enters, measured

`px7n-nested-computational-eliminator`, `const PROGRAM`, proc `main`:

    Cons _ tail |-> match tail {
      Nil      |-> wrap_again (\_. wrap_result True);
      Cons _ _ |-> wrap_again (\_. wrap_result False)
    }

Both arms call `wrap_again`, so inlining instantiates the
`wrap_again -> relay -> wrap_result` chain **once per arm** and presents one
program's response cases to `host_response_routes` twice.

**The discriminating run: a single-arm fixture, with the production check
LIVE.** Collapsing the inner `match tail` to one arm removes the collision
entirely, and the row then advances to
`OrientedSubcontinuationPlanV1: checked Runtime frame marker was consumed more
than once` — `[[RT-FRAME-MARKER-ONCE]]`, which this node does not own. The
fixture was reverted.

⇒ **The check is FAITHFUL: it reports a real second presentation, and the
presentation is what inlining two call sites of one proc produces.** That much
is measured.

**CORRECTED — an earlier revision of this section went one step further and
said "and the plan is CORRECT". It should not have.** That is an inference, not
a measurement, and it is exactly the step the node's own folded
premise-interrogation forbids taking without measuring. Two readings survive
the fixture result and nothing here separates them:

    the second instantiation is LEGITIMATE   -> the invariant is too strong
      -- per-arm inlining of a shared             and the route map must be
      callee is what the backend is                keyed on the OCCURRENCE
      supposed to do

    the second instantiation is ITSELF       -> the invariant is CORRECTLY
      the artefact -- the shared chain             refusing a plan that should
      should have been materialized once           never have held the block
      and reused                                   twice, and the repair is
                                                   upstream of the route map

**I measured WHERE the second presentation comes from. I did not measure
whether producing it is correct**, and the fixture cannot tell: collapsing the
match to one arm removes the second call site, which removes the collision
under **either** reading.

The node says this fork "routes to the Architect with a measurement in hand"
and that this node does not pick it. **It does not pick it here.** The
measurement in hand is 8.2 and 8.3.

**RULED, and neither arm survived — see 8.9a.** Both arms above are stated
over the **producer**; the Architect decided it on the **consumer**, which
neither arm mentions.

### 8.3 `AC-4` — deleting the check is not the repair, measured

`AC-4` is written for outcome `(2)`. The outcome is the third answer, so it is
discharged here in the form that actually bears: **what the uniqueness
assertion protects, and what is lost if it is simply removed.**

With the refusal suppressed and nothing else changed — the exact behaviour of
deleting `responses.rs:1281` — each invocation of `host_response_routes`
performs **29 silent overwrites, dropping 29 constructors' wiring
coordinates** (`effect_origin`, `producer_call_origin`, `response_origin`), and
**nothing refuses**: not at construction, and not at the point of use, where
`selected_host_response_route`'s own *one Vis operation subtree selects more
than one host response producer* does **not** fire. The last-written route is
simply used.

⇒ **The assertion is protecting route wiring, and the protection is real.**
Removing it does not relax a gate; it silently mis-wires. The key must gain the
occurrence, not lose the check.

### 8.4 `D2` and `AC-1` — four rows, four named dispositions, none readmitted

**All four STAY IGNORED.** No row is dispositioned by inheriting its
file-mate's result, and the two `rt_escape` rows are measured separately as
`AC-1`'s control requires.

    px7n:149   STAYS IGNORED. Collision is the first stop; with it suppressed
               the row reaches RT-FRAME-MARKER-ONCE. Label rewritten.
    px7n:170   STAYS IGNORED. Same program, measured in the same run; the
               census covers both rows because they build one program.
    esc:653    STAYS IGNORED. Program rt_escape_escape_file_then_readat,
               censused on its own. With the collision suppressed it reaches
               ComputationalMatch: tree-producing match scrutinee is not Bool
               or a constructor -- a SECOND blocker, newly seen.
    esc:713    STAYS IGNORED. Program rt_escape_nat_fanout_escaped, censused
               on its own, same second blocker.

**The `RT-CLOSURE-BOUNDARY-LANE` question on the two `esc` rows stays
UNDETERMINED.** Something has now been seen past the collision, but what it
reached is a different mechanism, and nothing has seen past **that**.

**Coordinate convention.** Row coordinates are the `#[ignore]` **attribute**
line. The predecessor frame's section 9.6 carried the `fn` lines
(`654 714 150 171`) and is corrected to `653 713 149 170` in the same commit,
with a note recording why. That file is not in this node's section 6 path list;
it is one docs file, coordinates only, no claim altered.

**Owner.** Each label's owning ID moves from `RT-HOST-RESPONSE-ROUTE-KEY-
COLLISION`, which is landed and cannot close these rows, to
`[[RT-DUPLICATED-RESPONSE-BLOCK]]`. The successor that owns the key change is
framed and **not on main at the time of writing**, so it is deliberately **not
named in any label** — a row pointing at an ID with no node is the routing
defect `[[RT-CONTEXT-FRAME-LABEL-CORRECTION]]` exists to fix, and this node
will not create a fifth instance of it. The labels state the readmission
condition without minting the pointer.

**The premise under that paragraph changed mid-turn and the labels were
rewritten again.** `[[RT-HOST-RESPONSE-OCCURRENCE-KEY]]` landed on main at
`4a6c091e3` while this section was being written, so "framed but not on main"
stopped being true. The labels now **name it**, as the owner of one arm of
8.2's fork, alongside the statement that the fork is unruled and that the other
arm has no node. Naming a `draft` node is not the
`[[RT-CONTEXT-FRAME-LABEL-CORRECTION]]` defect — that defect is naming an ID
with **no file**, and `RT-FRAME-MARKER-ONCE` is named by these same labels on
exactly that basis.

**The obligation that survives:** this node closes, so the four rows' owner
must move again when the fork is ruled. The labels say which node owns which
arm, so that move is a re-point with a stated destination rather than a
search.

### 8.5 `D3` and `AC-0` — the table re-measured, two numbers for one word

| program | overwrites | colliding constructors | agree on operation | deltas |
|---|---|---|---|---|
| `px7n-nested-computational-eliminator` | 29 | 29 | 29 of 29 | `[365]` |
| `rt_escape_escape_file_then_readat` | 29 | 29 | 29 of 29 | `[317]` |
| `rt_escape_nat_fanout_escaped` | 29 | 29 | 29 of 29 | `[317]` |
| `S6` (control) | 0 | 0 | 0 | `[]` |

**Every cell of the node's three-program table is unchanged at this base**, so
`D3` corrects nothing and records the re-measurement instead.

**`AC-0`'s control disagreed with the instrument, and the disagreement is the
finding, not an error.** The instrument reports **29**; a grep-derived line
count over the same probe output reports **58**. `host_response_routes` runs
**twice** per compile on all three colliding programs, the instrument's
counters are per invocation, and the grep spans both. Both numbers are correct
about different quantities, and the earlier "58 silent overwrites" figure is
the grep quantity. Stated here so one word does not carry two values.

**A first version of the instrument was discarded before it measured
anything.** It counted overwrites, agreement and deltas but not *distinct
colliding constructors* — the node table's actual column — which would have
left that column grep-derived, which is what `AC-0` forbids. The run was
stopped mid-build and re-cut; the cost was build time and no measurement.

### 8.5a The 29 keys, and the namespace question the Architect raised

**Asked during review and answered from the existing census output — a read,
no rebuild.** The concern: `host_response_routes` inserts under
`case.constructor`, a **match-arm** constructor (`responses.rs:1254`), and
`selected_host_response_route` looks up under the constructor of the
**operation value** being performed (`:1297`-`:1298`). Those are the same
symbol only when the scanned `Match` is matching on the host operation itself,
and nothing in the insert guards requires that.

Every one of the 29 keys, in each of the three programs:

    ClockOp      3    MonotonicNow, SleepUntil, WallNow
    ConsoleOp    4    Flush, IsTerminal, Read, Write
    EntropyOp    1    RandomBytes
    FSOp        21    10 named, 11 anonymous (ctor_541 .. ctor_551)
    ---------------
                29

**No data constructor appears as a key in any of the three plans**, and the
three key sets are **identical modulo the program-name prefix** — `diff` over
the stripped symbol lists is empty for `px7n` against each `rt_escape`
program.

**Why they coincide, from a measurement already on record rather than a new
one:** `EntropyOp::RandomBytes` is a key in all three plans, and the
de-duplication fold records that these programs perform `RandomBytes` **zero**
times and that neither test file mentions entropy. A key therefore exists for
an operation the program never performs ⇒ **the scanned `Match` is not program
code, it is the prelude's operation dispatcher, and that dispatcher matches on
the operation coproduct.** The divergence the concern requires is absent, for
a reason and not by coincidence.

**What is NOT measured, stated as the gap:** the per-route identity — for each
key, whether that route's own `operation` is the operation the key names — is
**not** in the census. The instrument printed the key and the agree-on-
`operation` comparison *between the two colliding routes*; it never printed
`operation` beside the key. Establishing it is one field and a rebuild, and it
was deliberately not queued behind `AC-3`.

**One of the 29 is settled already, from the predecessor:** `rt_escape:653`
selects `FSOp::ctor_543`, and that constructor is one of that plan's 29
counted collisions — so for that key the insert key and the lookup key are the
same symbol. One of twenty-nine, and an anonymous one.

⇒ The namespace conflation is **refuted at the level of which namespace the
keys inhabit** and **unmeasured at the level of per-key identity**. It is
narrowed, not open, and not closed.

### 8.6 `AC-2` — `S6` does not acquire a collision, and the control got sharper

`S6` reports `overwrites=0 colliding_constructors=0 distinct_deltas=[]
routes_final=29`. **It builds a route map of the same cardinality as the
colliding programs — 29 — with zero collisions in it**, so the control is not
"a smaller program"; it reaches the same map size by a plan that presents each
constructor once. It then refuses where it always did, on `source-specific
inheritances at one generated entry disagree on their typed consumer
projection, including the fresh-result route`.

The account in 8.2 predicts no collision in `S6`, and none is observed.

**One asymmetry, reported and deliberately not over-read:** the three colliding
programs emit two `host_response_routes` totals lines and `S6` emits one. `S6`
refuses on a planner invariant that may well abort before a second invocation,
so this is not read as a structural difference; it is unmeasured.

### 8.7 The correction this node owes, and its untested transfer

**The node's cross-program agreement rests on TWO programs, not three**, and
that correction is recorded in the node itself. Every proc of
`rt_escape_escape_file_then_readat` also appears in
`rt_escape_nat_fanout_escaped` — `after_file_escape`, `handle_outer` and `main`
byte-identical, `read_body` differing in four lines. Two agreeing censuses
across those rows are one shape seen twice.

**The transfer behind the correction is UNTESTED and is marked as such.** The
proc-level overlap was measured because the two programs produce the same
`ComputationalMatch` scrutinee refusal — a different refusal, reached by a
different probe, from the collision census the corrected sentence is about.
Whether the **collision** replicates independently across the two plans has
been measured by nobody. Restoring "three" needs that measurement; the overlap
does not license it, and its absence does not license the opposite either.

**The node's `title` is not corrected.** It says the colliding constructors
agree on their operation *across three programs*, and that is true as measured
— three programs were censused and all three agree. The correction is about
**independence**, which the title does not claim.

### 8.8 `AC-5` — `[[RT-FRAME-MARKER-ONCE]]` untouched

Not absorbed, not repaired, not edited; still `draft`. The two `px7n` rows
reaching it once the collision is suppressed is **the expected outcome of this
node succeeding**, not evidence of it failing. Nothing in this node's repair
path touches it, and nothing here should be read as bounding what lies behind
it.

### 8.8a `AC-3` — no regression, measured

    scripts/ken-cargo test -p ken-runtime --lib
    test result: ok. 1035 passed; 0 failed; 2 ignored; 0 measured

**1035 / 0 / 2, which is the predecessor's baseline exactly**, so section 2's
"re-measure at THIS base rather than carrying the number forward" comes back
with no difference to report. Workspace-green is CI's, per `COORDINATION`
section 12; this run is the targeted one the frame asks for.

**It took three attempts and the first two are not results.** Both earlier
runs returned `ken-cargo: timed out waiting for the build lock` after the full
1800-second wait and produced no measurement. A lock timeout is a failed
capture, not a clean run, and the two commits taken before this one say in
their own messages that nothing rested on it.

**Companion check, also green:** the two edited test targets compile and
enumerate — `--list` over `px7n_nested_computational_eliminator` and
`rt_escape_second_resource_native` exits 0 with 8 tests listed. That closes
the real risk in `D2`, which is that the rewritten labels are
`#[ignore = "..."]` string literals carrying an escaped backslash.

### 8.9 What this node does NOT deliver, and who owns the rest

Section 4's `D1` (repair on outcome 1) and `D1'` (retirement on outcome 2) are
both written against branches that 8.1 shows did not decide anything, so
neither is delivered as written. **No repair is delivered, and no repair unit
is selected here** — 8.2's fork is unruled, and picking the occurrence-keyed
route map would be sizing the expensive arm off a premise nobody has measured.

**Both arms have a shape and only one has a node.**
`[[RT-HOST-RESPONSE-OCCURRENCE-KEY]]` (on main, `draft`, `depends_on` this
node) carries the occurrence-keyed arm. The other arm — the shared chain
materialized once rather than per call site — has **no node and no owner**, and
its absence should not be read as evidence against it; it is what happens when
one arm was assumed.

Whichever arm is taken, 8.3 is a constraint on it: **the fail-closed check must
survive the change rather than be relaxed by it.**

**This node's outcome is therefore: the duplication is LOCATED, the check is
EXONERATED, `AC-4` is DISCHARGED, the fork is stated with its measurement
attached and routed rather than picked — and then RULED, on the consumer, in
8.9a — and no row is readmitted.** Section 1 says both outcomes close the
node; this is the second, reached with `AC-4` answered, which is the thing the
predecessor could not reach.

### 8.9a The ruling — neither arm, decided on the consumer

**The Architect's, recorded here because a ruling delivered in thread is not
a deliverable.** I did not reach it and am not restating it as mine.

    Dr   CITATION   evt_4eghtvj2fhpz0, architect, 2026-09-18T17:10:36Z
         ARM        NEITHER. Not Arm A, not Arm B. A third answer.

`Dr` and `AC-6` were restored to section 4 and section 5 at `831e521e5`, after
this section was written; the citation line above is what they ask for and is
added by position rather than from memory.

Both arms in 8.2 are about the **producer**: is the second entry legitimate.
The consumer is where it is settled, and nobody had quoted it:

    :1289  fn selected_host_response_route(plan, operation_origin, routes)
    :1298      if let Some(route) = routes.get(constructor).copied()
    :1301          "one Vis operation subtree selects more than one host
                    response producer"

**The consumer looks up by constructor alone**, and its uniqueness check is
*within one Vis subtree* — it guarantees a Vis selects at most one route, and
says nothing about whether it selected the **right** one.

Against what the two colliding entries are: `HostResponseRoute { operation,
effect_origin, producer_call_origin, response_origin }`. Measured in 8.5 and
the node's table — the copies **agree on `operation`** and **differ on the
three origins** by a constant delta. **Three of four fields differ, and they
are exactly the fields naming which producer and which continuation the
response goes to.**

⇒ **The two copies are not interchangeable**, the correct route for a Vis site
is the copy belonging to *that* call site, the key cannot distinguish them, and
the consumer cannot ask. So `responses.rs:1279` is not a tidiness assertion; it
is **the guard between here and a silent wrong-continuation route**, and 8.3
measured exactly what is behind it.

**Both arms rejected as framed:**

- **Arm A** — verdict right, repair wrong. The invariant is correct, but there
  is nothing to de-duplicate: per-arm inlining of a shared callee is a
  legitimate transformation. The invariant is a claim about the **source**
  dispatcher and `plan.source_occurrences` is **post-inlining**. The subject is
  wrong, not the claim.
- **Arm B** — diagnosis half right, direction dangerous. Re-keying the
  **producer alone** makes the map unambiguous per occurrence while the
  consumer still looks up by constructor, so the collision disappears and the
  mis-route does not. That converts a loud refusal into a silent miscompile.

**The third answer: producer and consumer move together, or neither moves.**
The route map is keyed on a coordinate that does not identify the Vis site;
any repair must give the consumer the same key the producer inserted under and
make it select with it. `operation_origin` is a `StaticOriginId`, so the
occurrence is derivable in principle, and threading it is the scope.

**The acceptance bar is REPLACED, not extended.** The word matters: the
two-clause bar is published on `main` right now, in
`[[RT-HOST-RESPONSE-OCCURRENCE-KEY]]` at `:102-113`, under the heading "THE
ACCEPTANCE BAR, SET BY THE ARCHITECT" — **and it is insufficient on its own**,
because occurrence-keying satisfies both of its clauses and still mis-routes.
Anyone reading that node today sees a bar that looks complete and is not.
Three clauses replace two:

    1. MUST STILL REFUSE  two response-handling sites within one occurrence
                          claiming one operation constructor.
    2. MUST NOT REFUSE    N occurrences that are inlined copies of ONE source
                          dispatcher arm.
    3. MUST ROUTE EACH VIS SITE TO ITS OWN COPY -- the consumer selects by the
       same key the producer inserted under, and a Vis site whose copy is
       absent REFUSES rather than falling back to another copy.

Clause 3's control is **not a green test**: revert clause 3 alone, keep 1 and
2, and exhibit a Vis site routing to the other copy's `producer_call_origin`.
If that control cannot be built, that is itself a finding about whether the
repair is observable, and it is worth more than a green run.

**What this changes in the sections above: nothing measured.** 8.1 through 8.6
stand as taken. What it replaces is 8.9's "no repair unit is selected here" —
one is now selected, it is neither arm, and it is larger than either.

### 8.9b The restored obligations, enumerated BY POSITION

`831e521e5` restored `Dr`, `D4`, `AC-6` and the `AC-7`..`AC-10` controls after
this candidate was first handed off. **Enumerated from section 4 and section 5
in order rather than from recall**, which is the discipline that recovered
them in the first place.

**`Dr` — DISCHARGED.** `evt_4eghtvj2fhpz0`, arm NEITHER. See 8.9a.

**`D1` — DOES NOT FIRE.** It is written for outcome `(1)`; `D0` returned the
third answer (8.1).

**`D1'` — DOES NOT FIRE, and this is the gate working.** It is gated behind
`Dr` and fires only on Arm A. The ruling returned neither arm, so **this node
closes on `Dr` + `D0` without touching the check**, which is exactly what
section 4 prescribes. No relaxation was performed on implementer judgment.

**`D4` — VACUOUS, and said so rather than skipped.** It asks for a registry
row in `.github/ignored-test-exemptions.toml` **for any row readmitted with an
accepted red**. `AC-1` readmits none, so there is no row to register.
**Control:** that file is not in this candidate's diff at all
(`git diff --name-only origin/main HEAD` lists five files and it is not one).

**`AC-6` — SATISFIED, and its mechanical control run.** Section 5 says to
check the diff for three shapes: an edit to `host_response_routes`' key, an
edit to its loop shape, or a retirement or relocation of the uniqueness
assertion. Measured:

    responses.rs in the diff                     NO (zero hits by name)
    non-#[ignore] lines anywhere under crates/   NONE
    crates/ footprint                            4 #[ignore] attribute lines

**`AC-7` — VACUOUS, and it is a vacuity worth naming rather than a pass.** It
asks to revert the repair and confirm the readmitted rows go red again. **No
repair landed and no row was readmitted**, so there is nothing to revert and
nothing whose redness could discriminate. Recording this as satisfied would be
a control defined by its own absent subject.

**`AC-8` — SATISFIED in its strongest available form.** It asks that the
invariant still refuses something. **It refuses in production, today, on all
four rows**: the unforced baseline in 8.5 stops every one of them at
`responses.rs:1279`. The concern behind the AC — a guard that now fires on
nothing — cannot arise here, because nothing was removed.

**`AC-9` — the crate set, DERIVED.** Touched set is two files, both
`crates/ken-cli/tests/`. `ken-cli` is a leaf here — `ken-runtime` depends on
nothing in it — so the reverse-dependency closure is the two test targets
themselves, and nothing else needs testing on account of this change.

**Target selection, stated beside the claim as the AC requires:** the check
run was `ken-cargo test -p ken-cli --test px7n_nested_computational_eliminator
--test rt_escape_second_resource_native -- --list`, `exit=0`, **8 tests
listed**. That builds and executes each test binary in listing mode. `cargo
check` would **not** have been evidence, because it does not compile
`#[cfg(test)]` — and the entire risk in `D2` is that the rewritten labels are
string literals inside a test file. `AC-3`'s `-p ken-runtime --lib` run
(8.8a) is beyond this closure and was run anyway.

**`AC-10` — the second blockers, measured, and NOT pulled in.**

    px7n:149, px7n:170   reach RT-FRAME-MARKER-ONCE's frame-marker message,
                         OrientedSubcontinuationPlanV1: checked Runtime frame
                         marker was consumed more than once. CONFIRMED.
    esc:653, esc:713     reach ComputationalMatch: tree-producing match
                         scrutinee is not Bool or a constructor.

**On `[[RT-CLOSURE-BOUNDARY-LANE]]`: UNDETERMINED, and that is not "ruled
out".** Something has now been seen past the collision on the `rt_escape`
pair, and what it reached is a **different** mechanism; nothing has seen past
**that**. The durable-lane claim is neither confirmed nor refuted, and the
labels say so in those words. Neither blocker is absorbed here.

### 8.10 Attribution

- The **key-adequacy** statement — that there is no key too narrow to
  distinguish cases that never differ — is the Architect's, from the
  predecessor's review.
- The distinction between **a refuted repair and an adequate key** — that
  `RT-HOST-RESPONSE-ROUTE-KEY-COLLISION` refuting *its* key change does not
  establish that the constructor key is adequate — is mine.
- The **plan-versus-instrument split** in 8.1 and 8.2 — that a correct plan and
  a faithful check can both be right while their meeting point is wrong — is
  mine, and it is the sentence the rest of section 8 rests on.
- The four labels' **attribution halves** are the predecessor ring's, carried
  and re-measured, not rewritten.
