# `RT-LEXICAL-ROW2-MISSING-MINT` — frame

**Owner:** runtime. **Size:** S. **Node:**
`docs/program/issues/RT-LEXICAL-ROW2-MISSING-MINT.md`.

**Fixed inputs measured at `41b75c7c91d29ffc7be7901b7d5deb003634a092`** — the
`RT-LEXICAL-RECURSOR-CONSUMERS` `D2a` candidate. Line numbers are anchors to
re-find at your own base, not values to trust. Re-measure and say so.

**Do not start this while Runtime holds `RT-LEXICAL-RECURSOR-CONSUMERS`.** See
*Contention*.

## 1. What the row is

Row 2 of the lexical-recursor population is the `all_three_producer_paths`
fixture family: one expression, complete residual set
`{LexicalCallArgumentRecursor}`, unexcluded lane `RecursiveDescent`, and under
`B`-only exclusion the lane becomes `FunctionizedUnits`.

The observation site is
`crates/ken-runtime/src/cranelift_backend/lowering/core/tests/control.rs`, in
`px8j_all_three_producer_paths_reach_real_consumers`:

```rust
.find_map(|event| match event {
    Px8jSourceTraceEvent::Mint { path: actual, origin, cursor, siblings, .. }
        if *actual == path && *siblings > 0 => Some((*origin, *cursor)),
    _ => None,
})
.unwrap_or_else(|| panic!("{path:?} must mint a recursive IH"));
```

**Under exclusion the compile returns `Ok`.** There is no refusal, no boundary,
and no lowering authority involved. The test panics because the captured trace
carries no `Mint` with `siblings > 0` for one of the two producer paths.

⇒ That is why this row left `RT-LEXICAL-RECURSOR-CONSUMERS`: that node's `D2` is
*"repair only the proven root boundary or boundaries"*, and a failure with no
boundary is not expressible in it. The split is a scope judgment, not a
difficulty judgment.

## 2. The design judgment, front-loaded

### 2.1 The whole `px8j` trace is a test-only instrument

Measured, not assumed, all three at `41b75c7c`:

- `lowering/mod.rs:432` — `#[cfg(test)]` on `enum Px8jSourceTraceEvent`.
- `lowering/mod.rs:605` — `#[cfg(test)]` on `fn px8j_record_source_event`.
- `lowering/core.rs`, immediately above the emission at `:4886` — `#[cfg(test)]`
  on the call itself. Same shape at `:5445`, `:6979`, `:13591`.

**There is no production consumer of `Mint`.** Nothing downstream of the
compiler reads this event, and no shipped behaviour changes when it is absent.

⛔ **This makes one tempting repair unfalsifiable, and it is the one to refuse.**
Adding a `Mint` emission on the `FunctionizedUnits` path so the assertion finds
one would be adding a test-only event to satisfy a test-only observer. The
suite would green and nothing about the lane would have changed. **Do not repair
at the emission site.**

### 2.2 The field that makes this falsifiable anyway

The emission at `core.rs:4886` sets

```rust
siblings: case.recursive_positions.len(),
```

**`siblings` is derived from production state.** The event is test-only; the
number in it is not. So `siblings > 0` failing is a fact about the lowering, and
it has exactly two causes:

- **(i) the emission point is never reached** on the `FunctionizedUnits` path
  for this fixture, or
- **(ii) it is reached with `case.recursive_positions` empty.**

**`D0` decides which, and that is the deliverable.** They are different findings
with different repairs: (i) is a missing step in the lane, (ii) is a lane that
runs the step over a case whose recursive positions were never populated.

⇒ The decision procedure does not rest on the instrument. Determine whether the
`FunctionizedUnits` path performs the underlying recursive-IH installation at
all, at the mechanism, independent of what it records about itself.

### 2.3 The guard this must not weaken

`RT-LEXICAL-RECURSOR-CONSUMERS` `AC-3` guard 5 — *a missing recursive-IH
authority still refuses* — is the one this row is adjacent to. Whatever `D1`
does, a fixture whose recursive-IH authority is **genuinely** absent must still
refuse afterwards. Row 2 is currently the campaign's only probe on this seat, so
a repair that satisfies row 2 by making the authority appear unconditionally
removes the probe and the guard together.

## 3. Deliverables

### `D0` — attribute, before any repair

Decide (i) versus (ii) from §2.2 at your own base, under `B`-only exclusion,
using the existing one-variant hook as designed.

- Record the **activation denominator**: prove the `FunctionizedUnits` path was
  actually reached for this fixture, so an absent `Mint` cannot be credited
  where the path never ran.
- State which producer path of the two fails — `Composed` or `SourceMachine` —
  and whether the other one mints normally in the same trace.
- Keep the ordinary unexcluded run green as a positive control.

**If `D0` shows row 2 is not one row** — a sibling in the same class, or a
reassignment — **post it and stop.** The population here is one row on the
strength of the Architect's ruling at `evt_nae7n2yxg0mk` accepting that all
three formerly-deferred `B`-only compiles are `R1`; that closed the `R1` cell at
five compiles across rows 1 and 4 and left nothing else here. That is the
evidence this node was waiting on, and it is now in. It is still a floor.

### `D1` — the bounded repair at the attributed cause

> #### `D1` IS CLOSED WITH NO PRODUCTION REPAIR
>
> Steward ruling `evt_26cb49zckgq4f`, 2026-08-12, on the `D1` handback.
>
> **The bounded-installation question was answered by measurement, and the
> answer is zero.** Under `B`-only exclusion the `SourceMachine` seat is entered
> **once**, with `LoweringOperand::Carried` — a runtime word — and returns before
> the specialized-constructor selection the mint sits behind. That arm is **not
> a branch taken instead of an installation step**: its own contract is that a
> carried value has no compile-time `Lowered::Constructor` to read, and
> `core.rs:7425` refuses exactly that. Descent's three entries at the same seat
> are compile-time unrolling.
>
> ⇒ **There is no missing step at the attributed cause. No mechanism is to be
> built here.** A required count of zero is a legitimate answer to a bounded
> question, not a failure to locate the repair.
>
> Two alternatives were closed by measurement rather than argument: the compile
> plans three units at origins 33, 17 and 31 and **the eliminator's origin is
> not among them**, so no unit's call went unemitted; and the surviving
> `Composed` mint comes from the **carried** site, not the baseline's
> specialized site.
>
> ##### WHAT IS *NOT* RULED, AND MUST NOT BE INHERITED AS THOUGH IT WERE
>
> **`SourceMachine` is NOT ruled descent-specific.** One occurrence in one
> fixture does not generalize to the producer path.
> `px8j_all_three_producer_paths_reach_real_consumers` builds its subject with
> `recursive_computational_result_depth(2, ..)`, so **the only occurrence it
> ever constructs is the recursive one** — the fixture cannot distinguish *this
> occurrence routes elsewhere* from *the path is dead*.
>
> That distinction is the whole distance between a row close and a campaign
> fact. If every recursive occurrence routes to the carried arm under
> functionization, retiring `RecursiveDescent` removes the `SourceMachine`
> recursive-IH producer's only input — **and it would also make this node's own
> reason for blocking `D3` false** (see *Sequencing* in the tracker node: row 2
> is on the bar because its subject *survives* the retirement, expressly
> contrasted against `#7`'s spent oracle). That may turn out to be so. It is not
> established, and it would reverse Architect ruling `evt_2jnf3x8f06psz`.
>
> ##### THE SUCCESSOR MEASUREMENT — Runtime's, and it is not a repair turn
>
> > On the functionized lane, is the recursive IH for the occurrence
> > `SourceMachine` would have handled actually installed **and consumed** — by
> > the carried/`Composed` path — or is it absent?
>
> **Absent** ⇒ a real semantic regression; row 2 stays on `D3`'s bar.
> **Present** ⇒ the assertion is **over-specified** — it pins *which producer
> path mints* where the invariant is that a recursive occurrence gets an IH
> installed and consumed — and the row's subject is re-cut. Either way the
> campaign question (*is the `SourceMachine` recursive-IH producer reachable on
> the functionized lane by any occurrence?*) goes to the **Architect on that
> evidence**, not as an open question and not as a row-level close.

The text below described the repair `D1` was expected to make. It is retained
because `D0`'s licence lives inside it and `D2` is grounded on that licence.
**Read it as the attribution's record, not as an outstanding instruction.**

Repair only what `D0` attributed, at the mechanism, not at the trace. Do not
touch the `#[cfg(test)]` emission, the enum, or the assertion.

> #### `D0` HAS LANDED. THE ATTRIBUTION IS FIXED INPUT — DO NOT RE-DERIVE IT.
>
> **Merged 2026-08-12 at exact `3e569191`**, QA `evt_5r8mdv2hjzzxd`, Architect
> Decision `dec_3rasp837yerxh`. The answer to §2.2's fork is **cause (i),
> never reached**, on the **`SourceMachine`** producer path. `Composed` mints
> normally at `siblings == 1` in the same trace.
>
> **The location is measured, not guessed.** The three mint sites sit in three
> different functions; the missing one is in **`lower_source_machine`**. `D1`
> is the branch that function takes on the functionized lane instead of the
> specialized-selection arm, for this occurrence. Start there.
>
> ##### THE LICENCE IS THE EMISSION SITE'S STRUCTURE, NOT THE SENTINEL'S POSITIVE
>
> **Corrected 2026-08-12 from Adversary `evt_42kas9r9vx0g0`, confirmed. An
> earlier version of this block stated the attribution and not its ground, and
> the ground a reader would have inherited is the wrong one.**
>
> The sentinel's positive control, `baseline_zero_sibling_source_machine`, is
> measured on the **baseline** run — the run the same assertion requires to
> have **not** reached the functionized seam (`baseline_declared.is_some()`
> required `false`). **It is a neighbouring-path witness and does not on its
> own license a claim about minting on the functionized lane.** Do not cite it
> as the ground, and do not treat the merge as having settled it.
>
> **What does license cause (i), measured at `2ced8796` from the git objects:**
>
> - There is **exactly one** production `SourceMachine` mint site,
>   `core.rs:7516`, in `lower_source_machine_with_continuation_inner`. The
>   other three are `Composed` (`:5316`, `:14174`) and `DeferredConstructor`
>   (`:5882`). There is no second site a functionized lane could own, so
>   lane-independence is the absence of an alternative rather than an
>   assumption.
> - **That site is NOT wrapped in an emptiness guard.** At `:7516` the emission
>   is unconditional once control arrives and carries
>   `siblings: case.recursive_positions.len()`.
>
>   **Read this as an absolute property of the one site, not as a contrast.**
>   An earlier version offered the guarded `Composed` mint at `:14174` as *the*
>   contrast, which reads as though guarding were the norm — **it is not; the
>   other `Composed` mint is unguarded too** (`D1` handback, 2026-08-12). The
>   argument does not need a sibling to compare against and is weaker when
>   stated as one.
>
> ⇒ **An empty case at `:7516` mints with `siblings == 0` rather than minting
> nothing** — that is cause (ii)'s signature, and it is absent. Zero
> `SourceMachine` mints of every arity therefore means the site was **not
> reached**.
>
> **The fallible exits, chased rather than left open. There are THREE, not
> one** — corrected from the `D1` handback, 2026-08-12. Besides
> `computational_ih_slots_for_case(case, frame.checked_frame_id)?` at `:7513`,
> the window carries the arity refusal and the malformed-recursive-position
> refusal. **All three are `Err`-valued**, and row 2's defining property is
> that its compile returns `Ok`, so none of them fired on the observed run —
> had the site been entered, the mint would have. **The caveat closes the
> argument rather than opening it, and it closes wider than first stated.**
>
> **Bounds, and they are the Steward's:** this is a structural read of the
> site, executed nothing, one seat. **`D2` must verify it rather than credit
> it**, and `D2` owns committing it as the replacement licence — see `D2`'s
> fourth bullet. **Replace the stated ground; do not leave the baseline
> positive standing beside it as though the two were alternatives.**
>
> **The one-versus-four count is `D1`'s bounded-installation question, and it
> is NOT authority to install every absent mint.** Runtime-leader scope
> clarification in `thr_2amp93z5apamk`, restated at merge: the functionized
> lane mints **once** where descent mints **four times**. `D1` owns deciding
> *which* `SourceMachine` installation this lane actually requires, because
> that bounds the attributed repair. Installing all three absent mints
> unconditionally is out of scope and is the tempting over-repair.
>
> **The `D0` sentinel is DELETED by this repair, never relaxed.** It is a
> transition sentinel that asserts all five facts as one assertion and is
> written to go red when `D1` lands. Deleting it is the designed retirement;
> weakening its operands to keep it green is not.
> `px8j_all_three_producer_paths_reach_real_consumers` is the real control.
>
> **The standing trap, and it is why a green suite proves nothing here.**
> The whole `px8j` trace is `#[cfg(test)]`. Adding a `Mint` emission on the
> `FunctionizedUnits` path to satisfy the assertion is a test-only event
> feeding a test-only observer: the suite greens and **the lane is unchanged**.
> What keeps this node falsifiable is that `siblings` derives from production
> state. **A repair at the emission site is the failure mode, not the fix.**

### `D2` — discriminating controls, separate from `D1`

> #### `D2` MERGED 2026-08-12 at exact `9e95767e`, PR #1955, CI green
>
> QA `evt_4vpp6pyx512ng`, Architect Decision `dec_5rwbbqmnre70f` resolved at
> `evt_2yp78y43pd61y`. One test-only path, `+152/-23`. M6 verified by blob
> identity from the declared merge-base `14578cf0`: one path, `MATCH`.
>
> **The fourth bullet is discharged. The `D0` sentinel's licence is replaced,
> not supplemented.** The baseline zero-sibling operand is **gone from the
> executable predicate**, and the new positive is on the excluded lane:
> `D6aRouteEvent::ConsumerRoute { seat: SourceMachine }` has exactly one
> production emitter, inside the carried arm of that seat — the arm that invokes
> carried lowering and breaks before the specialized selection and before the
> sole `SourceMachine` `Mint`. Under exclusion it fires; **the identical
> predicate is required false on the baseline**, so the operand cannot be a
> constant. QA forced that field red by exact mutation.
>
> **A false premise was removed rather than narrowed.** An earlier cast carried
> a `CarriedEliminationEntered` cross-event operand and an origin join; both are
> deleted from the predicate and the tuple. The prose keeps the rejected
> ordering only as an explicit non-claim, with equal origins explaining why an
> origin filter would still be false attribution.
>
> **The sentinel is corrected in place, not deleted**, because no repair landed.
> Its retiring event is restated as whatever settles row 2 — which, after `D1`'s
> closure, is no longer certainly a repair at the attributed cause.
>
> The first three bullets below are **not** discharged: they presuppose a
> repaired root, and there is no repair. They are superseded by the successor
> measurement in `D1`'s closure block. `AC-4`'s negative control remains owed
> wherever row 2 finally settles.

A control set appended to an implementation deliverable is the shape that got
three `D6` candidates rejected on the sibling node. Keep them separate.

- A **mutation at the repaired root** recreates the attributed failure while
  proving the detector was reached.
- A **negative control on `AC-3` guard 5**: a fixture with a genuinely absent
  recursive-IH authority still refuses after the repair.
- The unexcluded run and the same-family rows stay green and unchanged in
  meaning.
- **Re-ground `D0`'s cause-(i) licence, and replace it rather than adding to
  it.** The merged sentinel states its ground as the baseline's zero-sibling
  `SourceMachine` mint; that witness is on the lane the excluded run does not
  take, so it does not license the attribution by itself (Adversary
  `evt_42kas9r9vx0g0`, Steward ruling `evt_26f4pmjxchqzp`). Verify the
  structural argument in `D1`'s licence block — single emission site, no
  emptiness guard, `siblings` from `.len()` — and commit **that** as the
  stated ground, correcting the sentinel's doc block in place. **A wrong
  reason left standing beside a right one is what the next reader inherits.**
  If the structural read does not survive your check, that is a stop: the
  attribution loses its ground and `D1`'s premise is back open.

## 4. Acceptance criteria

- **`AC-1`** `D0` names cause (i) or (ii) with a stated activation denominator,
  and names which producer path fails. *Control:* the handback carries the
  denominator, not just the verdict.
- **`AC-2`** — **STRUCK AND REPLACED, Steward, 2026-08-12** (`evt_26cb49zckgq4f`).

  > **Struck: *"Row 2 passes under `B`-only exclusion. Control:
  > `px8j_all_three_producer_paths_reach_real_consumers` green with
  > `RecursiveDescent` excluded."***
  >
  > **It became dischargeable only by the two things this frame forbids.**
  > `D1` measured that the functionized lane requires **zero** `SourceMachine`
  > installations for this occurrence, so greening that control needs either a
  > fixture whose occurrence the exclusion does not functionize — changing the
  > assertion's subject, which `D1`'s licence forbids — or an added emission,
  > which §2.1 names as *the* failure mode. An acceptance criterion whose only
  > two discharges are both banned by its own frame is a defective criterion.
  > **This one is the Steward's; it is replaced, not to be satisfied.**

  **`AC-2` (replacement) — the sets are MEASURED now, at `6a804eb7` (PR #1957,
  CI green).** For row 2's occurrence, the row's producer-path assertion is
  re-cut **lane-conditionally, each side an exact enumerated set**:

  | lane | installed-and-consumed producer paths |
  |---|---|
  | descent (baseline) | exactly `{Composed, SourceMachine, SourceMachine}` |
  | functionized (excluded) | exactly `{Composed}` |

  *Control:* red if any enumerated path is absent **and** red if an unenumerated
  path appears.

  **`Mint`, `Install` and `DirectConsume` stay three separate observations.** A
  seat can compute a route on a path whose eliminator never runs, so *an IH
  exists* and *it was consumed* must not collapse into one predicate. The
  measured cells are `(minted, installed, consumed)` triples, and the fourth —
  functionized `SourceMachine` at `(false, false, false)` — is what proves the
  join can answer `false` rather than being satisfied by any trace.

  **State the identity bound in the AC, not only in the test.** No `Px8j` event
  carries a `StaticOriginId`, so the tie to row 2's occurrence is **uniqueness
  across traces**, not a same-event key. **Same static origin means same
  occurrence, not same seat** — the carried elimination is the `Composed`
  seat's. That is the easy misreading and it must stay named.

  **It stays absolute; it does not become existential.** Relaxing to *"some
  path mints"* would weaken the adjacent guard §2.3 protects —
  `RT-LEXICAL-RECURSOR-CONSUMERS` `AC-3` guard 5, *a missing recursive-IH
  authority still refuses*. A discriminator is not a substitute for the matrix.

  **UNBLOCKED — the successor measurement returned, and it fires the PRESENT
  branch.** On the functionized lane the recursive IH for row 2's occurrence
  **is** separately minted, installed and consumed, by the carried/`Composed`
  route. **Row 2 is not a semantic regression.** The assertion is
  **over-specified**: it pins *which producer path mints* where the invariant is
  that a recursive occurrence gets an IH installed and consumed.

  **What the measurement does NOT settle, and must not be read as settling.**
  Whether the `SourceMachine` producer path is reachable on the functionized
  lane by **any** occurrence. The object is occurrence-bounded by construction
  and grants no producer-path conclusion; the one-fixture limit still binds,
  because `recursive_computational_result_depth(2, ..)` builds only the
  recursive occurrence. Baseline showing **two** `SourceMachine` lifecycles
  against the excluded lane's **zero** is a hint consistent with
  descent-specificity **and** equally consistent with this occurrence routing
  elsewhere. A hint is not a measurement.

  **Two Architect calls, and the first must not silently answer the second.**
  (1) Is over-specification the right diagnosis, so the assertion is re-cut as
  above? (2) Does row 2 leave `D3`'s six-row bar? The bar was retained by
  `evt_2jnf3x8f06psz` on the ground that row 2's subject **survives** the
  retirement. If the invariant is IH-installed-and-consumed rather than
  path-identity, that ground reads differently — but (1) is a correction to an
  assertion and (2) changes the **retirement acceptance surface**. Different
  calls, and neither is the row's nor the Steward's.

  > **Struck, retained for provenance:** *"This AC is blocked on the successor
  > measurement in `D1`'s closure block and cannot be written until it returns:
  > which paths belong in the functionized set is exactly what that measurement
  > decides."*
- **`AC-3`** No repair lands in the `#[cfg(test)]` trace machinery. *Control:*
  `git diff` on the candidate touches neither the `Px8jSourceTraceEvent`
  declaration, nor `px8j_record_source_event`, nor any `Mint` emission
  expression.
- **`AC-4`** `AC-3` guard 5 still refuses on a genuinely missing recursive-IH
  authority. *Control:* the `D2` negative control, red before the guard and
  green after only because the authority is present.
- **`AC-5`** No added `#[ignore]`. *Control:* `git diff` contains none.
- **`AC-6`** No retirement, no lane deletion, no tracker `status:` change in this
  candidate.

"No regression" means **green in CI**, never a local `--workspace` run. Targeted
`scripts/ken-cargo` only.

## 5. Scope

`crates/ken-runtime/src/cranelift_backend/` only.

⛔ Not in scope: `R2` (row 3) and `R3` (row 5), which remain in
`RT-LEXICAL-RECURSOR-CONSUMERS` and are **not** authorized to fold on their
shared *boundary-transfer* name; the `StaticWorkerBinding` wall that rows 1 and 4
advanced to under `D2a`; `RecursiveDescent` retirement itself; and any change to
the `px8j` instrument.

## 6. Contention

**Contended with the active recursor arc.** This file set is
`crates/ken-runtime/src/cranelift_backend/lowering/core*`, which is exactly where
`RT-LEXICAL-RECURSOR-CONSUMERS` `D2a` landed and where its `R2`/`R3` residual
will land. Runtime runs one node at a time.

**ON the RecursiveDescent critical path. Corrected 2026-08-11**, Architect
ruling `evt_2jnf3x8f06psz` on a Steward escalation.

> **STRUCK — *"This is not on the RecursiveDescent critical path.
> `RT-DESCENT-RETIRE` gates on `RT-RECURSOR-TRANSPORT` and
> `RT-FNUNIT-RESULT-TOKEN`, and this node is neither. Sequence it behind the
> arc, not into it."***
>
> **The reasoning checked the wrong node.** It read `#7`'s dependencies
> correctly and never asked whether **`#6b`'s own `D3` bar** includes row 2. It
> does — `D3` must prove all six rows green with no exclusion hook, and row 2 is
> one of the six. `#6d` cannot repair it.
>
> **The `R4` carve-out changed the repair OWNER, not the retirement ACCEPTANCE
> SURFACE.** Those are two questions and the carve-out answered only the first.
>
> **This correction landed on the issue node first and was stranded here for
> several hours** — the frame is what a ring builds from, so the stale sentence
> was the one that would actually have been acted on.

**Release order: after [[RT-LEXICAL-RECURSOR-CONSUMERS]], before
[[RT-RECURSOR-TRANSPORT]] `D3`.**

### The file contention is real but it is not a standing bar

The `lowering/core*` overlap above is a **one-ring-at-a-time** coherence
constraint, not a parallel-edit hazard: Runtime is a single ring, so the only
question is which node it is running. **When `#6d` is stopped on a ruling with
its branch released and its tree clean, the contention is not live** and this
node is the right thing for the ring to run.

The one genuine intersection to clear first is **unmerged** work touching
`lowering/core/tests/control.rs`, which is this node's own observation site.
Merge that before releasing here.
