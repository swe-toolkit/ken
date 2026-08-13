# RT-4B-WALKED-CONSTANCY — record the measured constancy of `walked` in the observation's own doc

**Owner: runtime. Size: XS. Gate: none.**
**COMMENT-ONLY. No code, no test, no assertion changes.**

**Base: re-derive `origin/main` at cut time.** Fixed inputs measured at
`2a1d87a2`.

## Fixed inputs

| fact | site |
|---|---|
| the perturbed rows, asserting `(1, 0, 0, 4, 2, 0, 2, 1)` twice | `crates/ken-runtime/src/cranelift_backend/lowering/core/tests/control.rs:3565-3566` |
| the unperturbed row, `D2jCause::Exact`, asserting `(4, 2, 0, 2, 1)` | same file, `:3733`, inside `r3_4b_input_observation_is_artifact_identical_when_disabled` at `:3661` |
| the paragraph that understates the result | same file, `:3656` — the `THE GAP:` paragraph |
| the struct doc and its field | `lowering/core.rs:528-542` (doc), `:558-560` (`walked_admitted_continuation_discoveries`) |

## D1 — re-derive the constancy at cut time, then write it

**Read the two assertions first.** If the tuples are not identical across
`Exact`, `ExactSuffix` and `CallIdentity` on the tree you cut from, this node's
premise is false — hard stop, return it.

Then write the fact into **both** doc sites as a measurement:

> `walked` is invariant at 4 across `Exact`, `ExactSuffix` and `CallIdentity` —
> measured — so it discriminates input size and nothing downstream of it.

Use your own wording; the requirement is the content, not the sentence.

## D2 — correct the `THE GAP:` paragraph rather than adding beside it

The existing wording claims the number licenses no conclusion about **which**
relation declined. Replace it with what was measured: **the number does not
move between declining and not declining at all.**

## Acceptance criteria

- **AC-1 — the statement names all three causes and both assertion sites**, so
  a reader can check it without re-deriving which fixtures are perturbed.
- **AC-2 — the change is comment-only**, established mechanically before the
  publish flag is chosen, not asserted:
  ```sh
  git diff -U0 <BASE>...<SHA> | grep -E '^[+-]' | grep -vE '^(\+\+\+|---)' \
    | sed -E 's/^[+-]//' | grep -vE '^\s*(//|///|//!|\*|/\*)' | grep -vE '^\s*$'
  ```
  **Empty output both directions or it is not this node.**
- **AC-3 — no fenced code block is added inside any `///`.** A doctest is a
  compiled, executed test wearing a comment's syntax, and this node adds no
  test.
- **AC-4 — the `THE GAP:` paragraph is EDITED, not appended to.** Leaving the
  weaker claim standing beside the stronger one gives a reader two readings and
  the weaker one is the reading this node exists to remove.
- **AC-5 — the text does not claim the instrument is defective.** Recording the
  input population is sound. **The defect would be a claim drawn from it**, and
  a sentence that reads as "this field is broken" would invite exactly the
  removal that loses a working measurement.

## Pre-stated licensing — read BEFORE reporting

| what this lands | what it licenses |
|---|---|
| the constancy, in the doc | **Nothing about the planner, and nothing about 4b's status.** It bounds how any future `walked` read may be used. It does not settle whether the planner fuses for any witness — the unperturbed rows already show it fusing — and it does not close or reopen any 4b node. |

> **This node cannot conclude that the observation was a mistake.** It records
> what the number does and does not discriminate. If your report reads as "4b's
> instrument was wrong," rewrite it.

## Banned scope

- Any change to the instrument: no new field, no removed field, no signature
  change, no change to what is recorded or when.
- Widening the `d2f_0` positive rows to carry the five fields. **Rejected on
  purpose** — it duplicates a fact the identity test already pins.
- Any code, test or assertion change whatsoever (see AC-2).
- Enumeration, classifier, checker, marker, fusion-candidate, representation,
  ledger or closure-boundary repair. Gates 5 and 6 held; production unarmed.

## Hard stops — return to the Steward

- **The tuples are not identical across the three causes at cut time.**
- **The correction cannot be made without touching executable text.**

## Sequencing and contention

Runtime, one lane. `RT-4B-C2-REACHABILITY` **reads** `lowering/core.rs` and
`control.rs` and modifies nothing (its AC-2), so there is no write contention —
but run this after it hands back rather than beside it, one lane at a time.

Publish `--doc-only`: a comment-only diff is a doc change that happens to live
in a code file, and AC-2 is what establishes that mechanically.
