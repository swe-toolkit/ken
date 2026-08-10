# CONF-VERIFY-SPEC-SYNTAX-PHANTOM-CLAIMS

Owner: **spec-enclave**. Size **S**. Gate: none. Depends on: nothing.
**Blocks `CI-ROW-CLAIM-NAMESPACE`**, which cannot merge until this lands.

## 1. Objective

Four `v1_acceptance` tests claim `verify/spec-syntax` conformance rows that were
never authored. Resolve each — author the row, or correct/remove the claim — so
the widened row-claim checker's census resolves.

## 2. Fixed inputs, measured at `origin/main = 5790c761`

**2a. The four unresolved claims**, from the widened checker's real-tree census
on held checkpoint `8f8bad6d` (34 attached claims, 30 resolving):

| test site | claimed row |
|---|---|
| `crates/ken-elaborator/tests/v1_acceptance.rs:205` | `verify/spec-syntax/old-fails-closed-without-pre-state` |
| `crates/ken-elaborator/tests/v1_acceptance.rs:543` | `verify/spec-syntax/requires-on-first-param-of-two` |
| `crates/ken-elaborator/tests/v1_acceptance.rs:583` | `verify/spec-syntax/requires-on-middle-param-of-three` |
| `crates/ken-elaborator/tests/v1_acceptance.rs:624` | `verify/spec-syntax/requires-on-final-param-unaffected` |

**2b. All four resolve to zero headings**, measured by grepping the whole of
`conformance/` for each id: 0 hits each. They are absent, not duplicated and
not misfiled.

**2c. They are not near-misses.**
`conformance/verify/spec-syntax/seed-spec-syntax.md` carries **16** `### `
headings. The `old` family holds `old-resolves-in-space-op-ensures` and
`old-out-of-scope-rejects`; the `requires` family holds only
`requires-elaborates-to-pi-proof-arg`. **There is no positional
`requires-on-*-param` family at all.**

**2d. The tests are real and currently pass.** They are `#[test]` functions in a
committed acceptance suite; nothing here is a dead or ignored test.

**2e. The claims are pre-existing.** They predate `CI-ROW-CLAIM-NAMESPACE` and
that node does not touch `crates/ken-elaborator/`. It only made them visible.

## 3. Deliverables

**D1 — Decide each of the four independently, on the merits.** For each claim,
choose exactly one:

- **author the row** in `conformance/verify/spec-syntax/seed-spec-syntax.md`; or
- **correct the claim** to an existing row it should have named; or
- **remove the claim**, leaving the test.

**State which you chose for each and why.** A blanket disposition applied to all
four without per-claim reasoning is not a discharge — the three
`requires-on-*-param` ids are one family and `old-fails-closed-without-pre-state`
is not, so at minimum the reasoning splits.

**D2 — If you author rows, they must be implementation-neutral**, in the same
shape as the surrounding 16: behaviour stated so a second implementation could
satisfy it, not a restatement of what the elaborator happens to do.

**D3 — Report the resulting census.** Run
`python3 scripts/ci-ignored-sweep.py verify-row-claims` **against the widened
checker on Verify's held checkpoint `8f8bad6d`**, not against `main`'s
surface-only version, which cannot see these claims at all and will report a
misleading green. Say which checker you ran.

## 4. Acceptance criteria

**AC-1 — All four resolve, one heading each.** The widened checker's census
reports zero unresolved claims. State the before and after counts (before: 34
attached / 30 resolving).

**AC-2 — Exactly one heading per claim, not merely at least one.** The checker
fails a claim resolving to two headings as well as zero. If you author a row,
confirm the id does not collide with an existing heading anywhere under
`conformance/`.

**AC-3 — The other 30 are unperturbed.** Every claim that resolved before still
resolves. An edit to the seed that renames or reflows an existing heading is a
regression regardless of what it fixes.

**AC-4 — No checker edit.** `scripts/ci-ignored-sweep.py` and
`scripts/test-ci-ignored-sweep.py` are **out of scope and banned here** — they
belong to `CI-ROW-CLAIM-NAMESPACE` and are being edited on its branch right now.
Touching them creates a direct collision.

**AC-5 — No test deletion.** If a claim is wrong, the claim goes; the test
stays. Deleting a passing acceptance test to silence a claim reproduces the
defect in a quieter form.

## 5. Scope

**In:** `conformance/verify/spec-syntax/seed-spec-syntax.md`, and the four
`///` claim lines in `crates/ken-elaborator/tests/v1_acceptance.rs`.

**Out, and these are bans:**

- **No checker changes** — see `AC-4`.
- **No narrowing or quarantine** to reach green. Suppressing these four ids is
  the exact trap `CI-ROW-CLAIM-NAMESPACE` was framed to avoid.
- **No production code changes.**
- **No new tests.** This node documents or corrects existing claims; it does not
  extend coverage.

## 6. Contention

`CI-ROW-CLAIM-NAMESPACE` is live on Verify's ring at held checkpoint `8f8bad6d`,
scoped to the two checker scripts. **The file sets are disjoint** — that node
touches only `scripts/`, this one only `conformance/` and one Rust test file —
but the two are **semantically coupled**: this node's output is what makes that
node's census green. Sequence: this lands, then Verify resumes and merges.

`CONF-EVAL-COMPUTED-BOOL-ELIM` touches
`conformance/runtime/evaluation/seed-evaluation.md`, a different namespace file.
No collision.

> **Framing note.** Do not treat "author the four rows" as the foregone
> conclusion because it is the path that makes the census green fastest.
> **The green census is a consequence of the right answer, not the goal.** If a
> claim names a behaviour that does not warrant a conformance row, removing the
> claim is the correct discharge and the census goes green either way.
