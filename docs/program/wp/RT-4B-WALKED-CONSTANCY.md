# RT-4B-WALKED-CONSTANCY — record the measured constancy of `walked`, and bound the one comment that already depends on the weaker reading

**Owner: runtime. Size: XS. Gate: none.**
**COMMENT-ONLY. No code, no test, no assertion changes. Three comment sites.**

**Amended 2026-08-13 after the first kick and before any source read**, adding
D3 and AC-6 from a second Adversary finding. **The guardrails below were swept
for the addition** — fixed inputs, banned scope, hard stops and contention all
carry it.

**Base: re-derive `origin/main` at cut time.** Fixed inputs measured at
`2a1d87a2`.

## Fixed inputs

| fact | site |
|---|---|
| the perturbed rows, asserting `(1, 0, 0, 4, 2, 0, 2, 1)` twice | `crates/ken-runtime/src/cranelift_backend/lowering/core/tests/control.rs:3565-3566` |
| the unperturbed row, `D2jCause::Exact`, asserting `(4, 2, 0, 2, 1)` | same file, `:3733`, inside `r3_4b_input_observation_is_artifact_identical_when_disabled` at `:3661` |
| the paragraph that understates the result | same file, `:3656` — the `THE GAP:` paragraph |
| the struct doc and its field | `lowering/core.rs:528-542` (doc), `:558-560` (`walked_admitted_continuation_discoveries`) |
| the A/B whose comment D3 corrects, and its two branches | `planning/static_transition.rs:18998-19010` |
| the type whose derived equality diverges | `planning/static_transition.rs:8977` — `StaticContinuationFusionPlan` derives `Clone, Debug, Default, Eq, PartialEq`, and `walked` is `#[cfg(test)]` |

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

## D3 — bound the A/B comment at `static_transition.rs:18998-19010`

**Added 2026-08-13 from a second Adversary finding, same root cause, same
repair shape.** `StaticContinuationFusionPlan` derives `PartialEq`/`Eq` and
`walked` is `#[cfg(test)]` — so **in a test build two plans with identical keys
and descriptors but different `walked` compare unequal, and in a non-test build
they compare equal.** "Zero production footprint" is true of behaviour and false
of the type's value identity.

The A/B's two branches already differ on it: the `None` arm carries the
enumerator-set `walked`, the `Some(carrier)` arm is a synthetic
`::default()` with `walked == 0`. The comment above them says the operand moved
**"and nothing else"**, and that a refusal there is attributable to the carrier.

**There is no witness today** — the test compares views, headers and slots, not
planes. It becomes one the moment anyone adds `walked` to what the A/B observes,
which is a natural thing to do now the field exists: the mutated row would
differ for a reason that has nothing to do with the carrier, **and the comment
says such a difference is attributable to the carrier.**

Add one clause: *the mutated plane is synthetic and carries no enumerator input
count; compare only what the tuple below returns.*

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
- **AC-6 — the D3 clause names the SYNTHETIC branch as the reason**, not the
  carrier. The whole point is that a `walked` difference between those two
  branches is attributable to `::default()` and not to the operand under test.
  **A clause that says only "be careful comparing planes" has not said it.**

## Pre-stated licensing — read BEFORE reporting

| what this lands | what it licenses |
|---|---|
| the constancy, in the doc | **Nothing about the planner, and nothing about 4b's status.** It bounds how any future `walked` read may be used. It does not settle whether the planner fuses for any witness — the unperturbed rows already show it fusing — and it does not close or reopen any 4b node. |
| the D3 clause | **Nothing.** It is preventive: there is no witness today. It does not assert a defect in the field, the derive, or the A/B. |

> **This node cannot conclude that the observation was a mistake.** It records
> what the number does and does not discriminate. If your report reads as "4b's
> instrument was wrong," rewrite it.

## Banned scope

- Any change to the instrument: no new field, no removed field, no signature
  change, no change to what is recorded or when.
- Widening the `d2f_0` positive rows to carry the five fields. **Rejected on
  purpose** — it duplicates a fact the identity test already pins.
- Any code, test or assertion change whatsoever (see AC-2).
- **Changing the `PartialEq`/`Eq` derive on `StaticContinuationFusionPlan`, or
  the `#[cfg(test)]` gating on `walked`.** The divergence D3 documents is real
  and it is not this node's to repair — **and there is no live defect**, so
  "fixing" it here would trade a comment for a code change nobody authorized.
- **Adding `walked` to what the A/B at `:18998-19010` observes.** That is the
  edit D3's clause exists to warn the next reader about.
- Enumeration, classifier, checker, marker, fusion-candidate, representation,
  ledger or closure-boundary repair. Gates 5 and 6 held; production unarmed.

## Hard stops — return to the Steward

- **The tuples are not identical across the three causes at cut time.**
- **The correction cannot be made without touching executable text.**
- **The A/B comment at `:18998-19010` has already moved** — re-read it rather
  than editing to the wording quoted in D3.

## Sequencing and contention

Runtime, one lane. `RT-4B-C2-REACHABILITY` is `closed` — it read
`lowering/core.rs` and `control.rs` and modified nothing, so nothing it did
contends. **This node writes comments in three files: `lowering/core.rs`,
`lowering/core/tests/control.rs` and `planning/static_transition.rs`.**

`RT-4B-OBSERVATION-FEATURE-GATE` edits the first of those and is the next
runtime node — **they must not run beside each other.**

Publish `--doc-only`: a comment-only diff is a doc change that happens to live
in a code file, and AC-2 is what establishes that mechanically.
