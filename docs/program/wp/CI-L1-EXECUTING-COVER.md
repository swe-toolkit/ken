# WP CI-L1-EXECUTING-COVER — two executing rows that certify what they cannot check

**Owner:** Team Verify. **Branch:** `wp/CI-L1-EXECUTING-COVER`.
**Node:** [`docs/program/issues/CI-L1-EXECUTING-COVER.md`][n].
**Steward frame.** Size S. Gate: none.

> **Treat every anchor in this frame as perishable. If a fixed input turns out
> false against the landed code, say so and escalate -- do not quietly build
> around it.** Everything below was measured at `origin/main faabc2ed`; the
> deliverable edits some of the very code it cites.
>
> **Coordinates are given as grep-able phrases, not line numbers**, because this
> WP rewrites the lines it talks about. Where a number appears, it is an anchor
> to re-find, never a value to check.

## 1. Objective

Two tests in `crates/ken-interp/tests/l1_acceptance.rs` execute, pass, and are
counted as conformance cover for rows they cannot check. Make each one either
check its row or stop claiming it.

## 2. Why this is a separate node from `CI-ASSERTIONLESS-L1`

`CI-ASSERTIONLESS-L1` was cut for four rows -- three honest `#[ignore]`d
placeholders and one live assertion-free row -- and has been **rejected three
times**, each time on the file header's cover claim. The Architect's third
rejection (`dec_7yn4qg6q05t8n`, 2026-08-10) found these two additional rows and
directed that the header be **narrowed, not widened**.

**These two are the worse instance.** An `#[ignore]`d placeholder advertises
that it checks nothing. A green assertion is read as evidence the property
holds.

## 3. Fixed inputs — settled, do NOT reopen

### 3a. The seed is the authority, not the test's doc comment

That is the entire point of this node. Every claim about what a row requires is
read from `conformance/surface/numbers/seed-numbers.md`.

### 3b. `sec62` is vacuous against a soundness row — MEASURED

Search `l1_acceptance.rs` for `fn sec62_abstract_add_is_neutral`. It stands for
the seed row headed
`surface/numbers/algebraic-law-is-proposition-not-reduction  (soundness)`.

- The row's `given` is **the conversion query `a + b ≟ b + a`** on abstract
  `a b : Int`.
- The row's `expect` is that **kernel conversion rejects it**.
- The row names its own bug model: *"Under the exact bug this targets --
  registering an algebraic law as a kernel reduction (or making conversion
  accept it) -- `a + b ≡ b + a` would be accepted and this case flips."*

The test issues **no conversion query**. Its whole assertion compares the
`def_id`s of two separately-elaborated `fn` declarations and requires them to
differ -- true of any two declarations, including two with identical bodies.

⇒ **Under the exact bug the row targets, `sec62` stays green.**

### 3c. The capability EXISTS, so severance is NOT an available disposition

This is the audit that decides the deliverable, and it came out the way that
closes the easy exit. Measured at `faabc2ed`:

| needed | landed |
|---|---|
| a conversion query callable from a test | `pub fn convert(env, ctx, ty, a, b) -> bool` in `crates/ken-kernel/src/conv.rs`, **re-exported at the crate root** (`pub use conv::{convert, convert_type, level_eq, normalize, whnf}`) |
| a context holding abstract operands | `Context::new()` then `ctx.push(ty)` per binder -- the idiom used throughout `crates/ken-kernel/tests/k2c_conversion.rs` |
| the crate edge | `ken-interp/Cargo.toml` already depends on `ken-kernel`, and `l1_acceptance.rs` already has `use ken_kernel::{...}` |
| a `GlobalEnv` from the elaborator | `ElabEnv` exposes it; the landed `nc14_data_match_lowering.rs` reads `env.env.trusted_base()` |

⇒ **`sec62` is writable against landed public API.** The three severed rows in
`CI-ASSERTIONLESS-L1` wait on unbuilt capability; **this one does not**, so
"sever and mark" would be recording a capability gap that does not exist.

⛔ **Do not sever `sec62`.** If the ring believes it cannot be written, that is
a finding that contradicts this audit -- escalate it, do not route around it.

### 3d. `sec61` covers half of a two-half row, and its comment denies the other half

Search for `fn sec61_literal_reduces_in_kernel`. Its row is
`surface/numbers/primitive-op-runtime-value-k3-conversion-deferred`.

| seed half | test |
|---|---|
| "the **real interpreter** evaluates `add_int 2 3` to `Int 5`" | checked, via `eval_def` |
| "Kernel conversion does not... the application remains neutral and the equality does **not** close by `Refl`" | not checked |

The row marks a positive conversion oracle **DEFERRED/RED-UNTIL-K3**. The
test's doc comment says *"reduces to `5` definitionally in the kernel
evaluator"* -- **the opposite of the seed's second half**, because `add_int` is
a `PrimReduction::Op` rather than a `Literal`.

⚠ The assertion is correct about the interpreter. **The doc comment and the
cover claim are about the kernel, and they are wrong in the direction that
would hide a K3 boundary change.**

## 4. Deliverables

- **`D1` — `sec62` issues the conversion query its row names.** Build the
  abstract context (`a b : Int`), form `a + b` and `b + a`, call `convert`, and
  assert it returns `false`. ⛔ Severance is excluded by §3c.
- **`D2` — `sec61` is made honest.** Correct the doc comment so it does not
  assert kernel-definitional reduction of a `PrimReduction::Op`. Then either
  assert the unchecked half (conversion does **not** close the equality by
  `Refl`) or record that half as uncovered. **Naming which of the two you did,
  and why, is part of the deliverable.**
- **`D3` — the residual has a home.** An executing green test that does not
  cover its row is invisible to the ignored-test sweep by construction. If
  anything remains uncovered after `D1`/`D2`, it is recorded in
  `.github/ignored-test-exemptions.toml` with its reason, and the sweep
  resolves it. ⛔ A node that moves the gap out of the header and into nothing
  has not closed it.

## 5. Acceptance criteria

The seed hands us the discriminator for free. An AC that does not use it
repeats the defect this node exists to fix.

| AC | property | control |
|---|---|---|
| `AC-1` | `sec62` can detect the bug its row names. | **Mutation, and it must flip.** Make kernel conversion accept `a + b ≡ b + a` (the row's own stated bug), run `-p ken-interp --test l1_acceptance`, and show `sec62` **RED**. Restore byte-identically (`git diff --quiet`) and show it GREEN. Report both directions; a green-only run proves nothing. |
| `AC-2` | No test or comment in `l1_acceptance.rs` asserts kernel-definitional reduction of a `PrimReduction::Op`. | Enumerate every doc comment in the file that mentions the kernel, and state per comment whether it survived, was corrected, or was deleted. ⛔ Not "reviewed the file" -- a per-comment list. |
| `AC-3` | Every row left uncovered is named in the registry with its reason. | `PYTHONDONTWRITEBYTECODE=1 python3 scripts/test-ci-ignored-sweep.py` green, and the census and registry subtraction agree. |
| `AC-4` | The file header still makes **no** per-row conformance-cover enumeration. | Inherited guardrail from `CI-ASSERTIONLESS-L1`, and **the one this node is most likely to break** -- the natural instinct on fixing a row is to go write in the header that it is now covered. Quote the header verbatim in the handback. |
| `AC-5` | `sec61`/`sec62` are the only tests whose behaviour changed. | `git diff origin/main...HEAD -- crates/ken-interp/tests/l1_acceptance.rs`, and name every other hunk with its justification. |

## 6. Guardrails — do not reopen

- ⛔ **Do not touch the four `CI-ASSERTIONLESS-L1` rows** (`ac5_...`,
  `sec31_...`, `sec24_...`, and its live row). They belong to a node that is
  mid-flight on a fourth SHA. Editing them here creates a merge conflict on the
  same file and re-opens a settled disposition.
- ⛔ **Do not reintroduce a per-row cover enumeration in the header.** Three
  rewrites produced three different wrong subsets. The machine-checked artifact
  is the registry.
- ⛔ **Do not widen scope to other `l1_acceptance.rs` rows.** If you find a
  third, **report it and leave it** -- that is a finding for the Steward, and
  widening is how the sibling node reached three rejections.
- ⛔ **No kernel change, and no new trusted declaration.** `D1` consumes
  `convert`; it does not modify conversion. A mutation for `AC-1` is reverted
  byte-identically and never committed.

## 7. Contention

**Serialized behind `CI-ASSERTIONLESS-L1` by file, not by logic.** Both nodes
edit `crates/ken-interp/tests/l1_acceptance.rs`. ⛔ **Do not start until that
node merges**, then cut from the `origin/main` that carries it.

None with Runtime: `RT-DYNAMIC-ARM-SCALAR-MERGE` is confined to
`crates/ken-elaborator/src`, `crates/ken-runtime`, and `crates/ken-host`.

## 8. Validation — TARGETED ONLY

⛔ **Never `--workspace`** (operator, `agent/COORDINATION.md §12`).

```
scripts/ken-cargo test -p ken-interp --test l1_acceptance
PYTHONDONTWRITEBYTECODE=1 python3 scripts/test-ci-ignored-sweep.py
```

Workspace, `--locked`, and the conformance suite run **in CI**. A
"no-regression" criterion here means green in CI, never a local workspace run.

## 9. Reporting

Hand back the exact SHA, the paths touched, the `AC-1` mutation evidence in
**both** directions, the `AC-2` per-comment list, and the verbatim header for
`AC-4`. **State the `D2` choice explicitly** -- assert the half, or record it
uncovered -- rather than leaving a reader to infer it from the diff.

[n]: ../issues/CI-L1-EXECUTING-COVER.md
