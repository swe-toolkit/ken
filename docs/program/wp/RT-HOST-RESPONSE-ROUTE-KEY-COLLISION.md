# RT-HOST-RESPONSE-ROUTE-KEY-COLLISION — work package

**Owner: Team Runtime. Size S. Tier T1. Gate: none.**
**Implementation base: `origin/main` at `fe649049f2419be5e8c9d58547baeeb27cb06ddd`.**

Operator priority, 2026-09-15: *"The other tests should be fixed."* Second
repair node cut from the `RT-IGNORED-FAILING-ROWS-INVENTORY` ledger.

## 1. Objective

Decide whether the `two host response cases claim one operation constructor`
invariant is **too strong**, and disposition the four ignored rows that trip
it — readmitted if the invariant is wrong, retired with evidence if the
invariant is right and the tests assert something false.

**Both outcomes close this node.** An invariant that correctly refuses four
bad tests is as good a result as a key that needed widening, and it is
reported the same way.

## 2. Fixed inputs, measured

**The invariant:**

    crates/ken-runtime/src/cranelift_backend/planning/
      static_transition/responses.rs:1281

in a `BTreeMap<RuntimeSymbol, HostResponseRoute>` keyed on `case.constructor`
alone, whose value carries `operation` as a field.

**The four rows, by FILE and LINE** (at `c041de7c3`) — see the issue node's
warning about the name collision with `px7o_heterogeneous_eliminator_frames.rs`:

    rt_escape_second_resource_native.rs:654   escaped_resource_used_by_fanning_host_op_...
    rt_escape_second_resource_native.rs:714   nat_fanout_escaped_resource_matches_interpreter
    px7n_nested_computational_eliminator.rs:150  nested_ok_payload_reaches_both_real_executors
    px7n_nested_computational_eliminator.rs:171  nested_err_payload_reaches_both_real_executors

**Clean baseline, measured on `main` at `c041de7c3`:**

    ken-cargo test -p ken-runtime --lib    1035 passed   0 FAILED   2 ignored

**You are changing planner code those 1035 tests exercise.** The baseline is
clean, so a red appearing during this repair is yours and cannot be
pre-existing debt.

## 3. THE MEASUREMENT THAT DECIDES IT — RUN THIS FIRST

> ### RUN AND ANSWERED, 2026-09-17, AND BOTH BRANCHES BELOW WERE WRONG.
>
> **`SAME` on all four rows** — every colliding pair carries
> `EntropyOp::RandomBytes` / `EntropyRandomBytes` (runtime-implementer,
> `evt_73vfttwb7ncya`, rows re-derived at `89dc3b0e5`). **Read the amendment in
> §3a before acting on anything in this section.** The two branches below are
> retained because the measurement was taken against them, not because either
> is the disposition.

**For each of the four rows: do the two colliding cases carry the SAME
operation or DIFFERENT operations?**

    DIFFERENT   the key is too narrow. It omits what the value already
                carries, and the invariant is reporting a duplicate that is
                not one. Repair is the key.

    SAME        the invariant is RIGHT. Two cases genuinely claim one
                constructor for one operation, the tests are asserting
                something false, and the repair is to the tests -- or they
                retire.

**Do not write the repair before this comes back.** The key-too-narrow reading
is the Steward's hypothesis from reading the code, **not a measurement**, and
it is exactly the kind of reading that is refuted by one run. If it comes back
`SAME`, discard it.

**Report the answer per row, not in aggregate.** Four rows can split across the
two outcomes, and a summary would hide that.

## 3a. AMENDMENT — THE TWO BRANCHES ARE NOT EXHAUSTIVE. THE KEY OMITS THE OCCURRENCE.

**Steward, 2026-09-17. The defect above is mine and it is the second instance
in this ledger-derived series**, the first being
`RT-CARRIED-RESIDUAL-IH-ARITY`'s binary disposition. **§3 is exhaustive over
*operation equality* and not over *why two entries collide*, and the answer sits
in the gap.**

**Measured, per row** — same constructor, same operation, **different
occurrences**:

    PREV   effect=23    producer_call=20    response=18     (px7n: 25 / 22 / 20)
    NEW    effect=340   producer_call=337   response=335    (px7n: 390 / 387 / 385)

⇒ **`SAME` does not imply a genuine duplicate claim, and the four tests are not
asserting anything false — so the `SAME` branch's disposition does not follow.**

> **SUPERSEDED REASON — corrected in §9.2. Read that before using this
> paragraph.** I wrote that this holds because *"a program that performs
> `RandomBytes` at two sites with two response handlers is valid Ken."*
> **Measured: these programs perform `RandomBytes` zero times.** `EntropyOp` is
> a prelude declaration and neither test file mentions entropy. **The conclusion
> stands; the reason does not.** It is not two legitimate program sites — it is
> one prelude block materialized twice, which is also why §9.3's constant offset
> exists. **Do not carry the two-sites framing forward.**

⇒ **And the `DIFFERENT` branch's repair is equally dead: adding `operation` to
the key changes nothing, because both entries already carry
`EntropyRandomBytes`.**

**What the key actually omits is the OCCURRENCE.** `host_response_routes` maps
`case.constructor` over *every* `Match` in the plan, so the invariant encodes
**"one host-operation constructor ⇒ one response-handling site in the whole
program"**, and `selected_host_response_route` then resolves a `Vis` site's
operation subtree against that single entry.

**The invariant is too strong. The repair is NOT a tuple widening.** Pairing N
`Vis` sites to N handlers is planner work. **`AC-4` still governs it** — justify
against purpose, name a case that must still be refused — and `AC-4` is
unchanged by this amendment.

**CLOSED BY §9.2 AND §9.3. It was genuinely open when this amendment was
written and it is not open now.** I flagged that the `PREV` 18-25 / `NEW`
335-390 spread meant the two entries **may not be peers** — one possibly a
library or prelude occurrence rather than a second program site. **Measured, and
the answer is yes: the occurrence is the prelude's, duplicated.** **Do not
re-measure this from this paragraph.**

**Nothing in this amendment directs the disposition.** It retires two refuted
branches so they stop reading as the decision procedure.

## 4. Deliverables

1. The §3 measurement, per row, with the operations named.
2. The repair at whichever unit §3 selects — the key, or the tests.
3. **The four rows dispositioned by name**: readmitted-and-passing, or retired
   with the evidence that shows the assertion false.
4. If a row is readmitted with an accepted red, it needs a row in
   `.github/ignored-test-exemptions.toml` (`class` + `readmission` +
   `test_path`).
5. **The two stale `#[ignore]` labels are corrected or removed.** They name
   mechanisms the run does not exhibit (`RT-CLOSURE-BOUNDARY-LANE`'s durable
   lane, `RT-FRAME-MARKER-ONCE`'s frame marker) and cite base `21fd46dc`.
   **Leaving a wrong label on a readmitted row re-seeds the defect this whole
   program exists to clear.**

## 5. Acceptance criteria

**AC-1 — four rows, four named dispositions**, resolved by file:line rather
than by symbol. A count is not an answer.

**AC-2 — no regression on the `ken-runtime` lib suite** against the
`1035 / 0 / 2` baseline in §2. **Report the delta's three buckets, not the
total:** red-on-both (pre-existing), green-on-main-red-here (yours), and
candidate-only with no `main` counterpart (which no subtraction shows —
publish it even at zero). Re-measure the baseline if `main` moves under you.

**AC-3 — the crate set is DERIVED, not named.** Compute the
reverse-dependency closure over the touched set **to a fixpoint**,
mechanically, and test that set. **Never `--workspace`** (`COORDINATION §12`).
**State the target selection beside the claim** — `cargo check` does not
compile `#[cfg(test)]`, so a green `check` is not evidence that any test built.

**AC-4 — if the key changes, the widened key is justified against the
invariant's PURPOSE, not against these four rows.** The invariant exists to
catch a real ambiguity. **Say what it still catches after the change**, and
give one case that must still be refused. A widened key that refuses nothing
has deleted the check rather than corrected it.

**AC-5 — `S6` and `S8` are checked against the outcome, and NOT pulled in.**
Both are single rows carrying different planner-invariant messages from this
same subsystem. **Say whether your cause explains them.** If it does, that is a
finding for the Steward to re-cut on — **do not absorb them into this node**;
if it does not, say that too.

## 6. What this node is NOT

- **Not the other eleven ignored rows.**
- **Not a rewrite of the static-transition planner.** The scope is one
  invariant and the four rows that trip it.
- **Not an endorsement of the key-too-narrow reading.** §3 decides it.

## 7. Contention

`crates/ken-runtime/src/cranelift_backend/planning/` — `ABI-S6-HS18-D5B-
SUBSTRATE-PORT` is **parked** (dead candidate, recut is the Steward's,
unstarted). `RT-CARRIED-RESIDUAL-IH-ARITY` touches
`cranelift_backend/lowering/`, a **different** subtree. **No live contention at
cut time.** If both this and `RT-CARRIED-RESIDUAL-IH-ARITY` run concurrently,
they are separate branches and separate PRs.

## 8. Estimated tier: T1

**Deciding whether an invariant is too strong is a soundness-adjacent call**,
not a transcription. The wrong answer either deletes a real check or retires
four tests that were right.

## 9. MEASURED OUTCOME — section 3, answered, and both of its branches refuted

**Derivation output appended by the implementer. It adds no acceptance
criterion and amends none.** Rows re-derived at `89dc3b0e5`; all four sit at
the frame's exact file:line and all four reproduce `S2`.

### 9.1 The section 3 measurement, per row

    row                                          ctor                     operation           SAME?
    rt_escape:654  escaped_resource_used_by_...  EntropyOp::RandomBytes   EntropyRandomBytes  yes
    rt_escape:714  nat_fanout_escaped_resour...  EntropyOp::RandomBytes   EntropyRandomBytes  yes
    px7n:150       nested_ok_payload_...         EntropyOp::RandomBytes   EntropyRandomBytes  yes
    px7n:171       nested_err_payload_...        EntropyOp::RandomBytes   EntropyRandomBytes  yes

`SAME` on all four. **The key-too-narrow reading is dead in the form it was
stated** — both entries carry `EntropyRandomBytes`, so adding `operation` to
the key changes nothing.

**But the `SAME` branch does not follow either.** It infers "the invariant is
right, the tests assert something false". The colliding entries agree on the
operation and differ in every other field:

    PREV  effect=23   producer_call=20   response=18
    NEW   effect=340  producer_call=337  response=335

### 9.2 The constructor is not in the test programs at all

    grep -c 'Entropy\|RandomBytes'  px7n_nested_computational_eliminator.rs   0
    grep -c 'Entropy\|RandomBytes'  rt_escape_second_resource_native.rs       0

`EntropyOp = RandomBytes Int` is a **prelude declaration**
(`crates/ken-elaborator/src/prelude.rs:594`), with `entropy_resp` a prelude
match over it. **Four tests that never mention entropy cannot be asserting
something false about entropy.**

**RECONCILING WITH §3a, WHICH THIS REFINES.** §3a (Steward's amendment, on
`main`) reads *"a program that performs `RandomBytes` at two sites with two
response handlers is valid Ken"*. **That was my own earlier framing, from the
first probe run, and it is superseded by the grep above**: these programs
perform `RandomBytes` **zero** times. The conclusion §3a draws is untouched —
`SAME` still does not imply a genuine duplicate claim, and the tests still
assert nothing false — but the *reason* is stronger than the one stated there.
It is not two legitimate sites in one program; it is **one prelude block
materialized twice**, which is why §9.3's constant offset exists at all. Flagged
here rather than edited into §3a: that section is the Steward's and amending it
is a frame amendment, not a ring's fold.

### 9.3 THE CAUSE — a duplicated response block, measured per program

**CORRECTION, folded before merge.** An earlier draft of this section reported
"20" constructors and called them "all" of them. **That 20 was a `head -20` in
the probe pipeline — a property of the pipe, not of the population.** The tell
was in the output itself: the script also greps for `test result:` and no such
line appeared, because the truncation ate it. The Architect found the seam by
noticing the rt_escape label cites `FSOp::ctor_543` while the census listed
only up to `ctor_542`. **The numbers below are re-measured with the census run
to completion, and the probe emits its own total so the count is not one I
derive by grepping.**

Construction used to `return Err` on the first duplicate, which hid the
population. With the refusal deferred so the scan completes:

    row        program                                collisions  same_op  deltas
    esc:654    rt_escape_escape_file_then_readat          29      29 true   {317}
    esc:714    rt_escape_nat_fanout_escaped               29      29 true   {317}
    px7n:150   px7n-nested-computational-eliminator       29      29 true   {365}
    px7n:171   px7n-nested-computational-eliminator       (same plan as :150)

**THE FOUR ROWS SPAN THREE PROGRAMS, AND THE UNIT IS THE ROW, NOT THE FILE.**
`rt_escape_second_resource_native.rs` holds two `const` programs 250 lines
apart -- `ESCAPE_FILE_THEN_READAT` at `:217` and `NAT_FANOUT_ESCAPED_RESOURCE`
at `:468` -- reached by `differential(...)` at `:659` and `:721`. `px7n` is the
opposite: one `const PROGRAM`, both rows through `assert_case`.

Every total is the probe's own counter, cross-checked against an independent
line count, with `test result:` present in all three runs so none is truncated.

**A SECOND CORRECTION, THE SAME CLASS AS THE FIRST, FOUND BY THE ARCHITECT.**
An earlier fold gave each FILE its own census -- and the file is not the unit.
The `:714` label then asserted `rt_escape_escape_file_then_readat`'s identity,
count, constant and selected constructor on a program nobody had run. **The
values turn out to agree (29, 29/29, `{317}`), but they were TRANSFERRED
rather than measured** -- and this section's own argument is why that is not
acceptable: the offset is the size of the first copy, so it is a per-program
quantity by construction. They are now measured on
`rt_escape_nat_fanout_escaped` directly.

**The load-bearing fact is the SHAPE, not the count.** Within each program the
effect-origin deltas collapse to a **single distinct value**. A constant offset
across an entire set of colliding constructors is a **duplicated block**; 29
independent programs-each-claiming-an-operation-twice would produce 29
unrelated offsets. The constant is uniform WITHIN each program, which is what
a duplicated block predicts, since the offset is the size of the first copy.

**It is NOT simply "different per program", and the third measurement is what
corrected that.** Two of the three programs share `317`, and they are the two
declaring `capabilities FS AFull`; the one with `365` declares
`capabilities FS APartial`. **Consistent with the offset being the size of the
copied block** — same declared capability surface, same block, same offset.
That mechanism is an inference from three points, not a measurement: the
block size was never measured against the capability set, and a fourth program
could refute it. **What is measured is one constant per plan; the reason two
plans share one is not.**

⇒ The invariant encodes "one host-operation constructor ⇒ one response site
program-wide"; what breaks it is the plan carrying the response-handler block
twice.

**What these numbers do NOT say.** 29 is the count of **collisions observed in
that program's plan**. It is not a claim about how many host-operation
constructors exist, nor that every such constructor collides — neither was
measured, and the earlier draft asserted both.

### 9.4 Which makes BOTH branches of section 3 wrong, for one reason

`DIFFERENT ⇒ repair the key` is inert here (the operations agree).
`SAME ⇒ the tests are false` mistakes a planner duplication artifact for 29
competing claims. **The key omits neither the operation nor anything the value
carries — it omits the OCCURRENCE**, and the occurrences are duplicates of one
another.

### 9.5 A repair was BUILT and MEASURED, and is deliberately NOT landed

Moving the refusal from construction to use — refuse only when a `Vis` actually
selects an ambiguous constructor — was implemented and run. **It splits the
population 2/2 and closes neither half:**

- **px7n (2 rows): the collision clears, and the row's ORIGINAL label appears
  underneath.** They then fail with `OrientedSubcontinuationPlanV1: checked
  Runtime frame marker was consumed more than once` — verbatim the mechanism
  `RT-FRAME-MARKER-ONCE` names. **Deliverable 5's premise is falsified for these
  two: the label was not stale, it was SHADOWED by a newer refusal stacked in
  front of it.**
- **rt_escape (2 rows): unchanged, and each measured on its OWN program.**
  Under deferral both still refuse, and the selected constructor is
  `FSOp::ctor_543` **qualified by each row's own program** --
  `rt_escape_escape_file_then_readat::FSOp::ctor_543` for `:654` and
  `rt_escape_nat_fanout_escaped::FSOp::ctor_543` for `:714`. In both cases the
  selected constructor is itself one of that plan's 29 counted collisions,
  which is the step that makes the per-plan census bear on the row's own
  refusal rather than merely sitting beside it.

Not landed because it half-disables a fail-closed gate while closing nothing,
and `AC-4` asks the widened check be justified against PURPOSE. On this evidence
the right repair addresses the duplication, not the key — so relaxing the key is
the wrong unit, the same conclusion `RT-CARRIED-RESIDUAL-IH-ARITY` reached about
its own refusal.

### 9.6 AC-1 — four rows, four named dispositions

**Coordinate convention, corrected by `[[RT-DUPLICATED-RESPONSE-BLOCK]]` D2.**
The four numbers below were the `fn` lines. Both successor frames anchor a row
on its `#[ignore]` **attribute** line, one above —
`[[RT-DUPLICATED-RESPONSE-BLOCK]]` section 2 lists `:149 :170 :653 :713`, and
`[[RT-CONTEXT-FRAME-LABEL-CORRECTION]]` states the convention in its own text.
They are restated here on that convention so one row is not two coordinates.
(`docs/program/evidence/rt-ignored-failing-rows-ledger.md` keys its rows by
test name and carries no line number, so it is unaffected either way.)

**All four STAY IGNORED.** No row is readmitted and none is retired: the tests
assert native/interpreter agreement on valid Ken programs, and 9.2 shows they do
not even mention the construct that collides.

    rt_escape:653  STAYS IGNORED. Program rt_escape_escape_file_then_readat.
                   Refuses at the collision; selects
                   rt_escape_escape_file_then_readat::FSOp::ctor_543, itself
                   one of that plan's 29 collisions. Whether
                   RT-CLOSURE-BOUNDARY-LANE is also a real blocker underneath
                   is UNDETERMINED -- nothing has seen past the collision.
    rt_escape:713  STAYS IGNORED. A DIFFERENT program,
                   rt_escape_nat_fanout_escaped, measured separately. Same
                   shape and the same UNDETERMINED, and the selected
                   constructor is ctor_543 of ITS OWN program -- the two rows
                   agree in shape, not by sharing a measurement.
    px7n:149       STAYS IGNORED. Collision is a false alarm here; the real
                   blocker underneath is the labelled frame-marker mechanism,
                   confirmed by running with the collision deferred.
    px7n:170       STAYS IGNORED. Same.

### 9.7 Deliverable 5 — the labels, corrected and CLASSIFIED

All four rewritten to name the measured refusal, the duplication cause, the
readmission condition, and — for the two where it applies — that the previously
named mechanism is shadowed rather than wrong. The stale `21fd46dc` cite is
dropped **from the labels**. Surviving `21fd46dc` mentions in the surrounding
comment blocks are **intentionally historical** ("measured failing at the frozen
base 21fd46dc by the D10/D12 differential") and are correct precisely because
they name the old base; one further hit is on `RT-PROCESS-EXIT-STATUS`, a row
outside this node's population. Left in place, classified rather than swept.

### 9.8 AC-5 — S6 and S8 are NOT explained by this cause, measured

Both were run against the instrumented build. **Neither emits a collision at
all.**

    S6  escaped_buffer_used_by_fanning_host_op_matches_interpreter
        refuses at: source-specific inheritances at one generated entry
        disagree on their typed consumer projection, including the
        fresh-result route
    S8  sp_a_foreign_span_freeze_rejects_own_span_succeeds_on_both_engines
        refuses at: an exact detached required consumer has no computational
        occurrence

**Do not pull either in.** `S6` is the sharper control: it lives in the same
file as two of this node's rows and gets PAST response-route construction before
failing elsewhere, so co-location is not co-causation.

### 9.9 AC-2 and AC-3

**AC-2** — the candidate changes no `crates/ken-runtime` source; the built
repair of 9.5 was reverted before commit and `git diff` over `crates/ken-runtime`
is empty. Baseline re-measured at this node's base rather than carried:
`ken-cargo test -p ken-runtime --lib` = **1035 passed / 0 failed / 2 ignored**,
identical to the frame's. Buckets **0 / 0 / 0**, the third published at zero as
instructed.

**AC-3** — closure DERIVED, `cargo tree -i -e normal,dev`, no `--depth`,
iterated to a fixpoint, intersected with workspace members:
`ken-runtime  ken-cli  ken-elaborator  ken-interp  ken-verify` (5), stable under
both seeds. Target selection `--lib`, a `test` not a `check`.

**AC-4** — not reached: no key change is landed, so there is no widened key to
justify.

