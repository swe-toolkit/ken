# RT-HOST-RESULT-ARM-SHAPE-DISAGREEMENT — work package

> # DO NOT WORK FROM THIS FRAME. IT IS BEING RE-CUT.
>
> **The node is `draft`, not `ready` (Steward, 2026-09-18). Read the node's
> head banner first.** `RT-CARRIER-PRODUCER-OCCURRENCE` `D3`/`D4` refuted parts
> of this frame while it sat `ready`, and the refuted parts are not coordinates
> — **they are an acceptance criterion and a fixed input.**
>
>     §3 D2 point 2   "whether BOTH ImmediateBool assertions survive untouched,
>                      as the ruling predicts"
>                     REFUTED. One of the two was deleted by D4 and replaced
>                     with `expected_error_tag`, a plan query. An implementer
>                     would measure this first, find it false, and be RIGHT to
>                     stop.
>     §3 D2 point 1   "does reconcile_declared_children still refuse at its
>                      input"  -- ANSWERED, NO. Row green, 1036/0/1.
>     §2 fixed input  the row's `#[ignore]` at :3333 -- GONE. The row is
>                     un-ignored.
>     §5 title        "THIS NODE DOES NOT CLOSE ITS ROW, AND THE #[ignore]
>                      STAYS" -- false. There is no #[ignore] on that row.
>
> **`(c)` SURVIVES** — the Architect ruled NARROWED, not SUPERSEDED
> (`evt_3a8xxpwmcy5xz`). The borrow is byte-unchanged across
> `2d440e394..a9fb242f0`; `D3`/`D4` changed what the arm is fed and what the row
> asserts, never how it is built. The re-cut is the Steward's and it leads with
> the strengthened motivation recorded on the node: removing the borrow closes a
> coverage gap the Architect measured independently while reviewing `D4`.
>
> **The lesson for frame authors, and it is why this banner is this loud:** a
> stale **coordinate** is defended by the perishable-anchor rule every frame
> here carries, and a reader re-measures it. **A stale ACCEPTANCE CRITERION has
> no such defence** — an AC encoding a prediction about a file another node is
> actively rewriting reads, once refuted, as a finding about the implementer's
> own work rather than as frame rot.

**Owner: Team Runtime. Size S. Tier T1. Gate: none.**
**Implementation base: `origin/main` at
`823c4a67cbfe8ef3e8fccf06acd1fa4c4a13af97`.**

Operator priority, 2026-09-15: *"The other tests should be fixed."* Operator,
2026-09-17: *"Is L1 still working on clearing the ignored tests? That is the top
priority until it is done."*

> **Every coordinate, count, line number and status in this frame is PERISHABLE
> and was measured at the base SHA above.** Re-ground each one against
> `origin/main` before you act on it — including the prose. A frame is not a
> status source and a line number is not a binding. If a re-measurement
> disagrees with this document, the tree wins and the disagreement is a finding
> worth reporting, not a discrepancy to reconcile silently.

## 1. Objective

**The design question is already ruled. This node builds the ruled repair and
resolves the one residual the ruling left open.**

`docs/program/issues/RT-HOST-RESULT-ARM-SHAPE-DISAGREEMENT.md` carries the
ruling, the discriminator that was run, the repair spelled out, the banned
moves, and the residual. **Read it. This frame does not restate it and does not
supersede it** — where the two differ on design, the node governs and the
difference is a defect in this frame.

The one-line statement: the row's ERROR arm is recipe-derived from `FsWriteAt`
and asserts a shape that recipe yields under neither root. Outcome `(c)` is
CONFIRMED — **the error arm should never have been recipe-derived at all** —
and the repair is to hand-build it carrying the same four fields.

## 2. Fixed inputs, measured at `823c4a67c`

**Production coordinates — all four verified present at this base:**

    crates/ken-runtime/src/cranelift_backend/lowering/aggregates.rs
      :3444   fn synthesized_fixed_identity      the identity the row needs
      :3459   fn synthesized_constructor         the borrow being removed
      :3613   fn reconcile_declared_children     where the row actually refuses
      :3882   `_ => false`                       the disjoint-forms catch-all,
                                                 the one under the "THE FORMS
                                                 ARE DISJOINT" comment

> **`:3882`, not `:3881` — and the enclosing function does not disambiguate
> it.** There are two `_ => false` arms in `reconcile_declared_children`,
> at `:3774` and `:3882`. **The comment is the discriminator**, not the line
> number and not the function.

**The row:**

    crates/ken-runtime/src/cranelift_backend/lowering/core/tests/
      constructors.rs
      :3333   #[ignore = "RT-CARRIER-PRODUCER-OCCURRENCE: ..."]
      :3334   fn c2_ac4_runtime_host_result_selects_a_separately_generated_...
      :3578   let error = compiler.synthesized_constructor(
      :3580     &SynthesizedAggregatePath::root(SynthesizedAggregateRoot::HostResultOk)

> **THE ISSUE NODE'S TEST-FILE COORDINATES ARE WRONG, AND NOT BECAUSE ANYTHING
> MOVED.** The node cites the error slot at `:3467` and the `ok` arm at `:3449`;
> at this base the error slot is at `:3578` and those lines hold unrelated
> content. **Nothing drifted:**
>
>     constructors.rs   973997af8 and 823c4a67c   51ae9f6d48...  IDENTICAL
>     aggregates.rs     973997af8 and 823c4a67c   8d117792ed...  IDENTICAL
>
> **The whole of `crates/ken-runtime/` is byte-identical between the node's base
> and this one.** `:3467` did not become wrong; it was wrong when it was
> written, against the very tree its author was reading.
>
> ⇒ **Re-locate anything in `constructors.rs` by content** — but for this
> reason, not for a drift that did not happen. **A wrong citation and a drifted
> one look identical at the far end**, and they have opposite implications: a
> drifted coordinate says re-measure at your base, a wrong one says the author's
> own base does not support it either, so nothing downstream of it is safe on
> its say-so.
>
> **An earlier revision of this frame asserted the drift reading and presented
> it as the one thing worth reading here.** The instrument that produced it read
> the right file at the right base and showed unrelated content; a cause was
> inferred from that without checking the cause's own precondition. **Two
> `git rev-parse` calls refute it**, and they were not run. Architect,
> `evt_3r5pbrvm9pdg2`.

## 3. Deliverables

**`D0` — RESOLVE THE RESIDUAL. THIS IS A GATE, NOT A STEP.**

The ruling left exactly one thing open: **what `occurrence` the hand-built
error arm carries.** Establish it before writing the repair.

    occurrence: None       LEADING. `synthesized_constructor`'s own
                           no-emission-owner early return produces this shape,
                           so it is lawful rather than degenerate, and the
                           downstream identity read does not consult
                           `occurrence` at all. The issue node records a refusal
                           "at the allocation" for occurrence-less templates --
                           CHECK THAT. One run.

    occurrence: Some(..)   a plan-resolved coordinate, IF one exists for this
                           aggregate.

> **IF OBTAINING THE OCCURRENCE REQUIRES PLANNER WORK RATHER THAN A LOOKUP,
> STOP AND ROUTE IT BACK. The node is then NOT `S` and this frame is void.**
> Report what you found and do not build. A node that grows past its size in
> flight is the Steward's to re-cut, and re-cutting it costs minutes; building
> the wrong size costs the turn.

**`D1` — apply the ruled repair.** The node spells it out, including that
`args` is spelled to match what `synthesized_constructor` produces and **not**
the `ok` arm's `fields:` — those are different types and the spelling does not
transfer. **Keep `Wrote`**; the row's `assert_ne!` selection check is stated
over it.

**`D2` — the measured outcome.** Run the row. Report, in this order:

1. whether `reconcile_declared_children` still refuses at its input,
2. whether both `ImmediateBool` assertions survive untouched, as the ruling
   predicts,
3. **if the row now fails somewhere new: NAME THE REFUSAL AND STOP.**

> **DO NOT CLIMB.** This chain has already withdrawn one ladder and banned a
> root-switch for being a rung on it. A new refusal after this repair is a
> RESULT — report it with its coordinate and its text, and let it be routed. It
> is not an invitation to force past it, and a second repair in this turn is
> out of scope whatever it costs to stop.

## 4. Acceptance criteria

**AC-1 — the borrow is gone.** No call to `synthesized_constructor` remains in
this row's error slot. **Control:** the `ok` arm is hand-built and must be
BYTE-UNCHANGED by this WP; it was never the defect. A diff touching the `ok`
arm fails this AC.

**AC-2 — the identity is preserved, not re-derived by a second route.** The
hand-built arm's `synthesized_identity` comes from `synthesized_fixed_identity`
with role `Wrote`. **Control:** the row already computes that identity for its
own `assert_ne!`. The two must agree, and the node's whole argument is that
nothing downstream can distinguish a hand-set `Some(x)` from a synthesized-set
`Some(x)`. **If they disagree, that is a finding that refutes the ruling** —
report it and stop rather than reconciling it.

**AC-3 — `D0` is answered in the deliverable, with the run that answered it.**
An occurrence choice asserted without the check is this AC failing. The node
names the check as "one run"; cite it.

**AC-4 — no row-closure claim.** See §5. A deliverable reporting this as
progress against the fifteen ignored rows fails this AC.

**AC-5 — no-regression, green in CI.** Per `COORDINATION §12`: targeted local
runs only (`scripts/ken-cargo`, `-p ken-runtime` or `--test`), **never
`--workspace`**. Workspace-green means green in CI on GitHub, not on this box.

## 5. THIS NODE DOES NOT CLOSE ITS ROW, AND THE `#[ignore]` STAYS

**Row 15 of the ledger has TWO independent refusals and this node owns one.**

    #[ignore] at :3333 names   RT-CARRIER-PRODUCER-OCCURRENCE   status: ready
    this node owns             the arm shape, downstream and unrelated

The carrier refusal is **upstream of and unrelated to** the arm shape, and its
owner has not merged. **Do not remove the `#[ignore]`.** Do not report this
node as clearing an ignored row, and do not let the row's continued failure
after this repair read as this node failing — that outcome is expected and is
written here in advance so it is not met as a surprise and quietly reframed.

**Out of scope, explicitly:** `RT-CARRIER-PRODUCER-OCCURRENCE`'s `D3`/`D4`. Its
`D4` removes the `#[ignore]` on the premise that the row can run. It cannot,
for this node's reason. That interaction is the one place the two nodes touch
and it is the concrete reason the Architect routed the arm question out.

## 6. Banned

**Switching the template's root to `HostResultError` and re-running.** Banned in
the node, and the ban WIDENS under the confirmed outcome: the defect is that the
row borrowed from a recipe at all, so the "correct" root is a DEEPER borrow —
it takes on the twelve-alternative surface's constraints on top of those already
being paid. **An edit whose result you can predict is not a measurement.**

**Outcome `(a)`, rewriting the error assertion.** REFUTED as a repair, not
merely deprioritized: it leaves the input refusal untouched, so it requires
*also* replacing the `Scalar(Bool)` with a two-nat aggregate — at which point
the row carries no bool and *"selects a separately generated nested payload"* is
being checked against a different value than the row was written about. **`(a)`
changes what the row tests.**

## 7. Contention

**Clean at this base.** The repair is confined to one test function in
`core/tests/constructors.rs`; `lowering/aggregates.rs` is READ, not written.

**One live neighbour:** `RT-CARRIER-PRODUCER-OCCURRENCE` is `ready` and works
the same row and the same file. **Check with the Steward before starting if
that node is in flight** — two seats in one test function is a merge conflict
the ledger cannot arbitrate, and this node is small enough to wait.

**`RT-DUPLICATED-RESPONSE-BLOCK` is in flight and is NOT contended:** it works
`planning/static_transition/responses.rs` and the `px7n` / `rt_escape` rows,
disjoint from everything above.
