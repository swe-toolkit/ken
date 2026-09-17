# RT-CARRIED-RESIDUAL-IH-ARITY — work package

**Owner: Team Runtime. Size S. Tier T1. Gate: none.**
**Implementation base: `origin/main` at `fe649049f2419be5e8c9d58547baeeb27cb06ddd`.**

Operator priority, 2026-09-15: *"The other tests should be fixed."* This is the
first repair node cut from the `RT-IGNORED-FAILING-ROWS-INVENTORY` ledger.

## 1. Objective

Close the `BoundaryCarrier` refusal *"a carried recursive hypothesis is an
eliminated value, not a callable, so it takes no arguments, but the call
provides N"* for the four ignored rows that carry it, and **readmit those four
rows** — or, if the refusal is correct and the rows are wrong, say so with the
same evidence and retire them instead.

**Both outcomes close this node.** See §4.

## 2. Fixed inputs, measured

**The four rows, at `c041de7c3` (the ledger's base):**

    px7l_checked_host_recursive_bind.rs:153  delayed_capturing_generic_bind_...
    px7l_checked_host_recursive_bind.rs:220  runtime_selected_non_unit_response_...
    px7m_hostresult_computational_match.rs:153  dynamic_ok_payload_selects_...
    px7m_hostresult_computational_match.rs:185  dynamic_err_payload_selects_...

All four under `crates/ken-cli/tests/`. All four `#[ignore]` labels are
**byte-identical** and name this mechanism.

**The refusal, at `fe649049f`:**

    core.rs:3052   fn reject_carried_residual_arguments(arguments: usize)

    called from    core.rs:3101   core.rs:6063   core.rs:16133
                   source.rs:5016

**The pointer left by the previous seat:** `aggregates.rs:3340-3356`. It rules
this refusal out of scope for the carried SITE-OPERAND projector and says
*"widening this dispatch cannot close it and must not try."* **Read it first.**

**Cold-lowering witness:** `rt_allocate_stage`,
`rt_cold_lowering_path_enumeration.rs:156`, dispositioned `Completes` at `:547`.

**Clean baseline, measured on `main` at `c041de7c3`:**

    ken-cargo test -p ken-runtime --lib    1035 passed   0 FAILED   2 ignored

**You are changing lowering code those 1035 tests exercise.** The baseline is
clean, so any red appearing during this repair is yours and cannot be
pre-existing debt. That is the whole reason the number is in the ledger.

## 3. The design call — make it FIRST and write it down

**Is a carried recursive hypothesis non-callable at all five call sites, or
only at the site these four rows reach?**

    fix at the definition   changes all five paths
    fix at one call site    changes one

**Answer this before writing the repair, and record the answer with its
evidence in the handover.** A fix at the shared function is a claim about
fan-in; **the claim has to be checked, not inherited from where the helper
happens to live.** If you cannot determine it for all five, fix the site you
measured and say which sites you did not clear — a scoped fix with a named
residue beats a choke-point fix with an unchecked one.

**This is the reasoning content of the node.** The edit that follows is small.

## 4. Deliverables

1. The design call of §3, written down with its evidence.
2. The repair, at whichever unit §3 selects.
3. **The four rows readmitted** — `#[ignore]` removed, passing — **or** retired
   with the measurement that shows the refusal is correct and the rows are
   asserting something false. State which, per row.
4. If any row is readmitted with an accepted red, it needs a row in
   `.github/ignored-test-exemptions.toml` (`class` + `readmission` +
   `test_path`) — **an accepted red names what would readmit it.**

## 5. Acceptance criteria

**AC-1 — the four rows are dispositioned, per row, by name.** Each is
readmitted-and-passing or retired-with-evidence. **Four outcomes, four names.
A count is not an answer here** — the rows sit in two files and a per-file
summary hides a row.

**AC-2 — the `ken-runtime` lib suite does not regress.**

    ken-cargo test -p ken-runtime --lib

against the `1035 passed / 0 FAILED / 2 ignored` baseline in §2. **Report the
delta and its three buckets, not the total:** red-on-both (pre-existing),
green-on-main-red-here (yours), and **candidate-only with no `main`
counterpart** (which no subtraction can show — publish it even at zero).

**AC-2's control:** the baseline is `main`'s, not a number you carry forward.
Re-measure it if `main` moves under you.

**AC-3 — the crate set is DERIVED, not named.** This node touches
`ken-runtime` and `ken-cli`. Compute the reverse-dependency closure over the
touched set **to a fixpoint**, mechanically, and test that set. **Never
`--workspace`** (`COORDINATION §12`) — a derived list written down is still a
targeted build. **State the target selection beside the claim**
(`--lib`/`--tests`/`--all-targets`): `cargo check` does not compile
`#[cfg(test)]`, and a green `check` is not evidence that any test built.

> **AC-3 exists because this exact criterion was got wrong three times on
> `ABI-S6-HS18-D5B-SUBSTRATE-PORT` this week** — a hand-named single crate,
> then a hand-named four, then a hand-chosen one-hop closure. **The population
> was a default nobody selected each time.** Do not enumerate it.

**AC-4 — the `aggregates.rs:3340-3356` scope comment is reconciled.** It
asserts this refusal *"does not become reachable or unreachable by anything
this projector does."* If your repair makes that false, **update it**; if it
stays true, say so. **A comment that names the next layer is load-bearing and
goes stale silently.**

**AC-5 — `rt_allocate_stage`'s enumeration disposition is checked.** It is the
recorded witness. If it moves, that is a result; if it does not, that is also a
result. **Name which.**

## 6. What this node is NOT

- **Not the other eleven ignored rows.** `S2` (4 rows, planner-invariant) is a
  separate cut; `S3` (2 rows, `UnclassifiedRuntimeTrap`) is not routable yet
  because its signature names no mechanism; five singletons are queued.
- **Not an investigation into why these rows still fail.** The mechanism is
  named in the label, in the code, and in §2.
- **Not `RT-SITEOP-CARRIED-WITNESS` reopened.** That node landed its
  deliverable and these labels record it succeeding.

## 7. Contention

`crates/ken-runtime/src/cranelift_backend/lowering/` is touched by
`ABI-S6-HS18-D5B-SUBSTRATE-PORT`, which is **parked** — its candidate is dead
and its recut is the Steward's, unstarted. **No live contention at cut time.**
If D5B's recut is released while this is in flight, it comes back to the
Steward to sequence; do not resolve it in the ring.

## 8. Estimated tier: T1

Not for the size of the edit. **The design call in §3 decides a property of
five code paths from evidence about one**, and the wrong answer changes four
paths nobody measured. That is reasoning, not transcription.

## 9. MEASURED OUTCOME — the design call of section 3, and its evidence

**This section is derivation output, appended by the implementer. It adds no
acceptance criterion and amends none.** Everything above it is the frame as
routed.

### 9.1 The fan-in is FOUR call sites, not five

    definition   core.rs:3052   reject_carried_residual_arguments
    calls        core.rs:3101   core.rs:6063   core.rs:16133
                 source.rs:5016

Section 3 and the kickoff both say "five". The frame's own listing is right and
counts the definition line as the fifth entry; there are four calls. The
function's doc comment agrees -- it says "four residual consumers" and explains
why it is one shared associated fn rather than four inline refusals.

### 9.2 The answer: the repair belongs at NEITHER unit section 3 offers

All four call sites have the same shape, verified by reading each:

    if let LoweringOperand::Carried(word) = base {
        if let Some(body) = recursive_unit_body {
            // lower the args, call the declared recursive-position unit
        }
        Self::reject_carried_residual_arguments(args.len())?;
        // resume the carried word on the zero-argument route
    }

**The refusal is the else-branch of `recursive_unit_body == None` at every one
of the four.** Arguments are ALREADY supported at all four sites whenever a
declared recursive-position unit body exists. So:

- **A fix at the definition is wrong at all four.** It would admit arguments in
  exactly the state where there is no callee to pass them to.
- **A fix at one call site is wrong too.** It changes nothing about why the
  body is missing.

The property "a carried recursive hypothesis is non-callable" is therefore
uniform across the fan-in, and the claim is checked rather than inherited: it
rests only on the `if let Some(body)` guard that precedes each of the four
calls. **The choke point that matters is elsewhere.** `recursive_unit_body` is
DECIDED in exactly one production place -- `core.rs:14413`, the sole non-test
caller of `recursive_position_unit_body`; every other mention propagates the
value.

### 9.3 Which site fires, measured

Instrumented at each of the four calls and run against all four rows. **Only
`source.rs:5016`, the source-machine route, is reached, with `args=1`.**
`core.rs:3101`, `core.rs:6063` and `core.rs:16133` are never entered by any of
these rows. The other three are cleared by the structural argument in 9.2, not
by measurement, and this paragraph is the named residue.

### 9.4 The label does not name the mechanism

`resolve_recursive_unit_body` returns `None` for all four rows, and in every
case **the recursive-position argument IS a `LexicalClosure` with a callable
body.** The callable route is declined by
`recursive_position_captures_all_planner_recoverable`, and control then falls
to the zero-argument route, which reports the arity.

⇒ **The arity message is a fallback symptom reported one layer below the
decision.** Section 6 rules this node "not an investigation into why these rows
still fail" on the ground that "the mechanism is named in the label". The label
names the REFUSAL. The mechanism is a different function in a different file,
and the two are not the same claim.

### 9.5 The four rows are TWO causes, not one cluster

Measured per row at `3f4ae2d83`:

    px7m err   captures=5  claims=4   VERDICT cardinality
    px7m ok    captures=4  claims=3   VERDICT cardinality
    px7l 153   captures=3  claims=3   VERDICT claim-availability, index 0
    px7l 220   captures=3  claims=3   VERDICT claim-availability, index 0

The ledger clustered `S1` on a byte-identical signature. **The signature is
shared, and these two verdicts are not the same verdict.** ⚠ Read this table
with 9.7a, which measured one level further and found both verdicts to be
shadows of a SINGLE cause -- the two-cause reading here is the intermediate
result, not the finding. Kept because it is what the gate reports, and because
anyone re-running the predicate will see these two verdicts and needs to know
they were seen and superseded.

A control worth keeping: in the px7l programs a DIFFERENT eliminator in the
same program resolves `Some` (origins 352 and 371 return a body). So the
predicate is not uniformly false for these programs, and the failing origins
are 339 and 358 specifically.

### 9.6 Both verdicts are deliberate fail-closed refusals

Neither is a defect in the gate as written.
`recursive_position_captures_all_planner_recoverable` documents both exits as
intentional: the cardinality exit says admitting on a partial plan "is exactly
the unsound accept this gate exists to prevent", and the availability exit says
a `None` `context_capture` "is precisely the refusal this gate must respect
rather than route around". **Relaxing either is forbidden by the ruling
recorded at the gate.** Supplying what is missing is planner-side work in the
`RT-CAPTURE-PROJECTION-GROW` / `RT-CAPTURE-SUPPLY-DECLARED-INPUTS` lineage, not
an S-sized edit at this refusal.

### 9.7 The cause is already OWNED, and a six-node chain has attacked it

**Before flagging the operand question to the Architect I searched the issue
corpus for these row names, and it is already ruled.** The `D2` route inside
that predicate warns that its two operands count different populations, and
`D3` below it nonetheless requires `claims.len() == captures`. That is not an
expected off-by-one -- the difference IS the defect, measured and dispositioned:

[[RT-CAPTURE-CARDINALITY-GAP]] `D0` came back **all-H1, zero-H2**: every
unclaimed capture is a genuine body-referenced value the planner never
projects, dropped at `continuations.rs:6075`, where the context's capture set
is cloned from the enclosing specialization's `continuation_inputs` -- *a
different population than the closure's declared set*. Its own per-witness
table carries two of my four rows with the capture counts I measured
independently today:

    px7m dynamic_ok    Captures 4    (my probe: captures=4, claims=3)
    px7m dynamic_err   Captures 5    (my probe: captures=5, claims=4)

**The chain that owns this is six nodes long and every node is merged or
closed:**

    RT-BRANCH-LOCAL-DECLARED-CALLABLE   merged
    RT-CAPTURE-SUPPLY-DECLARED-INPUTS   merged
    RT-CONTSRC-ENTRY-FRAME-WIDEN        merged
    RT-CAPTURE-CARDINALITY-GAP          merged  (closed at D0, all-H1)
    RT-CAPTURE-PROJECTION-GROW          merged  (landed partial: D1 + D3)
    RT-CAPTURE-CONTEXT-FRAME-EMIT       merged  (carried the closing
                                                 deliverable; held the
                                                 blocks edge)
    NATIVE-HANDLE-CARRIER               closed
    PX8-F-CAP-41                        closed

⇒ **These four rows are the RESIDUAL of a fully-merged chain, not an unattacked
gap.** That is the fact this node contributes to sizing, and it is why the
repair is not S-sized at this refusal. One correction to the record while I am
here: [[RT-CAPTURE-SUPPLY-DECLARED-INPUTS]] line 212 records the px7l row as
*"no context owns the body at all"*. **At `3f4ae2d83` that is no longer true** --
a context does own the body and declares 3 claims for 3 captures; the refusal
has moved to claim 0's absent `availability.context_capture`. The chain moved
the failure without greening the row, which is exactly the
necessary-but-not-sufficient pattern its own frames describe.

### 9.7a THE MECHANISM, located to one field

The two verdicts in 9.5 are both DOWNSTREAM of a single cause, and the
discriminating measurement is the `D2` admission route that
[[RT-CAPTURE-CONTEXT-FRAME-EMIT]] landed. Instrumented at that route, the
constructed frame is **present in every one of the four failing cases** -- never
absent -- and its cardinalities MATCH every time:

    row          body   captures/claims   live frame: worker_body_origin  w/c
    px7m err     378        5 / 4                     341                5/4
    px7m ok      369        4 / 3                     332                4/3
    px7l 153     322        3 / 3                     343                3/3
    px7l 220     347        3 / 3                     362                3/3

    control, same programs, ADMITTED:
    px7l         343        3 / 3                     343                3/3
    px7l         362        3 / 3                     362                3/3

⇒ **The single discriminator across all four failures is
`frame.worker_body_origin != body_origin`.** Nothing else differs. The control
rows are decisive: origins 343 and 362 are admitted while their OWN frame is
live, and 322 and 347 are refused while a SIBLING body's frame is live.

**`function_local.constructed_context_frame` is one
`Option<ConstructedContextFrame>` slot** (`mod.rs:1249`), written
unconditionally at `core.rs:10866`. A function that constructs context frames
for two worker bodies keeps only the last, so the query for the other body
falls through to the `D3` route -- which then reports `cardinality` on px7m
(where the two populations genuinely differ in size) or `claim-availability` on
px7l (where they happen to agree and claim 0 is producer-local). **One slot
serving an N-body population is the defect; the two verdicts are its two
shadows.**

The write SITE is deliberate and its comment argues for it well -- a frame
written at the assembly point is available to whichever consumer reaches it,
and writing at a consumer is a shape the chain "has already paid for four
times". **That reasoning is about WHERE to write, and it is sound. It does not
address HOW MANY to keep.** Keying the slot by `worker_body_origin` is the
shape the evidence points at, and it is a successor node's call, not this one's
-- it changes a fail-closed admission gate that six merged nodes have worked
over, which is above this node's S/T1 authority.

**What this contributes: the successor's `D0` is already run.** The route is
REACHED, the frame EXISTS, the cardinalities AGREE, and the refusal is a
keying artifact. That is the measurement
[[RT-CAPTURE-CONTEXT-FRAME-EMIT]] itself had to pay for on its own population
("the push seam executes ZERO times"), and it does not need paying again here.

### 9.8 AC-3 -- the crate closure, derived

    seed        derived from the diff against the merge base
    method      cargo tree -i <p> -e normal,dev, NO --depth, iterated to a
                fixpoint, intersected with the workspace members
    closure     ken-runtime  ken-cli  ken-elaborator  ken-interp  ken-verify
    count       5

Seeding `{ken-runtime}` and `{ken-runtime, ken-cli}` both reach the same five,
so the closure is stable under the seed this candidate actually touches.
`cargo tree` compiles nothing -- it reads manifests and the lockfile -- so
section 12 is not in play. **Target selection is stated beside every green
claim below, because `cargo check` does not compile `#[cfg(test)]`.**

### 9.9 AC-4 -- the aggregates.rs:3340-3356 scope comment

**It stays true and is not edited.** It says the refusal "does not become
reachable or unreachable by anything this projector does", and this candidate
changes no lowering code at all, so reachability is byte-unchanged. One note
for whoever reads it next: the comment describes the refusal as "the next layer
behind" the projector, which is accurate about ORDER and, on the evidence in
9.4, understates the distance -- the layer that decides is
`resolve_recursive_unit_body`, and the refusal is downstream of it.

### 9.10 AC-5 -- the rt_allocate_stage disposition

**It did not move, and it had already moved before this node.**
`rt_cold_lowering_path_enumeration.rs:547` disposition `rt_allocate_stage` as
`Completes`, with an in-file note recording that it was previously
`Refuses { key: "a carried recursive hypothesis is an eliminated value" }`
until a recut retired that blocker. So the recorded cold-lowering witness of
this refusal no longer exhibits it, while the four rows still do. **That is a
result: the witness and the rows have come apart, and the witness is no longer
evidence about this signature.**

### 9.11 AC-1 -- the four rows, dispositioned per row by name

**None is readmitted and none is retired.** The refusal they hit is CORRECT,
and the rows are CORRECT -- they assert native/interpreter agreement on valid
Ken programs, and nothing measured here makes that assertion false. So the
honest disposition is the third one, and the node reports it as a hard stop
rather than manufacturing one of the two the frame named.

    px7l:153  delayed_capturing_generic_bind_agrees_across_real_executors
              STAYS IGNORED. Refused at source.rs:5016. Live frame keyed to
              body 343 while the query is body 322; fallthrough then refuses
              on claim 0's absent availability.context_capture.

    px7l:220  runtime_selected_non_unit_response_is_consumed_across_real_executors
              STAYS IGNORED. Same route. Frame keyed to 362, query is 347;
              same fallthrough, same claim-0 refusal.

    px7m:153  dynamic_ok_payload_selects_a_multistep_tree_across_real_executors
              STAYS IGNORED. Same route. Frame keyed to 332, query is 369;
              fallthrough refuses on D3 cardinality, 4 captures / 3 claims.

    px7m:185  dynamic_err_payload_selects_a_multistep_tree_across_real_executors
              STAYS IGNORED. Same route. Frame keyed to 341, query is 378;
              fallthrough refuses on D3 cardinality, 5 captures / 4 claims.

**What changed in the tree for each of them: the label.** All four carried a
byte-identical `#[ignore]` reason asserting the arity refusal AS the mechanism,
and a comment block above asserting an owner node
(`RT-CARRIER-BYTESPAN-OBSERVE`) and an *"observed signature, exactly"* that
these rows no longer produce. Both claims are now false in the tree and both
are corrected in place, per row, naming the measured cause and the readmission
condition. The superseded owner is kept and marked superseded rather than
deleted, because the px4b rows still carry it legitimately.

**No `.github/ignored-test-exemptions.toml` rows are added, deliberately.**
Deliverable 4 triggers on a row *readmitted with an accepted red*; no row is
readmitted. Registering them would move four rows out of the sweep's
failing-ignored population and into the exempt one, silently changing the
denominator of the ledger that was just landed at `5aa81990c`. That is a
measurement change disguised as bookkeeping, and it is not this node's to make.

### 9.12 AC-2 -- the lib suite, with its baseline re-measured

The frame's baseline is `main` at `c041de7c3`. **`main` moved under this node**
(`3f4ae2d83`), so per AC-2's own control it was re-measured rather than carried:

    ken-cargo build -p ken-runtime --lib     (staticlib materialized first)
    ken-cargo test  -p ken-runtime --lib
    1035 passed   0 failed   2 ignored       at 3f4ae2d83

**Identical to the frame's `1035 / 0 / 2`.** The three buckets AC-2 asks for:

    red-on-both (pre-existing)                    0
    green-on-main, red-here (mine)                0
    candidate-only, no main counterpart           0

The third is published at zero as instructed. It is zero for a structural
reason worth stating rather than asserting: **this candidate changes no
`crates/ken-runtime` source at all.** Its diff is two `ken-cli` test files
(annotations only -- `#[ignore]` reasons and comments, no test body and no
expectation touched) and this frame. The probes used to reach every measurement
above were reverted before the first commit; `git status` was clean of them at
commit time.

**Target selection, stated beside the claim as AC-3 requires:** `--lib`. This
is a `test` invocation, not `check`, so `#[cfg(test)]` was compiled and the
1035 rows genuinely built and ran.
