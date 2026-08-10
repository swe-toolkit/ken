# CI-ROW-CLAIM-COMMENT-FORM

Owner: **verify**. Size **S**. Gate: none.
Depends on: `CI-L1-EXECUTING-COVER` (merged).

## 1. Objective

`verify-row-claims` extracts row claims only from `///` doc comments, so a
claim written with `//` is invisible to it. **Two false soundness certificates
survive on `main` in exactly that form.** Widen the extractor to the population
the rule actually governs — a claim attached to a Rust `#[test]` — and retire
the two false certificates.

## 2. Fixed inputs, measured at `origin/main = 53c09f9b`

Independently corroborated by the Steward against the artifact, not taken from
the report.

**2a. Three `//`-form claims sit on `#[test]` functions**, all in
`crates/ken-interp/src/eval.rs`, all carrying the `(soundness)` marker:

| line | id | resolves |
|---|---|---|
| `:8451` | `surface/numbers/f1-store-roundtrip-above-i128-byte-identical` | **0 headings, 0 occurrences anywhere under `conformance/`** |
| `:8497` | `surface/numbers/f1-dedup-content-address-stable-across-paths` | **0 headings, 0 occurrences anywhere** |
| `:8519` | `surface/numbers/f1-zero-and-sign-canonical` | 1 heading — **valid, not a defect, do not touch it** |

**2b. The extractor sees every claim in its own form.** Measured on the claim
form (doc content *beginning* with `surface/`): `///` = **29**, `//!` = **0**,
`//` = **3**. The 29 is exactly the delivered tree's reported count, so the
`///` path is complete and **this is a population gap, not a parser bug**.

**2c. The `//!` hypothesis is dead — do not chase it.** A first census matching
`surface/[a-z0-9]` across `crates/` returned 157 mentions of which 29 were
`//!`, which looks like an uncounted module-level claim population. **Those 29
are file-path citations** (`spec/30-surface/35-numbers.md`,
`conformance/surface/numbers/seed-numbers.md`), not row ids. A count over a
mixed population is the exact shape that survived five reviews on
`CI-IGNORED-SWEEP`. **Match the claim form, never a bare substring.**

**2d. Heading matching must be by prefix.** Real headings carry trailing
markers — `(soundness)`, `[NODE]` — so an exact-match resolver produces false
negatives. The landed checker already handles this; do not regress it.

## 3. Deliverables

**D1 — Widen the extractor.** `scripts/ci-ignored-sweep.py verify-row-claims`
must treat a row claim on a `#[test]` function as a claim **regardless of
comment marker** (`//`, `///`). Resolution only, never adequacy — that boundary
is unchanged and is not reopened.

**D2 — Retire the two false certificates.** Convert the `:8451` and `:8497`
claim lines to ordinary prose that is not a row-id form. **Do not delete the
tests. Do not touch a single assertion. Do not touch `:8519`**, which resolves.

**D3 — Report the new resolved count**, measured on the delivered tree. **Do
not carry a predicted number from this frame** — the count is a function of
both the widening and the retirement, and predicting it is how the last two
census claims went wrong.

## 4. Acceptance criteria

**AC-1 — The gap is closed in the informative direction.** Add a control with a
**`//`-form fabricated id** on a `#[test]`. The checker must **red**, naming the
test and the id. Restore byte-identically and re-run green. **A control that
only adds a `///` case proves nothing here** — the `///` path already worked.

**AC-2 — Both directions of the widening.** A `//`-form claim that *resolves*
must pass (`:8519` is the live positive control), and a `//`-form claim that
does not must red. Without the first, the widening could be satisfied by a
checker that rejects every `//` claim.

**AC-3 — The retirement changes no behaviour.** `scripts/ken-cargo test -p
ken-interp` covering the two touched tests is green before and after, and the
diff shows comment lines only.

**AC-4 — No adequacy creep.** The checker still asserts only that a claimed id
resolves to exactly one `### <id>` heading. If you find yourself adding a check
about whether the test *covers* the row, stop — that is human judgment and it
is not this node's.

**AC-5 — Do not author `conformance/` rows.** The two retired ids are Findings.
If either looks like a row that should exist, **report it and stop**; the
Steward routes it to the Conformance Validator, as with the four that became
`CONF-EVAL-COMPUTED-BOOL-ELIM`.

## 5. Scope

**In:** `scripts/ci-ignored-sweep.py`, `scripts/test-ci-ignored-sweep.py`, and
the two comment lines in `crates/ken-interp/src/eval.rs`.

**Out, and these are bans:**

- **No `conformance/` authoring.** Ownership boundary, not a size one.
- **No narrowing the population to make anything pass.** This node exists
  because the population was too narrow. If a widened census surfaces further
  unresolved claims, **hold and report** — that is the disposition that worked
  on `D5` and it is the standing rule.
- **No production code changes.** `eval.rs` is touched only in comments.
- **No `//!` module-level extraction.** Measured at zero; adding it would
  re-admit the file-path citations as false claims.

## 6. Contention

`scripts/ci-ignored-sweep.py` and `crates/ken-interp/src/eval.rs` — no open node
writes either. `CONF-EVAL-COMPUTED-BOOL-ELIM` writes
`crates/ken-interp/tests/elim_bool_dispatch_acceptance.rs` and
`conformance/runtime/evaluation/seed-evaluation.md`; **disjoint from this node,
so the two may run concurrently.** They do interact through the resolved-claim
count: whichever lands second must re-measure rather than assume the other's
number.

> **Framing note carried in deliberately.** Verify's own retro carry from
> `CI-L1-EXECUTING-COVER`: run the prospective whole-corpus census **before**
> building or widening a corpus gate; if it finds pre-existing violations, hold
> the population, classify the hits, and get an explicit disposition rather than
> weakening the gate. Section 2 is that census, already run — which is why this
> node starts with its violations enumerated instead of discovering them as a
> hard stop.
