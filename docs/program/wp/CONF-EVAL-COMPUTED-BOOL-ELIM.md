# CONF-EVAL-COMPUTED-BOOL-ELIM

Owner: **spec-enclave**. Size **S**. Gate: none.
Depends on: `CI-L1-EXECUTING-COVER`.

## 1. Objective

State, in `conformance/runtime/evaluation/seed-evaluation.md`, the obligation
that **a closed computed `Bool` consumed by the `Bool` eliminator selects the
same method as the corresponding constructor `Bool`** — and attach the existing
discriminating tests to the resulting rows so the landed row-claim checker
resolves them.

This is a **composition seam**. It is not new coverage of numbers semantics and
not new coverage of iota reduction; both halves are already covered and both
halves stay green under the bug this seam admits.

## 2. Fixed inputs, measured at `origin/main = bebe1a79`

Re-derive anything you intend to rely on. These were measured, not inherited.

**2a. The two runtime representations of one `Bool` are distinct, and the
eliminator has a separate arm for each.** `crates/ken-interp/src/eval.rs`,
`elim_reduce`:

- `EvalVal::Bool(b)` — what `eq_int`, `leq_int`, `not_bool`, `and_bool`,
  `or_bool`, `eq_float`, `eq_float32` return.
- `EvalVal::Ctor { id, .. }` — what a literal `True` or `False` reduces to via
  `make_ctor`.

**2b. The two arms derive the method index by different routes, and nothing
in the code ties them together.** This is the fact the family exists to pin:

- The `Ctor` arm looks the constructor up (`globals.constructor(ctor_id)`) and
  uses the index that lookup returns.
- The `Bool` arm computes `let k = if b { 0 } else { 1 }` — a **hardcoded
  index**, correct only because `data Bool = True | False` declares its
  constructors in that order with arity 0.

The in-source comment states the dependency explicitly. So a change to `Bool`'s
declared constructor order would be followed by one arm and not the other, and
**the only observation that could catch it is a computed-versus-literal
agreement check.** No such check is stated in the matrix today.

**2c. The historical bug is recorded in the source.** Before the `EvalVal::Bool`
arm existed, a `match` on a computed `Bool` fell through to the catch-all and
became `Neutral` — a closed ground term left stuck. A **flipped** repair is
strictly worse than the original: it returns the wrong branch silently, where
the original produced a visible stuck term.

**2d. Four discriminating tests already exist** in
`crates/ken-interp/tests/elim_bool_dispatch_acceptance.rs`. Cite them by
**function name, never by line number** — `CI-L1-EXECUTING-COVER` rewrites their
doc comments, and a pinned line range silently repoints at real but unrelated
content:

- `computed_bool_true_dispatches_to_first_method`
- `computed_bool_false_dispatches_to_second_method`
- `computed_bool_agrees_with_literal_bool_dispatch`
- `computed_bool_via_leq_int_dispatches_correctly`

**2e. Their previous row ids were phantoms and have been retired.** They claimed
four `surface/numbers/elim-reduce-computed-bool-*` ids that resolve to zero
headings anywhere under `conformance/`. `CI-L1-EXECUTING-COVER` converted those
claim lines to ordinary prose and **changed no assertion and no test body**. The
tests are intact; only the false certificates are gone.

**2f. `surface/numbers/` was the wrong area, and this is a ruling, not a
preference.** Conformance Validator judgment `evt_2ah01fn9v4ev3`: the correct
home is `conformance/runtime/evaluation/`, because `eq_int` and `leq_int` are
**replaceable witnesses** — the obligation applies to any primitive computation
yielding `Bool`. It is not `surface/data-match/` (elaboration already
succeeded) and not `kernel/inductive/` (registered primitive ops stay neutral in
kernel conversion; the seam is the interpreter's post-primitive consumer).

**2g. The existing property row does not subsume this.**
`runtime/evaluation/can-no-stuck-closed-ground (soundness, property)` is the
row whose property the original bug violated. It is a property over a corpus,
and that corpus evidently did not contain a computed-`Bool` scrutinee. Do not
resolve this node by asserting the property row already covers it — **and note
it would not catch the flipped repair at all**, which produces a value rather
than a stuck term.

## 3. Deliverables

**D1 — Author the rows** in `conformance/runtime/evaluation/seed-evaluation.md`,
in that file's established format (`### runtime/evaluation/<id> (tags)` with
`spec` / `given` / `expect` / `why` fields). Keep the family **compact**, per
CV: a branch-orientation pair and an agreement row. **Producer diversity
(`eq_int` alongside `leq_int`) is a witness or property member, not
automatically its own semantic row** — if you make it one, say why in the frame
of the row.

**D2 — Attach the cover claims.** Add a `/// runtime/evaluation/<id>` line to
the corresponding tests in `elim_bool_dispatch_acceptance.rs`. **Comment lines
only.** No assertion, no test body, no other file.

**D3 — State the residual.** Whether any part of the obligation remains
uncovered by the four existing tests, and if so what would cover it. "Nothing
remains" is an acceptable answer and closes it.

## 4. Acceptance criteria

**AC-1 — The claims resolve.** After D2, `python3 scripts/ci-ignored-sweep.py
verify-row-claims` is green and its reported resolved-claim count has risen by
exactly the number of claims added. This checker lands with
`CI-L1-EXECUTING-COVER`; it is the dependency's whole point.

**AC-2 — The rows are implementation-neutral.** No row text names `EvalVal`,
`EvalVal::Bool`, `EvalVal::Ctor`, a `methods[k]` array index, or any other
private evaluator representation. State the obligation in terms a second
implementation could satisfy. **Control:** a reader who has never seen
`eval.rs` can decide whether a candidate implementation passes.

**AC-3 — The orientation pair is non-degenerate, proven by mutation.** Invert
the hardcoded index derivation in `elim_reduce` (`k = if b { 0 } else { 1 }`
becomes `{ 1 } else { 0 }`). The pair must go **red**; restore byte-identically
and re-run green. Name the layer each red occurred at.

> **If the enclave judges running this cell outside its lane, do not weaken the
> AC and do not skip it.** Hand that single cell back to the Steward and it
> routes to Verify. The AC is discharged by the evidence, not by who produced
> it. This escape hatch is deliberate: an AC whose only discharge is banned by
> its own frame is a defect I have shipped before.

**AC-4 — The agreement row is the one that carries the coupling.** Its `why`
field must say that the two representations reach the eliminator by
**independent index derivations**, so agreement is the property that ties them.
A row that merely says "computed and literal agree" without naming why that is
not automatic does not satisfy this.

**AC-5 — Tagging follows the destination seed's taxonomy**, not the retired
`surface/numbers/` ids' `(soundness)` suffix. Read
`seed-evaluation.md`'s own trust-posture section, which defines what
`(soundness)`, `(oracle)`, and `(property)` mean for X1, and tag accordingly.
The old suffix is evidence of intended seriousness and **not** an inherited tag.

## 5. Scope

**In:** `conformance/runtime/evaluation/seed-evaluation.md`, and comment lines
in `crates/ken-interp/tests/elim_bool_dispatch_acceptance.rs`.

**Out, and these are bans:**

- **No production code changes.** `eval.rs` is touched only by AC-3's
  temporary, restored mutation cell.
- **No new seed file.** These rows belong in the existing
  `seed-evaluation.md`.
- **No re-litigating the retirement.** The five phantom claims are settled;
  `surface/numbers/legacy-add-sub-mul-retired` is closed as decorative and is
  **not** part of this node.
- **No widening to other uncovered evaluator seams.** If you find one, report
  it to the Steward; do not fold it in.

## 6. Contention

`conformance/runtime/evaluation/seed-evaluation.md` — no open node writes it.
`elim_bool_dispatch_acceptance.rs` — written by `CI-L1-EXECUTING-COVER`, which
is why this node depends on it. **Do not start until that merges**, or the
comment edits collide.
