# RT-SUBCONTINUATION-LIFO-RELEASE-ORDER — work package

**Owner: Team Runtime. Size S. Tier T1. Gate: none.**
**Implementation base: `origin/main` at
`e85ef3f756b9d8325fc57402605aeb58fc3ce2f4`.**

Operator priority, 2026-09-15: *"The other tests should be fixed."* Operator,
2026-09-17: *"Is L1 still working on clearing the ignored tests? That is the top
priority until it is done."* Covers ledger row **9**.

> **Every coordinate, count and status here is PERISHABLE and was measured at
> the base above.** Re-ground before acting. If a re-measurement disagrees, the
> tree wins and the disagreement is a finding, not a discrepancy to reconcile.

## 1. Objective

**This is the only row of the failing fifteen that executes and returns a wrong
answer rather than refusing to build.** Establish whether the ordering defect is
real, and what owns it.

`docs/program/issues/RT-SUBCONTINUATION-LIFO-RELEASE-ORDER.md` carries the
measurement, the direction resolution, the `D0` fork and the bans. **Read it
first; this frame does not restate it.** Where they differ, the node governs.

**`D0` is a genuine fork and it is not a formality.** Under `(i)` this is a
product defect and the deliverable is an escalation, not a repair.

## 2. Fixed inputs, measured at `e85ef3f75`

**All verified by grepping for the token, not by reading a window.**

    crates/ken-cli/tests/px8ta_oriented_subcontinuation.rs
      :285   #[ignore = "RT-CLOSURE-BOUNDARY-LANE: ..."]       the live label
      :286   fn public_two_three_level_brackets_..._lifo       THE ROW
      :274   "It refuses at object emission, so the program    the INVERTING
             never executes and no binding order is            annotation,
             observable in it."                                spanning :274-275
      :311   .name("px8ta-nested-brackets")                    helper thread
      :315   assert_depth_finishes_and_releases_lifo(depth)    depth-2 call
      :396   #[ignore = "focused native resource-cost row"]    the DELIBERATE
                                                               exempt row -- not
                                                               one of the 15

    the assertion helper
      :458   fn assert_depth_finishes_and_releases_lifo(depth: usize)
      :484   "depth {depth} releases must be strict LIFO"

## 3. THE CONTROL THAT LOOKS LIKE ONE AND IS VACUOUS

**`assert_depth_finishes_and_releases_lifo` has a second caller that is NOT
ignored and passes:**

    :260   public_one_level_bracket_finishes_and_releases -> depth 1, GREEN
    :315   the ignored row                                -> depth 2, RED

**That green is not evidence the assertion is correct.** The helper builds
`opens` and `releases` with `len() == depth` and compares:

    assert_eq!(releases, opens.into_iter().rev().collect::<Vec<_>>())

⇒ **At depth 1 both vectors hold one element, and reversing a one-element vector
is the identity. The ordering assertion is VACUOUSLY SATISFIED at depth 1 under
every possible release order.** The depth-1 row cannot fail on ordering. It is a
control for exit status, terminal error, frame retention and identity — **not
for order.**

> **The mutation recorded in this file at `:249-256` does not rescue it
> either.** It reds the strict-LIFO assertion at depth 1 by perturbing the
> `ResourceRelease` arm of `ken-host` `effect_v1.rs` dispatch — but the recorded
> red is `[ResourceTraceIdentityV1(1001)]` against `[ResourceTraceIdentityV1(1)]`,
> **a changed identity VALUE, not a changed order.** It establishes the identity
> plumbing is live. It says nothing about ordering, because at depth 1 nothing
> can.

⇒ **The ignored depth-2 row is the ONLY observation of release ordering in this
file.** Do not cite the depth-1 green as stability evidence under `(iii)`, and
do not cite it as evidence the expectation is right under `(ii)`.

## 4. WHAT THE SAME FILE ALREADY KNOWS, AT `:234-256`

The depth-1 row carries a readmission note worth reading before `D0`:

- It records **three layers of annotation, each refuting the one above and all
  three still present** — including a layer-1 claim, in this same file, that a
  row *"refuses at object emission, so the program never executes"*, marked
  **refuted** because *"the program emits, executes, exits 0."*
- It was **readmitted on a mutation, not on a green.**
- It names the production site governing release dispatch: **`ken-host`
  `effect_v1.rs`, the `ResourceRelease` arm, the `Target` binding pushed from
  `pending.identity`** — and excludes `abi_v1.rs record_resource_settlements`
  as *"the process-exit finalize-all path, and a bracket that releases
  explicitly never reaches it."*

> **So this file already contains a refutation of the exact sentence the ignored
> row's annotation still asserts, twenty lines above it.** The inversion the
> node describes is not merely undetected — it is contradicted in-tree by its
> own neighbour. **That is a pointer for `D0`, not a licence to skip it.**

## 5. Deliverables

**`D0` — resolve the fork. `(iii)` FIRST.** Run the row enough times to
distinguish a stable wrong order from an unstable one. It is cheap, the observed
pair is exactly a transposition, and this row already has documented
harness-level interference (a 256 MiB stack wrapper and a helper thread whose
failure surfaces through a wrapper that *"carries no signature of its own"*).

**Under `(i)` — STOP AND ESCALATE. Do not size a repair.** A product defect in
release ordering is the only one the failing fifteen contains, and it is not
this WP's to fix. Report it with the run count that established stability.

**`D1` — re-measure at `main` and correct the annotation in the same change.**
The ledger's signature records `04d4dd38a`, not `main`. **Correct the annotation
to what is observed, not to this node's name** — if the row has advanced again,
say what it does now. Strike the layers rather than appending, as `:236-237`
already does.

## 6. Acceptance criteria

**AC-1 — the direction is read from the `assert_eq!`, not the message.** LEFT is
`releases` (observed), RIGHT is `opens.rev()` (expected). **Control:** a
deliverable saying "releases are reversed" rather than "releases are in
acquisition order" has read the panic message and fails this AC.

**AC-2 — `(iii)` is excluded with a stated run count**, or the row is reported
flaky. An assertion of stability without a count fails.

**AC-3 — the depth-1 green is not cited as an ordering control.** See §3. Any
argument resting on it fails this AC regardless of its conclusion.

**AC-4 — the annotation is corrected.** `D1` is not optional and not deferrable
to a follow-up; it is the thing that will mislead the next reader.

**AC-5 — no-regression, green in CI.** `COORDINATION §12`: targeted local runs
only, **never `--workspace`**.

## 7. What must not happen

- **Do not repair the assertion to match the observed order.** Under `(i)` that
  edits the test until it agrees with a defect. **The expectation is the thing
  under test here**, which is backwards from the other fourteen rows.
- **Do not un-ignore before `D0`.** Under `(iii)` a flaky row in CI is worse
  than a parked one.
- **Do not attribute it to `RT-CLOSURE-BOUNDARY-LANE`.** Merged, no frame, never
  names this row, and the ledger refutes its label for it. **Cited, not
  reopened.**
- **Do not fold this into the refusal-clearing program.** Its green means
  something different from every other row's. If it closes under `(i)`, that is
  a product fix and belongs in its own accounting.
- **Do not touch `:396`.** That row is `#[ignore]`d deliberately ("focused
  native resource-cost row; run outside default suite") and is **not** one of
  the fifteen. The file holds two `#[ignore]`s and only `:285` is in scope.

## 8. Contention

**Clean at this base.** `px8ta_oriented_subcontinuation.rs` is named by no other
live node. Under `(i)`, `ken-host` `effect_v1.rs` enters scope as a **read** —
and if a repair there is proposed, that is the escalation `D0` requires, not
this WP continuing.
