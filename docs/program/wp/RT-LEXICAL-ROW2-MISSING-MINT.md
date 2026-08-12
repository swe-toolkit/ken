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

A control set appended to an implementation deliverable is the shape that got
three `D6` candidates rejected on the sibling node. Keep them separate.

- A **mutation at the repaired root** recreates the attributed failure while
  proving the detector was reached.
- A **negative control on `AC-3` guard 5**: a fixture with a genuinely absent
  recursive-IH authority still refuses after the repair.
- The unexcluded run and the same-family rows stay green and unchanged in
  meaning.

## 4. Acceptance criteria

- **`AC-1`** `D0` names cause (i) or (ii) with a stated activation denominator,
  and names which producer path fails. *Control:* the handback carries the
  denominator, not just the verdict.
- **`AC-2`** Row 2 passes under `B`-only exclusion. *Control:*
  `px8j_all_three_producer_paths_reach_real_consumers` green with
  `RecursiveDescent` excluded.
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
